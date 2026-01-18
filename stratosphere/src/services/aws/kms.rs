///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kms-alias.html
pub struct Alias_ {
    pub alias_name: crate::value::ExpString,
    pub target_key_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_kms_Alias {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::KMS::Alias" $($field
        $value)*)
    };
}
pub use crate::__aws_kms_Alias as Alias;
impl crate::template::ToResource for Alias_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("KMS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Alias"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AliasName".to_string(),
            crate::value::ToValue::to_value(&self.alias_name),
        );
        properties.insert(
            "TargetKeyId".to_string(),
            crate::value::ToValue::to_value(&self.target_key_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kms-key.html
pub struct Key_ {
    pub bypass_policy_lockout_safety_check: Option<crate::value::ExpBool>,
    pub description: Option<crate::value::ExpString>,
    pub enable_key_rotation: Option<crate::value::ExpBool>,
    pub enabled: Option<crate::value::ExpBool>,
    pub key_policy: Option<serde_json::Value>,
    pub key_spec: Option<crate::value::ExpString>,
    pub key_usage: Option<crate::value::ExpString>,
    pub multi_region: Option<crate::value::ExpBool>,
    pub origin: Option<crate::value::ExpString>,
    pub pending_window_in_days: Option<i64>,
    pub rotation_period_in_days: Option<i64>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_kms_Key {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::KMS::Key" $($field
        $value)*)
    };
}
pub use crate::__aws_kms_Key as Key;
impl crate::template::ToResource for Key_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("KMS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Key"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.bypass_policy_lockout_safety_check {
            properties.insert(
                "BypassPolicyLockoutSafetyCheck".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enable_key_rotation {
            properties.insert(
                "EnableKeyRotation".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enabled {
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.key_policy {
            properties.insert(
                "KeyPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.key_spec {
            properties.insert(
                "KeySpec".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.key_usage {
            properties.insert(
                "KeyUsage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.multi_region {
            properties.insert(
                "MultiRegion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.origin {
            properties.insert("Origin".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.pending_window_in_days {
            properties.insert(
                "PendingWindowInDays".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.rotation_period_in_days {
            properties.insert(
                "RotationPeriodInDays".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kms-replicakey.html
pub struct ReplicaKey_ {
    pub description: Option<crate::value::ExpString>,
    pub enabled: Option<crate::value::ExpBool>,
    pub key_policy: serde_json::Value,
    pub pending_window_in_days: Option<i64>,
    pub primary_key_arn: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_kms_ReplicaKey {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::KMS::ReplicaKey" $($field
        $value)*)
    };
}
pub use crate::__aws_kms_ReplicaKey as ReplicaKey;
impl crate::template::ToResource for ReplicaKey_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("KMS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ReplicaKey"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enabled {
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "KeyPolicy".to_string(),
            crate::value::ToValue::to_value(&self.key_policy),
        );
        if let Some(ref value) = self.pending_window_in_days {
            properties.insert(
                "PendingWindowInDays".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "PrimaryKeyArn".to_string(),
            crate::value::ToValue::to_value(&self.primary_key_arn),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
