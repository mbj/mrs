pub mod bridge {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-bridgeflowsource.html
    pub struct BridgeFlowSource_ {
        pub flow_arn: crate::value::ExpString,
        pub flow_vpc_interface_attachment: Option<Box<VpcInterfaceAttachment_>>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_Bridge_BridgeFlowSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::Bridge.BridgeFlowSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_Bridge_BridgeFlowSource as BridgeFlowSource;
    impl crate::value::ToValue for BridgeFlowSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FlowArn".to_string(),
                crate::value::ToValue::to_value(&self.flow_arn),
            );
            if let Some(ref value) = self.flow_vpc_interface_attachment {
                properties.insert(
                    "FlowVpcInterfaceAttachment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-bridgenetworkoutput.html
    pub struct BridgeNetworkOutput_ {
        pub ip_address: crate::value::ExpString,
        pub name: crate::value::ExpString,
        pub network_name: crate::value::ExpString,
        pub port: i32,
        pub protocol: crate::value::ExpString,
        pub ttl: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_Bridge_BridgeNetworkOutput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::Bridge.BridgeNetworkOutput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_Bridge_BridgeNetworkOutput as BridgeNetworkOutput;
    impl crate::value::ToValue for BridgeNetworkOutput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IpAddress".to_string(),
                crate::value::ToValue::to_value(&self.ip_address),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "NetworkName".to_string(),
                crate::value::ToValue::to_value(&self.network_name),
            );
            properties.insert(
                "Port".to_string(),
                crate::value::ToValue::to_value(&self.port),
            );
            properties.insert(
                "Protocol".to_string(),
                crate::value::ToValue::to_value(&self.protocol),
            );
            properties.insert(
                "Ttl".to_string(),
                crate::value::ToValue::to_value(&self.ttl),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-bridgenetworksource.html
    pub struct BridgeNetworkSource_ {
        pub multicast_ip: crate::value::ExpString,
        pub multicast_source_settings: Option<Box<MulticastSourceSettings_>>,
        pub name: crate::value::ExpString,
        pub network_name: crate::value::ExpString,
        pub port: i32,
        pub protocol: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_Bridge_BridgeNetworkSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::Bridge.BridgeNetworkSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_Bridge_BridgeNetworkSource as BridgeNetworkSource;
    impl crate::value::ToValue for BridgeNetworkSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MulticastIp".to_string(),
                crate::value::ToValue::to_value(&self.multicast_ip),
            );
            if let Some(ref value) = self.multicast_source_settings {
                properties.insert(
                    "MulticastSourceSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "NetworkName".to_string(),
                crate::value::ToValue::to_value(&self.network_name),
            );
            properties.insert(
                "Port".to_string(),
                crate::value::ToValue::to_value(&self.port),
            );
            properties.insert(
                "Protocol".to_string(),
                crate::value::ToValue::to_value(&self.protocol),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-bridgeoutput.html
    pub struct BridgeOutput_ {
        pub network_output: Option<Box<BridgeNetworkOutput_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_Bridge_BridgeOutput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::Bridge.BridgeOutput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_Bridge_BridgeOutput as BridgeOutput;
    impl crate::value::ToValue for BridgeOutput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.network_output {
                properties.insert(
                    "NetworkOutput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-bridgesource.html
    pub struct BridgeSource_ {
        pub flow_source: Option<Box<BridgeFlowSource_>>,
        pub network_source: Option<Box<BridgeNetworkSource_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_Bridge_BridgeSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::Bridge.BridgeSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_Bridge_BridgeSource as BridgeSource;
    impl crate::value::ToValue for BridgeSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.flow_source {
                properties.insert(
                    "FlowSource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_source {
                properties.insert(
                    "NetworkSource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-egressgatewaybridge.html
    pub struct EgressGatewayBridge_ {
        pub max_bitrate: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_Bridge_EgressGatewayBridge {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::Bridge.EgressGatewayBridge"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_Bridge_EgressGatewayBridge as EgressGatewayBridge;
    impl crate::value::ToValue for EgressGatewayBridge_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MaxBitrate".to_string(),
                crate::value::ToValue::to_value(&self.max_bitrate),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-failoverconfig.html
    pub struct FailoverConfig_ {
        pub failover_mode: crate::value::ExpString,
        pub source_priority: Option<Box<SourcePriority_>>,
        pub state: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_Bridge_FailoverConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::Bridge.FailoverConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_Bridge_FailoverConfig as FailoverConfig;
    impl crate::value::ToValue for FailoverConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FailoverMode".to_string(),
                crate::value::ToValue::to_value(&self.failover_mode),
            );
            if let Some(ref value) = self.source_priority {
                properties.insert(
                    "SourcePriority".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.state {
                properties.insert("State".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-ingressgatewaybridge.html
    pub struct IngressGatewayBridge_ {
        pub max_bitrate: i32,
        pub max_outputs: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_Bridge_IngressGatewayBridge {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::Bridge.IngressGatewayBridge"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_Bridge_IngressGatewayBridge as IngressGatewayBridge;
    impl crate::value::ToValue for IngressGatewayBridge_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MaxBitrate".to_string(),
                crate::value::ToValue::to_value(&self.max_bitrate),
            );
            properties.insert(
                "MaxOutputs".to_string(),
                crate::value::ToValue::to_value(&self.max_outputs),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-multicastsourcesettings.html
    pub struct MulticastSourceSettings_ {
        pub multicast_source_ip: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_Bridge_MulticastSourceSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::Bridge.MulticastSourceSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_Bridge_MulticastSourceSettings as MulticastSourceSettings;
    impl crate::value::ToValue for MulticastSourceSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.multicast_source_ip {
                properties.insert(
                    "MulticastSourceIp".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-sourcepriority.html
    pub struct SourcePriority_ {
        pub primary_source: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_Bridge_SourcePriority {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::Bridge.SourcePriority"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_Bridge_SourcePriority as SourcePriority;
    impl crate::value::ToValue for SourcePriority_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.primary_source {
                properties.insert(
                    "PrimarySource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-vpcinterfaceattachment.html
    pub struct VpcInterfaceAttachment_ {
        pub vpc_interface_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_Bridge_VpcInterfaceAttachment {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::Bridge.VpcInterfaceAttachment"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_Bridge_VpcInterfaceAttachment as VpcInterfaceAttachment;
    impl crate::value::ToValue for VpcInterfaceAttachment_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.vpc_interface_name {
                properties.insert(
                    "VpcInterfaceName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod bridgeoutput {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridgeoutput-bridgenetworkoutput.html
    pub struct BridgeNetworkOutput_ {
        pub ip_address: crate::value::ExpString,
        pub network_name: crate::value::ExpString,
        pub port: i32,
        pub protocol: crate::value::ExpString,
        pub ttl: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_BridgeOutput_BridgeNetworkOutput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::BridgeOutput.BridgeNetworkOutput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_BridgeOutput_BridgeNetworkOutput as BridgeNetworkOutput;
    impl crate::value::ToValue for BridgeNetworkOutput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IpAddress".to_string(),
                crate::value::ToValue::to_value(&self.ip_address),
            );
            properties.insert(
                "NetworkName".to_string(),
                crate::value::ToValue::to_value(&self.network_name),
            );
            properties.insert(
                "Port".to_string(),
                crate::value::ToValue::to_value(&self.port),
            );
            properties.insert(
                "Protocol".to_string(),
                crate::value::ToValue::to_value(&self.protocol),
            );
            properties.insert(
                "Ttl".to_string(),
                crate::value::ToValue::to_value(&self.ttl),
            );
            properties.into()
        }
    }
}
pub mod bridgesource {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridgesource-bridgeflowsource.html
    pub struct BridgeFlowSource_ {
        pub flow_arn: crate::value::ExpString,
        pub flow_vpc_interface_attachment: Option<Box<VpcInterfaceAttachment_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_BridgeSource_BridgeFlowSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::BridgeSource.BridgeFlowSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_BridgeSource_BridgeFlowSource as BridgeFlowSource;
    impl crate::value::ToValue for BridgeFlowSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FlowArn".to_string(),
                crate::value::ToValue::to_value(&self.flow_arn),
            );
            if let Some(ref value) = self.flow_vpc_interface_attachment {
                properties.insert(
                    "FlowVpcInterfaceAttachment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridgesource-bridgenetworksource.html
    pub struct BridgeNetworkSource_ {
        pub multicast_ip: crate::value::ExpString,
        pub multicast_source_settings: Option<Box<MulticastSourceSettings_>>,
        pub network_name: crate::value::ExpString,
        pub port: i32,
        pub protocol: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_BridgeSource_BridgeNetworkSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::BridgeSource.BridgeNetworkSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_BridgeSource_BridgeNetworkSource as BridgeNetworkSource;
    impl crate::value::ToValue for BridgeNetworkSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MulticastIp".to_string(),
                crate::value::ToValue::to_value(&self.multicast_ip),
            );
            if let Some(ref value) = self.multicast_source_settings {
                properties.insert(
                    "MulticastSourceSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "NetworkName".to_string(),
                crate::value::ToValue::to_value(&self.network_name),
            );
            properties.insert(
                "Port".to_string(),
                crate::value::ToValue::to_value(&self.port),
            );
            properties.insert(
                "Protocol".to_string(),
                crate::value::ToValue::to_value(&self.protocol),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridgesource-multicastsourcesettings.html
    pub struct MulticastSourceSettings_ {
        pub multicast_source_ip: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_BridgeSource_MulticastSourceSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::BridgeSource.MulticastSourceSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_BridgeSource_MulticastSourceSettings as MulticastSourceSettings;
    impl crate::value::ToValue for MulticastSourceSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.multicast_source_ip {
                properties.insert(
                    "MulticastSourceIp".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridgesource-vpcinterfaceattachment.html
    pub struct VpcInterfaceAttachment_ {
        pub vpc_interface_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_BridgeSource_VpcInterfaceAttachment {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::BridgeSource.VpcInterfaceAttachment"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_BridgeSource_VpcInterfaceAttachment as VpcInterfaceAttachment;
    impl crate::value::ToValue for VpcInterfaceAttachment_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.vpc_interface_name {
                properties.insert(
                    "VpcInterfaceName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod flow {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-audiomonitoringsetting.html
    pub struct AudioMonitoringSetting_ {
        pub silent_audio: Option<Box<SilentAudio_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_Flow_AudioMonitoringSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::Flow.AudioMonitoringSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_Flow_AudioMonitoringSetting as AudioMonitoringSetting;
    impl crate::value::ToValue for AudioMonitoringSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.silent_audio {
                properties.insert(
                    "SilentAudio".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-blackframes.html
    pub struct BlackFrames_ {
        pub state: Option<crate::value::ExpString>,
        pub threshold_seconds: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_Flow_BlackFrames {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::Flow.BlackFrames"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_Flow_BlackFrames as BlackFrames;
    impl crate::value::ToValue for BlackFrames_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.state {
                properties.insert("State".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.threshold_seconds {
                properties.insert(
                    "ThresholdSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-encryption.html
    pub struct Encryption_ {
        pub algorithm: Option<crate::value::ExpString>,
        pub constant_initialization_vector: Option<crate::value::ExpString>,
        pub device_id: Option<crate::value::ExpString>,
        pub key_type: Option<crate::value::ExpString>,
        pub region: Option<crate::value::ExpString>,
        pub resource_id: Option<crate::value::ExpString>,
        pub role_arn: crate::value::ExpString,
        pub secret_arn: Option<crate::value::ExpString>,
        pub url: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_Flow_Encryption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::Flow.Encryption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_Flow_Encryption as Encryption;
    impl crate::value::ToValue for Encryption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.algorithm {
                properties.insert(
                    "Algorithm".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.constant_initialization_vector {
                properties.insert(
                    "ConstantInitializationVector".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.device_id {
                properties.insert(
                    "DeviceId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.key_type {
                properties.insert(
                    "KeyType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.region {
                properties.insert("Region".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.resource_id {
                properties.insert(
                    "ResourceId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            if let Some(ref value) = self.secret_arn {
                properties.insert(
                    "SecretArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.url {
                properties.insert("Url".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-failoverconfig.html
    pub struct FailoverConfig_ {
        pub failover_mode: Option<crate::value::ExpString>,
        pub recovery_window: Option<i32>,
        pub source_priority: Option<Box<SourcePriority_>>,
        pub state: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_Flow_FailoverConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::Flow.FailoverConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_Flow_FailoverConfig as FailoverConfig;
    impl crate::value::ToValue for FailoverConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.failover_mode {
                properties.insert(
                    "FailoverMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.recovery_window {
                properties.insert(
                    "RecoveryWindow".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_priority {
                properties.insert(
                    "SourcePriority".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.state {
                properties.insert("State".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-flowtransitencryption.html
    pub struct FlowTransitEncryption_ {
        pub encryption_key_configuration: Box<FlowTransitEncryptionKeyConfiguration_>,
        pub encryption_key_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_Flow_FlowTransitEncryption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::Flow.FlowTransitEncryption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_Flow_FlowTransitEncryption as FlowTransitEncryption;
    impl crate::value::ToValue for FlowTransitEncryption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EncryptionKeyConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.encryption_key_configuration),
            );
            if let Some(ref value) = self.encryption_key_type {
                properties.insert(
                    "EncryptionKeyType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-flowtransitencryptionkeyconfiguration.html
    pub struct FlowTransitEncryptionKeyConfiguration_ {
        pub automatic: Option<serde_json::Value>,
        pub secrets_manager: Option<Box<SecretsManagerEncryptionKeyConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_Flow_FlowTransitEncryptionKeyConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::Flow.FlowTransitEncryptionKeyConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_Flow_FlowTransitEncryptionKeyConfiguration as FlowTransitEncryptionKeyConfiguration;
    impl crate::value::ToValue for FlowTransitEncryptionKeyConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.automatic {
                properties.insert(
                    "Automatic".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secrets_manager {
                properties.insert(
                    "SecretsManager".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-fmtp.html
    pub struct Fmtp_ {
        pub channel_order: Option<crate::value::ExpString>,
        pub colorimetry: Option<crate::value::ExpString>,
        pub exact_framerate: Option<crate::value::ExpString>,
        pub par: Option<crate::value::ExpString>,
        pub range: Option<crate::value::ExpString>,
        pub scan_mode: Option<crate::value::ExpString>,
        pub tcs: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_Flow_Fmtp {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::Flow.Fmtp"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_Flow_Fmtp as Fmtp;
    impl crate::value::ToValue for Fmtp_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.channel_order {
                properties.insert(
                    "ChannelOrder".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.colorimetry {
                properties.insert(
                    "Colorimetry".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exact_framerate {
                properties.insert(
                    "ExactFramerate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.par {
                properties.insert("Par".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.range {
                properties.insert("Range".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.scan_mode {
                properties.insert(
                    "ScanMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tcs {
                properties.insert("Tcs".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-frozenframes.html
    pub struct FrozenFrames_ {
        pub state: Option<crate::value::ExpString>,
        pub threshold_seconds: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_Flow_FrozenFrames {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::Flow.FrozenFrames"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_Flow_FrozenFrames as FrozenFrames;
    impl crate::value::ToValue for FrozenFrames_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.state {
                properties.insert("State".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.threshold_seconds {
                properties.insert(
                    "ThresholdSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-gatewaybridgesource.html
    pub struct GatewayBridgeSource_ {
        pub bridge_arn: crate::value::ExpString,
        pub vpc_interface_attachment: Option<Box<VpcInterfaceAttachment_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_Flow_GatewayBridgeSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::Flow.GatewayBridgeSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_Flow_GatewayBridgeSource as GatewayBridgeSource;
    impl crate::value::ToValue for GatewayBridgeSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BridgeArn".to_string(),
                crate::value::ToValue::to_value(&self.bridge_arn),
            );
            if let Some(ref value) = self.vpc_interface_attachment {
                properties.insert(
                    "VpcInterfaceAttachment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-inputconfiguration.html
    pub struct InputConfiguration_ {
        pub input_port: i32,
        pub interface: Box<Interface_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_Flow_InputConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::Flow.InputConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_Flow_InputConfiguration as InputConfiguration;
    impl crate::value::ToValue for InputConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InputPort".to_string(),
                crate::value::ToValue::to_value(&self.input_port),
            );
            properties.insert(
                "Interface".to_string(),
                crate::value::ToValue::to_value(&self.interface),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-interface.html
    pub struct Interface_ {
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_Flow_Interface {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::Flow.Interface"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_Flow_Interface as Interface;
    impl crate::value::ToValue for Interface_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-maintenance.html
    pub struct Maintenance_ {
        pub maintenance_day: crate::value::ExpString,
        pub maintenance_start_hour: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_Flow_Maintenance {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::Flow.Maintenance"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_Flow_Maintenance as Maintenance;
    impl crate::value::ToValue for Maintenance_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MaintenanceDay".to_string(),
                crate::value::ToValue::to_value(&self.maintenance_day),
            );
            properties.insert(
                "MaintenanceStartHour".to_string(),
                crate::value::ToValue::to_value(&self.maintenance_start_hour),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-mediastream.html
    pub struct MediaStream_ {
        pub attributes: Option<Box<MediaStreamAttributes_>>,
        pub clock_rate: Option<i32>,
        pub description: Option<crate::value::ExpString>,
        pub fmt: Option<i32>,
        pub media_stream_id: i32,
        pub media_stream_name: crate::value::ExpString,
        pub media_stream_type: crate::value::ExpString,
        pub video_format: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_Flow_MediaStream {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::Flow.MediaStream"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_Flow_MediaStream as MediaStream;
    impl crate::value::ToValue for MediaStream_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attributes {
                properties.insert(
                    "Attributes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.clock_rate {
                properties.insert(
                    "ClockRate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.fmt {
                properties.insert("Fmt".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "MediaStreamId".to_string(),
                crate::value::ToValue::to_value(&self.media_stream_id),
            );
            properties.insert(
                "MediaStreamName".to_string(),
                crate::value::ToValue::to_value(&self.media_stream_name),
            );
            properties.insert(
                "MediaStreamType".to_string(),
                crate::value::ToValue::to_value(&self.media_stream_type),
            );
            if let Some(ref value) = self.video_format {
                properties.insert(
                    "VideoFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-mediastreamattributes.html
    pub struct MediaStreamAttributes_ {
        pub fmtp: Option<Box<Fmtp_>>,
        pub lang: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_Flow_MediaStreamAttributes {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::Flow.MediaStreamAttributes"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_Flow_MediaStreamAttributes as MediaStreamAttributes;
    impl crate::value::ToValue for MediaStreamAttributes_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.fmtp {
                properties.insert("Fmtp".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.lang {
                properties.insert("Lang".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-mediastreamsourceconfiguration.html
    pub struct MediaStreamSourceConfiguration_ {
        pub encoding_name: crate::value::ExpString,
        pub input_configurations: Option<Vec<InputConfiguration_>>,
        pub media_stream_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_Flow_MediaStreamSourceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::Flow.MediaStreamSourceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_Flow_MediaStreamSourceConfiguration as MediaStreamSourceConfiguration;
    impl crate::value::ToValue for MediaStreamSourceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EncodingName".to_string(),
                crate::value::ToValue::to_value(&self.encoding_name),
            );
            if let Some(ref value) = self.input_configurations {
                properties.insert(
                    "InputConfigurations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MediaStreamName".to_string(),
                crate::value::ToValue::to_value(&self.media_stream_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-ndiconfig.html
    pub struct NdiConfig_ {
        pub machine_name: Option<crate::value::ExpString>,
        pub ndi_discovery_servers: Option<Vec<NdiDiscoveryServerConfig_>>,
        pub ndi_state: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_Flow_NdiConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::Flow.NdiConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_Flow_NdiConfig as NdiConfig;
    impl crate::value::ToValue for NdiConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.machine_name {
                properties.insert(
                    "MachineName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ndi_discovery_servers {
                properties.insert(
                    "NdiDiscoveryServers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ndi_state {
                properties.insert(
                    "NdiState".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-ndidiscoveryserverconfig.html
    pub struct NdiDiscoveryServerConfig_ {
        pub discovery_server_address: crate::value::ExpString,
        pub discovery_server_port: Option<i32>,
        pub vpc_interface_adapter: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_Flow_NdiDiscoveryServerConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::Flow.NdiDiscoveryServerConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_Flow_NdiDiscoveryServerConfig as NdiDiscoveryServerConfig;
    impl crate::value::ToValue for NdiDiscoveryServerConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DiscoveryServerAddress".to_string(),
                crate::value::ToValue::to_value(&self.discovery_server_address),
            );
            if let Some(ref value) = self.discovery_server_port {
                properties.insert(
                    "DiscoveryServerPort".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "VpcInterfaceAdapter".to_string(),
                crate::value::ToValue::to_value(&self.vpc_interface_adapter),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-secretsmanagerencryptionkeyconfiguration.html
    pub struct SecretsManagerEncryptionKeyConfiguration_ {
        pub role_arn: crate::value::ExpString,
        pub secret_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_Flow_SecretsManagerEncryptionKeyConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::Flow.SecretsManagerEncryptionKeyConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_Flow_SecretsManagerEncryptionKeyConfiguration as SecretsManagerEncryptionKeyConfiguration;
    impl crate::value::ToValue for SecretsManagerEncryptionKeyConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.insert(
                "SecretArn".to_string(),
                crate::value::ToValue::to_value(&self.secret_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-silentaudio.html
    pub struct SilentAudio_ {
        pub state: Option<crate::value::ExpString>,
        pub threshold_seconds: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_Flow_SilentAudio {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::Flow.SilentAudio"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_Flow_SilentAudio as SilentAudio;
    impl crate::value::ToValue for SilentAudio_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.state {
                properties.insert("State".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.threshold_seconds {
                properties.insert(
                    "ThresholdSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-source.html
    pub struct Source_ {
        pub decryption: Option<Box<Encryption_>>,
        pub description: Option<crate::value::ExpString>,
        pub entitlement_arn: Option<crate::value::ExpString>,
        pub gateway_bridge_source: Option<Box<GatewayBridgeSource_>>,
        pub ingest_ip: Option<crate::value::ExpString>,
        pub ingest_port: Option<i32>,
        pub max_bitrate: Option<i32>,
        pub max_latency: Option<i32>,
        pub max_sync_buffer: Option<i32>,
        pub media_stream_source_configurations: Option<Vec<MediaStreamSourceConfiguration_>>,
        pub min_latency: Option<i32>,
        pub name: Option<crate::value::ExpString>,
        pub protocol: Option<crate::value::ExpString>,
        pub router_integration_state: Option<crate::value::ExpString>,
        pub router_integration_transit_decryption: Option<Box<FlowTransitEncryption_>>,
        pub sender_control_port: Option<i32>,
        pub sender_ip_address: Option<crate::value::ExpString>,
        pub source_arn: Option<crate::value::ExpString>,
        pub source_ingest_port: Option<crate::value::ExpString>,
        pub source_listener_address: Option<crate::value::ExpString>,
        pub source_listener_port: Option<i32>,
        pub stream_id: Option<crate::value::ExpString>,
        pub vpc_interface_name: Option<crate::value::ExpString>,
        pub whitelist_cidr: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_Flow_Source {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::Flow.Source"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_Flow_Source as Source;
    impl crate::value::ToValue for Source_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.decryption {
                properties.insert(
                    "Decryption".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.entitlement_arn {
                properties.insert(
                    "EntitlementArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.gateway_bridge_source {
                properties.insert(
                    "GatewayBridgeSource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ingest_ip {
                properties.insert(
                    "IngestIp".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ingest_port {
                properties.insert(
                    "IngestPort".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_bitrate {
                properties.insert(
                    "MaxBitrate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_latency {
                properties.insert(
                    "MaxLatency".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_sync_buffer {
                properties.insert(
                    "MaxSyncBuffer".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.media_stream_source_configurations {
                properties.insert(
                    "MediaStreamSourceConfigurations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.min_latency {
                properties.insert(
                    "MinLatency".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.protocol {
                properties.insert(
                    "Protocol".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.router_integration_state {
                properties.insert(
                    "RouterIntegrationState".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.router_integration_transit_decryption {
                properties.insert(
                    "RouterIntegrationTransitDecryption".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sender_control_port {
                properties.insert(
                    "SenderControlPort".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sender_ip_address {
                properties.insert(
                    "SenderIpAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_arn {
                properties.insert(
                    "SourceArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_ingest_port {
                properties.insert(
                    "SourceIngestPort".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_listener_address {
                properties.insert(
                    "SourceListenerAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_listener_port {
                properties.insert(
                    "SourceListenerPort".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stream_id {
                properties.insert(
                    "StreamId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpc_interface_name {
                properties.insert(
                    "VpcInterfaceName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.whitelist_cidr {
                properties.insert(
                    "WhitelistCidr".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-sourcemonitoringconfig.html
    pub struct SourceMonitoringConfig_ {
        pub audio_monitoring_settings: Option<Vec<AudioMonitoringSetting_>>,
        pub content_quality_analysis_state: Option<crate::value::ExpString>,
        pub thumbnail_state: Option<crate::value::ExpString>,
        pub video_monitoring_settings: Option<Vec<VideoMonitoringSetting_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_Flow_SourceMonitoringConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::Flow.SourceMonitoringConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_Flow_SourceMonitoringConfig as SourceMonitoringConfig;
    impl crate::value::ToValue for SourceMonitoringConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.audio_monitoring_settings {
                properties.insert(
                    "AudioMonitoringSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.content_quality_analysis_state {
                properties.insert(
                    "ContentQualityAnalysisState".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.thumbnail_state {
                properties.insert(
                    "ThumbnailState".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.video_monitoring_settings {
                properties.insert(
                    "VideoMonitoringSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-sourcepriority.html
    pub struct SourcePriority_ {
        pub primary_source: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_Flow_SourcePriority {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::Flow.SourcePriority"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_Flow_SourcePriority as SourcePriority;
    impl crate::value::ToValue for SourcePriority_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PrimarySource".to_string(),
                crate::value::ToValue::to_value(&self.primary_source),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-videomonitoringsetting.html
    pub struct VideoMonitoringSetting_ {
        pub black_frames: Option<Box<BlackFrames_>>,
        pub frozen_frames: Option<Box<FrozenFrames_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_Flow_VideoMonitoringSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::Flow.VideoMonitoringSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_Flow_VideoMonitoringSetting as VideoMonitoringSetting;
    impl crate::value::ToValue for VideoMonitoringSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.black_frames {
                properties.insert(
                    "BlackFrames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.frozen_frames {
                properties.insert(
                    "FrozenFrames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-vpcinterface.html
    pub struct VpcInterface_ {
        pub name: crate::value::ExpString,
        pub network_interface_ids: Option<Vec<crate::value::ExpString>>,
        pub network_interface_type: Option<crate::value::ExpString>,
        pub role_arn: crate::value::ExpString,
        pub security_group_ids: Vec<crate::value::ExpString>,
        pub subnet_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_Flow_VpcInterface {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::Flow.VpcInterface"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_Flow_VpcInterface as VpcInterface;
    impl crate::value::ToValue for VpcInterface_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.network_interface_ids {
                properties.insert(
                    "NetworkInterfaceIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_interface_type {
                properties.insert(
                    "NetworkInterfaceType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.insert(
                "SecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(&self.security_group_ids),
            );
            properties.insert(
                "SubnetId".to_string(),
                crate::value::ToValue::to_value(&self.subnet_id),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-vpcinterfaceattachment.html
    pub struct VpcInterfaceAttachment_ {
        pub vpc_interface_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_Flow_VpcInterfaceAttachment {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::Flow.VpcInterfaceAttachment"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_Flow_VpcInterfaceAttachment as VpcInterfaceAttachment;
    impl crate::value::ToValue for VpcInterfaceAttachment_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.vpc_interface_name {
                properties.insert(
                    "VpcInterfaceName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod flowentitlement {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowentitlement-encryption.html
    pub struct Encryption_ {
        pub algorithm: crate::value::ExpString,
        pub constant_initialization_vector: Option<crate::value::ExpString>,
        pub device_id: Option<crate::value::ExpString>,
        pub key_type: Option<crate::value::ExpString>,
        pub region: Option<crate::value::ExpString>,
        pub resource_id: Option<crate::value::ExpString>,
        pub role_arn: crate::value::ExpString,
        pub secret_arn: Option<crate::value::ExpString>,
        pub url: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_FlowEntitlement_Encryption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::FlowEntitlement.Encryption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_FlowEntitlement_Encryption as Encryption;
    impl crate::value::ToValue for Encryption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Algorithm".to_string(),
                crate::value::ToValue::to_value(&self.algorithm),
            );
            if let Some(ref value) = self.constant_initialization_vector {
                properties.insert(
                    "ConstantInitializationVector".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.device_id {
                properties.insert(
                    "DeviceId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.key_type {
                properties.insert(
                    "KeyType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.region {
                properties.insert("Region".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.resource_id {
                properties.insert(
                    "ResourceId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            if let Some(ref value) = self.secret_arn {
                properties.insert(
                    "SecretArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.url {
                properties.insert("Url".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod flowoutput {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowoutput-destinationconfiguration.html
    pub struct DestinationConfiguration_ {
        pub destination_ip: crate::value::ExpString,
        pub destination_port: i32,
        pub interface: Box<Interface_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_FlowOutput_DestinationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::FlowOutput.DestinationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_FlowOutput_DestinationConfiguration as DestinationConfiguration;
    impl crate::value::ToValue for DestinationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DestinationIp".to_string(),
                crate::value::ToValue::to_value(&self.destination_ip),
            );
            properties.insert(
                "DestinationPort".to_string(),
                crate::value::ToValue::to_value(&self.destination_port),
            );
            properties.insert(
                "Interface".to_string(),
                crate::value::ToValue::to_value(&self.interface),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowoutput-encodingparameters.html
    pub struct EncodingParameters_ {
        pub compression_factor: f64,
        pub encoder_profile: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_FlowOutput_EncodingParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::FlowOutput.EncodingParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_FlowOutput_EncodingParameters as EncodingParameters;
    impl crate::value::ToValue for EncodingParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CompressionFactor".to_string(),
                crate::value::ToValue::to_value(&self.compression_factor),
            );
            if let Some(ref value) = self.encoder_profile {
                properties.insert(
                    "EncoderProfile".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowoutput-encryption.html
    pub struct Encryption_ {
        pub algorithm: Option<crate::value::ExpString>,
        pub key_type: Option<crate::value::ExpString>,
        pub role_arn: crate::value::ExpString,
        pub secret_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_FlowOutput_Encryption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::FlowOutput.Encryption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_FlowOutput_Encryption as Encryption;
    impl crate::value::ToValue for Encryption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.algorithm {
                properties.insert(
                    "Algorithm".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.key_type {
                properties.insert(
                    "KeyType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.insert(
                "SecretArn".to_string(),
                crate::value::ToValue::to_value(&self.secret_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowoutput-flowtransitencryption.html
    pub struct FlowTransitEncryption_ {
        pub encryption_key_configuration: Box<FlowTransitEncryptionKeyConfiguration_>,
        pub encryption_key_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_FlowOutput_FlowTransitEncryption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::FlowOutput.FlowTransitEncryption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_FlowOutput_FlowTransitEncryption as FlowTransitEncryption;
    impl crate::value::ToValue for FlowTransitEncryption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EncryptionKeyConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.encryption_key_configuration),
            );
            if let Some(ref value) = self.encryption_key_type {
                properties.insert(
                    "EncryptionKeyType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowoutput-flowtransitencryptionkeyconfiguration.html
    pub struct FlowTransitEncryptionKeyConfiguration_ {
        pub automatic: Option<serde_json::Value>,
        pub secrets_manager: Option<Box<SecretsManagerEncryptionKeyConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_FlowOutput_FlowTransitEncryptionKeyConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::FlowOutput.FlowTransitEncryptionKeyConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_FlowOutput_FlowTransitEncryptionKeyConfiguration as FlowTransitEncryptionKeyConfiguration;
    impl crate::value::ToValue for FlowTransitEncryptionKeyConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.automatic {
                properties.insert(
                    "Automatic".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secrets_manager {
                properties.insert(
                    "SecretsManager".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowoutput-interface.html
    pub struct Interface_ {
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_FlowOutput_Interface {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::FlowOutput.Interface"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_FlowOutput_Interface as Interface;
    impl crate::value::ToValue for Interface_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowoutput-mediastreamoutputconfiguration.html
    pub struct MediaStreamOutputConfiguration_ {
        pub destination_configurations: Option<Vec<DestinationConfiguration_>>,
        pub encoding_name: crate::value::ExpString,
        pub encoding_parameters: Option<Box<EncodingParameters_>>,
        pub media_stream_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_FlowOutput_MediaStreamOutputConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::FlowOutput.MediaStreamOutputConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_FlowOutput_MediaStreamOutputConfiguration as MediaStreamOutputConfiguration;
    impl crate::value::ToValue for MediaStreamOutputConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.destination_configurations {
                properties.insert(
                    "DestinationConfigurations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "EncodingName".to_string(),
                crate::value::ToValue::to_value(&self.encoding_name),
            );
            if let Some(ref value) = self.encoding_parameters {
                properties.insert(
                    "EncodingParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MediaStreamName".to_string(),
                crate::value::ToValue::to_value(&self.media_stream_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowoutput-secretsmanagerencryptionkeyconfiguration.html
    pub struct SecretsManagerEncryptionKeyConfiguration_ {
        pub role_arn: crate::value::ExpString,
        pub secret_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_FlowOutput_SecretsManagerEncryptionKeyConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::FlowOutput.SecretsManagerEncryptionKeyConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_FlowOutput_SecretsManagerEncryptionKeyConfiguration as SecretsManagerEncryptionKeyConfiguration;
    impl crate::value::ToValue for SecretsManagerEncryptionKeyConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.insert(
                "SecretArn".to_string(),
                crate::value::ToValue::to_value(&self.secret_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowoutput-vpcinterfaceattachment.html
    pub struct VpcInterfaceAttachment_ {
        pub vpc_interface_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_FlowOutput_VpcInterfaceAttachment {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::FlowOutput.VpcInterfaceAttachment"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_FlowOutput_VpcInterfaceAttachment as VpcInterfaceAttachment;
    impl crate::value::ToValue for VpcInterfaceAttachment_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.vpc_interface_name {
                properties.insert(
                    "VpcInterfaceName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod flowsource {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowsource-encryption.html
    pub struct Encryption_ {
        pub algorithm: Option<crate::value::ExpString>,
        pub constant_initialization_vector: Option<crate::value::ExpString>,
        pub device_id: Option<crate::value::ExpString>,
        pub key_type: Option<crate::value::ExpString>,
        pub region: Option<crate::value::ExpString>,
        pub resource_id: Option<crate::value::ExpString>,
        pub role_arn: crate::value::ExpString,
        pub secret_arn: Option<crate::value::ExpString>,
        pub url: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_FlowSource_Encryption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::FlowSource.Encryption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_FlowSource_Encryption as Encryption;
    impl crate::value::ToValue for Encryption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.algorithm {
                properties.insert(
                    "Algorithm".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.constant_initialization_vector {
                properties.insert(
                    "ConstantInitializationVector".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.device_id {
                properties.insert(
                    "DeviceId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.key_type {
                properties.insert(
                    "KeyType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.region {
                properties.insert("Region".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.resource_id {
                properties.insert(
                    "ResourceId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            if let Some(ref value) = self.secret_arn {
                properties.insert(
                    "SecretArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.url {
                properties.insert("Url".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowsource-gatewaybridgesource.html
    pub struct GatewayBridgeSource_ {
        pub bridge_arn: crate::value::ExpString,
        pub vpc_interface_attachment: Option<Box<VpcInterfaceAttachment_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_FlowSource_GatewayBridgeSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::FlowSource.GatewayBridgeSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_FlowSource_GatewayBridgeSource as GatewayBridgeSource;
    impl crate::value::ToValue for GatewayBridgeSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BridgeArn".to_string(),
                crate::value::ToValue::to_value(&self.bridge_arn),
            );
            if let Some(ref value) = self.vpc_interface_attachment {
                properties.insert(
                    "VpcInterfaceAttachment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowsource-vpcinterfaceattachment.html
    pub struct VpcInterfaceAttachment_ {
        pub vpc_interface_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_FlowSource_VpcInterfaceAttachment {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::FlowSource.VpcInterfaceAttachment"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_FlowSource_VpcInterfaceAttachment as VpcInterfaceAttachment;
    impl crate::value::ToValue for VpcInterfaceAttachment_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.vpc_interface_name {
                properties.insert(
                    "VpcInterfaceName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod gateway {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-gateway-gatewaynetwork.html
    pub struct GatewayNetwork_ {
        pub cidr_block: crate::value::ExpString,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_Gateway_GatewayNetwork {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::Gateway.GatewayNetwork"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_Gateway_GatewayNetwork as GatewayNetwork;
    impl crate::value::ToValue for GatewayNetwork_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CidrBlock".to_string(),
                crate::value::ToValue::to_value(&self.cidr_block),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
}
pub mod routerinput {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routerinput-failoverrouterinputconfiguration.html
    pub struct FailoverRouterInputConfiguration_ {
        pub network_interface_arn: crate::value::ExpString,
        pub primary_source_index: Option<i32>,
        pub protocol_configurations: Vec<FailoverRouterInputProtocolConfiguration_>,
        pub source_priority_mode: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterInput_FailoverRouterInputConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterInput.FailoverRouterInputConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterInput_FailoverRouterInputConfiguration as FailoverRouterInputConfiguration;
    impl crate::value::ToValue for FailoverRouterInputConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "NetworkInterfaceArn".to_string(),
                crate::value::ToValue::to_value(&self.network_interface_arn),
            );
            if let Some(ref value) = self.primary_source_index {
                properties.insert(
                    "PrimarySourceIndex".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ProtocolConfigurations".to_string(),
                crate::value::ToValue::to_value(&self.protocol_configurations),
            );
            properties.insert(
                "SourcePriorityMode".to_string(),
                crate::value::ToValue::to_value(&self.source_priority_mode),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routerinput-failoverrouterinputprotocolconfiguration.html
    pub struct FailoverRouterInputProtocolConfiguration_ {
        pub rist: Option<Box<RistRouterInputConfiguration_>>,
        pub rtp: Option<Box<RtpRouterInputConfiguration_>>,
        pub srt_caller: Option<Box<SrtCallerRouterInputConfiguration_>>,
        pub srt_listener: Option<Box<SrtListenerRouterInputConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterInput_FailoverRouterInputProtocolConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterInput.FailoverRouterInputProtocolConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterInput_FailoverRouterInputProtocolConfiguration as FailoverRouterInputProtocolConfiguration;
    impl crate::value::ToValue for FailoverRouterInputProtocolConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.rist {
                properties.insert("Rist".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.rtp {
                properties.insert("Rtp".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.srt_caller {
                properties.insert(
                    "SrtCaller".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.srt_listener {
                properties.insert(
                    "SrtListener".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routerinput-flowtransitencryption.html
    pub struct FlowTransitEncryption_ {
        pub encryption_key_configuration: Box<FlowTransitEncryptionKeyConfiguration_>,
        pub encryption_key_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterInput_FlowTransitEncryption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterInput.FlowTransitEncryption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterInput_FlowTransitEncryption as FlowTransitEncryption;
    impl crate::value::ToValue for FlowTransitEncryption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EncryptionKeyConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.encryption_key_configuration),
            );
            if let Some(ref value) = self.encryption_key_type {
                properties.insert(
                    "EncryptionKeyType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routerinput-flowtransitencryptionkeyconfiguration.html
    pub struct FlowTransitEncryptionKeyConfiguration_ {
        pub automatic: Option<serde_json::Value>,
        pub secrets_manager: Option<Box<SecretsManagerEncryptionKeyConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterInput_FlowTransitEncryptionKeyConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterInput.FlowTransitEncryptionKeyConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterInput_FlowTransitEncryptionKeyConfiguration as FlowTransitEncryptionKeyConfiguration;
    impl crate::value::ToValue for FlowTransitEncryptionKeyConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.automatic {
                properties.insert(
                    "Automatic".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secrets_manager {
                properties.insert(
                    "SecretsManager".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routerinput-maintenanceconfiguration.html
    pub struct MaintenanceConfiguration_ {
        pub default: Option<serde_json::Value>,
        pub preferred_day_time: Option<Box<PreferredDayTimeMaintenanceConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterInput_MaintenanceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterInput.MaintenanceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterInput_MaintenanceConfiguration as MaintenanceConfiguration;
    impl crate::value::ToValue for MaintenanceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.default {
                properties.insert(
                    "Default".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.preferred_day_time {
                properties.insert(
                    "PreferredDayTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routerinput-mediaconnectflowrouterinputconfiguration.html
    pub struct MediaConnectFlowRouterInputConfiguration_ {
        pub flow_arn: Option<crate::value::ExpString>,
        pub flow_output_arn: Option<crate::value::ExpString>,
        pub source_transit_decryption: Box<FlowTransitEncryption_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterInput_MediaConnectFlowRouterInputConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterInput.MediaConnectFlowRouterInputConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterInput_MediaConnectFlowRouterInputConfiguration as MediaConnectFlowRouterInputConfiguration;
    impl crate::value::ToValue for MediaConnectFlowRouterInputConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.flow_arn {
                properties.insert(
                    "FlowArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.flow_output_arn {
                properties.insert(
                    "FlowOutputArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SourceTransitDecryption".to_string(),
                crate::value::ToValue::to_value(&self.source_transit_decryption),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routerinput-mergerouterinputconfiguration.html
    pub struct MergeRouterInputConfiguration_ {
        pub merge_recovery_window_milliseconds: i32,
        pub network_interface_arn: crate::value::ExpString,
        pub protocol_configurations: Vec<MergeRouterInputProtocolConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterInput_MergeRouterInputConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterInput.MergeRouterInputConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterInput_MergeRouterInputConfiguration as MergeRouterInputConfiguration;
    impl crate::value::ToValue for MergeRouterInputConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MergeRecoveryWindowMilliseconds".to_string(),
                crate::value::ToValue::to_value(&self.merge_recovery_window_milliseconds),
            );
            properties.insert(
                "NetworkInterfaceArn".to_string(),
                crate::value::ToValue::to_value(&self.network_interface_arn),
            );
            properties.insert(
                "ProtocolConfigurations".to_string(),
                crate::value::ToValue::to_value(&self.protocol_configurations),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routerinput-mergerouterinputprotocolconfiguration.html
    pub struct MergeRouterInputProtocolConfiguration_ {
        pub rist: Option<Box<RistRouterInputConfiguration_>>,
        pub rtp: Option<Box<RtpRouterInputConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterInput_MergeRouterInputProtocolConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterInput.MergeRouterInputProtocolConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterInput_MergeRouterInputProtocolConfiguration as MergeRouterInputProtocolConfiguration;
    impl crate::value::ToValue for MergeRouterInputProtocolConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.rist {
                properties.insert("Rist".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.rtp {
                properties.insert("Rtp".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routerinput-preferreddaytimemaintenanceconfiguration.html
    pub struct PreferredDayTimeMaintenanceConfiguration_ {
        pub day: crate::value::ExpString,
        pub time: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterInput_PreferredDayTimeMaintenanceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterInput.PreferredDayTimeMaintenanceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterInput_PreferredDayTimeMaintenanceConfiguration as PreferredDayTimeMaintenanceConfiguration;
    impl crate::value::ToValue for PreferredDayTimeMaintenanceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Day".to_string(),
                crate::value::ToValue::to_value(&self.day),
            );
            properties.insert(
                "Time".to_string(),
                crate::value::ToValue::to_value(&self.time),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routerinput-ristrouterinputconfiguration.html
    pub struct RistRouterInputConfiguration_ {
        pub port: i32,
        pub recovery_latency_milliseconds: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterInput_RistRouterInputConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterInput.RistRouterInputConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterInput_RistRouterInputConfiguration as RistRouterInputConfiguration;
    impl crate::value::ToValue for RistRouterInputConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Port".to_string(),
                crate::value::ToValue::to_value(&self.port),
            );
            properties.insert(
                "RecoveryLatencyMilliseconds".to_string(),
                crate::value::ToValue::to_value(&self.recovery_latency_milliseconds),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routerinput-routerinputconfiguration.html
    pub struct RouterInputConfiguration_ {
        pub failover: Option<Box<FailoverRouterInputConfiguration_>>,
        pub media_connect_flow: Option<Box<MediaConnectFlowRouterInputConfiguration_>>,
        pub merge: Option<Box<MergeRouterInputConfiguration_>>,
        pub standard: Option<Box<StandardRouterInputConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterInput_RouterInputConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterInput.RouterInputConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterInput_RouterInputConfiguration as RouterInputConfiguration;
    impl crate::value::ToValue for RouterInputConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.failover {
                properties.insert(
                    "Failover".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.media_connect_flow {
                properties.insert(
                    "MediaConnectFlow".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.merge {
                properties.insert("Merge".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.standard {
                properties.insert(
                    "Standard".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routerinput-routerinputprotocolconfiguration.html
    pub struct RouterInputProtocolConfiguration_ {
        pub rist: Option<Box<RistRouterInputConfiguration_>>,
        pub rtp: Option<Box<RtpRouterInputConfiguration_>>,
        pub srt_caller: Option<Box<SrtCallerRouterInputConfiguration_>>,
        pub srt_listener: Option<Box<SrtListenerRouterInputConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterInput_RouterInputProtocolConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterInput.RouterInputProtocolConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterInput_RouterInputProtocolConfiguration as RouterInputProtocolConfiguration;
    impl crate::value::ToValue for RouterInputProtocolConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.rist {
                properties.insert("Rist".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.rtp {
                properties.insert("Rtp".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.srt_caller {
                properties.insert(
                    "SrtCaller".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.srt_listener {
                properties.insert(
                    "SrtListener".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routerinput-routerinputtransitencryption.html
    pub struct RouterInputTransitEncryption_ {
        pub encryption_key_configuration: Box<RouterInputTransitEncryptionKeyConfiguration_>,
        pub encryption_key_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterInput_RouterInputTransitEncryption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterInput.RouterInputTransitEncryption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterInput_RouterInputTransitEncryption as RouterInputTransitEncryption;
    impl crate::value::ToValue for RouterInputTransitEncryption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EncryptionKeyConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.encryption_key_configuration),
            );
            if let Some(ref value) = self.encryption_key_type {
                properties.insert(
                    "EncryptionKeyType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routerinput-routerinputtransitencryptionkeyconfiguration.html
    pub struct RouterInputTransitEncryptionKeyConfiguration_ {
        pub automatic: Option<serde_json::Value>,
        pub secrets_manager: Option<Box<SecretsManagerEncryptionKeyConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterInput_RouterInputTransitEncryptionKeyConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterInput.RouterInputTransitEncryptionKeyConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterInput_RouterInputTransitEncryptionKeyConfiguration as RouterInputTransitEncryptionKeyConfiguration;
    impl crate::value::ToValue for RouterInputTransitEncryptionKeyConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.automatic {
                properties.insert(
                    "Automatic".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secrets_manager {
                properties.insert(
                    "SecretsManager".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routerinput-rtprouterinputconfiguration.html
    pub struct RtpRouterInputConfiguration_ {
        pub forward_error_correction: Option<crate::value::ExpString>,
        pub port: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterInput_RtpRouterInputConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterInput.RtpRouterInputConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterInput_RtpRouterInputConfiguration as RtpRouterInputConfiguration;
    impl crate::value::ToValue for RtpRouterInputConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.forward_error_correction {
                properties.insert(
                    "ForwardErrorCorrection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Port".to_string(),
                crate::value::ToValue::to_value(&self.port),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routerinput-secretsmanagerencryptionkeyconfiguration.html
    pub struct SecretsManagerEncryptionKeyConfiguration_ {
        pub role_arn: crate::value::ExpString,
        pub secret_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterInput_SecretsManagerEncryptionKeyConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterInput.SecretsManagerEncryptionKeyConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterInput_SecretsManagerEncryptionKeyConfiguration as SecretsManagerEncryptionKeyConfiguration;
    impl crate::value::ToValue for SecretsManagerEncryptionKeyConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.insert(
                "SecretArn".to_string(),
                crate::value::ToValue::to_value(&self.secret_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routerinput-srtcallerrouterinputconfiguration.html
    pub struct SrtCallerRouterInputConfiguration_ {
        pub decryption_configuration: Option<Box<SrtDecryptionConfiguration_>>,
        pub minimum_latency_milliseconds: i32,
        pub source_address: crate::value::ExpString,
        pub source_port: i32,
        pub stream_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterInput_SrtCallerRouterInputConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterInput.SrtCallerRouterInputConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterInput_SrtCallerRouterInputConfiguration as SrtCallerRouterInputConfiguration;
    impl crate::value::ToValue for SrtCallerRouterInputConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.decryption_configuration {
                properties.insert(
                    "DecryptionConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MinimumLatencyMilliseconds".to_string(),
                crate::value::ToValue::to_value(&self.minimum_latency_milliseconds),
            );
            properties.insert(
                "SourceAddress".to_string(),
                crate::value::ToValue::to_value(&self.source_address),
            );
            properties.insert(
                "SourcePort".to_string(),
                crate::value::ToValue::to_value(&self.source_port),
            );
            if let Some(ref value) = self.stream_id {
                properties.insert(
                    "StreamId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routerinput-srtdecryptionconfiguration.html
    pub struct SrtDecryptionConfiguration_ {
        pub encryption_key: Box<SecretsManagerEncryptionKeyConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterInput_SrtDecryptionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterInput.SrtDecryptionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterInput_SrtDecryptionConfiguration as SrtDecryptionConfiguration;
    impl crate::value::ToValue for SrtDecryptionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EncryptionKey".to_string(),
                crate::value::ToValue::to_value(&self.encryption_key),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routerinput-srtlistenerrouterinputconfiguration.html
    pub struct SrtListenerRouterInputConfiguration_ {
        pub decryption_configuration: Option<Box<SrtDecryptionConfiguration_>>,
        pub minimum_latency_milliseconds: i32,
        pub port: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterInput_SrtListenerRouterInputConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterInput.SrtListenerRouterInputConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterInput_SrtListenerRouterInputConfiguration as SrtListenerRouterInputConfiguration;
    impl crate::value::ToValue for SrtListenerRouterInputConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.decryption_configuration {
                properties.insert(
                    "DecryptionConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MinimumLatencyMilliseconds".to_string(),
                crate::value::ToValue::to_value(&self.minimum_latency_milliseconds),
            );
            properties.insert(
                "Port".to_string(),
                crate::value::ToValue::to_value(&self.port),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routerinput-standardrouterinputconfiguration.html
    pub struct StandardRouterInputConfiguration_ {
        pub network_interface_arn: crate::value::ExpString,
        pub protocol: Option<crate::value::ExpString>,
        pub protocol_configuration: Box<RouterInputProtocolConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterInput_StandardRouterInputConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterInput.StandardRouterInputConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterInput_StandardRouterInputConfiguration as StandardRouterInputConfiguration;
    impl crate::value::ToValue for StandardRouterInputConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "NetworkInterfaceArn".to_string(),
                crate::value::ToValue::to_value(&self.network_interface_arn),
            );
            if let Some(ref value) = self.protocol {
                properties.insert(
                    "Protocol".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ProtocolConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.protocol_configuration),
            );
            properties.into()
        }
    }
}
pub mod routernetworkinterface {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routernetworkinterface-publicrouternetworkinterfaceconfiguration.html
    pub struct PublicRouterNetworkInterfaceConfiguration_ {
        pub allow_rules: Vec<PublicRouterNetworkInterfaceRule_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterNetworkInterface_PublicRouterNetworkInterfaceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterNetworkInterface.PublicRouterNetworkInterfaceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterNetworkInterface_PublicRouterNetworkInterfaceConfiguration as PublicRouterNetworkInterfaceConfiguration;
    impl crate::value::ToValue for PublicRouterNetworkInterfaceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AllowRules".to_string(),
                crate::value::ToValue::to_value(&self.allow_rules),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routernetworkinterface-publicrouternetworkinterfacerule.html
    pub struct PublicRouterNetworkInterfaceRule_ {
        pub cidr: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterNetworkInterface_PublicRouterNetworkInterfaceRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterNetworkInterface.PublicRouterNetworkInterfaceRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterNetworkInterface_PublicRouterNetworkInterfaceRule as PublicRouterNetworkInterfaceRule;
    impl crate::value::ToValue for PublicRouterNetworkInterfaceRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Cidr".to_string(),
                crate::value::ToValue::to_value(&self.cidr),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routernetworkinterface-routernetworkinterfaceconfiguration.html
    pub struct RouterNetworkInterfaceConfiguration_ {
        pub public: Option<Box<PublicRouterNetworkInterfaceConfiguration_>>,
        pub vpc: Option<Box<VpcRouterNetworkInterfaceConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterNetworkInterface_RouterNetworkInterfaceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterNetworkInterface.RouterNetworkInterfaceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterNetworkInterface_RouterNetworkInterfaceConfiguration as RouterNetworkInterfaceConfiguration;
    impl crate::value::ToValue for RouterNetworkInterfaceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.public {
                properties.insert("Public".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.vpc {
                properties.insert("Vpc".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routernetworkinterface-vpcrouternetworkinterfaceconfiguration.html
    pub struct VpcRouterNetworkInterfaceConfiguration_ {
        pub security_group_ids: Vec<crate::value::ExpString>,
        pub subnet_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterNetworkInterface_VpcRouterNetworkInterfaceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterNetworkInterface.VpcRouterNetworkInterfaceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterNetworkInterface_VpcRouterNetworkInterfaceConfiguration as VpcRouterNetworkInterfaceConfiguration;
    impl crate::value::ToValue for VpcRouterNetworkInterfaceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(&self.security_group_ids),
            );
            properties.insert(
                "SubnetId".to_string(),
                crate::value::ToValue::to_value(&self.subnet_id),
            );
            properties.into()
        }
    }
}
pub mod routeroutput {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routeroutput-flowtransitencryption.html
    pub struct FlowTransitEncryption_ {
        pub encryption_key_configuration: Box<FlowTransitEncryptionKeyConfiguration_>,
        pub encryption_key_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterOutput_FlowTransitEncryption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterOutput.FlowTransitEncryption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterOutput_FlowTransitEncryption as FlowTransitEncryption;
    impl crate::value::ToValue for FlowTransitEncryption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EncryptionKeyConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.encryption_key_configuration),
            );
            if let Some(ref value) = self.encryption_key_type {
                properties.insert(
                    "EncryptionKeyType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routeroutput-flowtransitencryptionkeyconfiguration.html
    pub struct FlowTransitEncryptionKeyConfiguration_ {
        pub automatic: Option<serde_json::Value>,
        pub secrets_manager: Option<Box<SecretsManagerEncryptionKeyConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterOutput_FlowTransitEncryptionKeyConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterOutput.FlowTransitEncryptionKeyConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterOutput_FlowTransitEncryptionKeyConfiguration as FlowTransitEncryptionKeyConfiguration;
    impl crate::value::ToValue for FlowTransitEncryptionKeyConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.automatic {
                properties.insert(
                    "Automatic".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secrets_manager {
                properties.insert(
                    "SecretsManager".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routeroutput-maintenanceconfiguration.html
    pub struct MaintenanceConfiguration_ {
        pub default: Option<serde_json::Value>,
        pub preferred_day_time: Option<Box<PreferredDayTimeMaintenanceConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterOutput_MaintenanceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterOutput.MaintenanceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterOutput_MaintenanceConfiguration as MaintenanceConfiguration;
    impl crate::value::ToValue for MaintenanceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.default {
                properties.insert(
                    "Default".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.preferred_day_time {
                properties.insert(
                    "PreferredDayTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routeroutput-mediaconnectflowrouteroutputconfiguration.html
    pub struct MediaConnectFlowRouterOutputConfiguration_ {
        pub destination_transit_encryption: Box<FlowTransitEncryption_>,
        pub flow_arn: Option<crate::value::ExpString>,
        pub flow_source_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterOutput_MediaConnectFlowRouterOutputConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterOutput.MediaConnectFlowRouterOutputConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterOutput_MediaConnectFlowRouterOutputConfiguration as MediaConnectFlowRouterOutputConfiguration;
    impl crate::value::ToValue for MediaConnectFlowRouterOutputConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DestinationTransitEncryption".to_string(),
                crate::value::ToValue::to_value(&self.destination_transit_encryption),
            );
            if let Some(ref value) = self.flow_arn {
                properties.insert(
                    "FlowArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.flow_source_arn {
                properties.insert(
                    "FlowSourceArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routeroutput-medialiveinputrouteroutputconfiguration.html
    pub struct MediaLiveInputRouterOutputConfiguration_ {
        pub destination_transit_encryption: Box<MediaLiveTransitEncryption_>,
        pub media_live_input_arn: Option<crate::value::ExpString>,
        pub media_live_pipeline_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterOutput_MediaLiveInputRouterOutputConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterOutput.MediaLiveInputRouterOutputConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterOutput_MediaLiveInputRouterOutputConfiguration as MediaLiveInputRouterOutputConfiguration;
    impl crate::value::ToValue for MediaLiveInputRouterOutputConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DestinationTransitEncryption".to_string(),
                crate::value::ToValue::to_value(&self.destination_transit_encryption),
            );
            if let Some(ref value) = self.media_live_input_arn {
                properties.insert(
                    "MediaLiveInputArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.media_live_pipeline_id {
                properties.insert(
                    "MediaLivePipelineId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routeroutput-medialivetransitencryption.html
    pub struct MediaLiveTransitEncryption_ {
        pub encryption_key_configuration: Box<MediaLiveTransitEncryptionKeyConfiguration_>,
        pub encryption_key_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterOutput_MediaLiveTransitEncryption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterOutput.MediaLiveTransitEncryption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterOutput_MediaLiveTransitEncryption as MediaLiveTransitEncryption;
    impl crate::value::ToValue for MediaLiveTransitEncryption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EncryptionKeyConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.encryption_key_configuration),
            );
            if let Some(ref value) = self.encryption_key_type {
                properties.insert(
                    "EncryptionKeyType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routeroutput-medialivetransitencryptionkeyconfiguration.html
    pub struct MediaLiveTransitEncryptionKeyConfiguration_ {
        pub automatic: Option<serde_json::Value>,
        pub secrets_manager: Option<Box<SecretsManagerEncryptionKeyConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterOutput_MediaLiveTransitEncryptionKeyConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterOutput.MediaLiveTransitEncryptionKeyConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterOutput_MediaLiveTransitEncryptionKeyConfiguration as MediaLiveTransitEncryptionKeyConfiguration;
    impl crate::value::ToValue for MediaLiveTransitEncryptionKeyConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.automatic {
                properties.insert(
                    "Automatic".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secrets_manager {
                properties.insert(
                    "SecretsManager".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routeroutput-preferreddaytimemaintenanceconfiguration.html
    pub struct PreferredDayTimeMaintenanceConfiguration_ {
        pub day: crate::value::ExpString,
        pub time: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterOutput_PreferredDayTimeMaintenanceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterOutput.PreferredDayTimeMaintenanceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterOutput_PreferredDayTimeMaintenanceConfiguration as PreferredDayTimeMaintenanceConfiguration;
    impl crate::value::ToValue for PreferredDayTimeMaintenanceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Day".to_string(),
                crate::value::ToValue::to_value(&self.day),
            );
            properties.insert(
                "Time".to_string(),
                crate::value::ToValue::to_value(&self.time),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routeroutput-ristrouteroutputconfiguration.html
    pub struct RistRouterOutputConfiguration_ {
        pub destination_address: crate::value::ExpString,
        pub destination_port: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterOutput_RistRouterOutputConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterOutput.RistRouterOutputConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterOutput_RistRouterOutputConfiguration as RistRouterOutputConfiguration;
    impl crate::value::ToValue for RistRouterOutputConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DestinationAddress".to_string(),
                crate::value::ToValue::to_value(&self.destination_address),
            );
            properties.insert(
                "DestinationPort".to_string(),
                crate::value::ToValue::to_value(&self.destination_port),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routeroutput-routeroutputconfiguration.html
    pub struct RouterOutputConfiguration_ {
        pub media_connect_flow: Option<Box<MediaConnectFlowRouterOutputConfiguration_>>,
        pub media_live_input: Option<Box<MediaLiveInputRouterOutputConfiguration_>>,
        pub standard: Option<Box<StandardRouterOutputConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterOutput_RouterOutputConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterOutput.RouterOutputConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterOutput_RouterOutputConfiguration as RouterOutputConfiguration;
    impl crate::value::ToValue for RouterOutputConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.media_connect_flow {
                properties.insert(
                    "MediaConnectFlow".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.media_live_input {
                properties.insert(
                    "MediaLiveInput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.standard {
                properties.insert(
                    "Standard".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routeroutput-routeroutputprotocolconfiguration.html
    pub struct RouterOutputProtocolConfiguration_ {
        pub rist: Option<Box<RistRouterOutputConfiguration_>>,
        pub rtp: Option<Box<RtpRouterOutputConfiguration_>>,
        pub srt_caller: Option<Box<SrtCallerRouterOutputConfiguration_>>,
        pub srt_listener: Option<Box<SrtListenerRouterOutputConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterOutput_RouterOutputProtocolConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterOutput.RouterOutputProtocolConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterOutput_RouterOutputProtocolConfiguration as RouterOutputProtocolConfiguration;
    impl crate::value::ToValue for RouterOutputProtocolConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.rist {
                properties.insert("Rist".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.rtp {
                properties.insert("Rtp".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.srt_caller {
                properties.insert(
                    "SrtCaller".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.srt_listener {
                properties.insert(
                    "SrtListener".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routeroutput-rtprouteroutputconfiguration.html
    pub struct RtpRouterOutputConfiguration_ {
        pub destination_address: crate::value::ExpString,
        pub destination_port: i32,
        pub forward_error_correction: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterOutput_RtpRouterOutputConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterOutput.RtpRouterOutputConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterOutput_RtpRouterOutputConfiguration as RtpRouterOutputConfiguration;
    impl crate::value::ToValue for RtpRouterOutputConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DestinationAddress".to_string(),
                crate::value::ToValue::to_value(&self.destination_address),
            );
            properties.insert(
                "DestinationPort".to_string(),
                crate::value::ToValue::to_value(&self.destination_port),
            );
            if let Some(ref value) = self.forward_error_correction {
                properties.insert(
                    "ForwardErrorCorrection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routeroutput-secretsmanagerencryptionkeyconfiguration.html
    pub struct SecretsManagerEncryptionKeyConfiguration_ {
        pub role_arn: crate::value::ExpString,
        pub secret_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterOutput_SecretsManagerEncryptionKeyConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterOutput.SecretsManagerEncryptionKeyConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterOutput_SecretsManagerEncryptionKeyConfiguration as SecretsManagerEncryptionKeyConfiguration;
    impl crate::value::ToValue for SecretsManagerEncryptionKeyConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.insert(
                "SecretArn".to_string(),
                crate::value::ToValue::to_value(&self.secret_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routeroutput-srtcallerrouteroutputconfiguration.html
    pub struct SrtCallerRouterOutputConfiguration_ {
        pub destination_address: crate::value::ExpString,
        pub destination_port: i32,
        pub encryption_configuration: Option<Box<SrtEncryptionConfiguration_>>,
        pub minimum_latency_milliseconds: i32,
        pub stream_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterOutput_SrtCallerRouterOutputConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterOutput.SrtCallerRouterOutputConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterOutput_SrtCallerRouterOutputConfiguration as SrtCallerRouterOutputConfiguration;
    impl crate::value::ToValue for SrtCallerRouterOutputConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DestinationAddress".to_string(),
                crate::value::ToValue::to_value(&self.destination_address),
            );
            properties.insert(
                "DestinationPort".to_string(),
                crate::value::ToValue::to_value(&self.destination_port),
            );
            if let Some(ref value) = self.encryption_configuration {
                properties.insert(
                    "EncryptionConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MinimumLatencyMilliseconds".to_string(),
                crate::value::ToValue::to_value(&self.minimum_latency_milliseconds),
            );
            if let Some(ref value) = self.stream_id {
                properties.insert(
                    "StreamId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routeroutput-srtencryptionconfiguration.html
    pub struct SrtEncryptionConfiguration_ {
        pub encryption_key: Box<SecretsManagerEncryptionKeyConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterOutput_SrtEncryptionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterOutput.SrtEncryptionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterOutput_SrtEncryptionConfiguration as SrtEncryptionConfiguration;
    impl crate::value::ToValue for SrtEncryptionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EncryptionKey".to_string(),
                crate::value::ToValue::to_value(&self.encryption_key),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routeroutput-srtlistenerrouteroutputconfiguration.html
    pub struct SrtListenerRouterOutputConfiguration_ {
        pub encryption_configuration: Option<Box<SrtEncryptionConfiguration_>>,
        pub minimum_latency_milliseconds: i32,
        pub port: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterOutput_SrtListenerRouterOutputConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterOutput.SrtListenerRouterOutputConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterOutput_SrtListenerRouterOutputConfiguration as SrtListenerRouterOutputConfiguration;
    impl crate::value::ToValue for SrtListenerRouterOutputConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.encryption_configuration {
                properties.insert(
                    "EncryptionConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MinimumLatencyMilliseconds".to_string(),
                crate::value::ToValue::to_value(&self.minimum_latency_milliseconds),
            );
            properties.insert(
                "Port".to_string(),
                crate::value::ToValue::to_value(&self.port),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-routeroutput-standardrouteroutputconfiguration.html
    pub struct StandardRouterOutputConfiguration_ {
        pub network_interface_arn: crate::value::ExpString,
        pub protocol: Option<crate::value::ExpString>,
        pub protocol_configuration: Box<RouterOutputProtocolConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconnect_RouterOutput_StandardRouterOutputConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConnect::RouterOutput.StandardRouterOutputConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconnect_RouterOutput_StandardRouterOutputConfiguration as StandardRouterOutputConfiguration;
    impl crate::value::ToValue for StandardRouterOutputConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "NetworkInterfaceArn".to_string(),
                crate::value::ToValue::to_value(&self.network_interface_arn),
            );
            if let Some(ref value) = self.protocol {
                properties.insert(
                    "Protocol".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ProtocolConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.protocol_configuration),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-bridge.html
pub struct Bridge_ {
    pub egress_gateway_bridge: Option<super::mediaconnect::bridge::EgressGatewayBridge_>,
    pub ingress_gateway_bridge: Option<super::mediaconnect::bridge::IngressGatewayBridge_>,
    pub name: crate::value::ExpString,
    pub outputs: Option<Vec<super::mediaconnect::bridge::BridgeOutput_>>,
    pub placement_arn: crate::value::ExpString,
    pub source_failover_config: Option<super::mediaconnect::bridge::FailoverConfig_>,
    pub sources: Vec<super::mediaconnect::bridge::BridgeSource_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_mediaconnect_Bridge {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MediaConnect::Bridge"
        $($field $value)*)
    };
}
pub use crate::__aws_mediaconnect_Bridge as Bridge;
impl crate::template::ToResource for Bridge_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaConnect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Bridge"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.egress_gateway_bridge {
            properties.insert(
                "EgressGatewayBridge".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ingress_gateway_bridge {
            properties.insert(
                "IngressGatewayBridge".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.outputs {
            properties.insert(
                "Outputs".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "PlacementArn".to_string(),
            crate::value::ToValue::to_value(&self.placement_arn),
        );
        if let Some(ref value) = self.source_failover_config {
            properties.insert(
                "SourceFailoverConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Sources".to_string(),
            crate::value::ToValue::to_value(&self.sources),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-bridgeoutput.html
pub struct BridgeOutput_ {
    pub bridge_arn: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub network_output: super::mediaconnect::bridgeoutput::BridgeNetworkOutput_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_mediaconnect_BridgeOutput {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MediaConnect::BridgeOutput"
        $($field $value)*)
    };
}
pub use crate::__aws_mediaconnect_BridgeOutput as BridgeOutput;
impl crate::template::ToResource for BridgeOutput_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaConnect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("BridgeOutput"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "BridgeArn".to_string(),
            crate::value::ToValue::to_value(&self.bridge_arn),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "NetworkOutput".to_string(),
            crate::value::ToValue::to_value(&self.network_output),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-bridgesource.html
pub struct BridgeSource_ {
    pub bridge_arn: crate::value::ExpString,
    pub flow_source: Option<super::mediaconnect::bridgesource::BridgeFlowSource_>,
    pub name: crate::value::ExpString,
    pub network_source: Option<super::mediaconnect::bridgesource::BridgeNetworkSource_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_mediaconnect_BridgeSource {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MediaConnect::BridgeSource"
        $($field $value)*)
    };
}
pub use crate::__aws_mediaconnect_BridgeSource as BridgeSource;
impl crate::template::ToResource for BridgeSource_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaConnect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("BridgeSource"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "BridgeArn".to_string(),
            crate::value::ToValue::to_value(&self.bridge_arn),
        );
        if let Some(ref value) = self.flow_source {
            properties.insert(
                "FlowSource".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.network_source {
            properties.insert(
                "NetworkSource".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flow.html
pub struct Flow_ {
    pub availability_zone: Option<crate::value::ExpString>,
    pub flow_size: Option<crate::value::ExpString>,
    pub maintenance: Option<super::mediaconnect::flow::Maintenance_>,
    pub media_streams: Option<Vec<super::mediaconnect::flow::MediaStream_>>,
    pub name: crate::value::ExpString,
    pub ndi_config: Option<super::mediaconnect::flow::NdiConfig_>,
    pub source: super::mediaconnect::flow::Source_,
    pub source_failover_config: Option<super::mediaconnect::flow::FailoverConfig_>,
    pub source_monitoring_config: Option<super::mediaconnect::flow::SourceMonitoringConfig_>,
    pub vpc_interfaces: Option<Vec<super::mediaconnect::flow::VpcInterface_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_mediaconnect_Flow {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MediaConnect::Flow"
        $($field $value)*)
    };
}
pub use crate::__aws_mediaconnect_Flow as Flow;
impl crate::template::ToResource for Flow_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaConnect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Flow"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.availability_zone {
            properties.insert(
                "AvailabilityZone".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.flow_size {
            properties.insert(
                "FlowSize".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.maintenance {
            properties.insert(
                "Maintenance".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.media_streams {
            properties.insert(
                "MediaStreams".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.ndi_config {
            properties.insert(
                "NdiConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Source".to_string(),
            crate::value::ToValue::to_value(&self.source),
        );
        if let Some(ref value) = self.source_failover_config {
            properties.insert(
                "SourceFailoverConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_monitoring_config {
            properties.insert(
                "SourceMonitoringConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.vpc_interfaces {
            properties.insert(
                "VpcInterfaces".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowentitlement.html
pub struct FlowEntitlement_ {
    pub data_transfer_subscriber_fee_percent: Option<i32>,
    pub description: crate::value::ExpString,
    pub encryption: Option<super::mediaconnect::flowentitlement::Encryption_>,
    pub entitlement_status: Option<crate::value::ExpString>,
    pub flow_arn: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub subscribers: Vec<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_mediaconnect_FlowEntitlement {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MediaConnect::FlowEntitlement"
        $($field $value)*)
    };
}
pub use crate::__aws_mediaconnect_FlowEntitlement as FlowEntitlement;
impl crate::template::ToResource for FlowEntitlement_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaConnect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("FlowEntitlement"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.data_transfer_subscriber_fee_percent {
            properties.insert(
                "DataTransferSubscriberFeePercent".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Description".to_string(),
            crate::value::ToValue::to_value(&self.description),
        );
        if let Some(ref value) = self.encryption {
            properties.insert(
                "Encryption".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.entitlement_status {
            properties.insert(
                "EntitlementStatus".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "FlowArn".to_string(),
            crate::value::ToValue::to_value(&self.flow_arn),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "Subscribers".to_string(),
            crate::value::ToValue::to_value(&self.subscribers),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowoutput.html
pub struct FlowOutput_ {
    pub cidr_allow_list: Option<Vec<crate::value::ExpString>>,
    pub description: Option<crate::value::ExpString>,
    pub destination: Option<crate::value::ExpString>,
    pub encryption: Option<super::mediaconnect::flowoutput::Encryption_>,
    pub flow_arn: crate::value::ExpString,
    pub max_latency: Option<i32>,
    pub media_stream_output_configurations:
        Option<Vec<super::mediaconnect::flowoutput::MediaStreamOutputConfiguration_>>,
    pub min_latency: Option<i32>,
    pub name: Option<crate::value::ExpString>,
    pub ndi_program_name: Option<crate::value::ExpString>,
    pub ndi_speed_hq_quality: Option<i32>,
    pub output_status: Option<crate::value::ExpString>,
    pub port: Option<i32>,
    pub protocol: Option<crate::value::ExpString>,
    pub remote_id: Option<crate::value::ExpString>,
    pub router_integration_state: Option<crate::value::ExpString>,
    pub router_integration_transit_encryption:
        Option<super::mediaconnect::flowoutput::FlowTransitEncryption_>,
    pub smoothing_latency: Option<i32>,
    pub stream_id: Option<crate::value::ExpString>,
    pub vpc_interface_attachment: Option<super::mediaconnect::flowoutput::VpcInterfaceAttachment_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_mediaconnect_FlowOutput {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MediaConnect::FlowOutput"
        $($field $value)*)
    };
}
pub use crate::__aws_mediaconnect_FlowOutput as FlowOutput;
impl crate::template::ToResource for FlowOutput_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaConnect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("FlowOutput"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.cidr_allow_list {
            properties.insert(
                "CidrAllowList".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.destination {
            properties.insert(
                "Destination".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.encryption {
            properties.insert(
                "Encryption".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "FlowArn".to_string(),
            crate::value::ToValue::to_value(&self.flow_arn),
        );
        if let Some(ref value) = self.max_latency {
            properties.insert(
                "MaxLatency".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.media_stream_output_configurations {
            properties.insert(
                "MediaStreamOutputConfigurations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.min_latency {
            properties.insert(
                "MinLatency".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.ndi_program_name {
            properties.insert(
                "NdiProgramName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ndi_speed_hq_quality {
            properties.insert(
                "NdiSpeedHqQuality".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.output_status {
            properties.insert(
                "OutputStatus".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.port {
            properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.protocol {
            properties.insert(
                "Protocol".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.remote_id {
            properties.insert(
                "RemoteId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.router_integration_state {
            properties.insert(
                "RouterIntegrationState".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.router_integration_transit_encryption {
            properties.insert(
                "RouterIntegrationTransitEncryption".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.smoothing_latency {
            properties.insert(
                "SmoothingLatency".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.stream_id {
            properties.insert(
                "StreamId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.vpc_interface_attachment {
            properties.insert(
                "VpcInterfaceAttachment".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowsource.html
pub struct FlowSource_ {
    pub decryption: Option<super::mediaconnect::flowsource::Encryption_>,
    pub description: crate::value::ExpString,
    pub entitlement_arn: Option<crate::value::ExpString>,
    pub flow_arn: Option<crate::value::ExpString>,
    pub gateway_bridge_source: Option<super::mediaconnect::flowsource::GatewayBridgeSource_>,
    pub ingest_port: Option<i32>,
    pub max_bitrate: Option<i32>,
    pub max_latency: Option<i32>,
    pub min_latency: Option<i32>,
    pub name: crate::value::ExpString,
    pub protocol: Option<crate::value::ExpString>,
    pub sender_control_port: Option<i32>,
    pub sender_ip_address: Option<crate::value::ExpString>,
    pub source_listener_address: Option<crate::value::ExpString>,
    pub source_listener_port: Option<i32>,
    pub stream_id: Option<crate::value::ExpString>,
    pub vpc_interface_name: Option<crate::value::ExpString>,
    pub whitelist_cidr: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_mediaconnect_FlowSource {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MediaConnect::FlowSource"
        $($field $value)*)
    };
}
pub use crate::__aws_mediaconnect_FlowSource as FlowSource;
impl crate::template::ToResource for FlowSource_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaConnect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("FlowSource"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.decryption {
            properties.insert(
                "Decryption".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Description".to_string(),
            crate::value::ToValue::to_value(&self.description),
        );
        if let Some(ref value) = self.entitlement_arn {
            properties.insert(
                "EntitlementArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.flow_arn {
            properties.insert(
                "FlowArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.gateway_bridge_source {
            properties.insert(
                "GatewayBridgeSource".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ingest_port {
            properties.insert(
                "IngestPort".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.max_bitrate {
            properties.insert(
                "MaxBitrate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.max_latency {
            properties.insert(
                "MaxLatency".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.min_latency {
            properties.insert(
                "MinLatency".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.protocol {
            properties.insert(
                "Protocol".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sender_control_port {
            properties.insert(
                "SenderControlPort".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sender_ip_address {
            properties.insert(
                "SenderIpAddress".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_listener_address {
            properties.insert(
                "SourceListenerAddress".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_listener_port {
            properties.insert(
                "SourceListenerPort".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.stream_id {
            properties.insert(
                "StreamId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.vpc_interface_name {
            properties.insert(
                "VpcInterfaceName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.whitelist_cidr {
            properties.insert(
                "WhitelistCidr".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowvpcinterface.html
pub struct FlowVpcInterface_ {
    pub flow_arn: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub role_arn: crate::value::ExpString,
    pub security_group_ids: Vec<crate::value::ExpString>,
    pub subnet_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_mediaconnect_FlowVpcInterface {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MediaConnect::FlowVpcInterface"
        $($field $value)*)
    };
}
pub use crate::__aws_mediaconnect_FlowVpcInterface as FlowVpcInterface;
impl crate::template::ToResource for FlowVpcInterface_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaConnect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("FlowVpcInterface"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "FlowArn".to_string(),
            crate::value::ToValue::to_value(&self.flow_arn),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        properties.insert(
            "SecurityGroupIds".to_string(),
            crate::value::ToValue::to_value(&self.security_group_ids),
        );
        properties.insert(
            "SubnetId".to_string(),
            crate::value::ToValue::to_value(&self.subnet_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-gateway.html
pub struct Gateway_ {
    pub egress_cidr_blocks: Vec<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub networks: Vec<super::mediaconnect::gateway::GatewayNetwork_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_mediaconnect_Gateway {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MediaConnect::Gateway"
        $($field $value)*)
    };
}
pub use crate::__aws_mediaconnect_Gateway as Gateway;
impl crate::template::ToResource for Gateway_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaConnect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Gateway"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "EgressCidrBlocks".to_string(),
            crate::value::ToValue::to_value(&self.egress_cidr_blocks),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "Networks".to_string(),
            crate::value::ToValue::to_value(&self.networks),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-routerinput.html
pub struct RouterInput_ {
    pub availability_zone: Option<crate::value::ExpString>,
    pub configuration: super::mediaconnect::routerinput::RouterInputConfiguration_,
    pub maintenance_configuration:
        Option<super::mediaconnect::routerinput::MaintenanceConfiguration_>,
    pub maximum_bitrate: i32,
    pub name: crate::value::ExpString,
    pub region_name: Option<crate::value::ExpString>,
    pub routing_scope: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub tier: crate::value::ExpString,
    pub transit_encryption: Option<super::mediaconnect::routerinput::RouterInputTransitEncryption_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_mediaconnect_RouterInput {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MediaConnect::RouterInput"
        $($field $value)*)
    };
}
pub use crate::__aws_mediaconnect_RouterInput as RouterInput;
impl crate::template::ToResource for RouterInput_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaConnect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RouterInput"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.availability_zone {
            properties.insert(
                "AvailabilityZone".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Configuration".to_string(),
            crate::value::ToValue::to_value(&self.configuration),
        );
        if let Some(ref value) = self.maintenance_configuration {
            properties.insert(
                "MaintenanceConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "MaximumBitrate".to_string(),
            crate::value::ToValue::to_value(&self.maximum_bitrate),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.region_name {
            properties.insert(
                "RegionName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RoutingScope".to_string(),
            crate::value::ToValue::to_value(&self.routing_scope),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Tier".to_string(),
            crate::value::ToValue::to_value(&self.tier),
        );
        if let Some(ref value) = self.transit_encryption {
            properties.insert(
                "TransitEncryption".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-routernetworkinterface.html
pub struct RouterNetworkInterface_ {
    pub configuration:
        super::mediaconnect::routernetworkinterface::RouterNetworkInterfaceConfiguration_,
    pub name: crate::value::ExpString,
    pub region_name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_mediaconnect_RouterNetworkInterface {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MediaConnect::RouterNetworkInterface"
        $($field $value)*)
    };
}
pub use crate::__aws_mediaconnect_RouterNetworkInterface as RouterNetworkInterface;
impl crate::template::ToResource for RouterNetworkInterface_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaConnect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RouterNetworkInterface"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Configuration".to_string(),
            crate::value::ToValue::to_value(&self.configuration),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.region_name {
            properties.insert(
                "RegionName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-routeroutput.html
pub struct RouterOutput_ {
    pub availability_zone: Option<crate::value::ExpString>,
    pub configuration: super::mediaconnect::routeroutput::RouterOutputConfiguration_,
    pub maintenance_configuration:
        Option<super::mediaconnect::routeroutput::MaintenanceConfiguration_>,
    pub maximum_bitrate: i32,
    pub name: crate::value::ExpString,
    pub region_name: Option<crate::value::ExpString>,
    pub routing_scope: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub tier: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_mediaconnect_RouterOutput {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MediaConnect::RouterOutput"
        $($field $value)*)
    };
}
pub use crate::__aws_mediaconnect_RouterOutput as RouterOutput;
impl crate::template::ToResource for RouterOutput_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaConnect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RouterOutput"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.availability_zone {
            properties.insert(
                "AvailabilityZone".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Configuration".to_string(),
            crate::value::ToValue::to_value(&self.configuration),
        );
        if let Some(ref value) = self.maintenance_configuration {
            properties.insert(
                "MaintenanceConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "MaximumBitrate".to_string(),
            crate::value::ToValue::to_value(&self.maximum_bitrate),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.region_name {
            properties.insert(
                "RegionName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RoutingScope".to_string(),
            crate::value::ToValue::to_value(&self.routing_scope),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Tier".to_string(),
            crate::value::ToValue::to_value(&self.tier),
        );
        properties
    }
}
