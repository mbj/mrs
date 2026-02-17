pub mod organizationcentralizationrule {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-organizationcentralizationrule-centralizationrule.html>
    pub struct CentralizationRule_ {
        pub destination: Box<CentralizationRuleDestination_>,
        pub source: Box<CentralizationRuleSource_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_OrganizationCentralizationRule_CentralizationRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::OrganizationCentralizationRule.CentralizationRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_OrganizationCentralizationRule_CentralizationRule as CentralizationRule;
    impl crate::value::ToValue for CentralizationRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Destination".to_string(),
                crate::value::ToValue::to_value(&self.destination),
            );
            properties.insert(
                "Source".to_string(),
                crate::value::ToValue::to_value(&self.source),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-organizationcentralizationrule-centralizationruledestination.html>
    pub struct CentralizationRuleDestination_ {
        pub account: Option<crate::value::ExpString>,
        pub destination_logs_configuration: Option<Box<DestinationLogsConfiguration_>>,
        pub region: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_OrganizationCentralizationRule_CentralizationRuleDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::OrganizationCentralizationRule.CentralizationRuleDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_OrganizationCentralizationRule_CentralizationRuleDestination as CentralizationRuleDestination;
    impl crate::value::ToValue for CentralizationRuleDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.account {
                properties.insert(
                    "Account".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.destination_logs_configuration {
                properties.insert(
                    "DestinationLogsConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Region".to_string(),
                crate::value::ToValue::to_value(&self.region),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-organizationcentralizationrule-centralizationrulesource.html>
    pub struct CentralizationRuleSource_ {
        pub regions: Vec<crate::value::ExpString>,
        pub scope: Option<crate::value::ExpString>,
        pub source_logs_configuration: Option<Box<SourceLogsConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_OrganizationCentralizationRule_CentralizationRuleSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::OrganizationCentralizationRule.CentralizationRuleSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_OrganizationCentralizationRule_CentralizationRuleSource as CentralizationRuleSource;
    impl crate::value::ToValue for CentralizationRuleSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Regions".to_string(),
                crate::value::ToValue::to_value(&self.regions),
            );
            if let Some(ref value) = self.scope {
                properties.insert("Scope".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.source_logs_configuration {
                properties.insert(
                    "SourceLogsConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-organizationcentralizationrule-destinationlogsconfiguration.html>
    pub struct DestinationLogsConfiguration_ {
        pub backup_configuration: Option<Box<LogsBackupConfiguration_>>,
        pub logs_encryption_configuration: Option<Box<LogsEncryptionConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_OrganizationCentralizationRule_DestinationLogsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::OrganizationCentralizationRule.DestinationLogsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_OrganizationCentralizationRule_DestinationLogsConfiguration as DestinationLogsConfiguration;
    impl crate::value::ToValue for DestinationLogsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.backup_configuration {
                properties.insert(
                    "BackupConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.logs_encryption_configuration {
                properties.insert(
                    "LogsEncryptionConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-organizationcentralizationrule-logsbackupconfiguration.html>
    pub struct LogsBackupConfiguration_ {
        pub kms_key_arn: Option<crate::value::ExpString>,
        pub region: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_OrganizationCentralizationRule_LogsBackupConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::OrganizationCentralizationRule.LogsBackupConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_OrganizationCentralizationRule_LogsBackupConfiguration as LogsBackupConfiguration;
    impl crate::value::ToValue for LogsBackupConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_arn {
                properties.insert(
                    "KmsKeyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Region".to_string(),
                crate::value::ToValue::to_value(&self.region),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-organizationcentralizationrule-logsencryptionconfiguration.html>
    pub struct LogsEncryptionConfiguration_ {
        pub encryption_conflict_resolution_strategy: Option<crate::value::ExpString>,
        pub encryption_strategy: crate::value::ExpString,
        pub kms_key_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_OrganizationCentralizationRule_LogsEncryptionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::OrganizationCentralizationRule.LogsEncryptionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_OrganizationCentralizationRule_LogsEncryptionConfiguration as LogsEncryptionConfiguration;
    impl crate::value::ToValue for LogsEncryptionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.encryption_conflict_resolution_strategy {
                properties.insert(
                    "EncryptionConflictResolutionStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "EncryptionStrategy".to_string(),
                crate::value::ToValue::to_value(&self.encryption_strategy),
            );
            if let Some(ref value) = self.kms_key_arn {
                properties.insert(
                    "KmsKeyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-organizationcentralizationrule-sourcelogsconfiguration.html>
    pub struct SourceLogsConfiguration_ {
        pub encrypted_log_group_strategy: crate::value::ExpString,
        pub log_group_selection_criteria: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_OrganizationCentralizationRule_SourceLogsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::OrganizationCentralizationRule.SourceLogsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_OrganizationCentralizationRule_SourceLogsConfiguration as SourceLogsConfiguration;
    impl crate::value::ToValue for SourceLogsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EncryptedLogGroupStrategy".to_string(),
                crate::value::ToValue::to_value(&self.encrypted_log_group_strategy),
            );
            properties.insert(
                "LogGroupSelectionCriteria".to_string(),
                crate::value::ToValue::to_value(&self.log_group_selection_criteria),
            );
            properties.into()
        }
    }
}
pub mod organizationtelemetryrule {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-organizationtelemetryrule-actioncondition.html>
    pub struct ActionCondition_ {
        pub action: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_OrganizationTelemetryRule_ActionCondition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::OrganizationTelemetryRule.ActionCondition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_OrganizationTelemetryRule_ActionCondition as ActionCondition;
    impl crate::value::ToValue for ActionCondition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.action {
                properties.insert("Action".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-organizationtelemetryrule-advancedeventselector.html>
    pub struct AdvancedEventSelector_ {
        pub field_selectors: Vec<AdvancedFieldSelector_>,
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_OrganizationTelemetryRule_AdvancedEventSelector {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::OrganizationTelemetryRule.AdvancedEventSelector"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_OrganizationTelemetryRule_AdvancedEventSelector as AdvancedEventSelector;
    impl crate::value::ToValue for AdvancedEventSelector_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FieldSelectors".to_string(),
                crate::value::ToValue::to_value(&self.field_selectors),
            );
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-organizationtelemetryrule-advancedfieldselector.html>
    pub struct AdvancedFieldSelector_ {
        pub ends_with: Option<Vec<crate::value::ExpString>>,
        pub equals: Option<Vec<crate::value::ExpString>>,
        pub field: Option<crate::value::ExpString>,
        pub not_ends_with: Option<Vec<crate::value::ExpString>>,
        pub not_equals: Option<Vec<crate::value::ExpString>>,
        pub not_starts_with: Option<Vec<crate::value::ExpString>>,
        pub starts_with: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_OrganizationTelemetryRule_AdvancedFieldSelector {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::OrganizationTelemetryRule.AdvancedFieldSelector"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_OrganizationTelemetryRule_AdvancedFieldSelector as AdvancedFieldSelector;
    impl crate::value::ToValue for AdvancedFieldSelector_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ends_with {
                properties.insert(
                    "EndsWith".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.equals {
                properties.insert("Equals".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.field {
                properties.insert("Field".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.not_ends_with {
                properties.insert(
                    "NotEndsWith".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.not_equals {
                properties.insert(
                    "NotEquals".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.not_starts_with {
                properties.insert(
                    "NotStartsWith".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.starts_with {
                properties.insert(
                    "StartsWith".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-organizationtelemetryrule-cloudtrailparameters.html>
    pub struct CloudtrailParameters_ {
        pub advanced_event_selectors: Vec<AdvancedEventSelector_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_OrganizationTelemetryRule_CloudtrailParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::OrganizationTelemetryRule.CloudtrailParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_OrganizationTelemetryRule_CloudtrailParameters as CloudtrailParameters;
    impl crate::value::ToValue for CloudtrailParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AdvancedEventSelectors".to_string(),
                crate::value::ToValue::to_value(&self.advanced_event_selectors),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-organizationtelemetryrule-condition.html>
    pub struct Condition_ {
        pub action_condition: Option<Box<ActionCondition_>>,
        pub label_name_condition: Option<Box<LabelNameCondition_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_OrganizationTelemetryRule_Condition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::OrganizationTelemetryRule.Condition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_OrganizationTelemetryRule_Condition as Condition;
    impl crate::value::ToValue for Condition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.action_condition {
                properties.insert(
                    "ActionCondition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.label_name_condition {
                properties.insert(
                    "LabelNameCondition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-organizationtelemetryrule-elbloadbalancerloggingparameters.html>
    pub struct ELBLoadBalancerLoggingParameters_ {
        pub field_delimiter: Option<crate::value::ExpString>,
        pub output_format: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_OrganizationTelemetryRule_ELBLoadBalancerLoggingParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::OrganizationTelemetryRule.ELBLoadBalancerLoggingParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_OrganizationTelemetryRule_ELBLoadBalancerLoggingParameters as ELBLoadBalancerLoggingParameters;
    impl crate::value::ToValue for ELBLoadBalancerLoggingParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.field_delimiter {
                properties.insert(
                    "FieldDelimiter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.output_format {
                properties.insert(
                    "OutputFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-organizationtelemetryrule-fieldtomatch.html>
    pub struct FieldToMatch_ {
        pub method: Option<crate::value::ExpString>,
        pub query_string: Option<crate::value::ExpString>,
        pub single_header: Option<Box<SingleHeader_>>,
        pub uri_path: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_OrganizationTelemetryRule_FieldToMatch {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::OrganizationTelemetryRule.FieldToMatch"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_OrganizationTelemetryRule_FieldToMatch as FieldToMatch;
    impl crate::value::ToValue for FieldToMatch_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.method {
                properties.insert("Method".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.query_string {
                properties.insert(
                    "QueryString".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.single_header {
                properties.insert(
                    "SingleHeader".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.uri_path {
                properties.insert(
                    "UriPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-organizationtelemetryrule-filter.html>
    pub struct Filter_ {
        pub behavior: Option<crate::value::ExpString>,
        pub conditions: Option<Vec<Condition_>>,
        pub requirement: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_OrganizationTelemetryRule_Filter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::OrganizationTelemetryRule.Filter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_OrganizationTelemetryRule_Filter as Filter;
    impl crate::value::ToValue for Filter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.behavior {
                properties.insert(
                    "Behavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.conditions {
                properties.insert(
                    "Conditions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.requirement {
                properties.insert(
                    "Requirement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-organizationtelemetryrule-labelnamecondition.html>
    pub struct LabelNameCondition_ {
        pub label_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_OrganizationTelemetryRule_LabelNameCondition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::OrganizationTelemetryRule.LabelNameCondition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_OrganizationTelemetryRule_LabelNameCondition as LabelNameCondition;
    impl crate::value::ToValue for LabelNameCondition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.label_name {
                properties.insert(
                    "LabelName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-organizationtelemetryrule-loggingfilter.html>
    pub struct LoggingFilter_ {
        pub default_behavior: Option<crate::value::ExpString>,
        pub filters: Option<Vec<Filter_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_OrganizationTelemetryRule_LoggingFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::OrganizationTelemetryRule.LoggingFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_OrganizationTelemetryRule_LoggingFilter as LoggingFilter;
    impl crate::value::ToValue for LoggingFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.default_behavior {
                properties.insert(
                    "DefaultBehavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.filters {
                properties.insert(
                    "Filters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-organizationtelemetryrule-singleheader.html>
    pub struct SingleHeader_ {
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_OrganizationTelemetryRule_SingleHeader {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::OrganizationTelemetryRule.SingleHeader"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_OrganizationTelemetryRule_SingleHeader as SingleHeader;
    impl crate::value::ToValue for SingleHeader_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-organizationtelemetryrule-telemetrydestinationconfiguration.html>
    pub struct TelemetryDestinationConfiguration_ {
        pub cloudtrail_parameters: Option<Box<CloudtrailParameters_>>,
        pub destination_pattern: Option<crate::value::ExpString>,
        pub destination_type: Option<crate::value::ExpString>,
        pub elb_load_balancer_logging_parameters: Option<Box<ELBLoadBalancerLoggingParameters_>>,
        pub retention_in_days: Option<i32>,
        pub vpc_flow_log_parameters: Option<Box<VPCFlowLogParameters_>>,
        pub waf_logging_parameters: Option<Box<WAFLoggingParameters_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_OrganizationTelemetryRule_TelemetryDestinationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::OrganizationTelemetryRule.TelemetryDestinationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_OrganizationTelemetryRule_TelemetryDestinationConfiguration as TelemetryDestinationConfiguration;
    impl crate::value::ToValue for TelemetryDestinationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cloudtrail_parameters {
                properties.insert(
                    "CloudtrailParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.destination_pattern {
                properties.insert(
                    "DestinationPattern".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.destination_type {
                properties.insert(
                    "DestinationType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.elb_load_balancer_logging_parameters {
                properties.insert(
                    "ELBLoadBalancerLoggingParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.retention_in_days {
                properties.insert(
                    "RetentionInDays".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpc_flow_log_parameters {
                properties.insert(
                    "VPCFlowLogParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.waf_logging_parameters {
                properties.insert(
                    "WAFLoggingParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-organizationtelemetryrule-telemetryrule.html>
    pub struct TelemetryRule_ {
        pub destination_configuration: Option<Box<TelemetryDestinationConfiguration_>>,
        pub resource_type: crate::value::ExpString,
        pub scope: Option<crate::value::ExpString>,
        pub selection_criteria: Option<crate::value::ExpString>,
        pub telemetry_source_types: Option<Vec<crate::value::ExpString>>,
        pub telemetry_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_OrganizationTelemetryRule_TelemetryRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::OrganizationTelemetryRule.TelemetryRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_OrganizationTelemetryRule_TelemetryRule as TelemetryRule;
    impl crate::value::ToValue for TelemetryRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.destination_configuration {
                properties.insert(
                    "DestinationConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ResourceType".to_string(),
                crate::value::ToValue::to_value(&self.resource_type),
            );
            if let Some(ref value) = self.scope {
                properties.insert("Scope".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.selection_criteria {
                properties.insert(
                    "SelectionCriteria".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.telemetry_source_types {
                properties.insert(
                    "TelemetrySourceTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TelemetryType".to_string(),
                crate::value::ToValue::to_value(&self.telemetry_type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-organizationtelemetryrule-vpcflowlogparameters.html>
    pub struct VPCFlowLogParameters_ {
        pub log_format: Option<crate::value::ExpString>,
        pub max_aggregation_interval: Option<i32>,
        pub traffic_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_OrganizationTelemetryRule_VPCFlowLogParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::OrganizationTelemetryRule.VPCFlowLogParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_OrganizationTelemetryRule_VPCFlowLogParameters as VPCFlowLogParameters;
    impl crate::value::ToValue for VPCFlowLogParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.log_format {
                properties.insert(
                    "LogFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_aggregation_interval {
                properties.insert(
                    "MaxAggregationInterval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.traffic_type {
                properties.insert(
                    "TrafficType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-organizationtelemetryrule-wafloggingparameters.html>
    pub struct WAFLoggingParameters_ {
        pub log_type: Option<crate::value::ExpString>,
        pub logging_filter: Option<Box<LoggingFilter_>>,
        pub redacted_fields: Option<Vec<FieldToMatch_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_OrganizationTelemetryRule_WAFLoggingParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::OrganizationTelemetryRule.WAFLoggingParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_OrganizationTelemetryRule_WAFLoggingParameters as WAFLoggingParameters;
    impl crate::value::ToValue for WAFLoggingParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.log_type {
                properties.insert(
                    "LogType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.logging_filter {
                properties.insert(
                    "LoggingFilter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.redacted_fields {
                properties.insert(
                    "RedactedFields".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod s3tableintegration {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-s3tableintegration-encryptionconfig.html>
    pub struct EncryptionConfig_ {
        pub kms_key_arn: Option<crate::value::ExpString>,
        pub sse_algorithm: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_S3TableIntegration_EncryptionConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::S3TableIntegration.EncryptionConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_S3TableIntegration_EncryptionConfig as EncryptionConfig;
    impl crate::value::ToValue for EncryptionConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_arn {
                properties.insert(
                    "KmsKeyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SseAlgorithm".to_string(),
                crate::value::ToValue::to_value(&self.sse_algorithm),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-s3tableintegration-logsource.html>
    pub struct LogSource_ {
        pub identifier: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_S3TableIntegration_LogSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::S3TableIntegration.LogSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_S3TableIntegration_LogSource as LogSource;
    impl crate::value::ToValue for LogSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.identifier {
                properties.insert(
                    "Identifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
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
}
pub mod telemetrypipelines {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-telemetrypipelines-telemetrypipeline.html>
    pub struct TelemetryPipeline_ {
        pub arn: Option<crate::value::ExpString>,
        pub configuration: Option<Box<TelemetryPipelineConfiguration_>>,
        pub created_time_stamp: Option<f64>,
        pub last_update_time_stamp: Option<f64>,
        pub name: Option<crate::value::ExpString>,
        pub status: Option<crate::value::ExpString>,
        pub status_reason: Option<Box<TelemetryPipelineStatusReason_>>,
        pub tags: Option<Vec<crate::Tag_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_TelemetryPipelines_TelemetryPipeline {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::TelemetryPipelines.TelemetryPipeline"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_TelemetryPipelines_TelemetryPipeline as TelemetryPipeline;
    impl crate::value::ToValue for TelemetryPipeline_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.arn {
                properties.insert("Arn".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.configuration {
                properties.insert(
                    "Configuration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.created_time_stamp {
                properties.insert(
                    "CreatedTimeStamp".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.last_update_time_stamp {
                properties.insert(
                    "LastUpdateTimeStamp".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.status_reason {
                properties.insert(
                    "StatusReason".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tags {
                properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-telemetrypipelines-telemetrypipelineconfiguration.html>
    pub struct TelemetryPipelineConfiguration_ {
        pub body: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_TelemetryPipelines_TelemetryPipelineConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::TelemetryPipelines.TelemetryPipelineConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_TelemetryPipelines_TelemetryPipelineConfiguration as TelemetryPipelineConfiguration;
    impl crate::value::ToValue for TelemetryPipelineConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Body".to_string(),
                crate::value::ToValue::to_value(&self.body),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-telemetrypipelines-telemetrypipelinestatusreason.html>
    pub struct TelemetryPipelineStatusReason_ {
        pub description: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_TelemetryPipelines_TelemetryPipelineStatusReason {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::TelemetryPipelines.TelemetryPipelineStatusReason"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_TelemetryPipelines_TelemetryPipelineStatusReason as TelemetryPipelineStatusReason;
    impl crate::value::ToValue for TelemetryPipelineStatusReason_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod telemetryrule {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-telemetryrule-actioncondition.html>
    pub struct ActionCondition_ {
        pub action: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_TelemetryRule_ActionCondition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::TelemetryRule.ActionCondition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_TelemetryRule_ActionCondition as ActionCondition;
    impl crate::value::ToValue for ActionCondition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.action {
                properties.insert("Action".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-telemetryrule-advancedeventselector.html>
    pub struct AdvancedEventSelector_ {
        pub field_selectors: Vec<AdvancedFieldSelector_>,
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_TelemetryRule_AdvancedEventSelector {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::TelemetryRule.AdvancedEventSelector"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_TelemetryRule_AdvancedEventSelector as AdvancedEventSelector;
    impl crate::value::ToValue for AdvancedEventSelector_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FieldSelectors".to_string(),
                crate::value::ToValue::to_value(&self.field_selectors),
            );
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-telemetryrule-advancedfieldselector.html>
    pub struct AdvancedFieldSelector_ {
        pub ends_with: Option<Vec<crate::value::ExpString>>,
        pub equals: Option<Vec<crate::value::ExpString>>,
        pub field: Option<crate::value::ExpString>,
        pub not_ends_with: Option<Vec<crate::value::ExpString>>,
        pub not_equals: Option<Vec<crate::value::ExpString>>,
        pub not_starts_with: Option<Vec<crate::value::ExpString>>,
        pub starts_with: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_TelemetryRule_AdvancedFieldSelector {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::TelemetryRule.AdvancedFieldSelector"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_TelemetryRule_AdvancedFieldSelector as AdvancedFieldSelector;
    impl crate::value::ToValue for AdvancedFieldSelector_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ends_with {
                properties.insert(
                    "EndsWith".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.equals {
                properties.insert("Equals".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.field {
                properties.insert("Field".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.not_ends_with {
                properties.insert(
                    "NotEndsWith".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.not_equals {
                properties.insert(
                    "NotEquals".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.not_starts_with {
                properties.insert(
                    "NotStartsWith".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.starts_with {
                properties.insert(
                    "StartsWith".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-telemetryrule-cloudtrailparameters.html>
    pub struct CloudtrailParameters_ {
        pub advanced_event_selectors: Vec<AdvancedEventSelector_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_TelemetryRule_CloudtrailParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::TelemetryRule.CloudtrailParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_TelemetryRule_CloudtrailParameters as CloudtrailParameters;
    impl crate::value::ToValue for CloudtrailParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AdvancedEventSelectors".to_string(),
                crate::value::ToValue::to_value(&self.advanced_event_selectors),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-telemetryrule-condition.html>
    pub struct Condition_ {
        pub action_condition: Option<Box<ActionCondition_>>,
        pub label_name_condition: Option<Box<LabelNameCondition_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_TelemetryRule_Condition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::TelemetryRule.Condition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_TelemetryRule_Condition as Condition;
    impl crate::value::ToValue for Condition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.action_condition {
                properties.insert(
                    "ActionCondition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.label_name_condition {
                properties.insert(
                    "LabelNameCondition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-telemetryrule-elbloadbalancerloggingparameters.html>
    pub struct ELBLoadBalancerLoggingParameters_ {
        pub field_delimiter: Option<crate::value::ExpString>,
        pub output_format: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_TelemetryRule_ELBLoadBalancerLoggingParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::TelemetryRule.ELBLoadBalancerLoggingParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_TelemetryRule_ELBLoadBalancerLoggingParameters as ELBLoadBalancerLoggingParameters;
    impl crate::value::ToValue for ELBLoadBalancerLoggingParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.field_delimiter {
                properties.insert(
                    "FieldDelimiter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.output_format {
                properties.insert(
                    "OutputFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-telemetryrule-fieldtomatch.html>
    pub struct FieldToMatch_ {
        pub method: Option<crate::value::ExpString>,
        pub query_string: Option<crate::value::ExpString>,
        pub single_header: Option<Box<SingleHeader_>>,
        pub uri_path: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_TelemetryRule_FieldToMatch {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::TelemetryRule.FieldToMatch"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_TelemetryRule_FieldToMatch as FieldToMatch;
    impl crate::value::ToValue for FieldToMatch_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.method {
                properties.insert("Method".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.query_string {
                properties.insert(
                    "QueryString".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.single_header {
                properties.insert(
                    "SingleHeader".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.uri_path {
                properties.insert(
                    "UriPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-telemetryrule-filter.html>
    pub struct Filter_ {
        pub behavior: Option<crate::value::ExpString>,
        pub conditions: Option<Vec<Condition_>>,
        pub requirement: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_TelemetryRule_Filter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::TelemetryRule.Filter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_TelemetryRule_Filter as Filter;
    impl crate::value::ToValue for Filter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.behavior {
                properties.insert(
                    "Behavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.conditions {
                properties.insert(
                    "Conditions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.requirement {
                properties.insert(
                    "Requirement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-telemetryrule-labelnamecondition.html>
    pub struct LabelNameCondition_ {
        pub label_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_TelemetryRule_LabelNameCondition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::TelemetryRule.LabelNameCondition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_TelemetryRule_LabelNameCondition as LabelNameCondition;
    impl crate::value::ToValue for LabelNameCondition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.label_name {
                properties.insert(
                    "LabelName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-telemetryrule-logdeliveryparameters.html>
    pub struct LogDeliveryParameters_ {
        pub log_types: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_TelemetryRule_LogDeliveryParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::TelemetryRule.LogDeliveryParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_TelemetryRule_LogDeliveryParameters as LogDeliveryParameters;
    impl crate::value::ToValue for LogDeliveryParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.log_types {
                properties.insert(
                    "LogTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-telemetryrule-loggingfilter.html>
    pub struct LoggingFilter_ {
        pub default_behavior: Option<crate::value::ExpString>,
        pub filters: Option<Vec<Filter_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_TelemetryRule_LoggingFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::TelemetryRule.LoggingFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_TelemetryRule_LoggingFilter as LoggingFilter;
    impl crate::value::ToValue for LoggingFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.default_behavior {
                properties.insert(
                    "DefaultBehavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.filters {
                properties.insert(
                    "Filters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-telemetryrule-singleheader.html>
    pub struct SingleHeader_ {
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_TelemetryRule_SingleHeader {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::TelemetryRule.SingleHeader"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_TelemetryRule_SingleHeader as SingleHeader;
    impl crate::value::ToValue for SingleHeader_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-telemetryrule-telemetrydestinationconfiguration.html>
    pub struct TelemetryDestinationConfiguration_ {
        pub cloudtrail_parameters: Option<Box<CloudtrailParameters_>>,
        pub destination_pattern: Option<crate::value::ExpString>,
        pub destination_type: Option<crate::value::ExpString>,
        pub elb_load_balancer_logging_parameters: Option<Box<ELBLoadBalancerLoggingParameters_>>,
        pub log_delivery_parameters: Option<Box<LogDeliveryParameters_>>,
        pub retention_in_days: Option<i32>,
        pub vpc_flow_log_parameters: Option<Box<VPCFlowLogParameters_>>,
        pub waf_logging_parameters: Option<Box<WAFLoggingParameters_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_TelemetryRule_TelemetryDestinationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::TelemetryRule.TelemetryDestinationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_TelemetryRule_TelemetryDestinationConfiguration as TelemetryDestinationConfiguration;
    impl crate::value::ToValue for TelemetryDestinationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cloudtrail_parameters {
                properties.insert(
                    "CloudtrailParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.destination_pattern {
                properties.insert(
                    "DestinationPattern".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.destination_type {
                properties.insert(
                    "DestinationType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.elb_load_balancer_logging_parameters {
                properties.insert(
                    "ELBLoadBalancerLoggingParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.log_delivery_parameters {
                properties.insert(
                    "LogDeliveryParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.retention_in_days {
                properties.insert(
                    "RetentionInDays".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpc_flow_log_parameters {
                properties.insert(
                    "VPCFlowLogParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.waf_logging_parameters {
                properties.insert(
                    "WAFLoggingParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-telemetryrule-telemetryrule.html>
    pub struct TelemetryRule_ {
        pub destination_configuration: Option<Box<TelemetryDestinationConfiguration_>>,
        pub resource_type: crate::value::ExpString,
        pub selection_criteria: Option<crate::value::ExpString>,
        pub telemetry_source_types: Option<Vec<crate::value::ExpString>>,
        pub telemetry_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_TelemetryRule_TelemetryRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::TelemetryRule.TelemetryRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_TelemetryRule_TelemetryRule as TelemetryRule;
    impl crate::value::ToValue for TelemetryRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.destination_configuration {
                properties.insert(
                    "DestinationConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ResourceType".to_string(),
                crate::value::ToValue::to_value(&self.resource_type),
            );
            if let Some(ref value) = self.selection_criteria {
                properties.insert(
                    "SelectionCriteria".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.telemetry_source_types {
                properties.insert(
                    "TelemetrySourceTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TelemetryType".to_string(),
                crate::value::ToValue::to_value(&self.telemetry_type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-telemetryrule-vpcflowlogparameters.html>
    pub struct VPCFlowLogParameters_ {
        pub log_format: Option<crate::value::ExpString>,
        pub max_aggregation_interval: Option<i32>,
        pub traffic_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_TelemetryRule_VPCFlowLogParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::TelemetryRule.VPCFlowLogParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_TelemetryRule_VPCFlowLogParameters as VPCFlowLogParameters;
    impl crate::value::ToValue for VPCFlowLogParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.log_format {
                properties.insert(
                    "LogFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_aggregation_interval {
                properties.insert(
                    "MaxAggregationInterval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.traffic_type {
                properties.insert(
                    "TrafficType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-telemetryrule-wafloggingparameters.html>
    pub struct WAFLoggingParameters_ {
        pub log_type: Option<crate::value::ExpString>,
        pub logging_filter: Option<Box<LoggingFilter_>>,
        pub redacted_fields: Option<Vec<FieldToMatch_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_TelemetryRule_WAFLoggingParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::TelemetryRule.WAFLoggingParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_TelemetryRule_WAFLoggingParameters as WAFLoggingParameters;
    impl crate::value::ToValue for WAFLoggingParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.log_type {
                properties.insert(
                    "LogType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.logging_filter {
                properties.insert(
                    "LoggingFilter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.redacted_fields {
                properties.insert(
                    "RedactedFields".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-observabilityadmin-organizationcentralizationrule.html>
pub struct OrganizationCentralizationRule_ {
    pub rule: super::observabilityadmin::organizationcentralizationrule::CentralizationRule_,
    pub rule_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_observabilityadmin_OrganizationCentralizationRule {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ObservabilityAdmin::OrganizationCentralizationRule"
        $($field $value)*)
    };
}
pub use crate::__aws_observabilityadmin_OrganizationCentralizationRule as OrganizationCentralizationRule;
impl crate::template::ToResource for OrganizationCentralizationRule_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ObservabilityAdmin"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "OrganizationCentralizationRule",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Rule".to_string(),
            crate::value::ToValue::to_value(&self.rule),
        );
        properties.insert(
            "RuleName".to_string(),
            crate::value::ToValue::to_value(&self.rule_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-observabilityadmin-organizationtelemetryrule.html>
pub struct OrganizationTelemetryRule_ {
    pub rule: super::observabilityadmin::organizationtelemetryrule::TelemetryRule_,
    pub rule_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_observabilityadmin_OrganizationTelemetryRule {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ObservabilityAdmin::OrganizationTelemetryRule"
        $($field $value)*)
    };
}
pub use crate::__aws_observabilityadmin_OrganizationTelemetryRule as OrganizationTelemetryRule;
impl crate::template::ToResource for OrganizationTelemetryRule_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ObservabilityAdmin"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("OrganizationTelemetryRule"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Rule".to_string(),
            crate::value::ToValue::to_value(&self.rule),
        );
        properties.insert(
            "RuleName".to_string(),
            crate::value::ToValue::to_value(&self.rule_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-observabilityadmin-s3tableintegration.html>
pub struct S3TableIntegration_ {
    pub encryption: super::observabilityadmin::s3tableintegration::EncryptionConfig_,
    pub log_sources: Option<Vec<super::observabilityadmin::s3tableintegration::LogSource_>>,
    pub role_arn: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_observabilityadmin_S3TableIntegration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ObservabilityAdmin::S3TableIntegration"
        $($field $value)*)
    };
}
pub use crate::__aws_observabilityadmin_S3TableIntegration as S3TableIntegration;
impl crate::template::ToResource for S3TableIntegration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ObservabilityAdmin"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("S3TableIntegration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Encryption".to_string(),
            crate::value::ToValue::to_value(&self.encryption),
        );
        if let Some(ref value) = self.log_sources {
            properties.insert(
                "LogSources".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-observabilityadmin-telemetrypipelines.html>
pub struct TelemetryPipelines_ {
    pub configuration:
        super::observabilityadmin::telemetrypipelines::TelemetryPipelineConfiguration_,
    pub name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_observabilityadmin_TelemetryPipelines {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ObservabilityAdmin::TelemetryPipelines"
        $($field $value)*)
    };
}
pub use crate::__aws_observabilityadmin_TelemetryPipelines as TelemetryPipelines;
impl crate::template::ToResource for TelemetryPipelines_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ObservabilityAdmin"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TelemetryPipelines"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Configuration".to_string(),
            crate::value::ToValue::to_value(&self.configuration),
        );
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-observabilityadmin-telemetryrule.html>
pub struct TelemetryRule_ {
    pub rule: super::observabilityadmin::telemetryrule::TelemetryRule_,
    pub rule_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_observabilityadmin_TelemetryRule {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ObservabilityAdmin::TelemetryRule"
        $($field $value)*)
    };
}
pub use crate::__aws_observabilityadmin_TelemetryRule as TelemetryRule;
impl crate::template::ToResource for TelemetryRule_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ObservabilityAdmin"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TelemetryRule"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Rule".to_string(),
            crate::value::ToValue::to_value(&self.rule),
        );
        properties.insert(
            "RuleName".to_string(),
            crate::value::ToValue::to_value(&self.rule_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
