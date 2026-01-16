pub mod datalake {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securitylake-datalake-encryptionconfiguration.html
    pub struct EncryptionConfiguration_ {
        pub kms_key_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securitylake_DataLake_EncryptionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SecurityLake::DataLake.EncryptionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securitylake_DataLake_EncryptionConfiguration as EncryptionConfiguration;
    impl crate::value::ToValue for EncryptionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securitylake-datalake-expiration.html
    pub struct Expiration_ {
        pub days: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securitylake_DataLake_Expiration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SecurityLake::DataLake.Expiration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securitylake_DataLake_Expiration as Expiration;
    impl crate::value::ToValue for Expiration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.days {
                properties.insert("Days".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securitylake-datalake-lifecycleconfiguration.html
    pub struct LifecycleConfiguration_ {
        pub expiration: Option<Box<Expiration_>>,
        pub transitions: Option<Vec<Transitions_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securitylake_DataLake_LifecycleConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SecurityLake::DataLake.LifecycleConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securitylake_DataLake_LifecycleConfiguration as LifecycleConfiguration;
    impl crate::value::ToValue for LifecycleConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.expiration {
                properties.insert(
                    "Expiration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.transitions {
                properties.insert(
                    "Transitions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securitylake-datalake-replicationconfiguration.html
    pub struct ReplicationConfiguration_ {
        pub regions: Option<Vec<crate::value::ExpString>>,
        pub role_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securitylake_DataLake_ReplicationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SecurityLake::DataLake.ReplicationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securitylake_DataLake_ReplicationConfiguration as ReplicationConfiguration;
    impl crate::value::ToValue for ReplicationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.regions {
                properties.insert(
                    "Regions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securitylake-datalake-transitions.html
    pub struct Transitions_ {
        pub days: Option<i64>,
        pub storage_class: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securitylake_DataLake_Transitions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SecurityLake::DataLake.Transitions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securitylake_DataLake_Transitions as Transitions;
    impl crate::value::ToValue for Transitions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.days {
                properties.insert("Days".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.storage_class {
                properties.insert(
                    "StorageClass".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod subscriber {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securitylake-subscriber-awslogsource.html
    pub struct AwsLogSource_ {
        pub source_name: Option<crate::value::ExpString>,
        pub source_version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securitylake_Subscriber_AwsLogSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SecurityLake::Subscriber.AwsLogSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securitylake_Subscriber_AwsLogSource as AwsLogSource;
    impl crate::value::ToValue for AwsLogSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.source_name {
                properties.insert(
                    "SourceName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_version {
                properties.insert(
                    "SourceVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securitylake-subscriber-customlogsource.html
    pub struct CustomLogSource_ {
        pub source_name: Option<crate::value::ExpString>,
        pub source_version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securitylake_Subscriber_CustomLogSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SecurityLake::Subscriber.CustomLogSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securitylake_Subscriber_CustomLogSource as CustomLogSource;
    impl crate::value::ToValue for CustomLogSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.source_name {
                properties.insert(
                    "SourceName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_version {
                properties.insert(
                    "SourceVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securitylake-subscriber-source.html
    pub struct Source_ {
        pub aws_log_source: Option<Box<AwsLogSource_>>,
        pub custom_log_source: Option<Box<CustomLogSource_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securitylake_Subscriber_Source {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SecurityLake::Subscriber.Source"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securitylake_Subscriber_Source as Source;
    impl crate::value::ToValue for Source_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.aws_log_source {
                properties.insert(
                    "AwsLogSource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_log_source {
                properties.insert(
                    "CustomLogSource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securitylake-subscriber-subscriberidentity.html
    pub struct SubscriberIdentity_ {
        pub external_id: crate::value::ExpString,
        pub principal: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securitylake_Subscriber_SubscriberIdentity {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SecurityLake::Subscriber.SubscriberIdentity"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securitylake_Subscriber_SubscriberIdentity as SubscriberIdentity;
    impl crate::value::ToValue for SubscriberIdentity_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ExternalId".to_string(),
                crate::value::ToValue::to_value(&self.external_id),
            );
            properties.insert(
                "Principal".to_string(),
                crate::value::ToValue::to_value(&self.principal),
            );
            properties.into()
        }
    }
}
pub mod subscribernotification {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securitylake-subscribernotification-httpsnotificationconfiguration.html
    pub struct HttpsNotificationConfiguration_ {
        pub authorization_api_key_name: Option<crate::value::ExpString>,
        pub authorization_api_key_value: Option<crate::value::ExpString>,
        pub endpoint: crate::value::ExpString,
        pub http_method: Option<crate::value::ExpString>,
        pub target_role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securitylake_SubscriberNotification_HttpsNotificationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SecurityLake::SubscriberNotification.HttpsNotificationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securitylake_SubscriberNotification_HttpsNotificationConfiguration as HttpsNotificationConfiguration;
    impl crate::value::ToValue for HttpsNotificationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.authorization_api_key_name {
                properties.insert(
                    "AuthorizationApiKeyName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.authorization_api_key_value {
                properties.insert(
                    "AuthorizationApiKeyValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Endpoint".to_string(),
                crate::value::ToValue::to_value(&self.endpoint),
            );
            if let Some(ref value) = self.http_method {
                properties.insert(
                    "HttpMethod".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TargetRoleArn".to_string(),
                crate::value::ToValue::to_value(&self.target_role_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securitylake-subscribernotification-notificationconfiguration.html
    pub struct NotificationConfiguration_ {
        pub https_notification_configuration: Option<Box<HttpsNotificationConfiguration_>>,
        pub sqs_notification_configuration: Option<serde_json::Value>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_securitylake_SubscriberNotification_NotificationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SecurityLake::SubscriberNotification.NotificationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_securitylake_SubscriberNotification_NotificationConfiguration as NotificationConfiguration;
    impl crate::value::ToValue for NotificationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.https_notification_configuration {
                properties.insert(
                    "HttpsNotificationConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sqs_notification_configuration {
                properties.insert(
                    "SqsNotificationConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-securitylake-awslogsource.html
pub struct AwsLogSource_ {
    pub accounts: Option<Vec<crate::value::ExpString>>,
    pub data_lake_arn: crate::value::ExpString,
    pub source_name: crate::value::ExpString,
    pub source_version: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_securitylake_AwsLogSource {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SecurityLake::AwsLogSource"
        $($field $value)*)
    };
}
pub use crate::__aws_securitylake_AwsLogSource as AwsLogSource;
impl crate::template::ToResource for AwsLogSource_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SecurityLake"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AwsLogSource"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.accounts {
            properties.insert(
                "Accounts".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DataLakeArn".to_string(),
            crate::value::ToValue::to_value(&self.data_lake_arn),
        );
        properties.insert(
            "SourceName".to_string(),
            crate::value::ToValue::to_value(&self.source_name),
        );
        properties.insert(
            "SourceVersion".to_string(),
            crate::value::ToValue::to_value(&self.source_version),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-securitylake-datalake.html
pub struct DataLake_ {
    pub encryption_configuration: Option<super::securitylake::datalake::EncryptionConfiguration_>,
    pub lifecycle_configuration: Option<super::securitylake::datalake::LifecycleConfiguration_>,
    pub meta_store_manager_role_arn: Option<crate::value::ExpString>,
    pub replication_configuration: Option<super::securitylake::datalake::ReplicationConfiguration_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_securitylake_DataLake {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SecurityLake::DataLake"
        $($field $value)*)
    };
}
pub use crate::__aws_securitylake_DataLake as DataLake;
impl crate::template::ToResource for DataLake_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SecurityLake"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DataLake"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.encryption_configuration {
            properties.insert(
                "EncryptionConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.lifecycle_configuration {
            properties.insert(
                "LifecycleConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.meta_store_manager_role_arn {
            properties.insert(
                "MetaStoreManagerRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.replication_configuration {
            properties.insert(
                "ReplicationConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-securitylake-subscriber.html
pub struct Subscriber_ {
    pub access_types: Vec<crate::value::ExpString>,
    pub data_lake_arn: crate::value::ExpString,
    pub sources: Vec<super::securitylake::subscriber::Source_>,
    pub subscriber_description: Option<crate::value::ExpString>,
    pub subscriber_identity: super::securitylake::subscriber::SubscriberIdentity_,
    pub subscriber_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_securitylake_Subscriber {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SecurityLake::Subscriber"
        $($field $value)*)
    };
}
pub use crate::__aws_securitylake_Subscriber as Subscriber;
impl crate::template::ToResource for Subscriber_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SecurityLake"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Subscriber"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AccessTypes".to_string(),
            crate::value::ToValue::to_value(&self.access_types),
        );
        properties.insert(
            "DataLakeArn".to_string(),
            crate::value::ToValue::to_value(&self.data_lake_arn),
        );
        properties.insert(
            "Sources".to_string(),
            crate::value::ToValue::to_value(&self.sources),
        );
        if let Some(ref value) = self.subscriber_description {
            properties.insert(
                "SubscriberDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SubscriberIdentity".to_string(),
            crate::value::ToValue::to_value(&self.subscriber_identity),
        );
        properties.insert(
            "SubscriberName".to_string(),
            crate::value::ToValue::to_value(&self.subscriber_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-securitylake-subscribernotification.html
pub struct SubscriberNotification_ {
    pub notification_configuration:
        super::securitylake::subscribernotification::NotificationConfiguration_,
    pub subscriber_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_securitylake_SubscriberNotification {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SecurityLake::SubscriberNotification"
        $($field $value)*)
    };
}
pub use crate::__aws_securitylake_SubscriberNotification as SubscriberNotification;
impl crate::template::ToResource for SubscriberNotification_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SecurityLake"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SubscriberNotification"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "NotificationConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.notification_configuration),
        );
        properties.insert(
            "SubscriberArn".to_string(),
            crate::value::ToValue::to_value(&self.subscriber_arn),
        );
        properties
    }
}
