pub mod application {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appconfig-application-tags.html
    pub struct Tags_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appconfig_Application_Tags {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppConfig::Application.Tags"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appconfig_Application_Tags as Tags;
    impl crate::value::ToValue for Tags_ {
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
pub mod configurationprofile {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appconfig-configurationprofile-tags.html
    pub struct Tags_ {
        pub key: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appconfig_ConfigurationProfile_Tags {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppConfig::ConfigurationProfile.Tags"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appconfig_ConfigurationProfile_Tags as Tags;
    impl crate::value::ToValue for Tags_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appconfig-configurationprofile-validators.html
    pub struct Validators_ {
        pub content: Option<crate::value::ExpString>,
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appconfig_ConfigurationProfile_Validators {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppConfig::ConfigurationProfile.Validators"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appconfig_ConfigurationProfile_Validators as Validators;
    impl crate::value::ToValue for Validators_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.content {
                properties.insert(
                    "Content".to_string(),
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
pub mod deployment {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appconfig-deployment-dynamicextensionparameters.html
    pub struct DynamicExtensionParameters_ {
        pub extension_reference: Option<crate::value::ExpString>,
        pub parameter_name: Option<crate::value::ExpString>,
        pub parameter_value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appconfig_Deployment_DynamicExtensionParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppConfig::Deployment.DynamicExtensionParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appconfig_Deployment_DynamicExtensionParameters as DynamicExtensionParameters;
    impl crate::value::ToValue for DynamicExtensionParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.extension_reference {
                properties.insert(
                    "ExtensionReference".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parameter_name {
                properties.insert(
                    "ParameterName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parameter_value {
                properties.insert(
                    "ParameterValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod environment {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appconfig-environment-monitor.html
    pub struct Monitor_ {
        pub alarm_arn: crate::value::ExpString,
        pub alarm_role_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appconfig_Environment_Monitor {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppConfig::Environment.Monitor"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appconfig_Environment_Monitor as Monitor;
    impl crate::value::ToValue for Monitor_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AlarmArn".to_string(),
                crate::value::ToValue::to_value(&self.alarm_arn),
            );
            if let Some(ref value) = self.alarm_role_arn {
                properties.insert(
                    "AlarmRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod extension {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appconfig-extension-parameter.html
    pub struct Parameter_ {
        pub description: Option<crate::value::ExpString>,
        pub dynamic: Option<crate::value::ExpBool>,
        pub required: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appconfig_Extension_Parameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppConfig::Extension.Parameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appconfig_Extension_Parameter as Parameter;
    impl crate::value::ToValue for Parameter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dynamic {
                properties.insert(
                    "Dynamic".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Required".to_string(),
                crate::value::ToValue::to_value(&self.required),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appconfig-application.html
pub struct Application_ {
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<super::appconfig::application::Tags_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appconfig_Application {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::AppConfig::Application"
        $($field $value)*)
    };
}
pub use crate::__aws_appconfig_Application as Application;
impl crate::template::ToResource for Application_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppConfig"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Application"),
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
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appconfig-configurationprofile.html
pub struct ConfigurationProfile_ {
    pub application_id: crate::value::ExpString,
    pub deletion_protection_check: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub kms_key_identifier: Option<crate::value::ExpString>,
    pub location_uri: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub retrieval_role_arn: Option<crate::value::ExpString>,
    pub tags: Option<Vec<super::appconfig::configurationprofile::Tags_>>,
    pub r#type: Option<crate::value::ExpString>,
    pub validators: Option<Vec<super::appconfig::configurationprofile::Validators_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appconfig_ConfigurationProfile {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::AppConfig::ConfigurationProfile"
        $($field $value)*)
    };
}
pub use crate::__aws_appconfig_ConfigurationProfile as ConfigurationProfile;
impl crate::template::ToResource for ConfigurationProfile_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppConfig"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ConfigurationProfile"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApplicationId".to_string(),
            crate::value::ToValue::to_value(&self.application_id),
        );
        if let Some(ref value) = self.deletion_protection_check {
            properties.insert(
                "DeletionProtectionCheck".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_identifier {
            properties.insert(
                "KmsKeyIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "LocationUri".to_string(),
            crate::value::ToValue::to_value(&self.location_uri),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.retrieval_role_arn {
            properties.insert(
                "RetrievalRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.r#type {
            properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.validators {
            properties.insert(
                "Validators".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appconfig-deployment.html
pub struct Deployment_ {
    pub application_id: crate::value::ExpString,
    pub configuration_profile_id: crate::value::ExpString,
    pub configuration_version: crate::value::ExpString,
    pub deployment_strategy_id: crate::value::ExpString,
    pub description: Option<crate::value::ExpString>,
    pub dynamic_extension_parameters:
        Option<Vec<super::appconfig::deployment::DynamicExtensionParameters_>>,
    pub environment_id: crate::value::ExpString,
    pub kms_key_identifier: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appconfig_Deployment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::AppConfig::Deployment"
        $($field $value)*)
    };
}
pub use crate::__aws_appconfig_Deployment as Deployment;
impl crate::template::ToResource for Deployment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppConfig"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Deployment"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApplicationId".to_string(),
            crate::value::ToValue::to_value(&self.application_id),
        );
        properties.insert(
            "ConfigurationProfileId".to_string(),
            crate::value::ToValue::to_value(&self.configuration_profile_id),
        );
        properties.insert(
            "ConfigurationVersion".to_string(),
            crate::value::ToValue::to_value(&self.configuration_version),
        );
        properties.insert(
            "DeploymentStrategyId".to_string(),
            crate::value::ToValue::to_value(&self.deployment_strategy_id),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.dynamic_extension_parameters {
            properties.insert(
                "DynamicExtensionParameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "EnvironmentId".to_string(),
            crate::value::ToValue::to_value(&self.environment_id),
        );
        if let Some(ref value) = self.kms_key_identifier {
            properties.insert(
                "KmsKeyIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appconfig-deploymentstrategy.html
pub struct DeploymentStrategy_ {
    pub deployment_duration_in_minutes: f64,
    pub description: Option<crate::value::ExpString>,
    pub final_bake_time_in_minutes: Option<f64>,
    pub growth_factor: f64,
    pub growth_type: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub replicate_to: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appconfig_DeploymentStrategy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::AppConfig::DeploymentStrategy"
        $($field $value)*)
    };
}
pub use crate::__aws_appconfig_DeploymentStrategy as DeploymentStrategy;
impl crate::template::ToResource for DeploymentStrategy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppConfig"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DeploymentStrategy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DeploymentDurationInMinutes".to_string(),
            crate::value::ToValue::to_value(&self.deployment_duration_in_minutes),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.final_bake_time_in_minutes {
            properties.insert(
                "FinalBakeTimeInMinutes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "GrowthFactor".to_string(),
            crate::value::ToValue::to_value(&self.growth_factor),
        );
        if let Some(ref value) = self.growth_type {
            properties.insert(
                "GrowthType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "ReplicateTo".to_string(),
            crate::value::ToValue::to_value(&self.replicate_to),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appconfig-environment.html
pub struct Environment_ {
    pub application_id: crate::value::ExpString,
    pub deletion_protection_check: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub monitors: Option<Vec<super::appconfig::environment::Monitor_>>,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appconfig_Environment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::AppConfig::Environment"
        $($field $value)*)
    };
}
pub use crate::__aws_appconfig_Environment as Environment;
impl crate::template::ToResource for Environment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppConfig"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Environment"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApplicationId".to_string(),
            crate::value::ToValue::to_value(&self.application_id),
        );
        if let Some(ref value) = self.deletion_protection_check {
            properties.insert(
                "DeletionProtectionCheck".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.monitors {
            properties.insert(
                "Monitors".to_string(),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appconfig-extension.html
pub struct Extension_ {
    pub actions: serde_json::Value,
    pub description: Option<crate::value::ExpString>,
    pub latest_version_number: Option<i64>,
    pub name: crate::value::ExpString,
    pub parameters:
        Option<std::collections::BTreeMap<String, super::appconfig::extension::Parameter_>>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appconfig_Extension {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::AppConfig::Extension"
        $($field $value)*)
    };
}
pub use crate::__aws_appconfig_Extension as Extension;
impl crate::template::ToResource for Extension_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppConfig"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Extension"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Actions".to_string(),
            crate::value::ToValue::to_value(&self.actions),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.latest_version_number {
            properties.insert(
                "LatestVersionNumber".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appconfig-extensionassociation.html
pub struct ExtensionAssociation_ {
    pub extension_identifier: Option<crate::value::ExpString>,
    pub extension_version_number: Option<i64>,
    pub parameters: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub resource_identifier: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appconfig_ExtensionAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::AppConfig::ExtensionAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_appconfig_ExtensionAssociation as ExtensionAssociation;
impl crate::template::ToResource for ExtensionAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppConfig"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ExtensionAssociation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.extension_identifier {
            properties.insert(
                "ExtensionIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.extension_version_number {
            properties.insert(
                "ExtensionVersionNumber".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.parameters {
            properties.insert(
                "Parameters".to_string(),
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
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appconfig-hostedconfigurationversion.html
pub struct HostedConfigurationVersion_ {
    pub application_id: crate::value::ExpString,
    pub configuration_profile_id: crate::value::ExpString,
    pub content: crate::value::ExpString,
    pub content_type: crate::value::ExpString,
    pub description: Option<crate::value::ExpString>,
    pub latest_version_number: Option<i64>,
    pub version_label: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appconfig_HostedConfigurationVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::AppConfig::HostedConfigurationVersion"
        $($field $value)*)
    };
}
pub use crate::__aws_appconfig_HostedConfigurationVersion as HostedConfigurationVersion;
impl crate::template::ToResource for HostedConfigurationVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppConfig"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "HostedConfigurationVersion",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApplicationId".to_string(),
            crate::value::ToValue::to_value(&self.application_id),
        );
        properties.insert(
            "ConfigurationProfileId".to_string(),
            crate::value::ToValue::to_value(&self.configuration_profile_id),
        );
        properties.insert(
            "Content".to_string(),
            crate::value::ToValue::to_value(&self.content),
        );
        properties.insert(
            "ContentType".to_string(),
            crate::value::ToValue::to_value(&self.content_type),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.latest_version_number {
            properties.insert(
                "LatestVersionNumber".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.version_label {
            properties.insert(
                "VersionLabel".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
