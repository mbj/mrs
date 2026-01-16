pub mod alias {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-alias-aliasroutingconfiguration.html
    pub struct AliasRoutingConfiguration_ {
        pub additional_version_weights: Option<Vec<VersionWeight_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_Alias_AliasRoutingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::Alias.AliasRoutingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_Alias_AliasRoutingConfiguration as AliasRoutingConfiguration;
    impl crate::value::ToValue for AliasRoutingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.additional_version_weights {
                properties.insert(
                    "AdditionalVersionWeights".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-alias-provisionedconcurrencyconfiguration.html
    pub struct ProvisionedConcurrencyConfiguration_ {
        pub provisioned_concurrent_executions: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_Alias_ProvisionedConcurrencyConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::Alias.ProvisionedConcurrencyConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_Alias_ProvisionedConcurrencyConfiguration as ProvisionedConcurrencyConfiguration;
    impl crate::value::ToValue for ProvisionedConcurrencyConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ProvisionedConcurrentExecutions".to_string(),
                crate::value::ToValue::to_value(&self.provisioned_concurrent_executions),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-alias-versionweight.html
    pub struct VersionWeight_ {
        pub function_version: crate::value::ExpString,
        pub function_weight: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_Alias_VersionWeight {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::Alias.VersionWeight"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_Alias_VersionWeight as VersionWeight;
    impl crate::value::ToValue for VersionWeight_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FunctionVersion".to_string(),
                crate::value::ToValue::to_value(&self.function_version),
            );
            properties.insert(
                "FunctionWeight".to_string(),
                crate::value::ToValue::to_value(&self.function_weight),
            );
            properties.into()
        }
    }
}
pub mod codesigningconfig {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-codesigningconfig-allowedpublishers.html
    pub struct AllowedPublishers_ {
        pub signing_profile_version_arns: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_CodeSigningConfig_AllowedPublishers {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::CodeSigningConfig.AllowedPublishers"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_CodeSigningConfig_AllowedPublishers as AllowedPublishers;
    impl crate::value::ToValue for AllowedPublishers_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SigningProfileVersionArns".to_string(),
                crate::value::ToValue::to_value(&self.signing_profile_version_arns),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-codesigningconfig-codesigningpolicies.html
    pub struct CodeSigningPolicies_ {
        pub untrusted_artifact_on_deployment: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_CodeSigningConfig_CodeSigningPolicies {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::CodeSigningConfig.CodeSigningPolicies"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_CodeSigningConfig_CodeSigningPolicies as CodeSigningPolicies;
    impl crate::value::ToValue for CodeSigningPolicies_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "UntrustedArtifactOnDeployment".to_string(),
                crate::value::ToValue::to_value(&self.untrusted_artifact_on_deployment),
            );
            properties.into()
        }
    }
}
pub mod eventinvokeconfig {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventinvokeconfig-destinationconfig.html
    pub struct DestinationConfig_ {
        pub on_failure: Option<Box<OnFailure_>>,
        pub on_success: Option<Box<OnSuccess_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_EventInvokeConfig_DestinationConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::EventInvokeConfig.DestinationConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_EventInvokeConfig_DestinationConfig as DestinationConfig;
    impl crate::value::ToValue for DestinationConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.on_failure {
                properties.insert(
                    "OnFailure".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.on_success {
                properties.insert(
                    "OnSuccess".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventinvokeconfig-onfailure.html
    pub struct OnFailure_ {
        pub destination: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_EventInvokeConfig_OnFailure {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::EventInvokeConfig.OnFailure"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_EventInvokeConfig_OnFailure as OnFailure;
    impl crate::value::ToValue for OnFailure_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Destination".to_string(),
                crate::value::ToValue::to_value(&self.destination),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventinvokeconfig-onsuccess.html
    pub struct OnSuccess_ {
        pub destination: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_EventInvokeConfig_OnSuccess {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::EventInvokeConfig.OnSuccess"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_EventInvokeConfig_OnSuccess as OnSuccess;
    impl crate::value::ToValue for OnSuccess_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Destination".to_string(),
                crate::value::ToValue::to_value(&self.destination),
            );
            properties.into()
        }
    }
}
pub mod eventsourcemapping {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-amazonmanagedkafkaeventsourceconfig.html
    pub struct AmazonManagedKafkaEventSourceConfig_ {
        pub consumer_group_id: Option<crate::value::ExpString>,
        pub schema_registry_config: Option<Box<SchemaRegistryConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_EventSourceMapping_AmazonManagedKafkaEventSourceConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::EventSourceMapping.AmazonManagedKafkaEventSourceConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_EventSourceMapping_AmazonManagedKafkaEventSourceConfig as AmazonManagedKafkaEventSourceConfig;
    impl crate::value::ToValue for AmazonManagedKafkaEventSourceConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.consumer_group_id {
                properties.insert(
                    "ConsumerGroupId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.schema_registry_config {
                properties.insert(
                    "SchemaRegistryConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-destinationconfig.html
    pub struct DestinationConfig_ {
        pub on_failure: Option<Box<OnFailure_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_EventSourceMapping_DestinationConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::EventSourceMapping.DestinationConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_EventSourceMapping_DestinationConfig as DestinationConfig;
    impl crate::value::ToValue for DestinationConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.on_failure {
                properties.insert(
                    "OnFailure".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-documentdbeventsourceconfig.html
    pub struct DocumentDBEventSourceConfig_ {
        pub collection_name: Option<crate::value::ExpString>,
        pub database_name: Option<crate::value::ExpString>,
        pub full_document: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_EventSourceMapping_DocumentDBEventSourceConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::EventSourceMapping.DocumentDBEventSourceConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_EventSourceMapping_DocumentDBEventSourceConfig as DocumentDBEventSourceConfig;
    impl crate::value::ToValue for DocumentDBEventSourceConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.collection_name {
                properties.insert(
                    "CollectionName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.database_name {
                properties.insert(
                    "DatabaseName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.full_document {
                properties.insert(
                    "FullDocument".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-endpoints.html
    pub struct Endpoints_ {
        pub kafka_bootstrap_servers: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_EventSourceMapping_Endpoints {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::EventSourceMapping.Endpoints"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_EventSourceMapping_Endpoints as Endpoints;
    impl crate::value::ToValue for Endpoints_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kafka_bootstrap_servers {
                properties.insert(
                    "KafkaBootstrapServers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-filter.html
    pub struct Filter_ {
        pub pattern: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_EventSourceMapping_Filter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::EventSourceMapping.Filter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_EventSourceMapping_Filter as Filter;
    impl crate::value::ToValue for Filter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.pattern {
                properties.insert(
                    "Pattern".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-filtercriteria.html
    pub struct FilterCriteria_ {
        pub filters: Option<Vec<Filter_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_EventSourceMapping_FilterCriteria {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::EventSourceMapping.FilterCriteria"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_EventSourceMapping_FilterCriteria as FilterCriteria;
    impl crate::value::ToValue for FilterCriteria_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.filters {
                properties.insert(
                    "Filters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-metricsconfig.html
    pub struct MetricsConfig_ {
        pub metrics: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_EventSourceMapping_MetricsConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::EventSourceMapping.MetricsConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_EventSourceMapping_MetricsConfig as MetricsConfig;
    impl crate::value::ToValue for MetricsConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.metrics {
                properties.insert(
                    "Metrics".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-onfailure.html
    pub struct OnFailure_ {
        pub destination: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_EventSourceMapping_OnFailure {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::EventSourceMapping.OnFailure"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_EventSourceMapping_OnFailure as OnFailure;
    impl crate::value::ToValue for OnFailure_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.destination {
                properties.insert(
                    "Destination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-provisionedpollerconfig.html
    pub struct ProvisionedPollerConfig_ {
        pub maximum_pollers: Option<i64>,
        pub minimum_pollers: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_EventSourceMapping_ProvisionedPollerConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::EventSourceMapping.ProvisionedPollerConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_EventSourceMapping_ProvisionedPollerConfig as ProvisionedPollerConfig;
    impl crate::value::ToValue for ProvisionedPollerConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.maximum_pollers {
                properties.insert(
                    "MaximumPollers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.minimum_pollers {
                properties.insert(
                    "MinimumPollers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-scalingconfig.html
    pub struct ScalingConfig_ {
        pub maximum_concurrency: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_EventSourceMapping_ScalingConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::EventSourceMapping.ScalingConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_EventSourceMapping_ScalingConfig as ScalingConfig;
    impl crate::value::ToValue for ScalingConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.maximum_concurrency {
                properties.insert(
                    "MaximumConcurrency".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-schemaregistryaccessconfig.html
    pub struct SchemaRegistryAccessConfig_ {
        pub r#type: Option<crate::value::ExpString>,
        pub uri: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_EventSourceMapping_SchemaRegistryAccessConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::EventSourceMapping.SchemaRegistryAccessConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_EventSourceMapping_SchemaRegistryAccessConfig as SchemaRegistryAccessConfig;
    impl crate::value::ToValue for SchemaRegistryAccessConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.uri {
                properties.insert("URI".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-schemaregistryconfig.html
    pub struct SchemaRegistryConfig_ {
        pub access_configs: Option<Vec<SchemaRegistryAccessConfig_>>,
        pub event_record_format: Option<crate::value::ExpString>,
        pub schema_registry_uri: Option<crate::value::ExpString>,
        pub schema_validation_configs: Option<Vec<SchemaValidationConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_EventSourceMapping_SchemaRegistryConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::EventSourceMapping.SchemaRegistryConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_EventSourceMapping_SchemaRegistryConfig as SchemaRegistryConfig;
    impl crate::value::ToValue for SchemaRegistryConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.access_configs {
                properties.insert(
                    "AccessConfigs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.event_record_format {
                properties.insert(
                    "EventRecordFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.schema_registry_uri {
                properties.insert(
                    "SchemaRegistryURI".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.schema_validation_configs {
                properties.insert(
                    "SchemaValidationConfigs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-schemavalidationconfig.html
    pub struct SchemaValidationConfig_ {
        pub attribute: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_EventSourceMapping_SchemaValidationConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::EventSourceMapping.SchemaValidationConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_EventSourceMapping_SchemaValidationConfig as SchemaValidationConfig;
    impl crate::value::ToValue for SchemaValidationConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attribute {
                properties.insert(
                    "Attribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-selfmanagedeventsource.html
    pub struct SelfManagedEventSource_ {
        pub endpoints: Option<Box<Endpoints_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_EventSourceMapping_SelfManagedEventSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::EventSourceMapping.SelfManagedEventSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_EventSourceMapping_SelfManagedEventSource as SelfManagedEventSource;
    impl crate::value::ToValue for SelfManagedEventSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.endpoints {
                properties.insert(
                    "Endpoints".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-selfmanagedkafkaeventsourceconfig.html
    pub struct SelfManagedKafkaEventSourceConfig_ {
        pub consumer_group_id: Option<crate::value::ExpString>,
        pub schema_registry_config: Option<Box<SchemaRegistryConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_EventSourceMapping_SelfManagedKafkaEventSourceConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::EventSourceMapping.SelfManagedKafkaEventSourceConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_EventSourceMapping_SelfManagedKafkaEventSourceConfig as SelfManagedKafkaEventSourceConfig;
    impl crate::value::ToValue for SelfManagedKafkaEventSourceConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.consumer_group_id {
                properties.insert(
                    "ConsumerGroupId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.schema_registry_config {
                properties.insert(
                    "SchemaRegistryConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-sourceaccessconfiguration.html
    pub struct SourceAccessConfiguration_ {
        pub r#type: Option<crate::value::ExpString>,
        pub uri: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_EventSourceMapping_SourceAccessConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::EventSourceMapping.SourceAccessConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_EventSourceMapping_SourceAccessConfiguration as SourceAccessConfiguration;
    impl crate::value::ToValue for SourceAccessConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.uri {
                properties.insert("URI".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod function {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-code.html
    pub struct Code_ {
        pub image_uri: Option<crate::value::ExpString>,
        pub s3_bucket: Option<crate::value::ExpString>,
        pub s3_key: Option<crate::value::ExpString>,
        pub s3_object_version: Option<crate::value::ExpString>,
        pub source_kms_key_arn: Option<crate::value::ExpString>,
        pub zip_file: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_Function_Code {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::Function.Code"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_Function_Code as Code;
    impl crate::value::ToValue for Code_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.image_uri {
                properties.insert(
                    "ImageUri".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_bucket {
                properties.insert(
                    "S3Bucket".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_key {
                properties.insert("S3Key".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.s3_object_version {
                properties.insert(
                    "S3ObjectVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_kms_key_arn {
                properties.insert(
                    "SourceKMSKeyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.zip_file {
                properties.insert(
                    "ZipFile".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-deadletterconfig.html
    pub struct DeadLetterConfig_ {
        pub target_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_Function_DeadLetterConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::Function.DeadLetterConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_Function_DeadLetterConfig as DeadLetterConfig;
    impl crate::value::ToValue for DeadLetterConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.target_arn {
                properties.insert(
                    "TargetArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-environment.html
    pub struct Environment_ {
        pub variables: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_Function_Environment {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::Function.Environment"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_Function_Environment as Environment;
    impl crate::value::ToValue for Environment_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.variables {
                properties.insert(
                    "Variables".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-ephemeralstorage.html
    pub struct EphemeralStorage_ {
        pub size: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_Function_EphemeralStorage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::Function.EphemeralStorage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_Function_EphemeralStorage as EphemeralStorage;
    impl crate::value::ToValue for EphemeralStorage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Size".to_string(),
                crate::value::ToValue::to_value(&self.size),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-filesystemconfig.html
    pub struct FileSystemConfig_ {
        pub arn: crate::value::ExpString,
        pub local_mount_path: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_Function_FileSystemConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::Function.FileSystemConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_Function_FileSystemConfig as FileSystemConfig;
    impl crate::value::ToValue for FileSystemConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Arn".to_string(),
                crate::value::ToValue::to_value(&self.arn),
            );
            properties.insert(
                "LocalMountPath".to_string(),
                crate::value::ToValue::to_value(&self.local_mount_path),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-imageconfig.html
    pub struct ImageConfig_ {
        pub command: Option<Vec<crate::value::ExpString>>,
        pub entry_point: Option<Vec<crate::value::ExpString>>,
        pub working_directory: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_Function_ImageConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::Function.ImageConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_Function_ImageConfig as ImageConfig;
    impl crate::value::ToValue for ImageConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.command {
                properties.insert(
                    "Command".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.entry_point {
                properties.insert(
                    "EntryPoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.working_directory {
                properties.insert(
                    "WorkingDirectory".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-loggingconfig.html
    pub struct LoggingConfig_ {
        pub application_log_level: Option<crate::value::ExpString>,
        pub log_format: Option<crate::value::ExpString>,
        pub log_group: Option<crate::value::ExpString>,
        pub system_log_level: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_Function_LoggingConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::Function.LoggingConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_Function_LoggingConfig as LoggingConfig;
    impl crate::value::ToValue for LoggingConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.application_log_level {
                properties.insert(
                    "ApplicationLogLevel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.log_format {
                properties.insert(
                    "LogFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.log_group {
                properties.insert(
                    "LogGroup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.system_log_level {
                properties.insert(
                    "SystemLogLevel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-runtimemanagementconfig.html
    pub struct RuntimeManagementConfig_ {
        pub runtime_version_arn: Option<crate::value::ExpString>,
        pub update_runtime_on: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_Function_RuntimeManagementConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::Function.RuntimeManagementConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_Function_RuntimeManagementConfig as RuntimeManagementConfig;
    impl crate::value::ToValue for RuntimeManagementConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.runtime_version_arn {
                properties.insert(
                    "RuntimeVersionArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "UpdateRuntimeOn".to_string(),
                crate::value::ToValue::to_value(&self.update_runtime_on),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-snapstart.html
    pub struct SnapStart_ {
        pub apply_on: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_Function_SnapStart {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::Function.SnapStart"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_Function_SnapStart as SnapStart;
    impl crate::value::ToValue for SnapStart_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ApplyOn".to_string(),
                crate::value::ToValue::to_value(&self.apply_on),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-snapstartresponse.html
    pub struct SnapStartResponse_ {
        pub apply_on: Option<crate::value::ExpString>,
        pub optimization_status: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_Function_SnapStartResponse {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::Function.SnapStartResponse"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_Function_SnapStartResponse as SnapStartResponse;
    impl crate::value::ToValue for SnapStartResponse_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.apply_on {
                properties.insert(
                    "ApplyOn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.optimization_status {
                properties.insert(
                    "OptimizationStatus".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-tracingconfig.html
    pub struct TracingConfig_ {
        pub mode: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_Function_TracingConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::Function.TracingConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_Function_TracingConfig as TracingConfig;
    impl crate::value::ToValue for TracingConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.mode {
                properties.insert("Mode".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-vpcconfig.html
    pub struct VpcConfig_ {
        pub ipv6_allowed_for_dual_stack: Option<crate::value::ExpBool>,
        pub security_group_ids: Option<Vec<crate::value::ExpString>>,
        pub subnet_ids: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_Function_VpcConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::Function.VpcConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_Function_VpcConfig as VpcConfig;
    impl crate::value::ToValue for VpcConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ipv6_allowed_for_dual_stack {
                properties.insert(
                    "Ipv6AllowedForDualStack".to_string(),
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
            properties.into()
        }
    }
}
pub mod layerversion {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-layerversion-content.html
    pub struct Content_ {
        pub s3_bucket: crate::value::ExpString,
        pub s3_key: crate::value::ExpString,
        pub s3_object_version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_LayerVersion_Content {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::LayerVersion.Content"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_LayerVersion_Content as Content;
    impl crate::value::ToValue for Content_ {
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
            if let Some(ref value) = self.s3_object_version {
                properties.insert(
                    "S3ObjectVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod url {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-url-cors.html
    pub struct Cors_ {
        pub allow_credentials: Option<crate::value::ExpBool>,
        pub allow_headers: Option<Vec<crate::value::ExpString>>,
        pub allow_methods: Option<Vec<crate::value::ExpString>>,
        pub allow_origins: Option<Vec<crate::value::ExpString>>,
        pub expose_headers: Option<Vec<crate::value::ExpString>>,
        pub max_age: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_Url_Cors {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::Url.Cors"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_Url_Cors as Cors;
    impl crate::value::ToValue for Cors_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allow_credentials {
                properties.insert(
                    "AllowCredentials".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.allow_headers {
                properties.insert(
                    "AllowHeaders".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.allow_methods {
                properties.insert(
                    "AllowMethods".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.allow_origins {
                properties.insert(
                    "AllowOrigins".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.expose_headers {
                properties.insert(
                    "ExposeHeaders".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_age {
                properties.insert("MaxAge".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod version {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-version-provisionedconcurrencyconfiguration.html
    pub struct ProvisionedConcurrencyConfiguration_ {
        pub provisioned_concurrent_executions: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_Version_ProvisionedConcurrencyConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::Version.ProvisionedConcurrencyConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_Version_ProvisionedConcurrencyConfiguration as ProvisionedConcurrencyConfiguration;
    impl crate::value::ToValue for ProvisionedConcurrencyConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ProvisionedConcurrentExecutions".to_string(),
                crate::value::ToValue::to_value(&self.provisioned_concurrent_executions),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-version-runtimepolicy.html
    pub struct RuntimePolicy_ {
        pub runtime_version_arn: Option<crate::value::ExpString>,
        pub update_runtime_on: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lambda_Version_RuntimePolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Lambda::Version.RuntimePolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lambda_Version_RuntimePolicy as RuntimePolicy;
    impl crate::value::ToValue for RuntimePolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.runtime_version_arn {
                properties.insert(
                    "RuntimeVersionArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "UpdateRuntimeOn".to_string(),
                crate::value::ToValue::to_value(&self.update_runtime_on),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-alias.html
pub struct Alias_ {
    pub description: Option<crate::value::ExpString>,
    pub function_name: crate::value::ExpString,
    pub function_version: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub provisioned_concurrency_config:
        Option<super::lambda::alias::ProvisionedConcurrencyConfiguration_>,
    pub routing_config: Option<super::lambda::alias::AliasRoutingConfiguration_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lambda_Alias {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Lambda::Alias" $($field
        $value)*)
    };
}
pub use crate::__aws_lambda_Alias as Alias;
impl crate::template::ToResource for Alias_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Lambda"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Alias"),
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
            "FunctionName".to_string(),
            crate::value::ToValue::to_value(&self.function_name),
        );
        properties.insert(
            "FunctionVersion".to_string(),
            crate::value::ToValue::to_value(&self.function_version),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.provisioned_concurrency_config {
            properties.insert(
                "ProvisionedConcurrencyConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.routing_config {
            properties.insert(
                "RoutingConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-codesigningconfig.html
pub struct CodeSigningConfig_ {
    pub allowed_publishers: super::lambda::codesigningconfig::AllowedPublishers_,
    pub code_signing_policies: Option<super::lambda::codesigningconfig::CodeSigningPolicies_>,
    pub description: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lambda_CodeSigningConfig {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Lambda::CodeSigningConfig"
        $($field $value)*)
    };
}
pub use crate::__aws_lambda_CodeSigningConfig as CodeSigningConfig;
impl crate::template::ToResource for CodeSigningConfig_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Lambda"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("CodeSigningConfig"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AllowedPublishers".to_string(),
            crate::value::ToValue::to_value(&self.allowed_publishers),
        );
        if let Some(ref value) = self.code_signing_policies {
            properties.insert(
                "CodeSigningPolicies".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventinvokeconfig.html
pub struct EventInvokeConfig_ {
    pub destination_config: Option<super::lambda::eventinvokeconfig::DestinationConfig_>,
    pub function_name: crate::value::ExpString,
    pub maximum_event_age_in_seconds: Option<i64>,
    pub maximum_retry_attempts: Option<i64>,
    pub qualifier: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lambda_EventInvokeConfig {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Lambda::EventInvokeConfig"
        $($field $value)*)
    };
}
pub use crate::__aws_lambda_EventInvokeConfig as EventInvokeConfig;
impl crate::template::ToResource for EventInvokeConfig_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Lambda"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("EventInvokeConfig"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.destination_config {
            properties.insert(
                "DestinationConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "FunctionName".to_string(),
            crate::value::ToValue::to_value(&self.function_name),
        );
        if let Some(ref value) = self.maximum_event_age_in_seconds {
            properties.insert(
                "MaximumEventAgeInSeconds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.maximum_retry_attempts {
            properties.insert(
                "MaximumRetryAttempts".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Qualifier".to_string(),
            crate::value::ToValue::to_value(&self.qualifier),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html
pub struct EventSourceMapping_ {
    pub amazon_managed_kafka_event_source_config:
        Option<super::lambda::eventsourcemapping::AmazonManagedKafkaEventSourceConfig_>,
    pub batch_size: Option<i64>,
    pub bisect_batch_on_function_error: Option<crate::value::ExpBool>,
    pub destination_config: Option<super::lambda::eventsourcemapping::DestinationConfig_>,
    pub document_db_event_source_config:
        Option<super::lambda::eventsourcemapping::DocumentDBEventSourceConfig_>,
    pub enabled: Option<crate::value::ExpBool>,
    pub event_source_arn: Option<crate::value::ExpString>,
    pub filter_criteria: Option<super::lambda::eventsourcemapping::FilterCriteria_>,
    pub function_name: crate::value::ExpString,
    pub function_response_types: Option<Vec<crate::value::ExpString>>,
    pub kms_key_arn: Option<crate::value::ExpString>,
    pub maximum_batching_window_in_seconds: Option<i64>,
    pub maximum_record_age_in_seconds: Option<i64>,
    pub maximum_retry_attempts: Option<i64>,
    pub metrics_config: Option<super::lambda::eventsourcemapping::MetricsConfig_>,
    pub parallelization_factor: Option<i64>,
    pub provisioned_poller_config:
        Option<super::lambda::eventsourcemapping::ProvisionedPollerConfig_>,
    pub queues: Option<Vec<crate::value::ExpString>>,
    pub scaling_config: Option<super::lambda::eventsourcemapping::ScalingConfig_>,
    pub self_managed_event_source:
        Option<super::lambda::eventsourcemapping::SelfManagedEventSource_>,
    pub self_managed_kafka_event_source_config:
        Option<super::lambda::eventsourcemapping::SelfManagedKafkaEventSourceConfig_>,
    pub source_access_configurations:
        Option<Vec<super::lambda::eventsourcemapping::SourceAccessConfiguration_>>,
    pub starting_position: Option<crate::value::ExpString>,
    pub starting_position_timestamp: Option<f64>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub topics: Option<Vec<crate::value::ExpString>>,
    pub tumbling_window_in_seconds: Option<i64>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lambda_EventSourceMapping {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Lambda::EventSourceMapping"
        $($field $value)*)
    };
}
pub use crate::__aws_lambda_EventSourceMapping as EventSourceMapping;
impl crate::template::ToResource for EventSourceMapping_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Lambda"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("EventSourceMapping"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.amazon_managed_kafka_event_source_config {
            properties.insert(
                "AmazonManagedKafkaEventSourceConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.batch_size {
            properties.insert(
                "BatchSize".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.bisect_batch_on_function_error {
            properties.insert(
                "BisectBatchOnFunctionError".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.destination_config {
            properties.insert(
                "DestinationConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.document_db_event_source_config {
            properties.insert(
                "DocumentDBEventSourceConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enabled {
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.event_source_arn {
            properties.insert(
                "EventSourceArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.filter_criteria {
            properties.insert(
                "FilterCriteria".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "FunctionName".to_string(),
            crate::value::ToValue::to_value(&self.function_name),
        );
        if let Some(ref value) = self.function_response_types {
            properties.insert(
                "FunctionResponseTypes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_arn {
            properties.insert(
                "KmsKeyArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.maximum_batching_window_in_seconds {
            properties.insert(
                "MaximumBatchingWindowInSeconds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.maximum_record_age_in_seconds {
            properties.insert(
                "MaximumRecordAgeInSeconds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.maximum_retry_attempts {
            properties.insert(
                "MaximumRetryAttempts".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.metrics_config {
            properties.insert(
                "MetricsConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.parallelization_factor {
            properties.insert(
                "ParallelizationFactor".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.provisioned_poller_config {
            properties.insert(
                "ProvisionedPollerConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.queues {
            properties.insert("Queues".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.scaling_config {
            properties.insert(
                "ScalingConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.self_managed_event_source {
            properties.insert(
                "SelfManagedEventSource".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.self_managed_kafka_event_source_config {
            properties.insert(
                "SelfManagedKafkaEventSourceConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_access_configurations {
            properties.insert(
                "SourceAccessConfigurations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.starting_position {
            properties.insert(
                "StartingPosition".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.starting_position_timestamp {
            properties.insert(
                "StartingPositionTimestamp".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.topics {
            properties.insert("Topics".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tumbling_window_in_seconds {
            properties.insert(
                "TumblingWindowInSeconds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html
pub struct Function_ {
    pub architectures: Option<Vec<crate::value::ExpString>>,
    pub code: super::lambda::function::Code_,
    pub code_signing_config_arn: Option<crate::value::ExpString>,
    pub dead_letter_config: Option<super::lambda::function::DeadLetterConfig_>,
    pub description: Option<crate::value::ExpString>,
    pub environment: Option<super::lambda::function::Environment_>,
    pub ephemeral_storage: Option<super::lambda::function::EphemeralStorage_>,
    pub file_system_configs: Option<Vec<super::lambda::function::FileSystemConfig_>>,
    pub function_name: Option<crate::value::ExpString>,
    pub handler: Option<crate::value::ExpString>,
    pub image_config: Option<super::lambda::function::ImageConfig_>,
    pub kms_key_arn: Option<crate::value::ExpString>,
    pub layers: Option<Vec<crate::value::ExpString>>,
    pub logging_config: Option<super::lambda::function::LoggingConfig_>,
    pub memory_size: Option<i64>,
    pub package_type: Option<crate::value::ExpString>,
    pub recursive_loop: Option<crate::value::ExpString>,
    pub reserved_concurrent_executions: Option<i64>,
    pub role: crate::value::ExpString,
    pub runtime: Option<crate::value::ExpString>,
    pub runtime_management_config: Option<super::lambda::function::RuntimeManagementConfig_>,
    pub snap_start: Option<super::lambda::function::SnapStart_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub timeout: Option<i64>,
    pub tracing_config: Option<super::lambda::function::TracingConfig_>,
    pub vpc_config: Option<super::lambda::function::VpcConfig_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lambda_Function {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Lambda::Function" $($field
        $value)*)
    };
}
pub use crate::__aws_lambda_Function as Function;
impl crate::template::ToResource for Function_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Lambda"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Function"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.architectures {
            properties.insert(
                "Architectures".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Code".to_string(),
            crate::value::ToValue::to_value(&self.code),
        );
        if let Some(ref value) = self.code_signing_config_arn {
            properties.insert(
                "CodeSigningConfigArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.dead_letter_config {
            properties.insert(
                "DeadLetterConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.environment {
            properties.insert(
                "Environment".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ephemeral_storage {
            properties.insert(
                "EphemeralStorage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.file_system_configs {
            properties.insert(
                "FileSystemConfigs".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.function_name {
            properties.insert(
                "FunctionName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.handler {
            properties.insert(
                "Handler".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.image_config {
            properties.insert(
                "ImageConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_arn {
            properties.insert(
                "KmsKeyArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.layers {
            properties.insert("Layers".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.logging_config {
            properties.insert(
                "LoggingConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.memory_size {
            properties.insert(
                "MemorySize".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.package_type {
            properties.insert(
                "PackageType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.recursive_loop {
            properties.insert(
                "RecursiveLoop".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.reserved_concurrent_executions {
            properties.insert(
                "ReservedConcurrentExecutions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Role".to_string(),
            crate::value::ToValue::to_value(&self.role),
        );
        if let Some(ref value) = self.runtime {
            properties.insert(
                "Runtime".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.runtime_management_config {
            properties.insert(
                "RuntimeManagementConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.snap_start {
            properties.insert(
                "SnapStart".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.timeout {
            properties.insert(
                "Timeout".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tracing_config {
            properties.insert(
                "TracingConfig".to_string(),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-layerversion.html
pub struct LayerVersion_ {
    pub compatible_architectures: Option<Vec<crate::value::ExpString>>,
    pub compatible_runtimes: Option<Vec<crate::value::ExpString>>,
    pub content: super::lambda::layerversion::Content_,
    pub description: Option<crate::value::ExpString>,
    pub layer_name: Option<crate::value::ExpString>,
    pub license_info: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lambda_LayerVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Lambda::LayerVersion"
        $($field $value)*)
    };
}
pub use crate::__aws_lambda_LayerVersion as LayerVersion;
impl crate::template::ToResource for LayerVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Lambda"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LayerVersion"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.compatible_architectures {
            properties.insert(
                "CompatibleArchitectures".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.compatible_runtimes {
            properties.insert(
                "CompatibleRuntimes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Content".to_string(),
            crate::value::ToValue::to_value(&self.content),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.layer_name {
            properties.insert(
                "LayerName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.license_info {
            properties.insert(
                "LicenseInfo".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-layerversionpermission.html
pub struct LayerVersionPermission_ {
    pub action: crate::value::ExpString,
    pub layer_version_arn: crate::value::ExpString,
    pub organization_id: Option<crate::value::ExpString>,
    pub principal: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lambda_LayerVersionPermission {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Lambda::LayerVersionPermission"
        $($field $value)*)
    };
}
pub use crate::__aws_lambda_LayerVersionPermission as LayerVersionPermission;
impl crate::template::ToResource for LayerVersionPermission_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Lambda"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LayerVersionPermission"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Action".to_string(),
            crate::value::ToValue::to_value(&self.action),
        );
        properties.insert(
            "LayerVersionArn".to_string(),
            crate::value::ToValue::to_value(&self.layer_version_arn),
        );
        if let Some(ref value) = self.organization_id {
            properties.insert(
                "OrganizationId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Principal".to_string(),
            crate::value::ToValue::to_value(&self.principal),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-permission.html
pub struct Permission_ {
    pub action: crate::value::ExpString,
    pub event_source_token: Option<crate::value::ExpString>,
    pub function_name: crate::value::ExpString,
    pub function_url_auth_type: Option<crate::value::ExpString>,
    pub principal: crate::value::ExpString,
    pub principal_org_id: Option<crate::value::ExpString>,
    pub source_account: Option<crate::value::ExpString>,
    pub source_arn: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lambda_Permission {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Lambda::Permission"
        $($field $value)*)
    };
}
pub use crate::__aws_lambda_Permission as Permission;
impl crate::template::ToResource for Permission_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Lambda"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Permission"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Action".to_string(),
            crate::value::ToValue::to_value(&self.action),
        );
        if let Some(ref value) = self.event_source_token {
            properties.insert(
                "EventSourceToken".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "FunctionName".to_string(),
            crate::value::ToValue::to_value(&self.function_name),
        );
        if let Some(ref value) = self.function_url_auth_type {
            properties.insert(
                "FunctionUrlAuthType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Principal".to_string(),
            crate::value::ToValue::to_value(&self.principal),
        );
        if let Some(ref value) = self.principal_org_id {
            properties.insert(
                "PrincipalOrgID".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_account {
            properties.insert(
                "SourceAccount".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_arn {
            properties.insert(
                "SourceArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-url.html
pub struct Url_ {
    pub auth_type: crate::value::ExpString,
    pub cors: Option<super::lambda::url::Cors_>,
    pub invoke_mode: Option<crate::value::ExpString>,
    pub qualifier: Option<crate::value::ExpString>,
    pub target_function_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lambda_Url {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Lambda::Url" $($field
        $value)*)
    };
}
pub use crate::__aws_lambda_Url as Url;
impl crate::template::ToResource for Url_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Lambda"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Url"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AuthType".to_string(),
            crate::value::ToValue::to_value(&self.auth_type),
        );
        if let Some(ref value) = self.cors {
            properties.insert("Cors".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.invoke_mode {
            properties.insert(
                "InvokeMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.qualifier {
            properties.insert(
                "Qualifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "TargetFunctionArn".to_string(),
            crate::value::ToValue::to_value(&self.target_function_arn),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-version.html
pub struct Version_ {
    pub code_sha256: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub function_name: crate::value::ExpString,
    pub provisioned_concurrency_config:
        Option<super::lambda::version::ProvisionedConcurrencyConfiguration_>,
    pub runtime_policy: Option<super::lambda::version::RuntimePolicy_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lambda_Version {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Lambda::Version" $($field
        $value)*)
    };
}
pub use crate::__aws_lambda_Version as Version;
impl crate::template::ToResource for Version_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Lambda"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Version"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.code_sha256 {
            properties.insert(
                "CodeSha256".to_string(),
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
            "FunctionName".to_string(),
            crate::value::ToValue::to_value(&self.function_name),
        );
        if let Some(ref value) = self.provisioned_concurrency_config {
            properties.insert(
                "ProvisionedConcurrencyConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.runtime_policy {
            properties.insert(
                "RuntimePolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
