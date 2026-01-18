pub mod channel {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-channel-dashplaylistsettings.html
    pub struct DashPlaylistSettings_ {
        pub manifest_window_seconds: Option<f64>,
        pub min_buffer_time_seconds: Option<f64>,
        pub min_update_period_seconds: Option<f64>,
        pub suggested_presentation_delay_seconds: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediatailor_Channel_DashPlaylistSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaTailor::Channel.DashPlaylistSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediatailor_Channel_DashPlaylistSettings as DashPlaylistSettings;
    impl crate::value::ToValue for DashPlaylistSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
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
            if let Some(ref value) = self.suggested_presentation_delay_seconds {
                properties.insert(
                    "SuggestedPresentationDelaySeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-channel-hlsplaylistsettings.html
    pub struct HlsPlaylistSettings_ {
        pub ad_markup_type: Option<Vec<crate::value::ExpString>>,
        pub manifest_window_seconds: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediatailor_Channel_HlsPlaylistSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaTailor::Channel.HlsPlaylistSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediatailor_Channel_HlsPlaylistSettings as HlsPlaylistSettings;
    impl crate::value::ToValue for HlsPlaylistSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ad_markup_type {
                properties.insert(
                    "AdMarkupType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.manifest_window_seconds {
                properties.insert(
                    "ManifestWindowSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-channel-logconfigurationforchannel.html
    pub struct LogConfigurationForChannel_ {
        pub log_types: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediatailor_Channel_LogConfigurationForChannel {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaTailor::Channel.LogConfigurationForChannel"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediatailor_Channel_LogConfigurationForChannel as LogConfigurationForChannel;
    impl crate::value::ToValue for LogConfigurationForChannel_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.log_types {
                properties.insert(
                    "LogTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-channel-requestoutputitem.html
    pub struct RequestOutputItem_ {
        pub dash_playlist_settings: Option<Box<DashPlaylistSettings_>>,
        pub hls_playlist_settings: Option<Box<HlsPlaylistSettings_>>,
        pub manifest_name: crate::value::ExpString,
        pub source_group: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediatailor_Channel_RequestOutputItem {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaTailor::Channel.RequestOutputItem"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediatailor_Channel_RequestOutputItem as RequestOutputItem;
    impl crate::value::ToValue for RequestOutputItem_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dash_playlist_settings {
                properties.insert(
                    "DashPlaylistSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hls_playlist_settings {
                properties.insert(
                    "HlsPlaylistSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ManifestName".to_string(),
                crate::value::ToValue::to_value(&self.manifest_name),
            );
            properties.insert(
                "SourceGroup".to_string(),
                crate::value::ToValue::to_value(&self.source_group),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-channel-slatesource.html
    pub struct SlateSource_ {
        pub source_location_name: Option<crate::value::ExpString>,
        pub vod_source_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediatailor_Channel_SlateSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaTailor::Channel.SlateSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediatailor_Channel_SlateSource as SlateSource;
    impl crate::value::ToValue for SlateSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.source_location_name {
                properties.insert(
                    "SourceLocationName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vod_source_name {
                properties.insert(
                    "VodSourceName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-channel-timeshiftconfiguration.html
    pub struct TimeShiftConfiguration_ {
        pub max_time_delay_seconds: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediatailor_Channel_TimeShiftConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaTailor::Channel.TimeShiftConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediatailor_Channel_TimeShiftConfiguration as TimeShiftConfiguration;
    impl crate::value::ToValue for TimeShiftConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MaxTimeDelaySeconds".to_string(),
                crate::value::ToValue::to_value(&self.max_time_delay_seconds),
            );
            properties.into()
        }
    }
}
pub mod livesource {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-livesource-httppackageconfiguration.html
    pub struct HttpPackageConfiguration_ {
        pub path: crate::value::ExpString,
        pub source_group: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediatailor_LiveSource_HttpPackageConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaTailor::LiveSource.HttpPackageConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediatailor_LiveSource_HttpPackageConfiguration as HttpPackageConfiguration;
    impl crate::value::ToValue for HttpPackageConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Path".to_string(),
                crate::value::ToValue::to_value(&self.path),
            );
            properties.insert(
                "SourceGroup".to_string(),
                crate::value::ToValue::to_value(&self.source_group),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
}
pub mod playbackconfiguration {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-playbackconfiguration-adconditioningconfiguration.html
    pub struct AdConditioningConfiguration_ {
        pub streaming_media_file_conditioning: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediatailor_PlaybackConfiguration_AdConditioningConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaTailor::PlaybackConfiguration.AdConditioningConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediatailor_PlaybackConfiguration_AdConditioningConfiguration as AdConditioningConfiguration;
    impl crate::value::ToValue for AdConditioningConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "StreamingMediaFileConditioning".to_string(),
                crate::value::ToValue::to_value(&self.streaming_media_file_conditioning),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-playbackconfiguration-admarkerpassthrough.html
    pub struct AdMarkerPassthrough_ {
        pub enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediatailor_PlaybackConfiguration_AdMarkerPassthrough {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaTailor::PlaybackConfiguration.AdMarkerPassthrough"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediatailor_PlaybackConfiguration_AdMarkerPassthrough as AdMarkerPassthrough;
    impl crate::value::ToValue for AdMarkerPassthrough_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-playbackconfiguration-adsinteractionlog.html
    pub struct AdsInteractionLog_ {
        pub exclude_event_types: Option<Vec<crate::value::ExpString>>,
        pub publish_opt_in_event_types: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediatailor_PlaybackConfiguration_AdsInteractionLog {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaTailor::PlaybackConfiguration.AdsInteractionLog"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediatailor_PlaybackConfiguration_AdsInteractionLog as AdsInteractionLog;
    impl crate::value::ToValue for AdsInteractionLog_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.exclude_event_types {
                properties.insert(
                    "ExcludeEventTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.publish_opt_in_event_types {
                properties.insert(
                    "PublishOptInEventTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-playbackconfiguration-availsuppression.html
    pub struct AvailSuppression_ {
        pub fill_policy: Option<crate::value::ExpString>,
        pub mode: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediatailor_PlaybackConfiguration_AvailSuppression {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaTailor::PlaybackConfiguration.AvailSuppression"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediatailor_PlaybackConfiguration_AvailSuppression as AvailSuppression;
    impl crate::value::ToValue for AvailSuppression_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.fill_policy {
                properties.insert(
                    "FillPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mode {
                properties.insert("Mode".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-playbackconfiguration-bumper.html
    pub struct Bumper_ {
        pub end_url: Option<crate::value::ExpString>,
        pub start_url: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediatailor_PlaybackConfiguration_Bumper {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaTailor::PlaybackConfiguration.Bumper"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediatailor_PlaybackConfiguration_Bumper as Bumper;
    impl crate::value::ToValue for Bumper_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.end_url {
                properties.insert("EndUrl".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.start_url {
                properties.insert(
                    "StartUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-playbackconfiguration-cdnconfiguration.html
    pub struct CdnConfiguration_ {
        pub ad_segment_url_prefix: Option<crate::value::ExpString>,
        pub content_segment_url_prefix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediatailor_PlaybackConfiguration_CdnConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaTailor::PlaybackConfiguration.CdnConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediatailor_PlaybackConfiguration_CdnConfiguration as CdnConfiguration;
    impl crate::value::ToValue for CdnConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ad_segment_url_prefix {
                properties.insert(
                    "AdSegmentUrlPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.content_segment_url_prefix {
                properties.insert(
                    "ContentSegmentUrlPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-playbackconfiguration-dashconfiguration.html
    pub struct DashConfiguration_ {
        pub manifest_endpoint_prefix: Option<crate::value::ExpString>,
        pub mpd_location: Option<crate::value::ExpString>,
        pub origin_manifest_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediatailor_PlaybackConfiguration_DashConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaTailor::PlaybackConfiguration.DashConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediatailor_PlaybackConfiguration_DashConfiguration as DashConfiguration;
    impl crate::value::ToValue for DashConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.manifest_endpoint_prefix {
                properties.insert(
                    "ManifestEndpointPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mpd_location {
                properties.insert(
                    "MpdLocation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.origin_manifest_type {
                properties.insert(
                    "OriginManifestType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-playbackconfiguration-hlsconfiguration.html
    pub struct HlsConfiguration_ {
        pub manifest_endpoint_prefix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediatailor_PlaybackConfiguration_HlsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaTailor::PlaybackConfiguration.HlsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediatailor_PlaybackConfiguration_HlsConfiguration as HlsConfiguration;
    impl crate::value::ToValue for HlsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.manifest_endpoint_prefix {
                properties.insert(
                    "ManifestEndpointPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-playbackconfiguration-liveprerollconfiguration.html
    pub struct LivePreRollConfiguration_ {
        pub ad_decision_server_url: Option<crate::value::ExpString>,
        pub max_duration_seconds: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediatailor_PlaybackConfiguration_LivePreRollConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaTailor::PlaybackConfiguration.LivePreRollConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediatailor_PlaybackConfiguration_LivePreRollConfiguration as LivePreRollConfiguration;
    impl crate::value::ToValue for LivePreRollConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ad_decision_server_url {
                properties.insert(
                    "AdDecisionServerUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_duration_seconds {
                properties.insert(
                    "MaxDurationSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-playbackconfiguration-logconfiguration.html
    pub struct LogConfiguration_ {
        pub ads_interaction_log: Option<Box<AdsInteractionLog_>>,
        pub enabled_logging_strategies: Option<Vec<crate::value::ExpString>>,
        pub manifest_service_interaction_log: Option<Box<ManifestServiceInteractionLog_>>,
        pub percent_enabled: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediatailor_PlaybackConfiguration_LogConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaTailor::PlaybackConfiguration.LogConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediatailor_PlaybackConfiguration_LogConfiguration as LogConfiguration;
    impl crate::value::ToValue for LogConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ads_interaction_log {
                properties.insert(
                    "AdsInteractionLog".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enabled_logging_strategies {
                properties.insert(
                    "EnabledLoggingStrategies".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.manifest_service_interaction_log {
                properties.insert(
                    "ManifestServiceInteractionLog".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "PercentEnabled".to_string(),
                crate::value::ToValue::to_value(&self.percent_enabled),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-playbackconfiguration-manifestprocessingrules.html
    pub struct ManifestProcessingRules_ {
        pub ad_marker_passthrough: Option<Box<AdMarkerPassthrough_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediatailor_PlaybackConfiguration_ManifestProcessingRules {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaTailor::PlaybackConfiguration.ManifestProcessingRules"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediatailor_PlaybackConfiguration_ManifestProcessingRules as ManifestProcessingRules;
    impl crate::value::ToValue for ManifestProcessingRules_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ad_marker_passthrough {
                properties.insert(
                    "AdMarkerPassthrough".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-playbackconfiguration-manifestserviceinteractionlog.html
    pub struct ManifestServiceInteractionLog_ {
        pub exclude_event_types: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediatailor_PlaybackConfiguration_ManifestServiceInteractionLog {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaTailor::PlaybackConfiguration.ManifestServiceInteractionLog"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediatailor_PlaybackConfiguration_ManifestServiceInteractionLog as ManifestServiceInteractionLog;
    impl crate::value::ToValue for ManifestServiceInteractionLog_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.exclude_event_types {
                properties.insert(
                    "ExcludeEventTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod sourcelocation {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-sourcelocation-accessconfiguration.html
    pub struct AccessConfiguration_ {
        pub access_type: Option<crate::value::ExpString>,
        pub secrets_manager_access_token_configuration:
            Option<Box<SecretsManagerAccessTokenConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediatailor_SourceLocation_AccessConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaTailor::SourceLocation.AccessConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediatailor_SourceLocation_AccessConfiguration as AccessConfiguration;
    impl crate::value::ToValue for AccessConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.access_type {
                properties.insert(
                    "AccessType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secrets_manager_access_token_configuration {
                properties.insert(
                    "SecretsManagerAccessTokenConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-sourcelocation-defaultsegmentdeliveryconfiguration.html
    pub struct DefaultSegmentDeliveryConfiguration_ {
        pub base_url: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediatailor_SourceLocation_DefaultSegmentDeliveryConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaTailor::SourceLocation.DefaultSegmentDeliveryConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediatailor_SourceLocation_DefaultSegmentDeliveryConfiguration as DefaultSegmentDeliveryConfiguration;
    impl crate::value::ToValue for DefaultSegmentDeliveryConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.base_url {
                properties.insert(
                    "BaseUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-sourcelocation-httpconfiguration.html
    pub struct HttpConfiguration_ {
        pub base_url: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediatailor_SourceLocation_HttpConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaTailor::SourceLocation.HttpConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediatailor_SourceLocation_HttpConfiguration as HttpConfiguration;
    impl crate::value::ToValue for HttpConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BaseUrl".to_string(),
                crate::value::ToValue::to_value(&self.base_url),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-sourcelocation-secretsmanageraccesstokenconfiguration.html
    pub struct SecretsManagerAccessTokenConfiguration_ {
        pub header_name: Option<crate::value::ExpString>,
        pub secret_arn: Option<crate::value::ExpString>,
        pub secret_string_key: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediatailor_SourceLocation_SecretsManagerAccessTokenConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaTailor::SourceLocation.SecretsManagerAccessTokenConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediatailor_SourceLocation_SecretsManagerAccessTokenConfiguration as SecretsManagerAccessTokenConfiguration;
    impl crate::value::ToValue for SecretsManagerAccessTokenConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.header_name {
                properties.insert(
                    "HeaderName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secret_arn {
                properties.insert(
                    "SecretArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secret_string_key {
                properties.insert(
                    "SecretStringKey".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-sourcelocation-segmentdeliveryconfiguration.html
    pub struct SegmentDeliveryConfiguration_ {
        pub base_url: Option<crate::value::ExpString>,
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediatailor_SourceLocation_SegmentDeliveryConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaTailor::SourceLocation.SegmentDeliveryConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediatailor_SourceLocation_SegmentDeliveryConfiguration as SegmentDeliveryConfiguration;
    impl crate::value::ToValue for SegmentDeliveryConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.base_url {
                properties.insert(
                    "BaseUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod vodsource {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-vodsource-httppackageconfiguration.html
    pub struct HttpPackageConfiguration_ {
        pub path: crate::value::ExpString,
        pub source_group: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediatailor_VodSource_HttpPackageConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaTailor::VodSource.HttpPackageConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediatailor_VodSource_HttpPackageConfiguration as HttpPackageConfiguration;
    impl crate::value::ToValue for HttpPackageConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Path".to_string(),
                crate::value::ToValue::to_value(&self.path),
            );
            properties.insert(
                "SourceGroup".to_string(),
                crate::value::ToValue::to_value(&self.source_group),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-channel.html
pub struct Channel_ {
    pub audiences: Option<Vec<crate::value::ExpString>>,
    pub channel_name: crate::value::ExpString,
    pub filler_slate: Option<super::mediatailor::channel::SlateSource_>,
    pub log_configuration: Option<super::mediatailor::channel::LogConfigurationForChannel_>,
    pub outputs: Vec<super::mediatailor::channel::RequestOutputItem_>,
    pub playback_mode: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub tier: Option<crate::value::ExpString>,
    pub time_shift_configuration: Option<super::mediatailor::channel::TimeShiftConfiguration_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_mediatailor_Channel {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MediaTailor::Channel"
        $($field $value)*)
    };
}
pub use crate::__aws_mediatailor_Channel as Channel;
impl crate::template::ToResource for Channel_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaTailor"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Channel"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.audiences {
            properties.insert(
                "Audiences".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ChannelName".to_string(),
            crate::value::ToValue::to_value(&self.channel_name),
        );
        if let Some(ref value) = self.filler_slate {
            properties.insert(
                "FillerSlate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.log_configuration {
            properties.insert(
                "LogConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Outputs".to_string(),
            crate::value::ToValue::to_value(&self.outputs),
        );
        properties.insert(
            "PlaybackMode".to_string(),
            crate::value::ToValue::to_value(&self.playback_mode),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tier {
            properties.insert("Tier".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.time_shift_configuration {
            properties.insert(
                "TimeShiftConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-channelpolicy.html
pub struct ChannelPolicy_ {
    pub channel_name: crate::value::ExpString,
    pub policy: serde_json::Value,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_mediatailor_ChannelPolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MediaTailor::ChannelPolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_mediatailor_ChannelPolicy as ChannelPolicy;
impl crate::template::ToResource for ChannelPolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaTailor"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ChannelPolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-livesource.html
pub struct LiveSource_ {
    pub http_package_configurations: Vec<super::mediatailor::livesource::HttpPackageConfiguration_>,
    pub live_source_name: crate::value::ExpString,
    pub source_location_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_mediatailor_LiveSource {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MediaTailor::LiveSource"
        $($field $value)*)
    };
}
pub use crate::__aws_mediatailor_LiveSource as LiveSource;
impl crate::template::ToResource for LiveSource_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaTailor"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LiveSource"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "HttpPackageConfigurations".to_string(),
            crate::value::ToValue::to_value(&self.http_package_configurations),
        );
        properties.insert(
            "LiveSourceName".to_string(),
            crate::value::ToValue::to_value(&self.live_source_name),
        );
        properties.insert(
            "SourceLocationName".to_string(),
            crate::value::ToValue::to_value(&self.source_location_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-playbackconfiguration.html
pub struct PlaybackConfiguration_ {
    pub ad_conditioning_configuration:
        Option<super::mediatailor::playbackconfiguration::AdConditioningConfiguration_>,
    pub ad_decision_server_url: crate::value::ExpString,
    pub avail_suppression: Option<super::mediatailor::playbackconfiguration::AvailSuppression_>,
    pub bumper: Option<super::mediatailor::playbackconfiguration::Bumper_>,
    pub cdn_configuration: Option<super::mediatailor::playbackconfiguration::CdnConfiguration_>,
    pub configuration_aliases: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    pub dash_configuration: Option<super::mediatailor::playbackconfiguration::DashConfiguration_>,
    pub hls_configuration: Option<super::mediatailor::playbackconfiguration::HlsConfiguration_>,
    pub insertion_mode: Option<crate::value::ExpString>,
    pub live_pre_roll_configuration:
        Option<super::mediatailor::playbackconfiguration::LivePreRollConfiguration_>,
    pub log_configuration: Option<super::mediatailor::playbackconfiguration::LogConfiguration_>,
    pub manifest_processing_rules:
        Option<super::mediatailor::playbackconfiguration::ManifestProcessingRules_>,
    pub name: crate::value::ExpString,
    pub personalization_threshold_seconds: Option<i64>,
    pub slate_ad_url: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub transcode_profile_name: Option<crate::value::ExpString>,
    pub video_content_source_url: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_mediatailor_PlaybackConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MediaTailor::PlaybackConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_mediatailor_PlaybackConfiguration as PlaybackConfiguration;
impl crate::template::ToResource for PlaybackConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaTailor"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PlaybackConfiguration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.ad_conditioning_configuration {
            properties.insert(
                "AdConditioningConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "AdDecisionServerUrl".to_string(),
            crate::value::ToValue::to_value(&self.ad_decision_server_url),
        );
        if let Some(ref value) = self.avail_suppression {
            properties.insert(
                "AvailSuppression".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.bumper {
            properties.insert("Bumper".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.cdn_configuration {
            properties.insert(
                "CdnConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.configuration_aliases {
            properties.insert(
                "ConfigurationAliases".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.dash_configuration {
            properties.insert(
                "DashConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.hls_configuration {
            properties.insert(
                "HlsConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.insertion_mode {
            properties.insert(
                "InsertionMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.live_pre_roll_configuration {
            properties.insert(
                "LivePreRollConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.log_configuration {
            properties.insert(
                "LogConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.manifest_processing_rules {
            properties.insert(
                "ManifestProcessingRules".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.personalization_threshold_seconds {
            properties.insert(
                "PersonalizationThresholdSeconds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.slate_ad_url {
            properties.insert(
                "SlateAdUrl".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.transcode_profile_name {
            properties.insert(
                "TranscodeProfileName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "VideoContentSourceUrl".to_string(),
            crate::value::ToValue::to_value(&self.video_content_source_url),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-sourcelocation.html
pub struct SourceLocation_ {
    pub access_configuration: Option<super::mediatailor::sourcelocation::AccessConfiguration_>,
    pub default_segment_delivery_configuration:
        Option<super::mediatailor::sourcelocation::DefaultSegmentDeliveryConfiguration_>,
    pub http_configuration: super::mediatailor::sourcelocation::HttpConfiguration_,
    pub segment_delivery_configurations:
        Option<Vec<super::mediatailor::sourcelocation::SegmentDeliveryConfiguration_>>,
    pub source_location_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_mediatailor_SourceLocation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MediaTailor::SourceLocation"
        $($field $value)*)
    };
}
pub use crate::__aws_mediatailor_SourceLocation as SourceLocation;
impl crate::template::ToResource for SourceLocation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaTailor"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SourceLocation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.access_configuration {
            properties.insert(
                "AccessConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.default_segment_delivery_configuration {
            properties.insert(
                "DefaultSegmentDeliveryConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "HttpConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.http_configuration),
        );
        if let Some(ref value) = self.segment_delivery_configurations {
            properties.insert(
                "SegmentDeliveryConfigurations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SourceLocationName".to_string(),
            crate::value::ToValue::to_value(&self.source_location_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-vodsource.html
pub struct VodSource_ {
    pub http_package_configurations: Vec<super::mediatailor::vodsource::HttpPackageConfiguration_>,
    pub source_location_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vod_source_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_mediatailor_VodSource {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MediaTailor::VodSource"
        $($field $value)*)
    };
}
pub use crate::__aws_mediatailor_VodSource as VodSource;
impl crate::template::ToResource for VodSource_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaTailor"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("VodSource"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "HttpPackageConfigurations".to_string(),
            crate::value::ToValue::to_value(&self.http_package_configurations),
        );
        properties.insert(
            "SourceLocationName".to_string(),
            crate::value::ToValue::to_value(&self.source_location_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "VodSourceName".to_string(),
            crate::value::ToValue::to_value(&self.vod_source_name),
        );
        properties
    }
}
