pub mod app {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-app-autobranchcreationconfig.html>
    pub struct AutoBranchCreationConfig_ {
        pub auto_branch_creation_patterns: Option<Vec<crate::value::ExpString>>,
        pub basic_auth_config: Option<Box<BasicAuthConfig_>>,
        pub build_spec: Option<crate::value::ExpString>,
        pub enable_auto_branch_creation: Option<crate::value::ExpBool>,
        pub enable_auto_build: Option<crate::value::ExpBool>,
        pub enable_performance_mode: Option<crate::value::ExpBool>,
        pub enable_pull_request_preview: Option<crate::value::ExpBool>,
        pub environment_variables: Option<Vec<EnvironmentVariable_>>,
        pub framework: Option<crate::value::ExpString>,
        pub pull_request_environment_name: Option<crate::value::ExpString>,
        pub stage: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplify_App_AutoBranchCreationConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Amplify::App.AutoBranchCreationConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplify_App_AutoBranchCreationConfig as AutoBranchCreationConfig;
    impl crate::value::ToValue for AutoBranchCreationConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auto_branch_creation_patterns {
                properties.insert(
                    "AutoBranchCreationPatterns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.basic_auth_config {
                properties.insert(
                    "BasicAuthConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.build_spec {
                properties.insert(
                    "BuildSpec".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enable_auto_branch_creation {
                properties.insert(
                    "EnableAutoBranchCreation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enable_auto_build {
                properties.insert(
                    "EnableAutoBuild".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enable_performance_mode {
                properties.insert(
                    "EnablePerformanceMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enable_pull_request_preview {
                properties.insert(
                    "EnablePullRequestPreview".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.environment_variables {
                properties.insert(
                    "EnvironmentVariables".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.framework {
                properties.insert(
                    "Framework".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pull_request_environment_name {
                properties.insert(
                    "PullRequestEnvironmentName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stage {
                properties.insert("Stage".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-app-basicauthconfig.html>
    pub struct BasicAuthConfig_ {
        pub enable_basic_auth: Option<crate::value::ExpBool>,
        pub password: Option<crate::value::ExpString>,
        pub username: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplify_App_BasicAuthConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Amplify::App.BasicAuthConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplify_App_BasicAuthConfig as BasicAuthConfig;
    impl crate::value::ToValue for BasicAuthConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enable_basic_auth {
                properties.insert(
                    "EnableBasicAuth".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.password {
                properties.insert(
                    "Password".to_string(),
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-app-cacheconfig.html>
    pub struct CacheConfig_ {
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplify_App_CacheConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Amplify::App.CacheConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplify_App_CacheConfig as CacheConfig;
    impl crate::value::ToValue for CacheConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-app-customrule.html>
    pub struct CustomRule_ {
        pub condition: Option<crate::value::ExpString>,
        pub source: crate::value::ExpString,
        pub status: Option<crate::value::ExpString>,
        pub target: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplify_App_CustomRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Amplify::App.CustomRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplify_App_CustomRule as CustomRule;
    impl crate::value::ToValue for CustomRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.condition {
                properties.insert(
                    "Condition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Source".to_string(),
                crate::value::ToValue::to_value(&self.source),
            );
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Target".to_string(),
                crate::value::ToValue::to_value(&self.target),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-app-environmentvariable.html>
    pub struct EnvironmentVariable_ {
        pub name: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplify_App_EnvironmentVariable {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Amplify::App.EnvironmentVariable"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplify_App_EnvironmentVariable as EnvironmentVariable;
    impl crate::value::ToValue for EnvironmentVariable_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-app-jobconfig.html>
    pub struct JobConfig_ {
        pub build_compute_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplify_App_JobConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Amplify::App.JobConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplify_App_JobConfig as JobConfig;
    impl crate::value::ToValue for JobConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BuildComputeType".to_string(),
                crate::value::ToValue::to_value(&self.build_compute_type),
            );
            properties.into()
        }
    }
}
pub mod branch {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-branch-backend.html>
    pub struct Backend_ {
        pub stack_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplify_Branch_Backend {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Amplify::Branch.Backend"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplify_Branch_Backend as Backend;
    impl crate::value::ToValue for Backend_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.stack_arn {
                properties.insert(
                    "StackArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-branch-basicauthconfig.html>
    pub struct BasicAuthConfig_ {
        pub enable_basic_auth: Option<crate::value::ExpBool>,
        pub password: crate::value::ExpString,
        pub username: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplify_Branch_BasicAuthConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Amplify::Branch.BasicAuthConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplify_Branch_BasicAuthConfig as BasicAuthConfig;
    impl crate::value::ToValue for BasicAuthConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enable_basic_auth {
                properties.insert(
                    "EnableBasicAuth".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Password".to_string(),
                crate::value::ToValue::to_value(&self.password),
            );
            properties.insert(
                "Username".to_string(),
                crate::value::ToValue::to_value(&self.username),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-branch-environmentvariable.html>
    pub struct EnvironmentVariable_ {
        pub name: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplify_Branch_EnvironmentVariable {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Amplify::Branch.EnvironmentVariable"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplify_Branch_EnvironmentVariable as EnvironmentVariable;
    impl crate::value::ToValue for EnvironmentVariable_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
}
pub mod domain {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-domain-certificate.html>
    pub struct Certificate_ {
        pub certificate_arn: Option<crate::value::ExpString>,
        pub certificate_type: Option<crate::value::ExpString>,
        pub certificate_verification_dns_record: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplify_Domain_Certificate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Amplify::Domain.Certificate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplify_Domain_Certificate as Certificate;
    impl crate::value::ToValue for Certificate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.certificate_arn {
                properties.insert(
                    "CertificateArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.certificate_type {
                properties.insert(
                    "CertificateType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.certificate_verification_dns_record {
                properties.insert(
                    "CertificateVerificationDNSRecord".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-domain-certificatesettings.html>
    pub struct CertificateSettings_ {
        pub certificate_type: Option<crate::value::ExpString>,
        pub custom_certificate_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplify_Domain_CertificateSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Amplify::Domain.CertificateSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplify_Domain_CertificateSettings as CertificateSettings;
    impl crate::value::ToValue for CertificateSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.certificate_type {
                properties.insert(
                    "CertificateType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_certificate_arn {
                properties.insert(
                    "CustomCertificateArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplify-domain-subdomainsetting.html>
    pub struct SubDomainSetting_ {
        pub branch_name: crate::value::ExpString,
        pub prefix: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplify_Domain_SubDomainSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Amplify::Domain.SubDomainSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplify_Domain_SubDomainSetting as SubDomainSetting;
    impl crate::value::ToValue for SubDomainSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BranchName".to_string(),
                crate::value::ToValue::to_value(&self.branch_name),
            );
            properties.insert(
                "Prefix".to_string(),
                crate::value::ToValue::to_value(&self.prefix),
            );
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-app.html>
pub struct App_ {
    pub access_token: Option<crate::value::ExpString>,
    pub auto_branch_creation_config: Option<super::amplify::app::AutoBranchCreationConfig_>,
    pub basic_auth_config: Option<super::amplify::app::BasicAuthConfig_>,
    pub build_spec: Option<crate::value::ExpString>,
    pub cache_config: Option<super::amplify::app::CacheConfig_>,
    pub compute_role_arn: Option<crate::value::ExpString>,
    pub custom_headers: Option<crate::value::ExpString>,
    pub custom_rules: Option<Vec<super::amplify::app::CustomRule_>>,
    pub description: Option<crate::value::ExpString>,
    pub enable_branch_auto_deletion: Option<crate::value::ExpBool>,
    pub environment_variables: Option<Vec<super::amplify::app::EnvironmentVariable_>>,
    pub iam_service_role: Option<crate::value::ExpString>,
    pub job_config: Option<super::amplify::app::JobConfig_>,
    pub name: crate::value::ExpString,
    pub oauth_token: Option<crate::value::ExpString>,
    pub platform: Option<crate::value::ExpString>,
    pub repository: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_amplify_App {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Amplify::App" $($field
        $value)*)
    };
}
pub use crate::__aws_amplify_App as App;
impl crate::template::ToResource for App_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Amplify"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("App"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.access_token {
            properties.insert(
                "AccessToken".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.auto_branch_creation_config {
            properties.insert(
                "AutoBranchCreationConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.basic_auth_config {
            properties.insert(
                "BasicAuthConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.build_spec {
            properties.insert(
                "BuildSpec".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cache_config {
            properties.insert(
                "CacheConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.compute_role_arn {
            properties.insert(
                "ComputeRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.custom_headers {
            properties.insert(
                "CustomHeaders".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.custom_rules {
            properties.insert(
                "CustomRules".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enable_branch_auto_deletion {
            properties.insert(
                "EnableBranchAutoDeletion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.environment_variables {
            properties.insert(
                "EnvironmentVariables".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.iam_service_role {
            properties.insert(
                "IAMServiceRole".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.job_config {
            properties.insert(
                "JobConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.oauth_token {
            properties.insert(
                "OauthToken".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.platform {
            properties.insert(
                "Platform".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.repository {
            properties.insert(
                "Repository".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-branch.html>
pub struct Branch_ {
    pub app_id: crate::value::ExpString,
    pub backend: Option<super::amplify::branch::Backend_>,
    pub basic_auth_config: Option<super::amplify::branch::BasicAuthConfig_>,
    pub branch_name: crate::value::ExpString,
    pub build_spec: Option<crate::value::ExpString>,
    pub compute_role_arn: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub enable_auto_build: Option<crate::value::ExpBool>,
    pub enable_performance_mode: Option<crate::value::ExpBool>,
    pub enable_pull_request_preview: Option<crate::value::ExpBool>,
    pub enable_skew_protection: Option<crate::value::ExpBool>,
    pub environment_variables: Option<Vec<super::amplify::branch::EnvironmentVariable_>>,
    pub framework: Option<crate::value::ExpString>,
    pub pull_request_environment_name: Option<crate::value::ExpString>,
    pub stage: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_amplify_Branch {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Amplify::Branch" $($field
        $value)*)
    };
}
pub use crate::__aws_amplify_Branch as Branch;
impl crate::template::ToResource for Branch_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Amplify"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Branch"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AppId".to_string(),
            crate::value::ToValue::to_value(&self.app_id),
        );
        if let Some(ref value) = self.backend {
            properties.insert(
                "Backend".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.basic_auth_config {
            properties.insert(
                "BasicAuthConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "BranchName".to_string(),
            crate::value::ToValue::to_value(&self.branch_name),
        );
        if let Some(ref value) = self.build_spec {
            properties.insert(
                "BuildSpec".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.compute_role_arn {
            properties.insert(
                "ComputeRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enable_auto_build {
            properties.insert(
                "EnableAutoBuild".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enable_performance_mode {
            properties.insert(
                "EnablePerformanceMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enable_pull_request_preview {
            properties.insert(
                "EnablePullRequestPreview".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enable_skew_protection {
            properties.insert(
                "EnableSkewProtection".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.environment_variables {
            properties.insert(
                "EnvironmentVariables".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.framework {
            properties.insert(
                "Framework".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.pull_request_environment_name {
            properties.insert(
                "PullRequestEnvironmentName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.stage {
            properties.insert("Stage".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplify-domain.html>
pub struct Domain_ {
    pub app_id: crate::value::ExpString,
    pub auto_sub_domain_creation_patterns: Option<Vec<crate::value::ExpString>>,
    pub auto_sub_domain_iam_role: Option<crate::value::ExpString>,
    pub certificate_settings: Option<super::amplify::domain::CertificateSettings_>,
    pub domain_name: crate::value::ExpString,
    pub enable_auto_sub_domain: Option<crate::value::ExpBool>,
    pub sub_domain_settings: Vec<super::amplify::domain::SubDomainSetting_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_amplify_Domain {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Amplify::Domain" $($field
        $value)*)
    };
}
pub use crate::__aws_amplify_Domain as Domain;
impl crate::template::ToResource for Domain_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Amplify"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Domain"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AppId".to_string(),
            crate::value::ToValue::to_value(&self.app_id),
        );
        if let Some(ref value) = self.auto_sub_domain_creation_patterns {
            properties.insert(
                "AutoSubDomainCreationPatterns".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.auto_sub_domain_iam_role {
            properties.insert(
                "AutoSubDomainIAMRole".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.certificate_settings {
            properties.insert(
                "CertificateSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DomainName".to_string(),
            crate::value::ToValue::to_value(&self.domain_name),
        );
        if let Some(ref value) = self.enable_auto_sub_domain {
            properties.insert(
                "EnableAutoSubDomain".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SubDomainSettings".to_string(),
            crate::value::ToValue::to_value(&self.sub_domain_settings),
        );
        properties
    }
}
