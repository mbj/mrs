pub mod link {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-oam-link-linkconfiguration.html>
    pub struct LinkConfiguration_ {
        pub log_group_configuration: Option<Box<LinkFilter_>>,
        pub metric_configuration: Option<Box<LinkFilter_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_oam_Link_LinkConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Oam::Link.LinkConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_oam_Link_LinkConfiguration as LinkConfiguration;
    impl crate::value::ToValue for LinkConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.log_group_configuration {
                properties.insert(
                    "LogGroupConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metric_configuration {
                properties.insert(
                    "MetricConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-oam-link-linkfilter.html>
    pub struct LinkFilter_ {
        pub filter: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_oam_Link_LinkFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Oam::Link.LinkFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_oam_Link_LinkFilter as LinkFilter;
    impl crate::value::ToValue for LinkFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Filter".to_string(),
                crate::value::ToValue::to_value(&self.filter),
            );
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-oam-link.html>
pub struct Link_ {
    pub label_template: Option<crate::value::ExpString>,
    pub link_configuration: Option<super::oam::link::LinkConfiguration_>,
    pub resource_types: Vec<crate::value::ExpString>,
    pub sink_identifier: crate::value::ExpString,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_oam_Link {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Oam::Link" $($field
        $value)*)
    };
}
pub use crate::__aws_oam_Link as Link;
impl crate::template::ToResource for Link_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Oam"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Link"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.label_template {
            properties.insert(
                "LabelTemplate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.link_configuration {
            properties.insert(
                "LinkConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ResourceTypes".to_string(),
            crate::value::ToValue::to_value(&self.resource_types),
        );
        properties.insert(
            "SinkIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.sink_identifier),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-oam-sink.html>
pub struct Sink_ {
    pub name: crate::value::ExpString,
    pub policy: Option<serde_json::Value>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_oam_Sink {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Oam::Sink" $($field
        $value)*)
    };
}
pub use crate::__aws_oam_Sink as Sink;
impl crate::template::ToResource for Sink_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Oam"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Sink"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.policy {
            properties.insert("Policy".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
