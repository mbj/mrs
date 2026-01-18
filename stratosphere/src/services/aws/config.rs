pub mod configrule {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configrule-compliance.html
    pub struct Compliance_ {
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_config_ConfigRule_Compliance {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Config::ConfigRule.Compliance"
            $($field $value)*)
        };
    }
    pub use crate::__aws_config_ConfigRule_Compliance as Compliance;
    impl crate::value::ToValue for Compliance_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configrule-custompolicydetails.html
    pub struct CustomPolicyDetails_ {
        pub enable_debug_log_delivery: Option<crate::value::ExpBool>,
        pub policy_runtime: Option<crate::value::ExpString>,
        pub policy_text: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_config_ConfigRule_CustomPolicyDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Config::ConfigRule.CustomPolicyDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_config_ConfigRule_CustomPolicyDetails as CustomPolicyDetails;
    impl crate::value::ToValue for CustomPolicyDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enable_debug_log_delivery {
                properties.insert(
                    "EnableDebugLogDelivery".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.policy_runtime {
                properties.insert(
                    "PolicyRuntime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.policy_text {
                properties.insert(
                    "PolicyText".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configrule-evaluationmodeconfiguration.html
    pub struct EvaluationModeConfiguration_ {
        pub mode: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_config_ConfigRule_EvaluationModeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Config::ConfigRule.EvaluationModeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_config_ConfigRule_EvaluationModeConfiguration as EvaluationModeConfiguration;
    impl crate::value::ToValue for EvaluationModeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.mode {
                properties.insert("Mode".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configrule-scope.html
    pub struct Scope_ {
        pub compliance_resource_id: Option<crate::value::ExpString>,
        pub compliance_resource_types: Option<Vec<crate::value::ExpString>>,
        pub tag_key: Option<crate::value::ExpString>,
        pub tag_value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_config_ConfigRule_Scope {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Config::ConfigRule.Scope"
            $($field $value)*)
        };
    }
    pub use crate::__aws_config_ConfigRule_Scope as Scope;
    impl crate::value::ToValue for Scope_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.compliance_resource_id {
                properties.insert(
                    "ComplianceResourceId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.compliance_resource_types {
                properties.insert(
                    "ComplianceResourceTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tag_key {
                properties.insert("TagKey".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.tag_value {
                properties.insert(
                    "TagValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configrule-source.html
    pub struct Source_ {
        pub custom_policy_details: Option<Box<CustomPolicyDetails_>>,
        pub owner: crate::value::ExpString,
        pub source_details: Option<Vec<SourceDetail_>>,
        pub source_identifier: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_config_ConfigRule_Source {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Config::ConfigRule.Source"
            $($field $value)*)
        };
    }
    pub use crate::__aws_config_ConfigRule_Source as Source;
    impl crate::value::ToValue for Source_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_policy_details {
                properties.insert(
                    "CustomPolicyDetails".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Owner".to_string(),
                crate::value::ToValue::to_value(&self.owner),
            );
            if let Some(ref value) = self.source_details {
                properties.insert(
                    "SourceDetails".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_identifier {
                properties.insert(
                    "SourceIdentifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configrule-sourcedetail.html
    pub struct SourceDetail_ {
        pub event_source: crate::value::ExpString,
        pub maximum_execution_frequency: Option<crate::value::ExpString>,
        pub message_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_config_ConfigRule_SourceDetail {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Config::ConfigRule.SourceDetail"
            $($field $value)*)
        };
    }
    pub use crate::__aws_config_ConfigRule_SourceDetail as SourceDetail;
    impl crate::value::ToValue for SourceDetail_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EventSource".to_string(),
                crate::value::ToValue::to_value(&self.event_source),
            );
            if let Some(ref value) = self.maximum_execution_frequency {
                properties.insert(
                    "MaximumExecutionFrequency".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MessageType".to_string(),
                crate::value::ToValue::to_value(&self.message_type),
            );
            properties.into()
        }
    }
}
pub mod configurationaggregator {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configurationaggregator-accountaggregationsource.html
    pub struct AccountAggregationSource_ {
        pub account_ids: Vec<crate::value::ExpString>,
        pub all_aws_regions: Option<crate::value::ExpBool>,
        pub aws_regions: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_config_ConfigurationAggregator_AccountAggregationSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Config::ConfigurationAggregator.AccountAggregationSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_config_ConfigurationAggregator_AccountAggregationSource as AccountAggregationSource;
    impl crate::value::ToValue for AccountAggregationSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AccountIds".to_string(),
                crate::value::ToValue::to_value(&self.account_ids),
            );
            if let Some(ref value) = self.all_aws_regions {
                properties.insert(
                    "AllAwsRegions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.aws_regions {
                properties.insert(
                    "AwsRegions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configurationaggregator-organizationaggregationsource.html
    pub struct OrganizationAggregationSource_ {
        pub all_aws_regions: Option<crate::value::ExpBool>,
        pub aws_regions: Option<Vec<crate::value::ExpString>>,
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_config_ConfigurationAggregator_OrganizationAggregationSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Config::ConfigurationAggregator.OrganizationAggregationSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_config_ConfigurationAggregator_OrganizationAggregationSource as OrganizationAggregationSource;
    impl crate::value::ToValue for OrganizationAggregationSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.all_aws_regions {
                properties.insert(
                    "AllAwsRegions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.aws_regions {
                properties.insert(
                    "AwsRegions".to_string(),
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
}
pub mod configurationrecorder {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configurationrecorder-exclusionbyresourcetypes.html
    pub struct ExclusionByResourceTypes_ {
        pub resource_types: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_config_ConfigurationRecorder_ExclusionByResourceTypes {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Config::ConfigurationRecorder.ExclusionByResourceTypes"
            $($field $value)*)
        };
    }
    pub use crate::__aws_config_ConfigurationRecorder_ExclusionByResourceTypes as ExclusionByResourceTypes;
    impl crate::value::ToValue for ExclusionByResourceTypes_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ResourceTypes".to_string(),
                crate::value::ToValue::to_value(&self.resource_types),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configurationrecorder-recordinggroup.html
    pub struct RecordingGroup_ {
        pub all_supported: Option<crate::value::ExpBool>,
        pub exclusion_by_resource_types: Option<Box<ExclusionByResourceTypes_>>,
        pub include_global_resource_types: Option<crate::value::ExpBool>,
        pub recording_strategy: Option<Box<RecordingStrategy_>>,
        pub resource_types: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_config_ConfigurationRecorder_RecordingGroup {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Config::ConfigurationRecorder.RecordingGroup"
            $($field $value)*)
        };
    }
    pub use crate::__aws_config_ConfigurationRecorder_RecordingGroup as RecordingGroup;
    impl crate::value::ToValue for RecordingGroup_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.all_supported {
                properties.insert(
                    "AllSupported".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exclusion_by_resource_types {
                properties.insert(
                    "ExclusionByResourceTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_global_resource_types {
                properties.insert(
                    "IncludeGlobalResourceTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.recording_strategy {
                properties.insert(
                    "RecordingStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_types {
                properties.insert(
                    "ResourceTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configurationrecorder-recordingmode.html
    pub struct RecordingMode_ {
        pub recording_frequency: crate::value::ExpString,
        pub recording_mode_overrides: Option<Vec<RecordingModeOverride_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_config_ConfigurationRecorder_RecordingMode {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Config::ConfigurationRecorder.RecordingMode"
            $($field $value)*)
        };
    }
    pub use crate::__aws_config_ConfigurationRecorder_RecordingMode as RecordingMode;
    impl crate::value::ToValue for RecordingMode_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RecordingFrequency".to_string(),
                crate::value::ToValue::to_value(&self.recording_frequency),
            );
            if let Some(ref value) = self.recording_mode_overrides {
                properties.insert(
                    "RecordingModeOverrides".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configurationrecorder-recordingmodeoverride.html
    pub struct RecordingModeOverride_ {
        pub description: Option<crate::value::ExpString>,
        pub recording_frequency: crate::value::ExpString,
        pub resource_types: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_config_ConfigurationRecorder_RecordingModeOverride {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Config::ConfigurationRecorder.RecordingModeOverride"
            $($field $value)*)
        };
    }
    pub use crate::__aws_config_ConfigurationRecorder_RecordingModeOverride as RecordingModeOverride;
    impl crate::value::ToValue for RecordingModeOverride_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RecordingFrequency".to_string(),
                crate::value::ToValue::to_value(&self.recording_frequency),
            );
            properties.insert(
                "ResourceTypes".to_string(),
                crate::value::ToValue::to_value(&self.resource_types),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configurationrecorder-recordingstrategy.html
    pub struct RecordingStrategy_ {
        pub use_only: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_config_ConfigurationRecorder_RecordingStrategy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Config::ConfigurationRecorder.RecordingStrategy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_config_ConfigurationRecorder_RecordingStrategy as RecordingStrategy;
    impl crate::value::ToValue for RecordingStrategy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "UseOnly".to_string(),
                crate::value::ToValue::to_value(&self.use_only),
            );
            properties.into()
        }
    }
}
pub mod conformancepack {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-conformancepack-conformancepackinputparameter.html
    pub struct ConformancePackInputParameter_ {
        pub parameter_name: crate::value::ExpString,
        pub parameter_value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_config_ConformancePack_ConformancePackInputParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Config::ConformancePack.ConformancePackInputParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_config_ConformancePack_ConformancePackInputParameter as ConformancePackInputParameter;
    impl crate::value::ToValue for ConformancePackInputParameter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ParameterName".to_string(),
                crate::value::ToValue::to_value(&self.parameter_name),
            );
            properties.insert(
                "ParameterValue".to_string(),
                crate::value::ToValue::to_value(&self.parameter_value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-conformancepack-templatessmdocumentdetails.html
    pub struct TemplateSSMDocumentDetails_ {
        pub document_name: Option<crate::value::ExpString>,
        pub document_version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_config_ConformancePack_TemplateSSMDocumentDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Config::ConformancePack.TemplateSSMDocumentDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_config_ConformancePack_TemplateSSMDocumentDetails as TemplateSSMDocumentDetails;
    impl crate::value::ToValue for TemplateSSMDocumentDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.document_name {
                properties.insert(
                    "DocumentName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.document_version {
                properties.insert(
                    "DocumentVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod deliverychannel {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-deliverychannel-configsnapshotdeliveryproperties.html
    pub struct ConfigSnapshotDeliveryProperties_ {
        pub delivery_frequency: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_config_DeliveryChannel_ConfigSnapshotDeliveryProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Config::DeliveryChannel.ConfigSnapshotDeliveryProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_config_DeliveryChannel_ConfigSnapshotDeliveryProperties as ConfigSnapshotDeliveryProperties;
    impl crate::value::ToValue for ConfigSnapshotDeliveryProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.delivery_frequency {
                properties.insert(
                    "DeliveryFrequency".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod organizationconfigrule {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-organizationconfigrule-organizationcustompolicyrulemetadata.html
    pub struct OrganizationCustomPolicyRuleMetadata_ {
        pub debug_log_delivery_accounts: Option<Vec<crate::value::ExpString>>,
        pub description: Option<crate::value::ExpString>,
        pub input_parameters: Option<crate::value::ExpString>,
        pub maximum_execution_frequency: Option<crate::value::ExpString>,
        pub organization_config_rule_trigger_types: Option<Vec<crate::value::ExpString>>,
        pub policy_text: crate::value::ExpString,
        pub resource_id_scope: Option<crate::value::ExpString>,
        pub resource_types_scope: Option<Vec<crate::value::ExpString>>,
        pub runtime: crate::value::ExpString,
        pub tag_key_scope: Option<crate::value::ExpString>,
        pub tag_value_scope: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_config_OrganizationConfigRule_OrganizationCustomPolicyRuleMetadata {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Config::OrganizationConfigRule.OrganizationCustomPolicyRuleMetadata"
            $($field $value)*)
        };
    }
    pub use crate::__aws_config_OrganizationConfigRule_OrganizationCustomPolicyRuleMetadata as OrganizationCustomPolicyRuleMetadata;
    impl crate::value::ToValue for OrganizationCustomPolicyRuleMetadata_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.debug_log_delivery_accounts {
                properties.insert(
                    "DebugLogDeliveryAccounts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_parameters {
                properties.insert(
                    "InputParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.maximum_execution_frequency {
                properties.insert(
                    "MaximumExecutionFrequency".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.organization_config_rule_trigger_types {
                properties.insert(
                    "OrganizationConfigRuleTriggerTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "PolicyText".to_string(),
                crate::value::ToValue::to_value(&self.policy_text),
            );
            if let Some(ref value) = self.resource_id_scope {
                properties.insert(
                    "ResourceIdScope".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_types_scope {
                properties.insert(
                    "ResourceTypesScope".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Runtime".to_string(),
                crate::value::ToValue::to_value(&self.runtime),
            );
            if let Some(ref value) = self.tag_key_scope {
                properties.insert(
                    "TagKeyScope".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tag_value_scope {
                properties.insert(
                    "TagValueScope".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-organizationconfigrule-organizationcustomrulemetadata.html
    pub struct OrganizationCustomRuleMetadata_ {
        pub description: Option<crate::value::ExpString>,
        pub input_parameters: Option<crate::value::ExpString>,
        pub lambda_function_arn: crate::value::ExpString,
        pub maximum_execution_frequency: Option<crate::value::ExpString>,
        pub organization_config_rule_trigger_types: Vec<crate::value::ExpString>,
        pub resource_id_scope: Option<crate::value::ExpString>,
        pub resource_types_scope: Option<Vec<crate::value::ExpString>>,
        pub tag_key_scope: Option<crate::value::ExpString>,
        pub tag_value_scope: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_config_OrganizationConfigRule_OrganizationCustomRuleMetadata {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Config::OrganizationConfigRule.OrganizationCustomRuleMetadata"
            $($field $value)*)
        };
    }
    pub use crate::__aws_config_OrganizationConfigRule_OrganizationCustomRuleMetadata as OrganizationCustomRuleMetadata;
    impl crate::value::ToValue for OrganizationCustomRuleMetadata_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_parameters {
                properties.insert(
                    "InputParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "LambdaFunctionArn".to_string(),
                crate::value::ToValue::to_value(&self.lambda_function_arn),
            );
            if let Some(ref value) = self.maximum_execution_frequency {
                properties.insert(
                    "MaximumExecutionFrequency".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "OrganizationConfigRuleTriggerTypes".to_string(),
                crate::value::ToValue::to_value(&self.organization_config_rule_trigger_types),
            );
            if let Some(ref value) = self.resource_id_scope {
                properties.insert(
                    "ResourceIdScope".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_types_scope {
                properties.insert(
                    "ResourceTypesScope".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tag_key_scope {
                properties.insert(
                    "TagKeyScope".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tag_value_scope {
                properties.insert(
                    "TagValueScope".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-organizationconfigrule-organizationmanagedrulemetadata.html
    pub struct OrganizationManagedRuleMetadata_ {
        pub description: Option<crate::value::ExpString>,
        pub input_parameters: Option<crate::value::ExpString>,
        pub maximum_execution_frequency: Option<crate::value::ExpString>,
        pub resource_id_scope: Option<crate::value::ExpString>,
        pub resource_types_scope: Option<Vec<crate::value::ExpString>>,
        pub rule_identifier: crate::value::ExpString,
        pub tag_key_scope: Option<crate::value::ExpString>,
        pub tag_value_scope: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_config_OrganizationConfigRule_OrganizationManagedRuleMetadata {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Config::OrganizationConfigRule.OrganizationManagedRuleMetadata"
            $($field $value)*)
        };
    }
    pub use crate::__aws_config_OrganizationConfigRule_OrganizationManagedRuleMetadata as OrganizationManagedRuleMetadata;
    impl crate::value::ToValue for OrganizationManagedRuleMetadata_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_parameters {
                properties.insert(
                    "InputParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.maximum_execution_frequency {
                properties.insert(
                    "MaximumExecutionFrequency".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_id_scope {
                properties.insert(
                    "ResourceIdScope".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_types_scope {
                properties.insert(
                    "ResourceTypesScope".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RuleIdentifier".to_string(),
                crate::value::ToValue::to_value(&self.rule_identifier),
            );
            if let Some(ref value) = self.tag_key_scope {
                properties.insert(
                    "TagKeyScope".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tag_value_scope {
                properties.insert(
                    "TagValueScope".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod organizationconformancepack {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-organizationconformancepack-conformancepackinputparameter.html
    pub struct ConformancePackInputParameter_ {
        pub parameter_name: crate::value::ExpString,
        pub parameter_value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_config_OrganizationConformancePack_ConformancePackInputParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Config::OrganizationConformancePack.ConformancePackInputParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_config_OrganizationConformancePack_ConformancePackInputParameter as ConformancePackInputParameter;
    impl crate::value::ToValue for ConformancePackInputParameter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ParameterName".to_string(),
                crate::value::ToValue::to_value(&self.parameter_name),
            );
            properties.insert(
                "ParameterValue".to_string(),
                crate::value::ToValue::to_value(&self.parameter_value),
            );
            properties.into()
        }
    }
}
pub mod remediationconfiguration {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-remediationconfiguration-executioncontrols.html
    pub struct ExecutionControls_ {
        pub ssm_controls: Option<Box<SsmControls_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_config_RemediationConfiguration_ExecutionControls {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Config::RemediationConfiguration.ExecutionControls"
            $($field $value)*)
        };
    }
    pub use crate::__aws_config_RemediationConfiguration_ExecutionControls as ExecutionControls;
    impl crate::value::ToValue for ExecutionControls_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ssm_controls {
                properties.insert(
                    "SsmControls".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-remediationconfiguration-remediationparametervalue.html
    pub struct RemediationParameterValue_ {
        pub resource_value: Option<Box<ResourceValue_>>,
        pub static_value: Option<Box<StaticValue_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_config_RemediationConfiguration_RemediationParameterValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Config::RemediationConfiguration.RemediationParameterValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_config_RemediationConfiguration_RemediationParameterValue as RemediationParameterValue;
    impl crate::value::ToValue for RemediationParameterValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.resource_value {
                properties.insert(
                    "ResourceValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.static_value {
                properties.insert(
                    "StaticValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-remediationconfiguration-resourcevalue.html
    pub struct ResourceValue_ {
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_config_RemediationConfiguration_ResourceValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Config::RemediationConfiguration.ResourceValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_config_RemediationConfiguration_ResourceValue as ResourceValue;
    impl crate::value::ToValue for ResourceValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-remediationconfiguration-ssmcontrols.html
    pub struct SsmControls_ {
        pub concurrent_execution_rate_percentage: Option<i64>,
        pub error_percentage: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_config_RemediationConfiguration_SsmControls {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Config::RemediationConfiguration.SsmControls"
            $($field $value)*)
        };
    }
    pub use crate::__aws_config_RemediationConfiguration_SsmControls as SsmControls;
    impl crate::value::ToValue for SsmControls_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.concurrent_execution_rate_percentage {
                properties.insert(
                    "ConcurrentExecutionRatePercentage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.error_percentage {
                properties.insert(
                    "ErrorPercentage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-remediationconfiguration-staticvalue.html
    pub struct StaticValue_ {
        pub values: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_config_RemediationConfiguration_StaticValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Config::RemediationConfiguration.StaticValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_config_RemediationConfiguration_StaticValue as StaticValue;
    impl crate::value::ToValue for StaticValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.values {
                properties.insert("Values".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-aggregationauthorization.html
pub struct AggregationAuthorization_ {
    pub authorized_account_id: crate::value::ExpString,
    pub authorized_aws_region: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_config_AggregationAuthorization {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Config::AggregationAuthorization"
        $($field $value)*)
    };
}
pub use crate::__aws_config_AggregationAuthorization as AggregationAuthorization;
impl crate::template::ToResource for AggregationAuthorization_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Config"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AggregationAuthorization"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AuthorizedAccountId".to_string(),
            crate::value::ToValue::to_value(&self.authorized_account_id),
        );
        properties.insert(
            "AuthorizedAwsRegion".to_string(),
            crate::value::ToValue::to_value(&self.authorized_aws_region),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-configrule.html
pub struct ConfigRule_ {
    pub compliance: Option<super::config::configrule::Compliance_>,
    pub config_rule_name: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub evaluation_modes: Option<Vec<super::config::configrule::EvaluationModeConfiguration_>>,
    pub input_parameters: Option<serde_json::Value>,
    pub maximum_execution_frequency: Option<crate::value::ExpString>,
    pub scope: Option<super::config::configrule::Scope_>,
    pub source: super::config::configrule::Source_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_config_ConfigRule {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Config::ConfigRule"
        $($field $value)*)
    };
}
pub use crate::__aws_config_ConfigRule as ConfigRule;
impl crate::template::ToResource for ConfigRule_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Config"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ConfigRule"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.compliance {
            properties.insert(
                "Compliance".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.config_rule_name {
            properties.insert(
                "ConfigRuleName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.evaluation_modes {
            properties.insert(
                "EvaluationModes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.input_parameters {
            properties.insert(
                "InputParameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.maximum_execution_frequency {
            properties.insert(
                "MaximumExecutionFrequency".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.scope {
            properties.insert("Scope".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Source".to_string(),
            crate::value::ToValue::to_value(&self.source),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-configurationaggregator.html
pub struct ConfigurationAggregator_ {
    pub account_aggregation_sources:
        Option<Vec<super::config::configurationaggregator::AccountAggregationSource_>>,
    pub configuration_aggregator_name: Option<crate::value::ExpString>,
    pub organization_aggregation_source:
        Option<super::config::configurationaggregator::OrganizationAggregationSource_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_config_ConfigurationAggregator {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Config::ConfigurationAggregator"
        $($field $value)*)
    };
}
pub use crate::__aws_config_ConfigurationAggregator as ConfigurationAggregator;
impl crate::template::ToResource for ConfigurationAggregator_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Config"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ConfigurationAggregator"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.account_aggregation_sources {
            properties.insert(
                "AccountAggregationSources".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.configuration_aggregator_name {
            properties.insert(
                "ConfigurationAggregatorName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.organization_aggregation_source {
            properties.insert(
                "OrganizationAggregationSource".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-configurationrecorder.html
pub struct ConfigurationRecorder_ {
    pub name: Option<crate::value::ExpString>,
    pub recording_group: Option<super::config::configurationrecorder::RecordingGroup_>,
    pub recording_mode: Option<super::config::configurationrecorder::RecordingMode_>,
    pub role_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_config_ConfigurationRecorder {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Config::ConfigurationRecorder"
        $($field $value)*)
    };
}
pub use crate::__aws_config_ConfigurationRecorder as ConfigurationRecorder;
impl crate::template::ToResource for ConfigurationRecorder_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Config"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ConfigurationRecorder"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.recording_group {
            properties.insert(
                "RecordingGroup".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.recording_mode {
            properties.insert(
                "RecordingMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RoleARN".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-conformancepack.html
pub struct ConformancePack_ {
    pub conformance_pack_input_parameters:
        Option<Vec<super::config::conformancepack::ConformancePackInputParameter_>>,
    pub conformance_pack_name: crate::value::ExpString,
    pub delivery_s3_bucket: Option<crate::value::ExpString>,
    pub delivery_s3_key_prefix: Option<crate::value::ExpString>,
    pub template_body: Option<crate::value::ExpString>,
    pub template_s3_uri: Option<crate::value::ExpString>,
    pub template_ssm_document_details:
        Option<super::config::conformancepack::TemplateSSMDocumentDetails_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_config_ConformancePack {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Config::ConformancePack"
        $($field $value)*)
    };
}
pub use crate::__aws_config_ConformancePack as ConformancePack;
impl crate::template::ToResource for ConformancePack_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Config"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ConformancePack"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.conformance_pack_input_parameters {
            properties.insert(
                "ConformancePackInputParameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ConformancePackName".to_string(),
            crate::value::ToValue::to_value(&self.conformance_pack_name),
        );
        if let Some(ref value) = self.delivery_s3_bucket {
            properties.insert(
                "DeliveryS3Bucket".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.delivery_s3_key_prefix {
            properties.insert(
                "DeliveryS3KeyPrefix".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.template_body {
            properties.insert(
                "TemplateBody".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.template_s3_uri {
            properties.insert(
                "TemplateS3Uri".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.template_ssm_document_details {
            properties.insert(
                "TemplateSSMDocumentDetails".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-deliverychannel.html
pub struct DeliveryChannel_ {
    pub config_snapshot_delivery_properties:
        Option<super::config::deliverychannel::ConfigSnapshotDeliveryProperties_>,
    pub name: Option<crate::value::ExpString>,
    pub s3_bucket_name: crate::value::ExpString,
    pub s3_key_prefix: Option<crate::value::ExpString>,
    pub s3_kms_key_arn: Option<crate::value::ExpString>,
    pub sns_topic_arn: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_config_DeliveryChannel {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Config::DeliveryChannel"
        $($field $value)*)
    };
}
pub use crate::__aws_config_DeliveryChannel as DeliveryChannel;
impl crate::template::ToResource for DeliveryChannel_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Config"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DeliveryChannel"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.config_snapshot_delivery_properties {
            properties.insert(
                "ConfigSnapshotDeliveryProperties".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "S3BucketName".to_string(),
            crate::value::ToValue::to_value(&self.s3_bucket_name),
        );
        if let Some(ref value) = self.s3_key_prefix {
            properties.insert(
                "S3KeyPrefix".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.s3_kms_key_arn {
            properties.insert(
                "S3KmsKeyArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sns_topic_arn {
            properties.insert(
                "SnsTopicARN".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-organizationconfigrule.html
pub struct OrganizationConfigRule_ {
    pub excluded_accounts: Option<Vec<crate::value::ExpString>>,
    pub organization_config_rule_name: crate::value::ExpString,
    pub organization_custom_policy_rule_metadata:
        Option<super::config::organizationconfigrule::OrganizationCustomPolicyRuleMetadata_>,
    pub organization_custom_rule_metadata:
        Option<super::config::organizationconfigrule::OrganizationCustomRuleMetadata_>,
    pub organization_managed_rule_metadata:
        Option<super::config::organizationconfigrule::OrganizationManagedRuleMetadata_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_config_OrganizationConfigRule {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Config::OrganizationConfigRule"
        $($field $value)*)
    };
}
pub use crate::__aws_config_OrganizationConfigRule as OrganizationConfigRule;
impl crate::template::ToResource for OrganizationConfigRule_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Config"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("OrganizationConfigRule"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.excluded_accounts {
            properties.insert(
                "ExcludedAccounts".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "OrganizationConfigRuleName".to_string(),
            crate::value::ToValue::to_value(&self.organization_config_rule_name),
        );
        if let Some(ref value) = self.organization_custom_policy_rule_metadata {
            properties.insert(
                "OrganizationCustomPolicyRuleMetadata".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.organization_custom_rule_metadata {
            properties.insert(
                "OrganizationCustomRuleMetadata".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.organization_managed_rule_metadata {
            properties.insert(
                "OrganizationManagedRuleMetadata".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-organizationconformancepack.html
pub struct OrganizationConformancePack_ {
    pub conformance_pack_input_parameters:
        Option<Vec<super::config::organizationconformancepack::ConformancePackInputParameter_>>,
    pub delivery_s3_bucket: Option<crate::value::ExpString>,
    pub delivery_s3_key_prefix: Option<crate::value::ExpString>,
    pub excluded_accounts: Option<Vec<crate::value::ExpString>>,
    pub organization_conformance_pack_name: crate::value::ExpString,
    pub template_body: Option<crate::value::ExpString>,
    pub template_s3_uri: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_config_OrganizationConformancePack {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Config::OrganizationConformancePack"
        $($field $value)*)
    };
}
pub use crate::__aws_config_OrganizationConformancePack as OrganizationConformancePack;
impl crate::template::ToResource for OrganizationConformancePack_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Config"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "OrganizationConformancePack",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.conformance_pack_input_parameters {
            properties.insert(
                "ConformancePackInputParameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.delivery_s3_bucket {
            properties.insert(
                "DeliveryS3Bucket".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.delivery_s3_key_prefix {
            properties.insert(
                "DeliveryS3KeyPrefix".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.excluded_accounts {
            properties.insert(
                "ExcludedAccounts".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "OrganizationConformancePackName".to_string(),
            crate::value::ToValue::to_value(&self.organization_conformance_pack_name),
        );
        if let Some(ref value) = self.template_body {
            properties.insert(
                "TemplateBody".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.template_s3_uri {
            properties.insert(
                "TemplateS3Uri".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-remediationconfiguration.html
pub struct RemediationConfiguration_ {
    pub automatic: Option<crate::value::ExpBool>,
    pub config_rule_name: crate::value::ExpString,
    pub execution_controls: Option<super::config::remediationconfiguration::ExecutionControls_>,
    pub maximum_automatic_attempts: Option<i64>,
    pub parameters: Option<serde_json::Value>,
    pub resource_type: Option<crate::value::ExpString>,
    pub retry_attempt_seconds: Option<i64>,
    pub target_id: crate::value::ExpString,
    pub target_type: crate::value::ExpString,
    pub target_version: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_config_RemediationConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Config::RemediationConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_config_RemediationConfiguration as RemediationConfiguration;
impl crate::template::ToResource for RemediationConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Config"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RemediationConfiguration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.automatic {
            properties.insert(
                "Automatic".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ConfigRuleName".to_string(),
            crate::value::ToValue::to_value(&self.config_rule_name),
        );
        if let Some(ref value) = self.execution_controls {
            properties.insert(
                "ExecutionControls".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.maximum_automatic_attempts {
            properties.insert(
                "MaximumAutomaticAttempts".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.parameters {
            properties.insert(
                "Parameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resource_type {
            properties.insert(
                "ResourceType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.retry_attempt_seconds {
            properties.insert(
                "RetryAttemptSeconds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "TargetId".to_string(),
            crate::value::ToValue::to_value(&self.target_id),
        );
        properties.insert(
            "TargetType".to_string(),
            crate::value::ToValue::to_value(&self.target_type),
        );
        if let Some(ref value) = self.target_version {
            properties.insert(
                "TargetVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-storedquery.html
pub struct StoredQuery_ {
    pub query_description: Option<crate::value::ExpString>,
    pub query_expression: crate::value::ExpString,
    pub query_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_config_StoredQuery {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Config::StoredQuery"
        $($field $value)*)
    };
}
pub use crate::__aws_config_StoredQuery as StoredQuery;
impl crate::template::ToResource for StoredQuery_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Config"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("StoredQuery"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.query_description {
            properties.insert(
                "QueryDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "QueryExpression".to_string(),
            crate::value::ToValue::to_value(&self.query_expression),
        );
        properties.insert(
            "QueryName".to_string(),
            crate::value::ToValue::to_value(&self.query_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
