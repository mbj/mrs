///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-supportapp-accountalias.html>
pub struct AccountAlias_ {
    pub account_alias: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_supportapp_AccountAlias {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SupportApp::AccountAlias"
        $($field $value)*)
    };
}
pub use crate::__aws_supportapp_AccountAlias as AccountAlias;
impl crate::template::ToResource for AccountAlias_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SupportApp"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AccountAlias"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AccountAlias".to_string(),
            crate::value::ToValue::to_value(&self.account_alias),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-supportapp-slackchannelconfiguration.html>
pub struct SlackChannelConfiguration_ {
    pub channel_id: crate::value::ExpString,
    pub channel_name: Option<crate::value::ExpString>,
    pub channel_role_arn: crate::value::ExpString,
    pub notify_on_add_correspondence_to_case: Option<crate::value::ExpBool>,
    pub notify_on_case_severity: crate::value::ExpString,
    pub notify_on_create_or_reopen_case: Option<crate::value::ExpBool>,
    pub notify_on_resolve_case: Option<crate::value::ExpBool>,
    pub team_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_supportapp_SlackChannelConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SupportApp::SlackChannelConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_supportapp_SlackChannelConfiguration as SlackChannelConfiguration;
impl crate::template::ToResource for SlackChannelConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SupportApp"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SlackChannelConfiguration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ChannelId".to_string(),
            crate::value::ToValue::to_value(&self.channel_id),
        );
        if let Some(ref value) = self.channel_name {
            properties.insert(
                "ChannelName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ChannelRoleArn".to_string(),
            crate::value::ToValue::to_value(&self.channel_role_arn),
        );
        if let Some(ref value) = self.notify_on_add_correspondence_to_case {
            properties.insert(
                "NotifyOnAddCorrespondenceToCase".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "NotifyOnCaseSeverity".to_string(),
            crate::value::ToValue::to_value(&self.notify_on_case_severity),
        );
        if let Some(ref value) = self.notify_on_create_or_reopen_case {
            properties.insert(
                "NotifyOnCreateOrReopenCase".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.notify_on_resolve_case {
            properties.insert(
                "NotifyOnResolveCase".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "TeamId".to_string(),
            crate::value::ToValue::to_value(&self.team_id),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-supportapp-slackworkspaceconfiguration.html>
pub struct SlackWorkspaceConfiguration_ {
    pub team_id: crate::value::ExpString,
    pub version_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_supportapp_SlackWorkspaceConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SupportApp::SlackWorkspaceConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_supportapp_SlackWorkspaceConfiguration as SlackWorkspaceConfiguration;
impl crate::template::ToResource for SlackWorkspaceConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SupportApp"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "SlackWorkspaceConfiguration",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "TeamId".to_string(),
            crate::value::ToValue::to_value(&self.team_id),
        );
        if let Some(ref value) = self.version_id {
            properties.insert(
                "VersionId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
