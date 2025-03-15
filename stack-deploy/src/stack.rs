use crate::types::{OutputKey, StackId, StackName};

pub(crate) async fn try_load_stack(
    cloudformation: &aws_sdk_cloudformation::client::Client,
    stack_identifier: impl Into<String>,
) -> Option<aws_sdk_cloudformation::types::Stack> {
    let stack_identifier = stack_identifier.into();

    let response = cloudformation
        .describe_stacks()
        .stack_name(&stack_identifier)
        .send()
        .await;

    match response {
        Ok(output) => output.stacks.unwrap().first().cloned(),
        Err(error) => {
            let service_error = error.into_service_error();
            let meta = service_error.meta();
            let expected_message = format!("Stack with id {} does not exist", &stack_identifier);

            match meta.code() {
                // There is no nicer way I know to check if a stack exist.
                Some("ValidationError") if meta.message() == Some(&expected_message) => None,
                _ => panic!("unexpected service error: {service_error:#?}"),
            }
        }
    }
}

pub(crate) async fn fetch_stack(
    cloudformation: &aws_sdk_cloudformation::client::Client,
    stack_identifier: &String,
) -> aws_sdk_cloudformation::types::Stack {
    try_load_stack(cloudformation, stack_identifier)
        .await
        .unwrap_or_else(|| panic!("stack: {:#?} does not exist!", stack_identifier))
}

pub(crate) async fn fetch_stack_id(
    cloudformation: &aws_sdk_cloudformation::client::Client,
    stack_identifier: &String,
) -> StackId {
    StackId(
        fetch_stack(cloudformation, stack_identifier)
            .await
            .stack_id
            .unwrap(),
    )
}

pub async fn fetch_stack_output(
    cloudformation: &aws_sdk_cloudformation::client::Client,
    stack_name: &StackName,
    output_key: &OutputKey,
) -> String {
    fetch_stack(cloudformation, &stack_name.0)
        .await
        .outputs()
        .iter()
        .find(|output| output.output_key().unwrap() == output_key.0)
        .unwrap_or_else(|| panic!("stack: {} missing output: {}", stack_name.0, output_key.0))
        .output_value()
        .unwrap()
        .to_string()
}
