use crate::types::*;

struct RemoteOperation {
    client_request_token: ClientRequestToken,
    stack_id: StackId,
}

#[derive(Debug)]
pub struct InstanceSpec {
    pub capabilities: std::collections::BTreeSet<aws_sdk_cloudformation::types::Capability>,
    pub name: StackName,
    pub parameter_map: ParameterMap,
    pub template: Template,
}

impl InstanceSpec {
    pub async fn sync(&self, cloudformation: &aws_sdk_cloudformation::client::Client, parameter_map: &ParameterMap) {
        Self::wait(
            cloudformation,
            match try_load_stack(cloudformation, &self.name).await {
                Some(existing_stack) => self.start_update(cloudformation, parameter_map, existing_stack).await,
                None => self.start_create(cloudformation, parameter_map).await,
            },
        )
        .await
    }

    async fn wait(
        cloudformation: &aws_sdk_cloudformation::Client,
        remote_operation: RemoteOperation,
    ) {
        loop {
            match Self::read_stack_status(cloudformation, &remote_operation.stack_id).await {
                _ => todo!(),
            }
        }
    }

    async fn start_create(
        &self,
        cloudformation: &aws_sdk_cloudformation::Client,
        parameter_map: &ParameterMap
    ) -> RemoteOperation {
        let client_request_token = ClientRequestToken::generate();

        let stack_id = StackId(
            cloudformation
                .create_stack()
                .stack_name(&self.name)
                .set_parameters(Some(self.create_parameters(parameter_map)))
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

    async fn start_update(
        &self,
        cloudformation: &aws_sdk_cloudformation::Client,
        parameter_map: &ParameterMap,
        existing_stack: aws_sdk_cloudformation::types::Stack,
    ) -> RemoteOperation {
        todo!();
    }

    async fn read_stack_status(
        cloudformation: &aws_sdk_cloudformation::Client,
        stack_id: &StackId,
    ) -> aws_sdk_cloudformation::types::StackStatus {
        try_load_stack(cloudformation, stack_id)
            .await
            .unwrap()
            .stack_status
            .unwrap()
    }

    fn create_parameters(&self, parameter_map: &ParameterMap) -> Vec<aws_sdk_cloudformation::types::Parameter> {
        self.parameter_map.merge(parameter_map).to_create_parameters()
    }

    fn capabilities(&self) -> Vec<aws_sdk_cloudformation::types::Capability> {
        self.capabilities.iter().cloned().collect()
    }

    fn template_body(&self) -> &String {
        match &self.template {
            Template::Static(content) => content,
        }
    }
}

pub struct StaticRegistry(pub Vec<InstanceSpec>);

pub trait Registry {
    fn find(&self, instance_name: &StackName) -> Option<&InstanceSpec>;
}

impl Registry for StaticRegistry {
    fn find(&self, instance_name: &StackName) -> Option<&InstanceSpec> {
        self.0
            .iter()
            .find(|&instance_spec| instance_spec.name == *instance_name)
    }
}

pub async fn try_load_stack(
    cloudformation: &aws_sdk_cloudformation::client::Client,
    stack_identifier: impl Into<String>,
) -> Option<aws_sdk_cloudformation::types::Stack> {
    Some(
        cloudformation
            .describe_stacks()
            .stack_name(stack_identifier)
            .send()
            .await
            .unwrap()
            .stacks
            .unwrap()[0]
            .clone(),
    )
}
