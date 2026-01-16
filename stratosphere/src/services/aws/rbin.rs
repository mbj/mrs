pub mod rule {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rbin-rule-resourcetag.html
    pub struct ResourceTag_ {
        pub resource_tag_key: crate::value::ExpString,
        pub resource_tag_value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rbin_Rule_ResourceTag {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Rbin::Rule.ResourceTag"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rbin_Rule_ResourceTag as ResourceTag;
    impl crate::value::ToValue for ResourceTag_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ResourceTagKey".to_string(),
                crate::value::ToValue::to_value(&self.resource_tag_key),
            );
            properties.insert(
                "ResourceTagValue".to_string(),
                crate::value::ToValue::to_value(&self.resource_tag_value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rbin-rule-retentionperiod.html
    pub struct RetentionPeriod_ {
        pub retention_period_unit: crate::value::ExpString,
        pub retention_period_value: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rbin_Rule_RetentionPeriod {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Rbin::Rule.RetentionPeriod"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rbin_Rule_RetentionPeriod as RetentionPeriod;
    impl crate::value::ToValue for RetentionPeriod_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RetentionPeriodUnit".to_string(),
                crate::value::ToValue::to_value(&self.retention_period_unit),
            );
            properties.insert(
                "RetentionPeriodValue".to_string(),
                crate::value::ToValue::to_value(&self.retention_period_value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rbin-rule-unlockdelay.html
    pub struct UnlockDelay_ {
        pub unlock_delay_unit: Option<crate::value::ExpString>,
        pub unlock_delay_value: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rbin_Rule_UnlockDelay {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Rbin::Rule.UnlockDelay"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rbin_Rule_UnlockDelay as UnlockDelay;
    impl crate::value::ToValue for UnlockDelay_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.unlock_delay_unit {
                properties.insert(
                    "UnlockDelayUnit".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.unlock_delay_value {
                properties.insert(
                    "UnlockDelayValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rbin-rule.html
pub struct Rule_ {
    pub description: Option<crate::value::ExpString>,
    pub exclude_resource_tags: Option<Vec<super::rbin::rule::ResourceTag_>>,
    pub lock_configuration: Option<super::rbin::rule::UnlockDelay_>,
    pub resource_tags: Option<Vec<super::rbin::rule::ResourceTag_>>,
    pub resource_type: crate::value::ExpString,
    pub retention_period: super::rbin::rule::RetentionPeriod_,
    pub status: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_rbin_Rule {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Rbin::Rule" $($field
        $value)*)
    };
}
pub use crate::__aws_rbin_Rule as Rule;
impl crate::template::ToResource for Rule_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Rbin"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Rule"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.exclude_resource_tags {
            properties.insert(
                "ExcludeResourceTags".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.lock_configuration {
            properties.insert(
                "LockConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resource_tags {
            properties.insert(
                "ResourceTags".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ResourceType".to_string(),
            crate::value::ToValue::to_value(&self.resource_type),
        );
        properties.insert(
            "RetentionPeriod".to_string(),
            crate::value::ToValue::to_value(&self.retention_period),
        );
        if let Some(ref value) = self.status {
            properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
