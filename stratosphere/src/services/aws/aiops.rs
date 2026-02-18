pub mod investigationgroup {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-aiops-investigationgroup-chatbotnotificationchannel.html>
    pub struct ChatbotNotificationChannel_ {
        pub chat_configuration_arns: Option<Vec<crate::value::ExpString>>,
        pub sns_topic_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_aiops_InvestigationGroup_ChatbotNotificationChannel {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AIOps::InvestigationGroup.ChatbotNotificationChannel"
            $($field $value)*)
        };
    }
    pub use crate::__aws_aiops_InvestigationGroup_ChatbotNotificationChannel as ChatbotNotificationChannel;
    impl crate::value::ToValue for ChatbotNotificationChannel_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.chat_configuration_arns {
                properties.insert(
                    "ChatConfigurationArns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sns_topic_arn {
                properties.insert(
                    "SNSTopicArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-aiops-investigationgroup-crossaccountconfiguration.html>
    pub struct CrossAccountConfiguration_ {
        pub source_role_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_aiops_InvestigationGroup_CrossAccountConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AIOps::InvestigationGroup.CrossAccountConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_aiops_InvestigationGroup_CrossAccountConfiguration as CrossAccountConfiguration;
    impl crate::value::ToValue for CrossAccountConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.source_role_arn {
                properties.insert(
                    "SourceRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-aiops-investigationgroup-encryptionconfigmap.html>
    pub struct EncryptionConfigMap_ {
        pub encryption_configuration_type: Option<crate::value::ExpString>,
        pub kms_key_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_aiops_InvestigationGroup_EncryptionConfigMap {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AIOps::InvestigationGroup.EncryptionConfigMap"
            $($field $value)*)
        };
    }
    pub use crate::__aws_aiops_InvestigationGroup_EncryptionConfigMap as EncryptionConfigMap;
    impl crate::value::ToValue for EncryptionConfigMap_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.encryption_configuration_type {
                properties.insert(
                    "EncryptionConfigurationType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-aiops-investigationgroup.html>
pub struct InvestigationGroup_ {
    pub chatbot_notification_channels:
        Option<Vec<super::aiops::investigationgroup::ChatbotNotificationChannel_>>,
    pub cross_account_configurations:
        Option<Vec<super::aiops::investigationgroup::CrossAccountConfiguration_>>,
    pub encryption_config: Option<super::aiops::investigationgroup::EncryptionConfigMap_>,
    pub investigation_group_policy: Option<crate::value::ExpString>,
    pub is_cloud_trail_event_history_enabled: Option<crate::value::ExpBool>,
    pub name: crate::value::ExpString,
    pub retention_in_days: Option<i32>,
    pub role_arn: Option<crate::value::ExpString>,
    pub tag_key_boundaries: Option<Vec<crate::value::ExpString>>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_aiops_InvestigationGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::AIOps::InvestigationGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_aiops_InvestigationGroup as InvestigationGroup;
impl crate::template::ToResource for InvestigationGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AIOps"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("InvestigationGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.chatbot_notification_channels {
            properties.insert(
                "ChatbotNotificationChannels".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cross_account_configurations {
            properties.insert(
                "CrossAccountConfigurations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.encryption_config {
            properties.insert(
                "EncryptionConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.investigation_group_policy {
            properties.insert(
                "InvestigationGroupPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.is_cloud_trail_event_history_enabled {
            properties.insert(
                "IsCloudTrailEventHistoryEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.retention_in_days {
            properties.insert(
                "RetentionInDays".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.role_arn {
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tag_key_boundaries {
            properties.insert(
                "TagKeyBoundaries".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
