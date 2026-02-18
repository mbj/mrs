pub mod executionplan {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendraranking-executionplan-capacityunitsconfiguration.html>
    pub struct CapacityUnitsConfiguration_ {
        pub rescore_capacity_units: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendraranking_ExecutionPlan_CapacityUnitsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KendraRanking::ExecutionPlan.CapacityUnitsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendraranking_ExecutionPlan_CapacityUnitsConfiguration as CapacityUnitsConfiguration;
    impl crate::value::ToValue for CapacityUnitsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RescoreCapacityUnits".to_string(),
                crate::value::ToValue::to_value(&self.rescore_capacity_units),
            );
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kendraranking-executionplan.html>
pub struct ExecutionPlan_ {
    pub capacity_units: Option<super::kendraranking::executionplan::CapacityUnitsConfiguration_>,
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_kendraranking_ExecutionPlan {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::KendraRanking::ExecutionPlan"
        $($field $value)*)
    };
}
pub use crate::__aws_kendraranking_ExecutionPlan as ExecutionPlan;
impl crate::template::ToResource for ExecutionPlan_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("KendraRanking"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ExecutionPlan"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.capacity_units {
            properties.insert(
                "CapacityUnits".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
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
