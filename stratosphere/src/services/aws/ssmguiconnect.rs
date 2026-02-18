pub mod preferences {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmguiconnect-preferences-connectionrecordingpreferences.html>
    pub struct ConnectionRecordingPreferences_ {
        pub kms_key_arn: crate::value::ExpString,
        pub recording_destinations: Box<RecordingDestinations_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssmguiconnect_Preferences_ConnectionRecordingPreferences {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SSMGuiConnect::Preferences.ConnectionRecordingPreferences"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssmguiconnect_Preferences_ConnectionRecordingPreferences as ConnectionRecordingPreferences;
    impl crate::value::ToValue for ConnectionRecordingPreferences_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "KMSKeyArn".to_string(),
                crate::value::ToValue::to_value(&self.kms_key_arn),
            );
            properties.insert(
                "RecordingDestinations".to_string(),
                crate::value::ToValue::to_value(&self.recording_destinations),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmguiconnect-preferences-recordingdestinations.html>
    pub struct RecordingDestinations_ {
        pub s3_buckets: Vec<S3Bucket_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssmguiconnect_Preferences_RecordingDestinations {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SSMGuiConnect::Preferences.RecordingDestinations"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssmguiconnect_Preferences_RecordingDestinations as RecordingDestinations;
    impl crate::value::ToValue for RecordingDestinations_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "S3Buckets".to_string(),
                crate::value::ToValue::to_value(&self.s3_buckets),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmguiconnect-preferences-s3bucket.html>
    pub struct S3Bucket_ {
        pub bucket_name: crate::value::ExpString,
        pub bucket_owner: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssmguiconnect_Preferences_S3Bucket {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SSMGuiConnect::Preferences.S3Bucket"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssmguiconnect_Preferences_S3Bucket as S3Bucket;
    impl crate::value::ToValue for S3Bucket_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BucketName".to_string(),
                crate::value::ToValue::to_value(&self.bucket_name),
            );
            properties.insert(
                "BucketOwner".to_string(),
                crate::value::ToValue::to_value(&self.bucket_owner),
            );
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmguiconnect-preferences.html>
pub struct Preferences_ {
    pub connection_recording_preferences:
        Option<super::ssmguiconnect::preferences::ConnectionRecordingPreferences_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ssmguiconnect_Preferences {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SSMGuiConnect::Preferences"
        $($field $value)*)
    };
}
pub use crate::__aws_ssmguiconnect_Preferences as Preferences;
impl crate::template::ToResource for Preferences_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SSMGuiConnect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Preferences"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.connection_recording_preferences {
            properties.insert(
                "ConnectionRecordingPreferences".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
