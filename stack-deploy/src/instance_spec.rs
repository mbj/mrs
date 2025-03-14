use crate::stack::try_load_stack;
use crate::types::*;

pub(crate) struct RemoteOperation {
    pub(crate) client_request_token: ClientRequestToken,
    pub(crate) stack_id: StackId,
}

pub type Capabilities = std::collections::BTreeSet<aws_sdk_cloudformation::types::Capability>;

#[derive(Debug)]
pub struct InstanceSpec {
    pub capabilities: Capabilities,
    pub name: StackName,
    pub parameter_map: ParameterMap,
    pub template: Template,
}

impl InstanceSpec {
    pub async fn delete(&self, cloudformation: &aws_sdk_cloudformation::client::Client) {
        let client_request_token = ClientRequestToken::generate();
        let stack_id = crate::stack::fetch_stack_id(cloudformation, &self.name.0).await;

        cloudformation
            .delete_stack()
            .client_request_token(&client_request_token)
            .stack_name(&self.name)
            .send()
            .await
            .unwrap();

        Self::wait(
            cloudformation,
            RemoteOperation {
                client_request_token,
                stack_id,
            },
        )
        .await
    }
    pub async fn sync(
        &self,
        cloudformation: &aws_sdk_cloudformation::client::Client,
        user_parameter_map: &ParameterMap,
    ) {
        Self::process_update_result(
            cloudformation,
            match try_load_stack(cloudformation, &self.name).await {
                Some(existing_stack) => {
                    self.start_template_update(
                        cloudformation,
                        StackId(existing_stack.stack_id.unwrap()),
                        user_parameter_map,
                    )
                    .await
                }
                None => Some(self.start_create(cloudformation, user_parameter_map).await),
            },
        )
        .await
    }

    pub async fn update(
        &self,
        cloudformation: &aws_sdk_cloudformation::client::Client,
        user_parameter_map: &ParameterMap,
    ) {
        let existing_stack = try_load_stack(cloudformation, &self.name)
            .await
            .expect("stack exists");

        Self::process_update_result(
            cloudformation,
            self.start_template_update(
                cloudformation,
                StackId(existing_stack.stack_id.unwrap()),
                user_parameter_map,
            )
            .await,
        )
        .await
    }

    async fn process_update_result(
        cloudformation: &aws_sdk_cloudformation::Client,
        result: Option<RemoteOperation>,
    ) {
        match result {
            None => eprintln!("Stack is already up to date"),
            Some(remote_operation) => Self::wait(cloudformation, remote_operation).await,
        }
    }

    pub async fn parameter_update(
        &self,
        cloudformation: &aws_sdk_cloudformation::client::Client,
        user_parameter_map: &ParameterMap,
    ) {
        let result = self
            .start_parameter_update(
                cloudformation,
                &try_load_stack(cloudformation, &self.name)
                    .await
                    .expect("Stack should exist"),
                user_parameter_map,
            )
            .await;

        match result {
            None => eprintln!("Stack is already up to date"),
            Some(remote_operation) => Self::wait(cloudformation, remote_operation).await,
        }
    }

    async fn wait(
        cloudformation: &aws_sdk_cloudformation::Client,
        remote_operation: RemoteOperation,
    ) {
        crate::events::Poll::wait_for_remote_operation(remote_operation)
            .run(cloudformation, crate::events::print_event)
            .await
    }

    pub(crate) async fn watch(cloudformation: &aws_sdk_cloudformation::Client, stack_id: StackId) {
        crate::events::Poll::default(stack_id)
            .run(cloudformation, |stack_event| {
                crate::events::print_event(stack_event)
            })
            .await;
    }

    async fn start_create(
        &self,
        cloudformation: &aws_sdk_cloudformation::Client,
        user_parameter_map: &ParameterMap,
    ) -> RemoteOperation {
        let client_request_token = ClientRequestToken::generate();

        let stack_id = StackId(
            cloudformation
                .create_stack()
                .stack_name(&self.name)
                .set_parameters(Some(
                    self.parameter_map
                        .merge(user_parameter_map)
                        .to_create_parameters(),
                ))
                .template_body(self.template_body())
                .set_capabilities(Some(self.capabilities()))
                .client_request_token(&client_request_token)
                .send()
                .await
                .unwrap()
                .stack_id
                .unwrap(),
        );

        RemoteOperation {
            client_request_token,
            stack_id,
        }
    }

    async fn start_template_update(
        &self,
        cloudformation: &aws_sdk_cloudformation::Client,
        stack_id: StackId,
        user_parameter_map: &ParameterMap,
    ) -> Option<RemoteOperation> {
        let client_request_token = ClientRequestToken::generate();
        let response = cloudformation
            .update_stack()
            .stack_name(&stack_id)
            .set_parameters(Some(
                self.parameter_map
                    .merge(user_parameter_map)
                    .to_template_update_parameters(self.template.parameter_keys()),
            ))
            .template_body(self.template_body())
            .set_capabilities(Some(self.capabilities()))
            .client_request_token(&client_request_token)
            .send()
            .await;

        Self::process_update_response(client_request_token, response)
    }

    async fn start_parameter_update(
        &self,
        cloudformation: &aws_sdk_cloudformation::Client,
        existing_stack: &aws_sdk_cloudformation::types::Stack,
        user_parameter_map: &ParameterMap,
    ) -> Option<RemoteOperation> {
        let client_request_token = ClientRequestToken::generate();
        let response = cloudformation
            .update_stack()
            .stack_name(existing_stack.stack_id.as_ref().unwrap())
            .set_parameters(Some(
                user_parameter_map.to_parameter_update_parameters(existing_stack),
            ))
            .template_body(self.template_body())
            .set_capabilities(Some(self.capabilities()))
            .client_request_token(&client_request_token)
            .send()
            .await;

        Self::process_update_response(client_request_token, response)
    }

    fn process_update_response(
        client_request_token: ClientRequestToken,
        result: Result<
            aws_sdk_cloudformation::operation::update_stack::UpdateStackOutput,
            aws_sdk_cloudformation::error::SdkError<
                aws_sdk_cloudformation::operation::update_stack::UpdateStackError,
            >,
        >,
    ) -> Option<RemoteOperation> {
        match result {
            Ok(output) => Some(RemoteOperation {
                client_request_token,
                stack_id: StackId(output.stack_id.unwrap()),
            }),
            Err(error) => {
                let service_error = error.into_service_error();
                let meta = service_error.meta();

                match meta.code() {
                    // CF API has no more direct signal that an update is a noop.
                    Some("ValidationError")
                        if meta.message() == Some("No updates are to be performed.") =>
                    {
                        None
                    }
                    _ => panic!("unexpected service error: {service_error:#?}"),
                }
            }
        }
    }

    fn capabilities(&self) -> Vec<aws_sdk_cloudformation::types::Capability> {
        self.capabilities.iter().cloned().collect()
    }

    fn template_body(&self) -> &String {
        match &self.template {
            Template::Plain { body, .. } => body,
        }
    }
}

pub struct Registry(pub Vec<InstanceSpec>);

impl Registry {
    pub fn find(&self, instance_name: &StackName) -> Option<&InstanceSpec> {
        self.0
            .iter()
            .find(|&instance_spec| instance_spec.name == *instance_name)
    }
}
