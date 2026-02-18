pub mod agreement {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-agreement-customdirectories.html>
    pub struct CustomDirectories_ {
        pub failed_files_directory: crate::value::ExpString,
        pub mdn_files_directory: crate::value::ExpString,
        pub payload_files_directory: crate::value::ExpString,
        pub status_files_directory: crate::value::ExpString,
        pub temporary_files_directory: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_transfer_Agreement_CustomDirectories {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Transfer::Agreement.CustomDirectories"
            $($field $value)*)
        };
    }
    pub use crate::__aws_transfer_Agreement_CustomDirectories as CustomDirectories;
    impl crate::value::ToValue for CustomDirectories_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FailedFilesDirectory".to_string(),
                crate::value::ToValue::to_value(&self.failed_files_directory),
            );
            properties.insert(
                "MdnFilesDirectory".to_string(),
                crate::value::ToValue::to_value(&self.mdn_files_directory),
            );
            properties.insert(
                "PayloadFilesDirectory".to_string(),
                crate::value::ToValue::to_value(&self.payload_files_directory),
            );
            properties.insert(
                "StatusFilesDirectory".to_string(),
                crate::value::ToValue::to_value(&self.status_files_directory),
            );
            properties.insert(
                "TemporaryFilesDirectory".to_string(),
                crate::value::ToValue::to_value(&self.temporary_files_directory),
            );
            properties.into()
        }
    }
}
pub mod connector {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-connector-as2config.html>
    pub struct As2Config_ {
        pub async_mdn_config: Option<Box<ConnectorAsyncMdnConfig_>>,
        pub basic_auth_secret_id: Option<crate::value::ExpString>,
        pub compression: Option<crate::value::ExpString>,
        pub encryption_algorithm: Option<crate::value::ExpString>,
        pub local_profile_id: Option<crate::value::ExpString>,
        pub mdn_response: Option<crate::value::ExpString>,
        pub mdn_signing_algorithm: Option<crate::value::ExpString>,
        pub message_subject: Option<crate::value::ExpString>,
        pub partner_profile_id: Option<crate::value::ExpString>,
        pub preserve_content_type: Option<crate::value::ExpString>,
        pub signing_algorithm: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_transfer_Connector_As2Config {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Transfer::Connector.As2Config"
            $($field $value)*)
        };
    }
    pub use crate::__aws_transfer_Connector_As2Config as As2Config;
    impl crate::value::ToValue for As2Config_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.async_mdn_config {
                properties.insert(
                    "AsyncMdnConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.basic_auth_secret_id {
                properties.insert(
                    "BasicAuthSecretId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.compression {
                properties.insert(
                    "Compression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.encryption_algorithm {
                properties.insert(
                    "EncryptionAlgorithm".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.local_profile_id {
                properties.insert(
                    "LocalProfileId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mdn_response {
                properties.insert(
                    "MdnResponse".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mdn_signing_algorithm {
                properties.insert(
                    "MdnSigningAlgorithm".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.message_subject {
                properties.insert(
                    "MessageSubject".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.partner_profile_id {
                properties.insert(
                    "PartnerProfileId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.preserve_content_type {
                properties.insert(
                    "PreserveContentType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.signing_algorithm {
                properties.insert(
                    "SigningAlgorithm".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-connector-connectorasyncmdnconfig.html>
    pub struct ConnectorAsyncMdnConfig_ {
        pub server_ids: Vec<crate::value::ExpString>,
        pub url: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_transfer_Connector_ConnectorAsyncMdnConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Transfer::Connector.ConnectorAsyncMdnConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_transfer_Connector_ConnectorAsyncMdnConfig as ConnectorAsyncMdnConfig;
    impl crate::value::ToValue for ConnectorAsyncMdnConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ServerIds".to_string(),
                crate::value::ToValue::to_value(&self.server_ids),
            );
            properties.insert(
                "Url".to_string(),
                crate::value::ToValue::to_value(&self.url),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-connector-connectoregressconfig.html>
    pub struct ConnectorEgressConfig_ {
        pub vpc_lattice: Box<ConnectorVpcLatticeEgressConfig_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_transfer_Connector_ConnectorEgressConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Transfer::Connector.ConnectorEgressConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_transfer_Connector_ConnectorEgressConfig as ConnectorEgressConfig;
    impl crate::value::ToValue for ConnectorEgressConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "VpcLattice".to_string(),
                crate::value::ToValue::to_value(&self.vpc_lattice),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-connector-connectorvpclatticeegressconfig.html>
    pub struct ConnectorVpcLatticeEgressConfig_ {
        pub port_number: Option<i32>,
        pub resource_configuration_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_transfer_Connector_ConnectorVpcLatticeEgressConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Transfer::Connector.ConnectorVpcLatticeEgressConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_transfer_Connector_ConnectorVpcLatticeEgressConfig as ConnectorVpcLatticeEgressConfig;
    impl crate::value::ToValue for ConnectorVpcLatticeEgressConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.port_number {
                properties.insert(
                    "PortNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ResourceConfigurationArn".to_string(),
                crate::value::ToValue::to_value(&self.resource_configuration_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-connector-sftpconfig.html>
    pub struct SftpConfig_ {
        pub max_concurrent_connections: Option<i32>,
        pub trusted_host_keys: Option<Vec<crate::value::ExpString>>,
        pub user_secret_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_transfer_Connector_SftpConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Transfer::Connector.SftpConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_transfer_Connector_SftpConfig as SftpConfig;
    impl crate::value::ToValue for SftpConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max_concurrent_connections {
                properties.insert(
                    "MaxConcurrentConnections".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.trusted_host_keys {
                properties.insert(
                    "TrustedHostKeys".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_secret_id {
                properties.insert(
                    "UserSecretId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod server {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-endpointdetails.html>
    pub struct EndpointDetails_ {
        pub address_allocation_ids: Option<Vec<crate::value::ExpString>>,
        pub security_group_ids: Option<Vec<crate::value::ExpString>>,
        pub subnet_ids: Option<Vec<crate::value::ExpString>>,
        pub vpc_endpoint_id: Option<crate::value::ExpString>,
        pub vpc_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_transfer_Server_EndpointDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Transfer::Server.EndpointDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_transfer_Server_EndpointDetails as EndpointDetails;
    impl crate::value::ToValue for EndpointDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.address_allocation_ids {
                properties.insert(
                    "AddressAllocationIds".to_string(),
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
            if let Some(ref value) = self.vpc_endpoint_id {
                properties.insert(
                    "VpcEndpointId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpc_id {
                properties.insert("VpcId".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-identityproviderdetails.html>
    pub struct IdentityProviderDetails_ {
        pub directory_id: Option<crate::value::ExpString>,
        pub function: Option<crate::value::ExpString>,
        pub invocation_role: Option<crate::value::ExpString>,
        pub sftp_authentication_methods: Option<crate::value::ExpString>,
        pub url: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_transfer_Server_IdentityProviderDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Transfer::Server.IdentityProviderDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_transfer_Server_IdentityProviderDetails as IdentityProviderDetails;
    impl crate::value::ToValue for IdentityProviderDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.directory_id {
                properties.insert(
                    "DirectoryId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.function {
                properties.insert(
                    "Function".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.invocation_role {
                properties.insert(
                    "InvocationRole".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sftp_authentication_methods {
                properties.insert(
                    "SftpAuthenticationMethods".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.url {
                properties.insert("Url".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-protocoldetails.html>
    pub struct ProtocolDetails_ {
        pub as2_transports: Option<Vec<crate::value::ExpString>>,
        pub passive_ip: Option<crate::value::ExpString>,
        pub set_stat_option: Option<crate::value::ExpString>,
        pub tls_session_resumption_mode: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_transfer_Server_ProtocolDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Transfer::Server.ProtocolDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_transfer_Server_ProtocolDetails as ProtocolDetails;
    impl crate::value::ToValue for ProtocolDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.as2_transports {
                properties.insert(
                    "As2Transports".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.passive_ip {
                properties.insert(
                    "PassiveIp".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.set_stat_option {
                properties.insert(
                    "SetStatOption".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tls_session_resumption_mode {
                properties.insert(
                    "TlsSessionResumptionMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-s3storageoptions.html>
    pub struct S3StorageOptions_ {
        pub directory_listing_optimization: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_transfer_Server_S3StorageOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Transfer::Server.S3StorageOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_transfer_Server_S3StorageOptions as S3StorageOptions;
    impl crate::value::ToValue for S3StorageOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.directory_listing_optimization {
                properties.insert(
                    "DirectoryListingOptimization".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-workflowdetail.html>
    pub struct WorkflowDetail_ {
        pub execution_role: crate::value::ExpString,
        pub workflow_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_transfer_Server_WorkflowDetail {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Transfer::Server.WorkflowDetail"
            $($field $value)*)
        };
    }
    pub use crate::__aws_transfer_Server_WorkflowDetail as WorkflowDetail;
    impl crate::value::ToValue for WorkflowDetail_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ExecutionRole".to_string(),
                crate::value::ToValue::to_value(&self.execution_role),
            );
            properties.insert(
                "WorkflowId".to_string(),
                crate::value::ToValue::to_value(&self.workflow_id),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-workflowdetails.html>
    pub struct WorkflowDetails_ {
        pub on_partial_upload: Option<Vec<WorkflowDetail_>>,
        pub on_upload: Option<Vec<WorkflowDetail_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_transfer_Server_WorkflowDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Transfer::Server.WorkflowDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_transfer_Server_WorkflowDetails as WorkflowDetails;
    impl crate::value::ToValue for WorkflowDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.on_partial_upload {
                properties.insert(
                    "OnPartialUpload".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.on_upload {
                properties.insert(
                    "OnUpload".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod user {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-user-homedirectorymapentry.html>
    pub struct HomeDirectoryMapEntry_ {
        pub entry: crate::value::ExpString,
        pub target: crate::value::ExpString,
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_transfer_User_HomeDirectoryMapEntry {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Transfer::User.HomeDirectoryMapEntry"
            $($field $value)*)
        };
    }
    pub use crate::__aws_transfer_User_HomeDirectoryMapEntry as HomeDirectoryMapEntry;
    impl crate::value::ToValue for HomeDirectoryMapEntry_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Entry".to_string(),
                crate::value::ToValue::to_value(&self.entry),
            );
            properties.insert(
                "Target".to_string(),
                crate::value::ToValue::to_value(&self.target),
            );
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-user-posixprofile.html>
    pub struct PosixProfile_ {
        pub gid: f64,
        pub secondary_gids: Option<Vec<f64>>,
        pub uid: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_transfer_User_PosixProfile {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Transfer::User.PosixProfile"
            $($field $value)*)
        };
    }
    pub use crate::__aws_transfer_User_PosixProfile as PosixProfile;
    impl crate::value::ToValue for PosixProfile_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Gid".to_string(),
                crate::value::ToValue::to_value(&self.gid),
            );
            if let Some(ref value) = self.secondary_gids {
                properties.insert(
                    "SecondaryGids".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Uid".to_string(),
                crate::value::ToValue::to_value(&self.uid),
            );
            properties.into()
        }
    }
}
pub mod webapp {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-webapp-endpointdetails.html>
    pub struct EndpointDetails_ {
        pub vpc: Option<Box<Vpc_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_transfer_WebApp_EndpointDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Transfer::WebApp.EndpointDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_transfer_WebApp_EndpointDetails as EndpointDetails;
    impl crate::value::ToValue for EndpointDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.vpc {
                properties.insert("Vpc".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-webapp-identityproviderdetails.html>
    pub struct IdentityProviderDetails_ {
        pub application_arn: Option<crate::value::ExpString>,
        pub instance_arn: Option<crate::value::ExpString>,
        pub role: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_transfer_WebApp_IdentityProviderDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Transfer::WebApp.IdentityProviderDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_transfer_WebApp_IdentityProviderDetails as IdentityProviderDetails;
    impl crate::value::ToValue for IdentityProviderDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.application_arn {
                properties.insert(
                    "ApplicationArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.instance_arn {
                properties.insert(
                    "InstanceArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.role {
                properties.insert("Role".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-webapp-vpc.html>
    pub struct Vpc_ {
        pub security_group_ids: Option<Vec<crate::value::ExpString>>,
        pub subnet_ids: Option<Vec<crate::value::ExpString>>,
        pub vpc_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_transfer_WebApp_Vpc {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Transfer::WebApp.Vpc"
            $($field $value)*)
        };
    }
    pub use crate::__aws_transfer_WebApp_Vpc as Vpc;
    impl crate::value::ToValue for Vpc_ {
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
            if let Some(ref value) = self.vpc_id {
                properties.insert("VpcId".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-webapp-webappcustomization.html>
    pub struct WebAppCustomization_ {
        pub favicon_file: Option<crate::value::ExpString>,
        pub logo_file: Option<crate::value::ExpString>,
        pub title: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_transfer_WebApp_WebAppCustomization {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Transfer::WebApp.WebAppCustomization"
            $($field $value)*)
        };
    }
    pub use crate::__aws_transfer_WebApp_WebAppCustomization as WebAppCustomization;
    impl crate::value::ToValue for WebAppCustomization_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.favicon_file {
                properties.insert(
                    "FaviconFile".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.logo_file {
                properties.insert(
                    "LogoFile".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.title {
                properties.insert("Title".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-webapp-webappunits.html>
    pub struct WebAppUnits_ {
        pub provisioned: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_transfer_WebApp_WebAppUnits {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Transfer::WebApp.WebAppUnits"
            $($field $value)*)
        };
    }
    pub use crate::__aws_transfer_WebApp_WebAppUnits as WebAppUnits;
    impl crate::value::ToValue for WebAppUnits_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Provisioned".to_string(),
                crate::value::ToValue::to_value(&self.provisioned),
            );
            properties.into()
        }
    }
}
pub mod workflow {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-copystepdetails.html>
    pub struct CopyStepDetails_ {
        pub destination_file_location: Option<Box<S3FileLocation_>>,
        pub name: Option<crate::value::ExpString>,
        pub overwrite_existing: Option<crate::value::ExpString>,
        pub source_file_location: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_transfer_Workflow_CopyStepDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Transfer::Workflow.CopyStepDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_transfer_Workflow_CopyStepDetails as CopyStepDetails;
    impl crate::value::ToValue for CopyStepDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.destination_file_location {
                properties.insert(
                    "DestinationFileLocation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.overwrite_existing {
                properties.insert(
                    "OverwriteExisting".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_file_location {
                properties.insert(
                    "SourceFileLocation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-customstepdetails.html>
    pub struct CustomStepDetails_ {
        pub name: Option<crate::value::ExpString>,
        pub source_file_location: Option<crate::value::ExpString>,
        pub target: Option<crate::value::ExpString>,
        pub timeout_seconds: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_transfer_Workflow_CustomStepDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Transfer::Workflow.CustomStepDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_transfer_Workflow_CustomStepDetails as CustomStepDetails;
    impl crate::value::ToValue for CustomStepDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.source_file_location {
                properties.insert(
                    "SourceFileLocation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target {
                properties.insert("Target".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.timeout_seconds {
                properties.insert(
                    "TimeoutSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-decryptstepdetails.html>
    pub struct DecryptStepDetails_ {
        pub destination_file_location: Box<InputFileLocation_>,
        pub name: Option<crate::value::ExpString>,
        pub overwrite_existing: Option<crate::value::ExpString>,
        pub source_file_location: Option<crate::value::ExpString>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_transfer_Workflow_DecryptStepDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Transfer::Workflow.DecryptStepDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_transfer_Workflow_DecryptStepDetails as DecryptStepDetails;
    impl crate::value::ToValue for DecryptStepDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DestinationFileLocation".to_string(),
                crate::value::ToValue::to_value(&self.destination_file_location),
            );
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.overwrite_existing {
                properties.insert(
                    "OverwriteExisting".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_file_location {
                properties.insert(
                    "SourceFileLocation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-deletestepdetails.html>
    pub struct DeleteStepDetails_ {
        pub name: Option<crate::value::ExpString>,
        pub source_file_location: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_transfer_Workflow_DeleteStepDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Transfer::Workflow.DeleteStepDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_transfer_Workflow_DeleteStepDetails as DeleteStepDetails;
    impl crate::value::ToValue for DeleteStepDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.source_file_location {
                properties.insert(
                    "SourceFileLocation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-efsinputfilelocation.html>
    pub struct EfsInputFileLocation_ {
        pub file_system_id: Option<crate::value::ExpString>,
        pub path: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_transfer_Workflow_EfsInputFileLocation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Transfer::Workflow.EfsInputFileLocation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_transfer_Workflow_EfsInputFileLocation as EfsInputFileLocation;
    impl crate::value::ToValue for EfsInputFileLocation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.file_system_id {
                properties.insert(
                    "FileSystemId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.path {
                properties.insert("Path".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-inputfilelocation.html>
    pub struct InputFileLocation_ {
        pub efs_file_location: Option<Box<EfsInputFileLocation_>>,
        pub s3_file_location: Option<Box<S3InputFileLocation_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_transfer_Workflow_InputFileLocation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Transfer::Workflow.InputFileLocation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_transfer_Workflow_InputFileLocation as InputFileLocation;
    impl crate::value::ToValue for InputFileLocation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.efs_file_location {
                properties.insert(
                    "EfsFileLocation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_file_location {
                properties.insert(
                    "S3FileLocation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-s3filelocation.html>
    pub struct S3FileLocation_ {
        pub s3_file_location: Option<Box<S3InputFileLocation_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_transfer_Workflow_S3FileLocation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Transfer::Workflow.S3FileLocation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_transfer_Workflow_S3FileLocation as S3FileLocation;
    impl crate::value::ToValue for S3FileLocation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.s3_file_location {
                properties.insert(
                    "S3FileLocation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-s3inputfilelocation.html>
    pub struct S3InputFileLocation_ {
        pub bucket: Option<crate::value::ExpString>,
        pub key: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_transfer_Workflow_S3InputFileLocation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Transfer::Workflow.S3InputFileLocation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_transfer_Workflow_S3InputFileLocation as S3InputFileLocation;
    impl crate::value::ToValue for S3InputFileLocation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bucket {
                properties.insert("Bucket".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.key {
                properties.insert("Key".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-s3tag.html>
    pub struct S3Tag_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_transfer_Workflow_S3Tag {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Transfer::Workflow.S3Tag"
            $($field $value)*)
        };
    }
    pub use crate::__aws_transfer_Workflow_S3Tag as S3Tag;
    impl crate::value::ToValue for S3Tag_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-tagstepdetails.html>
    pub struct TagStepDetails_ {
        pub name: Option<crate::value::ExpString>,
        pub source_file_location: Option<crate::value::ExpString>,
        pub tags: Option<Vec<S3Tag_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_transfer_Workflow_TagStepDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Transfer::Workflow.TagStepDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_transfer_Workflow_TagStepDetails as TagStepDetails;
    impl crate::value::ToValue for TagStepDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.source_file_location {
                properties.insert(
                    "SourceFileLocation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tags {
                properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-workflowstep.html>
    pub struct WorkflowStep_ {
        pub copy_step_details: Option<Box<CopyStepDetails_>>,
        pub custom_step_details: Option<Box<CustomStepDetails_>>,
        pub decrypt_step_details: Option<Box<DecryptStepDetails_>>,
        pub delete_step_details: Option<Box<DeleteStepDetails_>>,
        pub tag_step_details: Option<Box<TagStepDetails_>>,
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_transfer_Workflow_WorkflowStep {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Transfer::Workflow.WorkflowStep"
            $($field $value)*)
        };
    }
    pub use crate::__aws_transfer_Workflow_WorkflowStep as WorkflowStep;
    impl crate::value::ToValue for WorkflowStep_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.copy_step_details {
                properties.insert(
                    "CopyStepDetails".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_step_details {
                properties.insert(
                    "CustomStepDetails".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.decrypt_step_details {
                properties.insert(
                    "DecryptStepDetails".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.delete_step_details {
                properties.insert(
                    "DeleteStepDetails".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tag_step_details {
                properties.insert(
                    "TagStepDetails".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-agreement.html>
pub struct Agreement_ {
    pub access_role: crate::value::ExpString,
    pub base_directory: Option<crate::value::ExpString>,
    pub custom_directories: Option<super::transfer::agreement::CustomDirectories_>,
    pub description: Option<crate::value::ExpString>,
    pub enforce_message_signing: Option<crate::value::ExpString>,
    pub local_profile_id: crate::value::ExpString,
    pub partner_profile_id: crate::value::ExpString,
    pub preserve_filename: Option<crate::value::ExpString>,
    pub server_id: crate::value::ExpString,
    pub status: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_transfer_Agreement {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Transfer::Agreement"
        $($field $value)*)
    };
}
pub use crate::__aws_transfer_Agreement as Agreement;
impl crate::template::ToResource for Agreement_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Transfer"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Agreement"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AccessRole".to_string(),
            crate::value::ToValue::to_value(&self.access_role),
        );
        if let Some(ref value) = self.base_directory {
            properties.insert(
                "BaseDirectory".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.custom_directories {
            properties.insert(
                "CustomDirectories".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enforce_message_signing {
            properties.insert(
                "EnforceMessageSigning".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "LocalProfileId".to_string(),
            crate::value::ToValue::to_value(&self.local_profile_id),
        );
        properties.insert(
            "PartnerProfileId".to_string(),
            crate::value::ToValue::to_value(&self.partner_profile_id),
        );
        if let Some(ref value) = self.preserve_filename {
            properties.insert(
                "PreserveFilename".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ServerId".to_string(),
            crate::value::ToValue::to_value(&self.server_id),
        );
        if let Some(ref value) = self.status {
            properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-certificate.html>
pub struct Certificate_ {
    pub active_date: Option<crate::value::ExpString>,
    pub certificate: crate::value::ExpString,
    pub certificate_chain: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub inactive_date: Option<crate::value::ExpString>,
    pub private_key: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub usage: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_transfer_Certificate {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Transfer::Certificate"
        $($field $value)*)
    };
}
pub use crate::__aws_transfer_Certificate as Certificate;
impl crate::template::ToResource for Certificate_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Transfer"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Certificate"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.active_date {
            properties.insert(
                "ActiveDate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Certificate".to_string(),
            crate::value::ToValue::to_value(&self.certificate),
        );
        if let Some(ref value) = self.certificate_chain {
            properties.insert(
                "CertificateChain".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.inactive_date {
            properties.insert(
                "InactiveDate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.private_key {
            properties.insert(
                "PrivateKey".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Usage".to_string(),
            crate::value::ToValue::to_value(&self.usage),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-connector.html>
pub struct Connector_ {
    pub access_role: crate::value::ExpString,
    pub as2_config: Option<super::transfer::connector::As2Config_>,
    pub egress_config: Option<super::transfer::connector::ConnectorEgressConfig_>,
    pub egress_type: Option<crate::value::ExpString>,
    pub logging_role: Option<crate::value::ExpString>,
    pub security_policy_name: Option<crate::value::ExpString>,
    pub sftp_config: Option<super::transfer::connector::SftpConfig_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub url: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_transfer_Connector {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Transfer::Connector"
        $($field $value)*)
    };
}
pub use crate::__aws_transfer_Connector as Connector;
impl crate::template::ToResource for Connector_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Transfer"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Connector"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AccessRole".to_string(),
            crate::value::ToValue::to_value(&self.access_role),
        );
        if let Some(ref value) = self.as2_config {
            properties.insert(
                "As2Config".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.egress_config {
            properties.insert(
                "EgressConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.egress_type {
            properties.insert(
                "EgressType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.logging_role {
            properties.insert(
                "LoggingRole".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.security_policy_name {
            properties.insert(
                "SecurityPolicyName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sftp_config {
            properties.insert(
                "SftpConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.url {
            properties.insert("Url".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-profile.html>
pub struct Profile_ {
    pub as2_id: crate::value::ExpString,
    pub certificate_ids: Option<Vec<crate::value::ExpString>>,
    pub profile_type: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_transfer_Profile {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Transfer::Profile"
        $($field $value)*)
    };
}
pub use crate::__aws_transfer_Profile as Profile;
impl crate::template::ToResource for Profile_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Transfer"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Profile"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "As2Id".to_string(),
            crate::value::ToValue::to_value(&self.as2_id),
        );
        if let Some(ref value) = self.certificate_ids {
            properties.insert(
                "CertificateIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ProfileType".to_string(),
            crate::value::ToValue::to_value(&self.profile_type),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-server.html>
pub struct Server_ {
    pub certificate: Option<crate::value::ExpString>,
    pub domain: Option<crate::value::ExpString>,
    pub endpoint_details: Option<super::transfer::server::EndpointDetails_>,
    pub endpoint_type: Option<crate::value::ExpString>,
    pub identity_provider_details: Option<super::transfer::server::IdentityProviderDetails_>,
    pub identity_provider_type: Option<crate::value::ExpString>,
    pub ip_address_type: Option<crate::value::ExpString>,
    pub logging_role: Option<crate::value::ExpString>,
    pub post_authentication_login_banner: Option<crate::value::ExpString>,
    pub pre_authentication_login_banner: Option<crate::value::ExpString>,
    pub protocol_details: Option<super::transfer::server::ProtocolDetails_>,
    pub protocols: Option<Vec<crate::value::ExpString>>,
    pub s3_storage_options: Option<super::transfer::server::S3StorageOptions_>,
    pub security_policy_name: Option<crate::value::ExpString>,
    pub structured_log_destinations: Option<Vec<crate::value::ExpString>>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub workflow_details: Option<super::transfer::server::WorkflowDetails_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_transfer_Server {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Transfer::Server"
        $($field $value)*)
    };
}
pub use crate::__aws_transfer_Server as Server;
impl crate::template::ToResource for Server_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Transfer"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Server"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.certificate {
            properties.insert(
                "Certificate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.domain {
            properties.insert("Domain".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.endpoint_details {
            properties.insert(
                "EndpointDetails".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.endpoint_type {
            properties.insert(
                "EndpointType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.identity_provider_details {
            properties.insert(
                "IdentityProviderDetails".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.identity_provider_type {
            properties.insert(
                "IdentityProviderType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ip_address_type {
            properties.insert(
                "IpAddressType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.logging_role {
            properties.insert(
                "LoggingRole".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.post_authentication_login_banner {
            properties.insert(
                "PostAuthenticationLoginBanner".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.pre_authentication_login_banner {
            properties.insert(
                "PreAuthenticationLoginBanner".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.protocol_details {
            properties.insert(
                "ProtocolDetails".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.protocols {
            properties.insert(
                "Protocols".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.s3_storage_options {
            properties.insert(
                "S3StorageOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.security_policy_name {
            properties.insert(
                "SecurityPolicyName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.structured_log_destinations {
            properties.insert(
                "StructuredLogDestinations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.workflow_details {
            properties.insert(
                "WorkflowDetails".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-user.html>
pub struct User_ {
    pub home_directory: Option<crate::value::ExpString>,
    pub home_directory_mappings: Option<Vec<super::transfer::user::HomeDirectoryMapEntry_>>,
    pub home_directory_type: Option<crate::value::ExpString>,
    pub policy: Option<crate::value::ExpString>,
    pub posix_profile: Option<super::transfer::user::PosixProfile_>,
    pub role: crate::value::ExpString,
    pub server_id: crate::value::ExpString,
    pub ssh_public_keys: Option<Vec<crate::value::ExpString>>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub user_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_transfer_User {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Transfer::User" $($field
        $value)*)
    };
}
pub use crate::__aws_transfer_User as User;
impl crate::template::ToResource for User_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Transfer"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("User"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.home_directory {
            properties.insert(
                "HomeDirectory".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.home_directory_mappings {
            properties.insert(
                "HomeDirectoryMappings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.home_directory_type {
            properties.insert(
                "HomeDirectoryType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.policy {
            properties.insert("Policy".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.posix_profile {
            properties.insert(
                "PosixProfile".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Role".to_string(),
            crate::value::ToValue::to_value(&self.role),
        );
        properties.insert(
            "ServerId".to_string(),
            crate::value::ToValue::to_value(&self.server_id),
        );
        if let Some(ref value) = self.ssh_public_keys {
            properties.insert(
                "SshPublicKeys".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "UserName".to_string(),
            crate::value::ToValue::to_value(&self.user_name),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-webapp.html>
pub struct WebApp_ {
    pub access_endpoint: Option<crate::value::ExpString>,
    pub endpoint_details: Option<super::transfer::webapp::EndpointDetails_>,
    pub identity_provider_details: super::transfer::webapp::IdentityProviderDetails_,
    pub tags: Option<Vec<crate::Tag_>>,
    pub web_app_customization: Option<super::transfer::webapp::WebAppCustomization_>,
    pub web_app_endpoint_policy: Option<crate::value::ExpString>,
    pub web_app_units: Option<super::transfer::webapp::WebAppUnits_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_transfer_WebApp {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Transfer::WebApp"
        $($field $value)*)
    };
}
pub use crate::__aws_transfer_WebApp as WebApp;
impl crate::template::ToResource for WebApp_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Transfer"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("WebApp"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.access_endpoint {
            properties.insert(
                "AccessEndpoint".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.endpoint_details {
            properties.insert(
                "EndpointDetails".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "IdentityProviderDetails".to_string(),
            crate::value::ToValue::to_value(&self.identity_provider_details),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.web_app_customization {
            properties.insert(
                "WebAppCustomization".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.web_app_endpoint_policy {
            properties.insert(
                "WebAppEndpointPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.web_app_units {
            properties.insert(
                "WebAppUnits".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-workflow.html>
pub struct Workflow_ {
    pub description: Option<crate::value::ExpString>,
    pub on_exception_steps: Option<Vec<super::transfer::workflow::WorkflowStep_>>,
    pub steps: Vec<super::transfer::workflow::WorkflowStep_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_transfer_Workflow {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Transfer::Workflow"
        $($field $value)*)
    };
}
pub use crate::__aws_transfer_Workflow as Workflow;
impl crate::template::ToResource for Workflow_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Transfer"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Workflow"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.on_exception_steps {
            properties.insert(
                "OnExceptionSteps".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Steps".to_string(),
            crate::value::ToValue::to_value(&self.steps),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
