pub mod member {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-managedblockchain-member-approvalthresholdpolicy.html
    pub struct ApprovalThresholdPolicy_ {
        pub proposal_duration_in_hours: Option<i32>,
        pub threshold_comparator: Option<crate::value::ExpString>,
        pub threshold_percentage: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_managedblockchain_Member_ApprovalThresholdPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ManagedBlockchain::Member.ApprovalThresholdPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_managedblockchain_Member_ApprovalThresholdPolicy as ApprovalThresholdPolicy;
    impl crate::value::ToValue for ApprovalThresholdPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.proposal_duration_in_hours {
                properties.insert(
                    "ProposalDurationInHours".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.threshold_comparator {
                properties.insert(
                    "ThresholdComparator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.threshold_percentage {
                properties.insert(
                    "ThresholdPercentage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-managedblockchain-member-memberconfiguration.html
    pub struct MemberConfiguration_ {
        pub description: Option<crate::value::ExpString>,
        pub member_framework_configuration: Option<Box<MemberFrameworkConfiguration_>>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_managedblockchain_Member_MemberConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ManagedBlockchain::Member.MemberConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_managedblockchain_Member_MemberConfiguration as MemberConfiguration;
    impl crate::value::ToValue for MemberConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.member_framework_configuration {
                properties.insert(
                    "MemberFrameworkConfiguration".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-managedblockchain-member-memberfabricconfiguration.html
    pub struct MemberFabricConfiguration_ {
        pub admin_password: crate::value::ExpString,
        pub admin_username: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_managedblockchain_Member_MemberFabricConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ManagedBlockchain::Member.MemberFabricConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_managedblockchain_Member_MemberFabricConfiguration as MemberFabricConfiguration;
    impl crate::value::ToValue for MemberFabricConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AdminPassword".to_string(),
                crate::value::ToValue::to_value(&self.admin_password),
            );
            properties.insert(
                "AdminUsername".to_string(),
                crate::value::ToValue::to_value(&self.admin_username),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-managedblockchain-member-memberframeworkconfiguration.html
    pub struct MemberFrameworkConfiguration_ {
        pub member_fabric_configuration: Option<Box<MemberFabricConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_managedblockchain_Member_MemberFrameworkConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ManagedBlockchain::Member.MemberFrameworkConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_managedblockchain_Member_MemberFrameworkConfiguration as MemberFrameworkConfiguration;
    impl crate::value::ToValue for MemberFrameworkConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.member_fabric_configuration {
                properties.insert(
                    "MemberFabricConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-managedblockchain-member-networkconfiguration.html
    pub struct NetworkConfiguration_ {
        pub description: Option<crate::value::ExpString>,
        pub framework: crate::value::ExpString,
        pub framework_version: crate::value::ExpString,
        pub name: crate::value::ExpString,
        pub network_framework_configuration: Option<Box<NetworkFrameworkConfiguration_>>,
        pub voting_policy: Box<VotingPolicy_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_managedblockchain_Member_NetworkConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ManagedBlockchain::Member.NetworkConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_managedblockchain_Member_NetworkConfiguration as NetworkConfiguration;
    impl crate::value::ToValue for NetworkConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Framework".to_string(),
                crate::value::ToValue::to_value(&self.framework),
            );
            properties.insert(
                "FrameworkVersion".to_string(),
                crate::value::ToValue::to_value(&self.framework_version),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.network_framework_configuration {
                properties.insert(
                    "NetworkFrameworkConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "VotingPolicy".to_string(),
                crate::value::ToValue::to_value(&self.voting_policy),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-managedblockchain-member-networkfabricconfiguration.html
    pub struct NetworkFabricConfiguration_ {
        pub edition: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_managedblockchain_Member_NetworkFabricConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ManagedBlockchain::Member.NetworkFabricConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_managedblockchain_Member_NetworkFabricConfiguration as NetworkFabricConfiguration;
    impl crate::value::ToValue for NetworkFabricConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Edition".to_string(),
                crate::value::ToValue::to_value(&self.edition),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-managedblockchain-member-networkframeworkconfiguration.html
    pub struct NetworkFrameworkConfiguration_ {
        pub network_fabric_configuration: Option<Box<NetworkFabricConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_managedblockchain_Member_NetworkFrameworkConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ManagedBlockchain::Member.NetworkFrameworkConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_managedblockchain_Member_NetworkFrameworkConfiguration as NetworkFrameworkConfiguration;
    impl crate::value::ToValue for NetworkFrameworkConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.network_fabric_configuration {
                properties.insert(
                    "NetworkFabricConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-managedblockchain-member-votingpolicy.html
    pub struct VotingPolicy_ {
        pub approval_threshold_policy: Option<Box<ApprovalThresholdPolicy_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_managedblockchain_Member_VotingPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ManagedBlockchain::Member.VotingPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_managedblockchain_Member_VotingPolicy as VotingPolicy;
    impl crate::value::ToValue for VotingPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.approval_threshold_policy {
                properties.insert(
                    "ApprovalThresholdPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod node {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-managedblockchain-node-nodeconfiguration.html
    pub struct NodeConfiguration_ {
        pub availability_zone: crate::value::ExpString,
        pub instance_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_managedblockchain_Node_NodeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ManagedBlockchain::Node.NodeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_managedblockchain_Node_NodeConfiguration as NodeConfiguration;
    impl crate::value::ToValue for NodeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AvailabilityZone".to_string(),
                crate::value::ToValue::to_value(&self.availability_zone),
            );
            properties.insert(
                "InstanceType".to_string(),
                crate::value::ToValue::to_value(&self.instance_type),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-managedblockchain-accessor.html
pub struct Accessor_ {
    pub accessor_type: crate::value::ExpString,
    pub network_type: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_managedblockchain_Accessor {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ManagedBlockchain::Accessor"
        $($field $value)*)
    };
}
pub use crate::__aws_managedblockchain_Accessor as Accessor;
impl crate::template::ToResource for Accessor_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ManagedBlockchain"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Accessor"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AccessorType".to_string(),
            crate::value::ToValue::to_value(&self.accessor_type),
        );
        if let Some(ref value) = self.network_type {
            properties.insert(
                "NetworkType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-managedblockchain-member.html
pub struct Member_ {
    pub invitation_id: Option<crate::value::ExpString>,
    pub member_configuration: super::managedblockchain::member::MemberConfiguration_,
    pub network_configuration: Option<super::managedblockchain::member::NetworkConfiguration_>,
    pub network_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_managedblockchain_Member {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ManagedBlockchain::Member"
        $($field $value)*)
    };
}
pub use crate::__aws_managedblockchain_Member as Member;
impl crate::template::ToResource for Member_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ManagedBlockchain"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Member"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.invitation_id {
            properties.insert(
                "InvitationId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "MemberConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.member_configuration),
        );
        if let Some(ref value) = self.network_configuration {
            properties.insert(
                "NetworkConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.network_id {
            properties.insert(
                "NetworkId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-managedblockchain-node.html
pub struct Node_ {
    pub member_id: Option<crate::value::ExpString>,
    pub network_id: crate::value::ExpString,
    pub node_configuration: super::managedblockchain::node::NodeConfiguration_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_managedblockchain_Node {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ManagedBlockchain::Node"
        $($field $value)*)
    };
}
pub use crate::__aws_managedblockchain_Node as Node;
impl crate::template::ToResource for Node_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ManagedBlockchain"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Node"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.member_id {
            properties.insert(
                "MemberId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "NetworkId".to_string(),
            crate::value::ToValue::to_value(&self.network_id),
        );
        properties.insert(
            "NodeConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.node_configuration),
        );
        properties
    }
}
