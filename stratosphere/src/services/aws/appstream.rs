pub mod appblock {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-appblock-s3location.html
    pub struct S3Location_ {
        pub s3_bucket: crate::value::ExpString,
        pub s3_key: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appstream_AppBlock_S3Location {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppStream::AppBlock.S3Location"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appstream_AppBlock_S3Location as S3Location;
    impl crate::value::ToValue for S3Location_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "S3Bucket".to_string(),
                crate::value::ToValue::to_value(&self.s3_bucket),
            );
            if let Some(ref value) = self.s3_key {
                properties.insert("S3Key".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-appblock-scriptdetails.html
    pub struct ScriptDetails_ {
        pub executable_parameters: Option<crate::value::ExpString>,
        pub executable_path: crate::value::ExpString,
        pub script_s3_location: Box<S3Location_>,
        pub timeout_in_seconds: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appstream_AppBlock_ScriptDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppStream::AppBlock.ScriptDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appstream_AppBlock_ScriptDetails as ScriptDetails;
    impl crate::value::ToValue for ScriptDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.executable_parameters {
                properties.insert(
                    "ExecutableParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ExecutablePath".to_string(),
                crate::value::ToValue::to_value(&self.executable_path),
            );
            properties.insert(
                "ScriptS3Location".to_string(),
                crate::value::ToValue::to_value(&self.script_s3_location),
            );
            properties.insert(
                "TimeoutInSeconds".to_string(),
                crate::value::ToValue::to_value(&self.timeout_in_seconds),
            );
            properties.into()
        }
    }
}
pub mod appblockbuilder {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-appblockbuilder-accessendpoint.html
    pub struct AccessEndpoint_ {
        pub endpoint_type: crate::value::ExpString,
        pub vpce_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appstream_AppBlockBuilder_AccessEndpoint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppStream::AppBlockBuilder.AccessEndpoint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appstream_AppBlockBuilder_AccessEndpoint as AccessEndpoint;
    impl crate::value::ToValue for AccessEndpoint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EndpointType".to_string(),
                crate::value::ToValue::to_value(&self.endpoint_type),
            );
            properties.insert(
                "VpceId".to_string(),
                crate::value::ToValue::to_value(&self.vpce_id),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-appblockbuilder-vpcconfig.html
    pub struct VpcConfig_ {
        pub security_group_ids: Option<Vec<crate::value::ExpString>>,
        pub subnet_ids: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appstream_AppBlockBuilder_VpcConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppStream::AppBlockBuilder.VpcConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appstream_AppBlockBuilder_VpcConfig as VpcConfig;
    impl crate::value::ToValue for VpcConfig_ {
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
}
pub mod application {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-application-s3location.html
    pub struct S3Location_ {
        pub s3_bucket: crate::value::ExpString,
        pub s3_key: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appstream_Application_S3Location {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppStream::Application.S3Location"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appstream_Application_S3Location as S3Location;
    impl crate::value::ToValue for S3Location_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "S3Bucket".to_string(),
                crate::value::ToValue::to_value(&self.s3_bucket),
            );
            properties.insert(
                "S3Key".to_string(),
                crate::value::ToValue::to_value(&self.s3_key),
            );
            properties.into()
        }
    }
}
pub mod directoryconfig {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-directoryconfig-certificatebasedauthproperties.html
    pub struct CertificateBasedAuthProperties_ {
        pub certificate_authority_arn: Option<crate::value::ExpString>,
        pub status: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appstream_DirectoryConfig_CertificateBasedAuthProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppStream::DirectoryConfig.CertificateBasedAuthProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appstream_DirectoryConfig_CertificateBasedAuthProperties as CertificateBasedAuthProperties;
    impl crate::value::ToValue for CertificateBasedAuthProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.certificate_authority_arn {
                properties.insert(
                    "CertificateAuthorityArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-directoryconfig-serviceaccountcredentials.html
    pub struct ServiceAccountCredentials_ {
        pub account_name: crate::value::ExpString,
        pub account_password: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appstream_DirectoryConfig_ServiceAccountCredentials {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppStream::DirectoryConfig.ServiceAccountCredentials"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appstream_DirectoryConfig_ServiceAccountCredentials as ServiceAccountCredentials;
    impl crate::value::ToValue for ServiceAccountCredentials_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AccountName".to_string(),
                crate::value::ToValue::to_value(&self.account_name),
            );
            properties.insert(
                "AccountPassword".to_string(),
                crate::value::ToValue::to_value(&self.account_password),
            );
            properties.into()
        }
    }
}
pub mod entitlement {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-entitlement-attribute.html
    pub struct Attribute_ {
        pub name: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appstream_Entitlement_Attribute {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppStream::Entitlement.Attribute"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appstream_Entitlement_Attribute as Attribute;
    impl crate::value::ToValue for Attribute_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
}
pub mod fleet {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-fleet-computecapacity.html
    pub struct ComputeCapacity_ {
        pub desired_instances: Option<i64>,
        pub desired_sessions: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appstream_Fleet_ComputeCapacity {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppStream::Fleet.ComputeCapacity"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appstream_Fleet_ComputeCapacity as ComputeCapacity;
    impl crate::value::ToValue for ComputeCapacity_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.desired_instances {
                properties.insert(
                    "DesiredInstances".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.desired_sessions {
                properties.insert(
                    "DesiredSessions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-fleet-domainjoininfo.html
    pub struct DomainJoinInfo_ {
        pub directory_name: Option<crate::value::ExpString>,
        pub organizational_unit_distinguished_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appstream_Fleet_DomainJoinInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppStream::Fleet.DomainJoinInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appstream_Fleet_DomainJoinInfo as DomainJoinInfo;
    impl crate::value::ToValue for DomainJoinInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.directory_name {
                properties.insert(
                    "DirectoryName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.organizational_unit_distinguished_name {
                properties.insert(
                    "OrganizationalUnitDistinguishedName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-fleet-s3location.html
    pub struct S3Location_ {
        pub s3_bucket: crate::value::ExpString,
        pub s3_key: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appstream_Fleet_S3Location {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppStream::Fleet.S3Location"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appstream_Fleet_S3Location as S3Location;
    impl crate::value::ToValue for S3Location_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "S3Bucket".to_string(),
                crate::value::ToValue::to_value(&self.s3_bucket),
            );
            properties.insert(
                "S3Key".to_string(),
                crate::value::ToValue::to_value(&self.s3_key),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-fleet-vpcconfig.html
    pub struct VpcConfig_ {
        pub security_group_ids: Option<Vec<crate::value::ExpString>>,
        pub subnet_ids: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appstream_Fleet_VpcConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppStream::Fleet.VpcConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appstream_Fleet_VpcConfig as VpcConfig;
    impl crate::value::ToValue for VpcConfig_ {
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
}
pub mod imagebuilder {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-imagebuilder-accessendpoint.html
    pub struct AccessEndpoint_ {
        pub endpoint_type: crate::value::ExpString,
        pub vpce_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appstream_ImageBuilder_AccessEndpoint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppStream::ImageBuilder.AccessEndpoint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appstream_ImageBuilder_AccessEndpoint as AccessEndpoint;
    impl crate::value::ToValue for AccessEndpoint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EndpointType".to_string(),
                crate::value::ToValue::to_value(&self.endpoint_type),
            );
            properties.insert(
                "VpceId".to_string(),
                crate::value::ToValue::to_value(&self.vpce_id),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-imagebuilder-domainjoininfo.html
    pub struct DomainJoinInfo_ {
        pub directory_name: Option<crate::value::ExpString>,
        pub organizational_unit_distinguished_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appstream_ImageBuilder_DomainJoinInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppStream::ImageBuilder.DomainJoinInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appstream_ImageBuilder_DomainJoinInfo as DomainJoinInfo;
    impl crate::value::ToValue for DomainJoinInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.directory_name {
                properties.insert(
                    "DirectoryName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.organizational_unit_distinguished_name {
                properties.insert(
                    "OrganizationalUnitDistinguishedName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-imagebuilder-vpcconfig.html
    pub struct VpcConfig_ {
        pub security_group_ids: Option<Vec<crate::value::ExpString>>,
        pub subnet_ids: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appstream_ImageBuilder_VpcConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppStream::ImageBuilder.VpcConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appstream_ImageBuilder_VpcConfig as VpcConfig;
    impl crate::value::ToValue for VpcConfig_ {
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
}
pub mod stack {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-stack-accessendpoint.html
    pub struct AccessEndpoint_ {
        pub endpoint_type: crate::value::ExpString,
        pub vpce_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appstream_Stack_AccessEndpoint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppStream::Stack.AccessEndpoint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appstream_Stack_AccessEndpoint as AccessEndpoint;
    impl crate::value::ToValue for AccessEndpoint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EndpointType".to_string(),
                crate::value::ToValue::to_value(&self.endpoint_type),
            );
            properties.insert(
                "VpceId".to_string(),
                crate::value::ToValue::to_value(&self.vpce_id),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-stack-applicationsettings.html
    pub struct ApplicationSettings_ {
        pub enabled: crate::value::ExpBool,
        pub settings_group: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appstream_Stack_ApplicationSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppStream::Stack.ApplicationSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appstream_Stack_ApplicationSettings as ApplicationSettings;
    impl crate::value::ToValue for ApplicationSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            if let Some(ref value) = self.settings_group {
                properties.insert(
                    "SettingsGroup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-stack-storageconnector.html
    pub struct StorageConnector_ {
        pub connector_type: crate::value::ExpString,
        pub domains: Option<Vec<crate::value::ExpString>>,
        pub resource_identifier: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appstream_Stack_StorageConnector {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppStream::Stack.StorageConnector"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appstream_Stack_StorageConnector as StorageConnector;
    impl crate::value::ToValue for StorageConnector_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ConnectorType".to_string(),
                crate::value::ToValue::to_value(&self.connector_type),
            );
            if let Some(ref value) = self.domains {
                properties.insert(
                    "Domains".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_identifier {
                properties.insert(
                    "ResourceIdentifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-stack-streamingexperiencesettings.html
    pub struct StreamingExperienceSettings_ {
        pub preferred_protocol: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appstream_Stack_StreamingExperienceSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppStream::Stack.StreamingExperienceSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appstream_Stack_StreamingExperienceSettings as StreamingExperienceSettings;
    impl crate::value::ToValue for StreamingExperienceSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.preferred_protocol {
                properties.insert(
                    "PreferredProtocol".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appstream-stack-usersetting.html
    pub struct UserSetting_ {
        pub action: crate::value::ExpString,
        pub maximum_length: Option<i64>,
        pub permission: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appstream_Stack_UserSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppStream::Stack.UserSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appstream_Stack_UserSetting as UserSetting;
    impl crate::value::ToValue for UserSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Action".to_string(),
                crate::value::ToValue::to_value(&self.action),
            );
            if let Some(ref value) = self.maximum_length {
                properties.insert(
                    "MaximumLength".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Permission".to_string(),
                crate::value::ToValue::to_value(&self.permission),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-appblock.html
pub struct AppBlock_ {
    pub description: Option<crate::value::ExpString>,
    pub display_name: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub packaging_type: Option<crate::value::ExpString>,
    pub post_setup_script_details: Option<super::appstream::appblock::ScriptDetails_>,
    pub setup_script_details: Option<super::appstream::appblock::ScriptDetails_>,
    pub source_s3_location: super::appstream::appblock::S3Location_,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appstream_AppBlock {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::AppStream::AppBlock"
        $($field $value)*)
    };
}
pub use crate::__aws_appstream_AppBlock as AppBlock;
impl crate::template::ToResource for AppBlock_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppStream"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AppBlock"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.display_name {
            properties.insert(
                "DisplayName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.packaging_type {
            properties.insert(
                "PackagingType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.post_setup_script_details {
            properties.insert(
                "PostSetupScriptDetails".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.setup_script_details {
            properties.insert(
                "SetupScriptDetails".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SourceS3Location".to_string(),
            crate::value::ToValue::to_value(&self.source_s3_location),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-appblockbuilder.html
pub struct AppBlockBuilder_ {
    pub access_endpoints: Option<Vec<super::appstream::appblockbuilder::AccessEndpoint_>>,
    pub app_block_arns: Option<Vec<crate::value::ExpString>>,
    pub description: Option<crate::value::ExpString>,
    pub display_name: Option<crate::value::ExpString>,
    pub enable_default_internet_access: Option<crate::value::ExpBool>,
    pub iam_role_arn: Option<crate::value::ExpString>,
    pub instance_type: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub platform: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_config: super::appstream::appblockbuilder::VpcConfig_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appstream_AppBlockBuilder {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::AppStream::AppBlockBuilder"
        $($field $value)*)
    };
}
pub use crate::__aws_appstream_AppBlockBuilder as AppBlockBuilder;
impl crate::template::ToResource for AppBlockBuilder_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppStream"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AppBlockBuilder"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.access_endpoints {
            properties.insert(
                "AccessEndpoints".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.app_block_arns {
            properties.insert(
                "AppBlockArns".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.display_name {
            properties.insert(
                "DisplayName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enable_default_internet_access {
            properties.insert(
                "EnableDefaultInternetAccess".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.iam_role_arn {
            properties.insert(
                "IamRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "InstanceType".to_string(),
            crate::value::ToValue::to_value(&self.instance_type),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "Platform".to_string(),
            crate::value::ToValue::to_value(&self.platform),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "VpcConfig".to_string(),
            crate::value::ToValue::to_value(&self.vpc_config),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-application.html
pub struct Application_ {
    pub app_block_arn: crate::value::ExpString,
    pub attributes_to_delete: Option<Vec<crate::value::ExpString>>,
    pub description: Option<crate::value::ExpString>,
    pub display_name: Option<crate::value::ExpString>,
    pub icon_s3_location: super::appstream::application::S3Location_,
    pub instance_families: Vec<crate::value::ExpString>,
    pub launch_parameters: Option<crate::value::ExpString>,
    pub launch_path: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub platforms: Vec<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub working_directory: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appstream_Application {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::AppStream::Application"
        $($field $value)*)
    };
}
pub use crate::__aws_appstream_Application as Application;
impl crate::template::ToResource for Application_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppStream"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Application"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AppBlockArn".to_string(),
            crate::value::ToValue::to_value(&self.app_block_arn),
        );
        if let Some(ref value) = self.attributes_to_delete {
            properties.insert(
                "AttributesToDelete".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.display_name {
            properties.insert(
                "DisplayName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "IconS3Location".to_string(),
            crate::value::ToValue::to_value(&self.icon_s3_location),
        );
        properties.insert(
            "InstanceFamilies".to_string(),
            crate::value::ToValue::to_value(&self.instance_families),
        );
        if let Some(ref value) = self.launch_parameters {
            properties.insert(
                "LaunchParameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "LaunchPath".to_string(),
            crate::value::ToValue::to_value(&self.launch_path),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "Platforms".to_string(),
            crate::value::ToValue::to_value(&self.platforms),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.working_directory {
            properties.insert(
                "WorkingDirectory".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-applicationentitlementassociation.html
pub struct ApplicationEntitlementAssociation_ {
    pub application_identifier: crate::value::ExpString,
    pub entitlement_name: crate::value::ExpString,
    pub stack_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appstream_ApplicationEntitlementAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::AppStream::ApplicationEntitlementAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_appstream_ApplicationEntitlementAssociation as ApplicationEntitlementAssociation;
impl crate::template::ToResource for ApplicationEntitlementAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppStream"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "ApplicationEntitlementAssociation",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApplicationIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.application_identifier),
        );
        properties.insert(
            "EntitlementName".to_string(),
            crate::value::ToValue::to_value(&self.entitlement_name),
        );
        properties.insert(
            "StackName".to_string(),
            crate::value::ToValue::to_value(&self.stack_name),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-applicationfleetassociation.html
pub struct ApplicationFleetAssociation_ {
    pub application_arn: crate::value::ExpString,
    pub fleet_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appstream_ApplicationFleetAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::AppStream::ApplicationFleetAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_appstream_ApplicationFleetAssociation as ApplicationFleetAssociation;
impl crate::template::ToResource for ApplicationFleetAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppStream"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "ApplicationFleetAssociation",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApplicationArn".to_string(),
            crate::value::ToValue::to_value(&self.application_arn),
        );
        properties.insert(
            "FleetName".to_string(),
            crate::value::ToValue::to_value(&self.fleet_name),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-directoryconfig.html
pub struct DirectoryConfig_ {
    pub certificate_based_auth_properties:
        Option<super::appstream::directoryconfig::CertificateBasedAuthProperties_>,
    pub directory_name: crate::value::ExpString,
    pub organizational_unit_distinguished_names: Vec<crate::value::ExpString>,
    pub service_account_credentials: super::appstream::directoryconfig::ServiceAccountCredentials_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appstream_DirectoryConfig {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::AppStream::DirectoryConfig"
        $($field $value)*)
    };
}
pub use crate::__aws_appstream_DirectoryConfig as DirectoryConfig;
impl crate::template::ToResource for DirectoryConfig_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppStream"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DirectoryConfig"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.certificate_based_auth_properties {
            properties.insert(
                "CertificateBasedAuthProperties".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DirectoryName".to_string(),
            crate::value::ToValue::to_value(&self.directory_name),
        );
        properties.insert(
            "OrganizationalUnitDistinguishedNames".to_string(),
            crate::value::ToValue::to_value(&self.organizational_unit_distinguished_names),
        );
        properties.insert(
            "ServiceAccountCredentials".to_string(),
            crate::value::ToValue::to_value(&self.service_account_credentials),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-entitlement.html
pub struct Entitlement_ {
    pub app_visibility: crate::value::ExpString,
    pub attributes: Vec<super::appstream::entitlement::Attribute_>,
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub stack_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appstream_Entitlement {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::AppStream::Entitlement"
        $($field $value)*)
    };
}
pub use crate::__aws_appstream_Entitlement as Entitlement;
impl crate::template::ToResource for Entitlement_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppStream"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Entitlement"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AppVisibility".to_string(),
            crate::value::ToValue::to_value(&self.app_visibility),
        );
        properties.insert(
            "Attributes".to_string(),
            crate::value::ToValue::to_value(&self.attributes),
        );
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
        properties.insert(
            "StackName".to_string(),
            crate::value::ToValue::to_value(&self.stack_name),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-fleet.html
pub struct Fleet_ {
    pub compute_capacity: Option<super::appstream::fleet::ComputeCapacity_>,
    pub description: Option<crate::value::ExpString>,
    pub disconnect_timeout_in_seconds: Option<i64>,
    pub display_name: Option<crate::value::ExpString>,
    pub domain_join_info: Option<super::appstream::fleet::DomainJoinInfo_>,
    pub enable_default_internet_access: Option<crate::value::ExpBool>,
    pub fleet_type: Option<crate::value::ExpString>,
    pub iam_role_arn: Option<crate::value::ExpString>,
    pub idle_disconnect_timeout_in_seconds: Option<i64>,
    pub image_arn: Option<crate::value::ExpString>,
    pub image_name: Option<crate::value::ExpString>,
    pub instance_type: crate::value::ExpString,
    pub max_concurrent_sessions: Option<i64>,
    pub max_sessions_per_instance: Option<i64>,
    pub max_user_duration_in_seconds: Option<i64>,
    pub name: crate::value::ExpString,
    pub platform: Option<crate::value::ExpString>,
    pub session_script_s3_location: Option<super::appstream::fleet::S3Location_>,
    pub stream_view: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub usb_device_filter_strings: Option<Vec<crate::value::ExpString>>,
    pub vpc_config: Option<super::appstream::fleet::VpcConfig_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appstream_Fleet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::AppStream::Fleet" $($field
        $value)*)
    };
}
pub use crate::__aws_appstream_Fleet as Fleet;
impl crate::template::ToResource for Fleet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppStream"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Fleet"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.compute_capacity {
            properties.insert(
                "ComputeCapacity".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.disconnect_timeout_in_seconds {
            properties.insert(
                "DisconnectTimeoutInSeconds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.display_name {
            properties.insert(
                "DisplayName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.domain_join_info {
            properties.insert(
                "DomainJoinInfo".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enable_default_internet_access {
            properties.insert(
                "EnableDefaultInternetAccess".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.fleet_type {
            properties.insert(
                "FleetType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.iam_role_arn {
            properties.insert(
                "IamRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.idle_disconnect_timeout_in_seconds {
            properties.insert(
                "IdleDisconnectTimeoutInSeconds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.image_arn {
            properties.insert(
                "ImageArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.image_name {
            properties.insert(
                "ImageName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "InstanceType".to_string(),
            crate::value::ToValue::to_value(&self.instance_type),
        );
        if let Some(ref value) = self.max_concurrent_sessions {
            properties.insert(
                "MaxConcurrentSessions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.max_sessions_per_instance {
            properties.insert(
                "MaxSessionsPerInstance".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.max_user_duration_in_seconds {
            properties.insert(
                "MaxUserDurationInSeconds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.platform {
            properties.insert(
                "Platform".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.session_script_s3_location {
            properties.insert(
                "SessionScriptS3Location".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.stream_view {
            properties.insert(
                "StreamView".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.usb_device_filter_strings {
            properties.insert(
                "UsbDeviceFilterStrings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.vpc_config {
            properties.insert(
                "VpcConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-imagebuilder.html
pub struct ImageBuilder_ {
    pub access_endpoints: Option<Vec<super::appstream::imagebuilder::AccessEndpoint_>>,
    pub appstream_agent_version: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub display_name: Option<crate::value::ExpString>,
    pub domain_join_info: Option<super::appstream::imagebuilder::DomainJoinInfo_>,
    pub enable_default_internet_access: Option<crate::value::ExpBool>,
    pub iam_role_arn: Option<crate::value::ExpString>,
    pub image_arn: Option<crate::value::ExpString>,
    pub image_name: Option<crate::value::ExpString>,
    pub instance_type: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_config: Option<super::appstream::imagebuilder::VpcConfig_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appstream_ImageBuilder {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::AppStream::ImageBuilder"
        $($field $value)*)
    };
}
pub use crate::__aws_appstream_ImageBuilder as ImageBuilder;
impl crate::template::ToResource for ImageBuilder_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppStream"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ImageBuilder"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.access_endpoints {
            properties.insert(
                "AccessEndpoints".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.appstream_agent_version {
            properties.insert(
                "AppstreamAgentVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.display_name {
            properties.insert(
                "DisplayName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.domain_join_info {
            properties.insert(
                "DomainJoinInfo".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enable_default_internet_access {
            properties.insert(
                "EnableDefaultInternetAccess".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.iam_role_arn {
            properties.insert(
                "IamRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.image_arn {
            properties.insert(
                "ImageArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.image_name {
            properties.insert(
                "ImageName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "InstanceType".to_string(),
            crate::value::ToValue::to_value(&self.instance_type),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.vpc_config {
            properties.insert(
                "VpcConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-stack.html
pub struct Stack_ {
    pub access_endpoints: Option<Vec<super::appstream::stack::AccessEndpoint_>>,
    pub application_settings: Option<super::appstream::stack::ApplicationSettings_>,
    pub attributes_to_delete: Option<Vec<crate::value::ExpString>>,
    pub delete_storage_connectors: Option<crate::value::ExpBool>,
    pub description: Option<crate::value::ExpString>,
    pub display_name: Option<crate::value::ExpString>,
    pub embed_host_domains: Option<Vec<crate::value::ExpString>>,
    pub feedback_url: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub redirect_url: Option<crate::value::ExpString>,
    pub storage_connectors: Option<Vec<super::appstream::stack::StorageConnector_>>,
    pub streaming_experience_settings:
        Option<super::appstream::stack::StreamingExperienceSettings_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub user_settings: Option<Vec<super::appstream::stack::UserSetting_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appstream_Stack {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::AppStream::Stack" $($field
        $value)*)
    };
}
pub use crate::__aws_appstream_Stack as Stack;
impl crate::template::ToResource for Stack_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppStream"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Stack"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.access_endpoints {
            properties.insert(
                "AccessEndpoints".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.application_settings {
            properties.insert(
                "ApplicationSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.attributes_to_delete {
            properties.insert(
                "AttributesToDelete".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.delete_storage_connectors {
            properties.insert(
                "DeleteStorageConnectors".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.display_name {
            properties.insert(
                "DisplayName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.embed_host_domains {
            properties.insert(
                "EmbedHostDomains".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.feedback_url {
            properties.insert(
                "FeedbackURL".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.redirect_url {
            properties.insert(
                "RedirectURL".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.storage_connectors {
            properties.insert(
                "StorageConnectors".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.streaming_experience_settings {
            properties.insert(
                "StreamingExperienceSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.user_settings {
            properties.insert(
                "UserSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-stackfleetassociation.html
pub struct StackFleetAssociation_ {
    pub fleet_name: crate::value::ExpString,
    pub stack_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appstream_StackFleetAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::AppStream::StackFleetAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_appstream_StackFleetAssociation as StackFleetAssociation;
impl crate::template::ToResource for StackFleetAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppStream"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("StackFleetAssociation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "FleetName".to_string(),
            crate::value::ToValue::to_value(&self.fleet_name),
        );
        properties.insert(
            "StackName".to_string(),
            crate::value::ToValue::to_value(&self.stack_name),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-stackuserassociation.html
pub struct StackUserAssociation_ {
    pub authentication_type: crate::value::ExpString,
    pub send_email_notification: Option<crate::value::ExpBool>,
    pub stack_name: crate::value::ExpString,
    pub user_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appstream_StackUserAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::AppStream::StackUserAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_appstream_StackUserAssociation as StackUserAssociation;
impl crate::template::ToResource for StackUserAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppStream"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("StackUserAssociation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AuthenticationType".to_string(),
            crate::value::ToValue::to_value(&self.authentication_type),
        );
        if let Some(ref value) = self.send_email_notification {
            properties.insert(
                "SendEmailNotification".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "StackName".to_string(),
            crate::value::ToValue::to_value(&self.stack_name),
        );
        properties.insert(
            "UserName".to_string(),
            crate::value::ToValue::to_value(&self.user_name),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appstream-user.html
pub struct User_ {
    pub authentication_type: crate::value::ExpString,
    pub first_name: Option<crate::value::ExpString>,
    pub last_name: Option<crate::value::ExpString>,
    pub message_action: Option<crate::value::ExpString>,
    pub user_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appstream_User {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::AppStream::User" $($field
        $value)*)
    };
}
pub use crate::__aws_appstream_User as User;
impl crate::template::ToResource for User_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppStream"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("User"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AuthenticationType".to_string(),
            crate::value::ToValue::to_value(&self.authentication_type),
        );
        if let Some(ref value) = self.first_name {
            properties.insert(
                "FirstName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.last_name {
            properties.insert(
                "LastName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.message_action {
            properties.insert(
                "MessageAction".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "UserName".to_string(),
            crate::value::ToValue::to_value(&self.user_name),
        );
        properties
    }
}
