pub mod config {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-antennadownlinkconfig.html>
    pub struct AntennaDownlinkConfig_ {
        pub spectrum_config: Option<Box<SpectrumConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_Config_AntennaDownlinkConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::Config.AntennaDownlinkConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_Config_AntennaDownlinkConfig as AntennaDownlinkConfig;
    impl crate::value::ToValue for AntennaDownlinkConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.spectrum_config {
                properties.insert(
                    "SpectrumConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-antennadownlinkdemoddecodeconfig.html>
    pub struct AntennaDownlinkDemodDecodeConfig_ {
        pub decode_config: Option<Box<DecodeConfig_>>,
        pub demodulation_config: Option<Box<DemodulationConfig_>>,
        pub spectrum_config: Option<Box<SpectrumConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_Config_AntennaDownlinkDemodDecodeConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::Config.AntennaDownlinkDemodDecodeConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_Config_AntennaDownlinkDemodDecodeConfig as AntennaDownlinkDemodDecodeConfig;
    impl crate::value::ToValue for AntennaDownlinkDemodDecodeConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.decode_config {
                properties.insert(
                    "DecodeConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.demodulation_config {
                properties.insert(
                    "DemodulationConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.spectrum_config {
                properties.insert(
                    "SpectrumConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-antennauplinkconfig.html>
    pub struct AntennaUplinkConfig_ {
        pub spectrum_config: Option<Box<UplinkSpectrumConfig_>>,
        pub target_eirp: Option<Box<Eirp_>>,
        pub transmit_disabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_Config_AntennaUplinkConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::Config.AntennaUplinkConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_Config_AntennaUplinkConfig as AntennaUplinkConfig;
    impl crate::value::ToValue for AntennaUplinkConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.spectrum_config {
                properties.insert(
                    "SpectrumConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_eirp {
                properties.insert(
                    "TargetEirp".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.transmit_disabled {
                properties.insert(
                    "TransmitDisabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-configdata.html>
    pub struct ConfigData_ {
        pub antenna_downlink_config: Option<Box<AntennaDownlinkConfig_>>,
        pub antenna_downlink_demod_decode_config: Option<Box<AntennaDownlinkDemodDecodeConfig_>>,
        pub antenna_uplink_config: Option<Box<AntennaUplinkConfig_>>,
        pub dataflow_endpoint_config: Option<Box<DataflowEndpointConfig_>>,
        pub s3_recording_config: Option<Box<S3RecordingConfig_>>,
        pub telemetry_sink_config: Option<Box<TelemetrySinkConfig_>>,
        pub tracking_config: Option<Box<TrackingConfig_>>,
        pub uplink_echo_config: Option<Box<UplinkEchoConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_Config_ConfigData {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::Config.ConfigData"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_Config_ConfigData as ConfigData;
    impl crate::value::ToValue for ConfigData_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.antenna_downlink_config {
                properties.insert(
                    "AntennaDownlinkConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.antenna_downlink_demod_decode_config {
                properties.insert(
                    "AntennaDownlinkDemodDecodeConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.antenna_uplink_config {
                properties.insert(
                    "AntennaUplinkConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dataflow_endpoint_config {
                properties.insert(
                    "DataflowEndpointConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_recording_config {
                properties.insert(
                    "S3RecordingConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.telemetry_sink_config {
                properties.insert(
                    "TelemetrySinkConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tracking_config {
                properties.insert(
                    "TrackingConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.uplink_echo_config {
                properties.insert(
                    "UplinkEchoConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-dataflowendpointconfig.html>
    pub struct DataflowEndpointConfig_ {
        pub dataflow_endpoint_name: Option<crate::value::ExpString>,
        pub dataflow_endpoint_region: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_Config_DataflowEndpointConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::Config.DataflowEndpointConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_Config_DataflowEndpointConfig as DataflowEndpointConfig;
    impl crate::value::ToValue for DataflowEndpointConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dataflow_endpoint_name {
                properties.insert(
                    "DataflowEndpointName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dataflow_endpoint_region {
                properties.insert(
                    "DataflowEndpointRegion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-decodeconfig.html>
    pub struct DecodeConfig_ {
        pub unvalidated_json: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_Config_DecodeConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::Config.DecodeConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_Config_DecodeConfig as DecodeConfig;
    impl crate::value::ToValue for DecodeConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.unvalidated_json {
                properties.insert(
                    "UnvalidatedJSON".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-demodulationconfig.html>
    pub struct DemodulationConfig_ {
        pub unvalidated_json: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_Config_DemodulationConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::Config.DemodulationConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_Config_DemodulationConfig as DemodulationConfig;
    impl crate::value::ToValue for DemodulationConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.unvalidated_json {
                properties.insert(
                    "UnvalidatedJSON".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-eirp.html>
    pub struct Eirp_ {
        pub units: Option<crate::value::ExpString>,
        pub value: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_Config_Eirp {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::Config.Eirp"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_Config_Eirp as Eirp;
    impl crate::value::ToValue for Eirp_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.units {
                properties.insert("Units".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-frequency.html>
    pub struct Frequency_ {
        pub units: Option<crate::value::ExpString>,
        pub value: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_Config_Frequency {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::Config.Frequency"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_Config_Frequency as Frequency;
    impl crate::value::ToValue for Frequency_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.units {
                properties.insert("Units".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-frequencybandwidth.html>
    pub struct FrequencyBandwidth_ {
        pub units: Option<crate::value::ExpString>,
        pub value: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_Config_FrequencyBandwidth {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::Config.FrequencyBandwidth"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_Config_FrequencyBandwidth as FrequencyBandwidth;
    impl crate::value::ToValue for FrequencyBandwidth_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.units {
                properties.insert("Units".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-kinesisdatastreamdata.html>
    pub struct KinesisDataStreamData_ {
        pub kinesis_data_stream_arn: crate::value::ExpString,
        pub kinesis_role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_Config_KinesisDataStreamData {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::Config.KinesisDataStreamData"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_Config_KinesisDataStreamData as KinesisDataStreamData;
    impl crate::value::ToValue for KinesisDataStreamData_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "KinesisDataStreamArn".to_string(),
                crate::value::ToValue::to_value(&self.kinesis_data_stream_arn),
            );
            properties.insert(
                "KinesisRoleArn".to_string(),
                crate::value::ToValue::to_value(&self.kinesis_role_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-s3recordingconfig.html>
    pub struct S3RecordingConfig_ {
        pub bucket_arn: Option<crate::value::ExpString>,
        pub prefix: Option<crate::value::ExpString>,
        pub role_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_Config_S3RecordingConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::Config.S3RecordingConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_Config_S3RecordingConfig as S3RecordingConfig;
    impl crate::value::ToValue for S3RecordingConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bucket_arn {
                properties.insert(
                    "BucketArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.prefix {
                properties.insert("Prefix".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.role_arn {
                properties.insert(
                    "RoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-spectrumconfig.html>
    pub struct SpectrumConfig_ {
        pub bandwidth: Option<Box<FrequencyBandwidth_>>,
        pub center_frequency: Option<Box<Frequency_>>,
        pub polarization: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_Config_SpectrumConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::Config.SpectrumConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_Config_SpectrumConfig as SpectrumConfig;
    impl crate::value::ToValue for SpectrumConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bandwidth {
                properties.insert(
                    "Bandwidth".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.center_frequency {
                properties.insert(
                    "CenterFrequency".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.polarization {
                properties.insert(
                    "Polarization".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-telemetrysinkconfig.html>
    pub struct TelemetrySinkConfig_ {
        pub telemetry_sink_data: Box<TelemetrySinkData_>,
        pub telemetry_sink_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_Config_TelemetrySinkConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::Config.TelemetrySinkConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_Config_TelemetrySinkConfig as TelemetrySinkConfig;
    impl crate::value::ToValue for TelemetrySinkConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TelemetrySinkData".to_string(),
                crate::value::ToValue::to_value(&self.telemetry_sink_data),
            );
            properties.insert(
                "TelemetrySinkType".to_string(),
                crate::value::ToValue::to_value(&self.telemetry_sink_type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-telemetrysinkdata.html>
    pub struct TelemetrySinkData_ {
        pub kinesis_data_stream_data: Box<KinesisDataStreamData_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_Config_TelemetrySinkData {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::Config.TelemetrySinkData"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_Config_TelemetrySinkData as TelemetrySinkData;
    impl crate::value::ToValue for TelemetrySinkData_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "KinesisDataStreamData".to_string(),
                crate::value::ToValue::to_value(&self.kinesis_data_stream_data),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-trackingconfig.html>
    pub struct TrackingConfig_ {
        pub autotrack: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_Config_TrackingConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::Config.TrackingConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_Config_TrackingConfig as TrackingConfig;
    impl crate::value::ToValue for TrackingConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.autotrack {
                properties.insert(
                    "Autotrack".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-uplinkechoconfig.html>
    pub struct UplinkEchoConfig_ {
        pub antenna_uplink_config_arn: Option<crate::value::ExpString>,
        pub enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_Config_UplinkEchoConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::Config.UplinkEchoConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_Config_UplinkEchoConfig as UplinkEchoConfig;
    impl crate::value::ToValue for UplinkEchoConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.antenna_uplink_config_arn {
                properties.insert(
                    "AntennaUplinkConfigArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-uplinkspectrumconfig.html>
    pub struct UplinkSpectrumConfig_ {
        pub center_frequency: Option<Box<Frequency_>>,
        pub polarization: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_Config_UplinkSpectrumConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::Config.UplinkSpectrumConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_Config_UplinkSpectrumConfig as UplinkSpectrumConfig;
    impl crate::value::ToValue for UplinkSpectrumConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.center_frequency {
                properties.insert(
                    "CenterFrequency".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.polarization {
                properties.insert(
                    "Polarization".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod dataflowendpointgroup {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-dataflowendpointgroup-awsgroundstationagentendpoint.html>
    pub struct AwsGroundStationAgentEndpoint_ {
        pub agent_status: Option<crate::value::ExpString>,
        pub audit_results: Option<crate::value::ExpString>,
        pub egress_address: Option<Box<ConnectionDetails_>>,
        pub ingress_address: Option<Box<RangedConnectionDetails_>>,
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_DataflowEndpointGroup_AwsGroundStationAgentEndpoint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::DataflowEndpointGroup.AwsGroundStationAgentEndpoint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_DataflowEndpointGroup_AwsGroundStationAgentEndpoint as AwsGroundStationAgentEndpoint;
    impl crate::value::ToValue for AwsGroundStationAgentEndpoint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.agent_status {
                properties.insert(
                    "AgentStatus".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.audit_results {
                properties.insert(
                    "AuditResults".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.egress_address {
                properties.insert(
                    "EgressAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ingress_address {
                properties.insert(
                    "IngressAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-dataflowendpointgroup-connectiondetails.html>
    pub struct ConnectionDetails_ {
        pub mtu: Option<i32>,
        pub socket_address: Option<Box<SocketAddress_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_DataflowEndpointGroup_ConnectionDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::DataflowEndpointGroup.ConnectionDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_DataflowEndpointGroup_ConnectionDetails as ConnectionDetails;
    impl crate::value::ToValue for ConnectionDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.mtu {
                properties.insert("Mtu".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.socket_address {
                properties.insert(
                    "SocketAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-dataflowendpointgroup-dataflowendpoint.html>
    pub struct DataflowEndpoint_ {
        pub address: Option<Box<SocketAddress_>>,
        pub mtu: Option<i32>,
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_DataflowEndpointGroup_DataflowEndpoint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::DataflowEndpointGroup.DataflowEndpoint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_DataflowEndpointGroup_DataflowEndpoint as DataflowEndpoint;
    impl crate::value::ToValue for DataflowEndpoint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.address {
                properties.insert(
                    "Address".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mtu {
                properties.insert("Mtu".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-dataflowendpointgroup-endpointdetails.html>
    pub struct EndpointDetails_ {
        pub aws_ground_station_agent_endpoint: Option<Box<AwsGroundStationAgentEndpoint_>>,
        pub endpoint: Option<Box<DataflowEndpoint_>>,
        pub security_details: Option<Box<SecurityDetails_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_DataflowEndpointGroup_EndpointDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::DataflowEndpointGroup.EndpointDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_DataflowEndpointGroup_EndpointDetails as EndpointDetails;
    impl crate::value::ToValue for EndpointDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.aws_ground_station_agent_endpoint {
                properties.insert(
                    "AwsGroundStationAgentEndpoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.endpoint {
                properties.insert(
                    "Endpoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.security_details {
                properties.insert(
                    "SecurityDetails".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-dataflowendpointgroup-integerrange.html>
    pub struct IntegerRange_ {
        pub maximum: Option<i32>,
        pub minimum: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_DataflowEndpointGroup_IntegerRange {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::DataflowEndpointGroup.IntegerRange"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_DataflowEndpointGroup_IntegerRange as IntegerRange;
    impl crate::value::ToValue for IntegerRange_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.maximum {
                properties.insert(
                    "Maximum".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.minimum {
                properties.insert(
                    "Minimum".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-dataflowendpointgroup-rangedconnectiondetails.html>
    pub struct RangedConnectionDetails_ {
        pub mtu: Option<i32>,
        pub socket_address: Option<Box<RangedSocketAddress_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_DataflowEndpointGroup_RangedConnectionDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::DataflowEndpointGroup.RangedConnectionDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_DataflowEndpointGroup_RangedConnectionDetails as RangedConnectionDetails;
    impl crate::value::ToValue for RangedConnectionDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.mtu {
                properties.insert("Mtu".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.socket_address {
                properties.insert(
                    "SocketAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-dataflowendpointgroup-rangedsocketaddress.html>
    pub struct RangedSocketAddress_ {
        pub name: Option<crate::value::ExpString>,
        pub port_range: Option<Box<IntegerRange_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_DataflowEndpointGroup_RangedSocketAddress {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::DataflowEndpointGroup.RangedSocketAddress"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_DataflowEndpointGroup_RangedSocketAddress as RangedSocketAddress;
    impl crate::value::ToValue for RangedSocketAddress_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.port_range {
                properties.insert(
                    "PortRange".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-dataflowendpointgroup-securitydetails.html>
    pub struct SecurityDetails_ {
        pub role_arn: Option<crate::value::ExpString>,
        pub security_group_ids: Option<Vec<crate::value::ExpString>>,
        pub subnet_ids: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_DataflowEndpointGroup_SecurityDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::DataflowEndpointGroup.SecurityDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_DataflowEndpointGroup_SecurityDetails as SecurityDetails;
    impl crate::value::ToValue for SecurityDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.role_arn {
                properties.insert(
                    "RoleArn".to_string(),
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-dataflowendpointgroup-socketaddress.html>
    pub struct SocketAddress_ {
        pub name: Option<crate::value::ExpString>,
        pub port: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_DataflowEndpointGroup_SocketAddress {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::DataflowEndpointGroup.SocketAddress"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_DataflowEndpointGroup_SocketAddress as SocketAddress;
    impl crate::value::ToValue for SocketAddress_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.port {
                properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod dataflowendpointgroupv2 {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-dataflowendpointgroupv2-connectiondetails.html>
    pub struct ConnectionDetails_ {
        pub mtu: Option<i32>,
        pub socket_address: Box<SocketAddress_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_DataflowEndpointGroupV2_ConnectionDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::DataflowEndpointGroupV2.ConnectionDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_DataflowEndpointGroupV2_ConnectionDetails as ConnectionDetails;
    impl crate::value::ToValue for ConnectionDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.mtu {
                properties.insert("Mtu".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "SocketAddress".to_string(),
                crate::value::ToValue::to_value(&self.socket_address),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-dataflowendpointgroupv2-createendpointdetails.html>
    pub struct CreateEndpointDetails_ {
        pub downlink_aws_ground_station_agent_endpoint:
            Option<Box<DownlinkAwsGroundStationAgentEndpoint_>>,
        pub uplink_aws_ground_station_agent_endpoint:
            Option<Box<UplinkAwsGroundStationAgentEndpoint_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_DataflowEndpointGroupV2_CreateEndpointDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::DataflowEndpointGroupV2.CreateEndpointDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_DataflowEndpointGroupV2_CreateEndpointDetails as CreateEndpointDetails;
    impl crate::value::ToValue for CreateEndpointDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.downlink_aws_ground_station_agent_endpoint {
                properties.insert(
                    "DownlinkAwsGroundStationAgentEndpoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.uplink_aws_ground_station_agent_endpoint {
                properties.insert(
                    "UplinkAwsGroundStationAgentEndpoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-dataflowendpointgroupv2-downlinkawsgroundstationagentendpoint.html>
    pub struct DownlinkAwsGroundStationAgentEndpoint_ {
        pub dataflow_details: Box<DownlinkDataflowDetails_>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_DataflowEndpointGroupV2_DownlinkAwsGroundStationAgentEndpoint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::DataflowEndpointGroupV2.DownlinkAwsGroundStationAgentEndpoint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_DataflowEndpointGroupV2_DownlinkAwsGroundStationAgentEndpoint as DownlinkAwsGroundStationAgentEndpoint;
    impl crate::value::ToValue for DownlinkAwsGroundStationAgentEndpoint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DataflowDetails".to_string(),
                crate::value::ToValue::to_value(&self.dataflow_details),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-dataflowendpointgroupv2-downlinkawsgroundstationagentendpointdetails.html>
    pub struct DownlinkAwsGroundStationAgentEndpointDetails_ {
        pub agent_status: Option<crate::value::ExpString>,
        pub audit_results: Option<crate::value::ExpString>,
        pub dataflow_details: Box<DownlinkDataflowDetails_>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_DataflowEndpointGroupV2_DownlinkAwsGroundStationAgentEndpointDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::DataflowEndpointGroupV2.DownlinkAwsGroundStationAgentEndpointDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_DataflowEndpointGroupV2_DownlinkAwsGroundStationAgentEndpointDetails as DownlinkAwsGroundStationAgentEndpointDetails;
    impl crate::value::ToValue for DownlinkAwsGroundStationAgentEndpointDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.agent_status {
                properties.insert(
                    "AgentStatus".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.audit_results {
                properties.insert(
                    "AuditResults".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DataflowDetails".to_string(),
                crate::value::ToValue::to_value(&self.dataflow_details),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-dataflowendpointgroupv2-downlinkconnectiondetails.html>
    pub struct DownlinkConnectionDetails_ {
        pub agent_ip_and_port_address: Box<RangedConnectionDetails_>,
        pub egress_address_and_port: Box<ConnectionDetails_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_DataflowEndpointGroupV2_DownlinkConnectionDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::DataflowEndpointGroupV2.DownlinkConnectionDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_DataflowEndpointGroupV2_DownlinkConnectionDetails as DownlinkConnectionDetails;
    impl crate::value::ToValue for DownlinkConnectionDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AgentIpAndPortAddress".to_string(),
                crate::value::ToValue::to_value(&self.agent_ip_and_port_address),
            );
            properties.insert(
                "EgressAddressAndPort".to_string(),
                crate::value::ToValue::to_value(&self.egress_address_and_port),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-dataflowendpointgroupv2-downlinkdataflowdetails.html>
    pub struct DownlinkDataflowDetails_ {
        pub agent_connection_details: Box<DownlinkConnectionDetails_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_DataflowEndpointGroupV2_DownlinkDataflowDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::DataflowEndpointGroupV2.DownlinkDataflowDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_DataflowEndpointGroupV2_DownlinkDataflowDetails as DownlinkDataflowDetails;
    impl crate::value::ToValue for DownlinkDataflowDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AgentConnectionDetails".to_string(),
                crate::value::ToValue::to_value(&self.agent_connection_details),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-dataflowendpointgroupv2-endpointdetails.html>
    pub struct EndpointDetails_ {
        pub downlink_aws_ground_station_agent_endpoint:
            Option<Box<DownlinkAwsGroundStationAgentEndpointDetails_>>,
        pub uplink_aws_ground_station_agent_endpoint:
            Option<Box<UplinkAwsGroundStationAgentEndpointDetails_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_DataflowEndpointGroupV2_EndpointDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::DataflowEndpointGroupV2.EndpointDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_DataflowEndpointGroupV2_EndpointDetails as EndpointDetails;
    impl crate::value::ToValue for EndpointDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.downlink_aws_ground_station_agent_endpoint {
                properties.insert(
                    "DownlinkAwsGroundStationAgentEndpoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.uplink_aws_ground_station_agent_endpoint {
                properties.insert(
                    "UplinkAwsGroundStationAgentEndpoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-dataflowendpointgroupv2-integerrange.html>
    pub struct IntegerRange_ {
        pub maximum: i32,
        pub minimum: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_DataflowEndpointGroupV2_IntegerRange {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::DataflowEndpointGroupV2.IntegerRange"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_DataflowEndpointGroupV2_IntegerRange as IntegerRange;
    impl crate::value::ToValue for IntegerRange_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Maximum".to_string(),
                crate::value::ToValue::to_value(&self.maximum),
            );
            properties.insert(
                "Minimum".to_string(),
                crate::value::ToValue::to_value(&self.minimum),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-dataflowendpointgroupv2-rangedconnectiondetails.html>
    pub struct RangedConnectionDetails_ {
        pub mtu: Option<i32>,
        pub socket_address: Box<RangedSocketAddress_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_DataflowEndpointGroupV2_RangedConnectionDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::DataflowEndpointGroupV2.RangedConnectionDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_DataflowEndpointGroupV2_RangedConnectionDetails as RangedConnectionDetails;
    impl crate::value::ToValue for RangedConnectionDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.mtu {
                properties.insert("Mtu".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "SocketAddress".to_string(),
                crate::value::ToValue::to_value(&self.socket_address),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-dataflowendpointgroupv2-rangedsocketaddress.html>
    pub struct RangedSocketAddress_ {
        pub name: crate::value::ExpString,
        pub port_range: Box<IntegerRange_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_DataflowEndpointGroupV2_RangedSocketAddress {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::DataflowEndpointGroupV2.RangedSocketAddress"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_DataflowEndpointGroupV2_RangedSocketAddress as RangedSocketAddress;
    impl crate::value::ToValue for RangedSocketAddress_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "PortRange".to_string(),
                crate::value::ToValue::to_value(&self.port_range),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-dataflowendpointgroupv2-socketaddress.html>
    pub struct SocketAddress_ {
        pub name: crate::value::ExpString,
        pub port: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_DataflowEndpointGroupV2_SocketAddress {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::DataflowEndpointGroupV2.SocketAddress"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_DataflowEndpointGroupV2_SocketAddress as SocketAddress;
    impl crate::value::ToValue for SocketAddress_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "Port".to_string(),
                crate::value::ToValue::to_value(&self.port),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-dataflowendpointgroupv2-uplinkawsgroundstationagentendpoint.html>
    pub struct UplinkAwsGroundStationAgentEndpoint_ {
        pub dataflow_details: Box<UplinkDataflowDetails_>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_DataflowEndpointGroupV2_UplinkAwsGroundStationAgentEndpoint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::DataflowEndpointGroupV2.UplinkAwsGroundStationAgentEndpoint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_DataflowEndpointGroupV2_UplinkAwsGroundStationAgentEndpoint as UplinkAwsGroundStationAgentEndpoint;
    impl crate::value::ToValue for UplinkAwsGroundStationAgentEndpoint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DataflowDetails".to_string(),
                crate::value::ToValue::to_value(&self.dataflow_details),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-dataflowendpointgroupv2-uplinkawsgroundstationagentendpointdetails.html>
    pub struct UplinkAwsGroundStationAgentEndpointDetails_ {
        pub agent_status: Option<crate::value::ExpString>,
        pub audit_results: Option<crate::value::ExpString>,
        pub dataflow_details: Box<UplinkDataflowDetails_>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_DataflowEndpointGroupV2_UplinkAwsGroundStationAgentEndpointDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::DataflowEndpointGroupV2.UplinkAwsGroundStationAgentEndpointDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_DataflowEndpointGroupV2_UplinkAwsGroundStationAgentEndpointDetails as UplinkAwsGroundStationAgentEndpointDetails;
    impl crate::value::ToValue for UplinkAwsGroundStationAgentEndpointDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.agent_status {
                properties.insert(
                    "AgentStatus".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.audit_results {
                properties.insert(
                    "AuditResults".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DataflowDetails".to_string(),
                crate::value::ToValue::to_value(&self.dataflow_details),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-dataflowendpointgroupv2-uplinkconnectiondetails.html>
    pub struct UplinkConnectionDetails_ {
        pub agent_ip_and_port_address: Box<RangedConnectionDetails_>,
        pub ingress_address_and_port: Box<ConnectionDetails_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_DataflowEndpointGroupV2_UplinkConnectionDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::DataflowEndpointGroupV2.UplinkConnectionDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_DataflowEndpointGroupV2_UplinkConnectionDetails as UplinkConnectionDetails;
    impl crate::value::ToValue for UplinkConnectionDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AgentIpAndPortAddress".to_string(),
                crate::value::ToValue::to_value(&self.agent_ip_and_port_address),
            );
            properties.insert(
                "IngressAddressAndPort".to_string(),
                crate::value::ToValue::to_value(&self.ingress_address_and_port),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-dataflowendpointgroupv2-uplinkdataflowdetails.html>
    pub struct UplinkDataflowDetails_ {
        pub agent_connection_details: Box<UplinkConnectionDetails_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_DataflowEndpointGroupV2_UplinkDataflowDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::DataflowEndpointGroupV2.UplinkDataflowDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_DataflowEndpointGroupV2_UplinkDataflowDetails as UplinkDataflowDetails;
    impl crate::value::ToValue for UplinkDataflowDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AgentConnectionDetails".to_string(),
                crate::value::ToValue::to_value(&self.agent_connection_details),
            );
            properties.into()
        }
    }
}
pub mod missionprofile {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-missionprofile-dataflowedge.html>
    pub struct DataflowEdge_ {
        pub destination: Option<crate::value::ExpString>,
        pub source: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_MissionProfile_DataflowEdge {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::MissionProfile.DataflowEdge"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_MissionProfile_DataflowEdge as DataflowEdge;
    impl crate::value::ToValue for DataflowEdge_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.destination {
                properties.insert(
                    "Destination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source {
                properties.insert("Source".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-missionprofile-streamskmskey.html>
    pub struct StreamsKmsKey_ {
        pub kms_alias_arn: Option<crate::value::ExpString>,
        pub kms_alias_name: Option<crate::value::ExpString>,
        pub kms_key_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_groundstation_MissionProfile_StreamsKmsKey {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GroundStation::MissionProfile.StreamsKmsKey"
            $($field $value)*)
        };
    }
    pub use crate::__aws_groundstation_MissionProfile_StreamsKmsKey as StreamsKmsKey;
    impl crate::value::ToValue for StreamsKmsKey_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_alias_arn {
                properties.insert(
                    "KmsAliasArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kms_alias_name {
                properties.insert(
                    "KmsAliasName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kms_key_arn {
                properties.insert(
                    "KmsKeyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-groundstation-config.html>
pub struct Config_ {
    pub config_data: super::groundstation::config::ConfigData_,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_groundstation_Config {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::GroundStation::Config"
        $($field $value)*)
    };
}
pub use crate::__aws_groundstation_Config as Config;
impl crate::template::ToResource for Config_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("GroundStation"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Config"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ConfigData".to_string(),
            crate::value::ToValue::to_value(&self.config_data),
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
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-groundstation-dataflowendpointgroup.html>
pub struct DataflowEndpointGroup_ {
    pub contact_post_pass_duration_seconds: Option<i32>,
    pub contact_pre_pass_duration_seconds: Option<i32>,
    pub endpoint_details: Vec<super::groundstation::dataflowendpointgroup::EndpointDetails_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_groundstation_DataflowEndpointGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::GroundStation::DataflowEndpointGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_groundstation_DataflowEndpointGroup as DataflowEndpointGroup;
impl crate::template::ToResource for DataflowEndpointGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("GroundStation"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DataflowEndpointGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.contact_post_pass_duration_seconds {
            properties.insert(
                "ContactPostPassDurationSeconds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.contact_pre_pass_duration_seconds {
            properties.insert(
                "ContactPrePassDurationSeconds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "EndpointDetails".to_string(),
            crate::value::ToValue::to_value(&self.endpoint_details),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-groundstation-dataflowendpointgroupv2.html>
pub struct DataflowEndpointGroupV2_ {
    pub contact_post_pass_duration_seconds: Option<i32>,
    pub contact_pre_pass_duration_seconds: Option<i32>,
    pub endpoints:
        Option<Vec<super::groundstation::dataflowendpointgroupv2::CreateEndpointDetails_>>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_groundstation_DataflowEndpointGroupV2 {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::GroundStation::DataflowEndpointGroupV2"
        $($field $value)*)
    };
}
pub use crate::__aws_groundstation_DataflowEndpointGroupV2 as DataflowEndpointGroupV2;
impl crate::template::ToResource for DataflowEndpointGroupV2_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("GroundStation"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DataflowEndpointGroupV2"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.contact_post_pass_duration_seconds {
            properties.insert(
                "ContactPostPassDurationSeconds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.contact_pre_pass_duration_seconds {
            properties.insert(
                "ContactPrePassDurationSeconds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.endpoints {
            properties.insert(
                "Endpoints".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-groundstation-missionprofile.html>
pub struct MissionProfile_ {
    pub contact_post_pass_duration_seconds: Option<i32>,
    pub contact_pre_pass_duration_seconds: Option<i32>,
    pub dataflow_edges: Vec<super::groundstation::missionprofile::DataflowEdge_>,
    pub minimum_viable_contact_duration_seconds: i32,
    pub name: crate::value::ExpString,
    pub streams_kms_key: Option<super::groundstation::missionprofile::StreamsKmsKey_>,
    pub streams_kms_role: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub telemetry_sink_config_arn: Option<crate::value::ExpString>,
    pub tracking_config_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_groundstation_MissionProfile {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::GroundStation::MissionProfile"
        $($field $value)*)
    };
}
pub use crate::__aws_groundstation_MissionProfile as MissionProfile;
impl crate::template::ToResource for MissionProfile_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("GroundStation"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("MissionProfile"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.contact_post_pass_duration_seconds {
            properties.insert(
                "ContactPostPassDurationSeconds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.contact_pre_pass_duration_seconds {
            properties.insert(
                "ContactPrePassDurationSeconds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DataflowEdges".to_string(),
            crate::value::ToValue::to_value(&self.dataflow_edges),
        );
        properties.insert(
            "MinimumViableContactDurationSeconds".to_string(),
            crate::value::ToValue::to_value(&self.minimum_viable_contact_duration_seconds),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.streams_kms_key {
            properties.insert(
                "StreamsKmsKey".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.streams_kms_role {
            properties.insert(
                "StreamsKmsRole".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.telemetry_sink_config_arn {
            properties.insert(
                "TelemetrySinkConfigArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "TrackingConfigArn".to_string(),
            crate::value::ToValue::to_value(&self.tracking_config_arn),
        );
        properties
    }
}
