pub mod guardhook {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-guardhook-hooktarget.html
    pub struct HookTarget_ {
        pub action: crate::value::ExpString,
        pub invocation_point: crate::value::ExpString,
        pub target_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudformation_GuardHook_HookTarget {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudFormation::GuardHook.HookTarget"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudformation_GuardHook_HookTarget as HookTarget;
    impl crate::value::ToValue for HookTarget_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Action".to_string(),
                crate::value::ToValue::to_value(&self.action),
            );
            properties.insert(
                "InvocationPoint".to_string(),
                crate::value::ToValue::to_value(&self.invocation_point),
            );
            properties.insert(
                "TargetName".to_string(),
                crate::value::ToValue::to_value(&self.target_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-guardhook-options.html
    pub struct Options_ {
        pub input_params: Option<Box<S3Location_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudformation_GuardHook_Options {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudFormation::GuardHook.Options"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudformation_GuardHook_Options as Options;
    impl crate::value::ToValue for Options_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.input_params {
                properties.insert(
                    "InputParams".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-guardhook-s3location.html
    pub struct S3Location_ {
        pub uri: crate::value::ExpString,
        pub version_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudformation_GuardHook_S3Location {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudFormation::GuardHook.S3Location"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudformation_GuardHook_S3Location as S3Location;
    impl crate::value::ToValue for S3Location_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Uri".to_string(),
                crate::value::ToValue::to_value(&self.uri),
            );
            if let Some(ref value) = self.version_id {
                properties.insert(
                    "VersionId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-guardhook-stackfilters.html
    pub struct StackFilters_ {
        pub filtering_criteria: crate::value::ExpString,
        pub stack_names: Option<Box<StackNames_>>,
        pub stack_roles: Option<Box<StackRoles_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudformation_GuardHook_StackFilters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudFormation::GuardHook.StackFilters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudformation_GuardHook_StackFilters as StackFilters;
    impl crate::value::ToValue for StackFilters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FilteringCriteria".to_string(),
                crate::value::ToValue::to_value(&self.filtering_criteria),
            );
            if let Some(ref value) = self.stack_names {
                properties.insert(
                    "StackNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stack_roles {
                properties.insert(
                    "StackRoles".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-guardhook-stacknames.html
    pub struct StackNames_ {
        pub exclude: Option<Vec<crate::value::ExpString>>,
        pub include: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudformation_GuardHook_StackNames {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudFormation::GuardHook.StackNames"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudformation_GuardHook_StackNames as StackNames;
    impl crate::value::ToValue for StackNames_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.exclude {
                properties.insert(
                    "Exclude".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include {
                properties.insert(
                    "Include".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-guardhook-stackroles.html
    pub struct StackRoles_ {
        pub exclude: Option<Vec<crate::value::ExpString>>,
        pub include: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudformation_GuardHook_StackRoles {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudFormation::GuardHook.StackRoles"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudformation_GuardHook_StackRoles as StackRoles;
    impl crate::value::ToValue for StackRoles_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.exclude {
                properties.insert(
                    "Exclude".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include {
                properties.insert(
                    "Include".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-guardhook-targetfilters.html
    pub struct TargetFilters_ {
        pub actions: Option<Vec<crate::value::ExpString>>,
        pub invocation_points: Option<Vec<crate::value::ExpString>>,
        pub target_names: Option<Vec<crate::value::ExpString>>,
        pub targets: Option<Vec<HookTarget_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudformation_GuardHook_TargetFilters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudFormation::GuardHook.TargetFilters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudformation_GuardHook_TargetFilters as TargetFilters;
    impl crate::value::ToValue for TargetFilters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.actions {
                properties.insert(
                    "Actions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.invocation_points {
                properties.insert(
                    "InvocationPoints".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_names {
                properties.insert(
                    "TargetNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.targets {
                properties.insert(
                    "Targets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod hookversion {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-hookversion-loggingconfig.html
    pub struct LoggingConfig_ {
        pub log_group_name: Option<crate::value::ExpString>,
        pub log_role_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudformation_HookVersion_LoggingConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudFormation::HookVersion.LoggingConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudformation_HookVersion_LoggingConfig as LoggingConfig;
    impl crate::value::ToValue for LoggingConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.log_group_name {
                properties.insert(
                    "LogGroupName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.log_role_arn {
                properties.insert(
                    "LogRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod lambdahook {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-lambdahook-hooktarget.html
    pub struct HookTarget_ {
        pub action: crate::value::ExpString,
        pub invocation_point: crate::value::ExpString,
        pub target_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudformation_LambdaHook_HookTarget {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudFormation::LambdaHook.HookTarget"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudformation_LambdaHook_HookTarget as HookTarget;
    impl crate::value::ToValue for HookTarget_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Action".to_string(),
                crate::value::ToValue::to_value(&self.action),
            );
            properties.insert(
                "InvocationPoint".to_string(),
                crate::value::ToValue::to_value(&self.invocation_point),
            );
            properties.insert(
                "TargetName".to_string(),
                crate::value::ToValue::to_value(&self.target_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-lambdahook-stackfilters.html
    pub struct StackFilters_ {
        pub filtering_criteria: crate::value::ExpString,
        pub stack_names: Option<Box<StackNames_>>,
        pub stack_roles: Option<Box<StackRoles_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudformation_LambdaHook_StackFilters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudFormation::LambdaHook.StackFilters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudformation_LambdaHook_StackFilters as StackFilters;
    impl crate::value::ToValue for StackFilters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FilteringCriteria".to_string(),
                crate::value::ToValue::to_value(&self.filtering_criteria),
            );
            if let Some(ref value) = self.stack_names {
                properties.insert(
                    "StackNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stack_roles {
                properties.insert(
                    "StackRoles".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-lambdahook-stacknames.html
    pub struct StackNames_ {
        pub exclude: Option<Vec<crate::value::ExpString>>,
        pub include: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudformation_LambdaHook_StackNames {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudFormation::LambdaHook.StackNames"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudformation_LambdaHook_StackNames as StackNames;
    impl crate::value::ToValue for StackNames_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.exclude {
                properties.insert(
                    "Exclude".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include {
                properties.insert(
                    "Include".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-lambdahook-stackroles.html
    pub struct StackRoles_ {
        pub exclude: Option<Vec<crate::value::ExpString>>,
        pub include: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudformation_LambdaHook_StackRoles {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudFormation::LambdaHook.StackRoles"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudformation_LambdaHook_StackRoles as StackRoles;
    impl crate::value::ToValue for StackRoles_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.exclude {
                properties.insert(
                    "Exclude".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include {
                properties.insert(
                    "Include".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-lambdahook-targetfilters.html
    pub struct TargetFilters_ {
        pub actions: Option<Vec<crate::value::ExpString>>,
        pub invocation_points: Option<Vec<crate::value::ExpString>>,
        pub target_names: Option<Vec<crate::value::ExpString>>,
        pub targets: Option<Vec<HookTarget_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudformation_LambdaHook_TargetFilters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudFormation::LambdaHook.TargetFilters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudformation_LambdaHook_TargetFilters as TargetFilters;
    impl crate::value::ToValue for TargetFilters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.actions {
                properties.insert(
                    "Actions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.invocation_points {
                properties.insert(
                    "InvocationPoints".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_names {
                properties.insert(
                    "TargetNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.targets {
                properties.insert(
                    "Targets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod resourceversion {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-resourceversion-loggingconfig.html
    pub struct LoggingConfig_ {
        pub log_group_name: Option<crate::value::ExpString>,
        pub log_role_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudformation_ResourceVersion_LoggingConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudFormation::ResourceVersion.LoggingConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudformation_ResourceVersion_LoggingConfig as LoggingConfig;
    impl crate::value::ToValue for LoggingConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.log_group_name {
                properties.insert(
                    "LogGroupName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.log_role_arn {
                properties.insert(
                    "LogRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod stackset {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-stackset-autodeployment.html
    pub struct AutoDeployment_ {
        pub depends_on: Option<Vec<crate::value::ExpString>>,
        pub enabled: Option<crate::value::ExpBool>,
        pub retain_stacks_on_account_removal: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudformation_StackSet_AutoDeployment {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudFormation::StackSet.AutoDeployment"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudformation_StackSet_AutoDeployment as AutoDeployment;
    impl crate::value::ToValue for AutoDeployment_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.depends_on {
                properties.insert(
                    "DependsOn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.retain_stacks_on_account_removal {
                properties.insert(
                    "RetainStacksOnAccountRemoval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-stackset-deploymenttargets.html
    pub struct DeploymentTargets_ {
        pub account_filter_type: Option<crate::value::ExpString>,
        pub accounts: Option<Vec<crate::value::ExpString>>,
        pub accounts_url: Option<crate::value::ExpString>,
        pub organizational_unit_ids: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudformation_StackSet_DeploymentTargets {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudFormation::StackSet.DeploymentTargets"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudformation_StackSet_DeploymentTargets as DeploymentTargets;
    impl crate::value::ToValue for DeploymentTargets_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.account_filter_type {
                properties.insert(
                    "AccountFilterType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.accounts {
                properties.insert(
                    "Accounts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.accounts_url {
                properties.insert(
                    "AccountsUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.organizational_unit_ids {
                properties.insert(
                    "OrganizationalUnitIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-stackset-managedexecution.html
    pub struct ManagedExecution_ {
        pub active: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudformation_StackSet_ManagedExecution {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudFormation::StackSet.ManagedExecution"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudformation_StackSet_ManagedExecution as ManagedExecution;
    impl crate::value::ToValue for ManagedExecution_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.active {
                properties.insert("Active".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-stackset-operationpreferences.html
    pub struct OperationPreferences_ {
        pub concurrency_mode: Option<crate::value::ExpString>,
        pub failure_tolerance_count: Option<i32>,
        pub failure_tolerance_percentage: Option<i32>,
        pub max_concurrent_count: Option<i32>,
        pub max_concurrent_percentage: Option<i32>,
        pub region_concurrency_type: Option<crate::value::ExpString>,
        pub region_order: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudformation_StackSet_OperationPreferences {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudFormation::StackSet.OperationPreferences"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudformation_StackSet_OperationPreferences as OperationPreferences;
    impl crate::value::ToValue for OperationPreferences_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.concurrency_mode {
                properties.insert(
                    "ConcurrencyMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.failure_tolerance_count {
                properties.insert(
                    "FailureToleranceCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.failure_tolerance_percentage {
                properties.insert(
                    "FailureTolerancePercentage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_concurrent_count {
                properties.insert(
                    "MaxConcurrentCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_concurrent_percentage {
                properties.insert(
                    "MaxConcurrentPercentage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.region_concurrency_type {
                properties.insert(
                    "RegionConcurrencyType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.region_order {
                properties.insert(
                    "RegionOrder".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-stackset-parameter.html
    pub struct Parameter_ {
        pub parameter_key: crate::value::ExpString,
        pub parameter_value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudformation_StackSet_Parameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudFormation::StackSet.Parameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudformation_StackSet_Parameter as Parameter;
    impl crate::value::ToValue for Parameter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ParameterKey".to_string(),
                crate::value::ToValue::to_value(&self.parameter_key),
            );
            properties.insert(
                "ParameterValue".to_string(),
                crate::value::ToValue::to_value(&self.parameter_value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-stackset-stackinstances.html
    pub struct StackInstances_ {
        pub deployment_targets: Box<DeploymentTargets_>,
        pub parameter_overrides: Option<Vec<Parameter_>>,
        pub regions: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudformation_StackSet_StackInstances {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudFormation::StackSet.StackInstances"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudformation_StackSet_StackInstances as StackInstances;
    impl crate::value::ToValue for StackInstances_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DeploymentTargets".to_string(),
                crate::value::ToValue::to_value(&self.deployment_targets),
            );
            if let Some(ref value) = self.parameter_overrides {
                properties.insert(
                    "ParameterOverrides".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Regions".to_string(),
                crate::value::ToValue::to_value(&self.regions),
            );
            properties.into()
        }
    }
}
pub mod typeactivation {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudformation-typeactivation-loggingconfig.html
    pub struct LoggingConfig_ {
        pub log_group_name: Option<crate::value::ExpString>,
        pub log_role_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudformation_TypeActivation_LoggingConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudFormation::TypeActivation.LoggingConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudformation_TypeActivation_LoggingConfig as LoggingConfig;
    impl crate::value::ToValue for LoggingConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.log_group_name {
                properties.insert(
                    "LogGroupName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.log_role_arn {
                properties.insert(
                    "LogRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cfn-customresource.html
pub struct CustomResource_ {
    pub service_timeout: Option<i32>,
    pub service_token: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudformation_CustomResource {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CloudFormation::CustomResource"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudformation_CustomResource as CustomResource;
impl crate::template::ToResource for CustomResource_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudFormation"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("CustomResource"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.service_timeout {
            properties.insert(
                "ServiceTimeout".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ServiceToken".to_string(),
            crate::value::ToValue::to_value(&self.service_token),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-guardhook.html
pub struct GuardHook_ {
    pub alias: crate::value::ExpString,
    pub execution_role: crate::value::ExpString,
    pub failure_mode: crate::value::ExpString,
    pub hook_status: crate::value::ExpString,
    pub log_bucket: Option<crate::value::ExpString>,
    pub options: Option<super::cloudformation::guardhook::Options_>,
    pub rule_location: super::cloudformation::guardhook::S3Location_,
    pub stack_filters: Option<super::cloudformation::guardhook::StackFilters_>,
    pub target_filters: Option<super::cloudformation::guardhook::TargetFilters_>,
    pub target_operations: Vec<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudformation_GuardHook {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CloudFormation::GuardHook"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudformation_GuardHook as GuardHook;
impl crate::template::ToResource for GuardHook_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudFormation"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("GuardHook"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Alias".to_string(),
            crate::value::ToValue::to_value(&self.alias),
        );
        properties.insert(
            "ExecutionRole".to_string(),
            crate::value::ToValue::to_value(&self.execution_role),
        );
        properties.insert(
            "FailureMode".to_string(),
            crate::value::ToValue::to_value(&self.failure_mode),
        );
        properties.insert(
            "HookStatus".to_string(),
            crate::value::ToValue::to_value(&self.hook_status),
        );
        if let Some(ref value) = self.log_bucket {
            properties.insert(
                "LogBucket".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.options {
            properties.insert(
                "Options".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RuleLocation".to_string(),
            crate::value::ToValue::to_value(&self.rule_location),
        );
        if let Some(ref value) = self.stack_filters {
            properties.insert(
                "StackFilters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.target_filters {
            properties.insert(
                "TargetFilters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "TargetOperations".to_string(),
            crate::value::ToValue::to_value(&self.target_operations),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-hookdefaultversion.html
pub struct HookDefaultVersion_ {
    pub type_name: Option<crate::value::ExpString>,
    pub type_version_arn: Option<crate::value::ExpString>,
    pub version_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudformation_HookDefaultVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CloudFormation::HookDefaultVersion"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudformation_HookDefaultVersion as HookDefaultVersion;
impl crate::template::ToResource for HookDefaultVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudFormation"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("HookDefaultVersion"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.type_name {
            properties.insert(
                "TypeName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.type_version_arn {
            properties.insert(
                "TypeVersionArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.version_id {
            properties.insert(
                "VersionId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-hooktypeconfig.html
pub struct HookTypeConfig_ {
    pub configuration: crate::value::ExpString,
    pub configuration_alias: Option<crate::value::ExpString>,
    pub type_arn: Option<crate::value::ExpString>,
    pub type_name: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudformation_HookTypeConfig {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CloudFormation::HookTypeConfig"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudformation_HookTypeConfig as HookTypeConfig;
impl crate::template::ToResource for HookTypeConfig_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudFormation"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("HookTypeConfig"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Configuration".to_string(),
            crate::value::ToValue::to_value(&self.configuration),
        );
        if let Some(ref value) = self.configuration_alias {
            properties.insert(
                "ConfigurationAlias".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.type_arn {
            properties.insert(
                "TypeArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.type_name {
            properties.insert(
                "TypeName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-hookversion.html
pub struct HookVersion_ {
    pub execution_role_arn: Option<crate::value::ExpString>,
    pub logging_config: Option<super::cloudformation::hookversion::LoggingConfig_>,
    pub schema_handler_package: crate::value::ExpString,
    pub type_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudformation_HookVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CloudFormation::HookVersion"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudformation_HookVersion as HookVersion;
impl crate::template::ToResource for HookVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudFormation"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("HookVersion"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.execution_role_arn {
            properties.insert(
                "ExecutionRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.logging_config {
            properties.insert(
                "LoggingConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SchemaHandlerPackage".to_string(),
            crate::value::ToValue::to_value(&self.schema_handler_package),
        );
        properties.insert(
            "TypeName".to_string(),
            crate::value::ToValue::to_value(&self.type_name),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-lambdahook.html
pub struct LambdaHook_ {
    pub alias: crate::value::ExpString,
    pub execution_role: crate::value::ExpString,
    pub failure_mode: crate::value::ExpString,
    pub hook_status: crate::value::ExpString,
    pub lambda_function: crate::value::ExpString,
    pub stack_filters: Option<super::cloudformation::lambdahook::StackFilters_>,
    pub target_filters: Option<super::cloudformation::lambdahook::TargetFilters_>,
    pub target_operations: Vec<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudformation_LambdaHook {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CloudFormation::LambdaHook"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudformation_LambdaHook as LambdaHook;
impl crate::template::ToResource for LambdaHook_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudFormation"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LambdaHook"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Alias".to_string(),
            crate::value::ToValue::to_value(&self.alias),
        );
        properties.insert(
            "ExecutionRole".to_string(),
            crate::value::ToValue::to_value(&self.execution_role),
        );
        properties.insert(
            "FailureMode".to_string(),
            crate::value::ToValue::to_value(&self.failure_mode),
        );
        properties.insert(
            "HookStatus".to_string(),
            crate::value::ToValue::to_value(&self.hook_status),
        );
        properties.insert(
            "LambdaFunction".to_string(),
            crate::value::ToValue::to_value(&self.lambda_function),
        );
        if let Some(ref value) = self.stack_filters {
            properties.insert(
                "StackFilters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.target_filters {
            properties.insert(
                "TargetFilters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "TargetOperations".to_string(),
            crate::value::ToValue::to_value(&self.target_operations),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-macro.html
pub struct Macro_ {
    pub description: Option<crate::value::ExpString>,
    pub function_name: crate::value::ExpString,
    pub log_group_name: Option<crate::value::ExpString>,
    pub log_role_arn: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudformation_Macro {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CloudFormation::Macro"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudformation_Macro as Macro;
impl crate::template::ToResource for Macro_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudFormation"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Macro"),
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
        if let Some(ref value) = self.log_group_name {
            properties.insert(
                "LogGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.log_role_arn {
            properties.insert(
                "LogRoleARN".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-moduledefaultversion.html
pub struct ModuleDefaultVersion_ {
    pub arn: Option<crate::value::ExpString>,
    pub module_name: Option<crate::value::ExpString>,
    pub version_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudformation_ModuleDefaultVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CloudFormation::ModuleDefaultVersion"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudformation_ModuleDefaultVersion as ModuleDefaultVersion;
impl crate::template::ToResource for ModuleDefaultVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudFormation"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ModuleDefaultVersion"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.arn {
            properties.insert("Arn".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.module_name {
            properties.insert(
                "ModuleName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.version_id {
            properties.insert(
                "VersionId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-moduleversion.html
pub struct ModuleVersion_ {
    pub module_name: crate::value::ExpString,
    pub module_package: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudformation_ModuleVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CloudFormation::ModuleVersion"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudformation_ModuleVersion as ModuleVersion;
impl crate::template::ToResource for ModuleVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudFormation"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ModuleVersion"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ModuleName".to_string(),
            crate::value::ToValue::to_value(&self.module_name),
        );
        properties.insert(
            "ModulePackage".to_string(),
            crate::value::ToValue::to_value(&self.module_package),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-publictypeversion.html
pub struct PublicTypeVersion_ {
    pub arn: Option<crate::value::ExpString>,
    pub log_delivery_bucket: Option<crate::value::ExpString>,
    pub public_version_number: Option<crate::value::ExpString>,
    pub r#type: Option<crate::value::ExpString>,
    pub type_name: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudformation_PublicTypeVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CloudFormation::PublicTypeVersion"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudformation_PublicTypeVersion as PublicTypeVersion;
impl crate::template::ToResource for PublicTypeVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudFormation"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PublicTypeVersion"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.arn {
            properties.insert("Arn".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.log_delivery_bucket {
            properties.insert(
                "LogDeliveryBucket".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.public_version_number {
            properties.insert(
                "PublicVersionNumber".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.r#type {
            properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.type_name {
            properties.insert(
                "TypeName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-publisher.html
pub struct Publisher_ {
    pub accept_terms_and_conditions: crate::value::ExpBool,
    pub connection_arn: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudformation_Publisher {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CloudFormation::Publisher"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudformation_Publisher as Publisher;
impl crate::template::ToResource for Publisher_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudFormation"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Publisher"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AcceptTermsAndConditions".to_string(),
            crate::value::ToValue::to_value(&self.accept_terms_and_conditions),
        );
        if let Some(ref value) = self.connection_arn {
            properties.insert(
                "ConnectionArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-resourcedefaultversion.html
pub struct ResourceDefaultVersion_ {
    pub type_name: Option<crate::value::ExpString>,
    pub type_version_arn: Option<crate::value::ExpString>,
    pub version_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudformation_ResourceDefaultVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CloudFormation::ResourceDefaultVersion"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudformation_ResourceDefaultVersion as ResourceDefaultVersion;
impl crate::template::ToResource for ResourceDefaultVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudFormation"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ResourceDefaultVersion"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.type_name {
            properties.insert(
                "TypeName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.type_version_arn {
            properties.insert(
                "TypeVersionArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.version_id {
            properties.insert(
                "VersionId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-resourceversion.html
pub struct ResourceVersion_ {
    pub execution_role_arn: Option<crate::value::ExpString>,
    pub logging_config: Option<super::cloudformation::resourceversion::LoggingConfig_>,
    pub schema_handler_package: crate::value::ExpString,
    pub type_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudformation_ResourceVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CloudFormation::ResourceVersion"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudformation_ResourceVersion as ResourceVersion;
impl crate::template::ToResource for ResourceVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudFormation"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ResourceVersion"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.execution_role_arn {
            properties.insert(
                "ExecutionRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.logging_config {
            properties.insert(
                "LoggingConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SchemaHandlerPackage".to_string(),
            crate::value::ToValue::to_value(&self.schema_handler_package),
        );
        properties.insert(
            "TypeName".to_string(),
            crate::value::ToValue::to_value(&self.type_name),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-stack.html
pub struct Stack_ {
    pub notification_ar_ns: Option<Vec<crate::value::ExpString>>,
    pub parameters: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub template_url: crate::value::ExpString,
    pub timeout_in_minutes: Option<i32>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudformation_Stack {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CloudFormation::Stack"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudformation_Stack as Stack;
impl crate::template::ToResource for Stack_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudFormation"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Stack"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.notification_ar_ns {
            properties.insert(
                "NotificationARNs".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.parameters {
            properties.insert(
                "Parameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TemplateURL".to_string(),
            crate::value::ToValue::to_value(&self.template_url),
        );
        if let Some(ref value) = self.timeout_in_minutes {
            properties.insert(
                "TimeoutInMinutes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-stackset.html
pub struct StackSet_ {
    pub administration_role_arn: Option<crate::value::ExpString>,
    pub auto_deployment: Option<super::cloudformation::stackset::AutoDeployment_>,
    pub call_as: Option<crate::value::ExpString>,
    pub capabilities: Option<Vec<crate::value::ExpString>>,
    pub description: Option<crate::value::ExpString>,
    pub execution_role_name: Option<crate::value::ExpString>,
    pub managed_execution: Option<super::cloudformation::stackset::ManagedExecution_>,
    pub operation_preferences: Option<super::cloudformation::stackset::OperationPreferences_>,
    pub parameters: Option<Vec<super::cloudformation::stackset::Parameter_>>,
    pub permission_model: crate::value::ExpString,
    pub stack_instances_group: Option<Vec<super::cloudformation::stackset::StackInstances_>>,
    pub stack_set_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub template_body: Option<crate::value::ExpString>,
    pub template_url: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudformation_StackSet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CloudFormation::StackSet"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudformation_StackSet as StackSet;
impl crate::template::ToResource for StackSet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudFormation"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("StackSet"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.administration_role_arn {
            properties.insert(
                "AdministrationRoleARN".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.auto_deployment {
            properties.insert(
                "AutoDeployment".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.call_as {
            properties.insert("CallAs".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.capabilities {
            properties.insert(
                "Capabilities".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.execution_role_name {
            properties.insert(
                "ExecutionRoleName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.managed_execution {
            properties.insert(
                "ManagedExecution".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.operation_preferences {
            properties.insert(
                "OperationPreferences".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.parameters {
            properties.insert(
                "Parameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "PermissionModel".to_string(),
            crate::value::ToValue::to_value(&self.permission_model),
        );
        if let Some(ref value) = self.stack_instances_group {
            properties.insert(
                "StackInstancesGroup".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "StackSetName".to_string(),
            crate::value::ToValue::to_value(&self.stack_set_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.template_body {
            properties.insert(
                "TemplateBody".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.template_url {
            properties.insert(
                "TemplateURL".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudformation-typeactivation.html
pub struct TypeActivation_ {
    pub auto_update: Option<crate::value::ExpBool>,
    pub execution_role_arn: Option<crate::value::ExpString>,
    pub logging_config: Option<super::cloudformation::typeactivation::LoggingConfig_>,
    pub major_version: Option<crate::value::ExpString>,
    pub public_type_arn: Option<crate::value::ExpString>,
    pub publisher_id: Option<crate::value::ExpString>,
    pub r#type: Option<crate::value::ExpString>,
    pub type_name: Option<crate::value::ExpString>,
    pub type_name_alias: Option<crate::value::ExpString>,
    pub version_bump: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudformation_TypeActivation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CloudFormation::TypeActivation"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudformation_TypeActivation as TypeActivation;
impl crate::template::ToResource for TypeActivation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudFormation"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TypeActivation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.auto_update {
            properties.insert(
                "AutoUpdate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.execution_role_arn {
            properties.insert(
                "ExecutionRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.logging_config {
            properties.insert(
                "LoggingConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.major_version {
            properties.insert(
                "MajorVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.public_type_arn {
            properties.insert(
                "PublicTypeArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.publisher_id {
            properties.insert(
                "PublisherId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.r#type {
            properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.type_name {
            properties.insert(
                "TypeName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.type_name_alias {
            properties.insert(
                "TypeNameAlias".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.version_bump {
            properties.insert(
                "VersionBump".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waitcondition.html
pub struct WaitCondition_ {
    pub count: Option<i32>,
    pub handle: Option<crate::value::ExpString>,
    pub timeout: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudformation_WaitCondition {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CloudFormation::WaitCondition"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudformation_WaitCondition as WaitCondition;
impl crate::template::ToResource for WaitCondition_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudFormation"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("WaitCondition"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.count {
            properties.insert("Count".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.handle {
            properties.insert("Handle".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.timeout {
            properties.insert(
                "Timeout".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waitconditionhandle.html
pub struct WaitConditionHandle_ {}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudformation_WaitConditionHandle {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CloudFormation::WaitConditionHandle"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudformation_WaitConditionHandle as WaitConditionHandle;
impl crate::template::ToResource for WaitConditionHandle_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudFormation"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("WaitConditionHandle"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        crate::template::ResourceProperties::new()
    }
}
