pub mod anomalymonitor {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ce-anomalymonitor-resourcetag.html
    pub struct ResourceTag_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ce_AnomalyMonitor_ResourceTag {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CE::AnomalyMonitor.ResourceTag"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ce_AnomalyMonitor_ResourceTag as ResourceTag;
    impl crate::value::ToValue for ResourceTag_ {
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
}
pub mod anomalysubscription {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ce-anomalysubscription-resourcetag.html
    pub struct ResourceTag_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ce_AnomalySubscription_ResourceTag {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CE::AnomalySubscription.ResourceTag"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ce_AnomalySubscription_ResourceTag as ResourceTag;
    impl crate::value::ToValue for ResourceTag_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ce-anomalysubscription-subscriber.html
    pub struct Subscriber_ {
        pub address: crate::value::ExpString,
        pub status: Option<crate::value::ExpString>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ce_AnomalySubscription_Subscriber {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CE::AnomalySubscription.Subscriber"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ce_AnomalySubscription_Subscriber as Subscriber;
    impl crate::value::ToValue for Subscriber_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Address".to_string(),
                crate::value::ToValue::to_value(&self.address),
            );
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
}
pub mod costcategory {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ce-costcategory-resourcetag.html
    pub struct ResourceTag_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ce_CostCategory_ResourceTag {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CE::CostCategory.ResourceTag"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ce_CostCategory_ResourceTag as ResourceTag;
    impl crate::value::ToValue for ResourceTag_ {
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
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ce-anomalymonitor.html
pub struct AnomalyMonitor_ {
    pub monitor_dimension: Option<crate::value::ExpString>,
    pub monitor_name: crate::value::ExpString,
    pub monitor_specification: Option<crate::value::ExpString>,
    pub monitor_type: crate::value::ExpString,
    pub resource_tags: Option<Vec<super::ce::anomalymonitor::ResourceTag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ce_AnomalyMonitor {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::CE::AnomalyMonitor"
        $($field $value)*)
    };
}
pub use crate::__aws_ce_AnomalyMonitor as AnomalyMonitor;
impl crate::template::ToResource for AnomalyMonitor_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CE"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AnomalyMonitor"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.monitor_dimension {
            properties.insert(
                "MonitorDimension".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "MonitorName".to_string(),
            crate::value::ToValue::to_value(&self.monitor_name),
        );
        if let Some(ref value) = self.monitor_specification {
            properties.insert(
                "MonitorSpecification".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "MonitorType".to_string(),
            crate::value::ToValue::to_value(&self.monitor_type),
        );
        if let Some(ref value) = self.resource_tags {
            properties.insert(
                "ResourceTags".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ce-anomalysubscription.html
pub struct AnomalySubscription_ {
    pub frequency: crate::value::ExpString,
    pub monitor_arn_list: Vec<crate::value::ExpString>,
    pub resource_tags: Option<Vec<super::ce::anomalysubscription::ResourceTag_>>,
    pub subscribers: Vec<super::ce::anomalysubscription::Subscriber_>,
    pub subscription_name: crate::value::ExpString,
    pub threshold: Option<f64>,
    pub threshold_expression: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ce_AnomalySubscription {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::CE::AnomalySubscription"
        $($field $value)*)
    };
}
pub use crate::__aws_ce_AnomalySubscription as AnomalySubscription;
impl crate::template::ToResource for AnomalySubscription_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CE"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AnomalySubscription"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Frequency".to_string(),
            crate::value::ToValue::to_value(&self.frequency),
        );
        properties.insert(
            "MonitorArnList".to_string(),
            crate::value::ToValue::to_value(&self.monitor_arn_list),
        );
        if let Some(ref value) = self.resource_tags {
            properties.insert(
                "ResourceTags".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Subscribers".to_string(),
            crate::value::ToValue::to_value(&self.subscribers),
        );
        properties.insert(
            "SubscriptionName".to_string(),
            crate::value::ToValue::to_value(&self.subscription_name),
        );
        if let Some(ref value) = self.threshold {
            properties.insert(
                "Threshold".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.threshold_expression {
            properties.insert(
                "ThresholdExpression".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ce-costcategory.html
pub struct CostCategory_ {
    pub default_value: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub rule_version: crate::value::ExpString,
    pub rules: crate::value::ExpString,
    pub split_charge_rules: Option<crate::value::ExpString>,
    pub tags: Option<Vec<super::ce::costcategory::ResourceTag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ce_CostCategory {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::CE::CostCategory" $($field
        $value)*)
    };
}
pub use crate::__aws_ce_CostCategory as CostCategory;
impl crate::template::ToResource for CostCategory_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CE"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("CostCategory"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.default_value {
            properties.insert(
                "DefaultValue".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "RuleVersion".to_string(),
            crate::value::ToValue::to_value(&self.rule_version),
        );
        properties.insert(
            "Rules".to_string(),
            crate::value::ToValue::to_value(&self.rules),
        );
        if let Some(ref value) = self.split_charge_rules {
            properties.insert(
                "SplitChargeRules".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
