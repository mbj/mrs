pub mod streamprocessor {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rekognition-streamprocessor-boundingbox.html
    pub struct BoundingBox_ {
        pub height: f64,
        pub left: f64,
        pub top: f64,
        pub width: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rekognition_StreamProcessor_BoundingBox {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Rekognition::StreamProcessor.BoundingBox"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rekognition_StreamProcessor_BoundingBox as BoundingBox;
    impl crate::value::ToValue for BoundingBox_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Height".to_string(),
                crate::value::ToValue::to_value(&self.height),
            );
            properties.insert(
                "Left".to_string(),
                crate::value::ToValue::to_value(&self.left),
            );
            properties.insert(
                "Top".to_string(),
                crate::value::ToValue::to_value(&self.top),
            );
            properties.insert(
                "Width".to_string(),
                crate::value::ToValue::to_value(&self.width),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rekognition-streamprocessor-connectedhomesettings.html
    pub struct ConnectedHomeSettings_ {
        pub labels: Vec<crate::value::ExpString>,
        pub min_confidence: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rekognition_StreamProcessor_ConnectedHomeSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Rekognition::StreamProcessor.ConnectedHomeSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rekognition_StreamProcessor_ConnectedHomeSettings as ConnectedHomeSettings;
    impl crate::value::ToValue for ConnectedHomeSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Labels".to_string(),
                crate::value::ToValue::to_value(&self.labels),
            );
            if let Some(ref value) = self.min_confidence {
                properties.insert(
                    "MinConfidence".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rekognition-streamprocessor-datasharingpreference.html
    pub struct DataSharingPreference_ {
        pub opt_in: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rekognition_StreamProcessor_DataSharingPreference {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Rekognition::StreamProcessor.DataSharingPreference"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rekognition_StreamProcessor_DataSharingPreference as DataSharingPreference;
    impl crate::value::ToValue for DataSharingPreference_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "OptIn".to_string(),
                crate::value::ToValue::to_value(&self.opt_in),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rekognition-streamprocessor-facesearchsettings.html
    pub struct FaceSearchSettings_ {
        pub collection_id: crate::value::ExpString,
        pub face_match_threshold: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rekognition_StreamProcessor_FaceSearchSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Rekognition::StreamProcessor.FaceSearchSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rekognition_StreamProcessor_FaceSearchSettings as FaceSearchSettings;
    impl crate::value::ToValue for FaceSearchSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CollectionId".to_string(),
                crate::value::ToValue::to_value(&self.collection_id),
            );
            if let Some(ref value) = self.face_match_threshold {
                properties.insert(
                    "FaceMatchThreshold".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rekognition-streamprocessor-kinesisdatastream.html
    pub struct KinesisDataStream_ {
        pub arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rekognition_StreamProcessor_KinesisDataStream {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Rekognition::StreamProcessor.KinesisDataStream"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rekognition_StreamProcessor_KinesisDataStream as KinesisDataStream;
    impl crate::value::ToValue for KinesisDataStream_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Arn".to_string(),
                crate::value::ToValue::to_value(&self.arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rekognition-streamprocessor-kinesisvideostream.html
    pub struct KinesisVideoStream_ {
        pub arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rekognition_StreamProcessor_KinesisVideoStream {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Rekognition::StreamProcessor.KinesisVideoStream"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rekognition_StreamProcessor_KinesisVideoStream as KinesisVideoStream;
    impl crate::value::ToValue for KinesisVideoStream_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Arn".to_string(),
                crate::value::ToValue::to_value(&self.arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rekognition-streamprocessor-notificationchannel.html
    pub struct NotificationChannel_ {
        pub arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rekognition_StreamProcessor_NotificationChannel {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Rekognition::StreamProcessor.NotificationChannel"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rekognition_StreamProcessor_NotificationChannel as NotificationChannel;
    impl crate::value::ToValue for NotificationChannel_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Arn".to_string(),
                crate::value::ToValue::to_value(&self.arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rekognition-streamprocessor-s3destination.html
    pub struct S3Destination_ {
        pub bucket_name: crate::value::ExpString,
        pub object_key_prefix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rekognition_StreamProcessor_S3Destination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Rekognition::StreamProcessor.S3Destination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rekognition_StreamProcessor_S3Destination as S3Destination;
    impl crate::value::ToValue for S3Destination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BucketName".to_string(),
                crate::value::ToValue::to_value(&self.bucket_name),
            );
            if let Some(ref value) = self.object_key_prefix {
                properties.insert(
                    "ObjectKeyPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rekognition-collection.html
pub struct Collection_ {
    pub collection_id: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_rekognition_Collection {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Rekognition::Collection"
        $($field $value)*)
    };
}
pub use crate::__aws_rekognition_Collection as Collection;
impl crate::template::ToResource for Collection_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Rekognition"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Collection"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "CollectionId".to_string(),
            crate::value::ToValue::to_value(&self.collection_id),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rekognition-project.html
pub struct Project_ {
    pub project_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_rekognition_Project {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Rekognition::Project"
        $($field $value)*)
    };
}
pub use crate::__aws_rekognition_Project as Project;
impl crate::template::ToResource for Project_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Rekognition"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Project"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ProjectName".to_string(),
            crate::value::ToValue::to_value(&self.project_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rekognition-streamprocessor.html
pub struct StreamProcessor_ {
    pub bounding_box_regions_of_interest:
        Option<Vec<super::rekognition::streamprocessor::BoundingBox_>>,
    pub connected_home_settings:
        Option<super::rekognition::streamprocessor::ConnectedHomeSettings_>,
    pub data_sharing_preference:
        Option<super::rekognition::streamprocessor::DataSharingPreference_>,
    pub face_search_settings: Option<super::rekognition::streamprocessor::FaceSearchSettings_>,
    pub kinesis_data_stream: Option<super::rekognition::streamprocessor::KinesisDataStream_>,
    pub kinesis_video_stream: super::rekognition::streamprocessor::KinesisVideoStream_,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub notification_channel: Option<super::rekognition::streamprocessor::NotificationChannel_>,
    pub polygon_regions_of_interest: Option<serde_json::Value>,
    pub role_arn: crate::value::ExpString,
    pub s3_destination: Option<super::rekognition::streamprocessor::S3Destination_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_rekognition_StreamProcessor {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Rekognition::StreamProcessor"
        $($field $value)*)
    };
}
pub use crate::__aws_rekognition_StreamProcessor as StreamProcessor;
impl crate::template::ToResource for StreamProcessor_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Rekognition"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("StreamProcessor"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.bounding_box_regions_of_interest {
            properties.insert(
                "BoundingBoxRegionsOfInterest".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.connected_home_settings {
            properties.insert(
                "ConnectedHomeSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.data_sharing_preference {
            properties.insert(
                "DataSharingPreference".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.face_search_settings {
            properties.insert(
                "FaceSearchSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kinesis_data_stream {
            properties.insert(
                "KinesisDataStream".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "KinesisVideoStream".to_string(),
            crate::value::ToValue::to_value(&self.kinesis_video_stream),
        );
        if let Some(ref value) = self.kms_key_id {
            properties.insert(
                "KmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.notification_channel {
            properties.insert(
                "NotificationChannel".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.polygon_regions_of_interest {
            properties.insert(
                "PolygonRegionsOfInterest".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        if let Some(ref value) = self.s3_destination {
            properties.insert(
                "S3Destination".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
