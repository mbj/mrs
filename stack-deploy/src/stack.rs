use crate::types::{OutputKey, StackId, StackIdentifier, StackName};

pub(crate) async fn try_load_stack<T: StackIdentifier>(
    cloudformation: &aws_sdk_cloudformation::client::Client,
    stack_identifier: &T,
) -> Option<aws_sdk_cloudformation::types::Stack> {
    let response = cloudformation
        .describe_stacks()
        .stack_name(stack_identifier.as_str())
        .send()
        .await;

    match response {
        Ok(output) => output.stacks.unwrap().first().cloned(),
        Err(error) => {
            let service_error = error.into_service_error();
            let meta = service_error.meta();
            let expected_message =
                format!("Stack with id {} does not exist", stack_identifier.as_str());

            match meta.code() {
                // There is no nicer way I know to check if a stack exist.
                Some("ValidationError") if meta.message() == Some(&expected_message) => None,
                _ => panic!("unexpected service error: {service_error:#?}"),
            }
        }
    }
}

pub async fn load_stack<T: StackIdentifier>(
    cloudformation: &aws_sdk_cloudformation::client::Client,
    stack_identifier: &T,
) -> aws_sdk_cloudformation::types::Stack {
    try_load_stack(cloudformation, stack_identifier)
        .await
        .unwrap_or_else(|| panic!("stack: {stack_identifier:#?} does not exist!"))
}

pub(crate) async fn load_stack_id(
    cloudformation: &aws_sdk_cloudformation::client::Client,
    stack_name: &StackName,
) -> StackId {
    StackId(
        load_stack(cloudformation, &stack_name)
            .await
            .stack_id
            .unwrap(),
    )
}

pub async fn read_stack_output<T: StackIdentifier>(
    cloudformation: &aws_sdk_cloudformation::client::Client,
    stack_identifier: &T,
    output_key: &OutputKey,
) -> String {
    log::info!("Reading stack output, stack: {stack_identifier} output: {output_key}");

    load_stack(cloudformation, stack_identifier)
        .await
        .outputs()
        .iter()
        .find(|output| output.output_key().unwrap() == output_key.0)
        .unwrap_or_else(|| {
            panic!(
                "stack: {:#?} missing output: {}",
                stack_identifier, output_key.0
            )
        })
        .output_value()
        .unwrap()
        .to_string()
}

pub fn fetch_stack_output(
    stack: &aws_sdk_cloudformation::types::Stack,
    output_key: &OutputKey,
) -> String {
    stack
        .outputs()
        .iter()
        .find(|output| output.output_key().unwrap() == output_key.0)
        .unwrap_or_else(|| {
            panic!(
                "stack: {} missing output: {}",
                stack.stack_name.as_ref().unwrap(),
                output_key.0
            )
        })
        .output_value()
        .unwrap()
        .to_string()
}
