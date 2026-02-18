pub mod environment {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evs-environment-check.html>
    pub struct Check_ {
        pub impaired_since: Option<crate::value::ExpString>,
        pub result: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_evs_Environment_Check {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EVS::Environment.Check"
            $($field $value)*)
        };
    }
    pub use crate::__aws_evs_Environment_Check as Check;
    impl crate::value::ToValue for Check_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.impaired_since {
                properties.insert(
                    "ImpairedSince".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Result".to_string(),
                crate::value::ToValue::to_value(&self.result),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evs-environment-connectivityinfo.html>
    pub struct ConnectivityInfo_ {
        pub private_route_server_peerings: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_evs_Environment_ConnectivityInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EVS::Environment.ConnectivityInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_evs_Environment_ConnectivityInfo as ConnectivityInfo;
    impl crate::value::ToValue for ConnectivityInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PrivateRouteServerPeerings".to_string(),
                crate::value::ToValue::to_value(&self.private_route_server_peerings),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evs-environment-hostinfoforcreate.html>
    pub struct HostInfoForCreate_ {
        pub dedicated_host_id: Option<crate::value::ExpString>,
        pub host_name: crate::value::ExpString,
        pub instance_type: crate::value::ExpString,
        pub key_name: crate::value::ExpString,
        pub placement_group_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_evs_Environment_HostInfoForCreate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EVS::Environment.HostInfoForCreate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_evs_Environment_HostInfoForCreate as HostInfoForCreate;
    impl crate::value::ToValue for HostInfoForCreate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dedicated_host_id {
                properties.insert(
                    "DedicatedHostId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "HostName".to_string(),
                crate::value::ToValue::to_value(&self.host_name),
            );
            properties.insert(
                "InstanceType".to_string(),
                crate::value::ToValue::to_value(&self.instance_type),
            );
            properties.insert(
                "KeyName".to_string(),
                crate::value::ToValue::to_value(&self.key_name),
            );
            if let Some(ref value) = self.placement_group_id {
                properties.insert(
                    "PlacementGroupId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evs-environment-initialvlaninfo.html>
    pub struct InitialVlanInfo_ {
        pub cidr: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_evs_Environment_InitialVlanInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EVS::Environment.InitialVlanInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_evs_Environment_InitialVlanInfo as InitialVlanInfo;
    impl crate::value::ToValue for InitialVlanInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Cidr".to_string(),
                crate::value::ToValue::to_value(&self.cidr),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evs-environment-initialvlans.html>
    pub struct InitialVlans_ {
        pub edge_v_tep: Box<InitialVlanInfo_>,
        pub expansion_vlan1: Box<InitialVlanInfo_>,
        pub expansion_vlan2: Box<InitialVlanInfo_>,
        pub hcx: Box<InitialVlanInfo_>,
        pub hcx_network_acl_id: Option<crate::value::ExpString>,
        pub is_hcx_public: Option<crate::value::ExpBool>,
        pub nsx_up_link: Box<InitialVlanInfo_>,
        pub v_motion: Box<InitialVlanInfo_>,
        pub v_san: Box<InitialVlanInfo_>,
        pub v_tep: Box<InitialVlanInfo_>,
        pub vm_management: Box<InitialVlanInfo_>,
        pub vmk_management: Box<InitialVlanInfo_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_evs_Environment_InitialVlans {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EVS::Environment.InitialVlans"
            $($field $value)*)
        };
    }
    pub use crate::__aws_evs_Environment_InitialVlans as InitialVlans;
    impl crate::value::ToValue for InitialVlans_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EdgeVTep".to_string(),
                crate::value::ToValue::to_value(&self.edge_v_tep),
            );
            properties.insert(
                "ExpansionVlan1".to_string(),
                crate::value::ToValue::to_value(&self.expansion_vlan1),
            );
            properties.insert(
                "ExpansionVlan2".to_string(),
                crate::value::ToValue::to_value(&self.expansion_vlan2),
            );
            properties.insert(
                "Hcx".to_string(),
                crate::value::ToValue::to_value(&self.hcx),
            );
            if let Some(ref value) = self.hcx_network_acl_id {
                properties.insert(
                    "HcxNetworkAclId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.is_hcx_public {
                properties.insert(
                    "IsHcxPublic".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "NsxUpLink".to_string(),
                crate::value::ToValue::to_value(&self.nsx_up_link),
            );
            properties.insert(
                "VMotion".to_string(),
                crate::value::ToValue::to_value(&self.v_motion),
            );
            properties.insert(
                "VSan".to_string(),
                crate::value::ToValue::to_value(&self.v_san),
            );
            properties.insert(
                "VTep".to_string(),
                crate::value::ToValue::to_value(&self.v_tep),
            );
            properties.insert(
                "VmManagement".to_string(),
                crate::value::ToValue::to_value(&self.vm_management),
            );
            properties.insert(
                "VmkManagement".to_string(),
                crate::value::ToValue::to_value(&self.vmk_management),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evs-environment-licenseinfo.html>
    pub struct LicenseInfo_ {
        pub solution_key: crate::value::ExpString,
        pub vsan_key: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_evs_Environment_LicenseInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EVS::Environment.LicenseInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_evs_Environment_LicenseInfo as LicenseInfo;
    impl crate::value::ToValue for LicenseInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SolutionKey".to_string(),
                crate::value::ToValue::to_value(&self.solution_key),
            );
            properties.insert(
                "VsanKey".to_string(),
                crate::value::ToValue::to_value(&self.vsan_key),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evs-environment-secret.html>
    pub struct Secret_ {
        pub secret_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_evs_Environment_Secret {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EVS::Environment.Secret"
            $($field $value)*)
        };
    }
    pub use crate::__aws_evs_Environment_Secret as Secret;
    impl crate::value::ToValue for Secret_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.secret_arn {
                properties.insert(
                    "SecretArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evs-environment-serviceaccesssecuritygroups.html>
    pub struct ServiceAccessSecurityGroups_ {
        pub security_groups: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_evs_Environment_ServiceAccessSecurityGroups {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EVS::Environment.ServiceAccessSecurityGroups"
            $($field $value)*)
        };
    }
    pub use crate::__aws_evs_Environment_ServiceAccessSecurityGroups as ServiceAccessSecurityGroups;
    impl crate::value::ToValue for ServiceAccessSecurityGroups_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.security_groups {
                properties.insert(
                    "SecurityGroups".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evs-environment-vcfhostnames.html>
    pub struct VcfHostnames_ {
        pub cloud_builder: crate::value::ExpString,
        pub nsx: crate::value::ExpString,
        pub nsx_edge1: crate::value::ExpString,
        pub nsx_edge2: crate::value::ExpString,
        pub nsx_manager1: crate::value::ExpString,
        pub nsx_manager2: crate::value::ExpString,
        pub nsx_manager3: crate::value::ExpString,
        pub sddc_manager: crate::value::ExpString,
        pub v_center: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_evs_Environment_VcfHostnames {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EVS::Environment.VcfHostnames"
            $($field $value)*)
        };
    }
    pub use crate::__aws_evs_Environment_VcfHostnames as VcfHostnames;
    impl crate::value::ToValue for VcfHostnames_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CloudBuilder".to_string(),
                crate::value::ToValue::to_value(&self.cloud_builder),
            );
            properties.insert(
                "Nsx".to_string(),
                crate::value::ToValue::to_value(&self.nsx),
            );
            properties.insert(
                "NsxEdge1".to_string(),
                crate::value::ToValue::to_value(&self.nsx_edge1),
            );
            properties.insert(
                "NsxEdge2".to_string(),
                crate::value::ToValue::to_value(&self.nsx_edge2),
            );
            properties.insert(
                "NsxManager1".to_string(),
                crate::value::ToValue::to_value(&self.nsx_manager1),
            );
            properties.insert(
                "NsxManager2".to_string(),
                crate::value::ToValue::to_value(&self.nsx_manager2),
            );
            properties.insert(
                "NsxManager3".to_string(),
                crate::value::ToValue::to_value(&self.nsx_manager3),
            );
            properties.insert(
                "SddcManager".to_string(),
                crate::value::ToValue::to_value(&self.sddc_manager),
            );
            properties.insert(
                "VCenter".to_string(),
                crate::value::ToValue::to_value(&self.v_center),
            );
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evs-environment.html>
pub struct Environment_ {
    pub connectivity_info: super::evs::environment::ConnectivityInfo_,
    pub environment_name: Option<crate::value::ExpString>,
    pub hosts: Option<Vec<super::evs::environment::HostInfoForCreate_>>,
    pub initial_vlans: Option<super::evs::environment::InitialVlans_>,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub license_info: super::evs::environment::LicenseInfo_,
    pub service_access_security_groups:
        Option<super::evs::environment::ServiceAccessSecurityGroups_>,
    pub service_access_subnet_id: crate::value::ExpString,
    pub site_id: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub terms_accepted: crate::value::ExpBool,
    pub vcf_hostnames: super::evs::environment::VcfHostnames_,
    pub vcf_version: crate::value::ExpString,
    pub vpc_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_evs_Environment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EVS::Environment"
        $($field $value)*)
    };
}
pub use crate::__aws_evs_Environment as Environment;
impl crate::template::ToResource for Environment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EVS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Environment"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ConnectivityInfo".to_string(),
            crate::value::ToValue::to_value(&self.connectivity_info),
        );
        if let Some(ref value) = self.environment_name {
            properties.insert(
                "EnvironmentName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.hosts {
            properties.insert("Hosts".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.initial_vlans {
            properties.insert(
                "InitialVlans".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_id {
            properties.insert(
                "KmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "LicenseInfo".to_string(),
            crate::value::ToValue::to_value(&self.license_info),
        );
        if let Some(ref value) = self.service_access_security_groups {
            properties.insert(
                "ServiceAccessSecurityGroups".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ServiceAccessSubnetId".to_string(),
            crate::value::ToValue::to_value(&self.service_access_subnet_id),
        );
        properties.insert(
            "SiteId".to_string(),
            crate::value::ToValue::to_value(&self.site_id),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TermsAccepted".to_string(),
            crate::value::ToValue::to_value(&self.terms_accepted),
        );
        properties.insert(
            "VcfHostnames".to_string(),
            crate::value::ToValue::to_value(&self.vcf_hostnames),
        );
        properties.insert(
            "VcfVersion".to_string(),
            crate::value::ToValue::to_value(&self.vcf_version),
        );
        properties.insert(
            "VpcId".to_string(),
            crate::value::ToValue::to_value(&self.vpc_id),
        );
        properties
    }
}
