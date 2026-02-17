pub mod app {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resiliencehub-app-eventsubscription.html>
    pub struct EventSubscription_ {
        pub event_type: crate::value::ExpString,
        pub name: crate::value::ExpString,
        pub sns_topic_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_resiliencehub_App_EventSubscription {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ResilienceHub::App.EventSubscription"
            $($field $value)*)
        };
    }
    pub use crate::__aws_resiliencehub_App_EventSubscription as EventSubscription;
    impl crate::value::ToValue for EventSubscription_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EventType".to_string(),
                crate::value::ToValue::to_value(&self.event_type),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.sns_topic_arn {
                properties.insert(
                    "SnsTopicArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resiliencehub-app-permissionmodel.html>
    pub struct PermissionModel_ {
        pub cross_account_role_arns: Option<Vec<crate::value::ExpString>>,
        pub invoker_role_name: Option<crate::value::ExpString>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_resiliencehub_App_PermissionModel {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ResilienceHub::App.PermissionModel"
            $($field $value)*)
        };
    }
    pub use crate::__aws_resiliencehub_App_PermissionModel as PermissionModel;
    impl crate::value::ToValue for PermissionModel_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cross_account_role_arns {
                properties.insert(
                    "CrossAccountRoleArns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.invoker_role_name {
                properties.insert(
                    "InvokerRoleName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resiliencehub-app-physicalresourceid.html>
    pub struct PhysicalResourceId_ {
        pub aws_account_id: Option<crate::value::ExpString>,
        pub aws_region: Option<crate::value::ExpString>,
        pub identifier: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_resiliencehub_App_PhysicalResourceId {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ResilienceHub::App.PhysicalResourceId"
            $($field $value)*)
        };
    }
    pub use crate::__aws_resiliencehub_App_PhysicalResourceId as PhysicalResourceId;
    impl crate::value::ToValue for PhysicalResourceId_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.aws_account_id {
                properties.insert(
                    "AwsAccountId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.aws_region {
                properties.insert(
                    "AwsRegion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Identifier".to_string(),
                crate::value::ToValue::to_value(&self.identifier),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resiliencehub-app-resourcemapping.html>
    pub struct ResourceMapping_ {
        pub eks_source_name: Option<crate::value::ExpString>,
        pub logical_stack_name: Option<crate::value::ExpString>,
        pub mapping_type: crate::value::ExpString,
        pub physical_resource_id: Box<PhysicalResourceId_>,
        pub resource_name: Option<crate::value::ExpString>,
        pub terraform_source_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_resiliencehub_App_ResourceMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ResilienceHub::App.ResourceMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_resiliencehub_App_ResourceMapping as ResourceMapping;
    impl crate::value::ToValue for ResourceMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.eks_source_name {
                properties.insert(
                    "EksSourceName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.logical_stack_name {
                properties.insert(
                    "LogicalStackName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MappingType".to_string(),
                crate::value::ToValue::to_value(&self.mapping_type),
            );
            properties.insert(
                "PhysicalResourceId".to_string(),
                crate::value::ToValue::to_value(&self.physical_resource_id),
            );
            if let Some(ref value) = self.resource_name {
                properties.insert(
                    "ResourceName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.terraform_source_name {
                properties.insert(
                    "TerraformSourceName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod resiliencypolicy {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resiliencehub-resiliencypolicy-failurepolicy.html>
    pub struct FailurePolicy_ {
        pub rpo_in_secs: i32,
        pub rto_in_secs: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_resiliencehub_ResiliencyPolicy_FailurePolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ResilienceHub::ResiliencyPolicy.FailurePolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_resiliencehub_ResiliencyPolicy_FailurePolicy as FailurePolicy;
    impl crate::value::ToValue for FailurePolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RpoInSecs".to_string(),
                crate::value::ToValue::to_value(&self.rpo_in_secs),
            );
            properties.insert(
                "RtoInSecs".to_string(),
                crate::value::ToValue::to_value(&self.rto_in_secs),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resiliencehub-resiliencypolicy-policymap.html>
    pub struct PolicyMap_ {
        pub az: Box<FailurePolicy_>,
        pub hardware: Box<FailurePolicy_>,
        pub region: Option<Box<FailurePolicy_>>,
        pub software: Box<FailurePolicy_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_resiliencehub_ResiliencyPolicy_PolicyMap {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ResilienceHub::ResiliencyPolicy.PolicyMap"
            $($field $value)*)
        };
    }
    pub use crate::__aws_resiliencehub_ResiliencyPolicy_PolicyMap as PolicyMap;
    impl crate::value::ToValue for PolicyMap_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert("AZ".to_string(), crate::value::ToValue::to_value(&self.az));
            properties.insert(
                "Hardware".to_string(),
                crate::value::ToValue::to_value(&self.hardware),
            );
            if let Some(ref value) = self.region {
                properties.insert("Region".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Software".to_string(),
                crate::value::ToValue::to_value(&self.software),
            );
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-resiliencehub-app.html>
pub struct App_ {
    pub app_assessment_schedule: Option<crate::value::ExpString>,
    pub app_template_body: crate::value::ExpString,
    pub description: Option<crate::value::ExpString>,
    pub event_subscriptions: Option<Vec<super::resiliencehub::app::EventSubscription_>>,
    pub name: crate::value::ExpString,
    pub permission_model: Option<super::resiliencehub::app::PermissionModel_>,
    pub resiliency_policy_arn: Option<crate::value::ExpString>,
    pub resource_mappings: Vec<super::resiliencehub::app::ResourceMapping_>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_resiliencehub_App {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ResilienceHub::App"
        $($field $value)*)
    };
}
pub use crate::__aws_resiliencehub_App as App;
impl crate::template::ToResource for App_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ResilienceHub"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("App"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.app_assessment_schedule {
            properties.insert(
                "AppAssessmentSchedule".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "AppTemplateBody".to_string(),
            crate::value::ToValue::to_value(&self.app_template_body),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.event_subscriptions {
            properties.insert(
                "EventSubscriptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.permission_model {
            properties.insert(
                "PermissionModel".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resiliency_policy_arn {
            properties.insert(
                "ResiliencyPolicyArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ResourceMappings".to_string(),
            crate::value::ToValue::to_value(&self.resource_mappings),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-resiliencehub-resiliencypolicy.html>
pub struct ResiliencyPolicy_ {
    pub data_location_constraint: Option<crate::value::ExpString>,
    pub policy: super::resiliencehub::resiliencypolicy::PolicyMap_,
    pub policy_description: Option<crate::value::ExpString>,
    pub policy_name: crate::value::ExpString,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub tier: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_resiliencehub_ResiliencyPolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ResilienceHub::ResiliencyPolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_resiliencehub_ResiliencyPolicy as ResiliencyPolicy;
impl crate::template::ToResource for ResiliencyPolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ResilienceHub"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ResiliencyPolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.data_location_constraint {
            properties.insert(
                "DataLocationConstraint".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Policy".to_string(),
            crate::value::ToValue::to_value(&self.policy),
        );
        if let Some(ref value) = self.policy_description {
            properties.insert(
                "PolicyDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "PolicyName".to_string(),
            crate::value::ToValue::to_value(&self.policy_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Tier".to_string(),
            crate::value::ToValue::to_value(&self.tier),
        );
        properties
    }
}
