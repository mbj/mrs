pub mod simulation {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-simspaceweaver-simulation-s3location.html
    pub struct S3Location_ {
        pub bucket_name: crate::value::ExpString,
        pub object_key: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_simspaceweaver_Simulation_S3Location {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SimSpaceWeaver::Simulation.S3Location"
            $($field $value)*)
        };
    }
    pub use crate::__aws_simspaceweaver_Simulation_S3Location as S3Location;
    impl crate::value::ToValue for S3Location_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BucketName".to_string(),
                crate::value::ToValue::to_value(&self.bucket_name),
            );
            properties.insert(
                "ObjectKey".to_string(),
                crate::value::ToValue::to_value(&self.object_key),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-simspaceweaver-simulation.html
pub struct Simulation_ {
    pub maximum_duration: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub role_arn: crate::value::ExpString,
    pub schema_s3_location: Option<super::simspaceweaver::simulation::S3Location_>,
    pub snapshot_s3_location: Option<super::simspaceweaver::simulation::S3Location_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_simspaceweaver_Simulation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SimSpaceWeaver::Simulation"
        $($field $value)*)
    };
}
pub use crate::__aws_simspaceweaver_Simulation as Simulation;
impl crate::template::ToResource for Simulation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SimSpaceWeaver"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Simulation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.maximum_duration {
            properties.insert(
                "MaximumDuration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        if let Some(ref value) = self.schema_s3_location {
            properties.insert(
                "SchemaS3Location".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.snapshot_s3_location {
            properties.insert(
                "SnapshotS3Location".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
