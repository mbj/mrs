pub mod observabilityconfiguration {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-observabilityconfiguration-traceconfiguration.html
    pub struct TraceConfiguration_ {
        pub vendor: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apprunner_ObservabilityConfiguration_TraceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppRunner::ObservabilityConfiguration.TraceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apprunner_ObservabilityConfiguration_TraceConfiguration as TraceConfiguration;
    impl crate::value::ToValue for TraceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Vendor".to_string(),
                crate::value::ToValue::to_value(&self.vendor),
            );
            properties.into()
        }
    }
}
pub mod service {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-authenticationconfiguration.html
    pub struct AuthenticationConfiguration_ {
        pub access_role_arn: Option<crate::value::ExpString>,
        pub connection_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apprunner_Service_AuthenticationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppRunner::Service.AuthenticationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apprunner_Service_AuthenticationConfiguration as AuthenticationConfiguration;
    impl crate::value::ToValue for AuthenticationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.access_role_arn {
                properties.insert(
                    "AccessRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.connection_arn {
                properties.insert(
                    "ConnectionArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-codeconfiguration.html
    pub struct CodeConfiguration_ {
        pub code_configuration_values: Option<Box<CodeConfigurationValues_>>,
        pub configuration_source: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apprunner_Service_CodeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppRunner::Service.CodeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apprunner_Service_CodeConfiguration as CodeConfiguration;
    impl crate::value::ToValue for CodeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.code_configuration_values {
                properties.insert(
                    "CodeConfigurationValues".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ConfigurationSource".to_string(),
                crate::value::ToValue::to_value(&self.configuration_source),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-codeconfigurationvalues.html
    pub struct CodeConfigurationValues_ {
        pub build_command: Option<crate::value::ExpString>,
        pub port: Option<crate::value::ExpString>,
        pub runtime: crate::value::ExpString,
        pub runtime_environment_secrets: Option<Vec<KeyValuePair_>>,
        pub runtime_environment_variables: Option<Vec<KeyValuePair_>>,
        pub start_command: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apprunner_Service_CodeConfigurationValues {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppRunner::Service.CodeConfigurationValues"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apprunner_Service_CodeConfigurationValues as CodeConfigurationValues;
    impl crate::value::ToValue for CodeConfigurationValues_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.build_command {
                properties.insert(
                    "BuildCommand".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.port {
                properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Runtime".to_string(),
                crate::value::ToValue::to_value(&self.runtime),
            );
            if let Some(ref value) = self.runtime_environment_secrets {
                properties.insert(
                    "RuntimeEnvironmentSecrets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.runtime_environment_variables {
                properties.insert(
                    "RuntimeEnvironmentVariables".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.start_command {
                properties.insert(
                    "StartCommand".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-coderepository.html
    pub struct CodeRepository_ {
        pub code_configuration: Option<Box<CodeConfiguration_>>,
        pub repository_url: crate::value::ExpString,
        pub source_code_version: Box<SourceCodeVersion_>,
        pub source_directory: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apprunner_Service_CodeRepository {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppRunner::Service.CodeRepository"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apprunner_Service_CodeRepository as CodeRepository;
    impl crate::value::ToValue for CodeRepository_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.code_configuration {
                properties.insert(
                    "CodeConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RepositoryUrl".to_string(),
                crate::value::ToValue::to_value(&self.repository_url),
            );
            properties.insert(
                "SourceCodeVersion".to_string(),
                crate::value::ToValue::to_value(&self.source_code_version),
            );
            if let Some(ref value) = self.source_directory {
                properties.insert(
                    "SourceDirectory".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-egressconfiguration.html
    pub struct EgressConfiguration_ {
        pub egress_type: crate::value::ExpString,
        pub vpc_connector_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apprunner_Service_EgressConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppRunner::Service.EgressConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apprunner_Service_EgressConfiguration as EgressConfiguration;
    impl crate::value::ToValue for EgressConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EgressType".to_string(),
                crate::value::ToValue::to_value(&self.egress_type),
            );
            if let Some(ref value) = self.vpc_connector_arn {
                properties.insert(
                    "VpcConnectorArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-encryptionconfiguration.html
    pub struct EncryptionConfiguration_ {
        pub kms_key: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apprunner_Service_EncryptionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppRunner::Service.EncryptionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apprunner_Service_EncryptionConfiguration as EncryptionConfiguration;
    impl crate::value::ToValue for EncryptionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "KmsKey".to_string(),
                crate::value::ToValue::to_value(&self.kms_key),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-healthcheckconfiguration.html
    pub struct HealthCheckConfiguration_ {
        pub healthy_threshold: Option<i64>,
        pub interval: Option<i64>,
        pub path: Option<crate::value::ExpString>,
        pub protocol: Option<crate::value::ExpString>,
        pub timeout: Option<i64>,
        pub unhealthy_threshold: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apprunner_Service_HealthCheckConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppRunner::Service.HealthCheckConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apprunner_Service_HealthCheckConfiguration as HealthCheckConfiguration;
    impl crate::value::ToValue for HealthCheckConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.healthy_threshold {
                properties.insert(
                    "HealthyThreshold".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.interval {
                properties.insert(
                    "Interval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.path {
                properties.insert("Path".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.protocol {
                properties.insert(
                    "Protocol".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timeout {
                properties.insert(
                    "Timeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.unhealthy_threshold {
                properties.insert(
                    "UnhealthyThreshold".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-imageconfiguration.html
    pub struct ImageConfiguration_ {
        pub port: Option<crate::value::ExpString>,
        pub runtime_environment_secrets: Option<Vec<KeyValuePair_>>,
        pub runtime_environment_variables: Option<Vec<KeyValuePair_>>,
        pub start_command: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apprunner_Service_ImageConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppRunner::Service.ImageConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apprunner_Service_ImageConfiguration as ImageConfiguration;
    impl crate::value::ToValue for ImageConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.port {
                properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.runtime_environment_secrets {
                properties.insert(
                    "RuntimeEnvironmentSecrets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.runtime_environment_variables {
                properties.insert(
                    "RuntimeEnvironmentVariables".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.start_command {
                properties.insert(
                    "StartCommand".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-imagerepository.html
    pub struct ImageRepository_ {
        pub image_configuration: Option<Box<ImageConfiguration_>>,
        pub image_identifier: crate::value::ExpString,
        pub image_repository_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apprunner_Service_ImageRepository {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppRunner::Service.ImageRepository"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apprunner_Service_ImageRepository as ImageRepository;
    impl crate::value::ToValue for ImageRepository_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.image_configuration {
                properties.insert(
                    "ImageConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ImageIdentifier".to_string(),
                crate::value::ToValue::to_value(&self.image_identifier),
            );
            properties.insert(
                "ImageRepositoryType".to_string(),
                crate::value::ToValue::to_value(&self.image_repository_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-ingressconfiguration.html
    pub struct IngressConfiguration_ {
        pub is_publicly_accessible: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apprunner_Service_IngressConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppRunner::Service.IngressConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apprunner_Service_IngressConfiguration as IngressConfiguration;
    impl crate::value::ToValue for IngressConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IsPubliclyAccessible".to_string(),
                crate::value::ToValue::to_value(&self.is_publicly_accessible),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-instanceconfiguration.html
    pub struct InstanceConfiguration_ {
        pub cpu: Option<crate::value::ExpString>,
        pub instance_role_arn: Option<crate::value::ExpString>,
        pub memory: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apprunner_Service_InstanceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppRunner::Service.InstanceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apprunner_Service_InstanceConfiguration as InstanceConfiguration;
    impl crate::value::ToValue for InstanceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cpu {
                properties.insert("Cpu".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.instance_role_arn {
                properties.insert(
                    "InstanceRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.memory {
                properties.insert("Memory".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-keyvaluepair.html
    pub struct KeyValuePair_ {
        pub name: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apprunner_Service_KeyValuePair {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppRunner::Service.KeyValuePair"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apprunner_Service_KeyValuePair as KeyValuePair;
    impl crate::value::ToValue for KeyValuePair_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-networkconfiguration.html
    pub struct NetworkConfiguration_ {
        pub egress_configuration: Option<Box<EgressConfiguration_>>,
        pub ingress_configuration: Option<Box<IngressConfiguration_>>,
        pub ip_address_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apprunner_Service_NetworkConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppRunner::Service.NetworkConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apprunner_Service_NetworkConfiguration as NetworkConfiguration;
    impl crate::value::ToValue for NetworkConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.egress_configuration {
                properties.insert(
                    "EgressConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ingress_configuration {
                properties.insert(
                    "IngressConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ip_address_type {
                properties.insert(
                    "IpAddressType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-serviceobservabilityconfiguration.html
    pub struct ServiceObservabilityConfiguration_ {
        pub observability_configuration_arn: Option<crate::value::ExpString>,
        pub observability_enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apprunner_Service_ServiceObservabilityConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppRunner::Service.ServiceObservabilityConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apprunner_Service_ServiceObservabilityConfiguration as ServiceObservabilityConfiguration;
    impl crate::value::ToValue for ServiceObservabilityConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.observability_configuration_arn {
                properties.insert(
                    "ObservabilityConfigurationArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ObservabilityEnabled".to_string(),
                crate::value::ToValue::to_value(&self.observability_enabled),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-sourcecodeversion.html
    pub struct SourceCodeVersion_ {
        pub r#type: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apprunner_Service_SourceCodeVersion {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppRunner::Service.SourceCodeVersion"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apprunner_Service_SourceCodeVersion as SourceCodeVersion;
    impl crate::value::ToValue for SourceCodeVersion_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-sourceconfiguration.html
    pub struct SourceConfiguration_ {
        pub authentication_configuration: Option<Box<AuthenticationConfiguration_>>,
        pub auto_deployments_enabled: Option<crate::value::ExpBool>,
        pub code_repository: Option<Box<CodeRepository_>>,
        pub image_repository: Option<Box<ImageRepository_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apprunner_Service_SourceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppRunner::Service.SourceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apprunner_Service_SourceConfiguration as SourceConfiguration;
    impl crate::value::ToValue for SourceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.authentication_configuration {
                properties.insert(
                    "AuthenticationConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.auto_deployments_enabled {
                properties.insert(
                    "AutoDeploymentsEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.code_repository {
                properties.insert(
                    "CodeRepository".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.image_repository {
                properties.insert(
                    "ImageRepository".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod vpcingressconnection {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-vpcingressconnection-ingressvpcconfiguration.html
    pub struct IngressVpcConfiguration_ {
        pub vpc_endpoint_id: crate::value::ExpString,
        pub vpc_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apprunner_VpcIngressConnection_IngressVpcConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppRunner::VpcIngressConnection.IngressVpcConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apprunner_VpcIngressConnection_IngressVpcConfiguration as IngressVpcConfiguration;
    impl crate::value::ToValue for IngressVpcConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "VpcEndpointId".to_string(),
                crate::value::ToValue::to_value(&self.vpc_endpoint_id),
            );
            properties.insert(
                "VpcId".to_string(),
                crate::value::ToValue::to_value(&self.vpc_id),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apprunner-autoscalingconfiguration.html
pub struct AutoScalingConfiguration_ {
    pub auto_scaling_configuration_name: Option<crate::value::ExpString>,
    pub max_concurrency: Option<i64>,
    pub max_size: Option<i64>,
    pub min_size: Option<i64>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apprunner_AutoScalingConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::AppRunner::AutoScalingConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_apprunner_AutoScalingConfiguration as AutoScalingConfiguration;
impl crate::template::ToResource for AutoScalingConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppRunner"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AutoScalingConfiguration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.auto_scaling_configuration_name {
            properties.insert(
                "AutoScalingConfigurationName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.max_concurrency {
            properties.insert(
                "MaxConcurrency".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.max_size {
            properties.insert(
                "MaxSize".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.min_size {
            properties.insert(
                "MinSize".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apprunner-observabilityconfiguration.html
pub struct ObservabilityConfiguration_ {
    pub observability_configuration_name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub trace_configuration:
        Option<super::apprunner::observabilityconfiguration::TraceConfiguration_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apprunner_ObservabilityConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::AppRunner::ObservabilityConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_apprunner_ObservabilityConfiguration as ObservabilityConfiguration;
impl crate::template::ToResource for ObservabilityConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppRunner"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "ObservabilityConfiguration",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.observability_configuration_name {
            properties.insert(
                "ObservabilityConfigurationName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.trace_configuration {
            properties.insert(
                "TraceConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apprunner-service.html
pub struct Service_ {
    pub auto_scaling_configuration_arn: Option<crate::value::ExpString>,
    pub encryption_configuration: Option<super::apprunner::service::EncryptionConfiguration_>,
    pub health_check_configuration: Option<super::apprunner::service::HealthCheckConfiguration_>,
    pub instance_configuration: Option<super::apprunner::service::InstanceConfiguration_>,
    pub network_configuration: Option<super::apprunner::service::NetworkConfiguration_>,
    pub observability_configuration:
        Option<super::apprunner::service::ServiceObservabilityConfiguration_>,
    pub service_name: Option<crate::value::ExpString>,
    pub source_configuration: super::apprunner::service::SourceConfiguration_,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apprunner_Service {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::AppRunner::Service"
        $($field $value)*)
    };
}
pub use crate::__aws_apprunner_Service as Service;
impl crate::template::ToResource for Service_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppRunner"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Service"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.auto_scaling_configuration_arn {
            properties.insert(
                "AutoScalingConfigurationArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.encryption_configuration {
            properties.insert(
                "EncryptionConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.health_check_configuration {
            properties.insert(
                "HealthCheckConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.instance_configuration {
            properties.insert(
                "InstanceConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.network_configuration {
            properties.insert(
                "NetworkConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.observability_configuration {
            properties.insert(
                "ObservabilityConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.service_name {
            properties.insert(
                "ServiceName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SourceConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.source_configuration),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apprunner-vpcconnector.html
pub struct VpcConnector_ {
    pub security_groups: Option<Vec<crate::value::ExpString>>,
    pub subnets: Vec<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_connector_name: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apprunner_VpcConnector {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::AppRunner::VpcConnector"
        $($field $value)*)
    };
}
pub use crate::__aws_apprunner_VpcConnector as VpcConnector;
impl crate::template::ToResource for VpcConnector_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppRunner"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("VpcConnector"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.security_groups {
            properties.insert(
                "SecurityGroups".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Subnets".to_string(),
            crate::value::ToValue::to_value(&self.subnets),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.vpc_connector_name {
            properties.insert(
                "VpcConnectorName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apprunner-vpcingressconnection.html
pub struct VpcIngressConnection_ {
    pub ingress_vpc_configuration: super::apprunner::vpcingressconnection::IngressVpcConfiguration_,
    pub service_arn: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_ingress_connection_name: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apprunner_VpcIngressConnection {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::AppRunner::VpcIngressConnection"
        $($field $value)*)
    };
}
pub use crate::__aws_apprunner_VpcIngressConnection as VpcIngressConnection;
impl crate::template::ToResource for VpcIngressConnection_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppRunner"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("VpcIngressConnection"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "IngressVpcConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.ingress_vpc_configuration),
        );
        properties.insert(
            "ServiceArn".to_string(),
            crate::value::ToValue::to_value(&self.service_arn),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.vpc_ingress_connection_name {
            properties.insert(
                "VpcIngressConnectionName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
