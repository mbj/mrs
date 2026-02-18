pub mod backupplan {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupplan-advancedbackupsettingresourcetype.html>
    pub struct AdvancedBackupSettingResourceType_ {
        pub backup_options: serde_json::Value,
        pub resource_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_backup_BackupPlan_AdvancedBackupSettingResourceType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Backup::BackupPlan.AdvancedBackupSettingResourceType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_backup_BackupPlan_AdvancedBackupSettingResourceType as AdvancedBackupSettingResourceType;
    impl crate::value::ToValue for AdvancedBackupSettingResourceType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BackupOptions".to_string(),
                crate::value::ToValue::to_value(&self.backup_options),
            );
            properties.insert(
                "ResourceType".to_string(),
                crate::value::ToValue::to_value(&self.resource_type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupplan-backupplanresourcetype.html>
    pub struct BackupPlanResourceType_ {
        pub advanced_backup_settings: Option<Vec<AdvancedBackupSettingResourceType_>>,
        pub backup_plan_name: crate::value::ExpString,
        pub backup_plan_rule: Vec<BackupRuleResourceType_>,
        pub scan_settings: Option<Vec<ScanSettingResourceType_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_backup_BackupPlan_BackupPlanResourceType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Backup::BackupPlan.BackupPlanResourceType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_backup_BackupPlan_BackupPlanResourceType as BackupPlanResourceType;
    impl crate::value::ToValue for BackupPlanResourceType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.advanced_backup_settings {
                properties.insert(
                    "AdvancedBackupSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "BackupPlanName".to_string(),
                crate::value::ToValue::to_value(&self.backup_plan_name),
            );
            properties.insert(
                "BackupPlanRule".to_string(),
                crate::value::ToValue::to_value(&self.backup_plan_rule),
            );
            if let Some(ref value) = self.scan_settings {
                properties.insert(
                    "ScanSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupplan-backupruleresourcetype.html>
    pub struct BackupRuleResourceType_ {
        pub completion_window_minutes: Option<f64>,
        pub copy_actions: Option<Vec<CopyActionResourceType_>>,
        pub enable_continuous_backup: Option<crate::value::ExpBool>,
        pub index_actions: Option<Vec<IndexActionsResourceType_>>,
        pub lifecycle: Option<Box<LifecycleResourceType_>>,
        pub recovery_point_tags:
            Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub rule_name: crate::value::ExpString,
        pub scan_actions: Option<Vec<ScanActionResourceType_>>,
        pub schedule_expression: Option<crate::value::ExpString>,
        pub schedule_expression_timezone: Option<crate::value::ExpString>,
        pub start_window_minutes: Option<f64>,
        pub target_backup_vault: crate::value::ExpString,
        pub target_logically_air_gapped_backup_vault_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_backup_BackupPlan_BackupRuleResourceType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Backup::BackupPlan.BackupRuleResourceType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_backup_BackupPlan_BackupRuleResourceType as BackupRuleResourceType;
    impl crate::value::ToValue for BackupRuleResourceType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.completion_window_minutes {
                properties.insert(
                    "CompletionWindowMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.copy_actions {
                properties.insert(
                    "CopyActions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enable_continuous_backup {
                properties.insert(
                    "EnableContinuousBackup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.index_actions {
                properties.insert(
                    "IndexActions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lifecycle {
                properties.insert(
                    "Lifecycle".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.recovery_point_tags {
                properties.insert(
                    "RecoveryPointTags".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RuleName".to_string(),
                crate::value::ToValue::to_value(&self.rule_name),
            );
            if let Some(ref value) = self.scan_actions {
                properties.insert(
                    "ScanActions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.schedule_expression {
                properties.insert(
                    "ScheduleExpression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.schedule_expression_timezone {
                properties.insert(
                    "ScheduleExpressionTimezone".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.start_window_minutes {
                properties.insert(
                    "StartWindowMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TargetBackupVault".to_string(),
                crate::value::ToValue::to_value(&self.target_backup_vault),
            );
            if let Some(ref value) = self.target_logically_air_gapped_backup_vault_arn {
                properties.insert(
                    "TargetLogicallyAirGappedBackupVaultArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupplan-copyactionresourcetype.html>
    pub struct CopyActionResourceType_ {
        pub destination_backup_vault_arn: crate::value::ExpString,
        pub lifecycle: Option<Box<LifecycleResourceType_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_backup_BackupPlan_CopyActionResourceType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Backup::BackupPlan.CopyActionResourceType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_backup_BackupPlan_CopyActionResourceType as CopyActionResourceType;
    impl crate::value::ToValue for CopyActionResourceType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DestinationBackupVaultArn".to_string(),
                crate::value::ToValue::to_value(&self.destination_backup_vault_arn),
            );
            if let Some(ref value) = self.lifecycle {
                properties.insert(
                    "Lifecycle".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupplan-indexactionsresourcetype.html>
    pub struct IndexActionsResourceType_ {
        pub resource_types: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_backup_BackupPlan_IndexActionsResourceType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Backup::BackupPlan.IndexActionsResourceType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_backup_BackupPlan_IndexActionsResourceType as IndexActionsResourceType;
    impl crate::value::ToValue for IndexActionsResourceType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.resource_types {
                properties.insert(
                    "ResourceTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupplan-lifecycleresourcetype.html>
    pub struct LifecycleResourceType_ {
        pub delete_after_days: Option<f64>,
        pub move_to_cold_storage_after_days: Option<f64>,
        pub opt_in_to_archive_for_supported_resources: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_backup_BackupPlan_LifecycleResourceType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Backup::BackupPlan.LifecycleResourceType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_backup_BackupPlan_LifecycleResourceType as LifecycleResourceType;
    impl crate::value::ToValue for LifecycleResourceType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.delete_after_days {
                properties.insert(
                    "DeleteAfterDays".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.move_to_cold_storage_after_days {
                properties.insert(
                    "MoveToColdStorageAfterDays".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.opt_in_to_archive_for_supported_resources {
                properties.insert(
                    "OptInToArchiveForSupportedResources".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupplan-scanactionresourcetype.html>
    pub struct ScanActionResourceType_ {
        pub malware_scanner: Option<crate::value::ExpString>,
        pub scan_mode: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_backup_BackupPlan_ScanActionResourceType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Backup::BackupPlan.ScanActionResourceType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_backup_BackupPlan_ScanActionResourceType as ScanActionResourceType;
    impl crate::value::ToValue for ScanActionResourceType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.malware_scanner {
                properties.insert(
                    "MalwareScanner".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scan_mode {
                properties.insert(
                    "ScanMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupplan-scansettingresourcetype.html>
    pub struct ScanSettingResourceType_ {
        pub malware_scanner: Option<crate::value::ExpString>,
        pub resource_types: Option<Vec<crate::value::ExpString>>,
        pub scanner_role_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_backup_BackupPlan_ScanSettingResourceType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Backup::BackupPlan.ScanSettingResourceType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_backup_BackupPlan_ScanSettingResourceType as ScanSettingResourceType;
    impl crate::value::ToValue for ScanSettingResourceType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.malware_scanner {
                properties.insert(
                    "MalwareScanner".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_types {
                properties.insert(
                    "ResourceTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scanner_role_arn {
                properties.insert(
                    "ScannerRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod backupselection {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupselection-backupselectionresourcetype.html>
    pub struct BackupSelectionResourceType_ {
        pub conditions: Option<Box<Conditions_>>,
        pub iam_role_arn: crate::value::ExpString,
        pub list_of_tags: Option<Vec<ConditionResourceType_>>,
        pub not_resources: Option<Vec<crate::value::ExpString>>,
        pub resources: Option<Vec<crate::value::ExpString>>,
        pub selection_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_backup_BackupSelection_BackupSelectionResourceType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Backup::BackupSelection.BackupSelectionResourceType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_backup_BackupSelection_BackupSelectionResourceType as BackupSelectionResourceType;
    impl crate::value::ToValue for BackupSelectionResourceType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.conditions {
                properties.insert(
                    "Conditions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "IamRoleArn".to_string(),
                crate::value::ToValue::to_value(&self.iam_role_arn),
            );
            if let Some(ref value) = self.list_of_tags {
                properties.insert(
                    "ListOfTags".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.not_resources {
                properties.insert(
                    "NotResources".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resources {
                properties.insert(
                    "Resources".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SelectionName".to_string(),
                crate::value::ToValue::to_value(&self.selection_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupselection-conditionparameter.html>
    pub struct ConditionParameter_ {
        pub condition_key: Option<crate::value::ExpString>,
        pub condition_value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_backup_BackupSelection_ConditionParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Backup::BackupSelection.ConditionParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_backup_BackupSelection_ConditionParameter as ConditionParameter;
    impl crate::value::ToValue for ConditionParameter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.condition_key {
                properties.insert(
                    "ConditionKey".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.condition_value {
                properties.insert(
                    "ConditionValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupselection-conditionresourcetype.html>
    pub struct ConditionResourceType_ {
        pub condition_key: crate::value::ExpString,
        pub condition_type: crate::value::ExpString,
        pub condition_value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_backup_BackupSelection_ConditionResourceType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Backup::BackupSelection.ConditionResourceType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_backup_BackupSelection_ConditionResourceType as ConditionResourceType;
    impl crate::value::ToValue for ConditionResourceType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ConditionKey".to_string(),
                crate::value::ToValue::to_value(&self.condition_key),
            );
            properties.insert(
                "ConditionType".to_string(),
                crate::value::ToValue::to_value(&self.condition_type),
            );
            properties.insert(
                "ConditionValue".to_string(),
                crate::value::ToValue::to_value(&self.condition_value),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupselection-conditions.html>
    pub struct Conditions_ {
        pub string_equals: Option<Vec<ConditionParameter_>>,
        pub string_like: Option<Vec<ConditionParameter_>>,
        pub string_not_equals: Option<Vec<ConditionParameter_>>,
        pub string_not_like: Option<Vec<ConditionParameter_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_backup_BackupSelection_Conditions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Backup::BackupSelection.Conditions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_backup_BackupSelection_Conditions as Conditions;
    impl crate::value::ToValue for Conditions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.string_equals {
                properties.insert(
                    "StringEquals".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.string_like {
                properties.insert(
                    "StringLike".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.string_not_equals {
                properties.insert(
                    "StringNotEquals".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.string_not_like {
                properties.insert(
                    "StringNotLike".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod backupvault {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupvault-lockconfigurationtype.html>
    pub struct LockConfigurationType_ {
        pub changeable_for_days: Option<i32>,
        pub max_retention_days: Option<i32>,
        pub min_retention_days: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_backup_BackupVault_LockConfigurationType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Backup::BackupVault.LockConfigurationType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_backup_BackupVault_LockConfigurationType as LockConfigurationType;
    impl crate::value::ToValue for LockConfigurationType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.changeable_for_days {
                properties.insert(
                    "ChangeableForDays".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_retention_days {
                properties.insert(
                    "MaxRetentionDays".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MinRetentionDays".to_string(),
                crate::value::ToValue::to_value(&self.min_retention_days),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-backupvault-notificationobjecttype.html>
    pub struct NotificationObjectType_ {
        pub backup_vault_events: Vec<crate::value::ExpString>,
        pub sns_topic_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_backup_BackupVault_NotificationObjectType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Backup::BackupVault.NotificationObjectType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_backup_BackupVault_NotificationObjectType as NotificationObjectType;
    impl crate::value::ToValue for NotificationObjectType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BackupVaultEvents".to_string(),
                crate::value::ToValue::to_value(&self.backup_vault_events),
            );
            properties.insert(
                "SNSTopicArn".to_string(),
                crate::value::ToValue::to_value(&self.sns_topic_arn),
            );
            properties.into()
        }
    }
}
pub mod framework {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-framework-controlinputparameter.html>
    pub struct ControlInputParameter_ {
        pub parameter_name: crate::value::ExpString,
        pub parameter_value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_backup_Framework_ControlInputParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Backup::Framework.ControlInputParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_backup_Framework_ControlInputParameter as ControlInputParameter;
    impl crate::value::ToValue for ControlInputParameter_ {
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-framework-controlscope.html>
    pub struct ControlScope_ {
        pub compliance_resource_ids: Option<Vec<crate::value::ExpString>>,
        pub compliance_resource_types: Option<Vec<crate::value::ExpString>>,
        pub tags: Option<Vec<crate::Tag_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_backup_Framework_ControlScope {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Backup::Framework.ControlScope"
            $($field $value)*)
        };
    }
    pub use crate::__aws_backup_Framework_ControlScope as ControlScope;
    impl crate::value::ToValue for ControlScope_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.compliance_resource_ids {
                properties.insert(
                    "ComplianceResourceIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.compliance_resource_types {
                properties.insert(
                    "ComplianceResourceTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tags {
                properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-framework-frameworkcontrol.html>
    pub struct FrameworkControl_ {
        pub control_input_parameters: Option<Vec<ControlInputParameter_>>,
        pub control_name: crate::value::ExpString,
        pub control_scope: Option<Box<ControlScope_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_backup_Framework_FrameworkControl {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Backup::Framework.FrameworkControl"
            $($field $value)*)
        };
    }
    pub use crate::__aws_backup_Framework_FrameworkControl as FrameworkControl;
    impl crate::value::ToValue for FrameworkControl_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.control_input_parameters {
                properties.insert(
                    "ControlInputParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ControlName".to_string(),
                crate::value::ToValue::to_value(&self.control_name),
            );
            if let Some(ref value) = self.control_scope {
                properties.insert(
                    "ControlScope".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod logicallyairgappedbackupvault {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-logicallyairgappedbackupvault-notificationobjecttype.html>
    pub struct NotificationObjectType_ {
        pub backup_vault_events: Vec<crate::value::ExpString>,
        pub sns_topic_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_backup_LogicallyAirGappedBackupVault_NotificationObjectType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Backup::LogicallyAirGappedBackupVault.NotificationObjectType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_backup_LogicallyAirGappedBackupVault_NotificationObjectType as NotificationObjectType;
    impl crate::value::ToValue for NotificationObjectType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BackupVaultEvents".to_string(),
                crate::value::ToValue::to_value(&self.backup_vault_events),
            );
            properties.insert(
                "SNSTopicArn".to_string(),
                crate::value::ToValue::to_value(&self.sns_topic_arn),
            );
            properties.into()
        }
    }
}
pub mod reportplan {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-reportplan-reportdeliverychannel.html>
    pub struct ReportDeliveryChannel_ {
        pub formats: Option<Vec<crate::value::ExpString>>,
        pub s3_bucket_name: crate::value::ExpString,
        pub s3_key_prefix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_backup_ReportPlan_ReportDeliveryChannel {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Backup::ReportPlan.ReportDeliveryChannel"
            $($field $value)*)
        };
    }
    pub use crate::__aws_backup_ReportPlan_ReportDeliveryChannel as ReportDeliveryChannel;
    impl crate::value::ToValue for ReportDeliveryChannel_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.formats {
                properties.insert(
                    "Formats".to_string(),
                    crate::value::ToValue::to_value(value),
                );
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
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-reportplan-reportsetting.html>
    pub struct ReportSetting_ {
        pub accounts: Option<Vec<crate::value::ExpString>>,
        pub framework_arns: Option<Vec<crate::value::ExpString>>,
        pub organization_units: Option<Vec<crate::value::ExpString>>,
        pub regions: Option<Vec<crate::value::ExpString>>,
        pub report_template: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_backup_ReportPlan_ReportSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Backup::ReportPlan.ReportSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_backup_ReportPlan_ReportSetting as ReportSetting;
    impl crate::value::ToValue for ReportSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.accounts {
                properties.insert(
                    "Accounts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.framework_arns {
                properties.insert(
                    "FrameworkArns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.organization_units {
                properties.insert(
                    "OrganizationUnits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.regions {
                properties.insert(
                    "Regions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ReportTemplate".to_string(),
                crate::value::ToValue::to_value(&self.report_template),
            );
            properties.into()
        }
    }
}
pub mod restoretestingplan {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-restoretestingplan-restoretestingrecoverypointselection.html>
    pub struct RestoreTestingRecoveryPointSelection_ {
        pub algorithm: crate::value::ExpString,
        pub exclude_vaults: Option<Vec<crate::value::ExpString>>,
        pub include_vaults: Vec<crate::value::ExpString>,
        pub recovery_point_types: Vec<crate::value::ExpString>,
        pub selection_window_days: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_backup_RestoreTestingPlan_RestoreTestingRecoveryPointSelection {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Backup::RestoreTestingPlan.RestoreTestingRecoveryPointSelection"
            $($field $value)*)
        };
    }
    pub use crate::__aws_backup_RestoreTestingPlan_RestoreTestingRecoveryPointSelection as RestoreTestingRecoveryPointSelection;
    impl crate::value::ToValue for RestoreTestingRecoveryPointSelection_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Algorithm".to_string(),
                crate::value::ToValue::to_value(&self.algorithm),
            );
            if let Some(ref value) = self.exclude_vaults {
                properties.insert(
                    "ExcludeVaults".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "IncludeVaults".to_string(),
                crate::value::ToValue::to_value(&self.include_vaults),
            );
            properties.insert(
                "RecoveryPointTypes".to_string(),
                crate::value::ToValue::to_value(&self.recovery_point_types),
            );
            if let Some(ref value) = self.selection_window_days {
                properties.insert(
                    "SelectionWindowDays".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod restoretestingselection {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-restoretestingselection-keyvalue.html>
    pub struct KeyValue_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_backup_RestoreTestingSelection_KeyValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Backup::RestoreTestingSelection.KeyValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_backup_RestoreTestingSelection_KeyValue as KeyValue;
    impl crate::value::ToValue for KeyValue_ {
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-restoretestingselection-protectedresourceconditions.html>
    pub struct ProtectedResourceConditions_ {
        pub string_equals: Option<Vec<KeyValue_>>,
        pub string_not_equals: Option<Vec<KeyValue_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_backup_RestoreTestingSelection_ProtectedResourceConditions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Backup::RestoreTestingSelection.ProtectedResourceConditions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_backup_RestoreTestingSelection_ProtectedResourceConditions as ProtectedResourceConditions;
    impl crate::value::ToValue for ProtectedResourceConditions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.string_equals {
                properties.insert(
                    "StringEquals".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.string_not_equals {
                properties.insert(
                    "StringNotEquals".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod tieringconfiguration {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-backup-tieringconfiguration-resourceselection.html>
    pub struct ResourceSelection_ {
        pub resource_type: crate::value::ExpString,
        pub resources: Vec<crate::value::ExpString>,
        pub tiering_down_settings_in_days: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_backup_TieringConfiguration_ResourceSelection {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Backup::TieringConfiguration.ResourceSelection"
            $($field $value)*)
        };
    }
    pub use crate::__aws_backup_TieringConfiguration_ResourceSelection as ResourceSelection;
    impl crate::value::ToValue for ResourceSelection_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ResourceType".to_string(),
                crate::value::ToValue::to_value(&self.resource_type),
            );
            properties.insert(
                "Resources".to_string(),
                crate::value::ToValue::to_value(&self.resources),
            );
            properties.insert(
                "TieringDownSettingsInDays".to_string(),
                crate::value::ToValue::to_value(&self.tiering_down_settings_in_days),
            );
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-backupplan.html>
pub struct BackupPlan_ {
    pub backup_plan: super::backup::backupplan::BackupPlanResourceType_,
    pub backup_plan_tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_backup_BackupPlan {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Backup::BackupPlan"
        $($field $value)*)
    };
}
pub use crate::__aws_backup_BackupPlan as BackupPlan;
impl crate::template::ToResource for BackupPlan_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Backup"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("BackupPlan"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "BackupPlan".to_string(),
            crate::value::ToValue::to_value(&self.backup_plan),
        );
        if let Some(ref value) = self.backup_plan_tags {
            properties.insert(
                "BackupPlanTags".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-backupselection.html>
pub struct BackupSelection_ {
    pub backup_plan_id: crate::value::ExpString,
    pub backup_selection: super::backup::backupselection::BackupSelectionResourceType_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_backup_BackupSelection {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Backup::BackupSelection"
        $($field $value)*)
    };
}
pub use crate::__aws_backup_BackupSelection as BackupSelection;
impl crate::template::ToResource for BackupSelection_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Backup"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("BackupSelection"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "BackupPlanId".to_string(),
            crate::value::ToValue::to_value(&self.backup_plan_id),
        );
        properties.insert(
            "BackupSelection".to_string(),
            crate::value::ToValue::to_value(&self.backup_selection),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-backupvault.html>
pub struct BackupVault_ {
    pub access_policy: Option<serde_json::Value>,
    pub backup_vault_name: crate::value::ExpString,
    pub backup_vault_tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub encryption_key_arn: Option<crate::value::ExpString>,
    pub lock_configuration: Option<super::backup::backupvault::LockConfigurationType_>,
    pub notifications: Option<super::backup::backupvault::NotificationObjectType_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_backup_BackupVault {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Backup::BackupVault"
        $($field $value)*)
    };
}
pub use crate::__aws_backup_BackupVault as BackupVault;
impl crate::template::ToResource for BackupVault_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Backup"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("BackupVault"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.access_policy {
            properties.insert(
                "AccessPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "BackupVaultName".to_string(),
            crate::value::ToValue::to_value(&self.backup_vault_name),
        );
        if let Some(ref value) = self.backup_vault_tags {
            properties.insert(
                "BackupVaultTags".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.encryption_key_arn {
            properties.insert(
                "EncryptionKeyArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.lock_configuration {
            properties.insert(
                "LockConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.notifications {
            properties.insert(
                "Notifications".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-framework.html>
pub struct Framework_ {
    pub framework_controls: Vec<super::backup::framework::FrameworkControl_>,
    pub framework_description: Option<crate::value::ExpString>,
    pub framework_name: Option<crate::value::ExpString>,
    pub framework_tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_backup_Framework {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Backup::Framework"
        $($field $value)*)
    };
}
pub use crate::__aws_backup_Framework as Framework;
impl crate::template::ToResource for Framework_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Backup"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Framework"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "FrameworkControls".to_string(),
            crate::value::ToValue::to_value(&self.framework_controls),
        );
        if let Some(ref value) = self.framework_description {
            properties.insert(
                "FrameworkDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.framework_name {
            properties.insert(
                "FrameworkName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.framework_tags {
            properties.insert(
                "FrameworkTags".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-logicallyairgappedbackupvault.html>
pub struct LogicallyAirGappedBackupVault_ {
    pub access_policy: Option<serde_json::Value>,
    pub backup_vault_name: crate::value::ExpString,
    pub backup_vault_tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub encryption_key_arn: Option<crate::value::ExpString>,
    pub max_retention_days: i32,
    pub min_retention_days: i32,
    pub mpa_approval_team_arn: Option<crate::value::ExpString>,
    pub notifications:
        Option<super::backup::logicallyairgappedbackupvault::NotificationObjectType_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_backup_LogicallyAirGappedBackupVault {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Backup::LogicallyAirGappedBackupVault"
        $($field $value)*)
    };
}
pub use crate::__aws_backup_LogicallyAirGappedBackupVault as LogicallyAirGappedBackupVault;
impl crate::template::ToResource for LogicallyAirGappedBackupVault_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Backup"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "LogicallyAirGappedBackupVault",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.access_policy {
            properties.insert(
                "AccessPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "BackupVaultName".to_string(),
            crate::value::ToValue::to_value(&self.backup_vault_name),
        );
        if let Some(ref value) = self.backup_vault_tags {
            properties.insert(
                "BackupVaultTags".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.encryption_key_arn {
            properties.insert(
                "EncryptionKeyArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "MaxRetentionDays".to_string(),
            crate::value::ToValue::to_value(&self.max_retention_days),
        );
        properties.insert(
            "MinRetentionDays".to_string(),
            crate::value::ToValue::to_value(&self.min_retention_days),
        );
        if let Some(ref value) = self.mpa_approval_team_arn {
            properties.insert(
                "MpaApprovalTeamArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.notifications {
            properties.insert(
                "Notifications".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-reportplan.html>
pub struct ReportPlan_ {
    pub report_delivery_channel: super::backup::reportplan::ReportDeliveryChannel_,
    pub report_plan_description: Option<crate::value::ExpString>,
    pub report_plan_name: Option<crate::value::ExpString>,
    pub report_plan_tags: Option<Vec<crate::Tag_>>,
    pub report_setting: super::backup::reportplan::ReportSetting_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_backup_ReportPlan {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Backup::ReportPlan"
        $($field $value)*)
    };
}
pub use crate::__aws_backup_ReportPlan as ReportPlan;
impl crate::template::ToResource for ReportPlan_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Backup"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ReportPlan"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ReportDeliveryChannel".to_string(),
            crate::value::ToValue::to_value(&self.report_delivery_channel),
        );
        if let Some(ref value) = self.report_plan_description {
            properties.insert(
                "ReportPlanDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.report_plan_name {
            properties.insert(
                "ReportPlanName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.report_plan_tags {
            properties.insert(
                "ReportPlanTags".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ReportSetting".to_string(),
            crate::value::ToValue::to_value(&self.report_setting),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-restoretestingplan.html>
pub struct RestoreTestingPlan_ {
    pub recovery_point_selection:
        super::backup::restoretestingplan::RestoreTestingRecoveryPointSelection_,
    pub restore_testing_plan_name: crate::value::ExpString,
    pub schedule_expression: crate::value::ExpString,
    pub schedule_expression_timezone: Option<crate::value::ExpString>,
    pub start_window_hours: Option<i32>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_backup_RestoreTestingPlan {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Backup::RestoreTestingPlan"
        $($field $value)*)
    };
}
pub use crate::__aws_backup_RestoreTestingPlan as RestoreTestingPlan;
impl crate::template::ToResource for RestoreTestingPlan_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Backup"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RestoreTestingPlan"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "RecoveryPointSelection".to_string(),
            crate::value::ToValue::to_value(&self.recovery_point_selection),
        );
        properties.insert(
            "RestoreTestingPlanName".to_string(),
            crate::value::ToValue::to_value(&self.restore_testing_plan_name),
        );
        properties.insert(
            "ScheduleExpression".to_string(),
            crate::value::ToValue::to_value(&self.schedule_expression),
        );
        if let Some(ref value) = self.schedule_expression_timezone {
            properties.insert(
                "ScheduleExpressionTimezone".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.start_window_hours {
            properties.insert(
                "StartWindowHours".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-restoretestingselection.html>
pub struct RestoreTestingSelection_ {
    pub iam_role_arn: crate::value::ExpString,
    pub protected_resource_arns: Option<Vec<crate::value::ExpString>>,
    pub protected_resource_conditions:
        Option<super::backup::restoretestingselection::ProtectedResourceConditions_>,
    pub protected_resource_type: crate::value::ExpString,
    pub restore_metadata_overrides:
        Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub restore_testing_plan_name: crate::value::ExpString,
    pub restore_testing_selection_name: crate::value::ExpString,
    pub validation_window_hours: Option<i32>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_backup_RestoreTestingSelection {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Backup::RestoreTestingSelection"
        $($field $value)*)
    };
}
pub use crate::__aws_backup_RestoreTestingSelection as RestoreTestingSelection;
impl crate::template::ToResource for RestoreTestingSelection_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Backup"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RestoreTestingSelection"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "IamRoleArn".to_string(),
            crate::value::ToValue::to_value(&self.iam_role_arn),
        );
        if let Some(ref value) = self.protected_resource_arns {
            properties.insert(
                "ProtectedResourceArns".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.protected_resource_conditions {
            properties.insert(
                "ProtectedResourceConditions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ProtectedResourceType".to_string(),
            crate::value::ToValue::to_value(&self.protected_resource_type),
        );
        if let Some(ref value) = self.restore_metadata_overrides {
            properties.insert(
                "RestoreMetadataOverrides".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RestoreTestingPlanName".to_string(),
            crate::value::ToValue::to_value(&self.restore_testing_plan_name),
        );
        properties.insert(
            "RestoreTestingSelectionName".to_string(),
            crate::value::ToValue::to_value(&self.restore_testing_selection_name),
        );
        if let Some(ref value) = self.validation_window_hours {
            properties.insert(
                "ValidationWindowHours".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backup-tieringconfiguration.html>
pub struct TieringConfiguration_ {
    pub backup_vault_name: crate::value::ExpString,
    pub resource_selection: Vec<super::backup::tieringconfiguration::ResourceSelection_>,
    pub tiering_configuration_name: crate::value::ExpString,
    pub tiering_configuration_tags:
        Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_backup_TieringConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Backup::TieringConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_backup_TieringConfiguration as TieringConfiguration;
impl crate::template::ToResource for TieringConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Backup"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TieringConfiguration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "BackupVaultName".to_string(),
            crate::value::ToValue::to_value(&self.backup_vault_name),
        );
        properties.insert(
            "ResourceSelection".to_string(),
            crate::value::ToValue::to_value(&self.resource_selection),
        );
        properties.insert(
            "TieringConfigurationName".to_string(),
            crate::value::ToValue::to_value(&self.tiering_configuration_name),
        );
        if let Some(ref value) = self.tiering_configuration_tags {
            properties.insert(
                "TieringConfigurationTags".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
