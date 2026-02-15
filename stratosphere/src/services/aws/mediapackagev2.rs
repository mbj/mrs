pub mod channel {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-channel-ingestendpoint.html
    pub struct IngestEndpoint_ {
        pub id: Option<crate::value::ExpString>,
        pub url: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackagev2_Channel_IngestEndpoint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackageV2::Channel.IngestEndpoint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackagev2_Channel_IngestEndpoint as IngestEndpoint;
    impl crate::value::ToValue for IngestEndpoint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.id {
                properties.insert("Id".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.url {
                properties.insert("Url".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-channel-inputswitchconfiguration.html
    pub struct InputSwitchConfiguration_ {
        pub mqcs_input_switching: Option<crate::value::ExpBool>,
        pub preferred_input: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackagev2_Channel_InputSwitchConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackageV2::Channel.InputSwitchConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackagev2_Channel_InputSwitchConfiguration as InputSwitchConfiguration;
    impl crate::value::ToValue for InputSwitchConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.mqcs_input_switching {
                properties.insert(
                    "MQCSInputSwitching".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.preferred_input {
                properties.insert(
                    "PreferredInput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-channel-outputheaderconfiguration.html
    pub struct OutputHeaderConfiguration_ {
        pub publish_mqcs: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackagev2_Channel_OutputHeaderConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackageV2::Channel.OutputHeaderConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackagev2_Channel_OutputHeaderConfiguration as OutputHeaderConfiguration;
    impl crate::value::ToValue for OutputHeaderConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.publish_mqcs {
                properties.insert(
                    "PublishMQCS".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod originendpoint {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-dashbaseurl.html
    pub struct DashBaseUrl_ {
        pub dvb_priority: Option<i32>,
        pub dvb_weight: Option<i32>,
        pub service_location: Option<crate::value::ExpString>,
        pub url: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackagev2_OriginEndpoint_DashBaseUrl {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackageV2::OriginEndpoint.DashBaseUrl"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackagev2_OriginEndpoint_DashBaseUrl as DashBaseUrl;
    impl crate::value::ToValue for DashBaseUrl_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dvb_priority {
                properties.insert(
                    "DvbPriority".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dvb_weight {
                properties.insert(
                    "DvbWeight".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.service_location {
                properties.insert(
                    "ServiceLocation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Url".to_string(),
                crate::value::ToValue::to_value(&self.url),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-dashdvbfontdownload.html
    pub struct DashDvbFontDownload_ {
        pub font_family: Option<crate::value::ExpString>,
        pub mime_type: Option<crate::value::ExpString>,
        pub url: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackagev2_OriginEndpoint_DashDvbFontDownload {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackageV2::OriginEndpoint.DashDvbFontDownload"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackagev2_OriginEndpoint_DashDvbFontDownload as DashDvbFontDownload;
    impl crate::value::ToValue for DashDvbFontDownload_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.font_family {
                properties.insert(
                    "FontFamily".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mime_type {
                properties.insert(
                    "MimeType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.url {
                properties.insert("Url".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-dashdvbmetricsreporting.html
    pub struct DashDvbMetricsReporting_ {
        pub probability: Option<i32>,
        pub reporting_url: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackagev2_OriginEndpoint_DashDvbMetricsReporting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackageV2::OriginEndpoint.DashDvbMetricsReporting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackagev2_OriginEndpoint_DashDvbMetricsReporting as DashDvbMetricsReporting;
    impl crate::value::ToValue for DashDvbMetricsReporting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.probability {
                properties.insert(
                    "Probability".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ReportingUrl".to_string(),
                crate::value::ToValue::to_value(&self.reporting_url),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-dashdvbsettings.html
    pub struct DashDvbSettings_ {
        pub error_metrics: Option<Vec<DashDvbMetricsReporting_>>,
        pub font_download: Option<Box<DashDvbFontDownload_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackagev2_OriginEndpoint_DashDvbSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackageV2::OriginEndpoint.DashDvbSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackagev2_OriginEndpoint_DashDvbSettings as DashDvbSettings;
    impl crate::value::ToValue for DashDvbSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.error_metrics {
                properties.insert(
                    "ErrorMetrics".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.font_download {
                properties.insert(
                    "FontDownload".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-dashmanifestconfiguration.html
    pub struct DashManifestConfiguration_ {
        pub base_urls: Option<Vec<DashBaseUrl_>>,
        pub compactness: Option<crate::value::ExpString>,
        pub drm_signaling: Option<crate::value::ExpString>,
        pub dvb_settings: Option<Box<DashDvbSettings_>>,
        pub filter_configuration: Option<Box<FilterConfiguration_>>,
        pub manifest_name: crate::value::ExpString,
        pub manifest_window_seconds: Option<i32>,
        pub min_buffer_time_seconds: Option<i32>,
        pub min_update_period_seconds: Option<i32>,
        pub period_triggers: Option<Vec<crate::value::ExpString>>,
        pub profiles: Option<Vec<crate::value::ExpString>>,
        pub program_information: Option<Box<DashProgramInformation_>>,
        pub scte_dash: Option<Box<ScteDash_>>,
        pub segment_template_format: Option<crate::value::ExpString>,
        pub subtitle_configuration: Option<Box<DashSubtitleConfiguration_>>,
        pub suggested_presentation_delay_seconds: Option<i32>,
        pub utc_timing: Option<Box<DashUtcTiming_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackagev2_OriginEndpoint_DashManifestConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackageV2::OriginEndpoint.DashManifestConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackagev2_OriginEndpoint_DashManifestConfiguration as DashManifestConfiguration;
    impl crate::value::ToValue for DashManifestConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.base_urls {
                properties.insert(
                    "BaseUrls".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.compactness {
                properties.insert(
                    "Compactness".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.drm_signaling {
                properties.insert(
                    "DrmSignaling".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dvb_settings {
                properties.insert(
                    "DvbSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.filter_configuration {
                properties.insert(
                    "FilterConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ManifestName".to_string(),
                crate::value::ToValue::to_value(&self.manifest_name),
            );
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
            if let Some(ref value) = self.profiles {
                properties.insert(
                    "Profiles".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.program_information {
                properties.insert(
                    "ProgramInformation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scte_dash {
                properties.insert(
                    "ScteDash".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.segment_template_format {
                properties.insert(
                    "SegmentTemplateFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.subtitle_configuration {
                properties.insert(
                    "SubtitleConfiguration".to_string(),
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
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-dashprograminformation.html
    pub struct DashProgramInformation_ {
        pub copyright: Option<crate::value::ExpString>,
        pub language_code: Option<crate::value::ExpString>,
        pub more_information_url: Option<crate::value::ExpString>,
        pub source: Option<crate::value::ExpString>,
        pub title: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackagev2_OriginEndpoint_DashProgramInformation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackageV2::OriginEndpoint.DashProgramInformation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackagev2_OriginEndpoint_DashProgramInformation as DashProgramInformation;
    impl crate::value::ToValue for DashProgramInformation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.copyright {
                properties.insert(
                    "Copyright".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.language_code {
                properties.insert(
                    "LanguageCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.more_information_url {
                properties.insert(
                    "MoreInformationUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source {
                properties.insert("Source".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.title {
                properties.insert("Title".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-dashsubtitleconfiguration.html
    pub struct DashSubtitleConfiguration_ {
        pub ttml_configuration: Option<Box<DashTtmlConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackagev2_OriginEndpoint_DashSubtitleConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackageV2::OriginEndpoint.DashSubtitleConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackagev2_OriginEndpoint_DashSubtitleConfiguration as DashSubtitleConfiguration;
    impl crate::value::ToValue for DashSubtitleConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ttml_configuration {
                properties.insert(
                    "TtmlConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-dashttmlconfiguration.html
    pub struct DashTtmlConfiguration_ {
        pub ttml_profile: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackagev2_OriginEndpoint_DashTtmlConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackageV2::OriginEndpoint.DashTtmlConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackagev2_OriginEndpoint_DashTtmlConfiguration as DashTtmlConfiguration;
    impl crate::value::ToValue for DashTtmlConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TtmlProfile".to_string(),
                crate::value::ToValue::to_value(&self.ttml_profile),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-dashutctiming.html
    pub struct DashUtcTiming_ {
        pub timing_mode: Option<crate::value::ExpString>,
        pub timing_source: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackagev2_OriginEndpoint_DashUtcTiming {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackageV2::OriginEndpoint.DashUtcTiming"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackagev2_OriginEndpoint_DashUtcTiming as DashUtcTiming;
    impl crate::value::ToValue for DashUtcTiming_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.timing_mode {
                properties.insert(
                    "TimingMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timing_source {
                properties.insert(
                    "TimingSource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-encryption.html
    pub struct Encryption_ {
        pub cmaf_exclude_segment_drm_metadata: Option<crate::value::ExpBool>,
        pub constant_initialization_vector: Option<crate::value::ExpString>,
        pub encryption_method: Box<EncryptionMethod_>,
        pub key_rotation_interval_seconds: Option<i32>,
        pub speke_key_provider: Box<SpekeKeyProvider_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackagev2_OriginEndpoint_Encryption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackageV2::OriginEndpoint.Encryption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackagev2_OriginEndpoint_Encryption as Encryption;
    impl crate::value::ToValue for Encryption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cmaf_exclude_segment_drm_metadata {
                properties.insert(
                    "CmafExcludeSegmentDrmMetadata".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.constant_initialization_vector {
                properties.insert(
                    "ConstantInitializationVector".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "EncryptionMethod".to_string(),
                crate::value::ToValue::to_value(&self.encryption_method),
            );
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-encryptioncontractconfiguration.html
    pub struct EncryptionContractConfiguration_ {
        pub preset_speke20_audio: crate::value::ExpString,
        pub preset_speke20_video: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackagev2_OriginEndpoint_EncryptionContractConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackageV2::OriginEndpoint.EncryptionContractConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackagev2_OriginEndpoint_EncryptionContractConfiguration as EncryptionContractConfiguration;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-encryptionmethod.html
    pub struct EncryptionMethod_ {
        pub cmaf_encryption_method: Option<crate::value::ExpString>,
        pub ism_encryption_method: Option<crate::value::ExpString>,
        pub ts_encryption_method: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackagev2_OriginEndpoint_EncryptionMethod {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackageV2::OriginEndpoint.EncryptionMethod"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackagev2_OriginEndpoint_EncryptionMethod as EncryptionMethod;
    impl crate::value::ToValue for EncryptionMethod_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cmaf_encryption_method {
                properties.insert(
                    "CmafEncryptionMethod".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ism_encryption_method {
                properties.insert(
                    "IsmEncryptionMethod".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ts_encryption_method {
                properties.insert(
                    "TsEncryptionMethod".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-filterconfiguration.html
    pub struct FilterConfiguration_ {
        pub clip_start_time: Option<crate::value::ExpString>,
        pub drm_settings: Option<crate::value::ExpString>,
        pub end: Option<crate::value::ExpString>,
        pub manifest_filter: Option<crate::value::ExpString>,
        pub start: Option<crate::value::ExpString>,
        pub time_delay_seconds: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackagev2_OriginEndpoint_FilterConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackageV2::OriginEndpoint.FilterConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackagev2_OriginEndpoint_FilterConfiguration as FilterConfiguration;
    impl crate::value::ToValue for FilterConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.clip_start_time {
                properties.insert(
                    "ClipStartTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.drm_settings {
                properties.insert(
                    "DrmSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.end {
                properties.insert("End".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.manifest_filter {
                properties.insert(
                    "ManifestFilter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.start {
                properties.insert("Start".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.time_delay_seconds {
                properties.insert(
                    "TimeDelaySeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-forceendpointerrorconfiguration.html
    pub struct ForceEndpointErrorConfiguration_ {
        pub endpoint_error_conditions: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackagev2_OriginEndpoint_ForceEndpointErrorConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackageV2::OriginEndpoint.ForceEndpointErrorConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackagev2_OriginEndpoint_ForceEndpointErrorConfiguration as ForceEndpointErrorConfiguration;
    impl crate::value::ToValue for ForceEndpointErrorConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.endpoint_error_conditions {
                properties.insert(
                    "EndpointErrorConditions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-hlsmanifestconfiguration.html
    pub struct HlsManifestConfiguration_ {
        pub child_manifest_name: Option<crate::value::ExpString>,
        pub filter_configuration: Option<Box<FilterConfiguration_>>,
        pub manifest_name: crate::value::ExpString,
        pub manifest_window_seconds: Option<i32>,
        pub program_date_time_interval_seconds: Option<i32>,
        pub scte_hls: Option<Box<ScteHls_>>,
        pub start_tag: Option<Box<StartTag_>>,
        pub url: Option<crate::value::ExpString>,
        pub url_encode_child_manifest: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackagev2_OriginEndpoint_HlsManifestConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackageV2::OriginEndpoint.HlsManifestConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackagev2_OriginEndpoint_HlsManifestConfiguration as HlsManifestConfiguration;
    impl crate::value::ToValue for HlsManifestConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.child_manifest_name {
                properties.insert(
                    "ChildManifestName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.filter_configuration {
                properties.insert(
                    "FilterConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ManifestName".to_string(),
                crate::value::ToValue::to_value(&self.manifest_name),
            );
            if let Some(ref value) = self.manifest_window_seconds {
                properties.insert(
                    "ManifestWindowSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.program_date_time_interval_seconds {
                properties.insert(
                    "ProgramDateTimeIntervalSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scte_hls {
                properties.insert(
                    "ScteHls".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.start_tag {
                properties.insert(
                    "StartTag".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.url {
                properties.insert("Url".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.url_encode_child_manifest {
                properties.insert(
                    "UrlEncodeChildManifest".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-lowlatencyhlsmanifestconfiguration.html
    pub struct LowLatencyHlsManifestConfiguration_ {
        pub child_manifest_name: Option<crate::value::ExpString>,
        pub filter_configuration: Option<Box<FilterConfiguration_>>,
        pub manifest_name: crate::value::ExpString,
        pub manifest_window_seconds: Option<i32>,
        pub program_date_time_interval_seconds: Option<i32>,
        pub scte_hls: Option<Box<ScteHls_>>,
        pub start_tag: Option<Box<StartTag_>>,
        pub url: Option<crate::value::ExpString>,
        pub url_encode_child_manifest: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackagev2_OriginEndpoint_LowLatencyHlsManifestConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackageV2::OriginEndpoint.LowLatencyHlsManifestConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackagev2_OriginEndpoint_LowLatencyHlsManifestConfiguration as LowLatencyHlsManifestConfiguration;
    impl crate::value::ToValue for LowLatencyHlsManifestConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.child_manifest_name {
                properties.insert(
                    "ChildManifestName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.filter_configuration {
                properties.insert(
                    "FilterConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ManifestName".to_string(),
                crate::value::ToValue::to_value(&self.manifest_name),
            );
            if let Some(ref value) = self.manifest_window_seconds {
                properties.insert(
                    "ManifestWindowSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.program_date_time_interval_seconds {
                properties.insert(
                    "ProgramDateTimeIntervalSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scte_hls {
                properties.insert(
                    "ScteHls".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.start_tag {
                properties.insert(
                    "StartTag".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.url {
                properties.insert("Url".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.url_encode_child_manifest {
                properties.insert(
                    "UrlEncodeChildManifest".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-mssmanifestconfiguration.html
    pub struct MssManifestConfiguration_ {
        pub filter_configuration: Option<Box<FilterConfiguration_>>,
        pub manifest_layout: Option<crate::value::ExpString>,
        pub manifest_name: crate::value::ExpString,
        pub manifest_window_seconds: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackagev2_OriginEndpoint_MssManifestConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackageV2::OriginEndpoint.MssManifestConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackagev2_OriginEndpoint_MssManifestConfiguration as MssManifestConfiguration;
    impl crate::value::ToValue for MssManifestConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.filter_configuration {
                properties.insert(
                    "FilterConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.manifest_layout {
                properties.insert(
                    "ManifestLayout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ManifestName".to_string(),
                crate::value::ToValue::to_value(&self.manifest_name),
            );
            if let Some(ref value) = self.manifest_window_seconds {
                properties.insert(
                    "ManifestWindowSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-scte.html
    pub struct Scte_ {
        pub scte_filter: Option<Vec<crate::value::ExpString>>,
        pub scte_in_segments: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackagev2_OriginEndpoint_Scte {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackageV2::OriginEndpoint.Scte"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackagev2_OriginEndpoint_Scte as Scte;
    impl crate::value::ToValue for Scte_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.scte_filter {
                properties.insert(
                    "ScteFilter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scte_in_segments {
                properties.insert(
                    "ScteInSegments".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-sctedash.html
    pub struct ScteDash_ {
        pub ad_marker_dash: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackagev2_OriginEndpoint_ScteDash {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackageV2::OriginEndpoint.ScteDash"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackagev2_OriginEndpoint_ScteDash as ScteDash;
    impl crate::value::ToValue for ScteDash_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ad_marker_dash {
                properties.insert(
                    "AdMarkerDash".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-sctehls.html
    pub struct ScteHls_ {
        pub ad_marker_hls: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackagev2_OriginEndpoint_ScteHls {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackageV2::OriginEndpoint.ScteHls"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackagev2_OriginEndpoint_ScteHls as ScteHls;
    impl crate::value::ToValue for ScteHls_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ad_marker_hls {
                properties.insert(
                    "AdMarkerHls".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-segment.html
    pub struct Segment_ {
        pub encryption: Option<Box<Encryption_>>,
        pub include_iframe_only_streams: Option<crate::value::ExpBool>,
        pub scte: Option<Box<Scte_>>,
        pub segment_duration_seconds: Option<i32>,
        pub segment_name: Option<crate::value::ExpString>,
        pub ts_include_dvb_subtitles: Option<crate::value::ExpBool>,
        pub ts_use_audio_rendition_group: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackagev2_OriginEndpoint_Segment {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackageV2::OriginEndpoint.Segment"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackagev2_OriginEndpoint_Segment as Segment;
    impl crate::value::ToValue for Segment_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.encryption {
                properties.insert(
                    "Encryption".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_iframe_only_streams {
                properties.insert(
                    "IncludeIframeOnlyStreams".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scte {
                properties.insert("Scte".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.segment_duration_seconds {
                properties.insert(
                    "SegmentDurationSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.segment_name {
                properties.insert(
                    "SegmentName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ts_include_dvb_subtitles {
                properties.insert(
                    "TsIncludeDvbSubtitles".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ts_use_audio_rendition_group {
                properties.insert(
                    "TsUseAudioRenditionGroup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-spekekeyprovider.html
    pub struct SpekeKeyProvider_ {
        pub certificate_arn: Option<crate::value::ExpString>,
        pub drm_systems: Vec<crate::value::ExpString>,
        pub encryption_contract_configuration: Box<EncryptionContractConfiguration_>,
        pub resource_id: crate::value::ExpString,
        pub role_arn: crate::value::ExpString,
        pub url: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackagev2_OriginEndpoint_SpekeKeyProvider {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackageV2::OriginEndpoint.SpekeKeyProvider"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackagev2_OriginEndpoint_SpekeKeyProvider as SpekeKeyProvider;
    impl crate::value::ToValue for SpekeKeyProvider_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.certificate_arn {
                properties.insert(
                    "CertificateArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DrmSystems".to_string(),
                crate::value::ToValue::to_value(&self.drm_systems),
            );
            properties.insert(
                "EncryptionContractConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.encryption_contract_configuration),
            );
            properties.insert(
                "ResourceId".to_string(),
                crate::value::ToValue::to_value(&self.resource_id),
            );
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.insert(
                "Url".to_string(),
                crate::value::ToValue::to_value(&self.url),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-starttag.html
    pub struct StartTag_ {
        pub precise: Option<crate::value::ExpBool>,
        pub time_offset: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackagev2_OriginEndpoint_StartTag {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackageV2::OriginEndpoint.StartTag"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackagev2_OriginEndpoint_StartTag as StartTag;
    impl crate::value::ToValue for StartTag_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.precise {
                properties.insert(
                    "Precise".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TimeOffset".to_string(),
                crate::value::ToValue::to_value(&self.time_offset),
            );
            properties.into()
        }
    }
}
pub mod originendpointpolicy {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpointpolicy-cdnauthconfiguration.html
    pub struct CdnAuthConfiguration_ {
        pub cdn_identifier_secret_arns: Vec<crate::value::ExpString>,
        pub secrets_role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediapackagev2_OriginEndpointPolicy_CdnAuthConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaPackageV2::OriginEndpointPolicy.CdnAuthConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediapackagev2_OriginEndpointPolicy_CdnAuthConfiguration as CdnAuthConfiguration;
    impl crate::value::ToValue for CdnAuthConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CdnIdentifierSecretArns".to_string(),
                crate::value::ToValue::to_value(&self.cdn_identifier_secret_arns),
            );
            properties.insert(
                "SecretsRoleArn".to_string(),
                crate::value::ToValue::to_value(&self.secrets_role_arn),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackagev2-channel.html
pub struct Channel_ {
    pub channel_group_name: crate::value::ExpString,
    pub channel_name: crate::value::ExpString,
    pub description: Option<crate::value::ExpString>,
    pub input_switch_configuration:
        Option<super::mediapackagev2::channel::InputSwitchConfiguration_>,
    pub input_type: Option<crate::value::ExpString>,
    pub output_header_configuration:
        Option<super::mediapackagev2::channel::OutputHeaderConfiguration_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_mediapackagev2_Channel {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MediaPackageV2::Channel"
        $($field $value)*)
    };
}
pub use crate::__aws_mediapackagev2_Channel as Channel;
impl crate::template::ToResource for Channel_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaPackageV2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Channel"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ChannelGroupName".to_string(),
            crate::value::ToValue::to_value(&self.channel_group_name),
        );
        properties.insert(
            "ChannelName".to_string(),
            crate::value::ToValue::to_value(&self.channel_name),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.input_switch_configuration {
            properties.insert(
                "InputSwitchConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.input_type {
            properties.insert(
                "InputType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.output_header_configuration {
            properties.insert(
                "OutputHeaderConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackagev2-channelgroup.html
pub struct ChannelGroup_ {
    pub channel_group_name: crate::value::ExpString,
    pub description: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_mediapackagev2_ChannelGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MediaPackageV2::ChannelGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_mediapackagev2_ChannelGroup as ChannelGroup;
impl crate::template::ToResource for ChannelGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaPackageV2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ChannelGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ChannelGroupName".to_string(),
            crate::value::ToValue::to_value(&self.channel_group_name),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackagev2-channelpolicy.html
pub struct ChannelPolicy_ {
    pub channel_group_name: crate::value::ExpString,
    pub channel_name: crate::value::ExpString,
    pub policy: serde_json::Value,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_mediapackagev2_ChannelPolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MediaPackageV2::ChannelPolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_mediapackagev2_ChannelPolicy as ChannelPolicy;
impl crate::template::ToResource for ChannelPolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaPackageV2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ChannelPolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ChannelGroupName".to_string(),
            crate::value::ToValue::to_value(&self.channel_group_name),
        );
        properties.insert(
            "ChannelName".to_string(),
            crate::value::ToValue::to_value(&self.channel_name),
        );
        properties.insert(
            "Policy".to_string(),
            crate::value::ToValue::to_value(&self.policy),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackagev2-originendpoint.html
pub struct OriginEndpoint_ {
    pub channel_group_name: crate::value::ExpString,
    pub channel_name: crate::value::ExpString,
    pub container_type: crate::value::ExpString,
    pub dash_manifests:
        Option<Vec<super::mediapackagev2::originendpoint::DashManifestConfiguration_>>,
    pub description: Option<crate::value::ExpString>,
    pub force_endpoint_error_configuration:
        Option<super::mediapackagev2::originendpoint::ForceEndpointErrorConfiguration_>,
    pub hls_manifests:
        Option<Vec<super::mediapackagev2::originendpoint::HlsManifestConfiguration_>>,
    pub low_latency_hls_manifests:
        Option<Vec<super::mediapackagev2::originendpoint::LowLatencyHlsManifestConfiguration_>>,
    pub mss_manifests:
        Option<Vec<super::mediapackagev2::originendpoint::MssManifestConfiguration_>>,
    pub origin_endpoint_name: crate::value::ExpString,
    pub segment: Option<super::mediapackagev2::originendpoint::Segment_>,
    pub startover_window_seconds: Option<i32>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_mediapackagev2_OriginEndpoint {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MediaPackageV2::OriginEndpoint"
        $($field $value)*)
    };
}
pub use crate::__aws_mediapackagev2_OriginEndpoint as OriginEndpoint;
impl crate::template::ToResource for OriginEndpoint_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaPackageV2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("OriginEndpoint"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ChannelGroupName".to_string(),
            crate::value::ToValue::to_value(&self.channel_group_name),
        );
        properties.insert(
            "ChannelName".to_string(),
            crate::value::ToValue::to_value(&self.channel_name),
        );
        properties.insert(
            "ContainerType".to_string(),
            crate::value::ToValue::to_value(&self.container_type),
        );
        if let Some(ref value) = self.dash_manifests {
            properties.insert(
                "DashManifests".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.force_endpoint_error_configuration {
            properties.insert(
                "ForceEndpointErrorConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.hls_manifests {
            properties.insert(
                "HlsManifests".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.low_latency_hls_manifests {
            properties.insert(
                "LowLatencyHlsManifests".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.mss_manifests {
            properties.insert(
                "MssManifests".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "OriginEndpointName".to_string(),
            crate::value::ToValue::to_value(&self.origin_endpoint_name),
        );
        if let Some(ref value) = self.segment {
            properties.insert(
                "Segment".to_string(),
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
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackagev2-originendpointpolicy.html
pub struct OriginEndpointPolicy_ {
    pub cdn_auth_configuration:
        Option<super::mediapackagev2::originendpointpolicy::CdnAuthConfiguration_>,
    pub channel_group_name: crate::value::ExpString,
    pub channel_name: crate::value::ExpString,
    pub origin_endpoint_name: crate::value::ExpString,
    pub policy: serde_json::Value,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_mediapackagev2_OriginEndpointPolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MediaPackageV2::OriginEndpointPolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_mediapackagev2_OriginEndpointPolicy as OriginEndpointPolicy;
impl crate::template::ToResource for OriginEndpointPolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaPackageV2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("OriginEndpointPolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.cdn_auth_configuration {
            properties.insert(
                "CdnAuthConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ChannelGroupName".to_string(),
            crate::value::ToValue::to_value(&self.channel_group_name),
        );
        properties.insert(
            "ChannelName".to_string(),
            crate::value::ToValue::to_value(&self.channel_name),
        );
        properties.insert(
            "OriginEndpointName".to_string(),
            crate::value::ToValue::to_value(&self.origin_endpoint_name),
        );
        properties.insert(
            "Policy".to_string(),
            crate::value::ToValue::to_value(&self.policy),
        );
        properties
    }
}
