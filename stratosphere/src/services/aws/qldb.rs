pub mod stream {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qldb-stream-kinesisconfiguration.html>
    pub struct KinesisConfiguration_ {
        pub aggregation_enabled: Option<crate::value::ExpBool>,
        pub stream_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qldb_Stream_KinesisConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QLDB::Stream.KinesisConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qldb_Stream_KinesisConfiguration as KinesisConfiguration;
    impl crate::value::ToValue for KinesisConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.aggregation_enabled {
                properties.insert(
                    "AggregationEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stream_arn {
                properties.insert(
                    "StreamArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-qldb-ledger.html>
pub struct Ledger_ {
    pub deletion_protection: Option<crate::value::ExpBool>,
    pub kms_key: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub permissions_mode: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_qldb_Ledger {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::QLDB::Ledger" $($field
        $value)*)
    };
}
pub use crate::__aws_qldb_Ledger as Ledger;
impl crate::template::ToResource for Ledger_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("QLDB"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Ledger"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.deletion_protection {
            properties.insert(
                "DeletionProtection".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key {
            properties.insert("KmsKey".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "PermissionsMode".to_string(),
            crate::value::ToValue::to_value(&self.permissions_mode),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-qldb-stream.html>
pub struct Stream_ {
    pub exclusive_end_time: Option<crate::value::ExpString>,
    pub inclusive_start_time: crate::value::ExpString,
    pub kinesis_configuration: super::qldb::stream::KinesisConfiguration_,
    pub ledger_name: crate::value::ExpString,
    pub role_arn: crate::value::ExpString,
    pub stream_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_qldb_Stream {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::QLDB::Stream" $($field
        $value)*)
    };
}
pub use crate::__aws_qldb_Stream as Stream;
impl crate::template::ToResource for Stream_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("QLDB"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Stream"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.exclusive_end_time {
            properties.insert(
                "ExclusiveEndTime".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "InclusiveStartTime".to_string(),
            crate::value::ToValue::to_value(&self.inclusive_start_time),
        );
        properties.insert(
            "KinesisConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.kinesis_configuration),
        );
        properties.insert(
            "LedgerName".to_string(),
            crate::value::ToValue::to_value(&self.ledger_name),
        );
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        properties.insert(
            "StreamName".to_string(),
            crate::value::ToValue::to_value(&self.stream_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
