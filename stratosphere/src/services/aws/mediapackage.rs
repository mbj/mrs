pub mod asset {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-asset-egressendpoint.html
    pub struct EgressEndpoint_ {
        pub packaging_configuration_id: crate::value::ExpString,
        pub url: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackage_Asset_EgressEndpoint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackage::Asset.EgressEndpoint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackage_Asset_EgressEndpoint as EgressEndpoint;
    impl crate::value::ToValue for EgressEndpoint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PackagingConfigurationId".to_string(),
                crate::value::ToValue::to_value(&self.packaging_configuration_id),
            );
            properties.insert(
                "Url".to_string(),
                crate::value::ToValue::to_value(&self.url),
            );
            properties.into()
        }
    }
}
pub mod channel {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-channel-hlsingest.html
    pub struct HlsIngest_ {
        pub ingest_endpoints: Option<Vec<IngestEndpoint_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackage_Channel_HlsIngest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackage::Channel.HlsIngest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackage_Channel_HlsIngest as HlsIngest;
    impl crate::value::ToValue for HlsIngest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ingest_endpoints {
                properties.insert(
                    "ingestEndpoints".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-channel-ingestendpoint.html
    pub struct IngestEndpoint_ {
        pub id: crate::value::ExpString,
        pub password: crate::value::ExpString,
        pub url: crate::value::ExpString,
        pub username: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackage_Channel_IngestEndpoint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackage::Channel.IngestEndpoint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackage_Channel_IngestEndpoint as IngestEndpoint;
    impl crate::value::ToValue for IngestEndpoint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            properties.insert(
                "Password".to_string(),
                crate::value::ToValue::to_value(&self.password),
            );
            properties.insert(
                "Url".to_string(),
                crate::value::ToValue::to_value(&self.url),
            );
            properties.insert(
                "Username".to_string(),
                crate::value::ToValue::to_value(&self.username),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-channel-logconfiguration.html
    pub struct LogConfiguration_ {
        pub log_group_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackage_Channel_LogConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackage::Channel.LogConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackage_Channel_LogConfiguration as LogConfiguration;
    impl crate::value::ToValue for LogConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.log_group_name {
                properties.insert(
                    "LogGroupName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod originendpoint {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-authorization.html
    pub struct Authorization_ {
        pub cdn_identifier_secret: crate::value::ExpString,
        pub secrets_role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackage_OriginEndpoint_Authorization {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackage::OriginEndpoint.Authorization"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackage_OriginEndpoint_Authorization as Authorization;
    impl crate::value::ToValue for Authorization_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CdnIdentifierSecret".to_string(),
                crate::value::ToValue::to_value(&self.cdn_identifier_secret),
            );
            properties.insert(
                "SecretsRoleArn".to_string(),
                crate::value::ToValue::to_value(&self.secrets_role_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-cmafencryption.html
    pub struct CmafEncryption_ {
        pub constant_initialization_vector: Option<crate::value::ExpString>,
        pub encryption_method: Option<crate::value::ExpString>,
        pub key_rotation_interval_seconds: Option<i32>,
        pub speke_key_provider: Box<SpekeKeyProvider_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackage_OriginEndpoint_CmafEncryption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackage::OriginEndpoint.CmafEncryption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackage_OriginEndpoint_CmafEncryption as CmafEncryption;
    impl crate::value::ToValue for CmafEncryption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.constant_initialization_vector {
                properties.insert(
                    "ConstantInitializationVector".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.encryption_method {
                properties.insert(
                    "EncryptionMethod".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.key_rotation_interval_seconds {
                properties.insert(
                    "KeyRotationIntervalSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SpekeKeyProvider".to_string(),
                crate::value::ToValue::to_value(&self.speke_key_provider),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-cmafpackage.html
    pub struct CmafPackage_ {
        pub encryption: Option<Box<CmafEncryption_>>,
        pub hls_manifests: Option<Vec<HlsManifest_>>,
        pub segment_duration_seconds: Option<i32>,
        pub segment_prefix: Option<crate::value::ExpString>,
        pub stream_selection: Option<Box<StreamSelection_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackage_OriginEndpoint_CmafPackage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackage::OriginEndpoint.CmafPackage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackage_OriginEndpoint_CmafPackage as CmafPackage;
    impl crate::value::ToValue for CmafPackage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.encryption {
                properties.insert(
                    "Encryption".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hls_manifests {
                properties.insert(
                    "HlsManifests".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.segment_duration_seconds {
                properties.insert(
                    "SegmentDurationSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.segment_prefix {
                properties.insert(
                    "SegmentPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stream_selection {
                properties.insert(
                    "StreamSelection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-dashencryption.html
    pub struct DashEncryption_ {
        pub key_rotation_interval_seconds: Option<i32>,
        pub speke_key_provider: Box<SpekeKeyProvider_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackage_OriginEndpoint_DashEncryption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackage::OriginEndpoint.DashEncryption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackage_OriginEndpoint_DashEncryption as DashEncryption;
    impl crate::value::ToValue for DashEncryption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.key_rotation_interval_seconds {
                properties.insert(
                    "KeyRotationIntervalSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SpekeKeyProvider".to_string(),
                crate::value::ToValue::to_value(&self.speke_key_provider),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-dashpackage.html
    pub struct DashPackage_ {
        pub ad_triggers: Option<Vec<crate::value::ExpString>>,
        pub ads_on_delivery_restrictions: Option<crate::value::ExpString>,
        pub encryption: Option<Box<DashEncryption_>>,
        pub include_iframe_only_stream: Option<crate::value::ExpBool>,
        pub manifest_layout: Option<crate::value::ExpString>,
        pub manifest_window_seconds: Option<i32>,
        pub min_buffer_time_seconds: Option<i32>,
        pub min_update_period_seconds: Option<i32>,
        pub period_triggers: Option<Vec<crate::value::ExpString>>,
        pub profile: Option<crate::value::ExpString>,
        pub segment_duration_seconds: Option<i32>,
        pub segment_template_format: Option<crate::value::ExpString>,
        pub stream_selection: Option<Box<StreamSelection_>>,
        pub suggested_presentation_delay_seconds: Option<i32>,
        pub utc_timing: Option<crate::value::ExpString>,
        pub utc_timing_uri: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackage_OriginEndpoint_DashPackage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackage::OriginEndpoint.DashPackage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackage_OriginEndpoint_DashPackage as DashPackage;
    impl crate::value::ToValue for DashPackage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ad_triggers {
                properties.insert(
                    "AdTriggers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ads_on_delivery_restrictions {
                properties.insert(
                    "AdsOnDeliveryRestrictions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.encryption {
                properties.insert(
                    "Encryption".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_iframe_only_stream {
                properties.insert(
                    "IncludeIframeOnlyStream".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.manifest_layout {
                properties.insert(
                    "ManifestLayout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.manifest_window_seconds {
                properties.insert(
                    "ManifestWindowSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.min_buffer_time_seconds {
                properties.insert(
                    "MinBufferTimeSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.min_update_period_seconds {
                properties.insert(
                    "MinUpdatePeriodSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.period_triggers {
                properties.insert(
                    "PeriodTriggers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.profile {
                properties.insert(
                    "Profile".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.segment_duration_seconds {
                properties.insert(
                    "SegmentDurationSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.segment_template_format {
                properties.insert(
                    "SegmentTemplateFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stream_selection {
                properties.insert(
                    "StreamSelection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.suggested_presentation_delay_seconds {
                properties.insert(
                    "SuggestedPresentationDelaySeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.utc_timing {
                properties.insert(
                    "UtcTiming".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.utc_timing_uri {
                properties.insert(
                    "UtcTimingUri".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-encryptioncontractconfiguration.html
    pub struct EncryptionContractConfiguration_ {}
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackage_OriginEndpoint_EncryptionContractConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackage::OriginEndpoint.EncryptionContractConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackage_OriginEndpoint_EncryptionContractConfiguration as EncryptionContractConfiguration;
    impl crate::value::ToValue for EncryptionContractConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            serde_json::Value::Object(serde_json::Map::new())
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-hlsencryption.html
    pub struct HlsEncryption_ {
        pub constant_initialization_vector: Option<crate::value::ExpString>,
        pub encryption_method: Option<crate::value::ExpString>,
        pub key_rotation_interval_seconds: Option<i32>,
        pub repeat_ext_x_key: Option<crate::value::ExpBool>,
        pub speke_key_provider: Box<SpekeKeyProvider_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackage_OriginEndpoint_HlsEncryption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackage::OriginEndpoint.HlsEncryption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackage_OriginEndpoint_HlsEncryption as HlsEncryption;
    impl crate::value::ToValue for HlsEncryption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.constant_initialization_vector {
                properties.insert(
                    "ConstantInitializationVector".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.encryption_method {
                properties.insert(
                    "EncryptionMethod".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.key_rotation_interval_seconds {
                properties.insert(
                    "KeyRotationIntervalSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.repeat_ext_x_key {
                properties.insert(
                    "RepeatExtXKey".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SpekeKeyProvider".to_string(),
                crate::value::ToValue::to_value(&self.speke_key_provider),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-hlsmanifest.html
    pub struct HlsManifest_ {
        pub ad_markers: Option<crate::value::ExpString>,
        pub ad_triggers: Option<Vec<crate::value::ExpString>>,
        pub ads_on_delivery_restrictions: Option<crate::value::ExpString>,
        pub id: crate::value::ExpString,
        pub include_iframe_only_stream: Option<crate::value::ExpBool>,
        pub manifest_name: Option<crate::value::ExpString>,
        pub playlist_type: Option<crate::value::ExpString>,
        pub playlist_window_seconds: Option<i32>,
        pub program_date_time_interval_seconds: Option<i32>,
        pub url: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackage_OriginEndpoint_HlsManifest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackage::OriginEndpoint.HlsManifest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackage_OriginEndpoint_HlsManifest as HlsManifest;
    impl crate::value::ToValue for HlsManifest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ad_markers {
                properties.insert(
                    "AdMarkers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ad_triggers {
                properties.insert(
                    "AdTriggers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ads_on_delivery_restrictions {
                properties.insert(
                    "AdsOnDeliveryRestrictions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            if let Some(ref value) = self.include_iframe_only_stream {
                properties.insert(
                    "IncludeIframeOnlyStream".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.manifest_name {
                properties.insert(
                    "ManifestName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.playlist_type {
                properties.insert(
                    "PlaylistType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.playlist_window_seconds {
                properties.insert(
                    "PlaylistWindowSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.program_date_time_interval_seconds {
                properties.insert(
                    "ProgramDateTimeIntervalSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.url {
                properties.insert("Url".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-hlspackage.html
    pub struct HlsPackage_ {
        pub ad_markers: Option<crate::value::ExpString>,
        pub ad_triggers: Option<Vec<crate::value::ExpString>>,
        pub ads_on_delivery_restrictions: Option<crate::value::ExpString>,
        pub encryption: Option<Box<HlsEncryption_>>,
        pub include_dvb_subtitles: Option<crate::value::ExpBool>,
        pub include_iframe_only_stream: Option<crate::value::ExpBool>,
        pub playlist_type: Option<crate::value::ExpString>,
        pub playlist_window_seconds: Option<i32>,
        pub program_date_time_interval_seconds: Option<i32>,
        pub segment_duration_seconds: Option<i32>,
        pub stream_selection: Option<Box<StreamSelection_>>,
        pub use_audio_rendition_group: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackage_OriginEndpoint_HlsPackage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackage::OriginEndpoint.HlsPackage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackage_OriginEndpoint_HlsPackage as HlsPackage;
    impl crate::value::ToValue for HlsPackage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ad_markers {
                properties.insert(
                    "AdMarkers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ad_triggers {
                properties.insert(
                    "AdTriggers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ads_on_delivery_restrictions {
                properties.insert(
                    "AdsOnDeliveryRestrictions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.encryption {
                properties.insert(
                    "Encryption".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_dvb_subtitles {
                properties.insert(
                    "IncludeDvbSubtitles".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_iframe_only_stream {
                properties.insert(
                    "IncludeIframeOnlyStream".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.playlist_type {
                properties.insert(
                    "PlaylistType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.playlist_window_seconds {
                properties.insert(
                    "PlaylistWindowSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.program_date_time_interval_seconds {
                properties.insert(
                    "ProgramDateTimeIntervalSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.segment_duration_seconds {
                properties.insert(
                    "SegmentDurationSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stream_selection {
                properties.insert(
                    "StreamSelection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.use_audio_rendition_group {
                properties.insert(
                    "UseAudioRenditionGroup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-mssencryption.html
    pub struct MssEncryption_ {
        pub speke_key_provider: Box<SpekeKeyProvider_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackage_OriginEndpoint_MssEncryption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackage::OriginEndpoint.MssEncryption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackage_OriginEndpoint_MssEncryption as MssEncryption;
    impl crate::value::ToValue for MssEncryption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SpekeKeyProvider".to_string(),
                crate::value::ToValue::to_value(&self.speke_key_provider),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-msspackage.html
    pub struct MssPackage_ {
        pub encryption: Option<Box<MssEncryption_>>,
        pub manifest_window_seconds: Option<i32>,
        pub segment_duration_seconds: Option<i32>,
        pub stream_selection: Option<Box<StreamSelection_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackage_OriginEndpoint_MssPackage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackage::OriginEndpoint.MssPackage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackage_OriginEndpoint_MssPackage as MssPackage;
    impl crate::value::ToValue for MssPackage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.encryption {
                properties.insert(
                    "Encryption".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.manifest_window_seconds {
                properties.insert(
                    "ManifestWindowSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.segment_duration_seconds {
                properties.insert(
                    "SegmentDurationSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stream_selection {
                properties.insert(
                    "StreamSelection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-spekekeyprovider.html
    pub struct SpekeKeyProvider_ {
        pub certificate_arn: Option<crate::value::ExpString>,
        pub encryption_contract_configuration: Option<Box<EncryptionContractConfiguration_>>,
        pub resource_id: crate::value::ExpString,
        pub role_arn: crate::value::ExpString,
        pub system_ids: Vec<crate::value::ExpString>,
        pub url: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackage_OriginEndpoint_SpekeKeyProvider {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackage::OriginEndpoint.SpekeKeyProvider"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackage_OriginEndpoint_SpekeKeyProvider as SpekeKeyProvider;
    impl crate::value::ToValue for SpekeKeyProvider_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.certificate_arn {
                properties.insert(
                    "CertificateArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.encryption_contract_configuration {
                properties.insert(
                    "EncryptionContractConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ResourceId".to_string(),
                crate::value::ToValue::to_value(&self.resource_id),
            );
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.insert(
                "SystemIds".to_string(),
                crate::value::ToValue::to_value(&self.system_ids),
            );
            properties.insert(
                "Url".to_string(),
                crate::value::ToValue::to_value(&self.url),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-streamselection.html
    pub struct StreamSelection_ {
        pub max_video_bits_per_second: Option<i32>,
        pub min_video_bits_per_second: Option<i32>,
        pub stream_order: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackage_OriginEndpoint_StreamSelection {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackage::OriginEndpoint.StreamSelection"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackage_OriginEndpoint_StreamSelection as StreamSelection;
    impl crate::value::ToValue for StreamSelection_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max_video_bits_per_second {
                properties.insert(
                    "MaxVideoBitsPerSecond".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.min_video_bits_per_second {
                properties.insert(
                    "MinVideoBitsPerSecond".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stream_order {
                properties.insert(
                    "StreamOrder".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod packagingconfiguration {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-cmafencryption.html
    pub struct CmafEncryption_ {
        pub speke_key_provider: Box<SpekeKeyProvider_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackage_PackagingConfiguration_CmafEncryption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackage::PackagingConfiguration.CmafEncryption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackage_PackagingConfiguration_CmafEncryption as CmafEncryption;
    impl crate::value::ToValue for CmafEncryption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SpekeKeyProvider".to_string(),
                crate::value::ToValue::to_value(&self.speke_key_provider),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-cmafpackage.html
    pub struct CmafPackage_ {
        pub encryption: Option<Box<CmafEncryption_>>,
        pub hls_manifests: Vec<HlsManifest_>,
        pub include_encoder_configuration_in_segments: Option<crate::value::ExpBool>,
        pub segment_duration_seconds: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackage_PackagingConfiguration_CmafPackage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackage::PackagingConfiguration.CmafPackage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackage_PackagingConfiguration_CmafPackage as CmafPackage;
    impl crate::value::ToValue for CmafPackage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.encryption {
                properties.insert(
                    "Encryption".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "HlsManifests".to_string(),
                crate::value::ToValue::to_value(&self.hls_manifests),
            );
            if let Some(ref value) = self.include_encoder_configuration_in_segments {
                properties.insert(
                    "IncludeEncoderConfigurationInSegments".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.segment_duration_seconds {
                properties.insert(
                    "SegmentDurationSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-dashencryption.html
    pub struct DashEncryption_ {
        pub speke_key_provider: Box<SpekeKeyProvider_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackage_PackagingConfiguration_DashEncryption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackage::PackagingConfiguration.DashEncryption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackage_PackagingConfiguration_DashEncryption as DashEncryption;
    impl crate::value::ToValue for DashEncryption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SpekeKeyProvider".to_string(),
                crate::value::ToValue::to_value(&self.speke_key_provider),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-dashmanifest.html
    pub struct DashManifest_ {
        pub manifest_layout: Option<crate::value::ExpString>,
        pub manifest_name: Option<crate::value::ExpString>,
        pub min_buffer_time_seconds: Option<i32>,
        pub profile: Option<crate::value::ExpString>,
        pub scte_markers_source: Option<crate::value::ExpString>,
        pub stream_selection: Option<Box<StreamSelection_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackage_PackagingConfiguration_DashManifest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackage::PackagingConfiguration.DashManifest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackage_PackagingConfiguration_DashManifest as DashManifest;
    impl crate::value::ToValue for DashManifest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.manifest_layout {
                properties.insert(
                    "ManifestLayout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.manifest_name {
                properties.insert(
                    "ManifestName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.min_buffer_time_seconds {
                properties.insert(
                    "MinBufferTimeSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.profile {
                properties.insert(
                    "Profile".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scte_markers_source {
                properties.insert(
                    "ScteMarkersSource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stream_selection {
                properties.insert(
                    "StreamSelection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-dashpackage.html
    pub struct DashPackage_ {
        pub dash_manifests: Vec<DashManifest_>,
        pub encryption: Option<Box<DashEncryption_>>,
        pub include_encoder_configuration_in_segments: Option<crate::value::ExpBool>,
        pub include_iframe_only_stream: Option<crate::value::ExpBool>,
        pub period_triggers: Option<Vec<crate::value::ExpString>>,
        pub segment_duration_seconds: Option<i32>,
        pub segment_template_format: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackage_PackagingConfiguration_DashPackage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackage::PackagingConfiguration.DashPackage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackage_PackagingConfiguration_DashPackage as DashPackage;
    impl crate::value::ToValue for DashPackage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DashManifests".to_string(),
                crate::value::ToValue::to_value(&self.dash_manifests),
            );
            if let Some(ref value) = self.encryption {
                properties.insert(
                    "Encryption".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_encoder_configuration_in_segments {
                properties.insert(
                    "IncludeEncoderConfigurationInSegments".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_iframe_only_stream {
                properties.insert(
                    "IncludeIframeOnlyStream".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.period_triggers {
                properties.insert(
                    "PeriodTriggers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.segment_duration_seconds {
                properties.insert(
                    "SegmentDurationSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.segment_template_format {
                properties.insert(
                    "SegmentTemplateFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-encryptioncontractconfiguration.html
    pub struct EncryptionContractConfiguration_ {
        pub preset_speke20_audio: crate::value::ExpString,
        pub preset_speke20_video: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackage_PackagingConfiguration_EncryptionContractConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackage::PackagingConfiguration.EncryptionContractConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackage_PackagingConfiguration_EncryptionContractConfiguration as EncryptionContractConfiguration;
    impl crate::value::ToValue for EncryptionContractConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PresetSpeke20Audio".to_string(),
                crate::value::ToValue::to_value(&self.preset_speke20_audio),
            );
            properties.insert(
                "PresetSpeke20Video".to_string(),
                crate::value::ToValue::to_value(&self.preset_speke20_video),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-hlsencryption.html
    pub struct HlsEncryption_ {
        pub constant_initialization_vector: Option<crate::value::ExpString>,
        pub encryption_method: Option<crate::value::ExpString>,
        pub speke_key_provider: Box<SpekeKeyProvider_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackage_PackagingConfiguration_HlsEncryption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackage::PackagingConfiguration.HlsEncryption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackage_PackagingConfiguration_HlsEncryption as HlsEncryption;
    impl crate::value::ToValue for HlsEncryption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.constant_initialization_vector {
                properties.insert(
                    "ConstantInitializationVector".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.encryption_method {
                properties.insert(
                    "EncryptionMethod".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SpekeKeyProvider".to_string(),
                crate::value::ToValue::to_value(&self.speke_key_provider),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-hlsmanifest.html
    pub struct HlsManifest_ {
        pub ad_markers: Option<crate::value::ExpString>,
        pub include_iframe_only_stream: Option<crate::value::ExpBool>,
        pub manifest_name: Option<crate::value::ExpString>,
        pub program_date_time_interval_seconds: Option<i32>,
        pub repeat_ext_x_key: Option<crate::value::ExpBool>,
        pub stream_selection: Option<Box<StreamSelection_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackage_PackagingConfiguration_HlsManifest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackage::PackagingConfiguration.HlsManifest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackage_PackagingConfiguration_HlsManifest as HlsManifest;
    impl crate::value::ToValue for HlsManifest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ad_markers {
                properties.insert(
                    "AdMarkers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_iframe_only_stream {
                properties.insert(
                    "IncludeIframeOnlyStream".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.manifest_name {
                properties.insert(
                    "ManifestName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.program_date_time_interval_seconds {
                properties.insert(
                    "ProgramDateTimeIntervalSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.repeat_ext_x_key {
                properties.insert(
                    "RepeatExtXKey".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stream_selection {
                properties.insert(
                    "StreamSelection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-hlspackage.html
    pub struct HlsPackage_ {
        pub encryption: Option<Box<HlsEncryption_>>,
        pub hls_manifests: Vec<HlsManifest_>,
        pub include_dvb_subtitles: Option<crate::value::ExpBool>,
        pub segment_duration_seconds: Option<i32>,
        pub use_audio_rendition_group: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackage_PackagingConfiguration_HlsPackage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackage::PackagingConfiguration.HlsPackage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackage_PackagingConfiguration_HlsPackage as HlsPackage;
    impl crate::value::ToValue for HlsPackage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.encryption {
                properties.insert(
                    "Encryption".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "HlsManifests".to_string(),
                crate::value::ToValue::to_value(&self.hls_manifests),
            );
            if let Some(ref value) = self.include_dvb_subtitles {
                properties.insert(
                    "IncludeDvbSubtitles".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.segment_duration_seconds {
                properties.insert(
                    "SegmentDurationSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.use_audio_rendition_group {
                properties.insert(
                    "UseAudioRenditionGroup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-mssencryption.html
    pub struct MssEncryption_ {
        pub speke_key_provider: Box<SpekeKeyProvider_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackage_PackagingConfiguration_MssEncryption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackage::PackagingConfiguration.MssEncryption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackage_PackagingConfiguration_MssEncryption as MssEncryption;
    impl crate::value::ToValue for MssEncryption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SpekeKeyProvider".to_string(),
                crate::value::ToValue::to_value(&self.speke_key_provider),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-mssmanifest.html
    pub struct MssManifest_ {
        pub manifest_name: Option<crate::value::ExpString>,
        pub stream_selection: Option<Box<StreamSelection_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackage_PackagingConfiguration_MssManifest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackage::PackagingConfiguration.MssManifest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackage_PackagingConfiguration_MssManifest as MssManifest;
    impl crate::value::ToValue for MssManifest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.manifest_name {
                properties.insert(
                    "ManifestName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stream_selection {
                properties.insert(
                    "StreamSelection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-msspackage.html
    pub struct MssPackage_ {
        pub encryption: Option<Box<MssEncryption_>>,
        pub mss_manifests: Vec<MssManifest_>,
        pub segment_duration_seconds: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackage_PackagingConfiguration_MssPackage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackage::PackagingConfiguration.MssPackage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackage_PackagingConfiguration_MssPackage as MssPackage;
    impl crate::value::ToValue for MssPackage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.encryption {
                properties.insert(
                    "Encryption".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MssManifests".to_string(),
                crate::value::ToValue::to_value(&self.mss_manifests),
            );
            if let Some(ref value) = self.segment_duration_seconds {
                properties.insert(
                    "SegmentDurationSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-spekekeyprovider.html
    pub struct SpekeKeyProvider_ {
        pub encryption_contract_configuration: Option<Box<EncryptionContractConfiguration_>>,
        pub role_arn: crate::value::ExpString,
        pub system_ids: Vec<crate::value::ExpString>,
        pub url: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackage_PackagingConfiguration_SpekeKeyProvider {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackage::PackagingConfiguration.SpekeKeyProvider"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackage_PackagingConfiguration_SpekeKeyProvider as SpekeKeyProvider;
    impl crate::value::ToValue for SpekeKeyProvider_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.encryption_contract_configuration {
                properties.insert(
                    "EncryptionContractConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.insert(
                "SystemIds".to_string(),
                crate::value::ToValue::to_value(&self.system_ids),
            );
            properties.insert(
                "Url".to_string(),
                crate::value::ToValue::to_value(&self.url),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-streamselection.html
    pub struct StreamSelection_ {
        pub max_video_bits_per_second: Option<i32>,
        pub min_video_bits_per_second: Option<i32>,
        pub stream_order: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackage_PackagingConfiguration_StreamSelection {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackage::PackagingConfiguration.StreamSelection"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackage_PackagingConfiguration_StreamSelection as StreamSelection;
    impl crate::value::ToValue for StreamSelection_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max_video_bits_per_second {
                properties.insert(
                    "MaxVideoBitsPerSecond".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.min_video_bits_per_second {
                properties.insert(
                    "MinVideoBitsPerSecond".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stream_order {
                properties.insert(
                    "StreamOrder".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod packaginggroup {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packaginggroup-authorization.html
    pub struct Authorization_ {
        pub cdn_identifier_secret: crate::value::ExpString,
        pub secrets_role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackage_PackagingGroup_Authorization {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackage::PackagingGroup.Authorization"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackage_PackagingGroup_Authorization as Authorization;
    impl crate::value::ToValue for Authorization_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CdnIdentifierSecret".to_string(),
                crate::value::ToValue::to_value(&self.cdn_identifier_secret),
            );
            properties.insert(
                "SecretsRoleArn".to_string(),
                crate::value::ToValue::to_value(&self.secrets_role_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packaginggroup-logconfiguration.html
    pub struct LogConfiguration_ {
        pub log_group_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackage_PackagingGroup_LogConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackage::PackagingGroup.LogConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackage_PackagingGroup_LogConfiguration as LogConfiguration;
    impl crate::value::ToValue for LogConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.log_group_name {
                properties.insert(
                    "LogGroupName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-asset.html
pub struct Asset_ {
    pub egress_endpoints: Option<Vec<super::mediapackage::asset::EgressEndpoint_>>,
    pub id: crate::value::ExpString,
    pub packaging_group_id: crate::value::ExpString,
    pub resource_id: Option<crate::value::ExpString>,
    pub source_arn: crate::value::ExpString,
    pub source_role_arn: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_mediapackage_Asset {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MediaPackage::Asset"
        $($field $value)*)
    };
}
pub use crate::__aws_mediapackage_Asset as Asset;
impl crate::template::ToResource for Asset_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaPackage"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Asset"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.egress_endpoints {
            properties.insert(
                "EgressEndpoints".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
        properties.insert(
            "PackagingGroupId".to_string(),
            crate::value::ToValue::to_value(&self.packaging_group_id),
        );
        if let Some(ref value) = self.resource_id {
            properties.insert(
                "ResourceId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SourceArn".to_string(),
            crate::value::ToValue::to_value(&self.source_arn),
        );
        properties.insert(
            "SourceRoleArn".to_string(),
            crate::value::ToValue::to_value(&self.source_role_arn),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-channel.html
pub struct Channel_ {
    pub description: Option<crate::value::ExpString>,
    pub egress_access_logs: Option<super::mediapackage::channel::LogConfiguration_>,
    pub hls_ingest: Option<super::mediapackage::channel::HlsIngest_>,
    pub id: crate::value::ExpString,
    pub ingress_access_logs: Option<super::mediapackage::channel::LogConfiguration_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_mediapackage_Channel {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MediaPackage::Channel"
        $($field $value)*)
    };
}
pub use crate::__aws_mediapackage_Channel as Channel;
impl crate::template::ToResource for Channel_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaPackage"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Channel"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.egress_access_logs {
            properties.insert(
                "EgressAccessLogs".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.hls_ingest {
            properties.insert(
                "HlsIngest".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
        if let Some(ref value) = self.ingress_access_logs {
            properties.insert(
                "IngressAccessLogs".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-originendpoint.html
pub struct OriginEndpoint_ {
    pub authorization: Option<super::mediapackage::originendpoint::Authorization_>,
    pub channel_id: crate::value::ExpString,
    pub cmaf_package: Option<super::mediapackage::originendpoint::CmafPackage_>,
    pub dash_package: Option<super::mediapackage::originendpoint::DashPackage_>,
    pub description: Option<crate::value::ExpString>,
    pub hls_package: Option<super::mediapackage::originendpoint::HlsPackage_>,
    pub id: crate::value::ExpString,
    pub manifest_name: Option<crate::value::ExpString>,
    pub mss_package: Option<super::mediapackage::originendpoint::MssPackage_>,
    pub origination: Option<crate::value::ExpString>,
    pub startover_window_seconds: Option<i32>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub time_delay_seconds: Option<i32>,
    pub whitelist: Option<Vec<crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_mediapackage_OriginEndpoint {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MediaPackage::OriginEndpoint"
        $($field $value)*)
    };
}
pub use crate::__aws_mediapackage_OriginEndpoint as OriginEndpoint;
impl crate::template::ToResource for OriginEndpoint_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaPackage"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("OriginEndpoint"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.authorization {
            properties.insert(
                "Authorization".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ChannelId".to_string(),
            crate::value::ToValue::to_value(&self.channel_id),
        );
        if let Some(ref value) = self.cmaf_package {
            properties.insert(
                "CmafPackage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.dash_package {
            properties.insert(
                "DashPackage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.hls_package {
            properties.insert(
                "HlsPackage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
        if let Some(ref value) = self.manifest_name {
            properties.insert(
                "ManifestName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.mss_package {
            properties.insert(
                "MssPackage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.origination {
            properties.insert(
                "Origination".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.startover_window_seconds {
            properties.insert(
                "StartoverWindowSeconds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.time_delay_seconds {
            properties.insert(
                "TimeDelaySeconds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.whitelist {
            properties.insert(
                "Whitelist".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-packagingconfiguration.html
pub struct PackagingConfiguration_ {
    pub cmaf_package: Option<super::mediapackage::packagingconfiguration::CmafPackage_>,
    pub dash_package: Option<super::mediapackage::packagingconfiguration::DashPackage_>,
    pub hls_package: Option<super::mediapackage::packagingconfiguration::HlsPackage_>,
    pub id: crate::value::ExpString,
    pub mss_package: Option<super::mediapackage::packagingconfiguration::MssPackage_>,
    pub packaging_group_id: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_mediapackage_PackagingConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MediaPackage::PackagingConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_mediapackage_PackagingConfiguration as PackagingConfiguration;
impl crate::template::ToResource for PackagingConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaPackage"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PackagingConfiguration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.cmaf_package {
            properties.insert(
                "CmafPackage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.dash_package {
            properties.insert(
                "DashPackage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.hls_package {
            properties.insert(
                "HlsPackage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
        if let Some(ref value) = self.mss_package {
            properties.insert(
                "MssPackage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "PackagingGroupId".to_string(),
            crate::value::ToValue::to_value(&self.packaging_group_id),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-packaginggroup.html
pub struct PackagingGroup_ {
    pub authorization: Option<super::mediapackage::packaginggroup::Authorization_>,
    pub egress_access_logs: Option<super::mediapackage::packaginggroup::LogConfiguration_>,
    pub id: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_mediapackage_PackagingGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MediaPackage::PackagingGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_mediapackage_PackagingGroup as PackagingGroup;
impl crate::template::ToResource for PackagingGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaPackage"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PackagingGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.authorization {
            properties.insert(
                "Authorization".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.egress_access_logs {
            properties.insert(
                "EgressAccessLogs".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
