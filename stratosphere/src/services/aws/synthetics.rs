pub mod canary {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-synthetics-canary-artifactconfig.html
    pub struct ArtifactConfig_ {
        pub s3_encryption: Option<Box<S3Encryption_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_synthetics_Canary_ArtifactConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Synthetics::Canary.ArtifactConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_synthetics_Canary_ArtifactConfig as ArtifactConfig;
    impl crate::value::ToValue for ArtifactConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.s3_encryption {
                properties.insert(
                    "S3Encryption".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-synthetics-canary-basescreenshot.html
    pub struct BaseScreenshot_ {
        pub ignore_coordinates: Option<Vec<crate::value::ExpString>>,
        pub screenshot_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_synthetics_Canary_BaseScreenshot {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Synthetics::Canary.BaseScreenshot"
            $($field $value)*)
        };
    }
    pub use crate::__aws_synthetics_Canary_BaseScreenshot as BaseScreenshot;
    impl crate::value::ToValue for BaseScreenshot_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ignore_coordinates {
                properties.insert(
                    "IgnoreCoordinates".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ScreenshotName".to_string(),
                crate::value::ToValue::to_value(&self.screenshot_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-synthetics-canary-browserconfig.html
    pub struct BrowserConfig_ {
        pub browser_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_synthetics_Canary_BrowserConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Synthetics::Canary.BrowserConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_synthetics_Canary_BrowserConfig as BrowserConfig;
    impl crate::value::ToValue for BrowserConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BrowserType".to_string(),
                crate::value::ToValue::to_value(&self.browser_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-synthetics-canary-code.html
    pub struct Code_ {
        pub dependencies: Option<Vec<Dependency_>>,
        pub handler: crate::value::ExpString,
        pub s3_bucket: Option<crate::value::ExpString>,
        pub s3_key: Option<crate::value::ExpString>,
        pub s3_object_version: Option<crate::value::ExpString>,
        pub script: Option<crate::value::ExpString>,
        pub source_location_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_synthetics_Canary_Code {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Synthetics::Canary.Code"
            $($field $value)*)
        };
    }
    pub use crate::__aws_synthetics_Canary_Code as Code;
    impl crate::value::ToValue for Code_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dependencies {
                properties.insert(
                    "Dependencies".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Handler".to_string(),
                crate::value::ToValue::to_value(&self.handler),
            );
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
            if let Some(ref value) = self.script {
                properties.insert("Script".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.source_location_arn {
                properties.insert(
                    "SourceLocationArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-synthetics-canary-dependency.html
    pub struct Dependency_ {
        pub reference: crate::value::ExpString,
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_synthetics_Canary_Dependency {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Synthetics::Canary.Dependency"
            $($field $value)*)
        };
    }
    pub use crate::__aws_synthetics_Canary_Dependency as Dependency;
    impl crate::value::ToValue for Dependency_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Reference".to_string(),
                crate::value::ToValue::to_value(&self.reference),
            );
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-synthetics-canary-retryconfig.html
    pub struct RetryConfig_ {
        pub max_retries: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_synthetics_Canary_RetryConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Synthetics::Canary.RetryConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_synthetics_Canary_RetryConfig as RetryConfig;
    impl crate::value::ToValue for RetryConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MaxRetries".to_string(),
                crate::value::ToValue::to_value(&self.max_retries),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-synthetics-canary-runconfig.html
    pub struct RunConfig_ {
        pub active_tracing: Option<crate::value::ExpBool>,
        pub environment_variables:
            Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub ephemeral_storage: Option<i64>,
        pub memory_in_mb: Option<i64>,
        pub timeout_in_seconds: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_synthetics_Canary_RunConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Synthetics::Canary.RunConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_synthetics_Canary_RunConfig as RunConfig;
    impl crate::value::ToValue for RunConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.active_tracing {
                properties.insert(
                    "ActiveTracing".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.environment_variables {
                properties.insert(
                    "EnvironmentVariables".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ephemeral_storage {
                properties.insert(
                    "EphemeralStorage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.memory_in_mb {
                properties.insert(
                    "MemoryInMB".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timeout_in_seconds {
                properties.insert(
                    "TimeoutInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-synthetics-canary-s3encryption.html
    pub struct S3Encryption_ {
        pub encryption_mode: Option<crate::value::ExpString>,
        pub kms_key_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_synthetics_Canary_S3Encryption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Synthetics::Canary.S3Encryption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_synthetics_Canary_S3Encryption as S3Encryption;
    impl crate::value::ToValue for S3Encryption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.encryption_mode {
                properties.insert(
                    "EncryptionMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kms_key_arn {
                properties.insert(
                    "KmsKeyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-synthetics-canary-schedule.html
    pub struct Schedule_ {
        pub duration_in_seconds: Option<crate::value::ExpString>,
        pub expression: crate::value::ExpString,
        pub retry_config: Option<Box<RetryConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_synthetics_Canary_Schedule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Synthetics::Canary.Schedule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_synthetics_Canary_Schedule as Schedule;
    impl crate::value::ToValue for Schedule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.duration_in_seconds {
                properties.insert(
                    "DurationInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Expression".to_string(),
                crate::value::ToValue::to_value(&self.expression),
            );
            if let Some(ref value) = self.retry_config {
                properties.insert(
                    "RetryConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-synthetics-canary-vpcconfig.html
    pub struct VPCConfig_ {
        pub ipv6_allowed_for_dual_stack: Option<crate::value::ExpBool>,
        pub security_group_ids: Vec<crate::value::ExpString>,
        pub subnet_ids: Vec<crate::value::ExpString>,
        pub vpc_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_synthetics_Canary_VPCConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Synthetics::Canary.VPCConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_synthetics_Canary_VPCConfig as VPCConfig;
    impl crate::value::ToValue for VPCConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ipv6_allowed_for_dual_stack {
                properties.insert(
                    "Ipv6AllowedForDualStack".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(&self.security_group_ids),
            );
            properties.insert(
                "SubnetIds".to_string(),
                crate::value::ToValue::to_value(&self.subnet_ids),
            );
            if let Some(ref value) = self.vpc_id {
                properties.insert("VpcId".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-synthetics-canary-visualreference.html
    pub struct VisualReference_ {
        pub base_canary_run_id: crate::value::ExpString,
        pub base_screenshots: Option<Vec<BaseScreenshot_>>,
        pub browser_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_synthetics_Canary_VisualReference {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Synthetics::Canary.VisualReference"
            $($field $value)*)
        };
    }
    pub use crate::__aws_synthetics_Canary_VisualReference as VisualReference;
    impl crate::value::ToValue for VisualReference_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BaseCanaryRunId".to_string(),
                crate::value::ToValue::to_value(&self.base_canary_run_id),
            );
            if let Some(ref value) = self.base_screenshots {
                properties.insert(
                    "BaseScreenshots".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.browser_type {
                properties.insert(
                    "BrowserType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-synthetics-canary.html
pub struct Canary_ {
    pub artifact_config: Option<super::synthetics::canary::ArtifactConfig_>,
    pub artifact_s3_location: crate::value::ExpString,
    pub browser_configs: Option<Vec<super::synthetics::canary::BrowserConfig_>>,
    pub code: super::synthetics::canary::Code_,
    pub dry_run_and_update: Option<crate::value::ExpBool>,
    pub execution_role_arn: crate::value::ExpString,
    pub failure_retention_period: Option<i64>,
    pub name: crate::value::ExpString,
    pub provisioned_resource_cleanup: Option<crate::value::ExpString>,
    pub resources_to_replicate_tags: Option<Vec<crate::value::ExpString>>,
    pub run_config: Option<super::synthetics::canary::RunConfig_>,
    pub runtime_version: crate::value::ExpString,
    pub schedule: super::synthetics::canary::Schedule_,
    pub start_canary_after_creation: Option<crate::value::ExpBool>,
    pub success_retention_period: Option<i64>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_config: Option<super::synthetics::canary::VPCConfig_>,
    pub visual_references: Option<Vec<super::synthetics::canary::VisualReference_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_synthetics_Canary {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Synthetics::Canary"
        $($field $value)*)
    };
}
pub use crate::__aws_synthetics_Canary as Canary;
impl crate::template::ToResource for Canary_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Synthetics"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Canary"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.artifact_config {
            properties.insert(
                "ArtifactConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ArtifactS3Location".to_string(),
            crate::value::ToValue::to_value(&self.artifact_s3_location),
        );
        if let Some(ref value) = self.browser_configs {
            properties.insert(
                "BrowserConfigs".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Code".to_string(),
            crate::value::ToValue::to_value(&self.code),
        );
        if let Some(ref value) = self.dry_run_and_update {
            properties.insert(
                "DryRunAndUpdate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ExecutionRoleArn".to_string(),
            crate::value::ToValue::to_value(&self.execution_role_arn),
        );
        if let Some(ref value) = self.failure_retention_period {
            properties.insert(
                "FailureRetentionPeriod".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.provisioned_resource_cleanup {
            properties.insert(
                "ProvisionedResourceCleanup".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resources_to_replicate_tags {
            properties.insert(
                "ResourcesToReplicateTags".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.run_config {
            properties.insert(
                "RunConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RuntimeVersion".to_string(),
            crate::value::ToValue::to_value(&self.runtime_version),
        );
        properties.insert(
            "Schedule".to_string(),
            crate::value::ToValue::to_value(&self.schedule),
        );
        if let Some(ref value) = self.start_canary_after_creation {
            properties.insert(
                "StartCanaryAfterCreation".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.success_retention_period {
            properties.insert(
                "SuccessRetentionPeriod".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.vpc_config {
            properties.insert(
                "VPCConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.visual_references {
            properties.insert(
                "VisualReferences".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-synthetics-group.html
pub struct Group_ {
    pub name: crate::value::ExpString,
    pub resource_arns: Option<Vec<crate::value::ExpString>>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_synthetics_Group {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Synthetics::Group"
        $($field $value)*)
    };
}
pub use crate::__aws_synthetics_Group as Group;
impl crate::template::ToResource for Group_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Synthetics"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Group"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.resource_arns {
            properties.insert(
                "ResourceArns".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
