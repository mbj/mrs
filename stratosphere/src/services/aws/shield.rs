pub mod proactiveengagement {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-shield-proactiveengagement-emergencycontact.html
    pub struct EmergencyContact_ {
        pub contact_notes: Option<crate::value::ExpString>,
        pub email_address: crate::value::ExpString,
        pub phone_number: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_shield_ProactiveEngagement_EmergencyContact {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Shield::ProactiveEngagement.EmergencyContact"
            $($field $value)*)
        };
    }
    pub use crate::__aws_shield_ProactiveEngagement_EmergencyContact as EmergencyContact;
    impl crate::value::ToValue for EmergencyContact_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.contact_notes {
                properties.insert(
                    "ContactNotes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "EmailAddress".to_string(),
                crate::value::ToValue::to_value(&self.email_address),
            );
            if let Some(ref value) = self.phone_number {
                properties.insert(
                    "PhoneNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod protection {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-shield-protection-action.html
    pub struct Action_ {
        pub block: Option<serde_json::Value>,
        pub count: Option<serde_json::Value>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_shield_Protection_Action {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Shield::Protection.Action"
            $($field $value)*)
        };
    }
    pub use crate::__aws_shield_Protection_Action as Action;
    impl crate::value::ToValue for Action_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.block {
                properties.insert("Block".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.count {
                properties.insert("Count".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-shield-protection-applicationlayerautomaticresponseconfiguration.html
    pub struct ApplicationLayerAutomaticResponseConfiguration_ {
        pub action: Box<Action_>,
        pub status: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_shield_Protection_ApplicationLayerAutomaticResponseConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Shield::Protection.ApplicationLayerAutomaticResponseConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_shield_Protection_ApplicationLayerAutomaticResponseConfiguration as ApplicationLayerAutomaticResponseConfiguration;
    impl crate::value::ToValue for ApplicationLayerAutomaticResponseConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Action".to_string(),
                crate::value::ToValue::to_value(&self.action),
            );
            properties.insert(
                "Status".to_string(),
                crate::value::ToValue::to_value(&self.status),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-drtaccess.html
pub struct DRTAccess_ {
    pub log_bucket_list: Option<Vec<crate::value::ExpString>>,
    pub role_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_shield_DRTAccess {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Shield::DRTAccess"
        $($field $value)*)
    };
}
pub use crate::__aws_shield_DRTAccess as DRTAccess;
impl crate::template::ToResource for DRTAccess_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Shield"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DRTAccess"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.log_bucket_list {
            properties.insert(
                "LogBucketList".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-proactiveengagement.html
pub struct ProactiveEngagement_ {
    pub emergency_contact_list: Vec<super::shield::proactiveengagement::EmergencyContact_>,
    pub proactive_engagement_status: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_shield_ProactiveEngagement {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Shield::ProactiveEngagement"
        $($field $value)*)
    };
}
pub use crate::__aws_shield_ProactiveEngagement as ProactiveEngagement;
impl crate::template::ToResource for ProactiveEngagement_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Shield"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ProactiveEngagement"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "EmergencyContactList".to_string(),
            crate::value::ToValue::to_value(&self.emergency_contact_list),
        );
        properties.insert(
            "ProactiveEngagementStatus".to_string(),
            crate::value::ToValue::to_value(&self.proactive_engagement_status),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protection.html
pub struct Protection_ {
    pub application_layer_automatic_response_configuration:
        Option<super::shield::protection::ApplicationLayerAutomaticResponseConfiguration_>,
    pub health_check_arns: Option<Vec<crate::value::ExpString>>,
    pub name: crate::value::ExpString,
    pub resource_arn: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_shield_Protection {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Shield::Protection"
        $($field $value)*)
    };
}
pub use crate::__aws_shield_Protection as Protection;
impl crate::template::ToResource for Protection_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Shield"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Protection"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.application_layer_automatic_response_configuration {
            properties.insert(
                "ApplicationLayerAutomaticResponseConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.health_check_arns {
            properties.insert(
                "HealthCheckArns".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "ResourceArn".to_string(),
            crate::value::ToValue::to_value(&self.resource_arn),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-shield-protectiongroup.html
pub struct ProtectionGroup_ {
    pub aggregation: crate::value::ExpString,
    pub members: Option<Vec<crate::value::ExpString>>,
    pub pattern: crate::value::ExpString,
    pub protection_group_id: crate::value::ExpString,
    pub resource_type: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_shield_ProtectionGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Shield::ProtectionGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_shield_ProtectionGroup as ProtectionGroup;
impl crate::template::ToResource for ProtectionGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Shield"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ProtectionGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Aggregation".to_string(),
            crate::value::ToValue::to_value(&self.aggregation),
        );
        if let Some(ref value) = self.members {
            properties.insert(
                "Members".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Pattern".to_string(),
            crate::value::ToValue::to_value(&self.pattern),
        );
        properties.insert(
            "ProtectionGroupId".to_string(),
            crate::value::ToValue::to_value(&self.protection_group_id),
        );
        if let Some(ref value) = self.resource_type {
            properties.insert(
                "ResourceType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
