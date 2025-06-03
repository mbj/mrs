use crate::stack::try_load_stack;
use crate::types::*;

pub(crate) struct RemoteOperation {
    pub(crate) client_request_token: ClientRequestToken,
    pub(crate) stack_id: StackId,
}

pub type Capabilities = std::collections::BTreeSet<aws_sdk_cloudformation::types::Capability>;

pub struct TemplateUploader<'a> {
    pub cloudformation: &'a aws_sdk_cloudformation::Client,
    pub s3: &'a aws_sdk_s3::Client,
    pub s3_bucket_name_output_key: &'a OutputKey,
    pub stack_name: &'a StackName,
}

impl TemplateUploader<'_> {
    async fn upload(&self, template_rendered: &TemplateRendered) -> TemplateUrl {
        let hex =
            hex::encode(<sha2::Sha256 as sha2::Digest>::digest(&template_rendered.body).as_slice());

        let s3_bucket_name = crate::stack::read_stack_output(
            self.cloudformation,
            self.stack_name,
            self.s3_bucket_name_output_key,
        )
        .await;

        let s3_object_key = format!("{hex}.{}", template_rendered.format.file_ext());

        log::info!("Uploading template to: {s3_bucket_name}/{s3_object_key}");

        self.s3
            .put_object()
            .bucket(&s3_bucket_name)
            .key(&s3_object_key)
            .body((&template_rendered.body).into())
            .send()
            .await
            .unwrap();

        format!("https://s3.amazonaws.com/{s3_bucket_name}/{s3_object_key}").into()
    }
}

#[derive(Debug)]
pub struct InstanceSpec {
    pub capabilities: Capabilities,
    pub stack_name: StackName,
    pub parameter_map: ParameterMap,
    pub template: Template,
}

impl InstanceSpec {
    pub fn context<'a>(
        &'a self,
        cloudformation: &'a aws_sdk_cloudformation::Client,
        template_uploader: Option<&'a TemplateUploader<'a>>,
    ) -> Context<'a> {
        Context {
            cloudformation,
            instance_spec: self,
            template_uploader,
        }
    }

    pub(crate) async fn watch(cloudformation: &aws_sdk_cloudformation::Client, stack_id: StackId) {
        crate::events::Poll::default(stack_id)
            .run(cloudformation, |stack_event| {
                crate::events::print_event(stack_event)
            })
            .await;
    }
}

pub struct Context<'a> {
    cloudformation: &'a aws_sdk_cloudformation::Client,
    instance_spec: &'a InstanceSpec,
    template_uploader: Option<&'a TemplateUploader<'a>>,
}

impl Context<'_> {
    pub async fn delete(&self) {
        let client_request_token = ClientRequestToken::generate();
        let stack_id =
            crate::stack::load_stack_id(self.cloudformation, &self.instance_spec.stack_name).await;

        self.cloudformation
            .delete_stack()
            .client_request_token(&client_request_token)
            .stack_name(self.instance_spec.stack_name.as_str())
            .send()
            .await
            .unwrap();

        Self::wait(
            self.cloudformation,
            RemoteOperation {
                client_request_token,
                stack_id,
            },
        )
        .await
    }
    pub async fn sync(&self, user_parameter_map: &ParameterMap) {
        Self::process_update_result(
            self.cloudformation,
            match try_load_stack(self.cloudformation, &self.instance_spec.stack_name).await {
                Some(existing_stack) => {
                    self.start_template_update(&existing_stack, user_parameter_map)
                        .await
                }
                None => Some(self.start_create(user_parameter_map).await),
            },
        )
        .await
    }

    pub async fn update(&self, user_parameter_map: &ParameterMap) {
        let existing_stack = try_load_stack(self.cloudformation, &self.instance_spec.stack_name)
            .await
            .expect("stack exists");

        Self::process_update_result(
            self.cloudformation,
            self.start_template_update(&existing_stack, user_parameter_map)
                .await,
        )
        .await
    }

    async fn process_update_result(
        cloudformation: &aws_sdk_cloudformation::Client,
        result: Option<RemoteOperation>,
    ) {
        match result {
            None => log::info!("Stack is already up to date"),
            Some(remote_operation) => Self::wait(cloudformation, remote_operation).await,
        }
    }

    pub async fn parameter_update(&self, user_parameter_map: &ParameterMap) {
        let result = self
            .start_parameter_update(
                self.cloudformation,
                &try_load_stack(self.cloudformation, &self.instance_spec.stack_name)
                    .await
                    .expect("Stack should exist"),
                user_parameter_map,
            )
            .await;

        match result {
            None => log::info!("Stack is already up to date"),
            Some(remote_operation) => Self::wait(self.cloudformation, remote_operation).await,
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

    async fn start_create(&self, user_parameter_map: &ParameterMap) -> RemoteOperation {
        let client_request_token = ClientRequestToken::generate();

        let request = self
            .cloudformation
            .create_stack()
            .stack_name(self.instance_spec.stack_name.as_str())
            .set_parameters(Some(
                self.instance_spec
                    .parameter_map
                    .merge(user_parameter_map)
                    .to_create_parameters(),
            ))
            .set_capabilities(Some(self.capabilities()))
            .client_request_token(&client_request_token);

        let stack_id = StackId(
            self.set_create_template(request)
                .await
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
        existing_stack: &aws_sdk_cloudformation::types::Stack,
        user_parameter_map: &ParameterMap,
    ) -> Option<RemoteOperation> {
        let client_request_token = ClientRequestToken::generate();
        let existing_stack_parameters =
            std::collections::BTreeSet::from_iter(match &existing_stack.parameters {
                None => vec![],
                Some(parameters) => parameters
                    .iter()
                    .map(|parameter| ParameterKey(parameter.parameter_key.clone().unwrap()))
                    .collect(),
            });

        let request = self
            .cloudformation
            .update_stack()
            .stack_name(existing_stack.stack_id.as_ref().unwrap())
            .set_parameters(Some(
                self.instance_spec
                    .parameter_map
                    .merge(user_parameter_map)
                    .to_template_update_parameters(
                        self.instance_spec.template.parameter_keys(),
                        &existing_stack_parameters,
                    ),
            ))
            .set_capabilities(Some(self.capabilities()))
            .client_request_token(&client_request_token);

        let response = self.set_update_template(request).await.send().await;

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
            .set_capabilities(Some(self.capabilities()))
            .client_request_token(&client_request_token)
            .use_previous_template(true)
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
        self.instance_spec.capabilities.iter().cloned().collect()
    }

    fn template_rendered(&self) -> &TemplateRendered {
        match &self.instance_spec.template {
            Template::Plain { rendered, .. } => rendered,
        }
    }

    async fn set_create_template(
        &self,
        request: aws_sdk_cloudformation::operation::create_stack::builders::CreateStackFluentBuilder,
    ) -> aws_sdk_cloudformation::operation::create_stack::builders::CreateStackFluentBuilder {
        match self.upload_template().await {
            Ok(template_url) => request.template_url(template_url),
            Err(template_body) => request.template_body(template_body),
        }
    }

    async fn set_update_template(
        &self,
        request: aws_sdk_cloudformation::operation::update_stack::builders::UpdateStackFluentBuilder,
    ) -> aws_sdk_cloudformation::operation::update_stack::builders::UpdateStackFluentBuilder {
        match self.upload_template().await {
            Ok(template_url) => request.template_url(template_url),
            Err(template_body) => request.template_body(template_body),
        }
    }

    async fn upload_template(&self) -> Result<TemplateUrl, &TemplateBody> {
        let template_rendered = self.template_rendered();

        if template_rendered.body.needs_upload() {
            log::debug!("Template is to big for inline, uploading it");

            let template_uploader = self
                .template_uploader
                .as_ref()
                .expect("Template needs upload but template uploader not configured!");

            Ok(template_uploader.upload(template_rendered).await)
        } else {
            Err(&template_rendered.body)
        }
    }
}

pub struct Registry(pub Vec<InstanceSpec>);

impl Registry {
    pub fn find(&self, instance_name: &StackName) -> Option<&InstanceSpec> {
        self.0
            .iter()
            .find(|&instance_spec| instance_spec.stack_name == *instance_name)
    }
}
