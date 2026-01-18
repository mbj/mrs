pub mod channel {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ivs-channel-multitrackinputconfiguration.html
    pub struct MultitrackInputConfiguration_ {
        pub enabled: Option<crate::value::ExpBool>,
        pub maximum_resolution: Option<crate::value::ExpString>,
        pub policy: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ivs_Channel_MultitrackInputConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IVS::Channel.MultitrackInputConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ivs_Channel_MultitrackInputConfiguration as MultitrackInputConfiguration;
    impl crate::value::ToValue for MultitrackInputConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.maximum_resolution {
                properties.insert(
                    "MaximumResolution".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.policy {
                properties.insert("Policy".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod encoderconfiguration {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ivs-encoderconfiguration-video.html
    pub struct Video_ {
        pub bitrate: Option<i64>,
        pub framerate: Option<f64>,
        pub height: Option<i64>,
        pub width: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ivs_EncoderConfiguration_Video {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IVS::EncoderConfiguration.Video"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ivs_EncoderConfiguration_Video as Video;
    impl crate::value::ToValue for Video_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bitrate {
                properties.insert(
                    "Bitrate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.framerate {
                properties.insert(
                    "Framerate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.height {
                properties.insert("Height".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.width {
                properties.insert("Width".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod recordingconfiguration {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ivs-recordingconfiguration-destinationconfiguration.html
    pub struct DestinationConfiguration_ {
        pub s3: Option<Box<S3DestinationConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ivs_RecordingConfiguration_DestinationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IVS::RecordingConfiguration.DestinationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ivs_RecordingConfiguration_DestinationConfiguration as DestinationConfiguration;
    impl crate::value::ToValue for DestinationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.s3 {
                properties.insert("S3".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ivs-recordingconfiguration-renditionconfiguration.html
    pub struct RenditionConfiguration_ {
        pub rendition_selection: Option<crate::value::ExpString>,
        pub renditions: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ivs_RecordingConfiguration_RenditionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IVS::RecordingConfiguration.RenditionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ivs_RecordingConfiguration_RenditionConfiguration as RenditionConfiguration;
    impl crate::value::ToValue for RenditionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.rendition_selection {
                properties.insert(
                    "RenditionSelection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.renditions {
                properties.insert(
                    "Renditions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ivs-recordingconfiguration-s3destinationconfiguration.html
    pub struct S3DestinationConfiguration_ {
        pub bucket_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ivs_RecordingConfiguration_S3DestinationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IVS::RecordingConfiguration.S3DestinationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ivs_RecordingConfiguration_S3DestinationConfiguration as S3DestinationConfiguration;
    impl crate::value::ToValue for S3DestinationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BucketName".to_string(),
                crate::value::ToValue::to_value(&self.bucket_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ivs-recordingconfiguration-thumbnailconfiguration.html
    pub struct ThumbnailConfiguration_ {
        pub recording_mode: Option<crate::value::ExpString>,
        pub resolution: Option<crate::value::ExpString>,
        pub storage: Option<Vec<crate::value::ExpString>>,
        pub target_interval_seconds: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ivs_RecordingConfiguration_ThumbnailConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IVS::RecordingConfiguration.ThumbnailConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ivs_RecordingConfiguration_ThumbnailConfiguration as ThumbnailConfiguration;
    impl crate::value::ToValue for ThumbnailConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.recording_mode {
                properties.insert(
                    "RecordingMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resolution {
                properties.insert(
                    "Resolution".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.storage {
                properties.insert(
                    "Storage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_interval_seconds {
                properties.insert(
                    "TargetIntervalSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod stage {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ivs-stage-autoparticipantrecordingconfiguration.html
    pub struct AutoParticipantRecordingConfiguration_ {
        pub hls_configuration: Option<Box<HlsConfiguration_>>,
        pub media_types: Option<Vec<crate::value::ExpString>>,
        pub recording_reconnect_window_seconds: Option<i64>,
        pub storage_configuration_arn: crate::value::ExpString,
        pub thumbnail_configuration: Option<Box<ThumbnailConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ivs_Stage_AutoParticipantRecordingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IVS::Stage.AutoParticipantRecordingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ivs_Stage_AutoParticipantRecordingConfiguration as AutoParticipantRecordingConfiguration;
    impl crate::value::ToValue for AutoParticipantRecordingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.hls_configuration {
                properties.insert(
                    "HlsConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.media_types {
                properties.insert(
                    "MediaTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.recording_reconnect_window_seconds {
                properties.insert(
                    "RecordingReconnectWindowSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "StorageConfigurationArn".to_string(),
                crate::value::ToValue::to_value(&self.storage_configuration_arn),
            );
            if let Some(ref value) = self.thumbnail_configuration {
                properties.insert(
                    "ThumbnailConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ivs-stage-hlsconfiguration.html
    pub struct HlsConfiguration_ {
        pub participant_recording_hls_configuration:
            Option<Box<ParticipantRecordingHlsConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ivs_Stage_HlsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IVS::Stage.HlsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ivs_Stage_HlsConfiguration as HlsConfiguration;
    impl crate::value::ToValue for HlsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.participant_recording_hls_configuration {
                properties.insert(
                    "ParticipantRecordingHlsConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ivs-stage-participantrecordinghlsconfiguration.html
    pub struct ParticipantRecordingHlsConfiguration_ {
        pub target_segment_duration_seconds: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ivs_Stage_ParticipantRecordingHlsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IVS::Stage.ParticipantRecordingHlsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ivs_Stage_ParticipantRecordingHlsConfiguration as ParticipantRecordingHlsConfiguration;
    impl crate::value::ToValue for ParticipantRecordingHlsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.target_segment_duration_seconds {
                properties.insert(
                    "TargetSegmentDurationSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ivs-stage-participantthumbnailconfiguration.html
    pub struct ParticipantThumbnailConfiguration_ {
        pub recording_mode: Option<crate::value::ExpString>,
        pub storage: Option<Vec<crate::value::ExpString>>,
        pub target_interval_seconds: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ivs_Stage_ParticipantThumbnailConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IVS::Stage.ParticipantThumbnailConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ivs_Stage_ParticipantThumbnailConfiguration as ParticipantThumbnailConfiguration;
    impl crate::value::ToValue for ParticipantThumbnailConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.recording_mode {
                properties.insert(
                    "RecordingMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.storage {
                properties.insert(
                    "Storage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_interval_seconds {
                properties.insert(
                    "TargetIntervalSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ivs-stage-thumbnailconfiguration.html
    pub struct ThumbnailConfiguration_ {
        pub participant_thumbnail_configuration: Option<Box<ParticipantThumbnailConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ivs_Stage_ThumbnailConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IVS::Stage.ThumbnailConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ivs_Stage_ThumbnailConfiguration as ThumbnailConfiguration;
    impl crate::value::ToValue for ThumbnailConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.participant_thumbnail_configuration {
                properties.insert(
                    "ParticipantThumbnailConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod storageconfiguration {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ivs-storageconfiguration-s3storageconfiguration.html
    pub struct S3StorageConfiguration_ {
        pub bucket_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ivs_StorageConfiguration_S3StorageConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IVS::StorageConfiguration.S3StorageConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ivs_StorageConfiguration_S3StorageConfiguration as S3StorageConfiguration;
    impl crate::value::ToValue for S3StorageConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BucketName".to_string(),
                crate::value::ToValue::to_value(&self.bucket_name),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ivs-channel.html
pub struct Channel_ {
    pub authorized: Option<crate::value::ExpBool>,
    pub container_format: Option<crate::value::ExpString>,
    pub insecure_ingest: Option<crate::value::ExpBool>,
    pub latency_mode: Option<crate::value::ExpString>,
    pub multitrack_input_configuration: Option<super::ivs::channel::MultitrackInputConfiguration_>,
    pub name: Option<crate::value::ExpString>,
    pub preset: Option<crate::value::ExpString>,
    pub recording_configuration_arn: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub r#type: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ivs_Channel {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IVS::Channel" $($field
        $value)*)
    };
}
pub use crate::__aws_ivs_Channel as Channel;
impl crate::template::ToResource for Channel_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IVS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Channel"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.authorized {
            properties.insert(
                "Authorized".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.container_format {
            properties.insert(
                "ContainerFormat".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.insecure_ingest {
            properties.insert(
                "InsecureIngest".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.latency_mode {
            properties.insert(
                "LatencyMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.multitrack_input_configuration {
            properties.insert(
                "MultitrackInputConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.preset {
            properties.insert("Preset".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.recording_configuration_arn {
            properties.insert(
                "RecordingConfigurationArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.r#type {
            properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ivs-encoderconfiguration.html
pub struct EncoderConfiguration_ {
    pub name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub video: Option<super::ivs::encoderconfiguration::Video_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ivs_EncoderConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IVS::EncoderConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_ivs_EncoderConfiguration as EncoderConfiguration;
impl crate::template::ToResource for EncoderConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IVS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("EncoderConfiguration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.video {
            properties.insert("Video".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ivs-ingestconfiguration.html
pub struct IngestConfiguration_ {
    pub ingest_protocol: Option<crate::value::ExpString>,
    pub insecure_ingest: Option<crate::value::ExpBool>,
    pub name: Option<crate::value::ExpString>,
    pub stage_arn: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub user_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ivs_IngestConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IVS::IngestConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_ivs_IngestConfiguration as IngestConfiguration;
impl crate::template::ToResource for IngestConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IVS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("IngestConfiguration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.ingest_protocol {
            properties.insert(
                "IngestProtocol".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.insecure_ingest {
            properties.insert(
                "InsecureIngest".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.stage_arn {
            properties.insert(
                "StageArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.user_id {
            properties.insert("UserId".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ivs-playbackkeypair.html
pub struct PlaybackKeyPair_ {
    pub name: Option<crate::value::ExpString>,
    pub public_key_material: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ivs_PlaybackKeyPair {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IVS::PlaybackKeyPair"
        $($field $value)*)
    };
}
pub use crate::__aws_ivs_PlaybackKeyPair as PlaybackKeyPair;
impl crate::template::ToResource for PlaybackKeyPair_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IVS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PlaybackKeyPair"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.public_key_material {
            properties.insert(
                "PublicKeyMaterial".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ivs-playbackrestrictionpolicy.html
pub struct PlaybackRestrictionPolicy_ {
    pub allowed_countries: Option<Vec<crate::value::ExpString>>,
    pub allowed_origins: Option<Vec<crate::value::ExpString>>,
    pub enable_strict_origin_enforcement: Option<crate::value::ExpBool>,
    pub name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ivs_PlaybackRestrictionPolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IVS::PlaybackRestrictionPolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_ivs_PlaybackRestrictionPolicy as PlaybackRestrictionPolicy;
impl crate::template::ToResource for PlaybackRestrictionPolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IVS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PlaybackRestrictionPolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.allowed_countries {
            properties.insert(
                "AllowedCountries".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.allowed_origins {
            properties.insert(
                "AllowedOrigins".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enable_strict_origin_enforcement {
            properties.insert(
                "EnableStrictOriginEnforcement".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ivs-publickey.html
pub struct PublicKey_ {
    pub name: Option<crate::value::ExpString>,
    pub public_key_material: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ivs_PublicKey {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IVS::PublicKey" $($field
        $value)*)
    };
}
pub use crate::__aws_ivs_PublicKey as PublicKey;
impl crate::template::ToResource for PublicKey_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IVS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PublicKey"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.public_key_material {
            properties.insert(
                "PublicKeyMaterial".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ivs-recordingconfiguration.html
pub struct RecordingConfiguration_ {
    pub destination_configuration: super::ivs::recordingconfiguration::DestinationConfiguration_,
    pub name: Option<crate::value::ExpString>,
    pub recording_reconnect_window_seconds: Option<i64>,
    pub rendition_configuration:
        Option<super::ivs::recordingconfiguration::RenditionConfiguration_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub thumbnail_configuration:
        Option<super::ivs::recordingconfiguration::ThumbnailConfiguration_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ivs_RecordingConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IVS::RecordingConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_ivs_RecordingConfiguration as RecordingConfiguration;
impl crate::template::ToResource for RecordingConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IVS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RecordingConfiguration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DestinationConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.destination_configuration),
        );
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.recording_reconnect_window_seconds {
            properties.insert(
                "RecordingReconnectWindowSeconds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.rendition_configuration {
            properties.insert(
                "RenditionConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.thumbnail_configuration {
            properties.insert(
                "ThumbnailConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ivs-stage.html
pub struct Stage_ {
    pub auto_participant_recording_configuration:
        Option<super::ivs::stage::AutoParticipantRecordingConfiguration_>,
    pub name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ivs_Stage {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IVS::Stage" $($field
        $value)*)
    };
}
pub use crate::__aws_ivs_Stage as Stage;
impl crate::template::ToResource for Stage_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IVS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Stage"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.auto_participant_recording_configuration {
            properties.insert(
                "AutoParticipantRecordingConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ivs-storageconfiguration.html
pub struct StorageConfiguration_ {
    pub name: Option<crate::value::ExpString>,
    pub s3: super::ivs::storageconfiguration::S3StorageConfiguration_,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ivs_StorageConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IVS::StorageConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_ivs_StorageConfiguration as StorageConfiguration;
impl crate::template::ToResource for StorageConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IVS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("StorageConfiguration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert("S3".to_string(), crate::value::ToValue::to_value(&self.s3));
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ivs-streamkey.html
pub struct StreamKey_ {
    pub channel_arn: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ivs_StreamKey {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IVS::StreamKey" $($field
        $value)*)
    };
}
pub use crate::__aws_ivs_StreamKey as StreamKey;
impl crate::template::ToResource for StreamKey_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IVS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("StreamKey"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ChannelArn".to_string(),
            crate::value::ToValue::to_value(&self.channel_arn),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
