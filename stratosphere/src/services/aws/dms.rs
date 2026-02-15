pub mod datamigration {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-datamigration-datamigrationsettings.html
    pub struct DataMigrationSettings_ {
        pub cloudwatch_logs_enabled: Option<crate::value::ExpBool>,
        pub number_of_jobs: Option<i32>,
        pub selection_rules: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dms_DataMigration_DataMigrationSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DMS::DataMigration.DataMigrationSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dms_DataMigration_DataMigrationSettings as DataMigrationSettings;
    impl crate::value::ToValue for DataMigrationSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cloudwatch_logs_enabled {
                properties.insert(
                    "CloudwatchLogsEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.number_of_jobs {
                properties.insert(
                    "NumberOfJobs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.selection_rules {
                properties.insert(
                    "SelectionRules".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-datamigration-sourcedatasettings.html
    pub struct SourceDataSettings_ {
        pub cdc_start_position: Option<crate::value::ExpString>,
        pub cdc_start_time: Option<crate::value::ExpString>,
        pub cdc_stop_time: Option<crate::value::ExpString>,
        pub slot_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dms_DataMigration_SourceDataSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DMS::DataMigration.SourceDataSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dms_DataMigration_SourceDataSettings as SourceDataSettings;
    impl crate::value::ToValue for SourceDataSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cdc_start_position {
                properties.insert(
                    "CDCStartPosition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cdc_start_time {
                properties.insert(
                    "CDCStartTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cdc_stop_time {
                properties.insert(
                    "CDCStopTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.slot_name {
                properties.insert(
                    "SlotName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod dataprovider {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-docdbsettings.html
    pub struct DocDbSettings_ {
        pub certificate_arn: Option<crate::value::ExpString>,
        pub database_name: crate::value::ExpString,
        pub port: i32,
        pub server_name: crate::value::ExpString,
        pub ssl_mode: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dms_DataProvider_DocDbSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DMS::DataProvider.DocDbSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dms_DataProvider_DocDbSettings as DocDbSettings;
    impl crate::value::ToValue for DocDbSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.certificate_arn {
                properties.insert(
                    "CertificateArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DatabaseName".to_string(),
                crate::value::ToValue::to_value(&self.database_name),
            );
            properties.insert(
                "Port".to_string(),
                crate::value::ToValue::to_value(&self.port),
            );
            properties.insert(
                "ServerName".to_string(),
                crate::value::ToValue::to_value(&self.server_name),
            );
            if let Some(ref value) = self.ssl_mode {
                properties.insert(
                    "SslMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-ibmdb2luwsettings.html
    pub struct IbmDb2LuwSettings_ {
        pub certificate_arn: Option<crate::value::ExpString>,
        pub database_name: crate::value::ExpString,
        pub port: i32,
        pub server_name: crate::value::ExpString,
        pub ssl_mode: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dms_DataProvider_IbmDb2LuwSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DMS::DataProvider.IbmDb2LuwSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dms_DataProvider_IbmDb2LuwSettings as IbmDb2LuwSettings;
    impl crate::value::ToValue for IbmDb2LuwSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.certificate_arn {
                properties.insert(
                    "CertificateArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DatabaseName".to_string(),
                crate::value::ToValue::to_value(&self.database_name),
            );
            properties.insert(
                "Port".to_string(),
                crate::value::ToValue::to_value(&self.port),
            );
            properties.insert(
                "ServerName".to_string(),
                crate::value::ToValue::to_value(&self.server_name),
            );
            properties.insert(
                "SslMode".to_string(),
                crate::value::ToValue::to_value(&self.ssl_mode),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-ibmdb2zossettings.html
    pub struct IbmDb2zOsSettings_ {
        pub certificate_arn: Option<crate::value::ExpString>,
        pub database_name: crate::value::ExpString,
        pub port: i32,
        pub server_name: crate::value::ExpString,
        pub ssl_mode: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dms_DataProvider_IbmDb2zOsSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DMS::DataProvider.IbmDb2zOsSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dms_DataProvider_IbmDb2zOsSettings as IbmDb2zOsSettings;
    impl crate::value::ToValue for IbmDb2zOsSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.certificate_arn {
                properties.insert(
                    "CertificateArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DatabaseName".to_string(),
                crate::value::ToValue::to_value(&self.database_name),
            );
            properties.insert(
                "Port".to_string(),
                crate::value::ToValue::to_value(&self.port),
            );
            properties.insert(
                "ServerName".to_string(),
                crate::value::ToValue::to_value(&self.server_name),
            );
            properties.insert(
                "SslMode".to_string(),
                crate::value::ToValue::to_value(&self.ssl_mode),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-mariadbsettings.html
    pub struct MariaDbSettings_ {
        pub certificate_arn: Option<crate::value::ExpString>,
        pub port: i32,
        pub server_name: crate::value::ExpString,
        pub ssl_mode: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dms_DataProvider_MariaDbSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DMS::DataProvider.MariaDbSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dms_DataProvider_MariaDbSettings as MariaDbSettings;
    impl crate::value::ToValue for MariaDbSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.certificate_arn {
                properties.insert(
                    "CertificateArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Port".to_string(),
                crate::value::ToValue::to_value(&self.port),
            );
            properties.insert(
                "ServerName".to_string(),
                crate::value::ToValue::to_value(&self.server_name),
            );
            properties.insert(
                "SslMode".to_string(),
                crate::value::ToValue::to_value(&self.ssl_mode),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-microsoftsqlserversettings.html
    pub struct MicrosoftSqlServerSettings_ {
        pub certificate_arn: Option<crate::value::ExpString>,
        pub database_name: crate::value::ExpString,
        pub port: i32,
        pub server_name: crate::value::ExpString,
        pub ssl_mode: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dms_DataProvider_MicrosoftSqlServerSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DMS::DataProvider.MicrosoftSqlServerSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dms_DataProvider_MicrosoftSqlServerSettings as MicrosoftSqlServerSettings;
    impl crate::value::ToValue for MicrosoftSqlServerSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.certificate_arn {
                properties.insert(
                    "CertificateArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DatabaseName".to_string(),
                crate::value::ToValue::to_value(&self.database_name),
            );
            properties.insert(
                "Port".to_string(),
                crate::value::ToValue::to_value(&self.port),
            );
            properties.insert(
                "ServerName".to_string(),
                crate::value::ToValue::to_value(&self.server_name),
            );
            properties.insert(
                "SslMode".to_string(),
                crate::value::ToValue::to_value(&self.ssl_mode),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-mongodbsettings.html
    pub struct MongoDbSettings_ {
        pub auth_mechanism: Option<crate::value::ExpString>,
        pub auth_source: Option<crate::value::ExpString>,
        pub auth_type: Option<crate::value::ExpString>,
        pub certificate_arn: Option<crate::value::ExpString>,
        pub database_name: Option<crate::value::ExpString>,
        pub port: i32,
        pub server_name: crate::value::ExpString,
        pub ssl_mode: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dms_DataProvider_MongoDbSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DMS::DataProvider.MongoDbSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dms_DataProvider_MongoDbSettings as MongoDbSettings;
    impl crate::value::ToValue for MongoDbSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auth_mechanism {
                properties.insert(
                    "AuthMechanism".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.auth_source {
                properties.insert(
                    "AuthSource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.auth_type {
                properties.insert(
                    "AuthType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.certificate_arn {
                properties.insert(
                    "CertificateArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.database_name {
                properties.insert(
                    "DatabaseName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Port".to_string(),
                crate::value::ToValue::to_value(&self.port),
            );
            properties.insert(
                "ServerName".to_string(),
                crate::value::ToValue::to_value(&self.server_name),
            );
            if let Some(ref value) = self.ssl_mode {
                properties.insert(
                    "SslMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-mysqlsettings.html
    pub struct MySqlSettings_ {
        pub certificate_arn: Option<crate::value::ExpString>,
        pub port: i32,
        pub server_name: crate::value::ExpString,
        pub ssl_mode: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dms_DataProvider_MySqlSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DMS::DataProvider.MySqlSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dms_DataProvider_MySqlSettings as MySqlSettings;
    impl crate::value::ToValue for MySqlSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.certificate_arn {
                properties.insert(
                    "CertificateArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Port".to_string(),
                crate::value::ToValue::to_value(&self.port),
            );
            properties.insert(
                "ServerName".to_string(),
                crate::value::ToValue::to_value(&self.server_name),
            );
            properties.insert(
                "SslMode".to_string(),
                crate::value::ToValue::to_value(&self.ssl_mode),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-oraclesettings.html
    pub struct OracleSettings_ {
        pub asm_server: Option<crate::value::ExpString>,
        pub certificate_arn: Option<crate::value::ExpString>,
        pub database_name: crate::value::ExpString,
        pub port: i32,
        pub secrets_manager_oracle_asm_access_role_arn: Option<crate::value::ExpString>,
        pub secrets_manager_oracle_asm_secret_id: Option<crate::value::ExpString>,
        pub secrets_manager_security_db_encryption_access_role_arn: Option<crate::value::ExpString>,
        pub secrets_manager_security_db_encryption_secret_id: Option<crate::value::ExpString>,
        pub server_name: crate::value::ExpString,
        pub ssl_mode: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dms_DataProvider_OracleSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DMS::DataProvider.OracleSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dms_DataProvider_OracleSettings as OracleSettings;
    impl crate::value::ToValue for OracleSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.asm_server {
                properties.insert(
                    "AsmServer".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.certificate_arn {
                properties.insert(
                    "CertificateArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DatabaseName".to_string(),
                crate::value::ToValue::to_value(&self.database_name),
            );
            properties.insert(
                "Port".to_string(),
                crate::value::ToValue::to_value(&self.port),
            );
            if let Some(ref value) = self.secrets_manager_oracle_asm_access_role_arn {
                properties.insert(
                    "SecretsManagerOracleAsmAccessRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secrets_manager_oracle_asm_secret_id {
                properties.insert(
                    "SecretsManagerOracleAsmSecretId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secrets_manager_security_db_encryption_access_role_arn {
                properties.insert(
                    "SecretsManagerSecurityDbEncryptionAccessRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secrets_manager_security_db_encryption_secret_id {
                properties.insert(
                    "SecretsManagerSecurityDbEncryptionSecretId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ServerName".to_string(),
                crate::value::ToValue::to_value(&self.server_name),
            );
            properties.insert(
                "SslMode".to_string(),
                crate::value::ToValue::to_value(&self.ssl_mode),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-postgresqlsettings.html
    pub struct PostgreSqlSettings_ {
        pub certificate_arn: Option<crate::value::ExpString>,
        pub database_name: crate::value::ExpString,
        pub port: i32,
        pub server_name: crate::value::ExpString,
        pub ssl_mode: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dms_DataProvider_PostgreSqlSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DMS::DataProvider.PostgreSqlSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dms_DataProvider_PostgreSqlSettings as PostgreSqlSettings;
    impl crate::value::ToValue for PostgreSqlSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.certificate_arn {
                properties.insert(
                    "CertificateArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DatabaseName".to_string(),
                crate::value::ToValue::to_value(&self.database_name),
            );
            properties.insert(
                "Port".to_string(),
                crate::value::ToValue::to_value(&self.port),
            );
            properties.insert(
                "ServerName".to_string(),
                crate::value::ToValue::to_value(&self.server_name),
            );
            properties.insert(
                "SslMode".to_string(),
                crate::value::ToValue::to_value(&self.ssl_mode),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-redshiftsettings.html
    pub struct RedshiftSettings_ {
        pub database_name: crate::value::ExpString,
        pub port: i32,
        pub server_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dms_DataProvider_RedshiftSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DMS::DataProvider.RedshiftSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dms_DataProvider_RedshiftSettings as RedshiftSettings;
    impl crate::value::ToValue for RedshiftSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DatabaseName".to_string(),
                crate::value::ToValue::to_value(&self.database_name),
            );
            properties.insert(
                "Port".to_string(),
                crate::value::ToValue::to_value(&self.port),
            );
            properties.insert(
                "ServerName".to_string(),
                crate::value::ToValue::to_value(&self.server_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-settings.html
    pub struct Settings_ {
        pub doc_db_settings: Option<Box<DocDbSettings_>>,
        pub ibm_db2_luw_settings: Option<Box<IbmDb2LuwSettings_>>,
        pub ibm_db2z_os_settings: Option<Box<IbmDb2zOsSettings_>>,
        pub maria_db_settings: Option<Box<MariaDbSettings_>>,
        pub microsoft_sql_server_settings: Option<Box<MicrosoftSqlServerSettings_>>,
        pub mongo_db_settings: Option<Box<MongoDbSettings_>>,
        pub my_sql_settings: Option<Box<MySqlSettings_>>,
        pub oracle_settings: Option<Box<OracleSettings_>>,
        pub postgre_sql_settings: Option<Box<PostgreSqlSettings_>>,
        pub redshift_settings: Option<Box<RedshiftSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dms_DataProvider_Settings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DMS::DataProvider.Settings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dms_DataProvider_Settings as Settings;
    impl crate::value::ToValue for Settings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.doc_db_settings {
                properties.insert(
                    "DocDbSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ibm_db2_luw_settings {
                properties.insert(
                    "IbmDb2LuwSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ibm_db2z_os_settings {
                properties.insert(
                    "IbmDb2zOsSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.maria_db_settings {
                properties.insert(
                    "MariaDbSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.microsoft_sql_server_settings {
                properties.insert(
                    "MicrosoftSqlServerSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mongo_db_settings {
                properties.insert(
                    "MongoDbSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.my_sql_settings {
                properties.insert(
                    "MySqlSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.oracle_settings {
                properties.insert(
                    "OracleSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.postgre_sql_settings {
                properties.insert(
                    "PostgreSqlSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.redshift_settings {
                properties.insert(
                    "RedshiftSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod endpoint {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-docdbsettings.html
    pub struct DocDbSettings_ {
        pub docs_to_investigate: Option<i32>,
        pub extract_doc_id: Option<crate::value::ExpBool>,
        pub nesting_level: Option<crate::value::ExpString>,
        pub secrets_manager_access_role_arn: Option<crate::value::ExpString>,
        pub secrets_manager_secret_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dms_Endpoint_DocDbSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DMS::Endpoint.DocDbSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dms_Endpoint_DocDbSettings as DocDbSettings;
    impl crate::value::ToValue for DocDbSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.docs_to_investigate {
                properties.insert(
                    "DocsToInvestigate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.extract_doc_id {
                properties.insert(
                    "ExtractDocId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.nesting_level {
                properties.insert(
                    "NestingLevel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secrets_manager_access_role_arn {
                properties.insert(
                    "SecretsManagerAccessRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secrets_manager_secret_id {
                properties.insert(
                    "SecretsManagerSecretId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-dynamodbsettings.html
    pub struct DynamoDbSettings_ {
        pub service_access_role_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dms_Endpoint_DynamoDbSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DMS::Endpoint.DynamoDbSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dms_Endpoint_DynamoDbSettings as DynamoDbSettings;
    impl crate::value::ToValue for DynamoDbSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.service_access_role_arn {
                properties.insert(
                    "ServiceAccessRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-elasticsearchsettings.html
    pub struct ElasticsearchSettings_ {
        pub endpoint_uri: Option<crate::value::ExpString>,
        pub error_retry_duration: Option<i32>,
        pub full_load_error_percentage: Option<i32>,
        pub service_access_role_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dms_Endpoint_ElasticsearchSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DMS::Endpoint.ElasticsearchSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dms_Endpoint_ElasticsearchSettings as ElasticsearchSettings;
    impl crate::value::ToValue for ElasticsearchSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.endpoint_uri {
                properties.insert(
                    "EndpointUri".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.error_retry_duration {
                properties.insert(
                    "ErrorRetryDuration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.full_load_error_percentage {
                properties.insert(
                    "FullLoadErrorPercentage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.service_access_role_arn {
                properties.insert(
                    "ServiceAccessRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-gcpmysqlsettings.html
    pub struct GcpMySQLSettings_ {
        pub after_connect_script: Option<crate::value::ExpString>,
        pub clean_source_metadata_on_mismatch: Option<crate::value::ExpBool>,
        pub database_name: Option<crate::value::ExpString>,
        pub events_poll_interval: Option<i32>,
        pub max_file_size: Option<i32>,
        pub parallel_load_threads: Option<i32>,
        pub password: Option<crate::value::ExpString>,
        pub port: Option<i32>,
        pub secrets_manager_access_role_arn: Option<crate::value::ExpString>,
        pub secrets_manager_secret_id: Option<crate::value::ExpString>,
        pub server_name: Option<crate::value::ExpString>,
        pub server_timezone: Option<crate::value::ExpString>,
        pub username: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dms_Endpoint_GcpMySQLSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DMS::Endpoint.GcpMySQLSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dms_Endpoint_GcpMySQLSettings as GcpMySQLSettings;
    impl crate::value::ToValue for GcpMySQLSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.after_connect_script {
                properties.insert(
                    "AfterConnectScript".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.clean_source_metadata_on_mismatch {
                properties.insert(
                    "CleanSourceMetadataOnMismatch".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.database_name {
                properties.insert(
                    "DatabaseName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.events_poll_interval {
                properties.insert(
                    "EventsPollInterval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_file_size {
                properties.insert(
                    "MaxFileSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parallel_load_threads {
                properties.insert(
                    "ParallelLoadThreads".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.password {
                properties.insert(
                    "Password".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.port {
                properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.secrets_manager_access_role_arn {
                properties.insert(
                    "SecretsManagerAccessRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secrets_manager_secret_id {
                properties.insert(
                    "SecretsManagerSecretId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.server_name {
                properties.insert(
                    "ServerName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.server_timezone {
                properties.insert(
                    "ServerTimezone".to_string(),
                    crate::value::ToValue::to_value(value),
                );
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-ibmdb2settings.html
    pub struct IbmDb2Settings_ {
        pub current_lsn: Option<crate::value::ExpString>,
        pub keep_csv_files: Option<crate::value::ExpBool>,
        pub load_timeout: Option<i32>,
        pub max_file_size: Option<i32>,
        pub max_k_bytes_per_read: Option<i32>,
        pub secrets_manager_access_role_arn: Option<crate::value::ExpString>,
        pub secrets_manager_secret_id: Option<crate::value::ExpString>,
        pub set_data_capture_changes: Option<crate::value::ExpBool>,
        pub write_buffer_size: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dms_Endpoint_IbmDb2Settings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DMS::Endpoint.IbmDb2Settings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dms_Endpoint_IbmDb2Settings as IbmDb2Settings;
    impl crate::value::ToValue for IbmDb2Settings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.current_lsn {
                properties.insert(
                    "CurrentLsn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.keep_csv_files {
                properties.insert(
                    "KeepCsvFiles".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.load_timeout {
                properties.insert(
                    "LoadTimeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_file_size {
                properties.insert(
                    "MaxFileSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_k_bytes_per_read {
                properties.insert(
                    "MaxKBytesPerRead".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secrets_manager_access_role_arn {
                properties.insert(
                    "SecretsManagerAccessRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secrets_manager_secret_id {
                properties.insert(
                    "SecretsManagerSecretId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.set_data_capture_changes {
                properties.insert(
                    "SetDataCaptureChanges".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.write_buffer_size {
                properties.insert(
                    "WriteBufferSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-kafkasettings.html
    pub struct KafkaSettings_ {
        pub broker: Option<crate::value::ExpString>,
        pub include_control_details: Option<crate::value::ExpBool>,
        pub include_null_and_empty: Option<crate::value::ExpBool>,
        pub include_partition_value: Option<crate::value::ExpBool>,
        pub include_table_alter_operations: Option<crate::value::ExpBool>,
        pub include_transaction_details: Option<crate::value::ExpBool>,
        pub message_format: Option<crate::value::ExpString>,
        pub message_max_bytes: Option<i32>,
        pub no_hex_prefix: Option<crate::value::ExpBool>,
        pub partition_include_schema_table: Option<crate::value::ExpBool>,
        pub sasl_password: Option<crate::value::ExpString>,
        pub sasl_user_name: Option<crate::value::ExpString>,
        pub security_protocol: Option<crate::value::ExpString>,
        pub ssl_ca_certificate_arn: Option<crate::value::ExpString>,
        pub ssl_client_certificate_arn: Option<crate::value::ExpString>,
        pub ssl_client_key_arn: Option<crate::value::ExpString>,
        pub ssl_client_key_password: Option<crate::value::ExpString>,
        pub topic: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dms_Endpoint_KafkaSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DMS::Endpoint.KafkaSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dms_Endpoint_KafkaSettings as KafkaSettings;
    impl crate::value::ToValue for KafkaSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.broker {
                properties.insert("Broker".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.include_control_details {
                properties.insert(
                    "IncludeControlDetails".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_null_and_empty {
                properties.insert(
                    "IncludeNullAndEmpty".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_partition_value {
                properties.insert(
                    "IncludePartitionValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_table_alter_operations {
                properties.insert(
                    "IncludeTableAlterOperations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_transaction_details {
                properties.insert(
                    "IncludeTransactionDetails".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.message_format {
                properties.insert(
                    "MessageFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.message_max_bytes {
                properties.insert(
                    "MessageMaxBytes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.no_hex_prefix {
                properties.insert(
                    "NoHexPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.partition_include_schema_table {
                properties.insert(
                    "PartitionIncludeSchemaTable".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sasl_password {
                properties.insert(
                    "SaslPassword".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sasl_user_name {
                properties.insert(
                    "SaslUserName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.security_protocol {
                properties.insert(
                    "SecurityProtocol".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ssl_ca_certificate_arn {
                properties.insert(
                    "SslCaCertificateArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ssl_client_certificate_arn {
                properties.insert(
                    "SslClientCertificateArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ssl_client_key_arn {
                properties.insert(
                    "SslClientKeyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ssl_client_key_password {
                properties.insert(
                    "SslClientKeyPassword".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.topic {
                properties.insert("Topic".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-kinesissettings.html
    pub struct KinesisSettings_ {
        pub include_control_details: Option<crate::value::ExpBool>,
        pub include_null_and_empty: Option<crate::value::ExpBool>,
        pub include_partition_value: Option<crate::value::ExpBool>,
        pub include_table_alter_operations: Option<crate::value::ExpBool>,
        pub include_transaction_details: Option<crate::value::ExpBool>,
        pub message_format: Option<crate::value::ExpString>,
        pub no_hex_prefix: Option<crate::value::ExpBool>,
        pub partition_include_schema_table: Option<crate::value::ExpBool>,
        pub service_access_role_arn: Option<crate::value::ExpString>,
        pub stream_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dms_Endpoint_KinesisSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DMS::Endpoint.KinesisSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dms_Endpoint_KinesisSettings as KinesisSettings;
    impl crate::value::ToValue for KinesisSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.include_control_details {
                properties.insert(
                    "IncludeControlDetails".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_null_and_empty {
                properties.insert(
                    "IncludeNullAndEmpty".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_partition_value {
                properties.insert(
                    "IncludePartitionValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_table_alter_operations {
                properties.insert(
                    "IncludeTableAlterOperations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_transaction_details {
                properties.insert(
                    "IncludeTransactionDetails".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.message_format {
                properties.insert(
                    "MessageFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.no_hex_prefix {
                properties.insert(
                    "NoHexPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.partition_include_schema_table {
                properties.insert(
                    "PartitionIncludeSchemaTable".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.service_access_role_arn {
                properties.insert(
                    "ServiceAccessRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stream_arn {
                properties.insert(
                    "StreamArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-microsoftsqlserversettings.html
    pub struct MicrosoftSqlServerSettings_ {
        pub bcp_packet_size: Option<i32>,
        pub control_tables_file_group: Option<crate::value::ExpString>,
        pub database_name: Option<crate::value::ExpString>,
        pub force_lob_lookup: Option<crate::value::ExpBool>,
        pub password: Option<crate::value::ExpString>,
        pub port: Option<i32>,
        pub query_single_always_on_node: Option<crate::value::ExpBool>,
        pub read_backup_only: Option<crate::value::ExpBool>,
        pub safeguard_policy: Option<crate::value::ExpString>,
        pub secrets_manager_access_role_arn: Option<crate::value::ExpString>,
        pub secrets_manager_secret_id: Option<crate::value::ExpString>,
        pub server_name: Option<crate::value::ExpString>,
        pub tlog_access_mode: Option<crate::value::ExpString>,
        pub trim_space_in_char: Option<crate::value::ExpBool>,
        pub use_bcp_full_load: Option<crate::value::ExpBool>,
        pub use_third_party_backup_device: Option<crate::value::ExpBool>,
        pub username: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dms_Endpoint_MicrosoftSqlServerSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DMS::Endpoint.MicrosoftSqlServerSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dms_Endpoint_MicrosoftSqlServerSettings as MicrosoftSqlServerSettings;
    impl crate::value::ToValue for MicrosoftSqlServerSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bcp_packet_size {
                properties.insert(
                    "BcpPacketSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.control_tables_file_group {
                properties.insert(
                    "ControlTablesFileGroup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.database_name {
                properties.insert(
                    "DatabaseName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.force_lob_lookup {
                properties.insert(
                    "ForceLobLookup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.password {
                properties.insert(
                    "Password".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.port {
                properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.query_single_always_on_node {
                properties.insert(
                    "QuerySingleAlwaysOnNode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.read_backup_only {
                properties.insert(
                    "ReadBackupOnly".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.safeguard_policy {
                properties.insert(
                    "SafeguardPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secrets_manager_access_role_arn {
                properties.insert(
                    "SecretsManagerAccessRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secrets_manager_secret_id {
                properties.insert(
                    "SecretsManagerSecretId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.server_name {
                properties.insert(
                    "ServerName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tlog_access_mode {
                properties.insert(
                    "TlogAccessMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.trim_space_in_char {
                properties.insert(
                    "TrimSpaceInChar".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.use_bcp_full_load {
                properties.insert(
                    "UseBcpFullLoad".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.use_third_party_backup_device {
                properties.insert(
                    "UseThirdPartyBackupDevice".to_string(),
                    crate::value::ToValue::to_value(value),
                );
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-mongodbsettings.html
    pub struct MongoDbSettings_ {
        pub auth_mechanism: Option<crate::value::ExpString>,
        pub auth_source: Option<crate::value::ExpString>,
        pub auth_type: Option<crate::value::ExpString>,
        pub database_name: Option<crate::value::ExpString>,
        pub docs_to_investigate: Option<crate::value::ExpString>,
        pub extract_doc_id: Option<crate::value::ExpString>,
        pub nesting_level: Option<crate::value::ExpString>,
        pub password: Option<crate::value::ExpString>,
        pub port: Option<i32>,
        pub secrets_manager_access_role_arn: Option<crate::value::ExpString>,
        pub secrets_manager_secret_id: Option<crate::value::ExpString>,
        pub server_name: Option<crate::value::ExpString>,
        pub username: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dms_Endpoint_MongoDbSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DMS::Endpoint.MongoDbSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dms_Endpoint_MongoDbSettings as MongoDbSettings;
    impl crate::value::ToValue for MongoDbSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auth_mechanism {
                properties.insert(
                    "AuthMechanism".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.auth_source {
                properties.insert(
                    "AuthSource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.auth_type {
                properties.insert(
                    "AuthType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.database_name {
                properties.insert(
                    "DatabaseName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.docs_to_investigate {
                properties.insert(
                    "DocsToInvestigate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.extract_doc_id {
                properties.insert(
                    "ExtractDocId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.nesting_level {
                properties.insert(
                    "NestingLevel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.password {
                properties.insert(
                    "Password".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.port {
                properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.secrets_manager_access_role_arn {
                properties.insert(
                    "SecretsManagerAccessRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secrets_manager_secret_id {
                properties.insert(
                    "SecretsManagerSecretId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.server_name {
                properties.insert(
                    "ServerName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-mysqlsettings.html
    pub struct MySqlSettings_ {
        pub after_connect_script: Option<crate::value::ExpString>,
        pub clean_source_metadata_on_mismatch: Option<crate::value::ExpBool>,
        pub events_poll_interval: Option<i32>,
        pub max_file_size: Option<i32>,
        pub parallel_load_threads: Option<i32>,
        pub secrets_manager_access_role_arn: Option<crate::value::ExpString>,
        pub secrets_manager_secret_id: Option<crate::value::ExpString>,
        pub server_timezone: Option<crate::value::ExpString>,
        pub target_db_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dms_Endpoint_MySqlSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DMS::Endpoint.MySqlSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dms_Endpoint_MySqlSettings as MySqlSettings;
    impl crate::value::ToValue for MySqlSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.after_connect_script {
                properties.insert(
                    "AfterConnectScript".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.clean_source_metadata_on_mismatch {
                properties.insert(
                    "CleanSourceMetadataOnMismatch".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.events_poll_interval {
                properties.insert(
                    "EventsPollInterval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_file_size {
                properties.insert(
                    "MaxFileSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parallel_load_threads {
                properties.insert(
                    "ParallelLoadThreads".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secrets_manager_access_role_arn {
                properties.insert(
                    "SecretsManagerAccessRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secrets_manager_secret_id {
                properties.insert(
                    "SecretsManagerSecretId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.server_timezone {
                properties.insert(
                    "ServerTimezone".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_db_type {
                properties.insert(
                    "TargetDbType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-neptunesettings.html
    pub struct NeptuneSettings_ {
        pub error_retry_duration: Option<i32>,
        pub iam_auth_enabled: Option<crate::value::ExpBool>,
        pub max_file_size: Option<i32>,
        pub max_retry_count: Option<i32>,
        pub s3_bucket_folder: Option<crate::value::ExpString>,
        pub s3_bucket_name: Option<crate::value::ExpString>,
        pub service_access_role_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dms_Endpoint_NeptuneSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DMS::Endpoint.NeptuneSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dms_Endpoint_NeptuneSettings as NeptuneSettings;
    impl crate::value::ToValue for NeptuneSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.error_retry_duration {
                properties.insert(
                    "ErrorRetryDuration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.iam_auth_enabled {
                properties.insert(
                    "IamAuthEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_file_size {
                properties.insert(
                    "MaxFileSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_retry_count {
                properties.insert(
                    "MaxRetryCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_bucket_folder {
                properties.insert(
                    "S3BucketFolder".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_bucket_name {
                properties.insert(
                    "S3BucketName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.service_access_role_arn {
                properties.insert(
                    "ServiceAccessRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-oraclesettings.html
    pub struct OracleSettings_ {
        pub access_alternate_directly: Option<crate::value::ExpBool>,
        pub add_supplemental_logging: Option<crate::value::ExpBool>,
        pub additional_archived_log_dest_id: Option<i32>,
        pub allow_select_nested_tables: Option<crate::value::ExpBool>,
        pub archived_log_dest_id: Option<i32>,
        pub archived_logs_only: Option<crate::value::ExpBool>,
        pub asm_password: Option<crate::value::ExpString>,
        pub asm_server: Option<crate::value::ExpString>,
        pub asm_user: Option<crate::value::ExpString>,
        pub char_length_semantics: Option<crate::value::ExpString>,
        pub direct_path_no_log: Option<crate::value::ExpBool>,
        pub direct_path_parallel_load: Option<crate::value::ExpBool>,
        pub enable_homogenous_tablespace: Option<crate::value::ExpBool>,
        pub extra_archived_log_dest_ids: Option<Vec<i32>>,
        pub fail_tasks_on_lob_truncation: Option<crate::value::ExpBool>,
        pub number_datatype_scale: Option<i32>,
        pub oracle_path_prefix: Option<crate::value::ExpString>,
        pub parallel_asm_read_threads: Option<i32>,
        pub read_ahead_blocks: Option<i32>,
        pub read_table_space_name: Option<crate::value::ExpBool>,
        pub replace_path_prefix: Option<crate::value::ExpBool>,
        pub retry_interval: Option<i32>,
        pub secrets_manager_access_role_arn: Option<crate::value::ExpString>,
        pub secrets_manager_oracle_asm_access_role_arn: Option<crate::value::ExpString>,
        pub secrets_manager_oracle_asm_secret_id: Option<crate::value::ExpString>,
        pub secrets_manager_secret_id: Option<crate::value::ExpString>,
        pub security_db_encryption: Option<crate::value::ExpString>,
        pub security_db_encryption_name: Option<crate::value::ExpString>,
        pub spatial_data_option_to_geo_json_function_name: Option<crate::value::ExpString>,
        pub standby_delay_time: Option<i32>,
        pub use_alternate_folder_for_online: Option<crate::value::ExpBool>,
        pub use_b_file: Option<crate::value::ExpBool>,
        pub use_direct_path_full_load: Option<crate::value::ExpBool>,
        pub use_logminer_reader: Option<crate::value::ExpBool>,
        pub use_path_prefix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dms_Endpoint_OracleSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DMS::Endpoint.OracleSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dms_Endpoint_OracleSettings as OracleSettings;
    impl crate::value::ToValue for OracleSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.access_alternate_directly {
                properties.insert(
                    "AccessAlternateDirectly".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.add_supplemental_logging {
                properties.insert(
                    "AddSupplementalLogging".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.additional_archived_log_dest_id {
                properties.insert(
                    "AdditionalArchivedLogDestId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.allow_select_nested_tables {
                properties.insert(
                    "AllowSelectNestedTables".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.archived_log_dest_id {
                properties.insert(
                    "ArchivedLogDestId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.archived_logs_only {
                properties.insert(
                    "ArchivedLogsOnly".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.asm_password {
                properties.insert(
                    "AsmPassword".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.asm_server {
                properties.insert(
                    "AsmServer".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.asm_user {
                properties.insert(
                    "AsmUser".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.char_length_semantics {
                properties.insert(
                    "CharLengthSemantics".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.direct_path_no_log {
                properties.insert(
                    "DirectPathNoLog".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.direct_path_parallel_load {
                properties.insert(
                    "DirectPathParallelLoad".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enable_homogenous_tablespace {
                properties.insert(
                    "EnableHomogenousTablespace".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.extra_archived_log_dest_ids {
                properties.insert(
                    "ExtraArchivedLogDestIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.fail_tasks_on_lob_truncation {
                properties.insert(
                    "FailTasksOnLobTruncation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.number_datatype_scale {
                properties.insert(
                    "NumberDatatypeScale".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.oracle_path_prefix {
                properties.insert(
                    "OraclePathPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parallel_asm_read_threads {
                properties.insert(
                    "ParallelAsmReadThreads".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.read_ahead_blocks {
                properties.insert(
                    "ReadAheadBlocks".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.read_table_space_name {
                properties.insert(
                    "ReadTableSpaceName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.replace_path_prefix {
                properties.insert(
                    "ReplacePathPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.retry_interval {
                properties.insert(
                    "RetryInterval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secrets_manager_access_role_arn {
                properties.insert(
                    "SecretsManagerAccessRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secrets_manager_oracle_asm_access_role_arn {
                properties.insert(
                    "SecretsManagerOracleAsmAccessRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secrets_manager_oracle_asm_secret_id {
                properties.insert(
                    "SecretsManagerOracleAsmSecretId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secrets_manager_secret_id {
                properties.insert(
                    "SecretsManagerSecretId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.security_db_encryption {
                properties.insert(
                    "SecurityDbEncryption".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.security_db_encryption_name {
                properties.insert(
                    "SecurityDbEncryptionName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.spatial_data_option_to_geo_json_function_name {
                properties.insert(
                    "SpatialDataOptionToGeoJsonFunctionName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.standby_delay_time {
                properties.insert(
                    "StandbyDelayTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.use_alternate_folder_for_online {
                properties.insert(
                    "UseAlternateFolderForOnline".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.use_b_file {
                properties.insert(
                    "UseBFile".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.use_direct_path_full_load {
                properties.insert(
                    "UseDirectPathFullLoad".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.use_logminer_reader {
                properties.insert(
                    "UseLogminerReader".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.use_path_prefix {
                properties.insert(
                    "UsePathPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-postgresqlsettings.html
    pub struct PostgreSqlSettings_ {
        pub after_connect_script: Option<crate::value::ExpString>,
        pub babelfish_database_name: Option<crate::value::ExpString>,
        pub capture_ddls: Option<crate::value::ExpBool>,
        pub database_mode: Option<crate::value::ExpString>,
        pub ddl_artifacts_schema: Option<crate::value::ExpString>,
        pub execute_timeout: Option<i32>,
        pub fail_tasks_on_lob_truncation: Option<crate::value::ExpBool>,
        pub heartbeat_enable: Option<crate::value::ExpBool>,
        pub heartbeat_frequency: Option<i32>,
        pub heartbeat_schema: Option<crate::value::ExpString>,
        pub map_boolean_as_boolean: Option<crate::value::ExpBool>,
        pub max_file_size: Option<i32>,
        pub plugin_name: Option<crate::value::ExpString>,
        pub secrets_manager_access_role_arn: Option<crate::value::ExpString>,
        pub secrets_manager_secret_id: Option<crate::value::ExpString>,
        pub slot_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dms_Endpoint_PostgreSqlSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DMS::Endpoint.PostgreSqlSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dms_Endpoint_PostgreSqlSettings as PostgreSqlSettings;
    impl crate::value::ToValue for PostgreSqlSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.after_connect_script {
                properties.insert(
                    "AfterConnectScript".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.babelfish_database_name {
                properties.insert(
                    "BabelfishDatabaseName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.capture_ddls {
                properties.insert(
                    "CaptureDdls".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.database_mode {
                properties.insert(
                    "DatabaseMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ddl_artifacts_schema {
                properties.insert(
                    "DdlArtifactsSchema".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.execute_timeout {
                properties.insert(
                    "ExecuteTimeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.fail_tasks_on_lob_truncation {
                properties.insert(
                    "FailTasksOnLobTruncation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.heartbeat_enable {
                properties.insert(
                    "HeartbeatEnable".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.heartbeat_frequency {
                properties.insert(
                    "HeartbeatFrequency".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.heartbeat_schema {
                properties.insert(
                    "HeartbeatSchema".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.map_boolean_as_boolean {
                properties.insert(
                    "MapBooleanAsBoolean".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_file_size {
                properties.insert(
                    "MaxFileSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.plugin_name {
                properties.insert(
                    "PluginName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secrets_manager_access_role_arn {
                properties.insert(
                    "SecretsManagerAccessRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secrets_manager_secret_id {
                properties.insert(
                    "SecretsManagerSecretId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.slot_name {
                properties.insert(
                    "SlotName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-redissettings.html
    pub struct RedisSettings_ {
        pub auth_password: Option<crate::value::ExpString>,
        pub auth_type: Option<crate::value::ExpString>,
        pub auth_user_name: Option<crate::value::ExpString>,
        pub port: Option<f64>,
        pub server_name: Option<crate::value::ExpString>,
        pub ssl_ca_certificate_arn: Option<crate::value::ExpString>,
        pub ssl_security_protocol: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dms_Endpoint_RedisSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DMS::Endpoint.RedisSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dms_Endpoint_RedisSettings as RedisSettings;
    impl crate::value::ToValue for RedisSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auth_password {
                properties.insert(
                    "AuthPassword".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.auth_type {
                properties.insert(
                    "AuthType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.auth_user_name {
                properties.insert(
                    "AuthUserName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.port {
                properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.server_name {
                properties.insert(
                    "ServerName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ssl_ca_certificate_arn {
                properties.insert(
                    "SslCaCertificateArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ssl_security_protocol {
                properties.insert(
                    "SslSecurityProtocol".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-redshiftsettings.html
    pub struct RedshiftSettings_ {
        pub accept_any_date: Option<crate::value::ExpBool>,
        pub after_connect_script: Option<crate::value::ExpString>,
        pub bucket_folder: Option<crate::value::ExpString>,
        pub bucket_name: Option<crate::value::ExpString>,
        pub case_sensitive_names: Option<crate::value::ExpBool>,
        pub comp_update: Option<crate::value::ExpBool>,
        pub connection_timeout: Option<i32>,
        pub date_format: Option<crate::value::ExpString>,
        pub empty_as_null: Option<crate::value::ExpBool>,
        pub encryption_mode: Option<crate::value::ExpString>,
        pub explicit_ids: Option<crate::value::ExpBool>,
        pub file_transfer_upload_streams: Option<i32>,
        pub load_timeout: Option<i32>,
        pub map_boolean_as_boolean: Option<crate::value::ExpBool>,
        pub max_file_size: Option<i32>,
        pub remove_quotes: Option<crate::value::ExpBool>,
        pub replace_chars: Option<crate::value::ExpString>,
        pub replace_invalid_chars: Option<crate::value::ExpString>,
        pub secrets_manager_access_role_arn: Option<crate::value::ExpString>,
        pub secrets_manager_secret_id: Option<crate::value::ExpString>,
        pub server_side_encryption_kms_key_id: Option<crate::value::ExpString>,
        pub service_access_role_arn: Option<crate::value::ExpString>,
        pub time_format: Option<crate::value::ExpString>,
        pub trim_blanks: Option<crate::value::ExpBool>,
        pub truncate_columns: Option<crate::value::ExpBool>,
        pub write_buffer_size: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dms_Endpoint_RedshiftSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DMS::Endpoint.RedshiftSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dms_Endpoint_RedshiftSettings as RedshiftSettings;
    impl crate::value::ToValue for RedshiftSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.accept_any_date {
                properties.insert(
                    "AcceptAnyDate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.after_connect_script {
                properties.insert(
                    "AfterConnectScript".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bucket_folder {
                properties.insert(
                    "BucketFolder".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bucket_name {
                properties.insert(
                    "BucketName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.case_sensitive_names {
                properties.insert(
                    "CaseSensitiveNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.comp_update {
                properties.insert(
                    "CompUpdate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.connection_timeout {
                properties.insert(
                    "ConnectionTimeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.date_format {
                properties.insert(
                    "DateFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.empty_as_null {
                properties.insert(
                    "EmptyAsNull".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.encryption_mode {
                properties.insert(
                    "EncryptionMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.explicit_ids {
                properties.insert(
                    "ExplicitIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.file_transfer_upload_streams {
                properties.insert(
                    "FileTransferUploadStreams".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.load_timeout {
                properties.insert(
                    "LoadTimeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.map_boolean_as_boolean {
                properties.insert(
                    "MapBooleanAsBoolean".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_file_size {
                properties.insert(
                    "MaxFileSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.remove_quotes {
                properties.insert(
                    "RemoveQuotes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.replace_chars {
                properties.insert(
                    "ReplaceChars".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.replace_invalid_chars {
                properties.insert(
                    "ReplaceInvalidChars".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secrets_manager_access_role_arn {
                properties.insert(
                    "SecretsManagerAccessRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secrets_manager_secret_id {
                properties.insert(
                    "SecretsManagerSecretId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.server_side_encryption_kms_key_id {
                properties.insert(
                    "ServerSideEncryptionKmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.service_access_role_arn {
                properties.insert(
                    "ServiceAccessRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.time_format {
                properties.insert(
                    "TimeFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.trim_blanks {
                properties.insert(
                    "TrimBlanks".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.truncate_columns {
                properties.insert(
                    "TruncateColumns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.write_buffer_size {
                properties.insert(
                    "WriteBufferSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html
    pub struct S3Settings_ {
        pub add_column_name: Option<crate::value::ExpBool>,
        pub add_trailing_padding_character: Option<crate::value::ExpBool>,
        pub bucket_folder: Option<crate::value::ExpString>,
        pub bucket_name: Option<crate::value::ExpString>,
        pub canned_acl_for_objects: Option<crate::value::ExpString>,
        pub cdc_inserts_and_updates: Option<crate::value::ExpBool>,
        pub cdc_inserts_only: Option<crate::value::ExpBool>,
        pub cdc_max_batch_interval: Option<i32>,
        pub cdc_min_file_size: Option<i32>,
        pub cdc_path: Option<crate::value::ExpString>,
        pub compression_type: Option<crate::value::ExpString>,
        pub csv_delimiter: Option<crate::value::ExpString>,
        pub csv_no_sup_value: Option<crate::value::ExpString>,
        pub csv_null_value: Option<crate::value::ExpString>,
        pub csv_row_delimiter: Option<crate::value::ExpString>,
        pub data_format: Option<crate::value::ExpString>,
        pub data_page_size: Option<i32>,
        pub date_partition_delimiter: Option<crate::value::ExpString>,
        pub date_partition_enabled: Option<crate::value::ExpBool>,
        pub date_partition_sequence: Option<crate::value::ExpString>,
        pub date_partition_timezone: Option<crate::value::ExpString>,
        pub dict_page_size_limit: Option<i32>,
        pub enable_statistics: Option<crate::value::ExpBool>,
        pub encoding_type: Option<crate::value::ExpString>,
        pub encryption_mode: Option<crate::value::ExpString>,
        pub expected_bucket_owner: Option<crate::value::ExpString>,
        pub external_table_definition: Option<crate::value::ExpString>,
        pub glue_catalog_generation: Option<crate::value::ExpBool>,
        pub ignore_header_rows: Option<i32>,
        pub include_op_for_full_load: Option<crate::value::ExpBool>,
        pub max_file_size: Option<i32>,
        pub parquet_timestamp_in_millisecond: Option<crate::value::ExpBool>,
        pub parquet_version: Option<crate::value::ExpString>,
        pub preserve_transactions: Option<crate::value::ExpBool>,
        pub rfc4180: Option<crate::value::ExpBool>,
        pub row_group_length: Option<i32>,
        pub server_side_encryption_kms_key_id: Option<crate::value::ExpString>,
        pub service_access_role_arn: Option<crate::value::ExpString>,
        pub timestamp_column_name: Option<crate::value::ExpString>,
        pub use_csv_no_sup_value: Option<crate::value::ExpBool>,
        pub use_task_start_time_for_full_load_timestamp: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dms_Endpoint_S3Settings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DMS::Endpoint.S3Settings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dms_Endpoint_S3Settings as S3Settings;
    impl crate::value::ToValue for S3Settings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.add_column_name {
                properties.insert(
                    "AddColumnName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.add_trailing_padding_character {
                properties.insert(
                    "AddTrailingPaddingCharacter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bucket_folder {
                properties.insert(
                    "BucketFolder".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bucket_name {
                properties.insert(
                    "BucketName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.canned_acl_for_objects {
                properties.insert(
                    "CannedAclForObjects".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cdc_inserts_and_updates {
                properties.insert(
                    "CdcInsertsAndUpdates".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cdc_inserts_only {
                properties.insert(
                    "CdcInsertsOnly".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cdc_max_batch_interval {
                properties.insert(
                    "CdcMaxBatchInterval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cdc_min_file_size {
                properties.insert(
                    "CdcMinFileSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cdc_path {
                properties.insert(
                    "CdcPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.compression_type {
                properties.insert(
                    "CompressionType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.csv_delimiter {
                properties.insert(
                    "CsvDelimiter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.csv_no_sup_value {
                properties.insert(
                    "CsvNoSupValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.csv_null_value {
                properties.insert(
                    "CsvNullValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.csv_row_delimiter {
                properties.insert(
                    "CsvRowDelimiter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.data_format {
                properties.insert(
                    "DataFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.data_page_size {
                properties.insert(
                    "DataPageSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.date_partition_delimiter {
                properties.insert(
                    "DatePartitionDelimiter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.date_partition_enabled {
                properties.insert(
                    "DatePartitionEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.date_partition_sequence {
                properties.insert(
                    "DatePartitionSequence".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.date_partition_timezone {
                properties.insert(
                    "DatePartitionTimezone".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dict_page_size_limit {
                properties.insert(
                    "DictPageSizeLimit".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enable_statistics {
                properties.insert(
                    "EnableStatistics".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.encoding_type {
                properties.insert(
                    "EncodingType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.encryption_mode {
                properties.insert(
                    "EncryptionMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.expected_bucket_owner {
                properties.insert(
                    "ExpectedBucketOwner".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.external_table_definition {
                properties.insert(
                    "ExternalTableDefinition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.glue_catalog_generation {
                properties.insert(
                    "GlueCatalogGeneration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ignore_header_rows {
                properties.insert(
                    "IgnoreHeaderRows".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_op_for_full_load {
                properties.insert(
                    "IncludeOpForFullLoad".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_file_size {
                properties.insert(
                    "MaxFileSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parquet_timestamp_in_millisecond {
                properties.insert(
                    "ParquetTimestampInMillisecond".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parquet_version {
                properties.insert(
                    "ParquetVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.preserve_transactions {
                properties.insert(
                    "PreserveTransactions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rfc4180 {
                properties.insert(
                    "Rfc4180".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.row_group_length {
                properties.insert(
                    "RowGroupLength".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.server_side_encryption_kms_key_id {
                properties.insert(
                    "ServerSideEncryptionKmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.service_access_role_arn {
                properties.insert(
                    "ServiceAccessRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timestamp_column_name {
                properties.insert(
                    "TimestampColumnName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.use_csv_no_sup_value {
                properties.insert(
                    "UseCsvNoSupValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.use_task_start_time_for_full_load_timestamp {
                properties.insert(
                    "UseTaskStartTimeForFullLoadTimestamp".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-sybasesettings.html
    pub struct SybaseSettings_ {
        pub secrets_manager_access_role_arn: Option<crate::value::ExpString>,
        pub secrets_manager_secret_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dms_Endpoint_SybaseSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DMS::Endpoint.SybaseSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dms_Endpoint_SybaseSettings as SybaseSettings;
    impl crate::value::ToValue for SybaseSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.secrets_manager_access_role_arn {
                properties.insert(
                    "SecretsManagerAccessRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secrets_manager_secret_id {
                properties.insert(
                    "SecretsManagerSecretId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod migrationproject {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-migrationproject-dataproviderdescriptor.html
    pub struct DataProviderDescriptor_ {
        pub data_provider_arn: Option<crate::value::ExpString>,
        pub data_provider_identifier: Option<crate::value::ExpString>,
        pub data_provider_name: Option<crate::value::ExpString>,
        pub secrets_manager_access_role_arn: Option<crate::value::ExpString>,
        pub secrets_manager_secret_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dms_MigrationProject_DataProviderDescriptor {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DMS::MigrationProject.DataProviderDescriptor"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dms_MigrationProject_DataProviderDescriptor as DataProviderDescriptor;
    impl crate::value::ToValue for DataProviderDescriptor_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.data_provider_arn {
                properties.insert(
                    "DataProviderArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.data_provider_identifier {
                properties.insert(
                    "DataProviderIdentifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.data_provider_name {
                properties.insert(
                    "DataProviderName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secrets_manager_access_role_arn {
                properties.insert(
                    "SecretsManagerAccessRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secrets_manager_secret_id {
                properties.insert(
                    "SecretsManagerSecretId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-migrationproject-schemaconversionapplicationattributes.html
    pub struct SchemaConversionApplicationAttributes_ {
        pub s3_bucket_path: Option<crate::value::ExpString>,
        pub s3_bucket_role_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dms_MigrationProject_SchemaConversionApplicationAttributes {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DMS::MigrationProject.SchemaConversionApplicationAttributes"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dms_MigrationProject_SchemaConversionApplicationAttributes as SchemaConversionApplicationAttributes;
    impl crate::value::ToValue for SchemaConversionApplicationAttributes_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.s3_bucket_path {
                properties.insert(
                    "S3BucketPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_bucket_role_arn {
                properties.insert(
                    "S3BucketRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod replicationconfig {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-replicationconfig-computeconfig.html
    pub struct ComputeConfig_ {
        pub availability_zone: Option<crate::value::ExpString>,
        pub dns_name_servers: Option<crate::value::ExpString>,
        pub kms_key_id: Option<crate::value::ExpString>,
        pub max_capacity_units: i32,
        pub min_capacity_units: Option<i32>,
        pub multi_az: Option<crate::value::ExpBool>,
        pub preferred_maintenance_window: Option<crate::value::ExpString>,
        pub replication_subnet_group_id: Option<crate::value::ExpString>,
        pub vpc_security_group_ids: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dms_ReplicationConfig_ComputeConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DMS::ReplicationConfig.ComputeConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dms_ReplicationConfig_ComputeConfig as ComputeConfig;
    impl crate::value::ToValue for ComputeConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.availability_zone {
                properties.insert(
                    "AvailabilityZone".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dns_name_servers {
                properties.insert(
                    "DnsNameServers".to_string(),
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
                "MaxCapacityUnits".to_string(),
                crate::value::ToValue::to_value(&self.max_capacity_units),
            );
            if let Some(ref value) = self.min_capacity_units {
                properties.insert(
                    "MinCapacityUnits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.multi_az {
                properties.insert(
                    "MultiAZ".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.preferred_maintenance_window {
                properties.insert(
                    "PreferredMaintenanceWindow".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.replication_subnet_group_id {
                properties.insert(
                    "ReplicationSubnetGroupId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpc_security_group_ids {
                properties.insert(
                    "VpcSecurityGroupIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-certificate.html
pub struct Certificate_ {
    pub certificate_identifier: Option<crate::value::ExpString>,
    pub certificate_pem: Option<crate::value::ExpString>,
    pub certificate_wallet: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_dms_Certificate {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DMS::Certificate"
        $($field $value)*)
    };
}
pub use crate::__aws_dms_Certificate as Certificate;
impl crate::template::ToResource for Certificate_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DMS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Certificate"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.certificate_identifier {
            properties.insert(
                "CertificateIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.certificate_pem {
            properties.insert(
                "CertificatePem".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.certificate_wallet {
            properties.insert(
                "CertificateWallet".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-datamigration.html
pub struct DataMigration_ {
    pub data_migration_identifier: Option<crate::value::ExpString>,
    pub data_migration_name: Option<crate::value::ExpString>,
    pub data_migration_settings: Option<super::dms::datamigration::DataMigrationSettings_>,
    pub data_migration_type: crate::value::ExpString,
    pub migration_project_identifier: crate::value::ExpString,
    pub service_access_role_arn: crate::value::ExpString,
    pub source_data_settings: Option<Vec<super::dms::datamigration::SourceDataSettings_>>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_dms_DataMigration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DMS::DataMigration"
        $($field $value)*)
    };
}
pub use crate::__aws_dms_DataMigration as DataMigration;
impl crate::template::ToResource for DataMigration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DMS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DataMigration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.data_migration_identifier {
            properties.insert(
                "DataMigrationIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.data_migration_name {
            properties.insert(
                "DataMigrationName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.data_migration_settings {
            properties.insert(
                "DataMigrationSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DataMigrationType".to_string(),
            crate::value::ToValue::to_value(&self.data_migration_type),
        );
        properties.insert(
            "MigrationProjectIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.migration_project_identifier),
        );
        properties.insert(
            "ServiceAccessRoleArn".to_string(),
            crate::value::ToValue::to_value(&self.service_access_role_arn),
        );
        if let Some(ref value) = self.source_data_settings {
            properties.insert(
                "SourceDataSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-dataprovider.html
pub struct DataProvider_ {
    pub data_provider_identifier: Option<crate::value::ExpString>,
    pub data_provider_name: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub engine: crate::value::ExpString,
    pub exact_settings: Option<crate::value::ExpBool>,
    pub settings: Option<super::dms::dataprovider::Settings_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_dms_DataProvider {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DMS::DataProvider"
        $($field $value)*)
    };
}
pub use crate::__aws_dms_DataProvider as DataProvider;
impl crate::template::ToResource for DataProvider_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DMS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DataProvider"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.data_provider_identifier {
            properties.insert(
                "DataProviderIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.data_provider_name {
            properties.insert(
                "DataProviderName".to_string(),
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
            "Engine".to_string(),
            crate::value::ToValue::to_value(&self.engine),
        );
        if let Some(ref value) = self.exact_settings {
            properties.insert(
                "ExactSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.settings {
            properties.insert(
                "Settings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-endpoint.html
pub struct Endpoint_ {
    pub certificate_arn: Option<crate::value::ExpString>,
    pub database_name: Option<crate::value::ExpString>,
    pub doc_db_settings: Option<super::dms::endpoint::DocDbSettings_>,
    pub dynamo_db_settings: Option<super::dms::endpoint::DynamoDbSettings_>,
    pub elasticsearch_settings: Option<super::dms::endpoint::ElasticsearchSettings_>,
    pub endpoint_identifier: Option<crate::value::ExpString>,
    pub endpoint_type: crate::value::ExpString,
    pub engine_name: crate::value::ExpString,
    pub extra_connection_attributes: Option<crate::value::ExpString>,
    pub gcp_my_sql_settings: Option<super::dms::endpoint::GcpMySQLSettings_>,
    pub ibm_db2_settings: Option<super::dms::endpoint::IbmDb2Settings_>,
    pub kafka_settings: Option<super::dms::endpoint::KafkaSettings_>,
    pub kinesis_settings: Option<super::dms::endpoint::KinesisSettings_>,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub microsoft_sql_server_settings: Option<super::dms::endpoint::MicrosoftSqlServerSettings_>,
    pub mongo_db_settings: Option<super::dms::endpoint::MongoDbSettings_>,
    pub my_sql_settings: Option<super::dms::endpoint::MySqlSettings_>,
    pub neptune_settings: Option<super::dms::endpoint::NeptuneSettings_>,
    pub oracle_settings: Option<super::dms::endpoint::OracleSettings_>,
    pub password: Option<crate::value::ExpString>,
    pub port: Option<i32>,
    pub postgre_sql_settings: Option<super::dms::endpoint::PostgreSqlSettings_>,
    pub redis_settings: Option<super::dms::endpoint::RedisSettings_>,
    pub redshift_settings: Option<super::dms::endpoint::RedshiftSettings_>,
    pub resource_identifier: Option<crate::value::ExpString>,
    pub s3_settings: Option<super::dms::endpoint::S3Settings_>,
    pub server_name: Option<crate::value::ExpString>,
    pub ssl_mode: Option<crate::value::ExpString>,
    pub sybase_settings: Option<super::dms::endpoint::SybaseSettings_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub username: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_dms_Endpoint {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DMS::Endpoint" $($field
        $value)*)
    };
}
pub use crate::__aws_dms_Endpoint as Endpoint;
impl crate::template::ToResource for Endpoint_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DMS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Endpoint"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.certificate_arn {
            properties.insert(
                "CertificateArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.database_name {
            properties.insert(
                "DatabaseName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.doc_db_settings {
            properties.insert(
                "DocDbSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.dynamo_db_settings {
            properties.insert(
                "DynamoDbSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.elasticsearch_settings {
            properties.insert(
                "ElasticsearchSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.endpoint_identifier {
            properties.insert(
                "EndpointIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "EndpointType".to_string(),
            crate::value::ToValue::to_value(&self.endpoint_type),
        );
        properties.insert(
            "EngineName".to_string(),
            crate::value::ToValue::to_value(&self.engine_name),
        );
        if let Some(ref value) = self.extra_connection_attributes {
            properties.insert(
                "ExtraConnectionAttributes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.gcp_my_sql_settings {
            properties.insert(
                "GcpMySQLSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ibm_db2_settings {
            properties.insert(
                "IbmDb2Settings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kafka_settings {
            properties.insert(
                "KafkaSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kinesis_settings {
            properties.insert(
                "KinesisSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_id {
            properties.insert(
                "KmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.microsoft_sql_server_settings {
            properties.insert(
                "MicrosoftSqlServerSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.mongo_db_settings {
            properties.insert(
                "MongoDbSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.my_sql_settings {
            properties.insert(
                "MySqlSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.neptune_settings {
            properties.insert(
                "NeptuneSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.oracle_settings {
            properties.insert(
                "OracleSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.password {
            properties.insert(
                "Password".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.port {
            properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.postgre_sql_settings {
            properties.insert(
                "PostgreSqlSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.redis_settings {
            properties.insert(
                "RedisSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.redshift_settings {
            properties.insert(
                "RedshiftSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resource_identifier {
            properties.insert(
                "ResourceIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.s3_settings {
            properties.insert(
                "S3Settings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.server_name {
            properties.insert(
                "ServerName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ssl_mode {
            properties.insert(
                "SslMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sybase_settings {
            properties.insert(
                "SybaseSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.username {
            properties.insert(
                "Username".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-eventsubscription.html
pub struct EventSubscription_ {
    pub enabled: Option<crate::value::ExpBool>,
    pub event_categories: Option<Vec<crate::value::ExpString>>,
    pub sns_topic_arn: crate::value::ExpString,
    pub source_ids: Option<Vec<crate::value::ExpString>>,
    pub source_type: Option<crate::value::ExpString>,
    pub subscription_name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_dms_EventSubscription {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DMS::EventSubscription"
        $($field $value)*)
    };
}
pub use crate::__aws_dms_EventSubscription as EventSubscription;
impl crate::template::ToResource for EventSubscription_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DMS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("EventSubscription"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.enabled {
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.event_categories {
            properties.insert(
                "EventCategories".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SnsTopicArn".to_string(),
            crate::value::ToValue::to_value(&self.sns_topic_arn),
        );
        if let Some(ref value) = self.source_ids {
            properties.insert(
                "SourceIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_type {
            properties.insert(
                "SourceType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.subscription_name {
            properties.insert(
                "SubscriptionName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-instanceprofile.html
pub struct InstanceProfile_ {
    pub availability_zone: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub instance_profile_identifier: Option<crate::value::ExpString>,
    pub instance_profile_name: Option<crate::value::ExpString>,
    pub kms_key_arn: Option<crate::value::ExpString>,
    pub network_type: Option<crate::value::ExpString>,
    pub publicly_accessible: Option<crate::value::ExpBool>,
    pub subnet_group_identifier: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_security_groups: Option<Vec<crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_dms_InstanceProfile {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DMS::InstanceProfile"
        $($field $value)*)
    };
}
pub use crate::__aws_dms_InstanceProfile as InstanceProfile;
impl crate::template::ToResource for InstanceProfile_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DMS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("InstanceProfile"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.availability_zone {
            properties.insert(
                "AvailabilityZone".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.instance_profile_identifier {
            properties.insert(
                "InstanceProfileIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.instance_profile_name {
            properties.insert(
                "InstanceProfileName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_arn {
            properties.insert(
                "KmsKeyArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.network_type {
            properties.insert(
                "NetworkType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.publicly_accessible {
            properties.insert(
                "PubliclyAccessible".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.subnet_group_identifier {
            properties.insert(
                "SubnetGroupIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.vpc_security_groups {
            properties.insert(
                "VpcSecurityGroups".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-migrationproject.html
pub struct MigrationProject_ {
    pub description: Option<crate::value::ExpString>,
    pub instance_profile_arn: Option<crate::value::ExpString>,
    pub instance_profile_identifier: Option<crate::value::ExpString>,
    pub instance_profile_name: Option<crate::value::ExpString>,
    pub migration_project_identifier: Option<crate::value::ExpString>,
    pub migration_project_name: Option<crate::value::ExpString>,
    pub schema_conversion_application_attributes:
        Option<super::dms::migrationproject::SchemaConversionApplicationAttributes_>,
    pub source_data_provider_descriptors:
        Option<Vec<super::dms::migrationproject::DataProviderDescriptor_>>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub target_data_provider_descriptors:
        Option<Vec<super::dms::migrationproject::DataProviderDescriptor_>>,
    pub transformation_rules: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_dms_MigrationProject {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DMS::MigrationProject"
        $($field $value)*)
    };
}
pub use crate::__aws_dms_MigrationProject as MigrationProject;
impl crate::template::ToResource for MigrationProject_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DMS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("MigrationProject"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.instance_profile_arn {
            properties.insert(
                "InstanceProfileArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.instance_profile_identifier {
            properties.insert(
                "InstanceProfileIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.instance_profile_name {
            properties.insert(
                "InstanceProfileName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.migration_project_identifier {
            properties.insert(
                "MigrationProjectIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.migration_project_name {
            properties.insert(
                "MigrationProjectName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.schema_conversion_application_attributes {
            properties.insert(
                "SchemaConversionApplicationAttributes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_data_provider_descriptors {
            properties.insert(
                "SourceDataProviderDescriptors".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.target_data_provider_descriptors {
            properties.insert(
                "TargetDataProviderDescriptors".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.transformation_rules {
            properties.insert(
                "TransformationRules".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationconfig.html
pub struct ReplicationConfig_ {
    pub compute_config: super::dms::replicationconfig::ComputeConfig_,
    pub replication_config_identifier: crate::value::ExpString,
    pub replication_settings: Option<serde_json::Value>,
    pub replication_type: crate::value::ExpString,
    pub resource_identifier: Option<crate::value::ExpString>,
    pub source_endpoint_arn: crate::value::ExpString,
    pub supplemental_settings: Option<serde_json::Value>,
    pub table_mappings: serde_json::Value,
    pub tags: Option<Vec<crate::Tag_>>,
    pub target_endpoint_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_dms_ReplicationConfig {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DMS::ReplicationConfig"
        $($field $value)*)
    };
}
pub use crate::__aws_dms_ReplicationConfig as ReplicationConfig;
impl crate::template::ToResource for ReplicationConfig_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DMS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ReplicationConfig"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ComputeConfig".to_string(),
            crate::value::ToValue::to_value(&self.compute_config),
        );
        properties.insert(
            "ReplicationConfigIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.replication_config_identifier),
        );
        if let Some(ref value) = self.replication_settings {
            properties.insert(
                "ReplicationSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ReplicationType".to_string(),
            crate::value::ToValue::to_value(&self.replication_type),
        );
        if let Some(ref value) = self.resource_identifier {
            properties.insert(
                "ResourceIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SourceEndpointArn".to_string(),
            crate::value::ToValue::to_value(&self.source_endpoint_arn),
        );
        if let Some(ref value) = self.supplemental_settings {
            properties.insert(
                "SupplementalSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "TableMappings".to_string(),
            crate::value::ToValue::to_value(&self.table_mappings),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TargetEndpointArn".to_string(),
            crate::value::ToValue::to_value(&self.target_endpoint_arn),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationinstance.html
pub struct ReplicationInstance_ {
    pub allocated_storage: Option<i32>,
    pub allow_major_version_upgrade: Option<crate::value::ExpBool>,
    pub auto_minor_version_upgrade: Option<crate::value::ExpBool>,
    pub availability_zone: Option<crate::value::ExpString>,
    pub dns_name_servers: Option<crate::value::ExpString>,
    pub engine_version: Option<crate::value::ExpString>,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub multi_az: Option<crate::value::ExpBool>,
    pub network_type: Option<crate::value::ExpString>,
    pub preferred_maintenance_window: Option<crate::value::ExpString>,
    pub publicly_accessible: Option<crate::value::ExpBool>,
    pub replication_instance_class: crate::value::ExpString,
    pub replication_instance_identifier: Option<crate::value::ExpString>,
    pub replication_subnet_group_identifier: Option<crate::value::ExpString>,
    pub resource_identifier: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_security_group_ids: Option<Vec<crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_dms_ReplicationInstance {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DMS::ReplicationInstance"
        $($field $value)*)
    };
}
pub use crate::__aws_dms_ReplicationInstance as ReplicationInstance;
impl crate::template::ToResource for ReplicationInstance_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DMS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ReplicationInstance"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.allocated_storage {
            properties.insert(
                "AllocatedStorage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.allow_major_version_upgrade {
            properties.insert(
                "AllowMajorVersionUpgrade".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.auto_minor_version_upgrade {
            properties.insert(
                "AutoMinorVersionUpgrade".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.availability_zone {
            properties.insert(
                "AvailabilityZone".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.dns_name_servers {
            properties.insert(
                "DnsNameServers".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.engine_version {
            properties.insert(
                "EngineVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_id {
            properties.insert(
                "KmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.multi_az {
            properties.insert(
                "MultiAZ".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.network_type {
            properties.insert(
                "NetworkType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.preferred_maintenance_window {
            properties.insert(
                "PreferredMaintenanceWindow".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.publicly_accessible {
            properties.insert(
                "PubliclyAccessible".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ReplicationInstanceClass".to_string(),
            crate::value::ToValue::to_value(&self.replication_instance_class),
        );
        if let Some(ref value) = self.replication_instance_identifier {
            properties.insert(
                "ReplicationInstanceIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.replication_subnet_group_identifier {
            properties.insert(
                "ReplicationSubnetGroupIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resource_identifier {
            properties.insert(
                "ResourceIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.vpc_security_group_ids {
            properties.insert(
                "VpcSecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationsubnetgroup.html
pub struct ReplicationSubnetGroup_ {
    pub replication_subnet_group_description: crate::value::ExpString,
    pub replication_subnet_group_identifier: Option<crate::value::ExpString>,
    pub subnet_ids: Vec<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_dms_ReplicationSubnetGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DMS::ReplicationSubnetGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_dms_ReplicationSubnetGroup as ReplicationSubnetGroup;
impl crate::template::ToResource for ReplicationSubnetGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DMS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ReplicationSubnetGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ReplicationSubnetGroupDescription".to_string(),
            crate::value::ToValue::to_value(&self.replication_subnet_group_description),
        );
        if let Some(ref value) = self.replication_subnet_group_identifier {
            properties.insert(
                "ReplicationSubnetGroupIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SubnetIds".to_string(),
            crate::value::ToValue::to_value(&self.subnet_ids),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationtask.html
pub struct ReplicationTask_ {
    pub cdc_start_position: Option<crate::value::ExpString>,
    pub cdc_start_time: Option<f64>,
    pub cdc_stop_position: Option<crate::value::ExpString>,
    pub migration_type: crate::value::ExpString,
    pub replication_instance_arn: crate::value::ExpString,
    pub replication_task_identifier: Option<crate::value::ExpString>,
    pub replication_task_settings: Option<crate::value::ExpString>,
    pub resource_identifier: Option<crate::value::ExpString>,
    pub source_endpoint_arn: crate::value::ExpString,
    pub table_mappings: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub target_endpoint_arn: crate::value::ExpString,
    pub task_data: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_dms_ReplicationTask {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DMS::ReplicationTask"
        $($field $value)*)
    };
}
pub use crate::__aws_dms_ReplicationTask as ReplicationTask;
impl crate::template::ToResource for ReplicationTask_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DMS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ReplicationTask"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.cdc_start_position {
            properties.insert(
                "CdcStartPosition".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cdc_start_time {
            properties.insert(
                "CdcStartTime".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cdc_stop_position {
            properties.insert(
                "CdcStopPosition".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "MigrationType".to_string(),
            crate::value::ToValue::to_value(&self.migration_type),
        );
        properties.insert(
            "ReplicationInstanceArn".to_string(),
            crate::value::ToValue::to_value(&self.replication_instance_arn),
        );
        if let Some(ref value) = self.replication_task_identifier {
            properties.insert(
                "ReplicationTaskIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.replication_task_settings {
            properties.insert(
                "ReplicationTaskSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resource_identifier {
            properties.insert(
                "ResourceIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SourceEndpointArn".to_string(),
            crate::value::ToValue::to_value(&self.source_endpoint_arn),
        );
        properties.insert(
            "TableMappings".to_string(),
            crate::value::ToValue::to_value(&self.table_mappings),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TargetEndpointArn".to_string(),
            crate::value::ToValue::to_value(&self.target_endpoint_arn),
        );
        if let Some(ref value) = self.task_data {
            properties.insert(
                "TaskData".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
