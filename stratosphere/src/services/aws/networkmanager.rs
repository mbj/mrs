pub mod connectattachment {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-connectattachment-connectattachmentoptions.html
    pub struct ConnectAttachmentOptions_ {
        pub protocol: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkmanager_ConnectAttachment_ConnectAttachmentOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::NetworkManager::ConnectAttachment.ConnectAttachmentOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkmanager_ConnectAttachment_ConnectAttachmentOptions as ConnectAttachmentOptions;
    impl crate::value::ToValue for ConnectAttachmentOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.protocol {
                properties.insert(
                    "Protocol".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-connectattachment-proposednetworkfunctiongroupchange.html
    pub struct ProposedNetworkFunctionGroupChange_ {
        pub attachment_policy_rule_number: Option<i32>,
        pub network_function_group_name: Option<crate::value::ExpString>,
        pub tags: Option<Vec<crate::Tag_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkmanager_ConnectAttachment_ProposedNetworkFunctionGroupChange {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::NetworkManager::ConnectAttachment.ProposedNetworkFunctionGroupChange"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkmanager_ConnectAttachment_ProposedNetworkFunctionGroupChange as ProposedNetworkFunctionGroupChange;
    impl crate::value::ToValue for ProposedNetworkFunctionGroupChange_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attachment_policy_rule_number {
                properties.insert(
                    "AttachmentPolicyRuleNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_function_group_name {
                properties.insert(
                    "NetworkFunctionGroupName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tags {
                properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-connectattachment-proposedsegmentchange.html
    pub struct ProposedSegmentChange_ {
        pub attachment_policy_rule_number: Option<i32>,
        pub segment_name: Option<crate::value::ExpString>,
        pub tags: Option<Vec<crate::Tag_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkmanager_ConnectAttachment_ProposedSegmentChange {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::NetworkManager::ConnectAttachment.ProposedSegmentChange"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkmanager_ConnectAttachment_ProposedSegmentChange as ProposedSegmentChange;
    impl crate::value::ToValue for ProposedSegmentChange_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attachment_policy_rule_number {
                properties.insert(
                    "AttachmentPolicyRuleNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.segment_name {
                properties.insert(
                    "SegmentName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tags {
                properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod connectpeer {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-connectpeer-bgpoptions.html
    pub struct BgpOptions_ {
        pub peer_asn: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkmanager_ConnectPeer_BgpOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::NetworkManager::ConnectPeer.BgpOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkmanager_ConnectPeer_BgpOptions as BgpOptions;
    impl crate::value::ToValue for BgpOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.peer_asn {
                properties.insert(
                    "PeerAsn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-connectpeer-connectpeerbgpconfiguration.html
    pub struct ConnectPeerBgpConfiguration_ {
        pub core_network_address: Option<crate::value::ExpString>,
        pub core_network_asn: Option<f64>,
        pub peer_address: Option<crate::value::ExpString>,
        pub peer_asn: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkmanager_ConnectPeer_ConnectPeerBgpConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::NetworkManager::ConnectPeer.ConnectPeerBgpConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkmanager_ConnectPeer_ConnectPeerBgpConfiguration as ConnectPeerBgpConfiguration;
    impl crate::value::ToValue for ConnectPeerBgpConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.core_network_address {
                properties.insert(
                    "CoreNetworkAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.core_network_asn {
                properties.insert(
                    "CoreNetworkAsn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.peer_address {
                properties.insert(
                    "PeerAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.peer_asn {
                properties.insert(
                    "PeerAsn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-connectpeer-connectpeerconfiguration.html
    pub struct ConnectPeerConfiguration_ {
        pub bgp_configurations: Option<Vec<ConnectPeerBgpConfiguration_>>,
        pub core_network_address: Option<crate::value::ExpString>,
        pub inside_cidr_blocks: Option<Vec<crate::value::ExpString>>,
        pub peer_address: Option<crate::value::ExpString>,
        pub protocol: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkmanager_ConnectPeer_ConnectPeerConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::NetworkManager::ConnectPeer.ConnectPeerConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkmanager_ConnectPeer_ConnectPeerConfiguration as ConnectPeerConfiguration;
    impl crate::value::ToValue for ConnectPeerConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bgp_configurations {
                properties.insert(
                    "BgpConfigurations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.core_network_address {
                properties.insert(
                    "CoreNetworkAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inside_cidr_blocks {
                properties.insert(
                    "InsideCidrBlocks".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.peer_address {
                properties.insert(
                    "PeerAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.protocol {
                properties.insert(
                    "Protocol".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod corenetwork {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-corenetwork-corenetworkedge.html
    pub struct CoreNetworkEdge_ {
        pub asn: Option<f64>,
        pub edge_location: Option<crate::value::ExpString>,
        pub inside_cidr_blocks: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkmanager_CoreNetwork_CoreNetworkEdge {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::NetworkManager::CoreNetwork.CoreNetworkEdge"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkmanager_CoreNetwork_CoreNetworkEdge as CoreNetworkEdge;
    impl crate::value::ToValue for CoreNetworkEdge_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.asn {
                properties.insert("Asn".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.edge_location {
                properties.insert(
                    "EdgeLocation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inside_cidr_blocks {
                properties.insert(
                    "InsideCidrBlocks".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-corenetwork-corenetworknetworkfunctiongroup.html
    pub struct CoreNetworkNetworkFunctionGroup_ {
        pub edge_locations: Option<Vec<crate::value::ExpString>>,
        pub name: Option<crate::value::ExpString>,
        pub segments: Option<Box<Segments_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkmanager_CoreNetwork_CoreNetworkNetworkFunctionGroup {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::NetworkManager::CoreNetwork.CoreNetworkNetworkFunctionGroup"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkmanager_CoreNetwork_CoreNetworkNetworkFunctionGroup as CoreNetworkNetworkFunctionGroup;
    impl crate::value::ToValue for CoreNetworkNetworkFunctionGroup_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.edge_locations {
                properties.insert(
                    "EdgeLocations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.segments {
                properties.insert(
                    "Segments".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-corenetwork-corenetworksegment.html
    pub struct CoreNetworkSegment_ {
        pub edge_locations: Option<Vec<crate::value::ExpString>>,
        pub name: Option<crate::value::ExpString>,
        pub shared_segments: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkmanager_CoreNetwork_CoreNetworkSegment {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::NetworkManager::CoreNetwork.CoreNetworkSegment"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkmanager_CoreNetwork_CoreNetworkSegment as CoreNetworkSegment;
    impl crate::value::ToValue for CoreNetworkSegment_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.edge_locations {
                properties.insert(
                    "EdgeLocations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.shared_segments {
                properties.insert(
                    "SharedSegments".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-corenetwork-segments.html
    pub struct Segments_ {
        pub send_to: Option<Vec<crate::value::ExpString>>,
        pub send_via: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkmanager_CoreNetwork_Segments {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::NetworkManager::CoreNetwork.Segments"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkmanager_CoreNetwork_Segments as Segments;
    impl crate::value::ToValue for Segments_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.send_to {
                properties.insert("SendTo".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.send_via {
                properties.insert(
                    "SendVia".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod device {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-device-awslocation.html
    pub struct AWSLocation_ {
        pub subnet_arn: Option<crate::value::ExpString>,
        pub zone: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkmanager_Device_AWSLocation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::NetworkManager::Device.AWSLocation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkmanager_Device_AWSLocation as AWSLocation;
    impl crate::value::ToValue for AWSLocation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.subnet_arn {
                properties.insert(
                    "SubnetArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.zone {
                properties.insert("Zone".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-device-location.html
    pub struct Location_ {
        pub address: Option<crate::value::ExpString>,
        pub latitude: Option<crate::value::ExpString>,
        pub longitude: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkmanager_Device_Location {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::NetworkManager::Device.Location"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkmanager_Device_Location as Location;
    impl crate::value::ToValue for Location_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.address {
                properties.insert(
                    "Address".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.latitude {
                properties.insert(
                    "Latitude".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.longitude {
                properties.insert(
                    "Longitude".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod directconnectgatewayattachment {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-directconnectgatewayattachment-proposednetworkfunctiongroupchange.html
    pub struct ProposedNetworkFunctionGroupChange_ {
        pub attachment_policy_rule_number: Option<i32>,
        pub network_function_group_name: Option<crate::value::ExpString>,
        pub tags: Option<Vec<crate::Tag_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkmanager_DirectConnectGatewayAttachment_ProposedNetworkFunctionGroupChange {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::NetworkManager::DirectConnectGatewayAttachment.ProposedNetworkFunctionGroupChange"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkmanager_DirectConnectGatewayAttachment_ProposedNetworkFunctionGroupChange as ProposedNetworkFunctionGroupChange;
    impl crate::value::ToValue for ProposedNetworkFunctionGroupChange_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attachment_policy_rule_number {
                properties.insert(
                    "AttachmentPolicyRuleNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_function_group_name {
                properties.insert(
                    "NetworkFunctionGroupName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tags {
                properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-directconnectgatewayattachment-proposedsegmentchange.html
    pub struct ProposedSegmentChange_ {
        pub attachment_policy_rule_number: Option<i32>,
        pub segment_name: Option<crate::value::ExpString>,
        pub tags: Option<Vec<crate::Tag_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkmanager_DirectConnectGatewayAttachment_ProposedSegmentChange {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::NetworkManager::DirectConnectGatewayAttachment.ProposedSegmentChange"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkmanager_DirectConnectGatewayAttachment_ProposedSegmentChange as ProposedSegmentChange;
    impl crate::value::ToValue for ProposedSegmentChange_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attachment_policy_rule_number {
                properties.insert(
                    "AttachmentPolicyRuleNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.segment_name {
                properties.insert(
                    "SegmentName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tags {
                properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod link {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-link-bandwidth.html
    pub struct Bandwidth_ {
        pub download_speed: Option<i32>,
        pub upload_speed: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkmanager_Link_Bandwidth {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::NetworkManager::Link.Bandwidth"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkmanager_Link_Bandwidth as Bandwidth;
    impl crate::value::ToValue for Bandwidth_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.download_speed {
                properties.insert(
                    "DownloadSpeed".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.upload_speed {
                properties.insert(
                    "UploadSpeed".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod site {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-site-location.html
    pub struct Location_ {
        pub address: Option<crate::value::ExpString>,
        pub latitude: Option<crate::value::ExpString>,
        pub longitude: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkmanager_Site_Location {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::NetworkManager::Site.Location"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkmanager_Site_Location as Location;
    impl crate::value::ToValue for Location_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.address {
                properties.insert(
                    "Address".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.latitude {
                properties.insert(
                    "Latitude".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.longitude {
                properties.insert(
                    "Longitude".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod sitetositevpnattachment {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-sitetositevpnattachment-proposednetworkfunctiongroupchange.html
    pub struct ProposedNetworkFunctionGroupChange_ {
        pub attachment_policy_rule_number: Option<i32>,
        pub network_function_group_name: Option<crate::value::ExpString>,
        pub tags: Option<Vec<crate::Tag_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkmanager_SiteToSiteVpnAttachment_ProposedNetworkFunctionGroupChange {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::NetworkManager::SiteToSiteVpnAttachment.ProposedNetworkFunctionGroupChange"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkmanager_SiteToSiteVpnAttachment_ProposedNetworkFunctionGroupChange as ProposedNetworkFunctionGroupChange;
    impl crate::value::ToValue for ProposedNetworkFunctionGroupChange_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attachment_policy_rule_number {
                properties.insert(
                    "AttachmentPolicyRuleNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_function_group_name {
                properties.insert(
                    "NetworkFunctionGroupName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tags {
                properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-sitetositevpnattachment-proposedsegmentchange.html
    pub struct ProposedSegmentChange_ {
        pub attachment_policy_rule_number: Option<i32>,
        pub segment_name: Option<crate::value::ExpString>,
        pub tags: Option<Vec<crate::Tag_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkmanager_SiteToSiteVpnAttachment_ProposedSegmentChange {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::NetworkManager::SiteToSiteVpnAttachment.ProposedSegmentChange"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkmanager_SiteToSiteVpnAttachment_ProposedSegmentChange as ProposedSegmentChange;
    impl crate::value::ToValue for ProposedSegmentChange_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attachment_policy_rule_number {
                properties.insert(
                    "AttachmentPolicyRuleNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.segment_name {
                properties.insert(
                    "SegmentName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tags {
                properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod transitgatewayroutetableattachment {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-transitgatewayroutetableattachment-proposednetworkfunctiongroupchange.html
    pub struct ProposedNetworkFunctionGroupChange_ {
        pub attachment_policy_rule_number: Option<i32>,
        pub network_function_group_name: Option<crate::value::ExpString>,
        pub tags: Option<Vec<crate::Tag_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkmanager_TransitGatewayRouteTableAttachment_ProposedNetworkFunctionGroupChange {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::NetworkManager::TransitGatewayRouteTableAttachment.ProposedNetworkFunctionGroupChange"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkmanager_TransitGatewayRouteTableAttachment_ProposedNetworkFunctionGroupChange as ProposedNetworkFunctionGroupChange;
    impl crate::value::ToValue for ProposedNetworkFunctionGroupChange_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attachment_policy_rule_number {
                properties.insert(
                    "AttachmentPolicyRuleNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_function_group_name {
                properties.insert(
                    "NetworkFunctionGroupName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tags {
                properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-transitgatewayroutetableattachment-proposedsegmentchange.html
    pub struct ProposedSegmentChange_ {
        pub attachment_policy_rule_number: Option<i32>,
        pub segment_name: Option<crate::value::ExpString>,
        pub tags: Option<Vec<crate::Tag_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkmanager_TransitGatewayRouteTableAttachment_ProposedSegmentChange {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::NetworkManager::TransitGatewayRouteTableAttachment.ProposedSegmentChange"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkmanager_TransitGatewayRouteTableAttachment_ProposedSegmentChange as ProposedSegmentChange;
    impl crate::value::ToValue for ProposedSegmentChange_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attachment_policy_rule_number {
                properties.insert(
                    "AttachmentPolicyRuleNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.segment_name {
                properties.insert(
                    "SegmentName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tags {
                properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod vpcattachment {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-vpcattachment-proposednetworkfunctiongroupchange.html
    pub struct ProposedNetworkFunctionGroupChange_ {
        pub attachment_policy_rule_number: Option<i32>,
        pub network_function_group_name: Option<crate::value::ExpString>,
        pub tags: Option<Vec<crate::Tag_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkmanager_VpcAttachment_ProposedNetworkFunctionGroupChange {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::NetworkManager::VpcAttachment.ProposedNetworkFunctionGroupChange"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkmanager_VpcAttachment_ProposedNetworkFunctionGroupChange as ProposedNetworkFunctionGroupChange;
    impl crate::value::ToValue for ProposedNetworkFunctionGroupChange_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attachment_policy_rule_number {
                properties.insert(
                    "AttachmentPolicyRuleNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_function_group_name {
                properties.insert(
                    "NetworkFunctionGroupName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tags {
                properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-vpcattachment-proposedsegmentchange.html
    pub struct ProposedSegmentChange_ {
        pub attachment_policy_rule_number: Option<i32>,
        pub segment_name: Option<crate::value::ExpString>,
        pub tags: Option<Vec<crate::Tag_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkmanager_VpcAttachment_ProposedSegmentChange {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::NetworkManager::VpcAttachment.ProposedSegmentChange"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkmanager_VpcAttachment_ProposedSegmentChange as ProposedSegmentChange;
    impl crate::value::ToValue for ProposedSegmentChange_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attachment_policy_rule_number {
                properties.insert(
                    "AttachmentPolicyRuleNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.segment_name {
                properties.insert(
                    "SegmentName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tags {
                properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-vpcattachment-vpcoptions.html
    pub struct VpcOptions_ {
        pub appliance_mode_support: Option<crate::value::ExpBool>,
        pub dns_support: Option<crate::value::ExpBool>,
        pub ipv6_support: Option<crate::value::ExpBool>,
        pub security_group_referencing_support: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkmanager_VpcAttachment_VpcOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::NetworkManager::VpcAttachment.VpcOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkmanager_VpcAttachment_VpcOptions as VpcOptions;
    impl crate::value::ToValue for VpcOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.appliance_mode_support {
                properties.insert(
                    "ApplianceModeSupport".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dns_support {
                properties.insert(
                    "DnsSupport".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ipv6_support {
                properties.insert(
                    "Ipv6Support".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.security_group_referencing_support {
                properties.insert(
                    "SecurityGroupReferencingSupport".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-connectattachment.html
pub struct ConnectAttachment_ {
    pub core_network_id: crate::value::ExpString,
    pub edge_location: crate::value::ExpString,
    pub network_function_group_name: Option<crate::value::ExpString>,
    pub options: super::networkmanager::connectattachment::ConnectAttachmentOptions_,
    pub proposed_network_function_group_change:
        Option<super::networkmanager::connectattachment::ProposedNetworkFunctionGroupChange_>,
    pub proposed_segment_change:
        Option<super::networkmanager::connectattachment::ProposedSegmentChange_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub transport_attachment_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_networkmanager_ConnectAttachment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::NetworkManager::ConnectAttachment"
        $($field $value)*)
    };
}
pub use crate::__aws_networkmanager_ConnectAttachment as ConnectAttachment;
impl crate::template::ToResource for ConnectAttachment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("NetworkManager"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ConnectAttachment"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "CoreNetworkId".to_string(),
            crate::value::ToValue::to_value(&self.core_network_id),
        );
        properties.insert(
            "EdgeLocation".to_string(),
            crate::value::ToValue::to_value(&self.edge_location),
        );
        if let Some(ref value) = self.network_function_group_name {
            properties.insert(
                "NetworkFunctionGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Options".to_string(),
            crate::value::ToValue::to_value(&self.options),
        );
        if let Some(ref value) = self.proposed_network_function_group_change {
            properties.insert(
                "ProposedNetworkFunctionGroupChange".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.proposed_segment_change {
            properties.insert(
                "ProposedSegmentChange".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TransportAttachmentId".to_string(),
            crate::value::ToValue::to_value(&self.transport_attachment_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-connectpeer.html
pub struct ConnectPeer_ {
    pub bgp_options: Option<super::networkmanager::connectpeer::BgpOptions_>,
    pub connect_attachment_id: crate::value::ExpString,
    pub core_network_address: Option<crate::value::ExpString>,
    pub inside_cidr_blocks: Option<Vec<crate::value::ExpString>>,
    pub peer_address: crate::value::ExpString,
    pub subnet_arn: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_networkmanager_ConnectPeer {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::NetworkManager::ConnectPeer"
        $($field $value)*)
    };
}
pub use crate::__aws_networkmanager_ConnectPeer as ConnectPeer;
impl crate::template::ToResource for ConnectPeer_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("NetworkManager"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ConnectPeer"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.bgp_options {
            properties.insert(
                "BgpOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ConnectAttachmentId".to_string(),
            crate::value::ToValue::to_value(&self.connect_attachment_id),
        );
        if let Some(ref value) = self.core_network_address {
            properties.insert(
                "CoreNetworkAddress".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.inside_cidr_blocks {
            properties.insert(
                "InsideCidrBlocks".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "PeerAddress".to_string(),
            crate::value::ToValue::to_value(&self.peer_address),
        );
        if let Some(ref value) = self.subnet_arn {
            properties.insert(
                "SubnetArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-corenetwork.html
pub struct CoreNetwork_ {
    pub description: Option<crate::value::ExpString>,
    pub global_network_id: crate::value::ExpString,
    pub policy_document: Option<serde_json::Value>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_networkmanager_CoreNetwork {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::NetworkManager::CoreNetwork"
        $($field $value)*)
    };
}
pub use crate::__aws_networkmanager_CoreNetwork as CoreNetwork;
impl crate::template::ToResource for CoreNetwork_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("NetworkManager"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("CoreNetwork"),
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
            "GlobalNetworkId".to_string(),
            crate::value::ToValue::to_value(&self.global_network_id),
        );
        if let Some(ref value) = self.policy_document {
            properties.insert(
                "PolicyDocument".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-customergatewayassociation.html
pub struct CustomerGatewayAssociation_ {
    pub customer_gateway_arn: crate::value::ExpString,
    pub device_id: crate::value::ExpString,
    pub global_network_id: crate::value::ExpString,
    pub link_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_networkmanager_CustomerGatewayAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::NetworkManager::CustomerGatewayAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_networkmanager_CustomerGatewayAssociation as CustomerGatewayAssociation;
impl crate::template::ToResource for CustomerGatewayAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("NetworkManager"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "CustomerGatewayAssociation",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "CustomerGatewayArn".to_string(),
            crate::value::ToValue::to_value(&self.customer_gateway_arn),
        );
        properties.insert(
            "DeviceId".to_string(),
            crate::value::ToValue::to_value(&self.device_id),
        );
        properties.insert(
            "GlobalNetworkId".to_string(),
            crate::value::ToValue::to_value(&self.global_network_id),
        );
        if let Some(ref value) = self.link_id {
            properties.insert("LinkId".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-device.html
pub struct Device_ {
    pub aws_location: Option<super::networkmanager::device::AWSLocation_>,
    pub description: Option<crate::value::ExpString>,
    pub global_network_id: crate::value::ExpString,
    pub location: Option<super::networkmanager::device::Location_>,
    pub model: Option<crate::value::ExpString>,
    pub serial_number: Option<crate::value::ExpString>,
    pub site_id: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub r#type: Option<crate::value::ExpString>,
    pub vendor: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_networkmanager_Device {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::NetworkManager::Device"
        $($field $value)*)
    };
}
pub use crate::__aws_networkmanager_Device as Device;
impl crate::template::ToResource for Device_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("NetworkManager"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Device"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.aws_location {
            properties.insert(
                "AWSLocation".to_string(),
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
            "GlobalNetworkId".to_string(),
            crate::value::ToValue::to_value(&self.global_network_id),
        );
        if let Some(ref value) = self.location {
            properties.insert(
                "Location".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.model {
            properties.insert("Model".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.serial_number {
            properties.insert(
                "SerialNumber".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.site_id {
            properties.insert("SiteId".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.r#type {
            properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.vendor {
            properties.insert("Vendor".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-directconnectgatewayattachment.html
pub struct DirectConnectGatewayAttachment_ {
    pub core_network_id: crate::value::ExpString,
    pub direct_connect_gateway_arn: crate::value::ExpString,
    pub edge_locations: Vec<crate::value::ExpString>,
    pub proposed_network_function_group_change: Option<
        super::networkmanager::directconnectgatewayattachment::ProposedNetworkFunctionGroupChange_,
    >,
    pub proposed_segment_change:
        Option<super::networkmanager::directconnectgatewayattachment::ProposedSegmentChange_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_networkmanager_DirectConnectGatewayAttachment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::NetworkManager::DirectConnectGatewayAttachment"
        $($field $value)*)
    };
}
pub use crate::__aws_networkmanager_DirectConnectGatewayAttachment as DirectConnectGatewayAttachment;
impl crate::template::ToResource for DirectConnectGatewayAttachment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("NetworkManager"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "DirectConnectGatewayAttachment",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "CoreNetworkId".to_string(),
            crate::value::ToValue::to_value(&self.core_network_id),
        );
        properties.insert(
            "DirectConnectGatewayArn".to_string(),
            crate::value::ToValue::to_value(&self.direct_connect_gateway_arn),
        );
        properties.insert(
            "EdgeLocations".to_string(),
            crate::value::ToValue::to_value(&self.edge_locations),
        );
        if let Some(ref value) = self.proposed_network_function_group_change {
            properties.insert(
                "ProposedNetworkFunctionGroupChange".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.proposed_segment_change {
            properties.insert(
                "ProposedSegmentChange".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-globalnetwork.html
pub struct GlobalNetwork_ {
    pub created_at: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub state: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_networkmanager_GlobalNetwork {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::NetworkManager::GlobalNetwork"
        $($field $value)*)
    };
}
pub use crate::__aws_networkmanager_GlobalNetwork as GlobalNetwork;
impl crate::template::ToResource for GlobalNetwork_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("NetworkManager"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("GlobalNetwork"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.created_at {
            properties.insert(
                "CreatedAt".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.state {
            properties.insert("State".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-link.html
pub struct Link_ {
    pub bandwidth: super::networkmanager::link::Bandwidth_,
    pub description: Option<crate::value::ExpString>,
    pub global_network_id: crate::value::ExpString,
    pub provider: Option<crate::value::ExpString>,
    pub site_id: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub r#type: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_networkmanager_Link {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::NetworkManager::Link"
        $($field $value)*)
    };
}
pub use crate::__aws_networkmanager_Link as Link;
impl crate::template::ToResource for Link_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("NetworkManager"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Link"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Bandwidth".to_string(),
            crate::value::ToValue::to_value(&self.bandwidth),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "GlobalNetworkId".to_string(),
            crate::value::ToValue::to_value(&self.global_network_id),
        );
        if let Some(ref value) = self.provider {
            properties.insert(
                "Provider".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SiteId".to_string(),
            crate::value::ToValue::to_value(&self.site_id),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.r#type {
            properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-linkassociation.html
pub struct LinkAssociation_ {
    pub device_id: crate::value::ExpString,
    pub global_network_id: crate::value::ExpString,
    pub link_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_networkmanager_LinkAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::NetworkManager::LinkAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_networkmanager_LinkAssociation as LinkAssociation;
impl crate::template::ToResource for LinkAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("NetworkManager"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LinkAssociation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DeviceId".to_string(),
            crate::value::ToValue::to_value(&self.device_id),
        );
        properties.insert(
            "GlobalNetworkId".to_string(),
            crate::value::ToValue::to_value(&self.global_network_id),
        );
        properties.insert(
            "LinkId".to_string(),
            crate::value::ToValue::to_value(&self.link_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-site.html
pub struct Site_ {
    pub description: Option<crate::value::ExpString>,
    pub global_network_id: crate::value::ExpString,
    pub location: Option<super::networkmanager::site::Location_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_networkmanager_Site {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::NetworkManager::Site"
        $($field $value)*)
    };
}
pub use crate::__aws_networkmanager_Site as Site;
impl crate::template::ToResource for Site_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("NetworkManager"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Site"),
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
            "GlobalNetworkId".to_string(),
            crate::value::ToValue::to_value(&self.global_network_id),
        );
        if let Some(ref value) = self.location {
            properties.insert(
                "Location".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-sitetositevpnattachment.html
pub struct SiteToSiteVpnAttachment_ {
    pub core_network_id: crate::value::ExpString,
    pub network_function_group_name: Option<crate::value::ExpString>,
    pub proposed_network_function_group_change:
        Option<super::networkmanager::sitetositevpnattachment::ProposedNetworkFunctionGroupChange_>,
    pub proposed_segment_change:
        Option<super::networkmanager::sitetositevpnattachment::ProposedSegmentChange_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpn_connection_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_networkmanager_SiteToSiteVpnAttachment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::NetworkManager::SiteToSiteVpnAttachment"
        $($field $value)*)
    };
}
pub use crate::__aws_networkmanager_SiteToSiteVpnAttachment as SiteToSiteVpnAttachment;
impl crate::template::ToResource for SiteToSiteVpnAttachment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("NetworkManager"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SiteToSiteVpnAttachment"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "CoreNetworkId".to_string(),
            crate::value::ToValue::to_value(&self.core_network_id),
        );
        if let Some(ref value) = self.network_function_group_name {
            properties.insert(
                "NetworkFunctionGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.proposed_network_function_group_change {
            properties.insert(
                "ProposedNetworkFunctionGroupChange".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.proposed_segment_change {
            properties.insert(
                "ProposedSegmentChange".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "VpnConnectionArn".to_string(),
            crate::value::ToValue::to_value(&self.vpn_connection_arn),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-transitgatewaypeering.html
pub struct TransitGatewayPeering_ {
    pub core_network_id: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub transit_gateway_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_networkmanager_TransitGatewayPeering {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::NetworkManager::TransitGatewayPeering"
        $($field $value)*)
    };
}
pub use crate::__aws_networkmanager_TransitGatewayPeering as TransitGatewayPeering;
impl crate::template::ToResource for TransitGatewayPeering_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("NetworkManager"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TransitGatewayPeering"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "CoreNetworkId".to_string(),
            crate::value::ToValue::to_value(&self.core_network_id),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TransitGatewayArn".to_string(),
            crate::value::ToValue::to_value(&self.transit_gateway_arn),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-transitgatewayregistration.html
pub struct TransitGatewayRegistration_ {
    pub global_network_id: crate::value::ExpString,
    pub transit_gateway_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_networkmanager_TransitGatewayRegistration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::NetworkManager::TransitGatewayRegistration"
        $($field $value)*)
    };
}
pub use crate::__aws_networkmanager_TransitGatewayRegistration as TransitGatewayRegistration;
impl crate::template::ToResource for TransitGatewayRegistration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("NetworkManager"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "TransitGatewayRegistration",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "GlobalNetworkId".to_string(),
            crate::value::ToValue::to_value(&self.global_network_id),
        );
        properties.insert(
            "TransitGatewayArn".to_string(),
            crate::value::ToValue::to_value(&self.transit_gateway_arn),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-transitgatewayroutetableattachment.html
pub struct TransitGatewayRouteTableAttachment_ {
    pub network_function_group_name: Option<crate::value::ExpString>,
    pub peering_id: crate::value::ExpString,
    pub proposed_network_function_group_change: Option<
        super::networkmanager::transitgatewayroutetableattachment::ProposedNetworkFunctionGroupChange_,
    >,
    pub proposed_segment_change: Option<
        super::networkmanager::transitgatewayroutetableattachment::ProposedSegmentChange_,
    >,
    pub tags: Option<Vec<crate::Tag_>>,
    pub transit_gateway_route_table_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_networkmanager_TransitGatewayRouteTableAttachment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::NetworkManager::TransitGatewayRouteTableAttachment"
        $($field $value)*)
    };
}
pub use crate::__aws_networkmanager_TransitGatewayRouteTableAttachment as TransitGatewayRouteTableAttachment;
impl crate::template::ToResource for TransitGatewayRouteTableAttachment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("NetworkManager"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "TransitGatewayRouteTableAttachment",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.network_function_group_name {
            properties.insert(
                "NetworkFunctionGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "PeeringId".to_string(),
            crate::value::ToValue::to_value(&self.peering_id),
        );
        if let Some(ref value) = self.proposed_network_function_group_change {
            properties.insert(
                "ProposedNetworkFunctionGroupChange".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.proposed_segment_change {
            properties.insert(
                "ProposedSegmentChange".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TransitGatewayRouteTableArn".to_string(),
            crate::value::ToValue::to_value(&self.transit_gateway_route_table_arn),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-vpcattachment.html
pub struct VpcAttachment_ {
    pub core_network_id: crate::value::ExpString,
    pub options: Option<super::networkmanager::vpcattachment::VpcOptions_>,
    pub proposed_network_function_group_change:
        Option<super::networkmanager::vpcattachment::ProposedNetworkFunctionGroupChange_>,
    pub proposed_segment_change:
        Option<super::networkmanager::vpcattachment::ProposedSegmentChange_>,
    pub subnet_arns: Vec<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_networkmanager_VpcAttachment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::NetworkManager::VpcAttachment"
        $($field $value)*)
    };
}
pub use crate::__aws_networkmanager_VpcAttachment as VpcAttachment;
impl crate::template::ToResource for VpcAttachment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("NetworkManager"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("VpcAttachment"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "CoreNetworkId".to_string(),
            crate::value::ToValue::to_value(&self.core_network_id),
        );
        if let Some(ref value) = self.options {
            properties.insert(
                "Options".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.proposed_network_function_group_change {
            properties.insert(
                "ProposedNetworkFunctionGroupChange".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.proposed_segment_change {
            properties.insert(
                "ProposedSegmentChange".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SubnetArns".to_string(),
            crate::value::ToValue::to_value(&self.subnet_arns),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "VpcArn".to_string(),
            crate::value::ToValue::to_value(&self.vpc_arn),
        );
        properties
    }
}
