use crate::value::ExpString;

/// Constructs the conventional CloudWatch log group name for a Lambda function.
///
/// Lambda functions by default write logs to `/aws/lambda/{function-name}`.
/// This helper constructs that path pattern using CloudFormation intrinsic functions,
/// allowing it to work with both literal function names and dynamic references.
#[must_use]
pub fn log_group_name(function_name: impl Into<ExpString>) -> ExpString {
    crate::value::join(
        "/",
        [
            "".into(),
            "aws".into(),
            "lambda".into(),
            function_name.into(),
        ],
    )
}

/// Constructs the ARN for a Lambda function's CloudWatch log group.
///
/// The ARN follows the pattern:
/// `arn:{partition}:logs:{region}:{account-id}:log-group:/aws/lambda/{function-name}`
///
/// This uses CloudFormation pseudo parameters for partition, region, and account ID
/// to ensure the ARN is correct in any AWS environment.
#[must_use]
pub fn log_group_arn(function_name: impl Into<ExpString>) -> ExpString {
    crate::logs::log_group_arn(log_group_name(function_name))
}

/// Constructs the ARN pattern for all log streams in a Lambda function's log group.
///
/// The ARN follows the pattern:
/// `arn:{partition}:logs:{region}:{account-id}:log-group:/aws/lambda/{function-name}:*`
///
/// This is useful for IAM policies that need to grant permissions to write log streams.
#[must_use]
pub fn log_group_streams_arn(function_name: impl Into<ExpString>) -> ExpString {
    crate::logs::log_group_streams_arn(log_group_name(function_name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::ToValue;
    use serde_json::json;

    #[test]
    fn test_log_group_name_literal() {
        let name = log_group_name("my-function");
        assert_eq!(
            name.to_value(),
            json!({"Fn::Join": ["/", ["", "aws", "lambda", "my-function"]]})
        );
    }

    #[test]
    fn test_log_group_name_ref() {
        let function_name = ExpString::Ref("FunctionNameParameter".into());
        let name = log_group_name(function_name);
        assert_eq!(
            name.to_value(),
            json!({"Fn::Join": ["/", ["", "aws", "lambda", {"Ref": "FunctionNameParameter"}]]})
        );
    }

    #[test]
    fn test_log_group_arn_literal() {
        let arn = log_group_arn("my-function");
        assert_eq!(
            arn.to_value(),
            json!({
                "Fn::Join": [":", [
                    "arn",
                    {"Ref": "AWS::Partition"},
                    "logs",
                    {"Ref": "AWS::Region"},
                    {"Ref": "AWS::AccountId"},
                    "log-group",
                    {"Fn::Join": ["/", ["", "aws", "lambda", "my-function"]]}
                ]]
            })
        );
    }

    #[test]
    fn test_log_group_arn_ref() {
        let function_name = ExpString::Ref("FunctionNameParameter".into());
        let arn = log_group_arn(function_name);
        assert_eq!(
            arn.to_value(),
            json!({
                "Fn::Join": [":", [
                    "arn",
                    {"Ref": "AWS::Partition"},
                    "logs",
                    {"Ref": "AWS::Region"},
                    {"Ref": "AWS::AccountId"},
                    "log-group",
                    {"Fn::Join": ["/", ["", "aws", "lambda", {"Ref": "FunctionNameParameter"}]]}
                ]]
            })
        );
    }

    #[test]
    fn test_log_group_streams_arn_literal() {
        let arn = log_group_streams_arn("my-function");
        assert_eq!(
            arn.to_value(),
            json!({
                "Fn::Join": [":", [
                    "arn",
                    {"Ref": "AWS::Partition"},
                    "logs",
                    {"Ref": "AWS::Region"},
                    {"Ref": "AWS::AccountId"},
                    "log-group",
                    {"Fn::Join": ["/", ["", "aws", "lambda", "my-function"]]},
                    "*"
                ]]
            })
        );
    }

    #[test]
    fn test_log_group_streams_arn_ref() {
        let function_name = ExpString::Ref("FunctionNameParameter".into());
        let arn = log_group_streams_arn(function_name);
        assert_eq!(
            arn.to_value(),
            json!({
                "Fn::Join": [":", [
                    "arn",
                    {"Ref": "AWS::Partition"},
                    "logs",
                    {"Ref": "AWS::Region"},
                    {"Ref": "AWS::AccountId"},
                    "log-group",
                    {"Fn::Join": ["/", ["", "aws", "lambda", {"Ref": "FunctionNameParameter"}]]},
                    "*"
                ]]
            })
        );
    }
}
