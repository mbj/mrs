pub mod broker {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-configurationid.html
    pub struct ConfigurationId_ {
        pub id: crate::value::ExpString,
        pub revision: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amazonmq_Broker_ConfigurationId {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AmazonMQ::Broker.ConfigurationId"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amazonmq_Broker_ConfigurationId as ConfigurationId;
    impl crate::value::ToValue for ConfigurationId_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            properties.insert(
                "Revision".to_string(),
                crate::value::ToValue::to_value(&self.revision),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-encryptionoptions.html
    pub struct EncryptionOptions_ {
        pub kms_key_id: Option<crate::value::ExpString>,
        pub use_aws_owned_key: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amazonmq_Broker_EncryptionOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AmazonMQ::Broker.EncryptionOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amazonmq_Broker_EncryptionOptions as EncryptionOptions;
    impl crate::value::ToValue for EncryptionOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "UseAwsOwnedKey".to_string(),
                crate::value::ToValue::to_value(&self.use_aws_owned_key),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-ldapservermetadata.html
    pub struct LdapServerMetadata_ {
        pub hosts: Vec<crate::value::ExpString>,
        pub role_base: crate::value::ExpString,
        pub role_name: Option<crate::value::ExpString>,
        pub role_search_matching: crate::value::ExpString,
        pub role_search_subtree: Option<crate::value::ExpBool>,
        pub service_account_password: crate::value::ExpString,
        pub service_account_username: crate::value::ExpString,
        pub user_base: crate::value::ExpString,
        pub user_role_name: Option<crate::value::ExpString>,
        pub user_search_matching: crate::value::ExpString,
        pub user_search_subtree: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amazonmq_Broker_LdapServerMetadata {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AmazonMQ::Broker.LdapServerMetadata"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amazonmq_Broker_LdapServerMetadata as LdapServerMetadata;
    impl crate::value::ToValue for LdapServerMetadata_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Hosts".to_string(),
                crate::value::ToValue::to_value(&self.hosts),
            );
            properties.insert(
                "RoleBase".to_string(),
                crate::value::ToValue::to_value(&self.role_base),
            );
            if let Some(ref value) = self.role_name {
                properties.insert(
                    "RoleName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RoleSearchMatching".to_string(),
                crate::value::ToValue::to_value(&self.role_search_matching),
            );
            if let Some(ref value) = self.role_search_subtree {
                properties.insert(
                    "RoleSearchSubtree".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ServiceAccountPassword".to_string(),
                crate::value::ToValue::to_value(&self.service_account_password),
            );
            properties.insert(
                "ServiceAccountUsername".to_string(),
                crate::value::ToValue::to_value(&self.service_account_username),
            );
            properties.insert(
                "UserBase".to_string(),
                crate::value::ToValue::to_value(&self.user_base),
            );
            if let Some(ref value) = self.user_role_name {
                properties.insert(
                    "UserRoleName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "UserSearchMatching".to_string(),
                crate::value::ToValue::to_value(&self.user_search_matching),
            );
            if let Some(ref value) = self.user_search_subtree {
                properties.insert(
                    "UserSearchSubtree".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-loglist.html
    pub struct LogList_ {
        pub audit: Option<crate::value::ExpBool>,
        pub general: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amazonmq_Broker_LogList {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AmazonMQ::Broker.LogList"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amazonmq_Broker_LogList as LogList;
    impl crate::value::ToValue for LogList_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.audit {
                properties.insert("Audit".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.general {
                properties.insert(
                    "General".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-maintenancewindow.html
    pub struct MaintenanceWindow_ {
        pub day_of_week: crate::value::ExpString,
        pub time_of_day: crate::value::ExpString,
        pub time_zone: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amazonmq_Broker_MaintenanceWindow {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AmazonMQ::Broker.MaintenanceWindow"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amazonmq_Broker_MaintenanceWindow as MaintenanceWindow;
    impl crate::value::ToValue for MaintenanceWindow_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DayOfWeek".to_string(),
                crate::value::ToValue::to_value(&self.day_of_week),
            );
            properties.insert(
                "TimeOfDay".to_string(),
                crate::value::ToValue::to_value(&self.time_of_day),
            );
            properties.insert(
                "TimeZone".to_string(),
                crate::value::ToValue::to_value(&self.time_zone),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-tagsentry.html
    pub struct TagsEntry_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amazonmq_Broker_TagsEntry {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AmazonMQ::Broker.TagsEntry"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amazonmq_Broker_TagsEntry as TagsEntry;
    impl crate::value::ToValue for TagsEntry_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-broker-user.html
    pub struct User_ {
        pub console_access: Option<crate::value::ExpBool>,
        pub groups: Option<Vec<crate::value::ExpString>>,
        pub password: crate::value::ExpString,
        pub replication_user: Option<crate::value::ExpBool>,
        pub username: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amazonmq_Broker_User {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AmazonMQ::Broker.User"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amazonmq_Broker_User as User;
    impl crate::value::ToValue for User_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.console_access {
                properties.insert(
                    "ConsoleAccess".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.groups {
                properties.insert("Groups".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Password".to_string(),
                crate::value::ToValue::to_value(&self.password),
            );
            if let Some(ref value) = self.replication_user {
                properties.insert(
                    "ReplicationUser".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Username".to_string(),
                crate::value::ToValue::to_value(&self.username),
            );
            properties.into()
        }
    }
}
pub mod configuration {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-configuration-tagsentry.html
    pub struct TagsEntry_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amazonmq_Configuration_TagsEntry {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AmazonMQ::Configuration.TagsEntry"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amazonmq_Configuration_TagsEntry as TagsEntry;
    impl crate::value::ToValue for TagsEntry_ {
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
}
pub mod configurationassociation {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amazonmq-configurationassociation-configurationid.html
    pub struct ConfigurationId_ {
        pub id: crate::value::ExpString,
        pub revision: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amazonmq_ConfigurationAssociation_ConfigurationId {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AmazonMQ::ConfigurationAssociation.ConfigurationId"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amazonmq_ConfigurationAssociation_ConfigurationId as ConfigurationId;
    impl crate::value::ToValue for ConfigurationId_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            properties.insert(
                "Revision".to_string(),
                crate::value::ToValue::to_value(&self.revision),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amazonmq-broker.html
pub struct Broker_ {
    pub authentication_strategy: Option<crate::value::ExpString>,
    pub auto_minor_version_upgrade: Option<crate::value::ExpBool>,
    pub broker_name: crate::value::ExpString,
    pub configuration: Option<super::amazonmq::broker::ConfigurationId_>,
    pub data_replication_mode: Option<crate::value::ExpString>,
    pub data_replication_primary_broker_arn: Option<crate::value::ExpString>,
    pub deployment_mode: crate::value::ExpString,
    pub encryption_options: Option<super::amazonmq::broker::EncryptionOptions_>,
    pub engine_type: crate::value::ExpString,
    pub engine_version: Option<crate::value::ExpString>,
    pub host_instance_type: crate::value::ExpString,
    pub ldap_server_metadata: Option<super::amazonmq::broker::LdapServerMetadata_>,
    pub logs: Option<super::amazonmq::broker::LogList_>,
    pub maintenance_window_start_time: Option<super::amazonmq::broker::MaintenanceWindow_>,
    pub publicly_accessible: crate::value::ExpBool,
    pub security_groups: Option<Vec<crate::value::ExpString>>,
    pub storage_type: Option<crate::value::ExpString>,
    pub subnet_ids: Option<Vec<crate::value::ExpString>>,
    pub tags: Option<Vec<super::amazonmq::broker::TagsEntry_>>,
    pub users: Vec<super::amazonmq::broker::User_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_amazonmq_Broker {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::AmazonMQ::Broker"
        $($field $value)*)
    };
}
pub use crate::__aws_amazonmq_Broker as Broker;
impl crate::template::ToResource for Broker_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AmazonMQ"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Broker"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.authentication_strategy {
            properties.insert(
                "AuthenticationStrategy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.auto_minor_version_upgrade {
            properties.insert(
                "AutoMinorVersionUpgrade".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "BrokerName".to_string(),
            crate::value::ToValue::to_value(&self.broker_name),
        );
        if let Some(ref value) = self.configuration {
            properties.insert(
                "Configuration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.data_replication_mode {
            properties.insert(
                "DataReplicationMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.data_replication_primary_broker_arn {
            properties.insert(
                "DataReplicationPrimaryBrokerArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DeploymentMode".to_string(),
            crate::value::ToValue::to_value(&self.deployment_mode),
        );
        if let Some(ref value) = self.encryption_options {
            properties.insert(
                "EncryptionOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "EngineType".to_string(),
            crate::value::ToValue::to_value(&self.engine_type),
        );
        if let Some(ref value) = self.engine_version {
            properties.insert(
                "EngineVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "HostInstanceType".to_string(),
            crate::value::ToValue::to_value(&self.host_instance_type),
        );
        if let Some(ref value) = self.ldap_server_metadata {
            properties.insert(
                "LdapServerMetadata".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.logs {
            properties.insert("Logs".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.maintenance_window_start_time {
            properties.insert(
                "MaintenanceWindowStartTime".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "PubliclyAccessible".to_string(),
            crate::value::ToValue::to_value(&self.publicly_accessible),
        );
        if let Some(ref value) = self.security_groups {
            properties.insert(
                "SecurityGroups".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.storage_type {
            properties.insert(
                "StorageType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.subnet_ids {
            properties.insert(
                "SubnetIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Users".to_string(),
            crate::value::ToValue::to_value(&self.users),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amazonmq-configuration.html
pub struct Configuration_ {
    pub authentication_strategy: Option<crate::value::ExpString>,
    pub data: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub engine_type: crate::value::ExpString,
    pub engine_version: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<super::amazonmq::configuration::TagsEntry_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_amazonmq_Configuration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::AmazonMQ::Configuration"
        $($field $value)*)
    };
}
pub use crate::__aws_amazonmq_Configuration as Configuration;
impl crate::template::ToResource for Configuration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AmazonMQ"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Configuration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.authentication_strategy {
            properties.insert(
                "AuthenticationStrategy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.data {
            properties.insert("Data".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "EngineType".to_string(),
            crate::value::ToValue::to_value(&self.engine_type),
        );
        if let Some(ref value) = self.engine_version {
            properties.insert(
                "EngineVersion".to_string(),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amazonmq-configurationassociation.html
pub struct ConfigurationAssociation_ {
    pub broker: crate::value::ExpString,
    pub configuration: super::amazonmq::configurationassociation::ConfigurationId_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_amazonmq_ConfigurationAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::AmazonMQ::ConfigurationAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_amazonmq_ConfigurationAssociation as ConfigurationAssociation;
impl crate::template::ToResource for ConfigurationAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AmazonMQ"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ConfigurationAssociation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Broker".to_string(),
            crate::value::ToValue::to_value(&self.broker),
        );
        properties.insert(
            "Configuration".to_string(),
            crate::value::ToValue::to_value(&self.configuration),
        );
        properties
    }
}
