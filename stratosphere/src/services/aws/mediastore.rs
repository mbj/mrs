pub mod container {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediastore-container-corsrule.html
    pub struct CorsRule_ {
        pub allowed_headers: Option<Vec<crate::value::ExpString>>,
        pub allowed_methods: Option<Vec<crate::value::ExpString>>,
        pub allowed_origins: Option<Vec<crate::value::ExpString>>,
        pub expose_headers: Option<Vec<crate::value::ExpString>>,
        pub max_age_seconds: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediastore_Container_CorsRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaStore::Container.CorsRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediastore_Container_CorsRule as CorsRule;
    impl crate::value::ToValue for CorsRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allowed_headers {
                properties.insert(
                    "AllowedHeaders".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.allowed_methods {
                properties.insert(
                    "AllowedMethods".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.allowed_origins {
                properties.insert(
                    "AllowedOrigins".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.expose_headers {
                properties.insert(
                    "ExposeHeaders".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_age_seconds {
                properties.insert(
                    "MaxAgeSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediastore-container-metricpolicy.html
    pub struct MetricPolicy_ {
        pub container_level_metrics: crate::value::ExpString,
        pub metric_policy_rules: Option<Vec<MetricPolicyRule_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediastore_Container_MetricPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaStore::Container.MetricPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediastore_Container_MetricPolicy as MetricPolicy;
    impl crate::value::ToValue for MetricPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ContainerLevelMetrics".to_string(),
                crate::value::ToValue::to_value(&self.container_level_metrics),
            );
            if let Some(ref value) = self.metric_policy_rules {
                properties.insert(
                    "MetricPolicyRules".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediastore-container-metricpolicyrule.html
    pub struct MetricPolicyRule_ {
        pub object_group: crate::value::ExpString,
        pub object_group_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediastore_Container_MetricPolicyRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaStore::Container.MetricPolicyRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediastore_Container_MetricPolicyRule as MetricPolicyRule;
    impl crate::value::ToValue for MetricPolicyRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ObjectGroup".to_string(),
                crate::value::ToValue::to_value(&self.object_group),
            );
            properties.insert(
                "ObjectGroupName".to_string(),
                crate::value::ToValue::to_value(&self.object_group_name),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediastore-container.html
pub struct Container_ {
    pub access_logging_enabled: Option<crate::value::ExpBool>,
    pub container_name: crate::value::ExpString,
    pub cors_policy: Option<Vec<super::mediastore::container::CorsRule_>>,
    pub lifecycle_policy: Option<crate::value::ExpString>,
    pub metric_policy: Option<super::mediastore::container::MetricPolicy_>,
    pub policy: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_mediastore_Container {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::MediaStore::Container"
        $($field $value)*)
    };
}
pub use crate::__aws_mediastore_Container as Container;
impl crate::template::ToResource for Container_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaStore"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Container"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.access_logging_enabled {
            properties.insert(
                "AccessLoggingEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ContainerName".to_string(),
            crate::value::ToValue::to_value(&self.container_name),
        );
        if let Some(ref value) = self.cors_policy {
            properties.insert(
                "CorsPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.lifecycle_policy {
            properties.insert(
                "LifecyclePolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.metric_policy {
            properties.insert(
                "MetricPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.policy {
            properties.insert("Policy".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
