pub mod replicationset {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmincidents-replicationset-regionconfiguration.html>
    pub struct RegionConfiguration_ {
        pub sse_kms_key_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssmincidents_ReplicationSet_RegionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SSMIncidents::ReplicationSet.RegionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssmincidents_ReplicationSet_RegionConfiguration as RegionConfiguration;
    impl crate::value::ToValue for RegionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SseKmsKeyId".to_string(),
                crate::value::ToValue::to_value(&self.sse_kms_key_id),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmincidents-replicationset-replicationregion.html>
    pub struct ReplicationRegion_ {
        pub region_configuration: Option<Box<RegionConfiguration_>>,
        pub region_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssmincidents_ReplicationSet_ReplicationRegion {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SSMIncidents::ReplicationSet.ReplicationRegion"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssmincidents_ReplicationSet_ReplicationRegion as ReplicationRegion;
    impl crate::value::ToValue for ReplicationRegion_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.region_configuration {
                properties.insert(
                    "RegionConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.region_name {
                properties.insert(
                    "RegionName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod responseplan {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmincidents-responseplan-action.html>
    pub struct Action_ {
        pub ssm_automation: Option<Box<SsmAutomation_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssmincidents_ResponsePlan_Action {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SSMIncidents::ResponsePlan.Action"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssmincidents_ResponsePlan_Action as Action;
    impl crate::value::ToValue for Action_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ssm_automation {
                properties.insert(
                    "SsmAutomation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmincidents-responseplan-chatchannel.html>
    pub struct ChatChannel_ {
        pub chatbot_sns: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssmincidents_ResponsePlan_ChatChannel {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SSMIncidents::ResponsePlan.ChatChannel"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssmincidents_ResponsePlan_ChatChannel as ChatChannel;
    impl crate::value::ToValue for ChatChannel_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.chatbot_sns {
                properties.insert(
                    "ChatbotSns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmincidents-responseplan-dynamicssmparameter.html>
    pub struct DynamicSsmParameter_ {
        pub key: crate::value::ExpString,
        pub value: Box<DynamicSsmParameterValue_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssmincidents_ResponsePlan_DynamicSsmParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SSMIncidents::ResponsePlan.DynamicSsmParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssmincidents_ResponsePlan_DynamicSsmParameter as DynamicSsmParameter;
    impl crate::value::ToValue for DynamicSsmParameter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmincidents-responseplan-dynamicssmparametervalue.html>
    pub struct DynamicSsmParameterValue_ {
        pub variable: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssmincidents_ResponsePlan_DynamicSsmParameterValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SSMIncidents::ResponsePlan.DynamicSsmParameterValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssmincidents_ResponsePlan_DynamicSsmParameterValue as DynamicSsmParameterValue;
    impl crate::value::ToValue for DynamicSsmParameterValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.variable {
                properties.insert(
                    "Variable".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmincidents-responseplan-incidenttemplate.html>
    pub struct IncidentTemplate_ {
        pub dedupe_string: Option<crate::value::ExpString>,
        pub impact: i32,
        pub incident_tags: Option<Vec<crate::Tag_>>,
        pub notification_targets: Option<Vec<NotificationTargetItem_>>,
        pub summary: Option<crate::value::ExpString>,
        pub title: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssmincidents_ResponsePlan_IncidentTemplate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SSMIncidents::ResponsePlan.IncidentTemplate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssmincidents_ResponsePlan_IncidentTemplate as IncidentTemplate;
    impl crate::value::ToValue for IncidentTemplate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dedupe_string {
                properties.insert(
                    "DedupeString".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Impact".to_string(),
                crate::value::ToValue::to_value(&self.impact),
            );
            if let Some(ref value) = self.incident_tags {
                properties.insert(
                    "IncidentTags".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.notification_targets {
                properties.insert(
                    "NotificationTargets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.summary {
                properties.insert(
                    "Summary".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Title".to_string(),
                crate::value::ToValue::to_value(&self.title),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmincidents-responseplan-integration.html>
    pub struct Integration_ {
        pub pager_duty_configuration: Box<PagerDutyConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssmincidents_ResponsePlan_Integration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SSMIncidents::ResponsePlan.Integration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssmincidents_ResponsePlan_Integration as Integration;
    impl crate::value::ToValue for Integration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PagerDutyConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.pager_duty_configuration),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmincidents-responseplan-notificationtargetitem.html>
    pub struct NotificationTargetItem_ {
        pub sns_topic_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssmincidents_ResponsePlan_NotificationTargetItem {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SSMIncidents::ResponsePlan.NotificationTargetItem"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssmincidents_ResponsePlan_NotificationTargetItem as NotificationTargetItem;
    impl crate::value::ToValue for NotificationTargetItem_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.sns_topic_arn {
                properties.insert(
                    "SnsTopicArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmincidents-responseplan-pagerdutyconfiguration.html>
    pub struct PagerDutyConfiguration_ {
        pub name: crate::value::ExpString,
        pub pager_duty_incident_configuration: Box<PagerDutyIncidentConfiguration_>,
        pub secret_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssmincidents_ResponsePlan_PagerDutyConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SSMIncidents::ResponsePlan.PagerDutyConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssmincidents_ResponsePlan_PagerDutyConfiguration as PagerDutyConfiguration;
    impl crate::value::ToValue for PagerDutyConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "PagerDutyIncidentConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.pager_duty_incident_configuration),
            );
            properties.insert(
                "SecretId".to_string(),
                crate::value::ToValue::to_value(&self.secret_id),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmincidents-responseplan-pagerdutyincidentconfiguration.html>
    pub struct PagerDutyIncidentConfiguration_ {
        pub service_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssmincidents_ResponsePlan_PagerDutyIncidentConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SSMIncidents::ResponsePlan.PagerDutyIncidentConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssmincidents_ResponsePlan_PagerDutyIncidentConfiguration as PagerDutyIncidentConfiguration;
    impl crate::value::ToValue for PagerDutyIncidentConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ServiceId".to_string(),
                crate::value::ToValue::to_value(&self.service_id),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmincidents-responseplan-ssmautomation.html>
    pub struct SsmAutomation_ {
        pub document_name: crate::value::ExpString,
        pub document_version: Option<crate::value::ExpString>,
        pub dynamic_parameters: Option<Vec<DynamicSsmParameter_>>,
        pub parameters: Option<Vec<SsmParameter_>>,
        pub role_arn: crate::value::ExpString,
        pub target_account: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssmincidents_ResponsePlan_SsmAutomation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SSMIncidents::ResponsePlan.SsmAutomation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssmincidents_ResponsePlan_SsmAutomation as SsmAutomation;
    impl crate::value::ToValue for SsmAutomation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DocumentName".to_string(),
                crate::value::ToValue::to_value(&self.document_name),
            );
            if let Some(ref value) = self.document_version {
                properties.insert(
                    "DocumentVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dynamic_parameters {
                properties.insert(
                    "DynamicParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parameters {
                properties.insert(
                    "Parameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            if let Some(ref value) = self.target_account {
                properties.insert(
                    "TargetAccount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmincidents-responseplan-ssmparameter.html>
    pub struct SsmParameter_ {
        pub key: crate::value::ExpString,
        pub values: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssmincidents_ResponsePlan_SsmParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SSMIncidents::ResponsePlan.SsmParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssmincidents_ResponsePlan_SsmParameter as SsmParameter;
    impl crate::value::ToValue for SsmParameter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            properties.insert(
                "Values".to_string(),
                crate::value::ToValue::to_value(&self.values),
            );
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmincidents-replicationset.html>
pub struct ReplicationSet_ {
    pub deletion_protected: Option<crate::value::ExpBool>,
    pub regions: Vec<super::ssmincidents::replicationset::ReplicationRegion_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ssmincidents_ReplicationSet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SSMIncidents::ReplicationSet"
        $($field $value)*)
    };
}
pub use crate::__aws_ssmincidents_ReplicationSet as ReplicationSet;
impl crate::template::ToResource for ReplicationSet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SSMIncidents"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ReplicationSet"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.deletion_protected {
            properties.insert(
                "DeletionProtected".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Regions".to_string(),
            crate::value::ToValue::to_value(&self.regions),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmincidents-responseplan.html>
pub struct ResponsePlan_ {
    pub actions: Option<Vec<super::ssmincidents::responseplan::Action_>>,
    pub chat_channel: Option<super::ssmincidents::responseplan::ChatChannel_>,
    pub display_name: Option<crate::value::ExpString>,
    pub engagements: Option<Vec<crate::value::ExpString>>,
    pub incident_template: super::ssmincidents::responseplan::IncidentTemplate_,
    pub integrations: Option<Vec<super::ssmincidents::responseplan::Integration_>>,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ssmincidents_ResponsePlan {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SSMIncidents::ResponsePlan"
        $($field $value)*)
    };
}
pub use crate::__aws_ssmincidents_ResponsePlan as ResponsePlan;
impl crate::template::ToResource for ResponsePlan_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SSMIncidents"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ResponsePlan"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.actions {
            properties.insert(
                "Actions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.chat_channel {
            properties.insert(
                "ChatChannel".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.display_name {
            properties.insert(
                "DisplayName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.engagements {
            properties.insert(
                "Engagements".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "IncidentTemplate".to_string(),
            crate::value::ToValue::to_value(&self.incident_template),
        );
        if let Some(ref value) = self.integrations {
            properties.insert(
                "Integrations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
