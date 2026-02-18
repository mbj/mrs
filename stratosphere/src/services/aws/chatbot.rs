pub mod customaction {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-chatbot-customaction-customactionattachment.html>
    pub struct CustomActionAttachment_ {
        pub button_text: Option<crate::value::ExpString>,
        pub criteria: Option<Vec<CustomActionAttachmentCriteria_>>,
        pub notification_type: Option<crate::value::ExpString>,
        pub variables: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_chatbot_CustomAction_CustomActionAttachment {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Chatbot::CustomAction.CustomActionAttachment"
            $($field $value)*)
        };
    }
    pub use crate::__aws_chatbot_CustomAction_CustomActionAttachment as CustomActionAttachment;
    impl crate::value::ToValue for CustomActionAttachment_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.button_text {
                properties.insert(
                    "ButtonText".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.criteria {
                properties.insert(
                    "Criteria".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.notification_type {
                properties.insert(
                    "NotificationType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.variables {
                properties.insert(
                    "Variables".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-chatbot-customaction-customactionattachmentcriteria.html>
    pub struct CustomActionAttachmentCriteria_ {
        pub operator: crate::value::ExpString,
        pub value: Option<crate::value::ExpString>,
        pub variable_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_chatbot_CustomAction_CustomActionAttachmentCriteria {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Chatbot::CustomAction.CustomActionAttachmentCriteria"
            $($field $value)*)
        };
    }
    pub use crate::__aws_chatbot_CustomAction_CustomActionAttachmentCriteria as CustomActionAttachmentCriteria;
    impl crate::value::ToValue for CustomActionAttachmentCriteria_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Operator".to_string(),
                crate::value::ToValue::to_value(&self.operator),
            );
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "VariableName".to_string(),
                crate::value::ToValue::to_value(&self.variable_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-chatbot-customaction-customactiondefinition.html>
    pub struct CustomActionDefinition_ {
        pub command_text: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_chatbot_CustomAction_CustomActionDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Chatbot::CustomAction.CustomActionDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_chatbot_CustomAction_CustomActionDefinition as CustomActionDefinition;
    impl crate::value::ToValue for CustomActionDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CommandText".to_string(),
                crate::value::ToValue::to_value(&self.command_text),
            );
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-chatbot-customaction.html>
pub struct CustomAction_ {
    pub action_name: crate::value::ExpString,
    pub alias_name: Option<crate::value::ExpString>,
    pub attachments: Option<Vec<super::chatbot::customaction::CustomActionAttachment_>>,
    pub definition: super::chatbot::customaction::CustomActionDefinition_,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_chatbot_CustomAction {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Chatbot::CustomAction"
        $($field $value)*)
    };
}
pub use crate::__aws_chatbot_CustomAction as CustomAction;
impl crate::template::ToResource for CustomAction_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Chatbot"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("CustomAction"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ActionName".to_string(),
            crate::value::ToValue::to_value(&self.action_name),
        );
        if let Some(ref value) = self.alias_name {
            properties.insert(
                "AliasName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.attachments {
            properties.insert(
                "Attachments".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Definition".to_string(),
            crate::value::ToValue::to_value(&self.definition),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-chatbot-microsoftteamschannelconfiguration.html>
pub struct MicrosoftTeamsChannelConfiguration_ {
    pub configuration_name: crate::value::ExpString,
    pub customization_resource_arns: Option<Vec<crate::value::ExpString>>,
    pub guardrail_policies: Option<Vec<crate::value::ExpString>>,
    pub iam_role_arn: crate::value::ExpString,
    pub logging_level: Option<crate::value::ExpString>,
    pub sns_topic_arns: Option<Vec<crate::value::ExpString>>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub team_id: crate::value::ExpString,
    pub teams_channel_id: crate::value::ExpString,
    pub teams_channel_name: Option<crate::value::ExpString>,
    pub teams_tenant_id: crate::value::ExpString,
    pub user_role_required: Option<crate::value::ExpBool>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_chatbot_MicrosoftTeamsChannelConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Chatbot::MicrosoftTeamsChannelConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_chatbot_MicrosoftTeamsChannelConfiguration as MicrosoftTeamsChannelConfiguration;
impl crate::template::ToResource for MicrosoftTeamsChannelConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Chatbot"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "MicrosoftTeamsChannelConfiguration",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ConfigurationName".to_string(),
            crate::value::ToValue::to_value(&self.configuration_name),
        );
        if let Some(ref value) = self.customization_resource_arns {
            properties.insert(
                "CustomizationResourceArns".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.guardrail_policies {
            properties.insert(
                "GuardrailPolicies".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "IamRoleArn".to_string(),
            crate::value::ToValue::to_value(&self.iam_role_arn),
        );
        if let Some(ref value) = self.logging_level {
            properties.insert(
                "LoggingLevel".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sns_topic_arns {
            properties.insert(
                "SnsTopicArns".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TeamId".to_string(),
            crate::value::ToValue::to_value(&self.team_id),
        );
        properties.insert(
            "TeamsChannelId".to_string(),
            crate::value::ToValue::to_value(&self.teams_channel_id),
        );
        if let Some(ref value) = self.teams_channel_name {
            properties.insert(
                "TeamsChannelName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "TeamsTenantId".to_string(),
            crate::value::ToValue::to_value(&self.teams_tenant_id),
        );
        if let Some(ref value) = self.user_role_required {
            properties.insert(
                "UserRoleRequired".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-chatbot-slackchannelconfiguration.html>
pub struct SlackChannelConfiguration_ {
    pub configuration_name: crate::value::ExpString,
    pub customization_resource_arns: Option<Vec<crate::value::ExpString>>,
    pub guardrail_policies: Option<Vec<crate::value::ExpString>>,
    pub iam_role_arn: crate::value::ExpString,
    pub logging_level: Option<crate::value::ExpString>,
    pub slack_channel_id: crate::value::ExpString,
    pub slack_workspace_id: crate::value::ExpString,
    pub sns_topic_arns: Option<Vec<crate::value::ExpString>>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub user_role_required: Option<crate::value::ExpBool>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_chatbot_SlackChannelConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Chatbot::SlackChannelConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_chatbot_SlackChannelConfiguration as SlackChannelConfiguration;
impl crate::template::ToResource for SlackChannelConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Chatbot"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SlackChannelConfiguration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ConfigurationName".to_string(),
            crate::value::ToValue::to_value(&self.configuration_name),
        );
        if let Some(ref value) = self.customization_resource_arns {
            properties.insert(
                "CustomizationResourceArns".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.guardrail_policies {
            properties.insert(
                "GuardrailPolicies".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "IamRoleArn".to_string(),
            crate::value::ToValue::to_value(&self.iam_role_arn),
        );
        if let Some(ref value) = self.logging_level {
            properties.insert(
                "LoggingLevel".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SlackChannelId".to_string(),
            crate::value::ToValue::to_value(&self.slack_channel_id),
        );
        properties.insert(
            "SlackWorkspaceId".to_string(),
            crate::value::ToValue::to_value(&self.slack_workspace_id),
        );
        if let Some(ref value) = self.sns_topic_arns {
            properties.insert(
                "SnsTopicArns".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.user_role_required {
            properties.insert(
                "UserRoleRequired".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
