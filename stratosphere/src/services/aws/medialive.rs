pub mod channel {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-aacsettings.html
    pub struct AacSettings_ {
        pub bitrate: Option<f64>,
        pub coding_mode: Option<crate::value::ExpString>,
        pub input_type: Option<crate::value::ExpString>,
        pub profile: Option<crate::value::ExpString>,
        pub rate_control_mode: Option<crate::value::ExpString>,
        pub raw_format: Option<crate::value::ExpString>,
        pub sample_rate: Option<f64>,
        pub spec: Option<crate::value::ExpString>,
        pub vbr_quality: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_AacSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.AacSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_AacSettings as AacSettings;
    impl crate::value::ToValue for AacSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bitrate {
                properties.insert(
                    "Bitrate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.coding_mode {
                properties.insert(
                    "CodingMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_type {
                properties.insert(
                    "InputType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.profile {
                properties.insert(
                    "Profile".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rate_control_mode {
                properties.insert(
                    "RateControlMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.raw_format {
                properties.insert(
                    "RawFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sample_rate {
                properties.insert(
                    "SampleRate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.spec {
                properties.insert("Spec".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.vbr_quality {
                properties.insert(
                    "VbrQuality".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-ac3settings.html
    pub struct Ac3Settings_ {
        pub attenuation_control: Option<crate::value::ExpString>,
        pub bitrate: Option<f64>,
        pub bitstream_mode: Option<crate::value::ExpString>,
        pub coding_mode: Option<crate::value::ExpString>,
        pub dialnorm: Option<i64>,
        pub drc_profile: Option<crate::value::ExpString>,
        pub lfe_filter: Option<crate::value::ExpString>,
        pub metadata_control: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_Ac3Settings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.Ac3Settings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_Ac3Settings as Ac3Settings;
    impl crate::value::ToValue for Ac3Settings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attenuation_control {
                properties.insert(
                    "AttenuationControl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bitrate {
                properties.insert(
                    "Bitrate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bitstream_mode {
                properties.insert(
                    "BitstreamMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.coding_mode {
                properties.insert(
                    "CodingMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dialnorm {
                properties.insert(
                    "Dialnorm".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.drc_profile {
                properties.insert(
                    "DrcProfile".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lfe_filter {
                properties.insert(
                    "LfeFilter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metadata_control {
                properties.insert(
                    "MetadataControl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-additionaldestinations.html
    pub struct AdditionalDestinations_ {
        pub destination: Option<Box<OutputLocationRef_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_AdditionalDestinations {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.AdditionalDestinations"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_AdditionalDestinations as AdditionalDestinations;
    impl crate::value::ToValue for AdditionalDestinations_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.destination {
                properties.insert(
                    "Destination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-ancillarysourcesettings.html
    pub struct AncillarySourceSettings_ {
        pub source_ancillary_channel_number: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_AncillarySourceSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.AncillarySourceSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_AncillarySourceSettings as AncillarySourceSettings;
    impl crate::value::ToValue for AncillarySourceSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.source_ancillary_channel_number {
                properties.insert(
                    "SourceAncillaryChannelNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-anywheresettings.html
    pub struct AnywhereSettings_ {
        pub channel_placement_group_id: Option<crate::value::ExpString>,
        pub cluster_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_AnywhereSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.AnywhereSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_AnywhereSettings as AnywhereSettings;
    impl crate::value::ToValue for AnywhereSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.channel_placement_group_id {
                properties.insert(
                    "ChannelPlacementGroupId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cluster_id {
                properties.insert(
                    "ClusterId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-archivecdnsettings.html
    pub struct ArchiveCdnSettings_ {
        pub archive_s3_settings: Option<Box<ArchiveS3Settings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_ArchiveCdnSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.ArchiveCdnSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_ArchiveCdnSettings as ArchiveCdnSettings;
    impl crate::value::ToValue for ArchiveCdnSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.archive_s3_settings {
                properties.insert(
                    "ArchiveS3Settings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-archivecontainersettings.html
    pub struct ArchiveContainerSettings_ {
        pub m2ts_settings: Option<Box<M2tsSettings_>>,
        pub raw_settings: Option<Box<RawSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_ArchiveContainerSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.ArchiveContainerSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_ArchiveContainerSettings as ArchiveContainerSettings;
    impl crate::value::ToValue for ArchiveContainerSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.m2ts_settings {
                properties.insert(
                    "M2tsSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.raw_settings {
                properties.insert(
                    "RawSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-archivegroupsettings.html
    pub struct ArchiveGroupSettings_ {
        pub archive_cdn_settings: Option<Box<ArchiveCdnSettings_>>,
        pub destination: Option<Box<OutputLocationRef_>>,
        pub rollover_interval: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_ArchiveGroupSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.ArchiveGroupSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_ArchiveGroupSettings as ArchiveGroupSettings;
    impl crate::value::ToValue for ArchiveGroupSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.archive_cdn_settings {
                properties.insert(
                    "ArchiveCdnSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.destination {
                properties.insert(
                    "Destination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rollover_interval {
                properties.insert(
                    "RolloverInterval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-archiveoutputsettings.html
    pub struct ArchiveOutputSettings_ {
        pub container_settings: Option<Box<ArchiveContainerSettings_>>,
        pub extension: Option<crate::value::ExpString>,
        pub name_modifier: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_ArchiveOutputSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.ArchiveOutputSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_ArchiveOutputSettings as ArchiveOutputSettings;
    impl crate::value::ToValue for ArchiveOutputSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.container_settings {
                properties.insert(
                    "ContainerSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.extension {
                properties.insert(
                    "Extension".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name_modifier {
                properties.insert(
                    "NameModifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-archives3settings.html
    pub struct ArchiveS3Settings_ {
        pub canned_acl: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_ArchiveS3Settings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.ArchiveS3Settings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_ArchiveS3Settings as ArchiveS3Settings;
    impl crate::value::ToValue for ArchiveS3Settings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.canned_acl {
                properties.insert(
                    "CannedAcl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-aribdestinationsettings.html
    pub struct AribDestinationSettings_ {}
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_AribDestinationSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.AribDestinationSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_AribDestinationSettings as AribDestinationSettings;
    impl crate::value::ToValue for AribDestinationSettings_ {
        fn to_value(&self) -> serde_json::Value {
            serde_json::Value::Object(serde_json::Map::new())
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-aribsourcesettings.html
    pub struct AribSourceSettings_ {}
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_AribSourceSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.AribSourceSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_AribSourceSettings as AribSourceSettings;
    impl crate::value::ToValue for AribSourceSettings_ {
        fn to_value(&self) -> serde_json::Value {
            serde_json::Value::Object(serde_json::Map::new())
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiochannelmapping.html
    pub struct AudioChannelMapping_ {
        pub input_channel_levels: Option<Vec<InputChannelLevel_>>,
        pub output_channel: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_AudioChannelMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.AudioChannelMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_AudioChannelMapping as AudioChannelMapping;
    impl crate::value::ToValue for AudioChannelMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.input_channel_levels {
                properties.insert(
                    "InputChannelLevels".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.output_channel {
                properties.insert(
                    "OutputChannel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiocodecsettings.html
    pub struct AudioCodecSettings_ {
        pub aac_settings: Option<Box<AacSettings_>>,
        pub ac3_settings: Option<Box<Ac3Settings_>>,
        pub eac3_atmos_settings: Option<Box<Eac3AtmosSettings_>>,
        pub eac3_settings: Option<Box<Eac3Settings_>>,
        pub mp2_settings: Option<Box<Mp2Settings_>>,
        pub pass_through_settings: Option<Box<PassThroughSettings_>>,
        pub wav_settings: Option<Box<WavSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_AudioCodecSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.AudioCodecSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_AudioCodecSettings as AudioCodecSettings;
    impl crate::value::ToValue for AudioCodecSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.aac_settings {
                properties.insert(
                    "AacSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ac3_settings {
                properties.insert(
                    "Ac3Settings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.eac3_atmos_settings {
                properties.insert(
                    "Eac3AtmosSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.eac3_settings {
                properties.insert(
                    "Eac3Settings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mp2_settings {
                properties.insert(
                    "Mp2Settings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pass_through_settings {
                properties.insert(
                    "PassThroughSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.wav_settings {
                properties.insert(
                    "WavSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiodescription.html
    pub struct AudioDescription_ {
        pub audio_dash_roles: Option<Vec<crate::value::ExpString>>,
        pub audio_normalization_settings: Option<Box<AudioNormalizationSettings_>>,
        pub audio_selector_name: Option<crate::value::ExpString>,
        pub audio_type: Option<crate::value::ExpString>,
        pub audio_type_control: Option<crate::value::ExpString>,
        pub audio_watermarking_settings: Option<Box<AudioWatermarkSettings_>>,
        pub codec_settings: Option<Box<AudioCodecSettings_>>,
        pub dvb_dash_accessibility: Option<crate::value::ExpString>,
        pub language_code: Option<crate::value::ExpString>,
        pub language_code_control: Option<crate::value::ExpString>,
        pub name: Option<crate::value::ExpString>,
        pub remix_settings: Option<Box<RemixSettings_>>,
        pub stream_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_AudioDescription {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.AudioDescription"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_AudioDescription as AudioDescription;
    impl crate::value::ToValue for AudioDescription_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.audio_dash_roles {
                properties.insert(
                    "AudioDashRoles".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.audio_normalization_settings {
                properties.insert(
                    "AudioNormalizationSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.audio_selector_name {
                properties.insert(
                    "AudioSelectorName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.audio_type {
                properties.insert(
                    "AudioType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.audio_type_control {
                properties.insert(
                    "AudioTypeControl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.audio_watermarking_settings {
                properties.insert(
                    "AudioWatermarkingSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.codec_settings {
                properties.insert(
                    "CodecSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dvb_dash_accessibility {
                properties.insert(
                    "DvbDashAccessibility".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.language_code {
                properties.insert(
                    "LanguageCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.language_code_control {
                properties.insert(
                    "LanguageCodeControl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.remix_settings {
                properties.insert(
                    "RemixSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stream_name {
                properties.insert(
                    "StreamName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiodolbyedecode.html
    pub struct AudioDolbyEDecode_ {
        pub program_selection: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_AudioDolbyEDecode {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.AudioDolbyEDecode"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_AudioDolbyEDecode as AudioDolbyEDecode;
    impl crate::value::ToValue for AudioDolbyEDecode_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.program_selection {
                properties.insert(
                    "ProgramSelection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiohlsrenditionselection.html
    pub struct AudioHlsRenditionSelection_ {
        pub group_id: Option<crate::value::ExpString>,
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_AudioHlsRenditionSelection {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.AudioHlsRenditionSelection"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_AudioHlsRenditionSelection as AudioHlsRenditionSelection;
    impl crate::value::ToValue for AudioHlsRenditionSelection_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.group_id {
                properties.insert(
                    "GroupId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiolanguageselection.html
    pub struct AudioLanguageSelection_ {
        pub language_code: Option<crate::value::ExpString>,
        pub language_selection_policy: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_AudioLanguageSelection {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.AudioLanguageSelection"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_AudioLanguageSelection as AudioLanguageSelection;
    impl crate::value::ToValue for AudioLanguageSelection_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.language_code {
                properties.insert(
                    "LanguageCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.language_selection_policy {
                properties.insert(
                    "LanguageSelectionPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audionormalizationsettings.html
    pub struct AudioNormalizationSettings_ {
        pub algorithm: Option<crate::value::ExpString>,
        pub algorithm_control: Option<crate::value::ExpString>,
        pub target_lkfs: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_AudioNormalizationSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.AudioNormalizationSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_AudioNormalizationSettings as AudioNormalizationSettings;
    impl crate::value::ToValue for AudioNormalizationSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.algorithm {
                properties.insert(
                    "Algorithm".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.algorithm_control {
                properties.insert(
                    "AlgorithmControl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_lkfs {
                properties.insert(
                    "TargetLkfs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audioonlyhlssettings.html
    pub struct AudioOnlyHlsSettings_ {
        pub audio_group_id: Option<crate::value::ExpString>,
        pub audio_only_image: Option<Box<InputLocation_>>,
        pub audio_track_type: Option<crate::value::ExpString>,
        pub segment_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_AudioOnlyHlsSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.AudioOnlyHlsSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_AudioOnlyHlsSettings as AudioOnlyHlsSettings;
    impl crate::value::ToValue for AudioOnlyHlsSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.audio_group_id {
                properties.insert(
                    "AudioGroupId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.audio_only_image {
                properties.insert(
                    "AudioOnlyImage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.audio_track_type {
                properties.insert(
                    "AudioTrackType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.segment_type {
                properties.insert(
                    "SegmentType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiopidselection.html
    pub struct AudioPidSelection_ {
        pub pid: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_AudioPidSelection {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.AudioPidSelection"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_AudioPidSelection as AudioPidSelection;
    impl crate::value::ToValue for AudioPidSelection_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.pid {
                properties.insert("Pid".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audioselector.html
    pub struct AudioSelector_ {
        pub name: Option<crate::value::ExpString>,
        pub selector_settings: Option<Box<AudioSelectorSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_AudioSelector {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.AudioSelector"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_AudioSelector as AudioSelector;
    impl crate::value::ToValue for AudioSelector_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.selector_settings {
                properties.insert(
                    "SelectorSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audioselectorsettings.html
    pub struct AudioSelectorSettings_ {
        pub audio_hls_rendition_selection: Option<Box<AudioHlsRenditionSelection_>>,
        pub audio_language_selection: Option<Box<AudioLanguageSelection_>>,
        pub audio_pid_selection: Option<Box<AudioPidSelection_>>,
        pub audio_track_selection: Option<Box<AudioTrackSelection_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_AudioSelectorSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.AudioSelectorSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_AudioSelectorSettings as AudioSelectorSettings;
    impl crate::value::ToValue for AudioSelectorSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.audio_hls_rendition_selection {
                properties.insert(
                    "AudioHlsRenditionSelection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.audio_language_selection {
                properties.insert(
                    "AudioLanguageSelection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.audio_pid_selection {
                properties.insert(
                    "AudioPidSelection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.audio_track_selection {
                properties.insert(
                    "AudioTrackSelection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiosilencefailoversettings.html
    pub struct AudioSilenceFailoverSettings_ {
        pub audio_selector_name: Option<crate::value::ExpString>,
        pub audio_silence_threshold_msec: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_AudioSilenceFailoverSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.AudioSilenceFailoverSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_AudioSilenceFailoverSettings as AudioSilenceFailoverSettings;
    impl crate::value::ToValue for AudioSilenceFailoverSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.audio_selector_name {
                properties.insert(
                    "AudioSelectorName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.audio_silence_threshold_msec {
                properties.insert(
                    "AudioSilenceThresholdMsec".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiotrack.html
    pub struct AudioTrack_ {
        pub track: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_AudioTrack {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.AudioTrack"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_AudioTrack as AudioTrack;
    impl crate::value::ToValue for AudioTrack_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.track {
                properties.insert("Track".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiotrackselection.html
    pub struct AudioTrackSelection_ {
        pub dolby_e_decode: Option<Box<AudioDolbyEDecode_>>,
        pub tracks: Option<Vec<AudioTrack_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_AudioTrackSelection {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.AudioTrackSelection"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_AudioTrackSelection as AudioTrackSelection;
    impl crate::value::ToValue for AudioTrackSelection_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dolby_e_decode {
                properties.insert(
                    "DolbyEDecode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tracks {
                properties.insert("Tracks".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiowatermarksettings.html
    pub struct AudioWatermarkSettings_ {
        pub nielsen_watermarks_settings: Option<Box<NielsenWatermarksSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_AudioWatermarkSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.AudioWatermarkSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_AudioWatermarkSettings as AudioWatermarkSettings;
    impl crate::value::ToValue for AudioWatermarkSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.nielsen_watermarks_settings {
                properties.insert(
                    "NielsenWatermarksSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-automaticinputfailoversettings.html
    pub struct AutomaticInputFailoverSettings_ {
        pub error_clear_time_msec: Option<i64>,
        pub failover_conditions: Option<Vec<FailoverCondition_>>,
        pub input_preference: Option<crate::value::ExpString>,
        pub secondary_input_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_AutomaticInputFailoverSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.AutomaticInputFailoverSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_AutomaticInputFailoverSettings as AutomaticInputFailoverSettings;
    impl crate::value::ToValue for AutomaticInputFailoverSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.error_clear_time_msec {
                properties.insert(
                    "ErrorClearTimeMsec".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.failover_conditions {
                properties.insert(
                    "FailoverConditions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_preference {
                properties.insert(
                    "InputPreference".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secondary_input_id {
                properties.insert(
                    "SecondaryInputId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-av1colorspacesettings.html
    pub struct Av1ColorSpaceSettings_ {
        pub color_space_passthrough_settings: Option<Box<ColorSpacePassthroughSettings_>>,
        pub hdr10_settings: Option<Box<Hdr10Settings_>>,
        pub rec601_settings: Option<Box<Rec601Settings_>>,
        pub rec709_settings: Option<Box<Rec709Settings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_Av1ColorSpaceSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.Av1ColorSpaceSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_Av1ColorSpaceSettings as Av1ColorSpaceSettings;
    impl crate::value::ToValue for Av1ColorSpaceSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.color_space_passthrough_settings {
                properties.insert(
                    "ColorSpacePassthroughSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hdr10_settings {
                properties.insert(
                    "Hdr10Settings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rec601_settings {
                properties.insert(
                    "Rec601Settings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rec709_settings {
                properties.insert(
                    "Rec709Settings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-av1settings.html
    pub struct Av1Settings_ {
        pub afd_signaling: Option<crate::value::ExpString>,
        pub bitrate: Option<i64>,
        pub buf_size: Option<i64>,
        pub color_space_settings: Option<Box<Av1ColorSpaceSettings_>>,
        pub fixed_afd: Option<crate::value::ExpString>,
        pub framerate_denominator: Option<i64>,
        pub framerate_numerator: Option<i64>,
        pub gop_size: Option<f64>,
        pub gop_size_units: Option<crate::value::ExpString>,
        pub level: Option<crate::value::ExpString>,
        pub look_ahead_rate_control: Option<crate::value::ExpString>,
        pub max_bitrate: Option<i64>,
        pub min_i_interval: Option<i64>,
        pub par_denominator: Option<i64>,
        pub par_numerator: Option<i64>,
        pub qvbr_quality_level: Option<i64>,
        pub rate_control_mode: Option<crate::value::ExpString>,
        pub scene_change_detect: Option<crate::value::ExpString>,
        pub timecode_burnin_settings: Option<Box<TimecodeBurninSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_Av1Settings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.Av1Settings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_Av1Settings as Av1Settings;
    impl crate::value::ToValue for Av1Settings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.afd_signaling {
                properties.insert(
                    "AfdSignaling".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bitrate {
                properties.insert(
                    "Bitrate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.buf_size {
                properties.insert(
                    "BufSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.color_space_settings {
                properties.insert(
                    "ColorSpaceSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.fixed_afd {
                properties.insert(
                    "FixedAfd".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.framerate_denominator {
                properties.insert(
                    "FramerateDenominator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.framerate_numerator {
                properties.insert(
                    "FramerateNumerator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.gop_size {
                properties.insert(
                    "GopSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.gop_size_units {
                properties.insert(
                    "GopSizeUnits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.level {
                properties.insert("Level".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.look_ahead_rate_control {
                properties.insert(
                    "LookAheadRateControl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_bitrate {
                properties.insert(
                    "MaxBitrate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.min_i_interval {
                properties.insert(
                    "MinIInterval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.par_denominator {
                properties.insert(
                    "ParDenominator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.par_numerator {
                properties.insert(
                    "ParNumerator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.qvbr_quality_level {
                properties.insert(
                    "QvbrQualityLevel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rate_control_mode {
                properties.insert(
                    "RateControlMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scene_change_detect {
                properties.insert(
                    "SceneChangeDetect".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timecode_burnin_settings {
                properties.insert(
                    "TimecodeBurninSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-availblanking.html
    pub struct AvailBlanking_ {
        pub avail_blanking_image: Option<Box<InputLocation_>>,
        pub state: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_AvailBlanking {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.AvailBlanking"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_AvailBlanking as AvailBlanking;
    impl crate::value::ToValue for AvailBlanking_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.avail_blanking_image {
                properties.insert(
                    "AvailBlankingImage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.state {
                properties.insert("State".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-availconfiguration.html
    pub struct AvailConfiguration_ {
        pub avail_settings: Option<Box<AvailSettings_>>,
        pub scte35_segmentation_scope: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_AvailConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.AvailConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_AvailConfiguration as AvailConfiguration;
    impl crate::value::ToValue for AvailConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.avail_settings {
                properties.insert(
                    "AvailSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scte35_segmentation_scope {
                properties.insert(
                    "Scte35SegmentationScope".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-availsettings.html
    pub struct AvailSettings_ {
        pub esam: Option<Box<Esam_>>,
        pub scte35_splice_insert: Option<Box<Scte35SpliceInsert_>>,
        pub scte35_time_signal_apos: Option<Box<Scte35TimeSignalApos_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_AvailSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.AvailSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_AvailSettings as AvailSettings;
    impl crate::value::ToValue for AvailSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.esam {
                properties.insert("Esam".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.scte35_splice_insert {
                properties.insert(
                    "Scte35SpliceInsert".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scte35_time_signal_apos {
                properties.insert(
                    "Scte35TimeSignalApos".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-bandwidthreductionfiltersettings.html
    pub struct BandwidthReductionFilterSettings_ {
        pub post_filter_sharpening: Option<crate::value::ExpString>,
        pub strength: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_BandwidthReductionFilterSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.BandwidthReductionFilterSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_BandwidthReductionFilterSettings as BandwidthReductionFilterSettings;
    impl crate::value::ToValue for BandwidthReductionFilterSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.post_filter_sharpening {
                properties.insert(
                    "PostFilterSharpening".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.strength {
                properties.insert(
                    "Strength".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-blackoutslate.html
    pub struct BlackoutSlate_ {
        pub blackout_slate_image: Option<Box<InputLocation_>>,
        pub network_end_blackout: Option<crate::value::ExpString>,
        pub network_end_blackout_image: Option<Box<InputLocation_>>,
        pub network_id: Option<crate::value::ExpString>,
        pub state: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_BlackoutSlate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.BlackoutSlate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_BlackoutSlate as BlackoutSlate;
    impl crate::value::ToValue for BlackoutSlate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.blackout_slate_image {
                properties.insert(
                    "BlackoutSlateImage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_end_blackout {
                properties.insert(
                    "NetworkEndBlackout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_end_blackout_image {
                properties.insert(
                    "NetworkEndBlackoutImage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_id {
                properties.insert(
                    "NetworkId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.state {
                properties.insert("State".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-burnindestinationsettings.html
    pub struct BurnInDestinationSettings_ {
        pub alignment: Option<crate::value::ExpString>,
        pub background_color: Option<crate::value::ExpString>,
        pub background_opacity: Option<i64>,
        pub font: Option<Box<InputLocation_>>,
        pub font_color: Option<crate::value::ExpString>,
        pub font_opacity: Option<i64>,
        pub font_resolution: Option<i64>,
        pub font_size: Option<crate::value::ExpString>,
        pub outline_color: Option<crate::value::ExpString>,
        pub outline_size: Option<i64>,
        pub shadow_color: Option<crate::value::ExpString>,
        pub shadow_opacity: Option<i64>,
        pub shadow_x_offset: Option<i64>,
        pub shadow_y_offset: Option<i64>,
        pub subtitle_rows: Option<crate::value::ExpString>,
        pub teletext_grid_control: Option<crate::value::ExpString>,
        pub x_position: Option<i64>,
        pub y_position: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_BurnInDestinationSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.BurnInDestinationSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_BurnInDestinationSettings as BurnInDestinationSettings;
    impl crate::value::ToValue for BurnInDestinationSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.alignment {
                properties.insert(
                    "Alignment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.background_color {
                properties.insert(
                    "BackgroundColor".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.background_opacity {
                properties.insert(
                    "BackgroundOpacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.font {
                properties.insert("Font".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.font_color {
                properties.insert(
                    "FontColor".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.font_opacity {
                properties.insert(
                    "FontOpacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.font_resolution {
                properties.insert(
                    "FontResolution".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.font_size {
                properties.insert(
                    "FontSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.outline_color {
                properties.insert(
                    "OutlineColor".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.outline_size {
                properties.insert(
                    "OutlineSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.shadow_color {
                properties.insert(
                    "ShadowColor".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.shadow_opacity {
                properties.insert(
                    "ShadowOpacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.shadow_x_offset {
                properties.insert(
                    "ShadowXOffset".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.shadow_y_offset {
                properties.insert(
                    "ShadowYOffset".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.subtitle_rows {
                properties.insert(
                    "SubtitleRows".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.teletext_grid_control {
                properties.insert(
                    "TeletextGridControl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.x_position {
                properties.insert(
                    "XPosition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.y_position {
                properties.insert(
                    "YPosition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captiondescription.html
    pub struct CaptionDescription_ {
        pub accessibility: Option<crate::value::ExpString>,
        pub caption_dash_roles: Option<Vec<crate::value::ExpString>>,
        pub caption_selector_name: Option<crate::value::ExpString>,
        pub destination_settings: Option<Box<CaptionDestinationSettings_>>,
        pub dvb_dash_accessibility: Option<crate::value::ExpString>,
        pub language_code: Option<crate::value::ExpString>,
        pub language_description: Option<crate::value::ExpString>,
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_CaptionDescription {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.CaptionDescription"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_CaptionDescription as CaptionDescription;
    impl crate::value::ToValue for CaptionDescription_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.accessibility {
                properties.insert(
                    "Accessibility".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.caption_dash_roles {
                properties.insert(
                    "CaptionDashRoles".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.caption_selector_name {
                properties.insert(
                    "CaptionSelectorName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.destination_settings {
                properties.insert(
                    "DestinationSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dvb_dash_accessibility {
                properties.insert(
                    "DvbDashAccessibility".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.language_code {
                properties.insert(
                    "LanguageCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.language_description {
                properties.insert(
                    "LanguageDescription".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captiondestinationsettings.html
    pub struct CaptionDestinationSettings_ {
        pub arib_destination_settings: Option<Box<AribDestinationSettings_>>,
        pub burn_in_destination_settings: Option<Box<BurnInDestinationSettings_>>,
        pub dvb_sub_destination_settings: Option<Box<DvbSubDestinationSettings_>>,
        pub ebu_tt_d_destination_settings: Option<Box<EbuTtDDestinationSettings_>>,
        pub embedded_destination_settings: Option<Box<EmbeddedDestinationSettings_>>,
        pub embedded_plus_scte20_destination_settings:
            Option<Box<EmbeddedPlusScte20DestinationSettings_>>,
        pub rtmp_caption_info_destination_settings:
            Option<Box<RtmpCaptionInfoDestinationSettings_>>,
        pub scte20_plus_embedded_destination_settings:
            Option<Box<Scte20PlusEmbeddedDestinationSettings_>>,
        pub scte27_destination_settings: Option<Box<Scte27DestinationSettings_>>,
        pub smpte_tt_destination_settings: Option<Box<SmpteTtDestinationSettings_>>,
        pub teletext_destination_settings: Option<Box<TeletextDestinationSettings_>>,
        pub ttml_destination_settings: Option<Box<TtmlDestinationSettings_>>,
        pub webvtt_destination_settings: Option<Box<WebvttDestinationSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_CaptionDestinationSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.CaptionDestinationSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_CaptionDestinationSettings as CaptionDestinationSettings;
    impl crate::value::ToValue for CaptionDestinationSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.arib_destination_settings {
                properties.insert(
                    "AribDestinationSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.burn_in_destination_settings {
                properties.insert(
                    "BurnInDestinationSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dvb_sub_destination_settings {
                properties.insert(
                    "DvbSubDestinationSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ebu_tt_d_destination_settings {
                properties.insert(
                    "EbuTtDDestinationSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.embedded_destination_settings {
                properties.insert(
                    "EmbeddedDestinationSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.embedded_plus_scte20_destination_settings {
                properties.insert(
                    "EmbeddedPlusScte20DestinationSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rtmp_caption_info_destination_settings {
                properties.insert(
                    "RtmpCaptionInfoDestinationSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scte20_plus_embedded_destination_settings {
                properties.insert(
                    "Scte20PlusEmbeddedDestinationSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scte27_destination_settings {
                properties.insert(
                    "Scte27DestinationSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.smpte_tt_destination_settings {
                properties.insert(
                    "SmpteTtDestinationSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.teletext_destination_settings {
                properties.insert(
                    "TeletextDestinationSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ttml_destination_settings {
                properties.insert(
                    "TtmlDestinationSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.webvtt_destination_settings {
                properties.insert(
                    "WebvttDestinationSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captionlanguagemapping.html
    pub struct CaptionLanguageMapping_ {
        pub caption_channel: Option<i64>,
        pub language_code: Option<crate::value::ExpString>,
        pub language_description: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_CaptionLanguageMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.CaptionLanguageMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_CaptionLanguageMapping as CaptionLanguageMapping;
    impl crate::value::ToValue for CaptionLanguageMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.caption_channel {
                properties.insert(
                    "CaptionChannel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.language_code {
                properties.insert(
                    "LanguageCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.language_description {
                properties.insert(
                    "LanguageDescription".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captionrectangle.html
    pub struct CaptionRectangle_ {
        pub height: Option<f64>,
        pub left_offset: Option<f64>,
        pub top_offset: Option<f64>,
        pub width: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_CaptionRectangle {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.CaptionRectangle"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_CaptionRectangle as CaptionRectangle;
    impl crate::value::ToValue for CaptionRectangle_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.height {
                properties.insert("Height".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.left_offset {
                properties.insert(
                    "LeftOffset".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.top_offset {
                properties.insert(
                    "TopOffset".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.width {
                properties.insert("Width".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captionselector.html
    pub struct CaptionSelector_ {
        pub language_code: Option<crate::value::ExpString>,
        pub name: Option<crate::value::ExpString>,
        pub selector_settings: Option<Box<CaptionSelectorSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_CaptionSelector {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.CaptionSelector"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_CaptionSelector as CaptionSelector;
    impl crate::value::ToValue for CaptionSelector_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.language_code {
                properties.insert(
                    "LanguageCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.selector_settings {
                properties.insert(
                    "SelectorSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captionselectorsettings.html
    pub struct CaptionSelectorSettings_ {
        pub ancillary_source_settings: Option<Box<AncillarySourceSettings_>>,
        pub arib_source_settings: Option<Box<AribSourceSettings_>>,
        pub dvb_sub_source_settings: Option<Box<DvbSubSourceSettings_>>,
        pub embedded_source_settings: Option<Box<EmbeddedSourceSettings_>>,
        pub scte20_source_settings: Option<Box<Scte20SourceSettings_>>,
        pub scte27_source_settings: Option<Box<Scte27SourceSettings_>>,
        pub teletext_source_settings: Option<Box<TeletextSourceSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_CaptionSelectorSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.CaptionSelectorSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_CaptionSelectorSettings as CaptionSelectorSettings;
    impl crate::value::ToValue for CaptionSelectorSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ancillary_source_settings {
                properties.insert(
                    "AncillarySourceSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.arib_source_settings {
                properties.insert(
                    "AribSourceSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dvb_sub_source_settings {
                properties.insert(
                    "DvbSubSourceSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.embedded_source_settings {
                properties.insert(
                    "EmbeddedSourceSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scte20_source_settings {
                properties.insert(
                    "Scte20SourceSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scte27_source_settings {
                properties.insert(
                    "Scte27SourceSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.teletext_source_settings {
                properties.insert(
                    "TeletextSourceSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-cdiinputspecification.html
    pub struct CdiInputSpecification_ {
        pub resolution: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_CdiInputSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.CdiInputSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_CdiInputSpecification as CdiInputSpecification;
    impl crate::value::ToValue for CdiInputSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.resolution {
                properties.insert(
                    "Resolution".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-channelengineversionrequest.html
    pub struct ChannelEngineVersionRequest_ {
        pub version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_ChannelEngineVersionRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.ChannelEngineVersionRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_ChannelEngineVersionRequest as ChannelEngineVersionRequest;
    impl crate::value::ToValue for ChannelEngineVersionRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.version {
                properties.insert(
                    "Version".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-cmafingestcaptionlanguagemapping.html
    pub struct CmafIngestCaptionLanguageMapping_ {
        pub caption_channel: Option<i64>,
        pub language_code: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_CmafIngestCaptionLanguageMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.CmafIngestCaptionLanguageMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_CmafIngestCaptionLanguageMapping as CmafIngestCaptionLanguageMapping;
    impl crate::value::ToValue for CmafIngestCaptionLanguageMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.caption_channel {
                properties.insert(
                    "CaptionChannel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.language_code {
                properties.insert(
                    "LanguageCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-cmafingestgroupsettings.html
    pub struct CmafIngestGroupSettings_ {
        pub additional_destinations: Option<Vec<AdditionalDestinations_>>,
        pub caption_language_mappings: Option<Vec<CmafIngestCaptionLanguageMapping_>>,
        pub destination: Option<Box<OutputLocationRef_>>,
        pub id3_behavior: Option<crate::value::ExpString>,
        pub id3_name_modifier: Option<crate::value::ExpString>,
        pub klv_behavior: Option<crate::value::ExpString>,
        pub klv_name_modifier: Option<crate::value::ExpString>,
        pub nielsen_id3_behavior: Option<crate::value::ExpString>,
        pub nielsen_id3_name_modifier: Option<crate::value::ExpString>,
        pub scte35_name_modifier: Option<crate::value::ExpString>,
        pub scte35_type: Option<crate::value::ExpString>,
        pub segment_length: Option<i64>,
        pub segment_length_units: Option<crate::value::ExpString>,
        pub send_delay_ms: Option<i64>,
        pub timed_metadata_id3_frame: Option<crate::value::ExpString>,
        pub timed_metadata_id3_period: Option<i64>,
        pub timed_metadata_passthrough: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_CmafIngestGroupSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.CmafIngestGroupSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_CmafIngestGroupSettings as CmafIngestGroupSettings;
    impl crate::value::ToValue for CmafIngestGroupSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.additional_destinations {
                properties.insert(
                    "AdditionalDestinations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.caption_language_mappings {
                properties.insert(
                    "CaptionLanguageMappings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.destination {
                properties.insert(
                    "Destination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.id3_behavior {
                properties.insert(
                    "Id3Behavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.id3_name_modifier {
                properties.insert(
                    "Id3NameModifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.klv_behavior {
                properties.insert(
                    "KlvBehavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.klv_name_modifier {
                properties.insert(
                    "KlvNameModifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.nielsen_id3_behavior {
                properties.insert(
                    "NielsenId3Behavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.nielsen_id3_name_modifier {
                properties.insert(
                    "NielsenId3NameModifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scte35_name_modifier {
                properties.insert(
                    "Scte35NameModifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scte35_type {
                properties.insert(
                    "Scte35Type".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.segment_length {
                properties.insert(
                    "SegmentLength".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.segment_length_units {
                properties.insert(
                    "SegmentLengthUnits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.send_delay_ms {
                properties.insert(
                    "SendDelayMs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timed_metadata_id3_frame {
                properties.insert(
                    "TimedMetadataId3Frame".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timed_metadata_id3_period {
                properties.insert(
                    "TimedMetadataId3Period".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timed_metadata_passthrough {
                properties.insert(
                    "TimedMetadataPassthrough".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-cmafingestoutputsettings.html
    pub struct CmafIngestOutputSettings_ {
        pub name_modifier: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_CmafIngestOutputSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.CmafIngestOutputSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_CmafIngestOutputSettings as CmafIngestOutputSettings;
    impl crate::value::ToValue for CmafIngestOutputSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name_modifier {
                properties.insert(
                    "NameModifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-colorcorrection.html
    pub struct ColorCorrection_ {
        pub input_color_space: Option<crate::value::ExpString>,
        pub output_color_space: Option<crate::value::ExpString>,
        pub uri: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_ColorCorrection {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.ColorCorrection"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_ColorCorrection as ColorCorrection;
    impl crate::value::ToValue for ColorCorrection_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.input_color_space {
                properties.insert(
                    "InputColorSpace".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.output_color_space {
                properties.insert(
                    "OutputColorSpace".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.uri {
                properties.insert("Uri".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-colorcorrectionsettings.html
    pub struct ColorCorrectionSettings_ {
        pub global_color_corrections: Option<Vec<ColorCorrection_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_ColorCorrectionSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.ColorCorrectionSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_ColorCorrectionSettings as ColorCorrectionSettings;
    impl crate::value::ToValue for ColorCorrectionSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.global_color_corrections {
                properties.insert(
                    "GlobalColorCorrections".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-colorspacepassthroughsettings.html
    pub struct ColorSpacePassthroughSettings_ {}
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_ColorSpacePassthroughSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.ColorSpacePassthroughSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_ColorSpacePassthroughSettings as ColorSpacePassthroughSettings;
    impl crate::value::ToValue for ColorSpacePassthroughSettings_ {
        fn to_value(&self) -> serde_json::Value {
            serde_json::Value::Object(serde_json::Map::new())
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-dolbyvision81settings.html
    pub struct DolbyVision81Settings_ {}
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_DolbyVision81Settings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.DolbyVision81Settings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_DolbyVision81Settings as DolbyVision81Settings;
    impl crate::value::ToValue for DolbyVision81Settings_ {
        fn to_value(&self) -> serde_json::Value {
            serde_json::Value::Object(serde_json::Map::new())
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-dvbnitsettings.html
    pub struct DvbNitSettings_ {
        pub network_id: Option<i64>,
        pub network_name: Option<crate::value::ExpString>,
        pub rep_interval: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_DvbNitSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.DvbNitSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_DvbNitSettings as DvbNitSettings;
    impl crate::value::ToValue for DvbNitSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.network_id {
                properties.insert(
                    "NetworkId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_name {
                properties.insert(
                    "NetworkName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rep_interval {
                properties.insert(
                    "RepInterval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-dvbsdtsettings.html
    pub struct DvbSdtSettings_ {
        pub output_sdt: Option<crate::value::ExpString>,
        pub rep_interval: Option<i64>,
        pub service_name: Option<crate::value::ExpString>,
        pub service_provider_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_DvbSdtSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.DvbSdtSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_DvbSdtSettings as DvbSdtSettings;
    impl crate::value::ToValue for DvbSdtSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.output_sdt {
                properties.insert(
                    "OutputSdt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rep_interval {
                properties.insert(
                    "RepInterval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.service_name {
                properties.insert(
                    "ServiceName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.service_provider_name {
                properties.insert(
                    "ServiceProviderName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-dvbsubdestinationsettings.html
    pub struct DvbSubDestinationSettings_ {
        pub alignment: Option<crate::value::ExpString>,
        pub background_color: Option<crate::value::ExpString>,
        pub background_opacity: Option<i64>,
        pub font: Option<Box<InputLocation_>>,
        pub font_color: Option<crate::value::ExpString>,
        pub font_opacity: Option<i64>,
        pub font_resolution: Option<i64>,
        pub font_size: Option<crate::value::ExpString>,
        pub outline_color: Option<crate::value::ExpString>,
        pub outline_size: Option<i64>,
        pub shadow_color: Option<crate::value::ExpString>,
        pub shadow_opacity: Option<i64>,
        pub shadow_x_offset: Option<i64>,
        pub shadow_y_offset: Option<i64>,
        pub subtitle_rows: Option<crate::value::ExpString>,
        pub teletext_grid_control: Option<crate::value::ExpString>,
        pub x_position: Option<i64>,
        pub y_position: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_DvbSubDestinationSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.DvbSubDestinationSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_DvbSubDestinationSettings as DvbSubDestinationSettings;
    impl crate::value::ToValue for DvbSubDestinationSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.alignment {
                properties.insert(
                    "Alignment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.background_color {
                properties.insert(
                    "BackgroundColor".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.background_opacity {
                properties.insert(
                    "BackgroundOpacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.font {
                properties.insert("Font".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.font_color {
                properties.insert(
                    "FontColor".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.font_opacity {
                properties.insert(
                    "FontOpacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.font_resolution {
                properties.insert(
                    "FontResolution".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.font_size {
                properties.insert(
                    "FontSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.outline_color {
                properties.insert(
                    "OutlineColor".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.outline_size {
                properties.insert(
                    "OutlineSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.shadow_color {
                properties.insert(
                    "ShadowColor".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.shadow_opacity {
                properties.insert(
                    "ShadowOpacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.shadow_x_offset {
                properties.insert(
                    "ShadowXOffset".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.shadow_y_offset {
                properties.insert(
                    "ShadowYOffset".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.subtitle_rows {
                properties.insert(
                    "SubtitleRows".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.teletext_grid_control {
                properties.insert(
                    "TeletextGridControl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.x_position {
                properties.insert(
                    "XPosition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.y_position {
                properties.insert(
                    "YPosition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-dvbsubsourcesettings.html
    pub struct DvbSubSourceSettings_ {
        pub ocr_language: Option<crate::value::ExpString>,
        pub pid: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_DvbSubSourceSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.DvbSubSourceSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_DvbSubSourceSettings as DvbSubSourceSettings;
    impl crate::value::ToValue for DvbSubSourceSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ocr_language {
                properties.insert(
                    "OcrLanguage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pid {
                properties.insert("Pid".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-dvbtdtsettings.html
    pub struct DvbTdtSettings_ {
        pub rep_interval: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_DvbTdtSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.DvbTdtSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_DvbTdtSettings as DvbTdtSettings;
    impl crate::value::ToValue for DvbTdtSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.rep_interval {
                properties.insert(
                    "RepInterval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-eac3atmossettings.html
    pub struct Eac3AtmosSettings_ {
        pub bitrate: Option<f64>,
        pub coding_mode: Option<crate::value::ExpString>,
        pub dialnorm: Option<i64>,
        pub drc_line: Option<crate::value::ExpString>,
        pub drc_rf: Option<crate::value::ExpString>,
        pub height_trim: Option<f64>,
        pub surround_trim: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_Eac3AtmosSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.Eac3AtmosSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_Eac3AtmosSettings as Eac3AtmosSettings;
    impl crate::value::ToValue for Eac3AtmosSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bitrate {
                properties.insert(
                    "Bitrate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.coding_mode {
                properties.insert(
                    "CodingMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dialnorm {
                properties.insert(
                    "Dialnorm".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.drc_line {
                properties.insert(
                    "DrcLine".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.drc_rf {
                properties.insert("DrcRf".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.height_trim {
                properties.insert(
                    "HeightTrim".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.surround_trim {
                properties.insert(
                    "SurroundTrim".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-eac3settings.html
    pub struct Eac3Settings_ {
        pub attenuation_control: Option<crate::value::ExpString>,
        pub bitrate: Option<f64>,
        pub bitstream_mode: Option<crate::value::ExpString>,
        pub coding_mode: Option<crate::value::ExpString>,
        pub dc_filter: Option<crate::value::ExpString>,
        pub dialnorm: Option<i64>,
        pub drc_line: Option<crate::value::ExpString>,
        pub drc_rf: Option<crate::value::ExpString>,
        pub lfe_control: Option<crate::value::ExpString>,
        pub lfe_filter: Option<crate::value::ExpString>,
        pub lo_ro_center_mix_level: Option<f64>,
        pub lo_ro_surround_mix_level: Option<f64>,
        pub lt_rt_center_mix_level: Option<f64>,
        pub lt_rt_surround_mix_level: Option<f64>,
        pub metadata_control: Option<crate::value::ExpString>,
        pub passthrough_control: Option<crate::value::ExpString>,
        pub phase_control: Option<crate::value::ExpString>,
        pub stereo_downmix: Option<crate::value::ExpString>,
        pub surround_ex_mode: Option<crate::value::ExpString>,
        pub surround_mode: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_Eac3Settings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.Eac3Settings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_Eac3Settings as Eac3Settings;
    impl crate::value::ToValue for Eac3Settings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attenuation_control {
                properties.insert(
                    "AttenuationControl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bitrate {
                properties.insert(
                    "Bitrate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bitstream_mode {
                properties.insert(
                    "BitstreamMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.coding_mode {
                properties.insert(
                    "CodingMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dc_filter {
                properties.insert(
                    "DcFilter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dialnorm {
                properties.insert(
                    "Dialnorm".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.drc_line {
                properties.insert(
                    "DrcLine".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.drc_rf {
                properties.insert("DrcRf".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.lfe_control {
                properties.insert(
                    "LfeControl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lfe_filter {
                properties.insert(
                    "LfeFilter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lo_ro_center_mix_level {
                properties.insert(
                    "LoRoCenterMixLevel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lo_ro_surround_mix_level {
                properties.insert(
                    "LoRoSurroundMixLevel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lt_rt_center_mix_level {
                properties.insert(
                    "LtRtCenterMixLevel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lt_rt_surround_mix_level {
                properties.insert(
                    "LtRtSurroundMixLevel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metadata_control {
                properties.insert(
                    "MetadataControl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.passthrough_control {
                properties.insert(
                    "PassthroughControl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.phase_control {
                properties.insert(
                    "PhaseControl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stereo_downmix {
                properties.insert(
                    "StereoDownmix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.surround_ex_mode {
                properties.insert(
                    "SurroundExMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.surround_mode {
                properties.insert(
                    "SurroundMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-ebuttddestinationsettings.html
    pub struct EbuTtDDestinationSettings_ {
        pub copyright_holder: Option<crate::value::ExpString>,
        pub default_font_size: Option<i64>,
        pub default_line_height: Option<i64>,
        pub fill_line_gap: Option<crate::value::ExpString>,
        pub font_family: Option<crate::value::ExpString>,
        pub style_control: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_EbuTtDDestinationSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.EbuTtDDestinationSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_EbuTtDDestinationSettings as EbuTtDDestinationSettings;
    impl crate::value::ToValue for EbuTtDDestinationSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.copyright_holder {
                properties.insert(
                    "CopyrightHolder".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.default_font_size {
                properties.insert(
                    "DefaultFontSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.default_line_height {
                properties.insert(
                    "DefaultLineHeight".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.fill_line_gap {
                properties.insert(
                    "FillLineGap".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.font_family {
                properties.insert(
                    "FontFamily".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.style_control {
                properties.insert(
                    "StyleControl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-embeddeddestinationsettings.html
    pub struct EmbeddedDestinationSettings_ {}
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_EmbeddedDestinationSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.EmbeddedDestinationSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_EmbeddedDestinationSettings as EmbeddedDestinationSettings;
    impl crate::value::ToValue for EmbeddedDestinationSettings_ {
        fn to_value(&self) -> serde_json::Value {
            serde_json::Value::Object(serde_json::Map::new())
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-embeddedplusscte20destinationsettings.html
    pub struct EmbeddedPlusScte20DestinationSettings_ {}
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_EmbeddedPlusScte20DestinationSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.EmbeddedPlusScte20DestinationSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_EmbeddedPlusScte20DestinationSettings as EmbeddedPlusScte20DestinationSettings;
    impl crate::value::ToValue for EmbeddedPlusScte20DestinationSettings_ {
        fn to_value(&self) -> serde_json::Value {
            serde_json::Value::Object(serde_json::Map::new())
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-embeddedsourcesettings.html
    pub struct EmbeddedSourceSettings_ {
        pub convert608_to708: Option<crate::value::ExpString>,
        pub scte20_detection: Option<crate::value::ExpString>,
        pub source608_channel_number: Option<i64>,
        pub source608_track_number: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_EmbeddedSourceSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.EmbeddedSourceSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_EmbeddedSourceSettings as EmbeddedSourceSettings;
    impl crate::value::ToValue for EmbeddedSourceSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.convert608_to708 {
                properties.insert(
                    "Convert608To708".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scte20_detection {
                properties.insert(
                    "Scte20Detection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source608_channel_number {
                properties.insert(
                    "Source608ChannelNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source608_track_number {
                properties.insert(
                    "Source608TrackNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-encodersettings.html
    pub struct EncoderSettings_ {
        pub audio_descriptions: Option<Vec<AudioDescription_>>,
        pub avail_blanking: Option<Box<AvailBlanking_>>,
        pub avail_configuration: Option<Box<AvailConfiguration_>>,
        pub blackout_slate: Option<Box<BlackoutSlate_>>,
        pub caption_descriptions: Option<Vec<CaptionDescription_>>,
        pub color_correction_settings: Option<Box<ColorCorrectionSettings_>>,
        pub feature_activations: Option<Box<FeatureActivations_>>,
        pub global_configuration: Option<Box<GlobalConfiguration_>>,
        pub motion_graphics_configuration: Option<Box<MotionGraphicsConfiguration_>>,
        pub nielsen_configuration: Option<Box<NielsenConfiguration_>>,
        pub output_groups: Option<Vec<OutputGroup_>>,
        pub thumbnail_configuration: Option<Box<ThumbnailConfiguration_>>,
        pub timecode_config: Option<Box<TimecodeConfig_>>,
        pub video_descriptions: Option<Vec<VideoDescription_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_EncoderSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.EncoderSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_EncoderSettings as EncoderSettings;
    impl crate::value::ToValue for EncoderSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.audio_descriptions {
                properties.insert(
                    "AudioDescriptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.avail_blanking {
                properties.insert(
                    "AvailBlanking".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.avail_configuration {
                properties.insert(
                    "AvailConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.blackout_slate {
                properties.insert(
                    "BlackoutSlate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.caption_descriptions {
                properties.insert(
                    "CaptionDescriptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.color_correction_settings {
                properties.insert(
                    "ColorCorrectionSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.feature_activations {
                properties.insert(
                    "FeatureActivations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.global_configuration {
                properties.insert(
                    "GlobalConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.motion_graphics_configuration {
                properties.insert(
                    "MotionGraphicsConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.nielsen_configuration {
                properties.insert(
                    "NielsenConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.output_groups {
                properties.insert(
                    "OutputGroups".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.thumbnail_configuration {
                properties.insert(
                    "ThumbnailConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timecode_config {
                properties.insert(
                    "TimecodeConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.video_descriptions {
                properties.insert(
                    "VideoDescriptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-epochlockingsettings.html
    pub struct EpochLockingSettings_ {
        pub custom_epoch: Option<crate::value::ExpString>,
        pub jam_sync_time: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_EpochLockingSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.EpochLockingSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_EpochLockingSettings as EpochLockingSettings;
    impl crate::value::ToValue for EpochLockingSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_epoch {
                properties.insert(
                    "CustomEpoch".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.jam_sync_time {
                properties.insert(
                    "JamSyncTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-esam.html
    pub struct Esam_ {
        pub acquisition_point_id: Option<crate::value::ExpString>,
        pub ad_avail_offset: Option<i64>,
        pub password_param: Option<crate::value::ExpString>,
        pub pois_endpoint: Option<crate::value::ExpString>,
        pub username: Option<crate::value::ExpString>,
        pub zone_identity: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_Esam {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.Esam"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_Esam as Esam;
    impl crate::value::ToValue for Esam_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.acquisition_point_id {
                properties.insert(
                    "AcquisitionPointId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ad_avail_offset {
                properties.insert(
                    "AdAvailOffset".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.password_param {
                properties.insert(
                    "PasswordParam".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pois_endpoint {
                properties.insert(
                    "PoisEndpoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.username {
                properties.insert(
                    "Username".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.zone_identity {
                properties.insert(
                    "ZoneIdentity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-failovercondition.html
    pub struct FailoverCondition_ {
        pub failover_condition_settings: Option<Box<FailoverConditionSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_FailoverCondition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.FailoverCondition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_FailoverCondition as FailoverCondition;
    impl crate::value::ToValue for FailoverCondition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.failover_condition_settings {
                properties.insert(
                    "FailoverConditionSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-failoverconditionsettings.html
    pub struct FailoverConditionSettings_ {
        pub audio_silence_settings: Option<Box<AudioSilenceFailoverSettings_>>,
        pub input_loss_settings: Option<Box<InputLossFailoverSettings_>>,
        pub video_black_settings: Option<Box<VideoBlackFailoverSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_FailoverConditionSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.FailoverConditionSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_FailoverConditionSettings as FailoverConditionSettings;
    impl crate::value::ToValue for FailoverConditionSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.audio_silence_settings {
                properties.insert(
                    "AudioSilenceSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_loss_settings {
                properties.insert(
                    "InputLossSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.video_black_settings {
                properties.insert(
                    "VideoBlackSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-featureactivations.html
    pub struct FeatureActivations_ {
        pub input_prepare_schedule_actions: Option<crate::value::ExpString>,
        pub output_static_image_overlay_schedule_actions: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_FeatureActivations {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.FeatureActivations"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_FeatureActivations as FeatureActivations;
    impl crate::value::ToValue for FeatureActivations_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.input_prepare_schedule_actions {
                properties.insert(
                    "InputPrepareScheduleActions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.output_static_image_overlay_schedule_actions {
                properties.insert(
                    "OutputStaticImageOverlayScheduleActions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-fecoutputsettings.html
    pub struct FecOutputSettings_ {
        pub column_depth: Option<i64>,
        pub include_fec: Option<crate::value::ExpString>,
        pub row_length: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_FecOutputSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.FecOutputSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_FecOutputSettings as FecOutputSettings;
    impl crate::value::ToValue for FecOutputSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.column_depth {
                properties.insert(
                    "ColumnDepth".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_fec {
                properties.insert(
                    "IncludeFec".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.row_length {
                properties.insert(
                    "RowLength".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-fmp4hlssettings.html
    pub struct Fmp4HlsSettings_ {
        pub audio_rendition_sets: Option<crate::value::ExpString>,
        pub nielsen_id3_behavior: Option<crate::value::ExpString>,
        pub timed_metadata_behavior: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_Fmp4HlsSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.Fmp4HlsSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_Fmp4HlsSettings as Fmp4HlsSettings;
    impl crate::value::ToValue for Fmp4HlsSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.audio_rendition_sets {
                properties.insert(
                    "AudioRenditionSets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.nielsen_id3_behavior {
                properties.insert(
                    "NielsenId3Behavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timed_metadata_behavior {
                properties.insert(
                    "TimedMetadataBehavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-framecapturecdnsettings.html
    pub struct FrameCaptureCdnSettings_ {
        pub frame_capture_s3_settings: Option<Box<FrameCaptureS3Settings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_FrameCaptureCdnSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.FrameCaptureCdnSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_FrameCaptureCdnSettings as FrameCaptureCdnSettings;
    impl crate::value::ToValue for FrameCaptureCdnSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.frame_capture_s3_settings {
                properties.insert(
                    "FrameCaptureS3Settings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-framecapturegroupsettings.html
    pub struct FrameCaptureGroupSettings_ {
        pub destination: Option<Box<OutputLocationRef_>>,
        pub frame_capture_cdn_settings: Option<Box<FrameCaptureCdnSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_FrameCaptureGroupSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.FrameCaptureGroupSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_FrameCaptureGroupSettings as FrameCaptureGroupSettings;
    impl crate::value::ToValue for FrameCaptureGroupSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.destination {
                properties.insert(
                    "Destination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.frame_capture_cdn_settings {
                properties.insert(
                    "FrameCaptureCdnSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-framecapturehlssettings.html
    pub struct FrameCaptureHlsSettings_ {}
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_FrameCaptureHlsSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.FrameCaptureHlsSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_FrameCaptureHlsSettings as FrameCaptureHlsSettings;
    impl crate::value::ToValue for FrameCaptureHlsSettings_ {
        fn to_value(&self) -> serde_json::Value {
            serde_json::Value::Object(serde_json::Map::new())
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-framecaptureoutputsettings.html
    pub struct FrameCaptureOutputSettings_ {
        pub name_modifier: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_FrameCaptureOutputSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.FrameCaptureOutputSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_FrameCaptureOutputSettings as FrameCaptureOutputSettings;
    impl crate::value::ToValue for FrameCaptureOutputSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name_modifier {
                properties.insert(
                    "NameModifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-framecaptures3settings.html
    pub struct FrameCaptureS3Settings_ {
        pub canned_acl: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_FrameCaptureS3Settings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.FrameCaptureS3Settings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_FrameCaptureS3Settings as FrameCaptureS3Settings;
    impl crate::value::ToValue for FrameCaptureS3Settings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.canned_acl {
                properties.insert(
                    "CannedAcl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-framecapturesettings.html
    pub struct FrameCaptureSettings_ {
        pub capture_interval: Option<i64>,
        pub capture_interval_units: Option<crate::value::ExpString>,
        pub timecode_burnin_settings: Option<Box<TimecodeBurninSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_FrameCaptureSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.FrameCaptureSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_FrameCaptureSettings as FrameCaptureSettings;
    impl crate::value::ToValue for FrameCaptureSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.capture_interval {
                properties.insert(
                    "CaptureInterval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.capture_interval_units {
                properties.insert(
                    "CaptureIntervalUnits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timecode_burnin_settings {
                properties.insert(
                    "TimecodeBurninSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-globalconfiguration.html
    pub struct GlobalConfiguration_ {
        pub initial_audio_gain: Option<i64>,
        pub input_end_action: Option<crate::value::ExpString>,
        pub input_loss_behavior: Option<Box<InputLossBehavior_>>,
        pub output_locking_mode: Option<crate::value::ExpString>,
        pub output_locking_settings: Option<Box<OutputLockingSettings_>>,
        pub output_timing_source: Option<crate::value::ExpString>,
        pub support_low_framerate_inputs: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_GlobalConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.GlobalConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_GlobalConfiguration as GlobalConfiguration;
    impl crate::value::ToValue for GlobalConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.initial_audio_gain {
                properties.insert(
                    "InitialAudioGain".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_end_action {
                properties.insert(
                    "InputEndAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_loss_behavior {
                properties.insert(
                    "InputLossBehavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.output_locking_mode {
                properties.insert(
                    "OutputLockingMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.output_locking_settings {
                properties.insert(
                    "OutputLockingSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.output_timing_source {
                properties.insert(
                    "OutputTimingSource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.support_low_framerate_inputs {
                properties.insert(
                    "SupportLowFramerateInputs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264colorspacesettings.html
    pub struct H264ColorSpaceSettings_ {
        pub color_space_passthrough_settings: Option<Box<ColorSpacePassthroughSettings_>>,
        pub rec601_settings: Option<Box<Rec601Settings_>>,
        pub rec709_settings: Option<Box<Rec709Settings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_H264ColorSpaceSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.H264ColorSpaceSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_H264ColorSpaceSettings as H264ColorSpaceSettings;
    impl crate::value::ToValue for H264ColorSpaceSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.color_space_passthrough_settings {
                properties.insert(
                    "ColorSpacePassthroughSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rec601_settings {
                properties.insert(
                    "Rec601Settings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rec709_settings {
                properties.insert(
                    "Rec709Settings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264filtersettings.html
    pub struct H264FilterSettings_ {
        pub bandwidth_reduction_filter_settings: Option<Box<BandwidthReductionFilterSettings_>>,
        pub temporal_filter_settings: Option<Box<TemporalFilterSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_H264FilterSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.H264FilterSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_H264FilterSettings as H264FilterSettings;
    impl crate::value::ToValue for H264FilterSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bandwidth_reduction_filter_settings {
                properties.insert(
                    "BandwidthReductionFilterSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.temporal_filter_settings {
                properties.insert(
                    "TemporalFilterSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html
    pub struct H264Settings_ {
        pub adaptive_quantization: Option<crate::value::ExpString>,
        pub afd_signaling: Option<crate::value::ExpString>,
        pub bitrate: Option<i64>,
        pub buf_fill_pct: Option<i64>,
        pub buf_size: Option<i64>,
        pub color_metadata: Option<crate::value::ExpString>,
        pub color_space_settings: Option<Box<H264ColorSpaceSettings_>>,
        pub entropy_encoding: Option<crate::value::ExpString>,
        pub filter_settings: Option<Box<H264FilterSettings_>>,
        pub fixed_afd: Option<crate::value::ExpString>,
        pub flicker_aq: Option<crate::value::ExpString>,
        pub force_field_pictures: Option<crate::value::ExpString>,
        pub framerate_control: Option<crate::value::ExpString>,
        pub framerate_denominator: Option<i64>,
        pub framerate_numerator: Option<i64>,
        pub gop_b_reference: Option<crate::value::ExpString>,
        pub gop_closed_cadence: Option<i64>,
        pub gop_num_b_frames: Option<i64>,
        pub gop_size: Option<f64>,
        pub gop_size_units: Option<crate::value::ExpString>,
        pub level: Option<crate::value::ExpString>,
        pub look_ahead_rate_control: Option<crate::value::ExpString>,
        pub max_bitrate: Option<i64>,
        pub min_i_interval: Option<i64>,
        pub min_qp: Option<i64>,
        pub num_ref_frames: Option<i64>,
        pub par_control: Option<crate::value::ExpString>,
        pub par_denominator: Option<i64>,
        pub par_numerator: Option<i64>,
        pub profile: Option<crate::value::ExpString>,
        pub quality_level: Option<crate::value::ExpString>,
        pub qvbr_quality_level: Option<i64>,
        pub rate_control_mode: Option<crate::value::ExpString>,
        pub scan_type: Option<crate::value::ExpString>,
        pub scene_change_detect: Option<crate::value::ExpString>,
        pub slices: Option<i64>,
        pub softness: Option<i64>,
        pub spatial_aq: Option<crate::value::ExpString>,
        pub subgop_length: Option<crate::value::ExpString>,
        pub syntax: Option<crate::value::ExpString>,
        pub temporal_aq: Option<crate::value::ExpString>,
        pub timecode_burnin_settings: Option<Box<TimecodeBurninSettings_>>,
        pub timecode_insertion: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_H264Settings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.H264Settings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_H264Settings as H264Settings;
    impl crate::value::ToValue for H264Settings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.adaptive_quantization {
                properties.insert(
                    "AdaptiveQuantization".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.afd_signaling {
                properties.insert(
                    "AfdSignaling".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bitrate {
                properties.insert(
                    "Bitrate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.buf_fill_pct {
                properties.insert(
                    "BufFillPct".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.buf_size {
                properties.insert(
                    "BufSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.color_metadata {
                properties.insert(
                    "ColorMetadata".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.color_space_settings {
                properties.insert(
                    "ColorSpaceSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.entropy_encoding {
                properties.insert(
                    "EntropyEncoding".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.filter_settings {
                properties.insert(
                    "FilterSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.fixed_afd {
                properties.insert(
                    "FixedAfd".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.flicker_aq {
                properties.insert(
                    "FlickerAq".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.force_field_pictures {
                properties.insert(
                    "ForceFieldPictures".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.framerate_control {
                properties.insert(
                    "FramerateControl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.framerate_denominator {
                properties.insert(
                    "FramerateDenominator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.framerate_numerator {
                properties.insert(
                    "FramerateNumerator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.gop_b_reference {
                properties.insert(
                    "GopBReference".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.gop_closed_cadence {
                properties.insert(
                    "GopClosedCadence".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.gop_num_b_frames {
                properties.insert(
                    "GopNumBFrames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.gop_size {
                properties.insert(
                    "GopSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.gop_size_units {
                properties.insert(
                    "GopSizeUnits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.level {
                properties.insert("Level".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.look_ahead_rate_control {
                properties.insert(
                    "LookAheadRateControl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_bitrate {
                properties.insert(
                    "MaxBitrate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.min_i_interval {
                properties.insert(
                    "MinIInterval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.min_qp {
                properties.insert("MinQp".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.num_ref_frames {
                properties.insert(
                    "NumRefFrames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.par_control {
                properties.insert(
                    "ParControl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.par_denominator {
                properties.insert(
                    "ParDenominator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.par_numerator {
                properties.insert(
                    "ParNumerator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.profile {
                properties.insert(
                    "Profile".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.quality_level {
                properties.insert(
                    "QualityLevel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.qvbr_quality_level {
                properties.insert(
                    "QvbrQualityLevel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rate_control_mode {
                properties.insert(
                    "RateControlMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scan_type {
                properties.insert(
                    "ScanType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scene_change_detect {
                properties.insert(
                    "SceneChangeDetect".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.slices {
                properties.insert("Slices".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.softness {
                properties.insert(
                    "Softness".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.spatial_aq {
                properties.insert(
                    "SpatialAq".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.subgop_length {
                properties.insert(
                    "SubgopLength".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.syntax {
                properties.insert("Syntax".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.temporal_aq {
                properties.insert(
                    "TemporalAq".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timecode_burnin_settings {
                properties.insert(
                    "TimecodeBurninSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timecode_insertion {
                properties.insert(
                    "TimecodeInsertion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265colorspacesettings.html
    pub struct H265ColorSpaceSettings_ {
        pub color_space_passthrough_settings: Option<Box<ColorSpacePassthroughSettings_>>,
        pub dolby_vision81_settings: Option<Box<DolbyVision81Settings_>>,
        pub hdr10_settings: Option<Box<Hdr10Settings_>>,
        pub rec601_settings: Option<Box<Rec601Settings_>>,
        pub rec709_settings: Option<Box<Rec709Settings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_H265ColorSpaceSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.H265ColorSpaceSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_H265ColorSpaceSettings as H265ColorSpaceSettings;
    impl crate::value::ToValue for H265ColorSpaceSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.color_space_passthrough_settings {
                properties.insert(
                    "ColorSpacePassthroughSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dolby_vision81_settings {
                properties.insert(
                    "DolbyVision81Settings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hdr10_settings {
                properties.insert(
                    "Hdr10Settings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rec601_settings {
                properties.insert(
                    "Rec601Settings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rec709_settings {
                properties.insert(
                    "Rec709Settings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265filtersettings.html
    pub struct H265FilterSettings_ {
        pub bandwidth_reduction_filter_settings: Option<Box<BandwidthReductionFilterSettings_>>,
        pub temporal_filter_settings: Option<Box<TemporalFilterSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_H265FilterSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.H265FilterSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_H265FilterSettings as H265FilterSettings;
    impl crate::value::ToValue for H265FilterSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bandwidth_reduction_filter_settings {
                properties.insert(
                    "BandwidthReductionFilterSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.temporal_filter_settings {
                properties.insert(
                    "TemporalFilterSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265settings.html
    pub struct H265Settings_ {
        pub adaptive_quantization: Option<crate::value::ExpString>,
        pub afd_signaling: Option<crate::value::ExpString>,
        pub alternative_transfer_function: Option<crate::value::ExpString>,
        pub bitrate: Option<i64>,
        pub buf_size: Option<i64>,
        pub color_metadata: Option<crate::value::ExpString>,
        pub color_space_settings: Option<Box<H265ColorSpaceSettings_>>,
        pub deblocking: Option<crate::value::ExpString>,
        pub filter_settings: Option<Box<H265FilterSettings_>>,
        pub fixed_afd: Option<crate::value::ExpString>,
        pub flicker_aq: Option<crate::value::ExpString>,
        pub framerate_denominator: Option<i64>,
        pub framerate_numerator: Option<i64>,
        pub gop_closed_cadence: Option<i64>,
        pub gop_size: Option<f64>,
        pub gop_size_units: Option<crate::value::ExpString>,
        pub level: Option<crate::value::ExpString>,
        pub look_ahead_rate_control: Option<crate::value::ExpString>,
        pub max_bitrate: Option<i64>,
        pub min_i_interval: Option<i64>,
        pub min_qp: Option<i64>,
        pub mv_over_picture_boundaries: Option<crate::value::ExpString>,
        pub mv_temporal_predictor: Option<crate::value::ExpString>,
        pub par_denominator: Option<i64>,
        pub par_numerator: Option<i64>,
        pub profile: Option<crate::value::ExpString>,
        pub qvbr_quality_level: Option<i64>,
        pub rate_control_mode: Option<crate::value::ExpString>,
        pub scan_type: Option<crate::value::ExpString>,
        pub scene_change_detect: Option<crate::value::ExpString>,
        pub slices: Option<i64>,
        pub tier: Option<crate::value::ExpString>,
        pub tile_height: Option<i64>,
        pub tile_padding: Option<crate::value::ExpString>,
        pub tile_width: Option<i64>,
        pub timecode_burnin_settings: Option<Box<TimecodeBurninSettings_>>,
        pub timecode_insertion: Option<crate::value::ExpString>,
        pub treeblock_size: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_H265Settings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.H265Settings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_H265Settings as H265Settings;
    impl crate::value::ToValue for H265Settings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.adaptive_quantization {
                properties.insert(
                    "AdaptiveQuantization".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.afd_signaling {
                properties.insert(
                    "AfdSignaling".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.alternative_transfer_function {
                properties.insert(
                    "AlternativeTransferFunction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bitrate {
                properties.insert(
                    "Bitrate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.buf_size {
                properties.insert(
                    "BufSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.color_metadata {
                properties.insert(
                    "ColorMetadata".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.color_space_settings {
                properties.insert(
                    "ColorSpaceSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.deblocking {
                properties.insert(
                    "Deblocking".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.filter_settings {
                properties.insert(
                    "FilterSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.fixed_afd {
                properties.insert(
                    "FixedAfd".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.flicker_aq {
                properties.insert(
                    "FlickerAq".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.framerate_denominator {
                properties.insert(
                    "FramerateDenominator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.framerate_numerator {
                properties.insert(
                    "FramerateNumerator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.gop_closed_cadence {
                properties.insert(
                    "GopClosedCadence".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.gop_size {
                properties.insert(
                    "GopSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.gop_size_units {
                properties.insert(
                    "GopSizeUnits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.level {
                properties.insert("Level".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.look_ahead_rate_control {
                properties.insert(
                    "LookAheadRateControl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_bitrate {
                properties.insert(
                    "MaxBitrate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.min_i_interval {
                properties.insert(
                    "MinIInterval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.min_qp {
                properties.insert("MinQp".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.mv_over_picture_boundaries {
                properties.insert(
                    "MvOverPictureBoundaries".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mv_temporal_predictor {
                properties.insert(
                    "MvTemporalPredictor".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.par_denominator {
                properties.insert(
                    "ParDenominator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.par_numerator {
                properties.insert(
                    "ParNumerator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.profile {
                properties.insert(
                    "Profile".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.qvbr_quality_level {
                properties.insert(
                    "QvbrQualityLevel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rate_control_mode {
                properties.insert(
                    "RateControlMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scan_type {
                properties.insert(
                    "ScanType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scene_change_detect {
                properties.insert(
                    "SceneChangeDetect".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.slices {
                properties.insert("Slices".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.tier {
                properties.insert("Tier".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.tile_height {
                properties.insert(
                    "TileHeight".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tile_padding {
                properties.insert(
                    "TilePadding".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tile_width {
                properties.insert(
                    "TileWidth".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timecode_burnin_settings {
                properties.insert(
                    "TimecodeBurninSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timecode_insertion {
                properties.insert(
                    "TimecodeInsertion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.treeblock_size {
                properties.insert(
                    "TreeblockSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hdr10settings.html
    pub struct Hdr10Settings_ {
        pub max_cll: Option<i64>,
        pub max_fall: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_Hdr10Settings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.Hdr10Settings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_Hdr10Settings as Hdr10Settings;
    impl crate::value::ToValue for Hdr10Settings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max_cll {
                properties.insert("MaxCll".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.max_fall {
                properties.insert(
                    "MaxFall".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsakamaisettings.html
    pub struct HlsAkamaiSettings_ {
        pub connection_retry_interval: Option<i64>,
        pub filecache_duration: Option<i64>,
        pub http_transfer_mode: Option<crate::value::ExpString>,
        pub num_retries: Option<i64>,
        pub restart_delay: Option<i64>,
        pub salt: Option<crate::value::ExpString>,
        pub token: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_HlsAkamaiSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.HlsAkamaiSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_HlsAkamaiSettings as HlsAkamaiSettings;
    impl crate::value::ToValue for HlsAkamaiSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.connection_retry_interval {
                properties.insert(
                    "ConnectionRetryInterval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.filecache_duration {
                properties.insert(
                    "FilecacheDuration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.http_transfer_mode {
                properties.insert(
                    "HttpTransferMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.num_retries {
                properties.insert(
                    "NumRetries".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.restart_delay {
                properties.insert(
                    "RestartDelay".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.salt {
                properties.insert("Salt".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.token {
                properties.insert("Token".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsbasicputsettings.html
    pub struct HlsBasicPutSettings_ {
        pub connection_retry_interval: Option<i64>,
        pub filecache_duration: Option<i64>,
        pub num_retries: Option<i64>,
        pub restart_delay: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_HlsBasicPutSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.HlsBasicPutSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_HlsBasicPutSettings as HlsBasicPutSettings;
    impl crate::value::ToValue for HlsBasicPutSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.connection_retry_interval {
                properties.insert(
                    "ConnectionRetryInterval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.filecache_duration {
                properties.insert(
                    "FilecacheDuration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.num_retries {
                properties.insert(
                    "NumRetries".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.restart_delay {
                properties.insert(
                    "RestartDelay".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlscdnsettings.html
    pub struct HlsCdnSettings_ {
        pub hls_akamai_settings: Option<Box<HlsAkamaiSettings_>>,
        pub hls_basic_put_settings: Option<Box<HlsBasicPutSettings_>>,
        pub hls_media_store_settings: Option<Box<HlsMediaStoreSettings_>>,
        pub hls_s3_settings: Option<Box<HlsS3Settings_>>,
        pub hls_webdav_settings: Option<Box<HlsWebdavSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_HlsCdnSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.HlsCdnSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_HlsCdnSettings as HlsCdnSettings;
    impl crate::value::ToValue for HlsCdnSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.hls_akamai_settings {
                properties.insert(
                    "HlsAkamaiSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hls_basic_put_settings {
                properties.insert(
                    "HlsBasicPutSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hls_media_store_settings {
                properties.insert(
                    "HlsMediaStoreSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hls_s3_settings {
                properties.insert(
                    "HlsS3Settings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hls_webdav_settings {
                properties.insert(
                    "HlsWebdavSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html
    pub struct HlsGroupSettings_ {
        pub ad_markers: Option<Vec<crate::value::ExpString>>,
        pub base_url_content: Option<crate::value::ExpString>,
        pub base_url_content1: Option<crate::value::ExpString>,
        pub base_url_manifest: Option<crate::value::ExpString>,
        pub base_url_manifest1: Option<crate::value::ExpString>,
        pub caption_language_mappings: Option<Vec<CaptionLanguageMapping_>>,
        pub caption_language_setting: Option<crate::value::ExpString>,
        pub client_cache: Option<crate::value::ExpString>,
        pub codec_specification: Option<crate::value::ExpString>,
        pub constant_iv: Option<crate::value::ExpString>,
        pub destination: Option<Box<OutputLocationRef_>>,
        pub directory_structure: Option<crate::value::ExpString>,
        pub discontinuity_tags: Option<crate::value::ExpString>,
        pub encryption_type: Option<crate::value::ExpString>,
        pub hls_cdn_settings: Option<Box<HlsCdnSettings_>>,
        pub hls_id3_segment_tagging: Option<crate::value::ExpString>,
        pub i_frame_only_playlists: Option<crate::value::ExpString>,
        pub incomplete_segment_behavior: Option<crate::value::ExpString>,
        pub index_n_segments: Option<i64>,
        pub input_loss_action: Option<crate::value::ExpString>,
        pub iv_in_manifest: Option<crate::value::ExpString>,
        pub iv_source: Option<crate::value::ExpString>,
        pub keep_segments: Option<i64>,
        pub key_format: Option<crate::value::ExpString>,
        pub key_format_versions: Option<crate::value::ExpString>,
        pub key_provider_settings: Option<Box<KeyProviderSettings_>>,
        pub manifest_compression: Option<crate::value::ExpString>,
        pub manifest_duration_format: Option<crate::value::ExpString>,
        pub min_segment_length: Option<i64>,
        pub mode: Option<crate::value::ExpString>,
        pub output_selection: Option<crate::value::ExpString>,
        pub program_date_time: Option<crate::value::ExpString>,
        pub program_date_time_clock: Option<crate::value::ExpString>,
        pub program_date_time_period: Option<i64>,
        pub redundant_manifest: Option<crate::value::ExpString>,
        pub segment_length: Option<i64>,
        pub segmentation_mode: Option<crate::value::ExpString>,
        pub segments_per_subdirectory: Option<i64>,
        pub stream_inf_resolution: Option<crate::value::ExpString>,
        pub timed_metadata_id3_frame: Option<crate::value::ExpString>,
        pub timed_metadata_id3_period: Option<i64>,
        pub timestamp_delta_milliseconds: Option<i64>,
        pub ts_file_mode: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_HlsGroupSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.HlsGroupSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_HlsGroupSettings as HlsGroupSettings;
    impl crate::value::ToValue for HlsGroupSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ad_markers {
                properties.insert(
                    "AdMarkers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.base_url_content {
                properties.insert(
                    "BaseUrlContent".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.base_url_content1 {
                properties.insert(
                    "BaseUrlContent1".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.base_url_manifest {
                properties.insert(
                    "BaseUrlManifest".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.base_url_manifest1 {
                properties.insert(
                    "BaseUrlManifest1".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.caption_language_mappings {
                properties.insert(
                    "CaptionLanguageMappings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.caption_language_setting {
                properties.insert(
                    "CaptionLanguageSetting".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.client_cache {
                properties.insert(
                    "ClientCache".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.codec_specification {
                properties.insert(
                    "CodecSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.constant_iv {
                properties.insert(
                    "ConstantIv".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.destination {
                properties.insert(
                    "Destination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.directory_structure {
                properties.insert(
                    "DirectoryStructure".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.discontinuity_tags {
                properties.insert(
                    "DiscontinuityTags".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.encryption_type {
                properties.insert(
                    "EncryptionType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hls_cdn_settings {
                properties.insert(
                    "HlsCdnSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hls_id3_segment_tagging {
                properties.insert(
                    "HlsId3SegmentTagging".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.i_frame_only_playlists {
                properties.insert(
                    "IFrameOnlyPlaylists".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.incomplete_segment_behavior {
                properties.insert(
                    "IncompleteSegmentBehavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.index_n_segments {
                properties.insert(
                    "IndexNSegments".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_loss_action {
                properties.insert(
                    "InputLossAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.iv_in_manifest {
                properties.insert(
                    "IvInManifest".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.iv_source {
                properties.insert(
                    "IvSource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.keep_segments {
                properties.insert(
                    "KeepSegments".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.key_format {
                properties.insert(
                    "KeyFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.key_format_versions {
                properties.insert(
                    "KeyFormatVersions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.key_provider_settings {
                properties.insert(
                    "KeyProviderSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.manifest_compression {
                properties.insert(
                    "ManifestCompression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.manifest_duration_format {
                properties.insert(
                    "ManifestDurationFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.min_segment_length {
                properties.insert(
                    "MinSegmentLength".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mode {
                properties.insert("Mode".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.output_selection {
                properties.insert(
                    "OutputSelection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.program_date_time {
                properties.insert(
                    "ProgramDateTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.program_date_time_clock {
                properties.insert(
                    "ProgramDateTimeClock".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.program_date_time_period {
                properties.insert(
                    "ProgramDateTimePeriod".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.redundant_manifest {
                properties.insert(
                    "RedundantManifest".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.segment_length {
                properties.insert(
                    "SegmentLength".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.segmentation_mode {
                properties.insert(
                    "SegmentationMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.segments_per_subdirectory {
                properties.insert(
                    "SegmentsPerSubdirectory".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stream_inf_resolution {
                properties.insert(
                    "StreamInfResolution".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timed_metadata_id3_frame {
                properties.insert(
                    "TimedMetadataId3Frame".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timed_metadata_id3_period {
                properties.insert(
                    "TimedMetadataId3Period".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timestamp_delta_milliseconds {
                properties.insert(
                    "TimestampDeltaMilliseconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ts_file_mode {
                properties.insert(
                    "TsFileMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsinputsettings.html
    pub struct HlsInputSettings_ {
        pub bandwidth: Option<i64>,
        pub buffer_segments: Option<i64>,
        pub retries: Option<i64>,
        pub retry_interval: Option<i64>,
        pub scte35_source: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_HlsInputSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.HlsInputSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_HlsInputSettings as HlsInputSettings;
    impl crate::value::ToValue for HlsInputSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bandwidth {
                properties.insert(
                    "Bandwidth".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.buffer_segments {
                properties.insert(
                    "BufferSegments".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.retries {
                properties.insert(
                    "Retries".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.retry_interval {
                properties.insert(
                    "RetryInterval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scte35_source {
                properties.insert(
                    "Scte35Source".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsmediastoresettings.html
    pub struct HlsMediaStoreSettings_ {
        pub connection_retry_interval: Option<i64>,
        pub filecache_duration: Option<i64>,
        pub media_store_storage_class: Option<crate::value::ExpString>,
        pub num_retries: Option<i64>,
        pub restart_delay: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_HlsMediaStoreSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.HlsMediaStoreSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_HlsMediaStoreSettings as HlsMediaStoreSettings;
    impl crate::value::ToValue for HlsMediaStoreSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.connection_retry_interval {
                properties.insert(
                    "ConnectionRetryInterval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.filecache_duration {
                properties.insert(
                    "FilecacheDuration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.media_store_storage_class {
                properties.insert(
                    "MediaStoreStorageClass".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.num_retries {
                properties.insert(
                    "NumRetries".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.restart_delay {
                properties.insert(
                    "RestartDelay".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsoutputsettings.html
    pub struct HlsOutputSettings_ {
        pub h265_packaging_type: Option<crate::value::ExpString>,
        pub hls_settings: Option<Box<HlsSettings_>>,
        pub name_modifier: Option<crate::value::ExpString>,
        pub segment_modifier: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_HlsOutputSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.HlsOutputSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_HlsOutputSettings as HlsOutputSettings;
    impl crate::value::ToValue for HlsOutputSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.h265_packaging_type {
                properties.insert(
                    "H265PackagingType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hls_settings {
                properties.insert(
                    "HlsSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name_modifier {
                properties.insert(
                    "NameModifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.segment_modifier {
                properties.insert(
                    "SegmentModifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlss3settings.html
    pub struct HlsS3Settings_ {
        pub canned_acl: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_HlsS3Settings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.HlsS3Settings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_HlsS3Settings as HlsS3Settings;
    impl crate::value::ToValue for HlsS3Settings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.canned_acl {
                properties.insert(
                    "CannedAcl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlssettings.html
    pub struct HlsSettings_ {
        pub audio_only_hls_settings: Option<Box<AudioOnlyHlsSettings_>>,
        pub fmp4_hls_settings: Option<Box<Fmp4HlsSettings_>>,
        pub frame_capture_hls_settings: Option<Box<FrameCaptureHlsSettings_>>,
        pub standard_hls_settings: Option<Box<StandardHlsSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_HlsSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.HlsSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_HlsSettings as HlsSettings;
    impl crate::value::ToValue for HlsSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.audio_only_hls_settings {
                properties.insert(
                    "AudioOnlyHlsSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.fmp4_hls_settings {
                properties.insert(
                    "Fmp4HlsSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.frame_capture_hls_settings {
                properties.insert(
                    "FrameCaptureHlsSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.standard_hls_settings {
                properties.insert(
                    "StandardHlsSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlswebdavsettings.html
    pub struct HlsWebdavSettings_ {
        pub connection_retry_interval: Option<i64>,
        pub filecache_duration: Option<i64>,
        pub http_transfer_mode: Option<crate::value::ExpString>,
        pub num_retries: Option<i64>,
        pub restart_delay: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_HlsWebdavSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.HlsWebdavSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_HlsWebdavSettings as HlsWebdavSettings;
    impl crate::value::ToValue for HlsWebdavSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.connection_retry_interval {
                properties.insert(
                    "ConnectionRetryInterval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.filecache_duration {
                properties.insert(
                    "FilecacheDuration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.http_transfer_mode {
                properties.insert(
                    "HttpTransferMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.num_retries {
                properties.insert(
                    "NumRetries".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.restart_delay {
                properties.insert(
                    "RestartDelay".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-htmlmotiongraphicssettings.html
    pub struct HtmlMotionGraphicsSettings_ {}
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_HtmlMotionGraphicsSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.HtmlMotionGraphicsSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_HtmlMotionGraphicsSettings as HtmlMotionGraphicsSettings;
    impl crate::value::ToValue for HtmlMotionGraphicsSettings_ {
        fn to_value(&self) -> serde_json::Value {
            serde_json::Value::Object(serde_json::Map::new())
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputattachment.html
    pub struct InputAttachment_ {
        pub automatic_input_failover_settings: Option<Box<AutomaticInputFailoverSettings_>>,
        pub input_attachment_name: Option<crate::value::ExpString>,
        pub input_id: Option<crate::value::ExpString>,
        pub input_settings: Option<Box<InputSettings_>>,
        pub logical_interface_names: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_InputAttachment {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.InputAttachment"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_InputAttachment as InputAttachment;
    impl crate::value::ToValue for InputAttachment_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.automatic_input_failover_settings {
                properties.insert(
                    "AutomaticInputFailoverSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_attachment_name {
                properties.insert(
                    "InputAttachmentName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_id {
                properties.insert(
                    "InputId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_settings {
                properties.insert(
                    "InputSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.logical_interface_names {
                properties.insert(
                    "LogicalInterfaceNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputchannellevel.html
    pub struct InputChannelLevel_ {
        pub gain: Option<i64>,
        pub input_channel: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_InputChannelLevel {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.InputChannelLevel"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_InputChannelLevel as InputChannelLevel;
    impl crate::value::ToValue for InputChannelLevel_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.gain {
                properties.insert("Gain".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.input_channel {
                properties.insert(
                    "InputChannel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputlocation.html
    pub struct InputLocation_ {
        pub password_param: Option<crate::value::ExpString>,
        pub uri: Option<crate::value::ExpString>,
        pub username: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_InputLocation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.InputLocation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_InputLocation as InputLocation;
    impl crate::value::ToValue for InputLocation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.password_param {
                properties.insert(
                    "PasswordParam".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.uri {
                properties.insert("Uri".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.username {
                properties.insert(
                    "Username".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputlossbehavior.html
    pub struct InputLossBehavior_ {
        pub black_frame_msec: Option<i64>,
        pub input_loss_image_color: Option<crate::value::ExpString>,
        pub input_loss_image_slate: Option<Box<InputLocation_>>,
        pub input_loss_image_type: Option<crate::value::ExpString>,
        pub repeat_frame_msec: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_InputLossBehavior {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.InputLossBehavior"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_InputLossBehavior as InputLossBehavior;
    impl crate::value::ToValue for InputLossBehavior_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.black_frame_msec {
                properties.insert(
                    "BlackFrameMsec".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_loss_image_color {
                properties.insert(
                    "InputLossImageColor".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_loss_image_slate {
                properties.insert(
                    "InputLossImageSlate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_loss_image_type {
                properties.insert(
                    "InputLossImageType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.repeat_frame_msec {
                properties.insert(
                    "RepeatFrameMsec".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputlossfailoversettings.html
    pub struct InputLossFailoverSettings_ {
        pub input_loss_threshold_msec: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_InputLossFailoverSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.InputLossFailoverSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_InputLossFailoverSettings as InputLossFailoverSettings;
    impl crate::value::ToValue for InputLossFailoverSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.input_loss_threshold_msec {
                properties.insert(
                    "InputLossThresholdMsec".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputsettings.html
    pub struct InputSettings_ {
        pub audio_selectors: Option<Vec<AudioSelector_>>,
        pub caption_selectors: Option<Vec<CaptionSelector_>>,
        pub deblock_filter: Option<crate::value::ExpString>,
        pub denoise_filter: Option<crate::value::ExpString>,
        pub filter_strength: Option<i64>,
        pub input_filter: Option<crate::value::ExpString>,
        pub network_input_settings: Option<Box<NetworkInputSettings_>>,
        pub scte35_pid: Option<i64>,
        pub smpte2038_data_preference: Option<crate::value::ExpString>,
        pub source_end_behavior: Option<crate::value::ExpString>,
        pub video_selector: Option<Box<VideoSelector_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_InputSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.InputSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_InputSettings as InputSettings;
    impl crate::value::ToValue for InputSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.audio_selectors {
                properties.insert(
                    "AudioSelectors".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.caption_selectors {
                properties.insert(
                    "CaptionSelectors".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.deblock_filter {
                properties.insert(
                    "DeblockFilter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.denoise_filter {
                properties.insert(
                    "DenoiseFilter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.filter_strength {
                properties.insert(
                    "FilterStrength".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_filter {
                properties.insert(
                    "InputFilter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_input_settings {
                properties.insert(
                    "NetworkInputSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scte35_pid {
                properties.insert(
                    "Scte35Pid".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.smpte2038_data_preference {
                properties.insert(
                    "Smpte2038DataPreference".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_end_behavior {
                properties.insert(
                    "SourceEndBehavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.video_selector {
                properties.insert(
                    "VideoSelector".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputspecification.html
    pub struct InputSpecification_ {
        pub codec: Option<crate::value::ExpString>,
        pub maximum_bitrate: Option<crate::value::ExpString>,
        pub resolution: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_InputSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.InputSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_InputSpecification as InputSpecification;
    impl crate::value::ToValue for InputSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.codec {
                properties.insert("Codec".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.maximum_bitrate {
                properties.insert(
                    "MaximumBitrate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resolution {
                properties.insert(
                    "Resolution".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-keyprovidersettings.html
    pub struct KeyProviderSettings_ {
        pub static_key_settings: Option<Box<StaticKeySettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_KeyProviderSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.KeyProviderSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_KeyProviderSettings as KeyProviderSettings;
    impl crate::value::ToValue for KeyProviderSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.static_key_settings {
                properties.insert(
                    "StaticKeySettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html
    pub struct M2tsSettings_ {
        pub absent_input_audio_behavior: Option<crate::value::ExpString>,
        pub arib: Option<crate::value::ExpString>,
        pub arib_captions_pid: Option<crate::value::ExpString>,
        pub arib_captions_pid_control: Option<crate::value::ExpString>,
        pub audio_buffer_model: Option<crate::value::ExpString>,
        pub audio_frames_per_pes: Option<i64>,
        pub audio_pids: Option<crate::value::ExpString>,
        pub audio_stream_type: Option<crate::value::ExpString>,
        pub bitrate: Option<i64>,
        pub buffer_model: Option<crate::value::ExpString>,
        pub cc_descriptor: Option<crate::value::ExpString>,
        pub dvb_nit_settings: Option<Box<DvbNitSettings_>>,
        pub dvb_sdt_settings: Option<Box<DvbSdtSettings_>>,
        pub dvb_sub_pids: Option<crate::value::ExpString>,
        pub dvb_tdt_settings: Option<Box<DvbTdtSettings_>>,
        pub dvb_teletext_pid: Option<crate::value::ExpString>,
        pub ebif: Option<crate::value::ExpString>,
        pub ebp_audio_interval: Option<crate::value::ExpString>,
        pub ebp_lookahead_ms: Option<i64>,
        pub ebp_placement: Option<crate::value::ExpString>,
        pub ecm_pid: Option<crate::value::ExpString>,
        pub es_rate_in_pes: Option<crate::value::ExpString>,
        pub etv_platform_pid: Option<crate::value::ExpString>,
        pub etv_signal_pid: Option<crate::value::ExpString>,
        pub fragment_time: Option<f64>,
        pub klv: Option<crate::value::ExpString>,
        pub klv_data_pids: Option<crate::value::ExpString>,
        pub nielsen_id3_behavior: Option<crate::value::ExpString>,
        pub null_packet_bitrate: Option<f64>,
        pub pat_interval: Option<i64>,
        pub pcr_control: Option<crate::value::ExpString>,
        pub pcr_period: Option<i64>,
        pub pcr_pid: Option<crate::value::ExpString>,
        pub pmt_interval: Option<i64>,
        pub pmt_pid: Option<crate::value::ExpString>,
        pub program_num: Option<i64>,
        pub rate_mode: Option<crate::value::ExpString>,
        pub scte27_pids: Option<crate::value::ExpString>,
        pub scte35_control: Option<crate::value::ExpString>,
        pub scte35_pid: Option<crate::value::ExpString>,
        pub scte35_preroll_pullup_milliseconds: Option<f64>,
        pub segmentation_markers: Option<crate::value::ExpString>,
        pub segmentation_style: Option<crate::value::ExpString>,
        pub segmentation_time: Option<f64>,
        pub timed_metadata_behavior: Option<crate::value::ExpString>,
        pub timed_metadata_pid: Option<crate::value::ExpString>,
        pub transport_stream_id: Option<i64>,
        pub video_pid: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_M2tsSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.M2tsSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_M2tsSettings as M2tsSettings;
    impl crate::value::ToValue for M2tsSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.absent_input_audio_behavior {
                properties.insert(
                    "AbsentInputAudioBehavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.arib {
                properties.insert("Arib".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.arib_captions_pid {
                properties.insert(
                    "AribCaptionsPid".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.arib_captions_pid_control {
                properties.insert(
                    "AribCaptionsPidControl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.audio_buffer_model {
                properties.insert(
                    "AudioBufferModel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.audio_frames_per_pes {
                properties.insert(
                    "AudioFramesPerPes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.audio_pids {
                properties.insert(
                    "AudioPids".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.audio_stream_type {
                properties.insert(
                    "AudioStreamType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bitrate {
                properties.insert(
                    "Bitrate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.buffer_model {
                properties.insert(
                    "BufferModel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cc_descriptor {
                properties.insert(
                    "CcDescriptor".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dvb_nit_settings {
                properties.insert(
                    "DvbNitSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dvb_sdt_settings {
                properties.insert(
                    "DvbSdtSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dvb_sub_pids {
                properties.insert(
                    "DvbSubPids".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dvb_tdt_settings {
                properties.insert(
                    "DvbTdtSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dvb_teletext_pid {
                properties.insert(
                    "DvbTeletextPid".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ebif {
                properties.insert("Ebif".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.ebp_audio_interval {
                properties.insert(
                    "EbpAudioInterval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ebp_lookahead_ms {
                properties.insert(
                    "EbpLookaheadMs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ebp_placement {
                properties.insert(
                    "EbpPlacement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ecm_pid {
                properties.insert("EcmPid".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.es_rate_in_pes {
                properties.insert(
                    "EsRateInPes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.etv_platform_pid {
                properties.insert(
                    "EtvPlatformPid".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.etv_signal_pid {
                properties.insert(
                    "EtvSignalPid".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.fragment_time {
                properties.insert(
                    "FragmentTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.klv {
                properties.insert("Klv".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.klv_data_pids {
                properties.insert(
                    "KlvDataPids".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.nielsen_id3_behavior {
                properties.insert(
                    "NielsenId3Behavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.null_packet_bitrate {
                properties.insert(
                    "NullPacketBitrate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pat_interval {
                properties.insert(
                    "PatInterval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pcr_control {
                properties.insert(
                    "PcrControl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pcr_period {
                properties.insert(
                    "PcrPeriod".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pcr_pid {
                properties.insert("PcrPid".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.pmt_interval {
                properties.insert(
                    "PmtInterval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pmt_pid {
                properties.insert("PmtPid".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.program_num {
                properties.insert(
                    "ProgramNum".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rate_mode {
                properties.insert(
                    "RateMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scte27_pids {
                properties.insert(
                    "Scte27Pids".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scte35_control {
                properties.insert(
                    "Scte35Control".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scte35_pid {
                properties.insert(
                    "Scte35Pid".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scte35_preroll_pullup_milliseconds {
                properties.insert(
                    "Scte35PrerollPullupMilliseconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.segmentation_markers {
                properties.insert(
                    "SegmentationMarkers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.segmentation_style {
                properties.insert(
                    "SegmentationStyle".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.segmentation_time {
                properties.insert(
                    "SegmentationTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timed_metadata_behavior {
                properties.insert(
                    "TimedMetadataBehavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timed_metadata_pid {
                properties.insert(
                    "TimedMetadataPid".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.transport_stream_id {
                properties.insert(
                    "TransportStreamId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.video_pid {
                properties.insert(
                    "VideoPid".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m3u8settings.html
    pub struct M3u8Settings_ {
        pub audio_frames_per_pes: Option<i64>,
        pub audio_pids: Option<crate::value::ExpString>,
        pub ecm_pid: Option<crate::value::ExpString>,
        pub klv_behavior: Option<crate::value::ExpString>,
        pub klv_data_pids: Option<crate::value::ExpString>,
        pub nielsen_id3_behavior: Option<crate::value::ExpString>,
        pub pat_interval: Option<i64>,
        pub pcr_control: Option<crate::value::ExpString>,
        pub pcr_period: Option<i64>,
        pub pcr_pid: Option<crate::value::ExpString>,
        pub pmt_interval: Option<i64>,
        pub pmt_pid: Option<crate::value::ExpString>,
        pub program_num: Option<i64>,
        pub scte35_behavior: Option<crate::value::ExpString>,
        pub scte35_pid: Option<crate::value::ExpString>,
        pub timed_metadata_behavior: Option<crate::value::ExpString>,
        pub timed_metadata_pid: Option<crate::value::ExpString>,
        pub transport_stream_id: Option<i64>,
        pub video_pid: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_M3u8Settings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.M3u8Settings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_M3u8Settings as M3u8Settings;
    impl crate::value::ToValue for M3u8Settings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.audio_frames_per_pes {
                properties.insert(
                    "AudioFramesPerPes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.audio_pids {
                properties.insert(
                    "AudioPids".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ecm_pid {
                properties.insert("EcmPid".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.klv_behavior {
                properties.insert(
                    "KlvBehavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.klv_data_pids {
                properties.insert(
                    "KlvDataPids".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.nielsen_id3_behavior {
                properties.insert(
                    "NielsenId3Behavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pat_interval {
                properties.insert(
                    "PatInterval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pcr_control {
                properties.insert(
                    "PcrControl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pcr_period {
                properties.insert(
                    "PcrPeriod".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pcr_pid {
                properties.insert("PcrPid".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.pmt_interval {
                properties.insert(
                    "PmtInterval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pmt_pid {
                properties.insert("PmtPid".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.program_num {
                properties.insert(
                    "ProgramNum".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scte35_behavior {
                properties.insert(
                    "Scte35Behavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scte35_pid {
                properties.insert(
                    "Scte35Pid".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timed_metadata_behavior {
                properties.insert(
                    "TimedMetadataBehavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timed_metadata_pid {
                properties.insert(
                    "TimedMetadataPid".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.transport_stream_id {
                properties.insert(
                    "TransportStreamId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.video_pid {
                properties.insert(
                    "VideoPid".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-maintenancecreatesettings.html
    pub struct MaintenanceCreateSettings_ {
        pub maintenance_day: Option<crate::value::ExpString>,
        pub maintenance_start_time: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_MaintenanceCreateSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.MaintenanceCreateSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_MaintenanceCreateSettings as MaintenanceCreateSettings;
    impl crate::value::ToValue for MaintenanceCreateSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.maintenance_day {
                properties.insert(
                    "MaintenanceDay".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.maintenance_start_time {
                properties.insert(
                    "MaintenanceStartTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-maintenanceupdatesettings.html
    pub struct MaintenanceUpdateSettings_ {
        pub maintenance_day: Option<crate::value::ExpString>,
        pub maintenance_scheduled_date: Option<crate::value::ExpString>,
        pub maintenance_start_time: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_MaintenanceUpdateSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.MaintenanceUpdateSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_MaintenanceUpdateSettings as MaintenanceUpdateSettings;
    impl crate::value::ToValue for MaintenanceUpdateSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.maintenance_day {
                properties.insert(
                    "MaintenanceDay".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.maintenance_scheduled_date {
                properties.insert(
                    "MaintenanceScheduledDate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.maintenance_start_time {
                properties.insert(
                    "MaintenanceStartTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mediapackagegroupsettings.html
    pub struct MediaPackageGroupSettings_ {
        pub destination: Option<Box<OutputLocationRef_>>,
        pub mediapackage_v2_group_settings: Option<Box<MediaPackageV2GroupSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_MediaPackageGroupSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.MediaPackageGroupSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_MediaPackageGroupSettings as MediaPackageGroupSettings;
    impl crate::value::ToValue for MediaPackageGroupSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.destination {
                properties.insert(
                    "Destination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mediapackage_v2_group_settings {
                properties.insert(
                    "MediapackageV2GroupSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mediapackageoutputdestinationsettings.html
    pub struct MediaPackageOutputDestinationSettings_ {
        pub channel_group: Option<crate::value::ExpString>,
        pub channel_id: Option<crate::value::ExpString>,
        pub channel_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_MediaPackageOutputDestinationSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.MediaPackageOutputDestinationSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_MediaPackageOutputDestinationSettings as MediaPackageOutputDestinationSettings;
    impl crate::value::ToValue for MediaPackageOutputDestinationSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.channel_group {
                properties.insert(
                    "ChannelGroup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.channel_id {
                properties.insert(
                    "ChannelId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.channel_name {
                properties.insert(
                    "ChannelName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mediapackageoutputsettings.html
    pub struct MediaPackageOutputSettings_ {
        pub media_package_v2_destination_settings: Option<Box<MediaPackageV2DestinationSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_MediaPackageOutputSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.MediaPackageOutputSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_MediaPackageOutputSettings as MediaPackageOutputSettings;
    impl crate::value::ToValue for MediaPackageOutputSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.media_package_v2_destination_settings {
                properties.insert(
                    "MediaPackageV2DestinationSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mediapackagev2destinationsettings.html
    pub struct MediaPackageV2DestinationSettings_ {
        pub audio_group_id: Option<crate::value::ExpString>,
        pub audio_rendition_sets: Option<crate::value::ExpString>,
        pub hls_auto_select: Option<crate::value::ExpString>,
        pub hls_default: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_MediaPackageV2DestinationSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.MediaPackageV2DestinationSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_MediaPackageV2DestinationSettings as MediaPackageV2DestinationSettings;
    impl crate::value::ToValue for MediaPackageV2DestinationSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.audio_group_id {
                properties.insert(
                    "AudioGroupId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.audio_rendition_sets {
                properties.insert(
                    "AudioRenditionSets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hls_auto_select {
                properties.insert(
                    "HlsAutoSelect".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hls_default {
                properties.insert(
                    "HlsDefault".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mediapackagev2groupsettings.html
    pub struct MediaPackageV2GroupSettings_ {
        pub caption_language_mappings: Option<Vec<CaptionLanguageMapping_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_MediaPackageV2GroupSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.MediaPackageV2GroupSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_MediaPackageV2GroupSettings as MediaPackageV2GroupSettings;
    impl crate::value::ToValue for MediaPackageV2GroupSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.caption_language_mappings {
                properties.insert(
                    "CaptionLanguageMappings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-motiongraphicsconfiguration.html
    pub struct MotionGraphicsConfiguration_ {
        pub motion_graphics_insertion: Option<crate::value::ExpString>,
        pub motion_graphics_settings: Option<Box<MotionGraphicsSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_MotionGraphicsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.MotionGraphicsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_MotionGraphicsConfiguration as MotionGraphicsConfiguration;
    impl crate::value::ToValue for MotionGraphicsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.motion_graphics_insertion {
                properties.insert(
                    "MotionGraphicsInsertion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.motion_graphics_settings {
                properties.insert(
                    "MotionGraphicsSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-motiongraphicssettings.html
    pub struct MotionGraphicsSettings_ {
        pub html_motion_graphics_settings: Option<Box<HtmlMotionGraphicsSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_MotionGraphicsSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.MotionGraphicsSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_MotionGraphicsSettings as MotionGraphicsSettings;
    impl crate::value::ToValue for MotionGraphicsSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.html_motion_graphics_settings {
                properties.insert(
                    "HtmlMotionGraphicsSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mp2settings.html
    pub struct Mp2Settings_ {
        pub bitrate: Option<f64>,
        pub coding_mode: Option<crate::value::ExpString>,
        pub sample_rate: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_Mp2Settings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.Mp2Settings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_Mp2Settings as Mp2Settings;
    impl crate::value::ToValue for Mp2Settings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bitrate {
                properties.insert(
                    "Bitrate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.coding_mode {
                properties.insert(
                    "CodingMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sample_rate {
                properties.insert(
                    "SampleRate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mpeg2filtersettings.html
    pub struct Mpeg2FilterSettings_ {
        pub temporal_filter_settings: Option<Box<TemporalFilterSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_Mpeg2FilterSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.Mpeg2FilterSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_Mpeg2FilterSettings as Mpeg2FilterSettings;
    impl crate::value::ToValue for Mpeg2FilterSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.temporal_filter_settings {
                properties.insert(
                    "TemporalFilterSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mpeg2settings.html
    pub struct Mpeg2Settings_ {
        pub adaptive_quantization: Option<crate::value::ExpString>,
        pub afd_signaling: Option<crate::value::ExpString>,
        pub color_metadata: Option<crate::value::ExpString>,
        pub color_space: Option<crate::value::ExpString>,
        pub display_aspect_ratio: Option<crate::value::ExpString>,
        pub filter_settings: Option<Box<Mpeg2FilterSettings_>>,
        pub fixed_afd: Option<crate::value::ExpString>,
        pub framerate_denominator: Option<i64>,
        pub framerate_numerator: Option<i64>,
        pub gop_closed_cadence: Option<i64>,
        pub gop_num_b_frames: Option<i64>,
        pub gop_size: Option<f64>,
        pub gop_size_units: Option<crate::value::ExpString>,
        pub scan_type: Option<crate::value::ExpString>,
        pub subgop_length: Option<crate::value::ExpString>,
        pub timecode_burnin_settings: Option<Box<TimecodeBurninSettings_>>,
        pub timecode_insertion: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_Mpeg2Settings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.Mpeg2Settings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_Mpeg2Settings as Mpeg2Settings;
    impl crate::value::ToValue for Mpeg2Settings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.adaptive_quantization {
                properties.insert(
                    "AdaptiveQuantization".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.afd_signaling {
                properties.insert(
                    "AfdSignaling".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.color_metadata {
                properties.insert(
                    "ColorMetadata".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.color_space {
                properties.insert(
                    "ColorSpace".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.display_aspect_ratio {
                properties.insert(
                    "DisplayAspectRatio".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.filter_settings {
                properties.insert(
                    "FilterSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.fixed_afd {
                properties.insert(
                    "FixedAfd".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.framerate_denominator {
                properties.insert(
                    "FramerateDenominator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.framerate_numerator {
                properties.insert(
                    "FramerateNumerator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.gop_closed_cadence {
                properties.insert(
                    "GopClosedCadence".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.gop_num_b_frames {
                properties.insert(
                    "GopNumBFrames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.gop_size {
                properties.insert(
                    "GopSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.gop_size_units {
                properties.insert(
                    "GopSizeUnits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scan_type {
                properties.insert(
                    "ScanType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.subgop_length {
                properties.insert(
                    "SubgopLength".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timecode_burnin_settings {
                properties.insert(
                    "TimecodeBurninSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timecode_insertion {
                properties.insert(
                    "TimecodeInsertion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mssmoothgroupsettings.html
    pub struct MsSmoothGroupSettings_ {
        pub acquisition_point_id: Option<crate::value::ExpString>,
        pub audio_only_timecode_control: Option<crate::value::ExpString>,
        pub certificate_mode: Option<crate::value::ExpString>,
        pub connection_retry_interval: Option<i64>,
        pub destination: Option<Box<OutputLocationRef_>>,
        pub event_id: Option<crate::value::ExpString>,
        pub event_id_mode: Option<crate::value::ExpString>,
        pub event_stop_behavior: Option<crate::value::ExpString>,
        pub filecache_duration: Option<i64>,
        pub fragment_length: Option<i64>,
        pub input_loss_action: Option<crate::value::ExpString>,
        pub num_retries: Option<i64>,
        pub restart_delay: Option<i64>,
        pub segmentation_mode: Option<crate::value::ExpString>,
        pub send_delay_ms: Option<i64>,
        pub sparse_track_type: Option<crate::value::ExpString>,
        pub stream_manifest_behavior: Option<crate::value::ExpString>,
        pub timestamp_offset: Option<crate::value::ExpString>,
        pub timestamp_offset_mode: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_MsSmoothGroupSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.MsSmoothGroupSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_MsSmoothGroupSettings as MsSmoothGroupSettings;
    impl crate::value::ToValue for MsSmoothGroupSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.acquisition_point_id {
                properties.insert(
                    "AcquisitionPointId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.audio_only_timecode_control {
                properties.insert(
                    "AudioOnlyTimecodeControl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.certificate_mode {
                properties.insert(
                    "CertificateMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.connection_retry_interval {
                properties.insert(
                    "ConnectionRetryInterval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.destination {
                properties.insert(
                    "Destination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.event_id {
                properties.insert(
                    "EventId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.event_id_mode {
                properties.insert(
                    "EventIdMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.event_stop_behavior {
                properties.insert(
                    "EventStopBehavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.filecache_duration {
                properties.insert(
                    "FilecacheDuration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.fragment_length {
                properties.insert(
                    "FragmentLength".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_loss_action {
                properties.insert(
                    "InputLossAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.num_retries {
                properties.insert(
                    "NumRetries".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.restart_delay {
                properties.insert(
                    "RestartDelay".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.segmentation_mode {
                properties.insert(
                    "SegmentationMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.send_delay_ms {
                properties.insert(
                    "SendDelayMs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sparse_track_type {
                properties.insert(
                    "SparseTrackType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stream_manifest_behavior {
                properties.insert(
                    "StreamManifestBehavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timestamp_offset {
                properties.insert(
                    "TimestampOffset".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timestamp_offset_mode {
                properties.insert(
                    "TimestampOffsetMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mssmoothoutputsettings.html
    pub struct MsSmoothOutputSettings_ {
        pub h265_packaging_type: Option<crate::value::ExpString>,
        pub name_modifier: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_MsSmoothOutputSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.MsSmoothOutputSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_MsSmoothOutputSettings as MsSmoothOutputSettings;
    impl crate::value::ToValue for MsSmoothOutputSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.h265_packaging_type {
                properties.insert(
                    "H265PackagingType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name_modifier {
                properties.insert(
                    "NameModifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-multicastinputsettings.html
    pub struct MulticastInputSettings_ {
        pub source_ip_address: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_MulticastInputSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.MulticastInputSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_MulticastInputSettings as MulticastInputSettings;
    impl crate::value::ToValue for MulticastInputSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.source_ip_address {
                properties.insert(
                    "SourceIpAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-multiplexcontainersettings.html
    pub struct MultiplexContainerSettings_ {
        pub multiplex_m2ts_settings: Option<Box<MultiplexM2tsSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_MultiplexContainerSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.MultiplexContainerSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_MultiplexContainerSettings as MultiplexContainerSettings;
    impl crate::value::ToValue for MultiplexContainerSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.multiplex_m2ts_settings {
                properties.insert(
                    "MultiplexM2tsSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-multiplexgroupsettings.html
    pub struct MultiplexGroupSettings_ {}
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_MultiplexGroupSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.MultiplexGroupSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_MultiplexGroupSettings as MultiplexGroupSettings;
    impl crate::value::ToValue for MultiplexGroupSettings_ {
        fn to_value(&self) -> serde_json::Value {
            serde_json::Value::Object(serde_json::Map::new())
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-multiplexm2tssettings.html
    pub struct MultiplexM2tsSettings_ {
        pub absent_input_audio_behavior: Option<crate::value::ExpString>,
        pub arib: Option<crate::value::ExpString>,
        pub audio_buffer_model: Option<crate::value::ExpString>,
        pub audio_frames_per_pes: Option<i64>,
        pub audio_stream_type: Option<crate::value::ExpString>,
        pub cc_descriptor: Option<crate::value::ExpString>,
        pub ebif: Option<crate::value::ExpString>,
        pub es_rate_in_pes: Option<crate::value::ExpString>,
        pub klv: Option<crate::value::ExpString>,
        pub nielsen_id3_behavior: Option<crate::value::ExpString>,
        pub pcr_control: Option<crate::value::ExpString>,
        pub pcr_period: Option<i64>,
        pub scte35_control: Option<crate::value::ExpString>,
        pub scte35_preroll_pullup_milliseconds: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_MultiplexM2tsSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.MultiplexM2tsSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_MultiplexM2tsSettings as MultiplexM2tsSettings;
    impl crate::value::ToValue for MultiplexM2tsSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.absent_input_audio_behavior {
                properties.insert(
                    "AbsentInputAudioBehavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.arib {
                properties.insert("Arib".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.audio_buffer_model {
                properties.insert(
                    "AudioBufferModel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.audio_frames_per_pes {
                properties.insert(
                    "AudioFramesPerPes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.audio_stream_type {
                properties.insert(
                    "AudioStreamType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cc_descriptor {
                properties.insert(
                    "CcDescriptor".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ebif {
                properties.insert("Ebif".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.es_rate_in_pes {
                properties.insert(
                    "EsRateInPes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.klv {
                properties.insert("Klv".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.nielsen_id3_behavior {
                properties.insert(
                    "NielsenId3Behavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pcr_control {
                properties.insert(
                    "PcrControl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pcr_period {
                properties.insert(
                    "PcrPeriod".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scte35_control {
                properties.insert(
                    "Scte35Control".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scte35_preroll_pullup_milliseconds {
                properties.insert(
                    "Scte35PrerollPullupMilliseconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-multiplexoutputsettings.html
    pub struct MultiplexOutputSettings_ {
        pub container_settings: Option<Box<MultiplexContainerSettings_>>,
        pub destination: Option<Box<OutputLocationRef_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_MultiplexOutputSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.MultiplexOutputSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_MultiplexOutputSettings as MultiplexOutputSettings;
    impl crate::value::ToValue for MultiplexOutputSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.container_settings {
                properties.insert(
                    "ContainerSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.destination {
                properties.insert(
                    "Destination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-multiplexprogramchanneldestinationsettings.html
    pub struct MultiplexProgramChannelDestinationSettings_ {
        pub multiplex_id: Option<crate::value::ExpString>,
        pub program_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_MultiplexProgramChannelDestinationSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.MultiplexProgramChannelDestinationSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_MultiplexProgramChannelDestinationSettings as MultiplexProgramChannelDestinationSettings;
    impl crate::value::ToValue for MultiplexProgramChannelDestinationSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.multiplex_id {
                properties.insert(
                    "MultiplexId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.program_name {
                properties.insert(
                    "ProgramName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-networkinputsettings.html
    pub struct NetworkInputSettings_ {
        pub hls_input_settings: Option<Box<HlsInputSettings_>>,
        pub multicast_input_settings: Option<Box<MulticastInputSettings_>>,
        pub server_validation: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_NetworkInputSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.NetworkInputSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_NetworkInputSettings as NetworkInputSettings;
    impl crate::value::ToValue for NetworkInputSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.hls_input_settings {
                properties.insert(
                    "HlsInputSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.multicast_input_settings {
                properties.insert(
                    "MulticastInputSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.server_validation {
                properties.insert(
                    "ServerValidation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-nielsencbet.html
    pub struct NielsenCBET_ {
        pub cbet_check_digit_string: Option<crate::value::ExpString>,
        pub cbet_stepaside: Option<crate::value::ExpString>,
        pub csid: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_NielsenCBET {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.NielsenCBET"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_NielsenCBET as NielsenCBET;
    impl crate::value::ToValue for NielsenCBET_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cbet_check_digit_string {
                properties.insert(
                    "CbetCheckDigitString".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cbet_stepaside {
                properties.insert(
                    "CbetStepaside".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.csid {
                properties.insert("Csid".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-nielsenconfiguration.html
    pub struct NielsenConfiguration_ {
        pub distributor_id: Option<crate::value::ExpString>,
        pub nielsen_pcm_to_id3_tagging: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_NielsenConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.NielsenConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_NielsenConfiguration as NielsenConfiguration;
    impl crate::value::ToValue for NielsenConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.distributor_id {
                properties.insert(
                    "DistributorId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.nielsen_pcm_to_id3_tagging {
                properties.insert(
                    "NielsenPcmToId3Tagging".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-nielsennaesiinw.html
    pub struct NielsenNaesIiNw_ {
        pub check_digit_string: Option<crate::value::ExpString>,
        pub sid: Option<f64>,
        pub timezone: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_NielsenNaesIiNw {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.NielsenNaesIiNw"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_NielsenNaesIiNw as NielsenNaesIiNw;
    impl crate::value::ToValue for NielsenNaesIiNw_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.check_digit_string {
                properties.insert(
                    "CheckDigitString".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sid {
                properties.insert("Sid".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.timezone {
                properties.insert(
                    "Timezone".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-nielsenwatermarkssettings.html
    pub struct NielsenWatermarksSettings_ {
        pub nielsen_cbet_settings: Option<Box<NielsenCBET_>>,
        pub nielsen_distribution_type: Option<crate::value::ExpString>,
        pub nielsen_naes_ii_nw_settings: Option<Box<NielsenNaesIiNw_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_NielsenWatermarksSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.NielsenWatermarksSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_NielsenWatermarksSettings as NielsenWatermarksSettings;
    impl crate::value::ToValue for NielsenWatermarksSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.nielsen_cbet_settings {
                properties.insert(
                    "NielsenCbetSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.nielsen_distribution_type {
                properties.insert(
                    "NielsenDistributionType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.nielsen_naes_ii_nw_settings {
                properties.insert(
                    "NielsenNaesIiNwSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-output.html
    pub struct Output_ {
        pub audio_description_names: Option<Vec<crate::value::ExpString>>,
        pub caption_description_names: Option<Vec<crate::value::ExpString>>,
        pub output_name: Option<crate::value::ExpString>,
        pub output_settings: Option<Box<OutputSettings_>>,
        pub video_description_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_Output {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.Output"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_Output as Output;
    impl crate::value::ToValue for Output_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.audio_description_names {
                properties.insert(
                    "AudioDescriptionNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.caption_description_names {
                properties.insert(
                    "CaptionDescriptionNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.output_name {
                properties.insert(
                    "OutputName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.output_settings {
                properties.insert(
                    "OutputSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.video_description_name {
                properties.insert(
                    "VideoDescriptionName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputdestination.html
    pub struct OutputDestination_ {
        pub id: Option<crate::value::ExpString>,
        pub logical_interface_names: Option<Vec<crate::value::ExpString>>,
        pub media_package_settings: Option<Vec<MediaPackageOutputDestinationSettings_>>,
        pub multiplex_settings: Option<Box<MultiplexProgramChannelDestinationSettings_>>,
        pub settings: Option<Vec<OutputDestinationSettings_>>,
        pub srt_settings: Option<Vec<SrtOutputDestinationSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_OutputDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.OutputDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_OutputDestination as OutputDestination;
    impl crate::value::ToValue for OutputDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.id {
                properties.insert("Id".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.logical_interface_names {
                properties.insert(
                    "LogicalInterfaceNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.media_package_settings {
                properties.insert(
                    "MediaPackageSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.multiplex_settings {
                properties.insert(
                    "MultiplexSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.settings {
                properties.insert(
                    "Settings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.srt_settings {
                properties.insert(
                    "SrtSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputdestinationsettings.html
    pub struct OutputDestinationSettings_ {
        pub password_param: Option<crate::value::ExpString>,
        pub stream_name: Option<crate::value::ExpString>,
        pub url: Option<crate::value::ExpString>,
        pub username: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_OutputDestinationSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.OutputDestinationSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_OutputDestinationSettings as OutputDestinationSettings;
    impl crate::value::ToValue for OutputDestinationSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.password_param {
                properties.insert(
                    "PasswordParam".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stream_name {
                properties.insert(
                    "StreamName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.url {
                properties.insert("Url".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.username {
                properties.insert(
                    "Username".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputgroup.html
    pub struct OutputGroup_ {
        pub name: Option<crate::value::ExpString>,
        pub output_group_settings: Option<Box<OutputGroupSettings_>>,
        pub outputs: Option<Vec<Output_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_OutputGroup {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.OutputGroup"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_OutputGroup as OutputGroup;
    impl crate::value::ToValue for OutputGroup_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.output_group_settings {
                properties.insert(
                    "OutputGroupSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.outputs {
                properties.insert(
                    "Outputs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputgroupsettings.html
    pub struct OutputGroupSettings_ {
        pub archive_group_settings: Option<Box<ArchiveGroupSettings_>>,
        pub cmaf_ingest_group_settings: Option<Box<CmafIngestGroupSettings_>>,
        pub frame_capture_group_settings: Option<Box<FrameCaptureGroupSettings_>>,
        pub hls_group_settings: Option<Box<HlsGroupSettings_>>,
        pub media_package_group_settings: Option<Box<MediaPackageGroupSettings_>>,
        pub ms_smooth_group_settings: Option<Box<MsSmoothGroupSettings_>>,
        pub multiplex_group_settings: Option<Box<MultiplexGroupSettings_>>,
        pub rtmp_group_settings: Option<Box<RtmpGroupSettings_>>,
        pub srt_group_settings: Option<Box<SrtGroupSettings_>>,
        pub udp_group_settings: Option<Box<UdpGroupSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_OutputGroupSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.OutputGroupSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_OutputGroupSettings as OutputGroupSettings;
    impl crate::value::ToValue for OutputGroupSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.archive_group_settings {
                properties.insert(
                    "ArchiveGroupSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cmaf_ingest_group_settings {
                properties.insert(
                    "CmafIngestGroupSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.frame_capture_group_settings {
                properties.insert(
                    "FrameCaptureGroupSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hls_group_settings {
                properties.insert(
                    "HlsGroupSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.media_package_group_settings {
                properties.insert(
                    "MediaPackageGroupSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ms_smooth_group_settings {
                properties.insert(
                    "MsSmoothGroupSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.multiplex_group_settings {
                properties.insert(
                    "MultiplexGroupSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rtmp_group_settings {
                properties.insert(
                    "RtmpGroupSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.srt_group_settings {
                properties.insert(
                    "SrtGroupSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.udp_group_settings {
                properties.insert(
                    "UdpGroupSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputlocationref.html
    pub struct OutputLocationRef_ {
        pub destination_ref_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_OutputLocationRef {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.OutputLocationRef"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_OutputLocationRef as OutputLocationRef;
    impl crate::value::ToValue for OutputLocationRef_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.destination_ref_id {
                properties.insert(
                    "DestinationRefId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputlockingsettings.html
    pub struct OutputLockingSettings_ {
        pub epoch_locking_settings: Option<Box<EpochLockingSettings_>>,
        pub pipeline_locking_settings: Option<Box<PipelineLockingSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_OutputLockingSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.OutputLockingSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_OutputLockingSettings as OutputLockingSettings;
    impl crate::value::ToValue for OutputLockingSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.epoch_locking_settings {
                properties.insert(
                    "EpochLockingSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pipeline_locking_settings {
                properties.insert(
                    "PipelineLockingSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputsettings.html
    pub struct OutputSettings_ {
        pub archive_output_settings: Option<Box<ArchiveOutputSettings_>>,
        pub cmaf_ingest_output_settings: Option<Box<CmafIngestOutputSettings_>>,
        pub frame_capture_output_settings: Option<Box<FrameCaptureOutputSettings_>>,
        pub hls_output_settings: Option<Box<HlsOutputSettings_>>,
        pub media_package_output_settings: Option<Box<MediaPackageOutputSettings_>>,
        pub ms_smooth_output_settings: Option<Box<MsSmoothOutputSettings_>>,
        pub multiplex_output_settings: Option<Box<MultiplexOutputSettings_>>,
        pub rtmp_output_settings: Option<Box<RtmpOutputSettings_>>,
        pub srt_output_settings: Option<Box<SrtOutputSettings_>>,
        pub udp_output_settings: Option<Box<UdpOutputSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_OutputSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.OutputSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_OutputSettings as OutputSettings;
    impl crate::value::ToValue for OutputSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.archive_output_settings {
                properties.insert(
                    "ArchiveOutputSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cmaf_ingest_output_settings {
                properties.insert(
                    "CmafIngestOutputSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.frame_capture_output_settings {
                properties.insert(
                    "FrameCaptureOutputSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hls_output_settings {
                properties.insert(
                    "HlsOutputSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.media_package_output_settings {
                properties.insert(
                    "MediaPackageOutputSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ms_smooth_output_settings {
                properties.insert(
                    "MsSmoothOutputSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.multiplex_output_settings {
                properties.insert(
                    "MultiplexOutputSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rtmp_output_settings {
                properties.insert(
                    "RtmpOutputSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.srt_output_settings {
                properties.insert(
                    "SrtOutputSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.udp_output_settings {
                properties.insert(
                    "UdpOutputSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-passthroughsettings.html
    pub struct PassThroughSettings_ {}
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_PassThroughSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.PassThroughSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_PassThroughSettings as PassThroughSettings;
    impl crate::value::ToValue for PassThroughSettings_ {
        fn to_value(&self) -> serde_json::Value {
            serde_json::Value::Object(serde_json::Map::new())
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-pipelinelockingsettings.html
    pub struct PipelineLockingSettings_ {}
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_PipelineLockingSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.PipelineLockingSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_PipelineLockingSettings as PipelineLockingSettings;
    impl crate::value::ToValue for PipelineLockingSettings_ {
        fn to_value(&self) -> serde_json::Value {
            serde_json::Value::Object(serde_json::Map::new())
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-rawsettings.html
    pub struct RawSettings_ {}
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_RawSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.RawSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_RawSettings as RawSettings;
    impl crate::value::ToValue for RawSettings_ {
        fn to_value(&self) -> serde_json::Value {
            serde_json::Value::Object(serde_json::Map::new())
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-rec601settings.html
    pub struct Rec601Settings_ {}
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_Rec601Settings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.Rec601Settings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_Rec601Settings as Rec601Settings;
    impl crate::value::ToValue for Rec601Settings_ {
        fn to_value(&self) -> serde_json::Value {
            serde_json::Value::Object(serde_json::Map::new())
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-rec709settings.html
    pub struct Rec709Settings_ {}
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_Rec709Settings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.Rec709Settings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_Rec709Settings as Rec709Settings;
    impl crate::value::ToValue for Rec709Settings_ {
        fn to_value(&self) -> serde_json::Value {
            serde_json::Value::Object(serde_json::Map::new())
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-remixsettings.html
    pub struct RemixSettings_ {
        pub channel_mappings: Option<Vec<AudioChannelMapping_>>,
        pub channels_in: Option<i64>,
        pub channels_out: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_RemixSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.RemixSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_RemixSettings as RemixSettings;
    impl crate::value::ToValue for RemixSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.channel_mappings {
                properties.insert(
                    "ChannelMappings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.channels_in {
                properties.insert(
                    "ChannelsIn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.channels_out {
                properties.insert(
                    "ChannelsOut".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-rtmpcaptioninfodestinationsettings.html
    pub struct RtmpCaptionInfoDestinationSettings_ {}
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_RtmpCaptionInfoDestinationSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.RtmpCaptionInfoDestinationSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_RtmpCaptionInfoDestinationSettings as RtmpCaptionInfoDestinationSettings;
    impl crate::value::ToValue for RtmpCaptionInfoDestinationSettings_ {
        fn to_value(&self) -> serde_json::Value {
            serde_json::Value::Object(serde_json::Map::new())
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-rtmpgroupsettings.html
    pub struct RtmpGroupSettings_ {
        pub ad_markers: Option<Vec<crate::value::ExpString>>,
        pub authentication_scheme: Option<crate::value::ExpString>,
        pub cache_full_behavior: Option<crate::value::ExpString>,
        pub cache_length: Option<i64>,
        pub caption_data: Option<crate::value::ExpString>,
        pub include_filler_nal_units: Option<crate::value::ExpString>,
        pub input_loss_action: Option<crate::value::ExpString>,
        pub restart_delay: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_RtmpGroupSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.RtmpGroupSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_RtmpGroupSettings as RtmpGroupSettings;
    impl crate::value::ToValue for RtmpGroupSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ad_markers {
                properties.insert(
                    "AdMarkers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.authentication_scheme {
                properties.insert(
                    "AuthenticationScheme".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cache_full_behavior {
                properties.insert(
                    "CacheFullBehavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cache_length {
                properties.insert(
                    "CacheLength".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.caption_data {
                properties.insert(
                    "CaptionData".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_filler_nal_units {
                properties.insert(
                    "IncludeFillerNalUnits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_loss_action {
                properties.insert(
                    "InputLossAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.restart_delay {
                properties.insert(
                    "RestartDelay".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-rtmpoutputsettings.html
    pub struct RtmpOutputSettings_ {
        pub certificate_mode: Option<crate::value::ExpString>,
        pub connection_retry_interval: Option<i64>,
        pub destination: Option<Box<OutputLocationRef_>>,
        pub num_retries: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_RtmpOutputSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.RtmpOutputSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_RtmpOutputSettings as RtmpOutputSettings;
    impl crate::value::ToValue for RtmpOutputSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.certificate_mode {
                properties.insert(
                    "CertificateMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.connection_retry_interval {
                properties.insert(
                    "ConnectionRetryInterval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.destination {
                properties.insert(
                    "Destination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.num_retries {
                properties.insert(
                    "NumRetries".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-scte20plusembeddeddestinationsettings.html
    pub struct Scte20PlusEmbeddedDestinationSettings_ {}
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_Scte20PlusEmbeddedDestinationSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.Scte20PlusEmbeddedDestinationSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_Scte20PlusEmbeddedDestinationSettings as Scte20PlusEmbeddedDestinationSettings;
    impl crate::value::ToValue for Scte20PlusEmbeddedDestinationSettings_ {
        fn to_value(&self) -> serde_json::Value {
            serde_json::Value::Object(serde_json::Map::new())
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-scte20sourcesettings.html
    pub struct Scte20SourceSettings_ {
        pub convert608_to708: Option<crate::value::ExpString>,
        pub source608_channel_number: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_Scte20SourceSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.Scte20SourceSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_Scte20SourceSettings as Scte20SourceSettings;
    impl crate::value::ToValue for Scte20SourceSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.convert608_to708 {
                properties.insert(
                    "Convert608To708".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source608_channel_number {
                properties.insert(
                    "Source608ChannelNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-scte27destinationsettings.html
    pub struct Scte27DestinationSettings_ {}
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_Scte27DestinationSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.Scte27DestinationSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_Scte27DestinationSettings as Scte27DestinationSettings;
    impl crate::value::ToValue for Scte27DestinationSettings_ {
        fn to_value(&self) -> serde_json::Value {
            serde_json::Value::Object(serde_json::Map::new())
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-scte27sourcesettings.html
    pub struct Scte27SourceSettings_ {
        pub ocr_language: Option<crate::value::ExpString>,
        pub pid: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_Scte27SourceSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.Scte27SourceSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_Scte27SourceSettings as Scte27SourceSettings;
    impl crate::value::ToValue for Scte27SourceSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ocr_language {
                properties.insert(
                    "OcrLanguage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pid {
                properties.insert("Pid".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-scte35spliceinsert.html
    pub struct Scte35SpliceInsert_ {
        pub ad_avail_offset: Option<i64>,
        pub no_regional_blackout_flag: Option<crate::value::ExpString>,
        pub web_delivery_allowed_flag: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_Scte35SpliceInsert {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.Scte35SpliceInsert"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_Scte35SpliceInsert as Scte35SpliceInsert;
    impl crate::value::ToValue for Scte35SpliceInsert_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ad_avail_offset {
                properties.insert(
                    "AdAvailOffset".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.no_regional_blackout_flag {
                properties.insert(
                    "NoRegionalBlackoutFlag".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.web_delivery_allowed_flag {
                properties.insert(
                    "WebDeliveryAllowedFlag".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-scte35timesignalapos.html
    pub struct Scte35TimeSignalApos_ {
        pub ad_avail_offset: Option<i64>,
        pub no_regional_blackout_flag: Option<crate::value::ExpString>,
        pub web_delivery_allowed_flag: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_Scte35TimeSignalApos {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.Scte35TimeSignalApos"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_Scte35TimeSignalApos as Scte35TimeSignalApos;
    impl crate::value::ToValue for Scte35TimeSignalApos_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ad_avail_offset {
                properties.insert(
                    "AdAvailOffset".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.no_regional_blackout_flag {
                properties.insert(
                    "NoRegionalBlackoutFlag".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.web_delivery_allowed_flag {
                properties.insert(
                    "WebDeliveryAllowedFlag".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-smptettdestinationsettings.html
    pub struct SmpteTtDestinationSettings_ {}
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_SmpteTtDestinationSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.SmpteTtDestinationSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_SmpteTtDestinationSettings as SmpteTtDestinationSettings;
    impl crate::value::ToValue for SmpteTtDestinationSettings_ {
        fn to_value(&self) -> serde_json::Value {
            serde_json::Value::Object(serde_json::Map::new())
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-srtgroupsettings.html
    pub struct SrtGroupSettings_ {
        pub input_loss_action: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_SrtGroupSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.SrtGroupSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_SrtGroupSettings as SrtGroupSettings;
    impl crate::value::ToValue for SrtGroupSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.input_loss_action {
                properties.insert(
                    "InputLossAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-srtoutputdestinationsettings.html
    pub struct SrtOutputDestinationSettings_ {
        pub encryption_passphrase_secret_arn: Option<crate::value::ExpString>,
        pub stream_id: Option<crate::value::ExpString>,
        pub url: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_SrtOutputDestinationSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.SrtOutputDestinationSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_SrtOutputDestinationSettings as SrtOutputDestinationSettings;
    impl crate::value::ToValue for SrtOutputDestinationSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.encryption_passphrase_secret_arn {
                properties.insert(
                    "EncryptionPassphraseSecretArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stream_id {
                properties.insert(
                    "StreamId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.url {
                properties.insert("Url".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-srtoutputsettings.html
    pub struct SrtOutputSettings_ {
        pub buffer_msec: Option<i64>,
        pub container_settings: Option<Box<UdpContainerSettings_>>,
        pub destination: Option<Box<OutputLocationRef_>>,
        pub encryption_type: Option<crate::value::ExpString>,
        pub latency: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_SrtOutputSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.SrtOutputSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_SrtOutputSettings as SrtOutputSettings;
    impl crate::value::ToValue for SrtOutputSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.buffer_msec {
                properties.insert(
                    "BufferMsec".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.container_settings {
                properties.insert(
                    "ContainerSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.destination {
                properties.insert(
                    "Destination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.encryption_type {
                properties.insert(
                    "EncryptionType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.latency {
                properties.insert(
                    "Latency".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-standardhlssettings.html
    pub struct StandardHlsSettings_ {
        pub audio_rendition_sets: Option<crate::value::ExpString>,
        pub m3u8_settings: Option<Box<M3u8Settings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_StandardHlsSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.StandardHlsSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_StandardHlsSettings as StandardHlsSettings;
    impl crate::value::ToValue for StandardHlsSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.audio_rendition_sets {
                properties.insert(
                    "AudioRenditionSets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.m3u8_settings {
                properties.insert(
                    "M3u8Settings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-statickeysettings.html
    pub struct StaticKeySettings_ {
        pub key_provider_server: Option<Box<InputLocation_>>,
        pub static_key_value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_StaticKeySettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.StaticKeySettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_StaticKeySettings as StaticKeySettings;
    impl crate::value::ToValue for StaticKeySettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.key_provider_server {
                properties.insert(
                    "KeyProviderServer".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.static_key_value {
                properties.insert(
                    "StaticKeyValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-teletextdestinationsettings.html
    pub struct TeletextDestinationSettings_ {}
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_TeletextDestinationSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.TeletextDestinationSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_TeletextDestinationSettings as TeletextDestinationSettings;
    impl crate::value::ToValue for TeletextDestinationSettings_ {
        fn to_value(&self) -> serde_json::Value {
            serde_json::Value::Object(serde_json::Map::new())
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-teletextsourcesettings.html
    pub struct TeletextSourceSettings_ {
        pub output_rectangle: Option<Box<CaptionRectangle_>>,
        pub page_number: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_TeletextSourceSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.TeletextSourceSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_TeletextSourceSettings as TeletextSourceSettings;
    impl crate::value::ToValue for TeletextSourceSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.output_rectangle {
                properties.insert(
                    "OutputRectangle".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.page_number {
                properties.insert(
                    "PageNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-temporalfiltersettings.html
    pub struct TemporalFilterSettings_ {
        pub post_filter_sharpening: Option<crate::value::ExpString>,
        pub strength: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_TemporalFilterSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.TemporalFilterSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_TemporalFilterSettings as TemporalFilterSettings;
    impl crate::value::ToValue for TemporalFilterSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.post_filter_sharpening {
                properties.insert(
                    "PostFilterSharpening".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.strength {
                properties.insert(
                    "Strength".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-thumbnailconfiguration.html
    pub struct ThumbnailConfiguration_ {
        pub state: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_ThumbnailConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.ThumbnailConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_ThumbnailConfiguration as ThumbnailConfiguration;
    impl crate::value::ToValue for ThumbnailConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.state {
                properties.insert("State".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-timecodeburninsettings.html
    pub struct TimecodeBurninSettings_ {
        pub font_size: Option<crate::value::ExpString>,
        pub position: Option<crate::value::ExpString>,
        pub prefix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_TimecodeBurninSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.TimecodeBurninSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_TimecodeBurninSettings as TimecodeBurninSettings;
    impl crate::value::ToValue for TimecodeBurninSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.font_size {
                properties.insert(
                    "FontSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.position {
                properties.insert(
                    "Position".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.prefix {
                properties.insert("Prefix".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-timecodeconfig.html
    pub struct TimecodeConfig_ {
        pub source: Option<crate::value::ExpString>,
        pub sync_threshold: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_TimecodeConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.TimecodeConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_TimecodeConfig as TimecodeConfig;
    impl crate::value::ToValue for TimecodeConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.source {
                properties.insert("Source".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.sync_threshold {
                properties.insert(
                    "SyncThreshold".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-ttmldestinationsettings.html
    pub struct TtmlDestinationSettings_ {
        pub style_control: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_TtmlDestinationSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.TtmlDestinationSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_TtmlDestinationSettings as TtmlDestinationSettings;
    impl crate::value::ToValue for TtmlDestinationSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.style_control {
                properties.insert(
                    "StyleControl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-udpcontainersettings.html
    pub struct UdpContainerSettings_ {
        pub m2ts_settings: Option<Box<M2tsSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_UdpContainerSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.UdpContainerSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_UdpContainerSettings as UdpContainerSettings;
    impl crate::value::ToValue for UdpContainerSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.m2ts_settings {
                properties.insert(
                    "M2tsSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-udpgroupsettings.html
    pub struct UdpGroupSettings_ {
        pub input_loss_action: Option<crate::value::ExpString>,
        pub timed_metadata_id3_frame: Option<crate::value::ExpString>,
        pub timed_metadata_id3_period: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_UdpGroupSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.UdpGroupSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_UdpGroupSettings as UdpGroupSettings;
    impl crate::value::ToValue for UdpGroupSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.input_loss_action {
                properties.insert(
                    "InputLossAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timed_metadata_id3_frame {
                properties.insert(
                    "TimedMetadataId3Frame".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timed_metadata_id3_period {
                properties.insert(
                    "TimedMetadataId3Period".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-udpoutputsettings.html
    pub struct UdpOutputSettings_ {
        pub buffer_msec: Option<i64>,
        pub container_settings: Option<Box<UdpContainerSettings_>>,
        pub destination: Option<Box<OutputLocationRef_>>,
        pub fec_output_settings: Option<Box<FecOutputSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_UdpOutputSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.UdpOutputSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_UdpOutputSettings as UdpOutputSettings;
    impl crate::value::ToValue for UdpOutputSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.buffer_msec {
                properties.insert(
                    "BufferMsec".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.container_settings {
                properties.insert(
                    "ContainerSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.destination {
                properties.insert(
                    "Destination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.fec_output_settings {
                properties.insert(
                    "FecOutputSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-videoblackfailoversettings.html
    pub struct VideoBlackFailoverSettings_ {
        pub black_detect_threshold: Option<f64>,
        pub video_black_threshold_msec: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_VideoBlackFailoverSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.VideoBlackFailoverSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_VideoBlackFailoverSettings as VideoBlackFailoverSettings;
    impl crate::value::ToValue for VideoBlackFailoverSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.black_detect_threshold {
                properties.insert(
                    "BlackDetectThreshold".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.video_black_threshold_msec {
                properties.insert(
                    "VideoBlackThresholdMsec".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-videocodecsettings.html
    pub struct VideoCodecSettings_ {
        pub av1_settings: Option<Box<Av1Settings_>>,
        pub frame_capture_settings: Option<Box<FrameCaptureSettings_>>,
        pub h264_settings: Option<Box<H264Settings_>>,
        pub h265_settings: Option<Box<H265Settings_>>,
        pub mpeg2_settings: Option<Box<Mpeg2Settings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_VideoCodecSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.VideoCodecSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_VideoCodecSettings as VideoCodecSettings;
    impl crate::value::ToValue for VideoCodecSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.av1_settings {
                properties.insert(
                    "Av1Settings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.frame_capture_settings {
                properties.insert(
                    "FrameCaptureSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.h264_settings {
                properties.insert(
                    "H264Settings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.h265_settings {
                properties.insert(
                    "H265Settings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mpeg2_settings {
                properties.insert(
                    "Mpeg2Settings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-videodescription.html
    pub struct VideoDescription_ {
        pub codec_settings: Option<Box<VideoCodecSettings_>>,
        pub height: Option<i64>,
        pub name: Option<crate::value::ExpString>,
        pub respond_to_afd: Option<crate::value::ExpString>,
        pub scaling_behavior: Option<crate::value::ExpString>,
        pub sharpness: Option<i64>,
        pub width: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_VideoDescription {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.VideoDescription"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_VideoDescription as VideoDescription;
    impl crate::value::ToValue for VideoDescription_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.codec_settings {
                properties.insert(
                    "CodecSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.height {
                properties.insert("Height".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.respond_to_afd {
                properties.insert(
                    "RespondToAfd".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scaling_behavior {
                properties.insert(
                    "ScalingBehavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sharpness {
                properties.insert(
                    "Sharpness".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.width {
                properties.insert("Width".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-videoselector.html
    pub struct VideoSelector_ {
        pub color_space: Option<crate::value::ExpString>,
        pub color_space_settings: Option<Box<VideoSelectorColorSpaceSettings_>>,
        pub color_space_usage: Option<crate::value::ExpString>,
        pub selector_settings: Option<Box<VideoSelectorSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_VideoSelector {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.VideoSelector"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_VideoSelector as VideoSelector;
    impl crate::value::ToValue for VideoSelector_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.color_space {
                properties.insert(
                    "ColorSpace".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.color_space_settings {
                properties.insert(
                    "ColorSpaceSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.color_space_usage {
                properties.insert(
                    "ColorSpaceUsage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.selector_settings {
                properties.insert(
                    "SelectorSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-videoselectorcolorspacesettings.html
    pub struct VideoSelectorColorSpaceSettings_ {
        pub hdr10_settings: Option<Box<Hdr10Settings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_VideoSelectorColorSpaceSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.VideoSelectorColorSpaceSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_VideoSelectorColorSpaceSettings as VideoSelectorColorSpaceSettings;
    impl crate::value::ToValue for VideoSelectorColorSpaceSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.hdr10_settings {
                properties.insert(
                    "Hdr10Settings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-videoselectorpid.html
    pub struct VideoSelectorPid_ {
        pub pid: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_VideoSelectorPid {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.VideoSelectorPid"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_VideoSelectorPid as VideoSelectorPid;
    impl crate::value::ToValue for VideoSelectorPid_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.pid {
                properties.insert("Pid".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-videoselectorprogramid.html
    pub struct VideoSelectorProgramId_ {
        pub program_id: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_VideoSelectorProgramId {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.VideoSelectorProgramId"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_VideoSelectorProgramId as VideoSelectorProgramId;
    impl crate::value::ToValue for VideoSelectorProgramId_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.program_id {
                properties.insert(
                    "ProgramId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-videoselectorsettings.html
    pub struct VideoSelectorSettings_ {
        pub video_selector_pid: Option<Box<VideoSelectorPid_>>,
        pub video_selector_program_id: Option<Box<VideoSelectorProgramId_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_VideoSelectorSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.VideoSelectorSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_VideoSelectorSettings as VideoSelectorSettings;
    impl crate::value::ToValue for VideoSelectorSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.video_selector_pid {
                properties.insert(
                    "VideoSelectorPid".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.video_selector_program_id {
                properties.insert(
                    "VideoSelectorProgramId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-vpcoutputsettings.html
    pub struct VpcOutputSettings_ {
        pub public_address_allocation_ids: Option<Vec<crate::value::ExpString>>,
        pub security_group_ids: Option<Vec<crate::value::ExpString>>,
        pub subnet_ids: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_VpcOutputSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.VpcOutputSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_VpcOutputSettings as VpcOutputSettings;
    impl crate::value::ToValue for VpcOutputSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.public_address_allocation_ids {
                properties.insert(
                    "PublicAddressAllocationIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.security_group_ids {
                properties.insert(
                    "SecurityGroupIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.subnet_ids {
                properties.insert(
                    "SubnetIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-wavsettings.html
    pub struct WavSettings_ {
        pub bit_depth: Option<f64>,
        pub coding_mode: Option<crate::value::ExpString>,
        pub sample_rate: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_WavSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.WavSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_WavSettings as WavSettings;
    impl crate::value::ToValue for WavSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bit_depth {
                properties.insert(
                    "BitDepth".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.coding_mode {
                properties.insert(
                    "CodingMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sample_rate {
                properties.insert(
                    "SampleRate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-webvttdestinationsettings.html
    pub struct WebvttDestinationSettings_ {
        pub style_control: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Channel_WebvttDestinationSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Channel.WebvttDestinationSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Channel_WebvttDestinationSettings as WebvttDestinationSettings;
    impl crate::value::ToValue for WebvttDestinationSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.style_control {
                properties.insert(
                    "StyleControl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod channelplacementgroup {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channelplacementgroup-tags.html
    pub struct Tags_ {
        pub key: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_ChannelPlacementGroup_Tags {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::ChannelPlacementGroup.Tags"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_ChannelPlacementGroup_Tags as Tags;
    impl crate::value::ToValue for Tags_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.key {
                properties.insert("Key".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod cluster {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-cluster-clusternetworksettings.html
    pub struct ClusterNetworkSettings_ {
        pub default_route: Option<crate::value::ExpString>,
        pub interface_mappings: Option<Vec<InterfaceMapping_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Cluster_ClusterNetworkSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Cluster.ClusterNetworkSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Cluster_ClusterNetworkSettings as ClusterNetworkSettings;
    impl crate::value::ToValue for ClusterNetworkSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.default_route {
                properties.insert(
                    "DefaultRoute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.interface_mappings {
                properties.insert(
                    "InterfaceMappings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-cluster-interfacemapping.html
    pub struct InterfaceMapping_ {
        pub logical_interface_name: Option<crate::value::ExpString>,
        pub network_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Cluster_InterfaceMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Cluster.InterfaceMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Cluster_InterfaceMapping as InterfaceMapping;
    impl crate::value::ToValue for InterfaceMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.logical_interface_name {
                properties.insert(
                    "LogicalInterfaceName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_id {
                properties.insert(
                    "NetworkId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-cluster-tags.html
    pub struct Tags_ {
        pub key: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Cluster_Tags {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Cluster.Tags"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Cluster_Tags as Tags;
    impl crate::value::ToValue for Tags_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.key {
                properties.insert("Key".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod eventbridgeruletemplate {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-eventbridgeruletemplate-eventbridgeruletemplatetarget.html
    pub struct EventBridgeRuleTemplateTarget_ {
        pub arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_EventBridgeRuleTemplate_EventBridgeRuleTemplateTarget {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::EventBridgeRuleTemplate.EventBridgeRuleTemplateTarget"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_EventBridgeRuleTemplate_EventBridgeRuleTemplateTarget as EventBridgeRuleTemplateTarget;
    impl crate::value::ToValue for EventBridgeRuleTemplateTarget_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Arn".to_string(),
                crate::value::ToValue::to_value(&self.arn),
            );
            properties.into()
        }
    }
}
pub mod input {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-input-inputdestinationrequest.html
    pub struct InputDestinationRequest_ {
        pub network: Option<crate::value::ExpString>,
        pub network_routes: Option<Vec<InputRequestDestinationRoute_>>,
        pub static_ip_address: Option<crate::value::ExpString>,
        pub stream_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Input_InputDestinationRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Input.InputDestinationRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Input_InputDestinationRequest as InputDestinationRequest;
    impl crate::value::ToValue for InputDestinationRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.network {
                properties.insert(
                    "Network".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_routes {
                properties.insert(
                    "NetworkRoutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.static_ip_address {
                properties.insert(
                    "StaticIpAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stream_name {
                properties.insert(
                    "StreamName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-input-inputdevicerequest.html
    pub struct InputDeviceRequest_ {
        pub id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Input_InputDeviceRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Input.InputDeviceRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Input_InputDeviceRequest as InputDeviceRequest;
    impl crate::value::ToValue for InputDeviceRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.id {
                properties.insert("Id".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-input-inputdevicesettings.html
    pub struct InputDeviceSettings_ {
        pub id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Input_InputDeviceSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Input.InputDeviceSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Input_InputDeviceSettings as InputDeviceSettings;
    impl crate::value::ToValue for InputDeviceSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.id {
                properties.insert("Id".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-input-inputrequestdestinationroute.html
    pub struct InputRequestDestinationRoute_ {
        pub cidr: Option<crate::value::ExpString>,
        pub gateway: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Input_InputRequestDestinationRoute {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Input.InputRequestDestinationRoute"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Input_InputRequestDestinationRoute as InputRequestDestinationRoute;
    impl crate::value::ToValue for InputRequestDestinationRoute_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cidr {
                properties.insert("Cidr".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.gateway {
                properties.insert(
                    "Gateway".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-input-inputsdplocation.html
    pub struct InputSdpLocation_ {
        pub media_index: Option<i64>,
        pub sdp_url: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Input_InputSdpLocation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Input.InputSdpLocation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Input_InputSdpLocation as InputSdpLocation;
    impl crate::value::ToValue for InputSdpLocation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.media_index {
                properties.insert(
                    "MediaIndex".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sdp_url {
                properties.insert("SdpUrl".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-input-inputsourcerequest.html
    pub struct InputSourceRequest_ {
        pub password_param: Option<crate::value::ExpString>,
        pub url: Option<crate::value::ExpString>,
        pub username: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Input_InputSourceRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Input.InputSourceRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Input_InputSourceRequest as InputSourceRequest;
    impl crate::value::ToValue for InputSourceRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.password_param {
                properties.insert(
                    "PasswordParam".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.url {
                properties.insert("Url".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.username {
                properties.insert(
                    "Username".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-input-inputvpcrequest.html
    pub struct InputVpcRequest_ {
        pub security_group_ids: Option<Vec<crate::value::ExpString>>,
        pub subnet_ids: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Input_InputVpcRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Input.InputVpcRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Input_InputVpcRequest as InputVpcRequest;
    impl crate::value::ToValue for InputVpcRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.security_group_ids {
                properties.insert(
                    "SecurityGroupIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.subnet_ids {
                properties.insert(
                    "SubnetIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-input-mediaconnectflowrequest.html
    pub struct MediaConnectFlowRequest_ {
        pub flow_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Input_MediaConnectFlowRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Input.MediaConnectFlowRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Input_MediaConnectFlowRequest as MediaConnectFlowRequest;
    impl crate::value::ToValue for MediaConnectFlowRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.flow_arn {
                properties.insert(
                    "FlowArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-input-multicastsettingscreaterequest.html
    pub struct MulticastSettingsCreateRequest_ {
        pub sources: Option<Vec<MulticastSourceCreateRequest_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Input_MulticastSettingsCreateRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Input.MulticastSettingsCreateRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Input_MulticastSettingsCreateRequest as MulticastSettingsCreateRequest;
    impl crate::value::ToValue for MulticastSettingsCreateRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.sources {
                properties.insert(
                    "Sources".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-input-multicastsettingsupdaterequest.html
    pub struct MulticastSettingsUpdateRequest_ {
        pub sources: Option<Vec<MulticastSourceUpdateRequest_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Input_MulticastSettingsUpdateRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Input.MulticastSettingsUpdateRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Input_MulticastSettingsUpdateRequest as MulticastSettingsUpdateRequest;
    impl crate::value::ToValue for MulticastSettingsUpdateRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.sources {
                properties.insert(
                    "Sources".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-input-multicastsourcecreaterequest.html
    pub struct MulticastSourceCreateRequest_ {
        pub source_ip: Option<crate::value::ExpString>,
        pub url: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Input_MulticastSourceCreateRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Input.MulticastSourceCreateRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Input_MulticastSourceCreateRequest as MulticastSourceCreateRequest;
    impl crate::value::ToValue for MulticastSourceCreateRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.source_ip {
                properties.insert(
                    "SourceIp".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.url {
                properties.insert("Url".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-input-multicastsourceupdaterequest.html
    pub struct MulticastSourceUpdateRequest_ {
        pub source_ip: Option<crate::value::ExpString>,
        pub url: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Input_MulticastSourceUpdateRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Input.MulticastSourceUpdateRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Input_MulticastSourceUpdateRequest as MulticastSourceUpdateRequest;
    impl crate::value::ToValue for MulticastSourceUpdateRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.source_ip {
                properties.insert(
                    "SourceIp".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.url {
                properties.insert("Url".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-input-smpte2110receivergroup.html
    pub struct Smpte2110ReceiverGroup_ {
        pub sdp_settings: Option<Box<Smpte2110ReceiverGroupSdpSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Input_Smpte2110ReceiverGroup {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Input.Smpte2110ReceiverGroup"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Input_Smpte2110ReceiverGroup as Smpte2110ReceiverGroup;
    impl crate::value::ToValue for Smpte2110ReceiverGroup_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.sdp_settings {
                properties.insert(
                    "SdpSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-input-smpte2110receivergroupsdpsettings.html
    pub struct Smpte2110ReceiverGroupSdpSettings_ {
        pub ancillary_sdps: Option<Vec<InputSdpLocation_>>,
        pub audio_sdps: Option<Vec<InputSdpLocation_>>,
        pub video_sdp: Option<Box<InputSdpLocation_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Input_Smpte2110ReceiverGroupSdpSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Input.Smpte2110ReceiverGroupSdpSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Input_Smpte2110ReceiverGroupSdpSettings as Smpte2110ReceiverGroupSdpSettings;
    impl crate::value::ToValue for Smpte2110ReceiverGroupSdpSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ancillary_sdps {
                properties.insert(
                    "AncillarySdps".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.audio_sdps {
                properties.insert(
                    "AudioSdps".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.video_sdp {
                properties.insert(
                    "VideoSdp".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-input-smpte2110receivergroupsettings.html
    pub struct Smpte2110ReceiverGroupSettings_ {
        pub smpte2110_receiver_groups: Option<Vec<Smpte2110ReceiverGroup_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Input_Smpte2110ReceiverGroupSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Input.Smpte2110ReceiverGroupSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Input_Smpte2110ReceiverGroupSettings as Smpte2110ReceiverGroupSettings;
    impl crate::value::ToValue for Smpte2110ReceiverGroupSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.smpte2110_receiver_groups {
                properties.insert(
                    "Smpte2110ReceiverGroups".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-input-srtcallerdecryptionrequest.html
    pub struct SrtCallerDecryptionRequest_ {
        pub algorithm: Option<crate::value::ExpString>,
        pub passphrase_secret_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Input_SrtCallerDecryptionRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Input.SrtCallerDecryptionRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Input_SrtCallerDecryptionRequest as SrtCallerDecryptionRequest;
    impl crate::value::ToValue for SrtCallerDecryptionRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.algorithm {
                properties.insert(
                    "Algorithm".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.passphrase_secret_arn {
                properties.insert(
                    "PassphraseSecretArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-input-srtcallersourcerequest.html
    pub struct SrtCallerSourceRequest_ {
        pub decryption: Option<Box<SrtCallerDecryptionRequest_>>,
        pub minimum_latency: Option<i64>,
        pub srt_listener_address: Option<crate::value::ExpString>,
        pub srt_listener_port: Option<crate::value::ExpString>,
        pub stream_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Input_SrtCallerSourceRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Input.SrtCallerSourceRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Input_SrtCallerSourceRequest as SrtCallerSourceRequest;
    impl crate::value::ToValue for SrtCallerSourceRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.decryption {
                properties.insert(
                    "Decryption".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.minimum_latency {
                properties.insert(
                    "MinimumLatency".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.srt_listener_address {
                properties.insert(
                    "SrtListenerAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.srt_listener_port {
                properties.insert(
                    "SrtListenerPort".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stream_id {
                properties.insert(
                    "StreamId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-input-srtsettingsrequest.html
    pub struct SrtSettingsRequest_ {
        pub srt_caller_sources: Option<Vec<SrtCallerSourceRequest_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Input_SrtSettingsRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Input.SrtSettingsRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Input_SrtSettingsRequest as SrtSettingsRequest;
    impl crate::value::ToValue for SrtSettingsRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.srt_caller_sources {
                properties.insert(
                    "SrtCallerSources".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod inputsecuritygroup {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-inputsecuritygroup-inputwhitelistrulecidr.html
    pub struct InputWhitelistRuleCidr_ {
        pub cidr: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_InputSecurityGroup_InputWhitelistRuleCidr {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::InputSecurityGroup.InputWhitelistRuleCidr"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_InputSecurityGroup_InputWhitelistRuleCidr as InputWhitelistRuleCidr;
    impl crate::value::ToValue for InputWhitelistRuleCidr_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cidr {
                properties.insert("Cidr".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod multiplex {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-multiplex-multiplexmediaconnectoutputdestinationsettings.html
    pub struct MultiplexMediaConnectOutputDestinationSettings_ {
        pub entitlement_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Multiplex_MultiplexMediaConnectOutputDestinationSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Multiplex.MultiplexMediaConnectOutputDestinationSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Multiplex_MultiplexMediaConnectOutputDestinationSettings as MultiplexMediaConnectOutputDestinationSettings;
    impl crate::value::ToValue for MultiplexMediaConnectOutputDestinationSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.entitlement_arn {
                properties.insert(
                    "EntitlementArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-multiplex-multiplexoutputdestination.html
    pub struct MultiplexOutputDestination_ {
        pub multiplex_media_connect_output_destination_settings:
            Option<Box<MultiplexMediaConnectOutputDestinationSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Multiplex_MultiplexOutputDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Multiplex.MultiplexOutputDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Multiplex_MultiplexOutputDestination as MultiplexOutputDestination;
    impl crate::value::ToValue for MultiplexOutputDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.multiplex_media_connect_output_destination_settings {
                properties.insert(
                    "MultiplexMediaConnectOutputDestinationSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-multiplex-multiplexsettings.html
    pub struct MultiplexSettings_ {
        pub maximum_video_buffer_delay_milliseconds: Option<i64>,
        pub transport_stream_bitrate: i64,
        pub transport_stream_id: i64,
        pub transport_stream_reserved_bitrate: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Multiplex_MultiplexSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Multiplex.MultiplexSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Multiplex_MultiplexSettings as MultiplexSettings;
    impl crate::value::ToValue for MultiplexSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.maximum_video_buffer_delay_milliseconds {
                properties.insert(
                    "MaximumVideoBufferDelayMilliseconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TransportStreamBitrate".to_string(),
                crate::value::ToValue::to_value(&self.transport_stream_bitrate),
            );
            properties.insert(
                "TransportStreamId".to_string(),
                crate::value::ToValue::to_value(&self.transport_stream_id),
            );
            if let Some(ref value) = self.transport_stream_reserved_bitrate {
                properties.insert(
                    "TransportStreamReservedBitrate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-multiplex-tags.html
    pub struct Tags_ {
        pub key: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Multiplex_Tags {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Multiplex.Tags"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Multiplex_Tags as Tags;
    impl crate::value::ToValue for Tags_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.key {
                properties.insert("Key".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod multiplexprogram {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-multiplexprogram-multiplexprogrampacketidentifiersmap.html
    pub struct MultiplexProgramPacketIdentifiersMap_ {
        pub audio_pids: Option<Vec<i64>>,
        pub dvb_sub_pids: Option<Vec<i64>>,
        pub dvb_teletext_pid: Option<i64>,
        pub etv_platform_pid: Option<i64>,
        pub etv_signal_pid: Option<i64>,
        pub klv_data_pids: Option<Vec<i64>>,
        pub pcr_pid: Option<i64>,
        pub pmt_pid: Option<i64>,
        pub private_metadata_pid: Option<i64>,
        pub scte27_pids: Option<Vec<i64>>,
        pub scte35_pid: Option<i64>,
        pub timed_metadata_pid: Option<i64>,
        pub video_pid: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Multiplexprogram_MultiplexProgramPacketIdentifiersMap {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Multiplexprogram.MultiplexProgramPacketIdentifiersMap"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Multiplexprogram_MultiplexProgramPacketIdentifiersMap as MultiplexProgramPacketIdentifiersMap;
    impl crate::value::ToValue for MultiplexProgramPacketIdentifiersMap_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.audio_pids {
                properties.insert(
                    "AudioPids".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dvb_sub_pids {
                properties.insert(
                    "DvbSubPids".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dvb_teletext_pid {
                properties.insert(
                    "DvbTeletextPid".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.etv_platform_pid {
                properties.insert(
                    "EtvPlatformPid".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.etv_signal_pid {
                properties.insert(
                    "EtvSignalPid".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.klv_data_pids {
                properties.insert(
                    "KlvDataPids".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pcr_pid {
                properties.insert("PcrPid".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.pmt_pid {
                properties.insert("PmtPid".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.private_metadata_pid {
                properties.insert(
                    "PrivateMetadataPid".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scte27_pids {
                properties.insert(
                    "Scte27Pids".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scte35_pid {
                properties.insert(
                    "Scte35Pid".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timed_metadata_pid {
                properties.insert(
                    "TimedMetadataPid".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.video_pid {
                properties.insert(
                    "VideoPid".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-multiplexprogram-multiplexprogrampipelinedetail.html
    pub struct MultiplexProgramPipelineDetail_ {
        pub active_channel_pipeline: Option<crate::value::ExpString>,
        pub pipeline_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Multiplexprogram_MultiplexProgramPipelineDetail {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Multiplexprogram.MultiplexProgramPipelineDetail"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Multiplexprogram_MultiplexProgramPipelineDetail as MultiplexProgramPipelineDetail;
    impl crate::value::ToValue for MultiplexProgramPipelineDetail_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.active_channel_pipeline {
                properties.insert(
                    "ActiveChannelPipeline".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pipeline_id {
                properties.insert(
                    "PipelineId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-multiplexprogram-multiplexprogramservicedescriptor.html
    pub struct MultiplexProgramServiceDescriptor_ {
        pub provider_name: crate::value::ExpString,
        pub service_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Multiplexprogram_MultiplexProgramServiceDescriptor {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Multiplexprogram.MultiplexProgramServiceDescriptor"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Multiplexprogram_MultiplexProgramServiceDescriptor as MultiplexProgramServiceDescriptor;
    impl crate::value::ToValue for MultiplexProgramServiceDescriptor_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ProviderName".to_string(),
                crate::value::ToValue::to_value(&self.provider_name),
            );
            properties.insert(
                "ServiceName".to_string(),
                crate::value::ToValue::to_value(&self.service_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-multiplexprogram-multiplexprogramsettings.html
    pub struct MultiplexProgramSettings_ {
        pub preferred_channel_pipeline: Option<crate::value::ExpString>,
        pub program_number: i64,
        pub service_descriptor: Option<Box<MultiplexProgramServiceDescriptor_>>,
        pub video_settings: Option<Box<MultiplexVideoSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Multiplexprogram_MultiplexProgramSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Multiplexprogram.MultiplexProgramSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Multiplexprogram_MultiplexProgramSettings as MultiplexProgramSettings;
    impl crate::value::ToValue for MultiplexProgramSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.preferred_channel_pipeline {
                properties.insert(
                    "PreferredChannelPipeline".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ProgramNumber".to_string(),
                crate::value::ToValue::to_value(&self.program_number),
            );
            if let Some(ref value) = self.service_descriptor {
                properties.insert(
                    "ServiceDescriptor".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.video_settings {
                properties.insert(
                    "VideoSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-multiplexprogram-multiplexstatmuxvideosettings.html
    pub struct MultiplexStatmuxVideoSettings_ {
        pub maximum_bitrate: Option<i64>,
        pub minimum_bitrate: Option<i64>,
        pub priority: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Multiplexprogram_MultiplexStatmuxVideoSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Multiplexprogram.MultiplexStatmuxVideoSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Multiplexprogram_MultiplexStatmuxVideoSettings as MultiplexStatmuxVideoSettings;
    impl crate::value::ToValue for MultiplexStatmuxVideoSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.maximum_bitrate {
                properties.insert(
                    "MaximumBitrate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.minimum_bitrate {
                properties.insert(
                    "MinimumBitrate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.priority {
                properties.insert(
                    "Priority".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-multiplexprogram-multiplexvideosettings.html
    pub struct MultiplexVideoSettings_ {
        pub constant_bitrate: Option<i64>,
        pub statmux_settings: Option<Box<MultiplexStatmuxVideoSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Multiplexprogram_MultiplexVideoSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Multiplexprogram.MultiplexVideoSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Multiplexprogram_MultiplexVideoSettings as MultiplexVideoSettings;
    impl crate::value::ToValue for MultiplexVideoSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.constant_bitrate {
                properties.insert(
                    "ConstantBitrate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.statmux_settings {
                properties.insert(
                    "StatmuxSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod network {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-network-ippool.html
    pub struct IpPool_ {
        pub cidr: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Network_IpPool {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Network.IpPool"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Network_IpPool as IpPool;
    impl crate::value::ToValue for IpPool_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cidr {
                properties.insert("Cidr".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-network-route.html
    pub struct Route_ {
        pub cidr: Option<crate::value::ExpString>,
        pub gateway: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Network_Route {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Network.Route"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Network_Route as Route;
    impl crate::value::ToValue for Route_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cidr {
                properties.insert("Cidr".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.gateway {
                properties.insert(
                    "Gateway".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-network-tags.html
    pub struct Tags_ {
        pub key: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_Network_Tags {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::Network.Tags"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_Network_Tags as Tags;
    impl crate::value::ToValue for Tags_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.key {
                properties.insert("Key".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod sdisource {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-sdisource-tags.html
    pub struct Tags_ {
        pub key: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_SdiSource_Tags {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::SdiSource.Tags"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_SdiSource_Tags as Tags;
    impl crate::value::ToValue for Tags_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.key {
                properties.insert("Key".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod signalmap {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-signalmap-mediaresource.html
    pub struct MediaResource_ {
        pub destinations: Option<Vec<MediaResourceNeighbor_>>,
        pub name: Option<crate::value::ExpString>,
        pub sources: Option<Vec<MediaResourceNeighbor_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_SignalMap_MediaResource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::SignalMap.MediaResource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_SignalMap_MediaResource as MediaResource;
    impl crate::value::ToValue for MediaResource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.destinations {
                properties.insert(
                    "Destinations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.sources {
                properties.insert(
                    "Sources".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-signalmap-mediaresourceneighbor.html
    pub struct MediaResourceNeighbor_ {
        pub arn: crate::value::ExpString,
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_SignalMap_MediaResourceNeighbor {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::SignalMap.MediaResourceNeighbor"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_SignalMap_MediaResourceNeighbor as MediaResourceNeighbor;
    impl crate::value::ToValue for MediaResourceNeighbor_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Arn".to_string(),
                crate::value::ToValue::to_value(&self.arn),
            );
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-signalmap-monitordeployment.html
    pub struct MonitorDeployment_ {
        pub details_uri: Option<crate::value::ExpString>,
        pub error_message: Option<crate::value::ExpString>,
        pub status: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_SignalMap_MonitorDeployment {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::SignalMap.MonitorDeployment"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_SignalMap_MonitorDeployment as MonitorDeployment;
    impl crate::value::ToValue for MonitorDeployment_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.details_uri {
                properties.insert(
                    "DetailsUri".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.error_message {
                properties.insert(
                    "ErrorMessage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Status".to_string(),
                crate::value::ToValue::to_value(&self.status),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-signalmap-successfulmonitordeployment.html
    pub struct SuccessfulMonitorDeployment_ {
        pub details_uri: crate::value::ExpString,
        pub status: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_medialive_SignalMap_SuccessfulMonitorDeployment {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MediaLive::SignalMap.SuccessfulMonitorDeployment"
            $($field $value)*)
        };
    }
    pub use crate::__aws_medialive_SignalMap_SuccessfulMonitorDeployment as SuccessfulMonitorDeployment;
    impl crate::value::ToValue for SuccessfulMonitorDeployment_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DetailsUri".to_string(),
                crate::value::ToValue::to_value(&self.details_uri),
            );
            properties.insert(
                "Status".to_string(),
                crate::value::ToValue::to_value(&self.status),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-channel.html
pub struct Channel_ {
    pub anywhere_settings: Option<super::medialive::channel::AnywhereSettings_>,
    pub cdi_input_specification: Option<super::medialive::channel::CdiInputSpecification_>,
    pub channel_class: Option<crate::value::ExpString>,
    pub channel_engine_version: Option<super::medialive::channel::ChannelEngineVersionRequest_>,
    pub destinations: Option<Vec<super::medialive::channel::OutputDestination_>>,
    pub dry_run: Option<crate::value::ExpBool>,
    pub encoder_settings: Option<super::medialive::channel::EncoderSettings_>,
    pub input_attachments: Option<Vec<super::medialive::channel::InputAttachment_>>,
    pub input_specification: Option<super::medialive::channel::InputSpecification_>,
    pub log_level: Option<crate::value::ExpString>,
    pub maintenance: Option<super::medialive::channel::MaintenanceCreateSettings_>,
    pub name: Option<crate::value::ExpString>,
    pub role_arn: Option<crate::value::ExpString>,
    pub tags: Option<serde_json::Value>,
    pub vpc: Option<super::medialive::channel::VpcOutputSettings_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_medialive_Channel {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::MediaLive::Channel"
        $($field $value)*)
    };
}
pub use crate::__aws_medialive_Channel as Channel;
impl crate::template::ToResource for Channel_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaLive"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Channel"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.anywhere_settings {
            properties.insert(
                "AnywhereSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cdi_input_specification {
            properties.insert(
                "CdiInputSpecification".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.channel_class {
            properties.insert(
                "ChannelClass".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.channel_engine_version {
            properties.insert(
                "ChannelEngineVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.destinations {
            properties.insert(
                "Destinations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.dry_run {
            properties.insert("DryRun".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.encoder_settings {
            properties.insert(
                "EncoderSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.input_attachments {
            properties.insert(
                "InputAttachments".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.input_specification {
            properties.insert(
                "InputSpecification".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.log_level {
            properties.insert(
                "LogLevel".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.maintenance {
            properties.insert(
                "Maintenance".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.role_arn {
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.vpc {
            properties.insert("Vpc".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-channelplacementgroup.html
pub struct ChannelPlacementGroup_ {
    pub cluster_id: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub nodes: Option<Vec<crate::value::ExpString>>,
    pub tags: Option<Vec<super::medialive::channelplacementgroup::Tags_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_medialive_ChannelPlacementGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::MediaLive::ChannelPlacementGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_medialive_ChannelPlacementGroup as ChannelPlacementGroup;
impl crate::template::ToResource for ChannelPlacementGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaLive"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ChannelPlacementGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.cluster_id {
            properties.insert(
                "ClusterId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.nodes {
            properties.insert("Nodes".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-cloudwatchalarmtemplate.html
pub struct CloudWatchAlarmTemplate_ {
    pub comparison_operator: crate::value::ExpString,
    pub datapoints_to_alarm: Option<f64>,
    pub description: Option<crate::value::ExpString>,
    pub evaluation_periods: f64,
    pub group_identifier: Option<crate::value::ExpString>,
    pub metric_name: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub period: f64,
    pub statistic: crate::value::ExpString,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub target_resource_type: crate::value::ExpString,
    pub threshold: f64,
    pub treat_missing_data: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_medialive_CloudWatchAlarmTemplate {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::MediaLive::CloudWatchAlarmTemplate"
        $($field $value)*)
    };
}
pub use crate::__aws_medialive_CloudWatchAlarmTemplate as CloudWatchAlarmTemplate;
impl crate::template::ToResource for CloudWatchAlarmTemplate_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaLive"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("CloudWatchAlarmTemplate"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ComparisonOperator".to_string(),
            crate::value::ToValue::to_value(&self.comparison_operator),
        );
        if let Some(ref value) = self.datapoints_to_alarm {
            properties.insert(
                "DatapointsToAlarm".to_string(),
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
            "EvaluationPeriods".to_string(),
            crate::value::ToValue::to_value(&self.evaluation_periods),
        );
        if let Some(ref value) = self.group_identifier {
            properties.insert(
                "GroupIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "MetricName".to_string(),
            crate::value::ToValue::to_value(&self.metric_name),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "Period".to_string(),
            crate::value::ToValue::to_value(&self.period),
        );
        properties.insert(
            "Statistic".to_string(),
            crate::value::ToValue::to_value(&self.statistic),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TargetResourceType".to_string(),
            crate::value::ToValue::to_value(&self.target_resource_type),
        );
        properties.insert(
            "Threshold".to_string(),
            crate::value::ToValue::to_value(&self.threshold),
        );
        properties.insert(
            "TreatMissingData".to_string(),
            crate::value::ToValue::to_value(&self.treat_missing_data),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-cloudwatchalarmtemplategroup.html
pub struct CloudWatchAlarmTemplateGroup_ {
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_medialive_CloudWatchAlarmTemplateGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::MediaLive::CloudWatchAlarmTemplateGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_medialive_CloudWatchAlarmTemplateGroup as CloudWatchAlarmTemplateGroup;
impl crate::template::ToResource for CloudWatchAlarmTemplateGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaLive"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "CloudWatchAlarmTemplateGroup",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-cluster.html
pub struct Cluster_ {
    pub cluster_type: Option<crate::value::ExpString>,
    pub instance_role_arn: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub network_settings: Option<super::medialive::cluster::ClusterNetworkSettings_>,
    pub tags: Option<Vec<super::medialive::cluster::Tags_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_medialive_Cluster {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::MediaLive::Cluster"
        $($field $value)*)
    };
}
pub use crate::__aws_medialive_Cluster as Cluster;
impl crate::template::ToResource for Cluster_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaLive"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Cluster"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.cluster_type {
            properties.insert(
                "ClusterType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.instance_role_arn {
            properties.insert(
                "InstanceRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.network_settings {
            properties.insert(
                "NetworkSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-eventbridgeruletemplate.html
pub struct EventBridgeRuleTemplate_ {
    pub description: Option<crate::value::ExpString>,
    pub event_targets:
        Option<Vec<super::medialive::eventbridgeruletemplate::EventBridgeRuleTemplateTarget_>>,
    pub event_type: crate::value::ExpString,
    pub group_identifier: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_medialive_EventBridgeRuleTemplate {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::MediaLive::EventBridgeRuleTemplate"
        $($field $value)*)
    };
}
pub use crate::__aws_medialive_EventBridgeRuleTemplate as EventBridgeRuleTemplate;
impl crate::template::ToResource for EventBridgeRuleTemplate_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaLive"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("EventBridgeRuleTemplate"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.event_targets {
            properties.insert(
                "EventTargets".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "EventType".to_string(),
            crate::value::ToValue::to_value(&self.event_type),
        );
        if let Some(ref value) = self.group_identifier {
            properties.insert(
                "GroupIdentifier".to_string(),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-eventbridgeruletemplategroup.html
pub struct EventBridgeRuleTemplateGroup_ {
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_medialive_EventBridgeRuleTemplateGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::MediaLive::EventBridgeRuleTemplateGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_medialive_EventBridgeRuleTemplateGroup as EventBridgeRuleTemplateGroup;
impl crate::template::ToResource for EventBridgeRuleTemplateGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaLive"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "EventBridgeRuleTemplateGroup",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-input.html
pub struct Input_ {
    pub destinations: Option<Vec<super::medialive::input::InputDestinationRequest_>>,
    pub input_devices: Option<Vec<super::medialive::input::InputDeviceSettings_>>,
    pub input_network_location: Option<crate::value::ExpString>,
    pub input_security_groups: Option<Vec<crate::value::ExpString>>,
    pub media_connect_flows: Option<Vec<super::medialive::input::MediaConnectFlowRequest_>>,
    pub multicast_settings: Option<super::medialive::input::MulticastSettingsCreateRequest_>,
    pub name: Option<crate::value::ExpString>,
    pub role_arn: Option<crate::value::ExpString>,
    pub sdi_sources: Option<Vec<crate::value::ExpString>>,
    pub smpte2110_receiver_group_settings:
        Option<super::medialive::input::Smpte2110ReceiverGroupSettings_>,
    pub sources: Option<Vec<super::medialive::input::InputSourceRequest_>>,
    pub srt_settings: Option<super::medialive::input::SrtSettingsRequest_>,
    pub tags: Option<serde_json::Value>,
    pub r#type: Option<crate::value::ExpString>,
    pub vpc: Option<super::medialive::input::InputVpcRequest_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_medialive_Input {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::MediaLive::Input" $($field
        $value)*)
    };
}
pub use crate::__aws_medialive_Input as Input;
impl crate::template::ToResource for Input_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaLive"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Input"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.destinations {
            properties.insert(
                "Destinations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.input_devices {
            properties.insert(
                "InputDevices".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.input_network_location {
            properties.insert(
                "InputNetworkLocation".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.input_security_groups {
            properties.insert(
                "InputSecurityGroups".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.media_connect_flows {
            properties.insert(
                "MediaConnectFlows".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.multicast_settings {
            properties.insert(
                "MulticastSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.role_arn {
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sdi_sources {
            properties.insert(
                "SdiSources".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.smpte2110_receiver_group_settings {
            properties.insert(
                "Smpte2110ReceiverGroupSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sources {
            properties.insert(
                "Sources".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.srt_settings {
            properties.insert(
                "SrtSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.r#type {
            properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.vpc {
            properties.insert("Vpc".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-inputsecuritygroup.html
pub struct InputSecurityGroup_ {
    pub tags: Option<serde_json::Value>,
    pub whitelist_rules: Option<Vec<super::medialive::inputsecuritygroup::InputWhitelistRuleCidr_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_medialive_InputSecurityGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::MediaLive::InputSecurityGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_medialive_InputSecurityGroup as InputSecurityGroup;
impl crate::template::ToResource for InputSecurityGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaLive"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("InputSecurityGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.whitelist_rules {
            properties.insert(
                "WhitelistRules".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-multiplex.html
pub struct Multiplex_ {
    pub availability_zones: Vec<crate::value::ExpString>,
    pub destinations: Option<Vec<super::medialive::multiplex::MultiplexOutputDestination_>>,
    pub multiplex_settings: super::medialive::multiplex::MultiplexSettings_,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<super::medialive::multiplex::Tags_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_medialive_Multiplex {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::MediaLive::Multiplex"
        $($field $value)*)
    };
}
pub use crate::__aws_medialive_Multiplex as Multiplex;
impl crate::template::ToResource for Multiplex_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaLive"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Multiplex"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AvailabilityZones".to_string(),
            crate::value::ToValue::to_value(&self.availability_zones),
        );
        if let Some(ref value) = self.destinations {
            properties.insert(
                "Destinations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "MultiplexSettings".to_string(),
            crate::value::ToValue::to_value(&self.multiplex_settings),
        );
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-multiplexprogram.html
pub struct Multiplexprogram_ {
    pub multiplex_id: Option<crate::value::ExpString>,
    pub multiplex_program_settings:
        Option<super::medialive::multiplexprogram::MultiplexProgramSettings_>,
    pub packet_identifiers_map:
        Option<super::medialive::multiplexprogram::MultiplexProgramPacketIdentifiersMap_>,
    pub pipeline_details:
        Option<Vec<super::medialive::multiplexprogram::MultiplexProgramPipelineDetail_>>,
    pub preferred_channel_pipeline: Option<crate::value::ExpString>,
    pub program_name: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_medialive_Multiplexprogram {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::MediaLive::Multiplexprogram"
        $($field $value)*)
    };
}
pub use crate::__aws_medialive_Multiplexprogram as Multiplexprogram;
impl crate::template::ToResource for Multiplexprogram_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaLive"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Multiplexprogram"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.multiplex_id {
            properties.insert(
                "MultiplexId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.multiplex_program_settings {
            properties.insert(
                "MultiplexProgramSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.packet_identifiers_map {
            properties.insert(
                "PacketIdentifiersMap".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.pipeline_details {
            properties.insert(
                "PipelineDetails".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.preferred_channel_pipeline {
            properties.insert(
                "PreferredChannelPipeline".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.program_name {
            properties.insert(
                "ProgramName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-network.html
pub struct Network_ {
    pub ip_pools: Vec<super::medialive::network::IpPool_>,
    pub name: crate::value::ExpString,
    pub routes: Option<Vec<super::medialive::network::Route_>>,
    pub tags: Option<Vec<super::medialive::network::Tags_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_medialive_Network {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::MediaLive::Network"
        $($field $value)*)
    };
}
pub use crate::__aws_medialive_Network as Network;
impl crate::template::ToResource for Network_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaLive"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Network"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "IpPools".to_string(),
            crate::value::ToValue::to_value(&self.ip_pools),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.routes {
            properties.insert("Routes".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-sdisource.html
pub struct SdiSource_ {
    pub mode: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<super::medialive::sdisource::Tags_>>,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_medialive_SdiSource {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::MediaLive::SdiSource"
        $($field $value)*)
    };
}
pub use crate::__aws_medialive_SdiSource as SdiSource;
impl crate::template::ToResource for SdiSource_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaLive"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SdiSource"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.mode {
            properties.insert("Mode".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-signalmap.html
pub struct SignalMap_ {
    pub cloud_watch_alarm_template_group_identifiers: Option<Vec<crate::value::ExpString>>,
    pub description: Option<crate::value::ExpString>,
    pub discovery_entry_point_arn: crate::value::ExpString,
    pub event_bridge_rule_template_group_identifiers: Option<Vec<crate::value::ExpString>>,
    pub force_rediscovery: Option<crate::value::ExpBool>,
    pub name: crate::value::ExpString,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_medialive_SignalMap {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::MediaLive::SignalMap"
        $($field $value)*)
    };
}
pub use crate::__aws_medialive_SignalMap as SignalMap;
impl crate::template::ToResource for SignalMap_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaLive"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SignalMap"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.cloud_watch_alarm_template_group_identifiers {
            properties.insert(
                "CloudWatchAlarmTemplateGroupIdentifiers".to_string(),
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
            "DiscoveryEntryPointArn".to_string(),
            crate::value::ToValue::to_value(&self.discovery_entry_point_arn),
        );
        if let Some(ref value) = self.event_bridge_rule_template_group_identifiers {
            properties.insert(
                "EventBridgeRuleTemplateGroupIdentifiers".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.force_rediscovery {
            properties.insert(
                "ForceRediscovery".to_string(),
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
