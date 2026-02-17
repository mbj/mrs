pub mod application {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticbeanstalk-application-applicationresourcelifecycleconfig.html>
    pub struct ApplicationResourceLifecycleConfig_ {
        pub service_role: Option<crate::value::ExpString>,
        pub version_lifecycle_config: Option<Box<ApplicationVersionLifecycleConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticbeanstalk_Application_ApplicationResourceLifecycleConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticBeanstalk::Application.ApplicationResourceLifecycleConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticbeanstalk_Application_ApplicationResourceLifecycleConfig as ApplicationResourceLifecycleConfig;
    impl crate::value::ToValue for ApplicationResourceLifecycleConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.service_role {
                properties.insert(
                    "ServiceRole".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.version_lifecycle_config {
                properties.insert(
                    "VersionLifecycleConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticbeanstalk-application-applicationversionlifecycleconfig.html>
    pub struct ApplicationVersionLifecycleConfig_ {
        pub max_age_rule: Option<Box<MaxAgeRule_>>,
        pub max_count_rule: Option<Box<MaxCountRule_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticbeanstalk_Application_ApplicationVersionLifecycleConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticBeanstalk::Application.ApplicationVersionLifecycleConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticbeanstalk_Application_ApplicationVersionLifecycleConfig as ApplicationVersionLifecycleConfig;
    impl crate::value::ToValue for ApplicationVersionLifecycleConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max_age_rule {
                properties.insert(
                    "MaxAgeRule".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_count_rule {
                properties.insert(
                    "MaxCountRule".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticbeanstalk-application-maxagerule.html>
    pub struct MaxAgeRule_ {
        pub delete_source_from_s3: Option<crate::value::ExpBool>,
        pub enabled: Option<crate::value::ExpBool>,
        pub max_age_in_days: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticbeanstalk_Application_MaxAgeRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticBeanstalk::Application.MaxAgeRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticbeanstalk_Application_MaxAgeRule as MaxAgeRule;
    impl crate::value::ToValue for MaxAgeRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.delete_source_from_s3 {
                properties.insert(
                    "DeleteSourceFromS3".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_age_in_days {
                properties.insert(
                    "MaxAgeInDays".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticbeanstalk-application-maxcountrule.html>
    pub struct MaxCountRule_ {
        pub delete_source_from_s3: Option<crate::value::ExpBool>,
        pub enabled: Option<crate::value::ExpBool>,
        pub max_count: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticbeanstalk_Application_MaxCountRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticBeanstalk::Application.MaxCountRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticbeanstalk_Application_MaxCountRule as MaxCountRule;
    impl crate::value::ToValue for MaxCountRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.delete_source_from_s3 {
                properties.insert(
                    "DeleteSourceFromS3".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_count {
                properties.insert(
                    "MaxCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod applicationversion {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticbeanstalk-applicationversion-sourcebundle.html>
    pub struct SourceBundle_ {
        pub s3_bucket: crate::value::ExpString,
        pub s3_key: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticbeanstalk_ApplicationVersion_SourceBundle {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticBeanstalk::ApplicationVersion.SourceBundle"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticbeanstalk_ApplicationVersion_SourceBundle as SourceBundle;
    impl crate::value::ToValue for SourceBundle_ {
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
pub mod configurationtemplate {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticbeanstalk-configurationtemplate-configurationoptionsetting.html>
    pub struct ConfigurationOptionSetting_ {
        pub namespace: crate::value::ExpString,
        pub option_name: crate::value::ExpString,
        pub resource_name: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticbeanstalk_ConfigurationTemplate_ConfigurationOptionSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticBeanstalk::ConfigurationTemplate.ConfigurationOptionSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticbeanstalk_ConfigurationTemplate_ConfigurationOptionSetting as ConfigurationOptionSetting;
    impl crate::value::ToValue for ConfigurationOptionSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Namespace".to_string(),
                crate::value::ToValue::to_value(&self.namespace),
            );
            properties.insert(
                "OptionName".to_string(),
                crate::value::ToValue::to_value(&self.option_name),
            );
            if let Some(ref value) = self.resource_name {
                properties.insert(
                    "ResourceName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticbeanstalk-configurationtemplate-sourceconfiguration.html>
    pub struct SourceConfiguration_ {
        pub application_name: crate::value::ExpString,
        pub template_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticbeanstalk_ConfigurationTemplate_SourceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticBeanstalk::ConfigurationTemplate.SourceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticbeanstalk_ConfigurationTemplate_SourceConfiguration as SourceConfiguration;
    impl crate::value::ToValue for SourceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ApplicationName".to_string(),
                crate::value::ToValue::to_value(&self.application_name),
            );
            properties.insert(
                "TemplateName".to_string(),
                crate::value::ToValue::to_value(&self.template_name),
            );
            properties.into()
        }
    }
}
pub mod environment {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticbeanstalk-environment-optionsetting.html>
    pub struct OptionSetting_ {
        pub namespace: crate::value::ExpString,
        pub option_name: crate::value::ExpString,
        pub resource_name: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticbeanstalk_Environment_OptionSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticBeanstalk::Environment.OptionSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticbeanstalk_Environment_OptionSetting as OptionSetting;
    impl crate::value::ToValue for OptionSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Namespace".to_string(),
                crate::value::ToValue::to_value(&self.namespace),
            );
            properties.insert(
                "OptionName".to_string(),
                crate::value::ToValue::to_value(&self.option_name),
            );
            if let Some(ref value) = self.resource_name {
                properties.insert(
                    "ResourceName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticbeanstalk-environment-tier.html>
    pub struct Tier_ {
        pub name: Option<crate::value::ExpString>,
        pub r#type: Option<crate::value::ExpString>,
        pub version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticbeanstalk_Environment_Tier {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticBeanstalk::Environment.Tier"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticbeanstalk_Environment_Tier as Tier;
    impl crate::value::ToValue for Tier_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.version {
                properties.insert(
                    "Version".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticbeanstalk-application.html>
pub struct Application_ {
    pub application_name: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub resource_lifecycle_config:
        Option<super::elasticbeanstalk::application::ApplicationResourceLifecycleConfig_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_elasticbeanstalk_Application {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ElasticBeanstalk::Application"
        $($field $value)*)
    };
}
pub use crate::__aws_elasticbeanstalk_Application as Application;
impl crate::template::ToResource for Application_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ElasticBeanstalk"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Application"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.application_name {
            properties.insert(
                "ApplicationName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resource_lifecycle_config {
            properties.insert(
                "ResourceLifecycleConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticbeanstalk-applicationversion.html>
pub struct ApplicationVersion_ {
    pub application_name: crate::value::ExpString,
    pub description: Option<crate::value::ExpString>,
    pub source_bundle: super::elasticbeanstalk::applicationversion::SourceBundle_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_elasticbeanstalk_ApplicationVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ElasticBeanstalk::ApplicationVersion"
        $($field $value)*)
    };
}
pub use crate::__aws_elasticbeanstalk_ApplicationVersion as ApplicationVersion;
impl crate::template::ToResource for ApplicationVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ElasticBeanstalk"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ApplicationVersion"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApplicationName".to_string(),
            crate::value::ToValue::to_value(&self.application_name),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SourceBundle".to_string(),
            crate::value::ToValue::to_value(&self.source_bundle),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticbeanstalk-configurationtemplate.html>
pub struct ConfigurationTemplate_ {
    pub application_name: crate::value::ExpString,
    pub description: Option<crate::value::ExpString>,
    pub environment_id: Option<crate::value::ExpString>,
    pub option_settings:
        Option<Vec<super::elasticbeanstalk::configurationtemplate::ConfigurationOptionSetting_>>,
    pub platform_arn: Option<crate::value::ExpString>,
    pub solution_stack_name: Option<crate::value::ExpString>,
    pub source_configuration:
        Option<super::elasticbeanstalk::configurationtemplate::SourceConfiguration_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_elasticbeanstalk_ConfigurationTemplate {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ElasticBeanstalk::ConfigurationTemplate"
        $($field $value)*)
    };
}
pub use crate::__aws_elasticbeanstalk_ConfigurationTemplate as ConfigurationTemplate;
impl crate::template::ToResource for ConfigurationTemplate_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ElasticBeanstalk"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ConfigurationTemplate"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApplicationName".to_string(),
            crate::value::ToValue::to_value(&self.application_name),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.environment_id {
            properties.insert(
                "EnvironmentId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.option_settings {
            properties.insert(
                "OptionSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.platform_arn {
            properties.insert(
                "PlatformArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.solution_stack_name {
            properties.insert(
                "SolutionStackName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_configuration {
            properties.insert(
                "SourceConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticbeanstalk-environment.html>
pub struct Environment_ {
    pub application_name: crate::value::ExpString,
    pub cname_prefix: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub environment_name: Option<crate::value::ExpString>,
    pub operations_role: Option<crate::value::ExpString>,
    pub option_settings: Option<Vec<super::elasticbeanstalk::environment::OptionSetting_>>,
    pub platform_arn: Option<crate::value::ExpString>,
    pub solution_stack_name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub template_name: Option<crate::value::ExpString>,
    pub tier: Option<super::elasticbeanstalk::environment::Tier_>,
    pub version_label: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_elasticbeanstalk_Environment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ElasticBeanstalk::Environment"
        $($field $value)*)
    };
}
pub use crate::__aws_elasticbeanstalk_Environment as Environment;
impl crate::template::ToResource for Environment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ElasticBeanstalk"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Environment"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApplicationName".to_string(),
            crate::value::ToValue::to_value(&self.application_name),
        );
        if let Some(ref value) = self.cname_prefix {
            properties.insert(
                "CNAMEPrefix".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.environment_name {
            properties.insert(
                "EnvironmentName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.operations_role {
            properties.insert(
                "OperationsRole".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.option_settings {
            properties.insert(
                "OptionSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.platform_arn {
            properties.insert(
                "PlatformArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.solution_stack_name {
            properties.insert(
                "SolutionStackName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.template_name {
            properties.insert(
                "TemplateName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tier {
            properties.insert("Tier".to_string(), crate::value::ToValue::to_value(value));
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
