use crate::value::{AWS_ACCOUNT_ID, AWS_PARTITION, AWS_REGION, ExpString};

fn build_arn(log_group_name: ExpString, with_streams: bool) -> ExpString {
    let mut parts: Vec<ExpString> = vec![
        "arn".into(),
        AWS_PARTITION,
        "logs".into(),
        AWS_REGION,
        AWS_ACCOUNT_ID,
        "log-group".into(),
        log_group_name,
    ];

    if with_streams {
        parts.push("*".into());
    }

    crate::value::join(":", parts)
}

/// Constructs the ARN for a CloudWatch log group.
///
/// The ARN follows the pattern:
/// `arn:{partition}:logs:{region}:{account-id}:log-group:{log-group-name}`
///
/// This uses CloudFormation pseudo parameters for partition, region, and account ID
/// to ensure the ARN is correct in any AWS environment.
#[must_use]
pub fn log_group_arn(log_group_name: impl Into<ExpString>) -> ExpString {
    build_arn(log_group_name.into(), false)
}

/// Constructs the ARN pattern for all log streams in a CloudWatch log group.
///
/// The ARN follows the pattern:
/// `arn:{partition}:logs:{region}:{account-id}:log-group:{log-group-name}:*`
///
/// This is useful for IAM policies that need to grant permissions to write log streams.
#[must_use]
pub fn log_group_streams_arn(log_group_name: impl Into<ExpString>) -> ExpString {
    build_arn(log_group_name.into(), true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::ToValue;
    use serde_json::json;

    #[test]
    fn test_log_group_arn_literal() {
        let arn = log_group_arn("/my/log/group");
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
                    "/my/log/group"
                ]]
            })
        );
    }

    #[test]
    fn test_log_group_arn_ref() {
        let log_group_name = ExpString::Ref("LogGroupNameParameter".into());
        let arn = log_group_arn(log_group_name);
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
                    {"Ref": "LogGroupNameParameter"}
                ]]
            })
        );
    }

    #[test]
    fn test_log_group_streams_arn_literal() {
        let arn = log_group_streams_arn("/my/log/group");
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
                    "/my/log/group",
                    "*"
                ]]
            })
        );
    }

    #[test]
    fn test_log_group_streams_arn_ref() {
        let log_group_name = ExpString::Ref("LogGroupNameParameter".into());
        let arn = log_group_streams_arn(log_group_name);
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
                    {"Ref": "LogGroupNameParameter"},
                    "*"
                ]]
            })
        );
    }
}
