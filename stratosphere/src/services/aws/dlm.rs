pub mod lifecyclepolicy {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-action.html
    pub struct Action_ {
        pub cross_region_copy: Vec<CrossRegionCopyAction_>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dlm_LifecyclePolicy_Action {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::DLM::LifecyclePolicy.Action"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dlm_LifecyclePolicy_Action as Action;
    impl crate::value::ToValue for Action_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CrossRegionCopy".to_string(),
                crate::value::ToValue::to_value(&self.cross_region_copy),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-archiveretainrule.html
    pub struct ArchiveRetainRule_ {
        pub retention_archive_tier: Box<RetentionArchiveTier_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dlm_LifecyclePolicy_ArchiveRetainRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::DLM::LifecyclePolicy.ArchiveRetainRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dlm_LifecyclePolicy_ArchiveRetainRule as ArchiveRetainRule;
    impl crate::value::ToValue for ArchiveRetainRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RetentionArchiveTier".to_string(),
                crate::value::ToValue::to_value(&self.retention_archive_tier),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-archiverule.html
    pub struct ArchiveRule_ {
        pub retain_rule: Box<ArchiveRetainRule_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dlm_LifecyclePolicy_ArchiveRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::DLM::LifecyclePolicy.ArchiveRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dlm_LifecyclePolicy_ArchiveRule as ArchiveRule;
    impl crate::value::ToValue for ArchiveRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RetainRule".to_string(),
                crate::value::ToValue::to_value(&self.retain_rule),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-createrule.html
    pub struct CreateRule_ {
        pub cron_expression: Option<crate::value::ExpString>,
        pub interval: Option<i64>,
        pub interval_unit: Option<crate::value::ExpString>,
        pub location: Option<crate::value::ExpString>,
        pub scripts: Option<Vec<Script_>>,
        pub times: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dlm_LifecyclePolicy_CreateRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::DLM::LifecyclePolicy.CreateRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dlm_LifecyclePolicy_CreateRule as CreateRule;
    impl crate::value::ToValue for CreateRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cron_expression {
                properties.insert(
                    "CronExpression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.interval {
                properties.insert(
                    "Interval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.interval_unit {
                properties.insert(
                    "IntervalUnit".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.location {
                properties.insert(
                    "Location".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scripts {
                properties.insert(
                    "Scripts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.times {
                properties.insert("Times".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-crossregioncopyaction.html
    pub struct CrossRegionCopyAction_ {
        pub encryption_configuration: Box<EncryptionConfiguration_>,
        pub retain_rule: Option<Box<CrossRegionCopyRetainRule_>>,
        pub target: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dlm_LifecyclePolicy_CrossRegionCopyAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::DLM::LifecyclePolicy.CrossRegionCopyAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dlm_LifecyclePolicy_CrossRegionCopyAction as CrossRegionCopyAction;
    impl crate::value::ToValue for CrossRegionCopyAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EncryptionConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.encryption_configuration),
            );
            if let Some(ref value) = self.retain_rule {
                properties.insert(
                    "RetainRule".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Target".to_string(),
                crate::value::ToValue::to_value(&self.target),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-crossregioncopydeprecaterule.html
    pub struct CrossRegionCopyDeprecateRule_ {
        pub interval: i64,
        pub interval_unit: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dlm_LifecyclePolicy_CrossRegionCopyDeprecateRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::DLM::LifecyclePolicy.CrossRegionCopyDeprecateRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dlm_LifecyclePolicy_CrossRegionCopyDeprecateRule as CrossRegionCopyDeprecateRule;
    impl crate::value::ToValue for CrossRegionCopyDeprecateRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Interval".to_string(),
                crate::value::ToValue::to_value(&self.interval),
            );
            properties.insert(
                "IntervalUnit".to_string(),
                crate::value::ToValue::to_value(&self.interval_unit),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-crossregioncopyretainrule.html
    pub struct CrossRegionCopyRetainRule_ {
        pub interval: i64,
        pub interval_unit: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dlm_LifecyclePolicy_CrossRegionCopyRetainRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::DLM::LifecyclePolicy.CrossRegionCopyRetainRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dlm_LifecyclePolicy_CrossRegionCopyRetainRule as CrossRegionCopyRetainRule;
    impl crate::value::ToValue for CrossRegionCopyRetainRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Interval".to_string(),
                crate::value::ToValue::to_value(&self.interval),
            );
            properties.insert(
                "IntervalUnit".to_string(),
                crate::value::ToValue::to_value(&self.interval_unit),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-crossregioncopyrule.html
    pub struct CrossRegionCopyRule_ {
        pub cmk_arn: Option<crate::value::ExpString>,
        pub copy_tags: Option<crate::value::ExpBool>,
        pub deprecate_rule: Option<Box<CrossRegionCopyDeprecateRule_>>,
        pub encrypted: crate::value::ExpBool,
        pub retain_rule: Option<Box<CrossRegionCopyRetainRule_>>,
        pub target: Option<crate::value::ExpString>,
        pub target_region: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dlm_LifecyclePolicy_CrossRegionCopyRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::DLM::LifecyclePolicy.CrossRegionCopyRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dlm_LifecyclePolicy_CrossRegionCopyRule as CrossRegionCopyRule;
    impl crate::value::ToValue for CrossRegionCopyRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cmk_arn {
                properties.insert("CmkArn".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.copy_tags {
                properties.insert(
                    "CopyTags".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.deprecate_rule {
                properties.insert(
                    "DeprecateRule".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Encrypted".to_string(),
                crate::value::ToValue::to_value(&self.encrypted),
            );
            if let Some(ref value) = self.retain_rule {
                properties.insert(
                    "RetainRule".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target {
                properties.insert("Target".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.target_region {
                properties.insert(
                    "TargetRegion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-crossregioncopytarget.html
    pub struct CrossRegionCopyTarget_ {
        pub target_region: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dlm_LifecyclePolicy_CrossRegionCopyTarget {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::DLM::LifecyclePolicy.CrossRegionCopyTarget"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dlm_LifecyclePolicy_CrossRegionCopyTarget as CrossRegionCopyTarget;
    impl crate::value::ToValue for CrossRegionCopyTarget_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.target_region {
                properties.insert(
                    "TargetRegion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-crossregioncopytargets.html
    pub struct CrossRegionCopyTargets_ {}
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dlm_LifecyclePolicy_CrossRegionCopyTargets {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::DLM::LifecyclePolicy.CrossRegionCopyTargets"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dlm_LifecyclePolicy_CrossRegionCopyTargets as CrossRegionCopyTargets;
    impl crate::value::ToValue for CrossRegionCopyTargets_ {
        fn to_value(&self) -> serde_json::Value {
            serde_json::Value::Object(serde_json::Map::new())
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-deprecaterule.html
    pub struct DeprecateRule_ {
        pub count: Option<i64>,
        pub interval: Option<i64>,
        pub interval_unit: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dlm_LifecyclePolicy_DeprecateRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::DLM::LifecyclePolicy.DeprecateRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dlm_LifecyclePolicy_DeprecateRule as DeprecateRule;
    impl crate::value::ToValue for DeprecateRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.count {
                properties.insert("Count".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.interval {
                properties.insert(
                    "Interval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.interval_unit {
                properties.insert(
                    "IntervalUnit".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-encryptionconfiguration.html
    pub struct EncryptionConfiguration_ {
        pub cmk_arn: Option<crate::value::ExpString>,
        pub encrypted: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dlm_LifecyclePolicy_EncryptionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::DLM::LifecyclePolicy.EncryptionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dlm_LifecyclePolicy_EncryptionConfiguration as EncryptionConfiguration;
    impl crate::value::ToValue for EncryptionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cmk_arn {
                properties.insert("CmkArn".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Encrypted".to_string(),
                crate::value::ToValue::to_value(&self.encrypted),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-eventparameters.html
    pub struct EventParameters_ {
        pub description_regex: Option<crate::value::ExpString>,
        pub event_type: crate::value::ExpString,
        pub snapshot_owner: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dlm_LifecyclePolicy_EventParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::DLM::LifecyclePolicy.EventParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dlm_LifecyclePolicy_EventParameters as EventParameters;
    impl crate::value::ToValue for EventParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.description_regex {
                properties.insert(
                    "DescriptionRegex".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "EventType".to_string(),
                crate::value::ToValue::to_value(&self.event_type),
            );
            properties.insert(
                "SnapshotOwner".to_string(),
                crate::value::ToValue::to_value(&self.snapshot_owner),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-eventsource.html
    pub struct EventSource_ {
        pub parameters: Option<Box<EventParameters_>>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dlm_LifecyclePolicy_EventSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::DLM::LifecyclePolicy.EventSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dlm_LifecyclePolicy_EventSource as EventSource;
    impl crate::value::ToValue for EventSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.parameters {
                properties.insert(
                    "Parameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-excludetags.html
    pub struct ExcludeTags_ {}
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dlm_LifecyclePolicy_ExcludeTags {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::DLM::LifecyclePolicy.ExcludeTags"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dlm_LifecyclePolicy_ExcludeTags as ExcludeTags;
    impl crate::value::ToValue for ExcludeTags_ {
        fn to_value(&self) -> serde_json::Value {
            serde_json::Value::Object(serde_json::Map::new())
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-excludevolumetypeslist.html
    pub struct ExcludeVolumeTypesList_ {}
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dlm_LifecyclePolicy_ExcludeVolumeTypesList {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::DLM::LifecyclePolicy.ExcludeVolumeTypesList"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dlm_LifecyclePolicy_ExcludeVolumeTypesList as ExcludeVolumeTypesList;
    impl crate::value::ToValue for ExcludeVolumeTypesList_ {
        fn to_value(&self) -> serde_json::Value {
            serde_json::Value::Object(serde_json::Map::new())
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-exclusions.html
    pub struct Exclusions_ {
        pub exclude_boot_volumes: Option<crate::value::ExpBool>,
        pub exclude_tags: Option<Box<ExcludeTags_>>,
        pub exclude_volume_types: Option<Box<ExcludeVolumeTypesList_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dlm_LifecyclePolicy_Exclusions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::DLM::LifecyclePolicy.Exclusions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dlm_LifecyclePolicy_Exclusions as Exclusions;
    impl crate::value::ToValue for Exclusions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.exclude_boot_volumes {
                properties.insert(
                    "ExcludeBootVolumes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exclude_tags {
                properties.insert(
                    "ExcludeTags".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exclude_volume_types {
                properties.insert(
                    "ExcludeVolumeTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-fastrestorerule.html
    pub struct FastRestoreRule_ {
        pub availability_zones: Option<Vec<crate::value::ExpString>>,
        pub count: Option<i64>,
        pub interval: Option<i64>,
        pub interval_unit: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dlm_LifecyclePolicy_FastRestoreRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::DLM::LifecyclePolicy.FastRestoreRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dlm_LifecyclePolicy_FastRestoreRule as FastRestoreRule;
    impl crate::value::ToValue for FastRestoreRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.availability_zones {
                properties.insert(
                    "AvailabilityZones".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.count {
                properties.insert("Count".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.interval {
                properties.insert(
                    "Interval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.interval_unit {
                properties.insert(
                    "IntervalUnit".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-parameters.html
    pub struct Parameters_ {
        pub exclude_boot_volume: Option<crate::value::ExpBool>,
        pub exclude_data_volume_tags: Option<Vec<crate::Tag_>>,
        pub no_reboot: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dlm_LifecyclePolicy_Parameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::DLM::LifecyclePolicy.Parameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dlm_LifecyclePolicy_Parameters as Parameters;
    impl crate::value::ToValue for Parameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.exclude_boot_volume {
                properties.insert(
                    "ExcludeBootVolume".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exclude_data_volume_tags {
                properties.insert(
                    "ExcludeDataVolumeTags".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.no_reboot {
                properties.insert(
                    "NoReboot".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-policydetails.html
    pub struct PolicyDetails_ {
        pub actions: Option<Vec<Action_>>,
        pub copy_tags: Option<crate::value::ExpBool>,
        pub create_interval: Option<i64>,
        pub cross_region_copy_targets: Option<Box<CrossRegionCopyTargets_>>,
        pub event_source: Option<Box<EventSource_>>,
        pub exclusions: Option<Box<Exclusions_>>,
        pub extend_deletion: Option<crate::value::ExpBool>,
        pub parameters: Option<Box<Parameters_>>,
        pub policy_language: Option<crate::value::ExpString>,
        pub policy_type: Option<crate::value::ExpString>,
        pub resource_locations: Option<Vec<crate::value::ExpString>>,
        pub resource_type: Option<crate::value::ExpString>,
        pub resource_types: Option<Vec<crate::value::ExpString>>,
        pub retain_interval: Option<i64>,
        pub schedules: Option<Vec<Schedule_>>,
        pub target_tags: Option<Vec<crate::Tag_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dlm_LifecyclePolicy_PolicyDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::DLM::LifecyclePolicy.PolicyDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dlm_LifecyclePolicy_PolicyDetails as PolicyDetails;
    impl crate::value::ToValue for PolicyDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.actions {
                properties.insert(
                    "Actions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.copy_tags {
                properties.insert(
                    "CopyTags".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.create_interval {
                properties.insert(
                    "CreateInterval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cross_region_copy_targets {
                properties.insert(
                    "CrossRegionCopyTargets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.event_source {
                properties.insert(
                    "EventSource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exclusions {
                properties.insert(
                    "Exclusions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.extend_deletion {
                properties.insert(
                    "ExtendDeletion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parameters {
                properties.insert(
                    "Parameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.policy_language {
                properties.insert(
                    "PolicyLanguage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.policy_type {
                properties.insert(
                    "PolicyType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_locations {
                properties.insert(
                    "ResourceLocations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_type {
                properties.insert(
                    "ResourceType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_types {
                properties.insert(
                    "ResourceTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.retain_interval {
                properties.insert(
                    "RetainInterval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.schedules {
                properties.insert(
                    "Schedules".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_tags {
                properties.insert(
                    "TargetTags".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-retainrule.html
    pub struct RetainRule_ {
        pub count: Option<i64>,
        pub interval: Option<i64>,
        pub interval_unit: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dlm_LifecyclePolicy_RetainRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::DLM::LifecyclePolicy.RetainRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dlm_LifecyclePolicy_RetainRule as RetainRule;
    impl crate::value::ToValue for RetainRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.count {
                properties.insert("Count".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.interval {
                properties.insert(
                    "Interval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.interval_unit {
                properties.insert(
                    "IntervalUnit".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-retentionarchivetier.html
    pub struct RetentionArchiveTier_ {
        pub count: Option<i64>,
        pub interval: Option<i64>,
        pub interval_unit: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dlm_LifecyclePolicy_RetentionArchiveTier {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::DLM::LifecyclePolicy.RetentionArchiveTier"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dlm_LifecyclePolicy_RetentionArchiveTier as RetentionArchiveTier;
    impl crate::value::ToValue for RetentionArchiveTier_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.count {
                properties.insert("Count".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.interval {
                properties.insert(
                    "Interval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.interval_unit {
                properties.insert(
                    "IntervalUnit".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-schedule.html
    pub struct Schedule_ {
        pub archive_rule: Option<Box<ArchiveRule_>>,
        pub copy_tags: Option<crate::value::ExpBool>,
        pub create_rule: Option<Box<CreateRule_>>,
        pub cross_region_copy_rules: Option<Vec<CrossRegionCopyRule_>>,
        pub deprecate_rule: Option<Box<DeprecateRule_>>,
        pub fast_restore_rule: Option<Box<FastRestoreRule_>>,
        pub name: Option<crate::value::ExpString>,
        pub retain_rule: Option<Box<RetainRule_>>,
        pub share_rules: Option<Vec<ShareRule_>>,
        pub tags_to_add: Option<Vec<crate::Tag_>>,
        pub variable_tags: Option<Vec<crate::Tag_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dlm_LifecyclePolicy_Schedule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::DLM::LifecyclePolicy.Schedule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dlm_LifecyclePolicy_Schedule as Schedule;
    impl crate::value::ToValue for Schedule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.archive_rule {
                properties.insert(
                    "ArchiveRule".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.copy_tags {
                properties.insert(
                    "CopyTags".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.create_rule {
                properties.insert(
                    "CreateRule".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cross_region_copy_rules {
                properties.insert(
                    "CrossRegionCopyRules".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.deprecate_rule {
                properties.insert(
                    "DeprecateRule".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.fast_restore_rule {
                properties.insert(
                    "FastRestoreRule".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.retain_rule {
                properties.insert(
                    "RetainRule".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.share_rules {
                properties.insert(
                    "ShareRules".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tags_to_add {
                properties.insert(
                    "TagsToAdd".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.variable_tags {
                properties.insert(
                    "VariableTags".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-script.html
    pub struct Script_ {
        pub execute_operation_on_script_failure: Option<crate::value::ExpBool>,
        pub execution_handler: Option<crate::value::ExpString>,
        pub execution_handler_service: Option<crate::value::ExpString>,
        pub execution_timeout: Option<i64>,
        pub maximum_retry_count: Option<i64>,
        pub stages: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dlm_LifecyclePolicy_Script {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::DLM::LifecyclePolicy.Script"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dlm_LifecyclePolicy_Script as Script;
    impl crate::value::ToValue for Script_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.execute_operation_on_script_failure {
                properties.insert(
                    "ExecuteOperationOnScriptFailure".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.execution_handler {
                properties.insert(
                    "ExecutionHandler".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.execution_handler_service {
                properties.insert(
                    "ExecutionHandlerService".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.execution_timeout {
                properties.insert(
                    "ExecutionTimeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.maximum_retry_count {
                properties.insert(
                    "MaximumRetryCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stages {
                properties.insert("Stages".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-sharerule.html
    pub struct ShareRule_ {
        pub target_accounts: Option<Vec<crate::value::ExpString>>,
        pub unshare_interval: Option<i64>,
        pub unshare_interval_unit: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dlm_LifecyclePolicy_ShareRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::DLM::LifecyclePolicy.ShareRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dlm_LifecyclePolicy_ShareRule as ShareRule;
    impl crate::value::ToValue for ShareRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.target_accounts {
                properties.insert(
                    "TargetAccounts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.unshare_interval {
                properties.insert(
                    "UnshareInterval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.unshare_interval_unit {
                properties.insert(
                    "UnshareIntervalUnit".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-volumetypevalues.html
    pub struct VolumeTypeValues_ {}
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dlm_LifecyclePolicy_VolumeTypeValues {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::DLM::LifecyclePolicy.VolumeTypeValues"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dlm_LifecyclePolicy_VolumeTypeValues as VolumeTypeValues;
    impl crate::value::ToValue for VolumeTypeValues_ {
        fn to_value(&self) -> serde_json::Value {
            serde_json::Value::Object(serde_json::Map::new())
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dlm-lifecyclepolicy.html
pub struct LifecyclePolicy_ {
    pub copy_tags: Option<crate::value::ExpBool>,
    pub create_interval: Option<i64>,
    pub cross_region_copy_targets: Option<super::dlm::lifecyclepolicy::CrossRegionCopyTargets_>,
    pub default_policy: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub exclusions: Option<super::dlm::lifecyclepolicy::Exclusions_>,
    pub execution_role_arn: Option<crate::value::ExpString>,
    pub extend_deletion: Option<crate::value::ExpBool>,
    pub policy_details: Option<super::dlm::lifecyclepolicy::PolicyDetails_>,
    pub retain_interval: Option<i64>,
    pub state: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_dlm_LifecyclePolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::DLM::LifecyclePolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_dlm_LifecyclePolicy as LifecyclePolicy;
impl crate::template::ToResource for LifecyclePolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DLM"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LifecyclePolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.copy_tags {
            properties.insert(
                "CopyTags".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.create_interval {
            properties.insert(
                "CreateInterval".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cross_region_copy_targets {
            properties.insert(
                "CrossRegionCopyTargets".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.default_policy {
            properties.insert(
                "DefaultPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.exclusions {
            properties.insert(
                "Exclusions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.execution_role_arn {
            properties.insert(
                "ExecutionRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.extend_deletion {
            properties.insert(
                "ExtendDeletion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.policy_details {
            properties.insert(
                "PolicyDetails".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.retain_interval {
            properties.insert(
                "RetainInterval".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.state {
            properties.insert("State".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
