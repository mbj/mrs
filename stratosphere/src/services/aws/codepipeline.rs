pub mod customactiontype {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-customactiontype-artifactdetails.html
    pub struct ArtifactDetails_ {
        pub maximum_count: i64,
        pub minimum_count: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codepipeline_CustomActionType_ArtifactDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodePipeline::CustomActionType.ArtifactDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codepipeline_CustomActionType_ArtifactDetails as ArtifactDetails;
    impl crate::value::ToValue for ArtifactDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MaximumCount".to_string(),
                crate::value::ToValue::to_value(&self.maximum_count),
            );
            properties.insert(
                "MinimumCount".to_string(),
                crate::value::ToValue::to_value(&self.minimum_count),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-customactiontype-configurationproperties.html
    pub struct ConfigurationProperties_ {
        pub description: Option<crate::value::ExpString>,
        pub key: crate::value::ExpBool,
        pub name: crate::value::ExpString,
        pub queryable: Option<crate::value::ExpBool>,
        pub required: crate::value::ExpBool,
        pub secret: crate::value::ExpBool,
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codepipeline_CustomActionType_ConfigurationProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodePipeline::CustomActionType.ConfigurationProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codepipeline_CustomActionType_ConfigurationProperties as ConfigurationProperties;
    impl crate::value::ToValue for ConfigurationProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.queryable {
                properties.insert(
                    "Queryable".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Required".to_string(),
                crate::value::ToValue::to_value(&self.required),
            );
            properties.insert(
                "Secret".to_string(),
                crate::value::ToValue::to_value(&self.secret),
            );
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-customactiontype-settings.html
    pub struct Settings_ {
        pub entity_url_template: Option<crate::value::ExpString>,
        pub execution_url_template: Option<crate::value::ExpString>,
        pub revision_url_template: Option<crate::value::ExpString>,
        pub third_party_configuration_url: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codepipeline_CustomActionType_Settings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodePipeline::CustomActionType.Settings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codepipeline_CustomActionType_Settings as Settings;
    impl crate::value::ToValue for Settings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.entity_url_template {
                properties.insert(
                    "EntityUrlTemplate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.execution_url_template {
                properties.insert(
                    "ExecutionUrlTemplate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.revision_url_template {
                properties.insert(
                    "RevisionUrlTemplate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.third_party_configuration_url {
                properties.insert(
                    "ThirdPartyConfigurationUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod pipeline {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-actiondeclaration.html
    pub struct ActionDeclaration_ {
        pub action_type_id: Box<ActionTypeId_>,
        pub commands: Option<Vec<crate::value::ExpString>>,
        pub configuration: Option<serde_json::Value>,
        pub environment_variables: Option<Vec<EnvironmentVariable_>>,
        pub input_artifacts: Option<Vec<InputArtifact_>>,
        pub name: crate::value::ExpString,
        pub namespace: Option<crate::value::ExpString>,
        pub output_artifacts: Option<Vec<OutputArtifact_>>,
        pub output_variables: Option<Vec<crate::value::ExpString>>,
        pub region: Option<crate::value::ExpString>,
        pub role_arn: Option<crate::value::ExpString>,
        pub run_order: Option<i64>,
        pub timeout_in_minutes: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codepipeline_Pipeline_ActionDeclaration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodePipeline::Pipeline.ActionDeclaration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codepipeline_Pipeline_ActionDeclaration as ActionDeclaration;
    impl crate::value::ToValue for ActionDeclaration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ActionTypeId".to_string(),
                crate::value::ToValue::to_value(&self.action_type_id),
            );
            if let Some(ref value) = self.commands {
                properties.insert(
                    "Commands".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.configuration {
                properties.insert(
                    "Configuration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.environment_variables {
                properties.insert(
                    "EnvironmentVariables".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_artifacts {
                properties.insert(
                    "InputArtifacts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.namespace {
                properties.insert(
                    "Namespace".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.output_artifacts {
                properties.insert(
                    "OutputArtifacts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.output_variables {
                properties.insert(
                    "OutputVariables".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.region {
                properties.insert("Region".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.role_arn {
                properties.insert(
                    "RoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.run_order {
                properties.insert(
                    "RunOrder".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timeout_in_minutes {
                properties.insert(
                    "TimeoutInMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-actiontypeid.html
    pub struct ActionTypeId_ {
        pub category: crate::value::ExpString,
        pub owner: crate::value::ExpString,
        pub provider: crate::value::ExpString,
        pub version: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codepipeline_Pipeline_ActionTypeId {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodePipeline::Pipeline.ActionTypeId"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codepipeline_Pipeline_ActionTypeId as ActionTypeId;
    impl crate::value::ToValue for ActionTypeId_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Category".to_string(),
                crate::value::ToValue::to_value(&self.category),
            );
            properties.insert(
                "Owner".to_string(),
                crate::value::ToValue::to_value(&self.owner),
            );
            properties.insert(
                "Provider".to_string(),
                crate::value::ToValue::to_value(&self.provider),
            );
            properties.insert(
                "Version".to_string(),
                crate::value::ToValue::to_value(&self.version),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-artifactstore.html
    pub struct ArtifactStore_ {
        pub encryption_key: Option<Box<EncryptionKey_>>,
        pub location: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codepipeline_Pipeline_ArtifactStore {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodePipeline::Pipeline.ArtifactStore"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codepipeline_Pipeline_ArtifactStore as ArtifactStore;
    impl crate::value::ToValue for ArtifactStore_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.encryption_key {
                properties.insert(
                    "EncryptionKey".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Location".to_string(),
                crate::value::ToValue::to_value(&self.location),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-artifactstoremap.html
    pub struct ArtifactStoreMap_ {
        pub artifact_store: Box<ArtifactStore_>,
        pub region: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codepipeline_Pipeline_ArtifactStoreMap {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodePipeline::Pipeline.ArtifactStoreMap"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codepipeline_Pipeline_ArtifactStoreMap as ArtifactStoreMap;
    impl crate::value::ToValue for ArtifactStoreMap_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ArtifactStore".to_string(),
                crate::value::ToValue::to_value(&self.artifact_store),
            );
            properties.insert(
                "Region".to_string(),
                crate::value::ToValue::to_value(&self.region),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-beforeentryconditions.html
    pub struct BeforeEntryConditions_ {
        pub conditions: Option<Vec<Condition_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codepipeline_Pipeline_BeforeEntryConditions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodePipeline::Pipeline.BeforeEntryConditions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codepipeline_Pipeline_BeforeEntryConditions as BeforeEntryConditions;
    impl crate::value::ToValue for BeforeEntryConditions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.conditions {
                properties.insert(
                    "Conditions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-blockerdeclaration.html
    pub struct BlockerDeclaration_ {
        pub name: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codepipeline_Pipeline_BlockerDeclaration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodePipeline::Pipeline.BlockerDeclaration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codepipeline_Pipeline_BlockerDeclaration as BlockerDeclaration;
    impl crate::value::ToValue for BlockerDeclaration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-condition.html
    pub struct Condition_ {
        pub result: Option<crate::value::ExpString>,
        pub rules: Option<Vec<RuleDeclaration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codepipeline_Pipeline_Condition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodePipeline::Pipeline.Condition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codepipeline_Pipeline_Condition as Condition;
    impl crate::value::ToValue for Condition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.result {
                properties.insert("Result".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.rules {
                properties.insert("Rules".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-encryptionkey.html
    pub struct EncryptionKey_ {
        pub id: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codepipeline_Pipeline_EncryptionKey {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodePipeline::Pipeline.EncryptionKey"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codepipeline_Pipeline_EncryptionKey as EncryptionKey;
    impl crate::value::ToValue for EncryptionKey_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-environmentvariable.html
    pub struct EnvironmentVariable_ {
        pub name: crate::value::ExpString,
        pub r#type: Option<crate::value::ExpString>,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codepipeline_Pipeline_EnvironmentVariable {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodePipeline::Pipeline.EnvironmentVariable"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codepipeline_Pipeline_EnvironmentVariable as EnvironmentVariable;
    impl crate::value::ToValue for EnvironmentVariable_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-failureconditions.html
    pub struct FailureConditions_ {
        pub conditions: Option<Vec<Condition_>>,
        pub result: Option<crate::value::ExpString>,
        pub retry_configuration: Option<Box<RetryConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codepipeline_Pipeline_FailureConditions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodePipeline::Pipeline.FailureConditions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codepipeline_Pipeline_FailureConditions as FailureConditions;
    impl crate::value::ToValue for FailureConditions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.conditions {
                properties.insert(
                    "Conditions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.result {
                properties.insert("Result".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.retry_configuration {
                properties.insert(
                    "RetryConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-gitbranchfiltercriteria.html
    pub struct GitBranchFilterCriteria_ {
        pub excludes: Option<Vec<crate::value::ExpString>>,
        pub includes: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codepipeline_Pipeline_GitBranchFilterCriteria {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodePipeline::Pipeline.GitBranchFilterCriteria"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codepipeline_Pipeline_GitBranchFilterCriteria as GitBranchFilterCriteria;
    impl crate::value::ToValue for GitBranchFilterCriteria_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.excludes {
                properties.insert(
                    "Excludes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.includes {
                properties.insert(
                    "Includes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-gitconfiguration.html
    pub struct GitConfiguration_ {
        pub pull_request: Option<Vec<GitPullRequestFilter_>>,
        pub push: Option<Vec<GitPushFilter_>>,
        pub source_action_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codepipeline_Pipeline_GitConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodePipeline::Pipeline.GitConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codepipeline_Pipeline_GitConfiguration as GitConfiguration;
    impl crate::value::ToValue for GitConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.pull_request {
                properties.insert(
                    "PullRequest".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.push {
                properties.insert("Push".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "SourceActionName".to_string(),
                crate::value::ToValue::to_value(&self.source_action_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-gitfilepathfiltercriteria.html
    pub struct GitFilePathFilterCriteria_ {
        pub excludes: Option<Vec<crate::value::ExpString>>,
        pub includes: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codepipeline_Pipeline_GitFilePathFilterCriteria {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodePipeline::Pipeline.GitFilePathFilterCriteria"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codepipeline_Pipeline_GitFilePathFilterCriteria as GitFilePathFilterCriteria;
    impl crate::value::ToValue for GitFilePathFilterCriteria_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.excludes {
                properties.insert(
                    "Excludes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.includes {
                properties.insert(
                    "Includes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-gitpullrequestfilter.html
    pub struct GitPullRequestFilter_ {
        pub branches: Option<Box<GitBranchFilterCriteria_>>,
        pub events: Option<Vec<crate::value::ExpString>>,
        pub file_paths: Option<Box<GitFilePathFilterCriteria_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codepipeline_Pipeline_GitPullRequestFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodePipeline::Pipeline.GitPullRequestFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codepipeline_Pipeline_GitPullRequestFilter as GitPullRequestFilter;
    impl crate::value::ToValue for GitPullRequestFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.branches {
                properties.insert(
                    "Branches".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.events {
                properties.insert("Events".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.file_paths {
                properties.insert(
                    "FilePaths".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-gitpushfilter.html
    pub struct GitPushFilter_ {
        pub branches: Option<Box<GitBranchFilterCriteria_>>,
        pub file_paths: Option<Box<GitFilePathFilterCriteria_>>,
        pub tags: Option<Box<GitTagFilterCriteria_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codepipeline_Pipeline_GitPushFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodePipeline::Pipeline.GitPushFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codepipeline_Pipeline_GitPushFilter as GitPushFilter;
    impl crate::value::ToValue for GitPushFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.branches {
                properties.insert(
                    "Branches".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.file_paths {
                properties.insert(
                    "FilePaths".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tags {
                properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-gittagfiltercriteria.html
    pub struct GitTagFilterCriteria_ {
        pub excludes: Option<Vec<crate::value::ExpString>>,
        pub includes: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codepipeline_Pipeline_GitTagFilterCriteria {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodePipeline::Pipeline.GitTagFilterCriteria"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codepipeline_Pipeline_GitTagFilterCriteria as GitTagFilterCriteria;
    impl crate::value::ToValue for GitTagFilterCriteria_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.excludes {
                properties.insert(
                    "Excludes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.includes {
                properties.insert(
                    "Includes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-inputartifact.html
    pub struct InputArtifact_ {
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codepipeline_Pipeline_InputArtifact {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodePipeline::Pipeline.InputArtifact"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codepipeline_Pipeline_InputArtifact as InputArtifact;
    impl crate::value::ToValue for InputArtifact_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-outputartifact.html
    pub struct OutputArtifact_ {
        pub files: Option<Vec<crate::value::ExpString>>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codepipeline_Pipeline_OutputArtifact {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodePipeline::Pipeline.OutputArtifact"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codepipeline_Pipeline_OutputArtifact as OutputArtifact;
    impl crate::value::ToValue for OutputArtifact_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.files {
                properties.insert("Files".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-pipelinetriggerdeclaration.html
    pub struct PipelineTriggerDeclaration_ {
        pub git_configuration: Option<Box<GitConfiguration_>>,
        pub provider_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codepipeline_Pipeline_PipelineTriggerDeclaration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodePipeline::Pipeline.PipelineTriggerDeclaration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codepipeline_Pipeline_PipelineTriggerDeclaration as PipelineTriggerDeclaration;
    impl crate::value::ToValue for PipelineTriggerDeclaration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.git_configuration {
                properties.insert(
                    "GitConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ProviderType".to_string(),
                crate::value::ToValue::to_value(&self.provider_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-retryconfiguration.html
    pub struct RetryConfiguration_ {
        pub retry_mode: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codepipeline_Pipeline_RetryConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodePipeline::Pipeline.RetryConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codepipeline_Pipeline_RetryConfiguration as RetryConfiguration;
    impl crate::value::ToValue for RetryConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.retry_mode {
                properties.insert(
                    "RetryMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-ruledeclaration.html
    pub struct RuleDeclaration_ {
        pub commands: Option<Vec<crate::value::ExpString>>,
        pub configuration: Option<serde_json::Value>,
        pub input_artifacts: Option<Vec<InputArtifact_>>,
        pub name: Option<crate::value::ExpString>,
        pub region: Option<crate::value::ExpString>,
        pub role_arn: Option<crate::value::ExpString>,
        pub rule_type_id: Option<Box<RuleTypeId_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codepipeline_Pipeline_RuleDeclaration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodePipeline::Pipeline.RuleDeclaration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codepipeline_Pipeline_RuleDeclaration as RuleDeclaration;
    impl crate::value::ToValue for RuleDeclaration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.commands {
                properties.insert(
                    "Commands".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.configuration {
                properties.insert(
                    "Configuration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_artifacts {
                properties.insert(
                    "InputArtifacts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.region {
                properties.insert("Region".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.role_arn {
                properties.insert(
                    "RoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rule_type_id {
                properties.insert(
                    "RuleTypeId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-ruletypeid.html
    pub struct RuleTypeId_ {
        pub category: Option<crate::value::ExpString>,
        pub owner: Option<crate::value::ExpString>,
        pub provider: Option<crate::value::ExpString>,
        pub version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codepipeline_Pipeline_RuleTypeId {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodePipeline::Pipeline.RuleTypeId"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codepipeline_Pipeline_RuleTypeId as RuleTypeId;
    impl crate::value::ToValue for RuleTypeId_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.category {
                properties.insert(
                    "Category".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.owner {
                properties.insert("Owner".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.provider {
                properties.insert(
                    "Provider".to_string(),
                    crate::value::ToValue::to_value(value),
                );
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-stagedeclaration.html
    pub struct StageDeclaration_ {
        pub actions: Vec<ActionDeclaration_>,
        pub before_entry: Option<Box<BeforeEntryConditions_>>,
        pub blockers: Option<Vec<BlockerDeclaration_>>,
        pub name: crate::value::ExpString,
        pub on_failure: Option<Box<FailureConditions_>>,
        pub on_success: Option<Box<SuccessConditions_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codepipeline_Pipeline_StageDeclaration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodePipeline::Pipeline.StageDeclaration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codepipeline_Pipeline_StageDeclaration as StageDeclaration;
    impl crate::value::ToValue for StageDeclaration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Actions".to_string(),
                crate::value::ToValue::to_value(&self.actions),
            );
            if let Some(ref value) = self.before_entry {
                properties.insert(
                    "BeforeEntry".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.blockers {
                properties.insert(
                    "Blockers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-stagetransition.html
    pub struct StageTransition_ {
        pub reason: crate::value::ExpString,
        pub stage_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codepipeline_Pipeline_StageTransition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodePipeline::Pipeline.StageTransition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codepipeline_Pipeline_StageTransition as StageTransition;
    impl crate::value::ToValue for StageTransition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Reason".to_string(),
                crate::value::ToValue::to_value(&self.reason),
            );
            properties.insert(
                "StageName".to_string(),
                crate::value::ToValue::to_value(&self.stage_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-successconditions.html
    pub struct SuccessConditions_ {
        pub conditions: Option<Vec<Condition_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codepipeline_Pipeline_SuccessConditions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodePipeline::Pipeline.SuccessConditions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codepipeline_Pipeline_SuccessConditions as SuccessConditions;
    impl crate::value::ToValue for SuccessConditions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.conditions {
                properties.insert(
                    "Conditions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-variabledeclaration.html
    pub struct VariableDeclaration_ {
        pub default_value: Option<crate::value::ExpString>,
        pub description: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codepipeline_Pipeline_VariableDeclaration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodePipeline::Pipeline.VariableDeclaration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codepipeline_Pipeline_VariableDeclaration as VariableDeclaration;
    impl crate::value::ToValue for VariableDeclaration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.default_value {
                properties.insert(
                    "DefaultValue".to_string(),
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
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
}
pub mod webhook {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-webhook-webhookauthconfiguration.html
    pub struct WebhookAuthConfiguration_ {
        pub allowed_ip_range: Option<crate::value::ExpString>,
        pub secret_token: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codepipeline_Webhook_WebhookAuthConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodePipeline::Webhook.WebhookAuthConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codepipeline_Webhook_WebhookAuthConfiguration as WebhookAuthConfiguration;
    impl crate::value::ToValue for WebhookAuthConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allowed_ip_range {
                properties.insert(
                    "AllowedIPRange".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secret_token {
                properties.insert(
                    "SecretToken".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-webhook-webhookfilterrule.html
    pub struct WebhookFilterRule_ {
        pub json_path: crate::value::ExpString,
        pub match_equals: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codepipeline_Webhook_WebhookFilterRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodePipeline::Webhook.WebhookFilterRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codepipeline_Webhook_WebhookFilterRule as WebhookFilterRule;
    impl crate::value::ToValue for WebhookFilterRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "JsonPath".to_string(),
                crate::value::ToValue::to_value(&self.json_path),
            );
            if let Some(ref value) = self.match_equals {
                properties.insert(
                    "MatchEquals".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codepipeline-customactiontype.html
pub struct CustomActionType_ {
    pub category: crate::value::ExpString,
    pub configuration_properties:
        Option<Vec<super::codepipeline::customactiontype::ConfigurationProperties_>>,
    pub input_artifact_details: super::codepipeline::customactiontype::ArtifactDetails_,
    pub output_artifact_details: super::codepipeline::customactiontype::ArtifactDetails_,
    pub provider: crate::value::ExpString,
    pub settings: Option<super::codepipeline::customactiontype::Settings_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub version: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_codepipeline_CustomActionType {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CodePipeline::CustomActionType"
        $($field $value)*)
    };
}
pub use crate::__aws_codepipeline_CustomActionType as CustomActionType;
impl crate::template::ToResource for CustomActionType_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CodePipeline"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("CustomActionType"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Category".to_string(),
            crate::value::ToValue::to_value(&self.category),
        );
        if let Some(ref value) = self.configuration_properties {
            properties.insert(
                "ConfigurationProperties".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "InputArtifactDetails".to_string(),
            crate::value::ToValue::to_value(&self.input_artifact_details),
        );
        properties.insert(
            "OutputArtifactDetails".to_string(),
            crate::value::ToValue::to_value(&self.output_artifact_details),
        );
        properties.insert(
            "Provider".to_string(),
            crate::value::ToValue::to_value(&self.provider),
        );
        if let Some(ref value) = self.settings {
            properties.insert(
                "Settings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Version".to_string(),
            crate::value::ToValue::to_value(&self.version),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codepipeline-pipeline.html
pub struct Pipeline_ {
    pub artifact_store: Option<super::codepipeline::pipeline::ArtifactStore_>,
    pub artifact_stores: Option<Vec<super::codepipeline::pipeline::ArtifactStoreMap_>>,
    pub disable_inbound_stage_transitions:
        Option<Vec<super::codepipeline::pipeline::StageTransition_>>,
    pub execution_mode: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub pipeline_type: Option<crate::value::ExpString>,
    pub restart_execution_on_update: Option<crate::value::ExpBool>,
    pub role_arn: crate::value::ExpString,
    pub stages: Vec<super::codepipeline::pipeline::StageDeclaration_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub triggers: Option<Vec<super::codepipeline::pipeline::PipelineTriggerDeclaration_>>,
    pub variables: Option<Vec<super::codepipeline::pipeline::VariableDeclaration_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_codepipeline_Pipeline {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CodePipeline::Pipeline"
        $($field $value)*)
    };
}
pub use crate::__aws_codepipeline_Pipeline as Pipeline;
impl crate::template::ToResource for Pipeline_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CodePipeline"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Pipeline"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.artifact_store {
            properties.insert(
                "ArtifactStore".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.artifact_stores {
            properties.insert(
                "ArtifactStores".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.disable_inbound_stage_transitions {
            properties.insert(
                "DisableInboundStageTransitions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.execution_mode {
            properties.insert(
                "ExecutionMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.pipeline_type {
            properties.insert(
                "PipelineType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.restart_execution_on_update {
            properties.insert(
                "RestartExecutionOnUpdate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        properties.insert(
            "Stages".to_string(),
            crate::value::ToValue::to_value(&self.stages),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.triggers {
            properties.insert(
                "Triggers".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.variables {
            properties.insert(
                "Variables".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codepipeline-webhook.html
pub struct Webhook_ {
    pub authentication: crate::value::ExpString,
    pub authentication_configuration: super::codepipeline::webhook::WebhookAuthConfiguration_,
    pub filters: Vec<super::codepipeline::webhook::WebhookFilterRule_>,
    pub name: Option<crate::value::ExpString>,
    pub register_with_third_party: Option<crate::value::ExpBool>,
    pub target_action: crate::value::ExpString,
    pub target_pipeline: crate::value::ExpString,
    pub target_pipeline_version: Option<i64>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_codepipeline_Webhook {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CodePipeline::Webhook"
        $($field $value)*)
    };
}
pub use crate::__aws_codepipeline_Webhook as Webhook;
impl crate::template::ToResource for Webhook_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CodePipeline"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Webhook"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Authentication".to_string(),
            crate::value::ToValue::to_value(&self.authentication),
        );
        properties.insert(
            "AuthenticationConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.authentication_configuration),
        );
        properties.insert(
            "Filters".to_string(),
            crate::value::ToValue::to_value(&self.filters),
        );
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.register_with_third_party {
            properties.insert(
                "RegisterWithThirdParty".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "TargetAction".to_string(),
            crate::value::ToValue::to_value(&self.target_action),
        );
        properties.insert(
            "TargetPipeline".to_string(),
            crate::value::ToValue::to_value(&self.target_pipeline),
        );
        if let Some(ref value) = self.target_pipeline_version {
            properties.insert(
                "TargetPipelineVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
