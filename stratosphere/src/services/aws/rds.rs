pub mod dbcluster {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbcluster-dbclusterrole.html
    pub struct DBClusterRole_ {
        pub feature_name: Option<crate::value::ExpString>,
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rds_DBCluster_DBClusterRole {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RDS::DBCluster.DBClusterRole"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rds_DBCluster_DBClusterRole as DBClusterRole;
    impl crate::value::ToValue for DBClusterRole_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.feature_name {
                properties.insert(
                    "FeatureName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbcluster-endpoint.html
    pub struct Endpoint_ {
        pub address: Option<crate::value::ExpString>,
        pub port: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rds_DBCluster_Endpoint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RDS::DBCluster.Endpoint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rds_DBCluster_Endpoint as Endpoint;
    impl crate::value::ToValue for Endpoint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.address {
                properties.insert(
                    "Address".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.port {
                properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbcluster-masterusersecret.html
    pub struct MasterUserSecret_ {
        pub kms_key_id: Option<crate::value::ExpString>,
        pub secret_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rds_DBCluster_MasterUserSecret {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RDS::DBCluster.MasterUserSecret"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rds_DBCluster_MasterUserSecret as MasterUserSecret;
    impl crate::value::ToValue for MasterUserSecret_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secret_arn {
                properties.insert(
                    "SecretArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbcluster-readendpoint.html
    pub struct ReadEndpoint_ {
        pub address: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rds_DBCluster_ReadEndpoint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RDS::DBCluster.ReadEndpoint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rds_DBCluster_ReadEndpoint as ReadEndpoint;
    impl crate::value::ToValue for ReadEndpoint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.address {
                properties.insert(
                    "Address".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbcluster-scalingconfiguration.html
    pub struct ScalingConfiguration_ {
        pub auto_pause: Option<crate::value::ExpBool>,
        pub max_capacity: Option<i32>,
        pub min_capacity: Option<i32>,
        pub seconds_before_timeout: Option<i32>,
        pub seconds_until_auto_pause: Option<i32>,
        pub timeout_action: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rds_DBCluster_ScalingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RDS::DBCluster.ScalingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rds_DBCluster_ScalingConfiguration as ScalingConfiguration;
    impl crate::value::ToValue for ScalingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auto_pause {
                properties.insert(
                    "AutoPause".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_capacity {
                properties.insert(
                    "MaxCapacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.min_capacity {
                properties.insert(
                    "MinCapacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.seconds_before_timeout {
                properties.insert(
                    "SecondsBeforeTimeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.seconds_until_auto_pause {
                properties.insert(
                    "SecondsUntilAutoPause".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timeout_action {
                properties.insert(
                    "TimeoutAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbcluster-serverlessv2scalingconfiguration.html
    pub struct ServerlessV2ScalingConfiguration_ {
        pub max_capacity: Option<f64>,
        pub min_capacity: Option<f64>,
        pub seconds_until_auto_pause: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rds_DBCluster_ServerlessV2ScalingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RDS::DBCluster.ServerlessV2ScalingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rds_DBCluster_ServerlessV2ScalingConfiguration as ServerlessV2ScalingConfiguration;
    impl crate::value::ToValue for ServerlessV2ScalingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max_capacity {
                properties.insert(
                    "MaxCapacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.min_capacity {
                properties.insert(
                    "MinCapacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.seconds_until_auto_pause {
                properties.insert(
                    "SecondsUntilAutoPause".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod dbinstance {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbinstance-additionalstoragevolume.html
    pub struct AdditionalStorageVolume_ {
        pub allocated_storage: Option<crate::value::ExpString>,
        pub iops: Option<i32>,
        pub max_allocated_storage: Option<i32>,
        pub storage_throughput: Option<i32>,
        pub storage_type: Option<crate::value::ExpString>,
        pub volume_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rds_DBInstance_AdditionalStorageVolume {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RDS::DBInstance.AdditionalStorageVolume"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rds_DBInstance_AdditionalStorageVolume as AdditionalStorageVolume;
    impl crate::value::ToValue for AdditionalStorageVolume_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allocated_storage {
                properties.insert(
                    "AllocatedStorage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.iops {
                properties.insert("Iops".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.max_allocated_storage {
                properties.insert(
                    "MaxAllocatedStorage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.storage_throughput {
                properties.insert(
                    "StorageThroughput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.storage_type {
                properties.insert(
                    "StorageType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.volume_name {
                properties.insert(
                    "VolumeName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbinstance-certificatedetails.html
    pub struct CertificateDetails_ {
        pub ca_identifier: Option<crate::value::ExpString>,
        pub valid_till: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rds_DBInstance_CertificateDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RDS::DBInstance.CertificateDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rds_DBInstance_CertificateDetails as CertificateDetails;
    impl crate::value::ToValue for CertificateDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ca_identifier {
                properties.insert(
                    "CAIdentifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.valid_till {
                properties.insert(
                    "ValidTill".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbinstance-dbinstancerole.html
    pub struct DBInstanceRole_ {
        pub feature_name: crate::value::ExpString,
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rds_DBInstance_DBInstanceRole {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RDS::DBInstance.DBInstanceRole"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rds_DBInstance_DBInstanceRole as DBInstanceRole;
    impl crate::value::ToValue for DBInstanceRole_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FeatureName".to_string(),
                crate::value::ToValue::to_value(&self.feature_name),
            );
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbinstance-dbinstancestatusinfo.html
    pub struct DBInstanceStatusInfo_ {
        pub message: Option<crate::value::ExpString>,
        pub normal: Option<crate::value::ExpBool>,
        pub status: Option<crate::value::ExpString>,
        pub status_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rds_DBInstance_DBInstanceStatusInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RDS::DBInstance.DBInstanceStatusInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rds_DBInstance_DBInstanceStatusInfo as DBInstanceStatusInfo;
    impl crate::value::ToValue for DBInstanceStatusInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.message {
                properties.insert(
                    "Message".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.normal {
                properties.insert("Normal".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.status_type {
                properties.insert(
                    "StatusType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbinstance-endpoint.html
    pub struct Endpoint_ {
        pub address: Option<crate::value::ExpString>,
        pub hosted_zone_id: Option<crate::value::ExpString>,
        pub port: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rds_DBInstance_Endpoint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RDS::DBInstance.Endpoint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rds_DBInstance_Endpoint as Endpoint;
    impl crate::value::ToValue for Endpoint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.address {
                properties.insert(
                    "Address".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hosted_zone_id {
                properties.insert(
                    "HostedZoneId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.port {
                properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbinstance-masterusersecret.html
    pub struct MasterUserSecret_ {
        pub kms_key_id: Option<crate::value::ExpString>,
        pub secret_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rds_DBInstance_MasterUserSecret {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RDS::DBInstance.MasterUserSecret"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rds_DBInstance_MasterUserSecret as MasterUserSecret;
    impl crate::value::ToValue for MasterUserSecret_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secret_arn {
                properties.insert(
                    "SecretArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbinstance-processorfeature.html
    pub struct ProcessorFeature_ {
        pub name: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rds_DBInstance_ProcessorFeature {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RDS::DBInstance.ProcessorFeature"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rds_DBInstance_ProcessorFeature as ProcessorFeature;
    impl crate::value::ToValue for ProcessorFeature_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod dbproxy {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxy-authformat.html
    pub struct AuthFormat_ {
        pub auth_scheme: Option<crate::value::ExpString>,
        pub client_password_auth_type: Option<crate::value::ExpString>,
        pub description: Option<crate::value::ExpString>,
        pub iam_auth: Option<crate::value::ExpString>,
        pub secret_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rds_DBProxy_AuthFormat {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RDS::DBProxy.AuthFormat"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rds_DBProxy_AuthFormat as AuthFormat;
    impl crate::value::ToValue for AuthFormat_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auth_scheme {
                properties.insert(
                    "AuthScheme".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.client_password_auth_type {
                properties.insert(
                    "ClientPasswordAuthType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.iam_auth {
                properties.insert(
                    "IAMAuth".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secret_arn {
                properties.insert(
                    "SecretArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxy-tagformat.html
    pub struct TagFormat_ {
        pub key: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rds_DBProxy_TagFormat {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RDS::DBProxy.TagFormat"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rds_DBProxy_TagFormat as TagFormat;
    impl crate::value::ToValue for TagFormat_ {
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
pub mod dbproxyendpoint {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxyendpoint-tagformat.html
    pub struct TagFormat_ {
        pub key: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rds_DBProxyEndpoint_TagFormat {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RDS::DBProxyEndpoint.TagFormat"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rds_DBProxyEndpoint_TagFormat as TagFormat;
    impl crate::value::ToValue for TagFormat_ {
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
pub mod dbproxytargetgroup {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxytargetgroup-connectionpoolconfigurationinfoformat.html
    pub struct ConnectionPoolConfigurationInfoFormat_ {
        pub connection_borrow_timeout: Option<i32>,
        pub init_query: Option<crate::value::ExpString>,
        pub max_connections_percent: Option<i32>,
        pub max_idle_connections_percent: Option<i32>,
        pub session_pinning_filters: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rds_DBProxyTargetGroup_ConnectionPoolConfigurationInfoFormat {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RDS::DBProxyTargetGroup.ConnectionPoolConfigurationInfoFormat"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rds_DBProxyTargetGroup_ConnectionPoolConfigurationInfoFormat as ConnectionPoolConfigurationInfoFormat;
    impl crate::value::ToValue for ConnectionPoolConfigurationInfoFormat_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.connection_borrow_timeout {
                properties.insert(
                    "ConnectionBorrowTimeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.init_query {
                properties.insert(
                    "InitQuery".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_connections_percent {
                properties.insert(
                    "MaxConnectionsPercent".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_idle_connections_percent {
                properties.insert(
                    "MaxIdleConnectionsPercent".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.session_pinning_filters {
                properties.insert(
                    "SessionPinningFilters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod dbsecuritygroup {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-security-group-rule.html
    pub struct Ingress_ {
        pub cidrip: Option<crate::value::ExpString>,
        pub ec2_security_group_id: Option<crate::value::ExpString>,
        pub ec2_security_group_name: Option<crate::value::ExpString>,
        pub ec2_security_group_owner_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rds_DBSecurityGroup_Ingress {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RDS::DBSecurityGroup.Ingress"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rds_DBSecurityGroup_Ingress as Ingress;
    impl crate::value::ToValue for Ingress_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cidrip {
                properties.insert("CIDRIP".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.ec2_security_group_id {
                properties.insert(
                    "EC2SecurityGroupId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ec2_security_group_name {
                properties.insert(
                    "EC2SecurityGroupName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ec2_security_group_owner_id {
                properties.insert(
                    "EC2SecurityGroupOwnerId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod globalcluster {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-globalcluster-globalendpoint.html
    pub struct GlobalEndpoint_ {
        pub address: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rds_GlobalCluster_GlobalEndpoint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RDS::GlobalCluster.GlobalEndpoint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rds_GlobalCluster_GlobalEndpoint as GlobalEndpoint;
    impl crate::value::ToValue for GlobalEndpoint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.address {
                properties.insert(
                    "Address".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod optiongroup {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-optiongroup-optionconfiguration.html
    pub struct OptionConfiguration_ {
        pub db_security_group_memberships: Option<Vec<crate::value::ExpString>>,
        pub option_name: crate::value::ExpString,
        pub option_settings: Option<Vec<OptionSetting_>>,
        pub option_version: Option<crate::value::ExpString>,
        pub port: Option<i32>,
        pub vpc_security_group_memberships: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rds_OptionGroup_OptionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RDS::OptionGroup.OptionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rds_OptionGroup_OptionConfiguration as OptionConfiguration;
    impl crate::value::ToValue for OptionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.db_security_group_memberships {
                properties.insert(
                    "DBSecurityGroupMemberships".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "OptionName".to_string(),
                crate::value::ToValue::to_value(&self.option_name),
            );
            if let Some(ref value) = self.option_settings {
                properties.insert(
                    "OptionSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.option_version {
                properties.insert(
                    "OptionVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.port {
                properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.vpc_security_group_memberships {
                properties.insert(
                    "VpcSecurityGroupMemberships".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-optiongroup-optionsetting.html
    pub struct OptionSetting_ {
        pub name: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rds_OptionGroup_OptionSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RDS::OptionGroup.OptionSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rds_OptionGroup_OptionSetting as OptionSetting;
    impl crate::value::ToValue for OptionSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-customdbengineversion.html
pub struct CustomDBEngineVersion_ {
    pub database_installation_files_s3_bucket_name: Option<crate::value::ExpString>,
    pub database_installation_files_s3_prefix: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub engine: crate::value::ExpString,
    pub engine_version: crate::value::ExpString,
    pub image_id: Option<crate::value::ExpString>,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub manifest: Option<crate::value::ExpString>,
    pub source_custom_db_engine_version_identifier: Option<crate::value::ExpString>,
    pub status: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub use_aws_provided_latest_image: Option<crate::value::ExpBool>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_rds_CustomDBEngineVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::RDS::CustomDBEngineVersion"
        $($field $value)*)
    };
}
pub use crate::__aws_rds_CustomDBEngineVersion as CustomDBEngineVersion;
impl crate::template::ToResource for CustomDBEngineVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RDS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("CustomDBEngineVersion"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.database_installation_files_s3_bucket_name {
            properties.insert(
                "DatabaseInstallationFilesS3BucketName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.database_installation_files_s3_prefix {
            properties.insert(
                "DatabaseInstallationFilesS3Prefix".to_string(),
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
        properties.insert(
            "EngineVersion".to_string(),
            crate::value::ToValue::to_value(&self.engine_version),
        );
        if let Some(ref value) = self.image_id {
            properties.insert(
                "ImageId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_id {
            properties.insert(
                "KMSKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.manifest {
            properties.insert(
                "Manifest".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_custom_db_engine_version_identifier {
            properties.insert(
                "SourceCustomDbEngineVersionIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.status {
            properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.use_aws_provided_latest_image {
            properties.insert(
                "UseAwsProvidedLatestImage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html
pub struct DBCluster_ {
    pub allocated_storage: Option<i32>,
    pub associated_roles: Option<Vec<super::rds::dbcluster::DBClusterRole_>>,
    pub auto_minor_version_upgrade: Option<crate::value::ExpBool>,
    pub availability_zones: Option<Vec<crate::value::ExpString>>,
    pub backtrack_window: Option<i32>,
    pub backup_retention_period: Option<i32>,
    pub cluster_scalability_type: Option<crate::value::ExpString>,
    pub copy_tags_to_snapshot: Option<crate::value::ExpBool>,
    pub db_cluster_identifier: Option<crate::value::ExpString>,
    pub db_cluster_instance_class: Option<crate::value::ExpString>,
    pub db_cluster_parameter_group_name: Option<crate::value::ExpString>,
    pub db_instance_parameter_group_name: Option<crate::value::ExpString>,
    pub db_subnet_group_name: Option<crate::value::ExpString>,
    pub db_system_id: Option<crate::value::ExpString>,
    pub database_insights_mode: Option<crate::value::ExpString>,
    pub database_name: Option<crate::value::ExpString>,
    pub delete_automated_backups: Option<crate::value::ExpBool>,
    pub deletion_protection: Option<crate::value::ExpBool>,
    pub domain: Option<crate::value::ExpString>,
    pub domain_iam_role_name: Option<crate::value::ExpString>,
    pub enable_cloudwatch_logs_exports: Option<Vec<crate::value::ExpString>>,
    pub enable_global_write_forwarding: Option<crate::value::ExpBool>,
    pub enable_http_endpoint: Option<crate::value::ExpBool>,
    pub enable_iam_database_authentication: Option<crate::value::ExpBool>,
    pub enable_local_write_forwarding: Option<crate::value::ExpBool>,
    pub engine: Option<crate::value::ExpString>,
    pub engine_lifecycle_support: Option<crate::value::ExpString>,
    pub engine_mode: Option<crate::value::ExpString>,
    pub engine_version: Option<crate::value::ExpString>,
    pub global_cluster_identifier: Option<crate::value::ExpString>,
    pub iops: Option<i32>,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub manage_master_user_password: Option<crate::value::ExpBool>,
    pub master_user_authentication_type: Option<crate::value::ExpString>,
    pub master_user_password: Option<crate::value::ExpString>,
    pub master_user_secret: Option<super::rds::dbcluster::MasterUserSecret_>,
    pub master_username: Option<crate::value::ExpString>,
    pub monitoring_interval: Option<i32>,
    pub monitoring_role_arn: Option<crate::value::ExpString>,
    pub network_type: Option<crate::value::ExpString>,
    pub performance_insights_enabled: Option<crate::value::ExpBool>,
    pub performance_insights_kms_key_id: Option<crate::value::ExpString>,
    pub performance_insights_retention_period: Option<i32>,
    pub port: Option<i32>,
    pub preferred_backup_window: Option<crate::value::ExpString>,
    pub preferred_maintenance_window: Option<crate::value::ExpString>,
    pub publicly_accessible: Option<crate::value::ExpBool>,
    pub replication_source_identifier: Option<crate::value::ExpString>,
    pub restore_to_time: Option<crate::value::ExpString>,
    pub restore_type: Option<crate::value::ExpString>,
    pub scaling_configuration: Option<super::rds::dbcluster::ScalingConfiguration_>,
    pub serverless_v2_scaling_configuration:
        Option<super::rds::dbcluster::ServerlessV2ScalingConfiguration_>,
    pub snapshot_identifier: Option<crate::value::ExpString>,
    pub source_db_cluster_identifier: Option<crate::value::ExpString>,
    pub source_db_cluster_resource_id: Option<crate::value::ExpString>,
    pub source_region: Option<crate::value::ExpString>,
    pub storage_encrypted: Option<crate::value::ExpBool>,
    pub storage_type: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub use_latest_restorable_time: Option<crate::value::ExpBool>,
    pub vpc_security_group_ids: Option<Vec<crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_rds_DBCluster {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::RDS::DBCluster" $($field
        $value)*)
    };
}
pub use crate::__aws_rds_DBCluster as DBCluster;
impl crate::template::ToResource for DBCluster_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RDS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DBCluster"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.allocated_storage {
            properties.insert(
                "AllocatedStorage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.associated_roles {
            properties.insert(
                "AssociatedRoles".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.auto_minor_version_upgrade {
            properties.insert(
                "AutoMinorVersionUpgrade".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.availability_zones {
            properties.insert(
                "AvailabilityZones".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.backtrack_window {
            properties.insert(
                "BacktrackWindow".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.backup_retention_period {
            properties.insert(
                "BackupRetentionPeriod".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cluster_scalability_type {
            properties.insert(
                "ClusterScalabilityType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.copy_tags_to_snapshot {
            properties.insert(
                "CopyTagsToSnapshot".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.db_cluster_identifier {
            properties.insert(
                "DBClusterIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.db_cluster_instance_class {
            properties.insert(
                "DBClusterInstanceClass".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.db_cluster_parameter_group_name {
            properties.insert(
                "DBClusterParameterGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.db_instance_parameter_group_name {
            properties.insert(
                "DBInstanceParameterGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.db_subnet_group_name {
            properties.insert(
                "DBSubnetGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.db_system_id {
            properties.insert(
                "DBSystemId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.database_insights_mode {
            properties.insert(
                "DatabaseInsightsMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.database_name {
            properties.insert(
                "DatabaseName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.delete_automated_backups {
            properties.insert(
                "DeleteAutomatedBackups".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.deletion_protection {
            properties.insert(
                "DeletionProtection".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.domain {
            properties.insert("Domain".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.domain_iam_role_name {
            properties.insert(
                "DomainIAMRoleName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enable_cloudwatch_logs_exports {
            properties.insert(
                "EnableCloudwatchLogsExports".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enable_global_write_forwarding {
            properties.insert(
                "EnableGlobalWriteForwarding".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enable_http_endpoint {
            properties.insert(
                "EnableHttpEndpoint".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enable_iam_database_authentication {
            properties.insert(
                "EnableIAMDatabaseAuthentication".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enable_local_write_forwarding {
            properties.insert(
                "EnableLocalWriteForwarding".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.engine {
            properties.insert("Engine".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.engine_lifecycle_support {
            properties.insert(
                "EngineLifecycleSupport".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.engine_mode {
            properties.insert(
                "EngineMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.engine_version {
            properties.insert(
                "EngineVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.global_cluster_identifier {
            properties.insert(
                "GlobalClusterIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.iops {
            properties.insert("Iops".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.kms_key_id {
            properties.insert(
                "KmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.manage_master_user_password {
            properties.insert(
                "ManageMasterUserPassword".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.master_user_authentication_type {
            properties.insert(
                "MasterUserAuthenticationType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.master_user_password {
            properties.insert(
                "MasterUserPassword".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.master_user_secret {
            properties.insert(
                "MasterUserSecret".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.master_username {
            properties.insert(
                "MasterUsername".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.monitoring_interval {
            properties.insert(
                "MonitoringInterval".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.monitoring_role_arn {
            properties.insert(
                "MonitoringRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.network_type {
            properties.insert(
                "NetworkType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.performance_insights_enabled {
            properties.insert(
                "PerformanceInsightsEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.performance_insights_kms_key_id {
            properties.insert(
                "PerformanceInsightsKmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.performance_insights_retention_period {
            properties.insert(
                "PerformanceInsightsRetentionPeriod".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.port {
            properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.preferred_backup_window {
            properties.insert(
                "PreferredBackupWindow".to_string(),
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
        if let Some(ref value) = self.replication_source_identifier {
            properties.insert(
                "ReplicationSourceIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.restore_to_time {
            properties.insert(
                "RestoreToTime".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.restore_type {
            properties.insert(
                "RestoreType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.scaling_configuration {
            properties.insert(
                "ScalingConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.serverless_v2_scaling_configuration {
            properties.insert(
                "ServerlessV2ScalingConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.snapshot_identifier {
            properties.insert(
                "SnapshotIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_db_cluster_identifier {
            properties.insert(
                "SourceDBClusterIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_db_cluster_resource_id {
            properties.insert(
                "SourceDbClusterResourceId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_region {
            properties.insert(
                "SourceRegion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.storage_encrypted {
            properties.insert(
                "StorageEncrypted".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.storage_type {
            properties.insert(
                "StorageType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.use_latest_restorable_time {
            properties.insert(
                "UseLatestRestorableTime".to_string(),
                crate::value::ToValue::to_value(value),
            );
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbclusterparametergroup.html
pub struct DBClusterParameterGroup_ {
    pub db_cluster_parameter_group_name: Option<crate::value::ExpString>,
    pub description: crate::value::ExpString,
    pub family: crate::value::ExpString,
    pub parameters: serde_json::Value,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_rds_DBClusterParameterGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::RDS::DBClusterParameterGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_rds_DBClusterParameterGroup as DBClusterParameterGroup;
impl crate::template::ToResource for DBClusterParameterGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RDS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DBClusterParameterGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.db_cluster_parameter_group_name {
            properties.insert(
                "DBClusterParameterGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Description".to_string(),
            crate::value::ToValue::to_value(&self.description),
        );
        properties.insert(
            "Family".to_string(),
            crate::value::ToValue::to_value(&self.family),
        );
        properties.insert(
            "Parameters".to_string(),
            crate::value::ToValue::to_value(&self.parameters),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html
pub struct DBInstance_ {
    pub additional_storage_volumes: Option<Vec<super::rds::dbinstance::AdditionalStorageVolume_>>,
    pub allocated_storage: Option<crate::value::ExpString>,
    pub allow_major_version_upgrade: Option<crate::value::ExpBool>,
    pub apply_immediately: Option<crate::value::ExpBool>,
    pub associated_roles: Option<Vec<super::rds::dbinstance::DBInstanceRole_>>,
    pub auto_minor_version_upgrade: Option<crate::value::ExpBool>,
    pub automatic_backup_replication_kms_key_id: Option<crate::value::ExpString>,
    pub automatic_backup_replication_region: Option<crate::value::ExpString>,
    pub automatic_backup_replication_retention_period: Option<i32>,
    pub availability_zone: Option<crate::value::ExpString>,
    pub backup_retention_period: Option<i32>,
    pub backup_target: Option<crate::value::ExpString>,
    pub ca_certificate_identifier: Option<crate::value::ExpString>,
    pub certificate_rotation_restart: Option<crate::value::ExpBool>,
    pub character_set_name: Option<crate::value::ExpString>,
    pub copy_tags_to_snapshot: Option<crate::value::ExpBool>,
    pub custom_iam_instance_profile: Option<crate::value::ExpString>,
    pub db_cluster_identifier: Option<crate::value::ExpString>,
    pub db_cluster_snapshot_identifier: Option<crate::value::ExpString>,
    pub db_instance_class: Option<crate::value::ExpString>,
    pub db_instance_identifier: Option<crate::value::ExpString>,
    pub db_name: Option<crate::value::ExpString>,
    pub db_parameter_group_name: Option<crate::value::ExpString>,
    pub db_security_groups: Option<Vec<crate::value::ExpString>>,
    pub db_snapshot_identifier: Option<crate::value::ExpString>,
    pub db_subnet_group_name: Option<crate::value::ExpString>,
    pub db_system_id: Option<crate::value::ExpString>,
    pub database_insights_mode: Option<crate::value::ExpString>,
    pub dedicated_log_volume: Option<crate::value::ExpBool>,
    pub delete_automated_backups: Option<crate::value::ExpBool>,
    pub deletion_protection: Option<crate::value::ExpBool>,
    pub domain: Option<crate::value::ExpString>,
    pub domain_auth_secret_arn: Option<crate::value::ExpString>,
    pub domain_dns_ips: Option<Vec<crate::value::ExpString>>,
    pub domain_fqdn: Option<crate::value::ExpString>,
    pub domain_iam_role_name: Option<crate::value::ExpString>,
    pub domain_ou: Option<crate::value::ExpString>,
    pub enable_cloudwatch_logs_exports: Option<Vec<crate::value::ExpString>>,
    pub enable_iam_database_authentication: Option<crate::value::ExpBool>,
    pub enable_performance_insights: Option<crate::value::ExpBool>,
    pub engine: Option<crate::value::ExpString>,
    pub engine_lifecycle_support: Option<crate::value::ExpString>,
    pub engine_version: Option<crate::value::ExpString>,
    pub iops: Option<i32>,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub license_model: Option<crate::value::ExpString>,
    pub manage_master_user_password: Option<crate::value::ExpBool>,
    pub master_user_authentication_type: Option<crate::value::ExpString>,
    pub master_user_password: Option<crate::value::ExpString>,
    pub master_user_secret: Option<super::rds::dbinstance::MasterUserSecret_>,
    pub master_username: Option<crate::value::ExpString>,
    pub max_allocated_storage: Option<i32>,
    pub monitoring_interval: Option<i32>,
    pub monitoring_role_arn: Option<crate::value::ExpString>,
    pub multi_az: Option<crate::value::ExpBool>,
    pub nchar_character_set_name: Option<crate::value::ExpString>,
    pub network_type: Option<crate::value::ExpString>,
    pub option_group_name: Option<crate::value::ExpString>,
    pub performance_insights_kms_key_id: Option<crate::value::ExpString>,
    pub performance_insights_retention_period: Option<i32>,
    pub port: Option<crate::value::ExpString>,
    pub preferred_backup_window: Option<crate::value::ExpString>,
    pub preferred_maintenance_window: Option<crate::value::ExpString>,
    pub processor_features: Option<Vec<super::rds::dbinstance::ProcessorFeature_>>,
    pub promotion_tier: Option<i32>,
    pub publicly_accessible: Option<crate::value::ExpBool>,
    pub replica_mode: Option<crate::value::ExpString>,
    pub restore_time: Option<crate::value::ExpString>,
    pub source_db_cluster_identifier: Option<crate::value::ExpString>,
    pub source_db_instance_automated_backups_arn: Option<crate::value::ExpString>,
    pub source_db_instance_identifier: Option<crate::value::ExpString>,
    pub source_dbi_resource_id: Option<crate::value::ExpString>,
    pub source_region: Option<crate::value::ExpString>,
    pub storage_encrypted: Option<crate::value::ExpBool>,
    pub storage_throughput: Option<i32>,
    pub storage_type: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub timezone: Option<crate::value::ExpString>,
    pub use_default_processor_features: Option<crate::value::ExpBool>,
    pub use_latest_restorable_time: Option<crate::value::ExpBool>,
    pub vpc_security_groups: Option<Vec<crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_rds_DBInstance {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::RDS::DBInstance" $($field
        $value)*)
    };
}
pub use crate::__aws_rds_DBInstance as DBInstance;
impl crate::template::ToResource for DBInstance_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RDS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DBInstance"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.additional_storage_volumes {
            properties.insert(
                "AdditionalStorageVolumes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
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
        if let Some(ref value) = self.apply_immediately {
            properties.insert(
                "ApplyImmediately".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.associated_roles {
            properties.insert(
                "AssociatedRoles".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.auto_minor_version_upgrade {
            properties.insert(
                "AutoMinorVersionUpgrade".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.automatic_backup_replication_kms_key_id {
            properties.insert(
                "AutomaticBackupReplicationKmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.automatic_backup_replication_region {
            properties.insert(
                "AutomaticBackupReplicationRegion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.automatic_backup_replication_retention_period {
            properties.insert(
                "AutomaticBackupReplicationRetentionPeriod".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.availability_zone {
            properties.insert(
                "AvailabilityZone".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.backup_retention_period {
            properties.insert(
                "BackupRetentionPeriod".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.backup_target {
            properties.insert(
                "BackupTarget".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ca_certificate_identifier {
            properties.insert(
                "CACertificateIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.certificate_rotation_restart {
            properties.insert(
                "CertificateRotationRestart".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.character_set_name {
            properties.insert(
                "CharacterSetName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.copy_tags_to_snapshot {
            properties.insert(
                "CopyTagsToSnapshot".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.custom_iam_instance_profile {
            properties.insert(
                "CustomIAMInstanceProfile".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.db_cluster_identifier {
            properties.insert(
                "DBClusterIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.db_cluster_snapshot_identifier {
            properties.insert(
                "DBClusterSnapshotIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.db_instance_class {
            properties.insert(
                "DBInstanceClass".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.db_instance_identifier {
            properties.insert(
                "DBInstanceIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.db_name {
            properties.insert("DBName".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.db_parameter_group_name {
            properties.insert(
                "DBParameterGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.db_security_groups {
            properties.insert(
                "DBSecurityGroups".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.db_snapshot_identifier {
            properties.insert(
                "DBSnapshotIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.db_subnet_group_name {
            properties.insert(
                "DBSubnetGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.db_system_id {
            properties.insert(
                "DBSystemId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.database_insights_mode {
            properties.insert(
                "DatabaseInsightsMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.dedicated_log_volume {
            properties.insert(
                "DedicatedLogVolume".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.delete_automated_backups {
            properties.insert(
                "DeleteAutomatedBackups".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.deletion_protection {
            properties.insert(
                "DeletionProtection".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.domain {
            properties.insert("Domain".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.domain_auth_secret_arn {
            properties.insert(
                "DomainAuthSecretArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.domain_dns_ips {
            properties.insert(
                "DomainDnsIps".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.domain_fqdn {
            properties.insert(
                "DomainFqdn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.domain_iam_role_name {
            properties.insert(
                "DomainIAMRoleName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.domain_ou {
            properties.insert(
                "DomainOu".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enable_cloudwatch_logs_exports {
            properties.insert(
                "EnableCloudwatchLogsExports".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enable_iam_database_authentication {
            properties.insert(
                "EnableIAMDatabaseAuthentication".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enable_performance_insights {
            properties.insert(
                "EnablePerformanceInsights".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.engine {
            properties.insert("Engine".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.engine_lifecycle_support {
            properties.insert(
                "EngineLifecycleSupport".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.engine_version {
            properties.insert(
                "EngineVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.iops {
            properties.insert("Iops".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.kms_key_id {
            properties.insert(
                "KmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.license_model {
            properties.insert(
                "LicenseModel".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.manage_master_user_password {
            properties.insert(
                "ManageMasterUserPassword".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.master_user_authentication_type {
            properties.insert(
                "MasterUserAuthenticationType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.master_user_password {
            properties.insert(
                "MasterUserPassword".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.master_user_secret {
            properties.insert(
                "MasterUserSecret".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.master_username {
            properties.insert(
                "MasterUsername".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.max_allocated_storage {
            properties.insert(
                "MaxAllocatedStorage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.monitoring_interval {
            properties.insert(
                "MonitoringInterval".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.monitoring_role_arn {
            properties.insert(
                "MonitoringRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.multi_az {
            properties.insert(
                "MultiAZ".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.nchar_character_set_name {
            properties.insert(
                "NcharCharacterSetName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.network_type {
            properties.insert(
                "NetworkType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.option_group_name {
            properties.insert(
                "OptionGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.performance_insights_kms_key_id {
            properties.insert(
                "PerformanceInsightsKMSKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.performance_insights_retention_period {
            properties.insert(
                "PerformanceInsightsRetentionPeriod".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.port {
            properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.preferred_backup_window {
            properties.insert(
                "PreferredBackupWindow".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.preferred_maintenance_window {
            properties.insert(
                "PreferredMaintenanceWindow".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.processor_features {
            properties.insert(
                "ProcessorFeatures".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.promotion_tier {
            properties.insert(
                "PromotionTier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.publicly_accessible {
            properties.insert(
                "PubliclyAccessible".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.replica_mode {
            properties.insert(
                "ReplicaMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.restore_time {
            properties.insert(
                "RestoreTime".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_db_cluster_identifier {
            properties.insert(
                "SourceDBClusterIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_db_instance_automated_backups_arn {
            properties.insert(
                "SourceDBInstanceAutomatedBackupsArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_db_instance_identifier {
            properties.insert(
                "SourceDBInstanceIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_dbi_resource_id {
            properties.insert(
                "SourceDbiResourceId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_region {
            properties.insert(
                "SourceRegion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.storage_encrypted {
            properties.insert(
                "StorageEncrypted".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.storage_throughput {
            properties.insert(
                "StorageThroughput".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.storage_type {
            properties.insert(
                "StorageType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.timezone {
            properties.insert(
                "Timezone".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.use_default_processor_features {
            properties.insert(
                "UseDefaultProcessorFeatures".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.use_latest_restorable_time {
            properties.insert(
                "UseLatestRestorableTime".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.vpc_security_groups {
            properties.insert(
                "VPCSecurityGroups".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbparametergroup.html
pub struct DBParameterGroup_ {
    pub db_parameter_group_name: Option<crate::value::ExpString>,
    pub description: crate::value::ExpString,
    pub family: crate::value::ExpString,
    pub parameters: Option<serde_json::Value>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_rds_DBParameterGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::RDS::DBParameterGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_rds_DBParameterGroup as DBParameterGroup;
impl crate::template::ToResource for DBParameterGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RDS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DBParameterGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.db_parameter_group_name {
            properties.insert(
                "DBParameterGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Description".to_string(),
            crate::value::ToValue::to_value(&self.description),
        );
        properties.insert(
            "Family".to_string(),
            crate::value::ToValue::to_value(&self.family),
        );
        if let Some(ref value) = self.parameters {
            properties.insert(
                "Parameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxy.html
pub struct DBProxy_ {
    pub auth: Option<Vec<super::rds::dbproxy::AuthFormat_>>,
    pub db_proxy_name: crate::value::ExpString,
    pub debug_logging: Option<crate::value::ExpBool>,
    pub default_auth_scheme: Option<crate::value::ExpString>,
    pub endpoint_network_type: Option<crate::value::ExpString>,
    pub engine_family: crate::value::ExpString,
    pub idle_client_timeout: Option<i32>,
    pub require_tls: Option<crate::value::ExpBool>,
    pub role_arn: crate::value::ExpString,
    pub tags: Option<Vec<super::rds::dbproxy::TagFormat_>>,
    pub target_connection_network_type: Option<crate::value::ExpString>,
    pub vpc_security_group_ids: Option<Vec<crate::value::ExpString>>,
    pub vpc_subnet_ids: Vec<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_rds_DBProxy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::RDS::DBProxy" $($field
        $value)*)
    };
}
pub use crate::__aws_rds_DBProxy as DBProxy;
impl crate::template::ToResource for DBProxy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RDS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DBProxy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.auth {
            properties.insert("Auth".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "DBProxyName".to_string(),
            crate::value::ToValue::to_value(&self.db_proxy_name),
        );
        if let Some(ref value) = self.debug_logging {
            properties.insert(
                "DebugLogging".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.default_auth_scheme {
            properties.insert(
                "DefaultAuthScheme".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.endpoint_network_type {
            properties.insert(
                "EndpointNetworkType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "EngineFamily".to_string(),
            crate::value::ToValue::to_value(&self.engine_family),
        );
        if let Some(ref value) = self.idle_client_timeout {
            properties.insert(
                "IdleClientTimeout".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.require_tls {
            properties.insert(
                "RequireTLS".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.target_connection_network_type {
            properties.insert(
                "TargetConnectionNetworkType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.vpc_security_group_ids {
            properties.insert(
                "VpcSecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "VpcSubnetIds".to_string(),
            crate::value::ToValue::to_value(&self.vpc_subnet_ids),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxyendpoint.html
pub struct DBProxyEndpoint_ {
    pub db_proxy_endpoint_name: crate::value::ExpString,
    pub db_proxy_name: crate::value::ExpString,
    pub endpoint_network_type: Option<crate::value::ExpString>,
    pub tags: Option<Vec<super::rds::dbproxyendpoint::TagFormat_>>,
    pub target_role: Option<crate::value::ExpString>,
    pub vpc_security_group_ids: Option<Vec<crate::value::ExpString>>,
    pub vpc_subnet_ids: Vec<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_rds_DBProxyEndpoint {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::RDS::DBProxyEndpoint"
        $($field $value)*)
    };
}
pub use crate::__aws_rds_DBProxyEndpoint as DBProxyEndpoint;
impl crate::template::ToResource for DBProxyEndpoint_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RDS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DBProxyEndpoint"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DBProxyEndpointName".to_string(),
            crate::value::ToValue::to_value(&self.db_proxy_endpoint_name),
        );
        properties.insert(
            "DBProxyName".to_string(),
            crate::value::ToValue::to_value(&self.db_proxy_name),
        );
        if let Some(ref value) = self.endpoint_network_type {
            properties.insert(
                "EndpointNetworkType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.target_role {
            properties.insert(
                "TargetRole".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.vpc_security_group_ids {
            properties.insert(
                "VpcSecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "VpcSubnetIds".to_string(),
            crate::value::ToValue::to_value(&self.vpc_subnet_ids),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxytargetgroup.html
pub struct DBProxyTargetGroup_ {
    pub connection_pool_configuration_info:
        Option<super::rds::dbproxytargetgroup::ConnectionPoolConfigurationInfoFormat_>,
    pub db_cluster_identifiers: Option<Vec<crate::value::ExpString>>,
    pub db_instance_identifiers: Option<Vec<crate::value::ExpString>>,
    pub db_proxy_name: crate::value::ExpString,
    pub target_group_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_rds_DBProxyTargetGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::RDS::DBProxyTargetGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_rds_DBProxyTargetGroup as DBProxyTargetGroup;
impl crate::template::ToResource for DBProxyTargetGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RDS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DBProxyTargetGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.connection_pool_configuration_info {
            properties.insert(
                "ConnectionPoolConfigurationInfo".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.db_cluster_identifiers {
            properties.insert(
                "DBClusterIdentifiers".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.db_instance_identifiers {
            properties.insert(
                "DBInstanceIdentifiers".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DBProxyName".to_string(),
            crate::value::ToValue::to_value(&self.db_proxy_name),
        );
        properties.insert(
            "TargetGroupName".to_string(),
            crate::value::ToValue::to_value(&self.target_group_name),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-security-group.html
pub struct DBSecurityGroup_ {
    pub db_security_group_ingress: Vec<super::rds::dbsecuritygroup::Ingress_>,
    pub ec2_vpc_id: Option<crate::value::ExpString>,
    pub group_description: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_rds_DBSecurityGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::RDS::DBSecurityGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_rds_DBSecurityGroup as DBSecurityGroup;
impl crate::template::ToResource for DBSecurityGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RDS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DBSecurityGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DBSecurityGroupIngress".to_string(),
            crate::value::ToValue::to_value(&self.db_security_group_ingress),
        );
        if let Some(ref value) = self.ec2_vpc_id {
            properties.insert(
                "EC2VpcId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "GroupDescription".to_string(),
            crate::value::ToValue::to_value(&self.group_description),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-security-group-ingress.html
pub struct DBSecurityGroupIngress_ {
    pub cidrip: Option<crate::value::ExpString>,
    pub db_security_group_name: crate::value::ExpString,
    pub ec2_security_group_id: Option<crate::value::ExpString>,
    pub ec2_security_group_name: Option<crate::value::ExpString>,
    pub ec2_security_group_owner_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_rds_DBSecurityGroupIngress {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::RDS::DBSecurityGroupIngress"
        $($field $value)*)
    };
}
pub use crate::__aws_rds_DBSecurityGroupIngress as DBSecurityGroupIngress;
impl crate::template::ToResource for DBSecurityGroupIngress_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RDS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DBSecurityGroupIngress"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.cidrip {
            properties.insert("CIDRIP".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "DBSecurityGroupName".to_string(),
            crate::value::ToValue::to_value(&self.db_security_group_name),
        );
        if let Some(ref value) = self.ec2_security_group_id {
            properties.insert(
                "EC2SecurityGroupId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ec2_security_group_name {
            properties.insert(
                "EC2SecurityGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ec2_security_group_owner_id {
            properties.insert(
                "EC2SecurityGroupOwnerId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbshardgroup.html
pub struct DBShardGroup_ {
    pub compute_redundancy: Option<i32>,
    pub db_cluster_identifier: crate::value::ExpString,
    pub db_shard_group_identifier: Option<crate::value::ExpString>,
    pub max_acu: f64,
    pub min_acu: Option<f64>,
    pub publicly_accessible: Option<crate::value::ExpBool>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_rds_DBShardGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::RDS::DBShardGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_rds_DBShardGroup as DBShardGroup;
impl crate::template::ToResource for DBShardGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RDS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DBShardGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.compute_redundancy {
            properties.insert(
                "ComputeRedundancy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DBClusterIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.db_cluster_identifier),
        );
        if let Some(ref value) = self.db_shard_group_identifier {
            properties.insert(
                "DBShardGroupIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "MaxACU".to_string(),
            crate::value::ToValue::to_value(&self.max_acu),
        );
        if let Some(ref value) = self.min_acu {
            properties.insert("MinACU".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.publicly_accessible {
            properties.insert(
                "PubliclyAccessible".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbsubnetgroup.html
pub struct DBSubnetGroup_ {
    pub db_subnet_group_description: crate::value::ExpString,
    pub db_subnet_group_name: Option<crate::value::ExpString>,
    pub subnet_ids: Vec<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_rds_DBSubnetGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::RDS::DBSubnetGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_rds_DBSubnetGroup as DBSubnetGroup;
impl crate::template::ToResource for DBSubnetGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RDS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DBSubnetGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DBSubnetGroupDescription".to_string(),
            crate::value::ToValue::to_value(&self.db_subnet_group_description),
        );
        if let Some(ref value) = self.db_subnet_group_name {
            properties.insert(
                "DBSubnetGroupName".to_string(),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-eventsubscription.html
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
macro_rules! __aws_rds_EventSubscription {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::RDS::EventSubscription"
        $($field $value)*)
    };
}
pub use crate::__aws_rds_EventSubscription as EventSubscription;
impl crate::template::ToResource for EventSubscription_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RDS"),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-globalcluster.html
pub struct GlobalCluster_ {
    pub deletion_protection: Option<crate::value::ExpBool>,
    pub engine: Option<crate::value::ExpString>,
    pub engine_lifecycle_support: Option<crate::value::ExpString>,
    pub engine_version: Option<crate::value::ExpString>,
    pub global_cluster_identifier: Option<crate::value::ExpString>,
    pub source_db_cluster_identifier: Option<crate::value::ExpString>,
    pub storage_encrypted: Option<crate::value::ExpBool>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_rds_GlobalCluster {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::RDS::GlobalCluster"
        $($field $value)*)
    };
}
pub use crate::__aws_rds_GlobalCluster as GlobalCluster;
impl crate::template::ToResource for GlobalCluster_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RDS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("GlobalCluster"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.deletion_protection {
            properties.insert(
                "DeletionProtection".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.engine {
            properties.insert("Engine".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.engine_lifecycle_support {
            properties.insert(
                "EngineLifecycleSupport".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.engine_version {
            properties.insert(
                "EngineVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.global_cluster_identifier {
            properties.insert(
                "GlobalClusterIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_db_cluster_identifier {
            properties.insert(
                "SourceDBClusterIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.storage_encrypted {
            properties.insert(
                "StorageEncrypted".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-integration.html
pub struct Integration_ {
    pub additional_encryption_context:
        Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub data_filter: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub integration_name: Option<crate::value::ExpString>,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub source_arn: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub target_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_rds_Integration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::RDS::Integration"
        $($field $value)*)
    };
}
pub use crate::__aws_rds_Integration as Integration;
impl crate::template::ToResource for Integration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RDS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Integration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.additional_encryption_context {
            properties.insert(
                "AdditionalEncryptionContext".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.data_filter {
            properties.insert(
                "DataFilter".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.integration_name {
            properties.insert(
                "IntegrationName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_id {
            properties.insert(
                "KMSKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SourceArn".to_string(),
            crate::value::ToValue::to_value(&self.source_arn),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TargetArn".to_string(),
            crate::value::ToValue::to_value(&self.target_arn),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-optiongroup.html
pub struct OptionGroup_ {
    pub engine_name: crate::value::ExpString,
    pub major_engine_version: crate::value::ExpString,
    pub option_configurations: Option<Vec<super::rds::optiongroup::OptionConfiguration_>>,
    pub option_group_description: crate::value::ExpString,
    pub option_group_name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_rds_OptionGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::RDS::OptionGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_rds_OptionGroup as OptionGroup;
impl crate::template::ToResource for OptionGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RDS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("OptionGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "EngineName".to_string(),
            crate::value::ToValue::to_value(&self.engine_name),
        );
        properties.insert(
            "MajorEngineVersion".to_string(),
            crate::value::ToValue::to_value(&self.major_engine_version),
        );
        if let Some(ref value) = self.option_configurations {
            properties.insert(
                "OptionConfigurations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "OptionGroupDescription".to_string(),
            crate::value::ToValue::to_value(&self.option_group_description),
        );
        if let Some(ref value) = self.option_group_name {
            properties.insert(
                "OptionGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
