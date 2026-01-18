pub mod calculatedattributedefinition {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-calculatedattributedefinition-attributedetails.html
    pub struct AttributeDetails_ {
        pub attributes: Vec<AttributeItem_>,
        pub expression: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_CalculatedAttributeDefinition_AttributeDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::CalculatedAttributeDefinition.AttributeDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_CalculatedAttributeDefinition_AttributeDetails as AttributeDetails;
    impl crate::value::ToValue for AttributeDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Attributes".to_string(),
                crate::value::ToValue::to_value(&self.attributes),
            );
            properties.insert(
                "Expression".to_string(),
                crate::value::ToValue::to_value(&self.expression),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-calculatedattributedefinition-attributeitem.html
    pub struct AttributeItem_ {
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_CalculatedAttributeDefinition_AttributeItem {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::CalculatedAttributeDefinition.AttributeItem"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_CalculatedAttributeDefinition_AttributeItem as AttributeItem;
    impl crate::value::ToValue for AttributeItem_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-calculatedattributedefinition-conditions.html
    pub struct Conditions_ {
        pub object_count: Option<i64>,
        pub range: Option<Box<Range_>>,
        pub threshold: Option<Box<Threshold_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_CalculatedAttributeDefinition_Conditions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::CalculatedAttributeDefinition.Conditions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_CalculatedAttributeDefinition_Conditions as Conditions;
    impl crate::value::ToValue for Conditions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.object_count {
                properties.insert(
                    "ObjectCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.range {
                properties.insert("Range".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.threshold {
                properties.insert(
                    "Threshold".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-calculatedattributedefinition-range.html
    pub struct Range_ {
        pub timestamp_format: Option<crate::value::ExpString>,
        pub timestamp_source: Option<crate::value::ExpString>,
        pub unit: crate::value::ExpString,
        pub value: Option<i64>,
        pub value_range: Option<Box<ValueRange_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_CalculatedAttributeDefinition_Range {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::CalculatedAttributeDefinition.Range"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_CalculatedAttributeDefinition_Range as Range;
    impl crate::value::ToValue for Range_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.timestamp_format {
                properties.insert(
                    "TimestampFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timestamp_source {
                properties.insert(
                    "TimestampSource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Unit".to_string(),
                crate::value::ToValue::to_value(&self.unit),
            );
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value_range {
                properties.insert(
                    "ValueRange".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-calculatedattributedefinition-readiness.html
    pub struct Readiness_ {
        pub message: Option<crate::value::ExpString>,
        pub progress_percentage: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_CalculatedAttributeDefinition_Readiness {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::CalculatedAttributeDefinition.Readiness"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_CalculatedAttributeDefinition_Readiness as Readiness;
    impl crate::value::ToValue for Readiness_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.message {
                properties.insert(
                    "Message".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.progress_percentage {
                properties.insert(
                    "ProgressPercentage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-calculatedattributedefinition-threshold.html
    pub struct Threshold_ {
        pub operator: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_CalculatedAttributeDefinition_Threshold {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::CalculatedAttributeDefinition.Threshold"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_CalculatedAttributeDefinition_Threshold as Threshold;
    impl crate::value::ToValue for Threshold_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Operator".to_string(),
                crate::value::ToValue::to_value(&self.operator),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-calculatedattributedefinition-valuerange.html
    pub struct ValueRange_ {
        pub end: i64,
        pub start: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_CalculatedAttributeDefinition_ValueRange {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::CalculatedAttributeDefinition.ValueRange"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_CalculatedAttributeDefinition_ValueRange as ValueRange;
    impl crate::value::ToValue for ValueRange_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "End".to_string(),
                crate::value::ToValue::to_value(&self.end),
            );
            properties.insert(
                "Start".to_string(),
                crate::value::ToValue::to_value(&self.start),
            );
            properties.into()
        }
    }
}
pub mod domain {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-attributetypesselector.html
    pub struct AttributeTypesSelector_ {
        pub address: Option<Vec<crate::value::ExpString>>,
        pub attribute_matching_model: crate::value::ExpString,
        pub email_address: Option<Vec<crate::value::ExpString>>,
        pub phone_number: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_Domain_AttributeTypesSelector {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::Domain.AttributeTypesSelector"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_Domain_AttributeTypesSelector as AttributeTypesSelector;
    impl crate::value::ToValue for AttributeTypesSelector_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.address {
                properties.insert(
                    "Address".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "AttributeMatchingModel".to_string(),
                crate::value::ToValue::to_value(&self.attribute_matching_model),
            );
            if let Some(ref value) = self.email_address {
                properties.insert(
                    "EmailAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.phone_number {
                properties.insert(
                    "PhoneNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-automerging.html
    pub struct AutoMerging_ {
        pub conflict_resolution: Option<Box<ConflictResolution_>>,
        pub consolidation: Option<Box<Consolidation_>>,
        pub enabled: crate::value::ExpBool,
        pub min_allowed_confidence_score_for_merging: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_Domain_AutoMerging {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::Domain.AutoMerging"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_Domain_AutoMerging as AutoMerging;
    impl crate::value::ToValue for AutoMerging_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.conflict_resolution {
                properties.insert(
                    "ConflictResolution".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.consolidation {
                properties.insert(
                    "Consolidation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            if let Some(ref value) = self.min_allowed_confidence_score_for_merging {
                properties.insert(
                    "MinAllowedConfidenceScoreForMerging".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-conflictresolution.html
    pub struct ConflictResolution_ {
        pub conflict_resolving_model: crate::value::ExpString,
        pub source_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_Domain_ConflictResolution {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::Domain.ConflictResolution"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_Domain_ConflictResolution as ConflictResolution;
    impl crate::value::ToValue for ConflictResolution_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ConflictResolvingModel".to_string(),
                crate::value::ToValue::to_value(&self.conflict_resolving_model),
            );
            if let Some(ref value) = self.source_name {
                properties.insert(
                    "SourceName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-consolidation.html
    pub struct Consolidation_ {
        pub matching_attributes_list: serde_json::Value,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_Domain_Consolidation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::Domain.Consolidation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_Domain_Consolidation as Consolidation;
    impl crate::value::ToValue for Consolidation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MatchingAttributesList".to_string(),
                crate::value::ToValue::to_value(&self.matching_attributes_list),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-domainstats.html
    pub struct DomainStats_ {
        pub metering_profile_count: Option<f64>,
        pub object_count: Option<f64>,
        pub profile_count: Option<f64>,
        pub total_size: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_Domain_DomainStats {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::Domain.DomainStats"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_Domain_DomainStats as DomainStats;
    impl crate::value::ToValue for DomainStats_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.metering_profile_count {
                properties.insert(
                    "MeteringProfileCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.object_count {
                properties.insert(
                    "ObjectCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.profile_count {
                properties.insert(
                    "ProfileCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.total_size {
                properties.insert(
                    "TotalSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-exportingconfig.html
    pub struct ExportingConfig_ {
        pub s3_exporting: Option<Box<S3ExportingConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_Domain_ExportingConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::Domain.ExportingConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_Domain_ExportingConfig as ExportingConfig;
    impl crate::value::ToValue for ExportingConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.s3_exporting {
                properties.insert(
                    "S3Exporting".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-jobschedule.html
    pub struct JobSchedule_ {
        pub day_of_the_week: crate::value::ExpString,
        pub time: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_Domain_JobSchedule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::Domain.JobSchedule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_Domain_JobSchedule as JobSchedule;
    impl crate::value::ToValue for JobSchedule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DayOfTheWeek".to_string(),
                crate::value::ToValue::to_value(&self.day_of_the_week),
            );
            properties.insert(
                "Time".to_string(),
                crate::value::ToValue::to_value(&self.time),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-matching.html
    pub struct Matching_ {
        pub auto_merging: Option<Box<AutoMerging_>>,
        pub enabled: crate::value::ExpBool,
        pub exporting_config: Option<Box<ExportingConfig_>>,
        pub job_schedule: Option<Box<JobSchedule_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_Domain_Matching {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::Domain.Matching"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_Domain_Matching as Matching;
    impl crate::value::ToValue for Matching_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auto_merging {
                properties.insert(
                    "AutoMerging".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            if let Some(ref value) = self.exporting_config {
                properties.insert(
                    "ExportingConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.job_schedule {
                properties.insert(
                    "JobSchedule".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-matchingrule.html
    pub struct MatchingRule_ {
        pub rule: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_Domain_MatchingRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::Domain.MatchingRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_Domain_MatchingRule as MatchingRule;
    impl crate::value::ToValue for MatchingRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Rule".to_string(),
                crate::value::ToValue::to_value(&self.rule),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-rulebasedmatching.html
    pub struct RuleBasedMatching_ {
        pub attribute_types_selector: Option<Box<AttributeTypesSelector_>>,
        pub conflict_resolution: Option<Box<ConflictResolution_>>,
        pub enabled: crate::value::ExpBool,
        pub exporting_config: Option<Box<ExportingConfig_>>,
        pub matching_rules: Option<Vec<MatchingRule_>>,
        pub max_allowed_rule_level_for_matching: Option<i64>,
        pub max_allowed_rule_level_for_merging: Option<i64>,
        pub status: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_Domain_RuleBasedMatching {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::Domain.RuleBasedMatching"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_Domain_RuleBasedMatching as RuleBasedMatching;
    impl crate::value::ToValue for RuleBasedMatching_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attribute_types_selector {
                properties.insert(
                    "AttributeTypesSelector".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.conflict_resolution {
                properties.insert(
                    "ConflictResolution".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            if let Some(ref value) = self.exporting_config {
                properties.insert(
                    "ExportingConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.matching_rules {
                properties.insert(
                    "MatchingRules".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_allowed_rule_level_for_matching {
                properties.insert(
                    "MaxAllowedRuleLevelForMatching".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_allowed_rule_level_for_merging {
                properties.insert(
                    "MaxAllowedRuleLevelForMerging".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-s3exportingconfig.html
    pub struct S3ExportingConfig_ {
        pub s3_bucket_name: crate::value::ExpString,
        pub s3_key_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_Domain_S3ExportingConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::Domain.S3ExportingConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_Domain_S3ExportingConfig as S3ExportingConfig;
    impl crate::value::ToValue for S3ExportingConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "S3BucketName".to_string(),
                crate::value::ToValue::to_value(&self.s3_bucket_name),
            );
            if let Some(ref value) = self.s3_key_name {
                properties.insert(
                    "S3KeyName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod eventstream {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-eventstream-destinationdetails.html
    pub struct DestinationDetails_ {
        pub status: crate::value::ExpString,
        pub uri: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_EventStream_DestinationDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::EventStream.DestinationDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_EventStream_DestinationDetails as DestinationDetails;
    impl crate::value::ToValue for DestinationDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Status".to_string(),
                crate::value::ToValue::to_value(&self.status),
            );
            properties.insert(
                "Uri".to_string(),
                crate::value::ToValue::to_value(&self.uri),
            );
            properties.into()
        }
    }
}
pub mod eventtrigger {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-eventtrigger-eventtriggercondition.html
    pub struct EventTriggerCondition_ {
        pub event_trigger_dimensions: Vec<EventTriggerDimension_>,
        pub logical_operator: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_EventTrigger_EventTriggerCondition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::EventTrigger.EventTriggerCondition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_EventTrigger_EventTriggerCondition as EventTriggerCondition;
    impl crate::value::ToValue for EventTriggerCondition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EventTriggerDimensions".to_string(),
                crate::value::ToValue::to_value(&self.event_trigger_dimensions),
            );
            properties.insert(
                "LogicalOperator".to_string(),
                crate::value::ToValue::to_value(&self.logical_operator),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-eventtrigger-eventtriggerdimension.html
    pub struct EventTriggerDimension_ {
        pub object_attributes: Vec<ObjectAttribute_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_EventTrigger_EventTriggerDimension {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::EventTrigger.EventTriggerDimension"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_EventTrigger_EventTriggerDimension as EventTriggerDimension;
    impl crate::value::ToValue for EventTriggerDimension_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ObjectAttributes".to_string(),
                crate::value::ToValue::to_value(&self.object_attributes),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-eventtrigger-eventtriggerlimits.html
    pub struct EventTriggerLimits_ {
        pub event_expiration: Option<i64>,
        pub periods: Option<Vec<Period_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_EventTrigger_EventTriggerLimits {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::EventTrigger.EventTriggerLimits"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_EventTrigger_EventTriggerLimits as EventTriggerLimits;
    impl crate::value::ToValue for EventTriggerLimits_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.event_expiration {
                properties.insert(
                    "EventExpiration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.periods {
                properties.insert(
                    "Periods".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-eventtrigger-objectattribute.html
    pub struct ObjectAttribute_ {
        pub comparison_operator: crate::value::ExpString,
        pub field_name: Option<crate::value::ExpString>,
        pub source: Option<crate::value::ExpString>,
        pub values: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_EventTrigger_ObjectAttribute {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::EventTrigger.ObjectAttribute"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_EventTrigger_ObjectAttribute as ObjectAttribute;
    impl crate::value::ToValue for ObjectAttribute_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ComparisonOperator".to_string(),
                crate::value::ToValue::to_value(&self.comparison_operator),
            );
            if let Some(ref value) = self.field_name {
                properties.insert(
                    "FieldName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source {
                properties.insert("Source".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Values".to_string(),
                crate::value::ToValue::to_value(&self.values),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-eventtrigger-period.html
    pub struct Period_ {
        pub max_invocations_per_profile: Option<i64>,
        pub unit: crate::value::ExpString,
        pub unlimited: Option<crate::value::ExpBool>,
        pub value: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_EventTrigger_Period {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::EventTrigger.Period"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_EventTrigger_Period as Period;
    impl crate::value::ToValue for Period_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max_invocations_per_profile {
                properties.insert(
                    "MaxInvocationsPerProfile".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Unit".to_string(),
                crate::value::ToValue::to_value(&self.unit),
            );
            if let Some(ref value) = self.unlimited {
                properties.insert(
                    "Unlimited".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
}
pub mod integration {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-connectoroperator.html
    pub struct ConnectorOperator_ {
        pub marketo: Option<crate::value::ExpString>,
        pub s3: Option<crate::value::ExpString>,
        pub salesforce: Option<crate::value::ExpString>,
        pub service_now: Option<crate::value::ExpString>,
        pub zendesk: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_Integration_ConnectorOperator {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::Integration.ConnectorOperator"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_Integration_ConnectorOperator as ConnectorOperator;
    impl crate::value::ToValue for ConnectorOperator_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.marketo {
                properties.insert(
                    "Marketo".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3 {
                properties.insert("S3".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.salesforce {
                properties.insert(
                    "Salesforce".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.service_now {
                properties.insert(
                    "ServiceNow".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.zendesk {
                properties.insert(
                    "Zendesk".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-flowdefinition.html
    pub struct FlowDefinition_ {
        pub description: Option<crate::value::ExpString>,
        pub flow_name: crate::value::ExpString,
        pub kms_arn: crate::value::ExpString,
        pub source_flow_config: Box<SourceFlowConfig_>,
        pub tasks: Vec<Task_>,
        pub trigger_config: Box<TriggerConfig_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_Integration_FlowDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::Integration.FlowDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_Integration_FlowDefinition as FlowDefinition;
    impl crate::value::ToValue for FlowDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "FlowName".to_string(),
                crate::value::ToValue::to_value(&self.flow_name),
            );
            properties.insert(
                "KmsArn".to_string(),
                crate::value::ToValue::to_value(&self.kms_arn),
            );
            properties.insert(
                "SourceFlowConfig".to_string(),
                crate::value::ToValue::to_value(&self.source_flow_config),
            );
            properties.insert(
                "Tasks".to_string(),
                crate::value::ToValue::to_value(&self.tasks),
            );
            properties.insert(
                "TriggerConfig".to_string(),
                crate::value::ToValue::to_value(&self.trigger_config),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-incrementalpullconfig.html
    pub struct IncrementalPullConfig_ {
        pub datetime_type_field_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_Integration_IncrementalPullConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::Integration.IncrementalPullConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_Integration_IncrementalPullConfig as IncrementalPullConfig;
    impl crate::value::ToValue for IncrementalPullConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.datetime_type_field_name {
                properties.insert(
                    "DatetimeTypeFieldName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-marketosourceproperties.html
    pub struct MarketoSourceProperties_ {
        pub object: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_Integration_MarketoSourceProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::Integration.MarketoSourceProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_Integration_MarketoSourceProperties as MarketoSourceProperties;
    impl crate::value::ToValue for MarketoSourceProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Object".to_string(),
                crate::value::ToValue::to_value(&self.object),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-objecttypemapping.html
    pub struct ObjectTypeMapping_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_Integration_ObjectTypeMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::Integration.ObjectTypeMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_Integration_ObjectTypeMapping as ObjectTypeMapping;
    impl crate::value::ToValue for ObjectTypeMapping_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-s3sourceproperties.html
    pub struct S3SourceProperties_ {
        pub bucket_name: crate::value::ExpString,
        pub bucket_prefix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_Integration_S3SourceProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::Integration.S3SourceProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_Integration_S3SourceProperties as S3SourceProperties;
    impl crate::value::ToValue for S3SourceProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BucketName".to_string(),
                crate::value::ToValue::to_value(&self.bucket_name),
            );
            if let Some(ref value) = self.bucket_prefix {
                properties.insert(
                    "BucketPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-salesforcesourceproperties.html
    pub struct SalesforceSourceProperties_ {
        pub enable_dynamic_field_update: Option<crate::value::ExpBool>,
        pub include_deleted_records: Option<crate::value::ExpBool>,
        pub object: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_Integration_SalesforceSourceProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::Integration.SalesforceSourceProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_Integration_SalesforceSourceProperties as SalesforceSourceProperties;
    impl crate::value::ToValue for SalesforceSourceProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enable_dynamic_field_update {
                properties.insert(
                    "EnableDynamicFieldUpdate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_deleted_records {
                properties.insert(
                    "IncludeDeletedRecords".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Object".to_string(),
                crate::value::ToValue::to_value(&self.object),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-scheduledtriggerproperties.html
    pub struct ScheduledTriggerProperties_ {
        pub data_pull_mode: Option<crate::value::ExpString>,
        pub first_execution_from: Option<f64>,
        pub schedule_end_time: Option<f64>,
        pub schedule_expression: crate::value::ExpString,
        pub schedule_offset: Option<i64>,
        pub schedule_start_time: Option<f64>,
        pub timezone: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_Integration_ScheduledTriggerProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::Integration.ScheduledTriggerProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_Integration_ScheduledTriggerProperties as ScheduledTriggerProperties;
    impl crate::value::ToValue for ScheduledTriggerProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.data_pull_mode {
                properties.insert(
                    "DataPullMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.first_execution_from {
                properties.insert(
                    "FirstExecutionFrom".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.schedule_end_time {
                properties.insert(
                    "ScheduleEndTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ScheduleExpression".to_string(),
                crate::value::ToValue::to_value(&self.schedule_expression),
            );
            if let Some(ref value) = self.schedule_offset {
                properties.insert(
                    "ScheduleOffset".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.schedule_start_time {
                properties.insert(
                    "ScheduleStartTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timezone {
                properties.insert(
                    "Timezone".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-servicenowsourceproperties.html
    pub struct ServiceNowSourceProperties_ {
        pub object: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_Integration_ServiceNowSourceProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::Integration.ServiceNowSourceProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_Integration_ServiceNowSourceProperties as ServiceNowSourceProperties;
    impl crate::value::ToValue for ServiceNowSourceProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Object".to_string(),
                crate::value::ToValue::to_value(&self.object),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-sourceconnectorproperties.html
    pub struct SourceConnectorProperties_ {
        pub marketo: Option<Box<MarketoSourceProperties_>>,
        pub s3: Option<Box<S3SourceProperties_>>,
        pub salesforce: Option<Box<SalesforceSourceProperties_>>,
        pub service_now: Option<Box<ServiceNowSourceProperties_>>,
        pub zendesk: Option<Box<ZendeskSourceProperties_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_Integration_SourceConnectorProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::Integration.SourceConnectorProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_Integration_SourceConnectorProperties as SourceConnectorProperties;
    impl crate::value::ToValue for SourceConnectorProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.marketo {
                properties.insert(
                    "Marketo".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3 {
                properties.insert("S3".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.salesforce {
                properties.insert(
                    "Salesforce".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.service_now {
                properties.insert(
                    "ServiceNow".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.zendesk {
                properties.insert(
                    "Zendesk".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-sourceflowconfig.html
    pub struct SourceFlowConfig_ {
        pub connector_profile_name: Option<crate::value::ExpString>,
        pub connector_type: crate::value::ExpString,
        pub incremental_pull_config: Option<Box<IncrementalPullConfig_>>,
        pub source_connector_properties: Box<SourceConnectorProperties_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_Integration_SourceFlowConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::Integration.SourceFlowConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_Integration_SourceFlowConfig as SourceFlowConfig;
    impl crate::value::ToValue for SourceFlowConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.connector_profile_name {
                properties.insert(
                    "ConnectorProfileName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ConnectorType".to_string(),
                crate::value::ToValue::to_value(&self.connector_type),
            );
            if let Some(ref value) = self.incremental_pull_config {
                properties.insert(
                    "IncrementalPullConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SourceConnectorProperties".to_string(),
                crate::value::ToValue::to_value(&self.source_connector_properties),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-task.html
    pub struct Task_ {
        pub connector_operator: Option<Box<ConnectorOperator_>>,
        pub destination_field: Option<crate::value::ExpString>,
        pub source_fields: Vec<crate::value::ExpString>,
        pub task_properties: Option<Vec<TaskPropertiesMap_>>,
        pub task_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_Integration_Task {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::Integration.Task"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_Integration_Task as Task;
    impl crate::value::ToValue for Task_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.connector_operator {
                properties.insert(
                    "ConnectorOperator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.destination_field {
                properties.insert(
                    "DestinationField".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SourceFields".to_string(),
                crate::value::ToValue::to_value(&self.source_fields),
            );
            if let Some(ref value) = self.task_properties {
                properties.insert(
                    "TaskProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TaskType".to_string(),
                crate::value::ToValue::to_value(&self.task_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-taskpropertiesmap.html
    pub struct TaskPropertiesMap_ {
        pub operator_property_key: crate::value::ExpString,
        pub property: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_Integration_TaskPropertiesMap {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::Integration.TaskPropertiesMap"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_Integration_TaskPropertiesMap as TaskPropertiesMap;
    impl crate::value::ToValue for TaskPropertiesMap_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "OperatorPropertyKey".to_string(),
                crate::value::ToValue::to_value(&self.operator_property_key),
            );
            properties.insert(
                "Property".to_string(),
                crate::value::ToValue::to_value(&self.property),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-triggerconfig.html
    pub struct TriggerConfig_ {
        pub trigger_properties: Option<Box<TriggerProperties_>>,
        pub trigger_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_Integration_TriggerConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::Integration.TriggerConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_Integration_TriggerConfig as TriggerConfig;
    impl crate::value::ToValue for TriggerConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.trigger_properties {
                properties.insert(
                    "TriggerProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TriggerType".to_string(),
                crate::value::ToValue::to_value(&self.trigger_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-triggerproperties.html
    pub struct TriggerProperties_ {
        pub scheduled: Option<Box<ScheduledTriggerProperties_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_Integration_TriggerProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::Integration.TriggerProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_Integration_TriggerProperties as TriggerProperties;
    impl crate::value::ToValue for TriggerProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.scheduled {
                properties.insert(
                    "Scheduled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-zendesksourceproperties.html
    pub struct ZendeskSourceProperties_ {
        pub object: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_Integration_ZendeskSourceProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::Integration.ZendeskSourceProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_Integration_ZendeskSourceProperties as ZendeskSourceProperties;
    impl crate::value::ToValue for ZendeskSourceProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Object".to_string(),
                crate::value::ToValue::to_value(&self.object),
            );
            properties.into()
        }
    }
}
pub mod objecttype {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-objecttype-fieldmap.html
    pub struct FieldMap_ {
        pub name: Option<crate::value::ExpString>,
        pub object_type_field: Option<Box<ObjectTypeField_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_ObjectType_FieldMap {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::ObjectType.FieldMap"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_ObjectType_FieldMap as FieldMap;
    impl crate::value::ToValue for FieldMap_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.object_type_field {
                properties.insert(
                    "ObjectTypeField".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-objecttype-keymap.html
    pub struct KeyMap_ {
        pub name: Option<crate::value::ExpString>,
        pub object_type_key_list: Option<Vec<ObjectTypeKey_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_ObjectType_KeyMap {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::ObjectType.KeyMap"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_ObjectType_KeyMap as KeyMap;
    impl crate::value::ToValue for KeyMap_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.object_type_key_list {
                properties.insert(
                    "ObjectTypeKeyList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-objecttype-objecttypefield.html
    pub struct ObjectTypeField_ {
        pub content_type: Option<crate::value::ExpString>,
        pub source: Option<crate::value::ExpString>,
        pub target: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_ObjectType_ObjectTypeField {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::ObjectType.ObjectTypeField"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_ObjectType_ObjectTypeField as ObjectTypeField;
    impl crate::value::ToValue for ObjectTypeField_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.content_type {
                properties.insert(
                    "ContentType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source {
                properties.insert("Source".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.target {
                properties.insert("Target".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-objecttype-objecttypekey.html
    pub struct ObjectTypeKey_ {
        pub field_names: Option<Vec<crate::value::ExpString>>,
        pub standard_identifiers: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_ObjectType_ObjectTypeKey {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::ObjectType.ObjectTypeKey"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_ObjectType_ObjectTypeKey as ObjectTypeKey;
    impl crate::value::ToValue for ObjectTypeKey_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.field_names {
                properties.insert(
                    "FieldNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.standard_identifiers {
                properties.insert(
                    "StandardIdentifiers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod segmentdefinition {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-segmentdefinition-addressdimension.html
    pub struct AddressDimension_ {
        pub city: Option<Box<ProfileDimension_>>,
        pub country: Option<Box<ProfileDimension_>>,
        pub county: Option<Box<ProfileDimension_>>,
        pub postal_code: Option<Box<ProfileDimension_>>,
        pub province: Option<Box<ProfileDimension_>>,
        pub state: Option<Box<ProfileDimension_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_SegmentDefinition_AddressDimension {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::SegmentDefinition.AddressDimension"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_SegmentDefinition_AddressDimension as AddressDimension;
    impl crate::value::ToValue for AddressDimension_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.city {
                properties.insert("City".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.country {
                properties.insert(
                    "Country".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.county {
                properties.insert("County".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.postal_code {
                properties.insert(
                    "PostalCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.province {
                properties.insert(
                    "Province".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.state {
                properties.insert("State".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-segmentdefinition-attributedimension.html
    pub struct AttributeDimension_ {
        pub dimension_type: crate::value::ExpString,
        pub values: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_SegmentDefinition_AttributeDimension {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::SegmentDefinition.AttributeDimension"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_SegmentDefinition_AttributeDimension as AttributeDimension;
    impl crate::value::ToValue for AttributeDimension_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DimensionType".to_string(),
                crate::value::ToValue::to_value(&self.dimension_type),
            );
            properties.insert(
                "Values".to_string(),
                crate::value::ToValue::to_value(&self.values),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-segmentdefinition-calculatedattributedimension.html
    pub struct CalculatedAttributeDimension_ {
        pub condition_overrides: Option<Box<ConditionOverrides_>>,
        pub dimension_type: crate::value::ExpString,
        pub values: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_SegmentDefinition_CalculatedAttributeDimension {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::SegmentDefinition.CalculatedAttributeDimension"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_SegmentDefinition_CalculatedAttributeDimension as CalculatedAttributeDimension;
    impl crate::value::ToValue for CalculatedAttributeDimension_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.condition_overrides {
                properties.insert(
                    "ConditionOverrides".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DimensionType".to_string(),
                crate::value::ToValue::to_value(&self.dimension_type),
            );
            properties.insert(
                "Values".to_string(),
                crate::value::ToValue::to_value(&self.values),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-segmentdefinition-conditionoverrides.html
    pub struct ConditionOverrides_ {
        pub range: Option<Box<RangeOverride_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_SegmentDefinition_ConditionOverrides {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::SegmentDefinition.ConditionOverrides"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_SegmentDefinition_ConditionOverrides as ConditionOverrides;
    impl crate::value::ToValue for ConditionOverrides_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.range {
                properties.insert("Range".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-segmentdefinition-datedimension.html
    pub struct DateDimension_ {
        pub dimension_type: crate::value::ExpString,
        pub values: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_SegmentDefinition_DateDimension {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::SegmentDefinition.DateDimension"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_SegmentDefinition_DateDimension as DateDimension;
    impl crate::value::ToValue for DateDimension_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DimensionType".to_string(),
                crate::value::ToValue::to_value(&self.dimension_type),
            );
            properties.insert(
                "Values".to_string(),
                crate::value::ToValue::to_value(&self.values),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-segmentdefinition-dimension.html
    pub struct Dimension_ {
        pub calculated_attributes:
            Option<std::collections::BTreeMap<String, CalculatedAttributeDimension_>>,
        pub profile_attributes: Option<Box<ProfileAttributes_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_SegmentDefinition_Dimension {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::SegmentDefinition.Dimension"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_SegmentDefinition_Dimension as Dimension;
    impl crate::value::ToValue for Dimension_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.calculated_attributes {
                properties.insert(
                    "CalculatedAttributes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.profile_attributes {
                properties.insert(
                    "ProfileAttributes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-segmentdefinition-extralengthvalueprofiledimension.html
    pub struct ExtraLengthValueProfileDimension_ {
        pub dimension_type: crate::value::ExpString,
        pub values: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_SegmentDefinition_ExtraLengthValueProfileDimension {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::SegmentDefinition.ExtraLengthValueProfileDimension"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_SegmentDefinition_ExtraLengthValueProfileDimension as ExtraLengthValueProfileDimension;
    impl crate::value::ToValue for ExtraLengthValueProfileDimension_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DimensionType".to_string(),
                crate::value::ToValue::to_value(&self.dimension_type),
            );
            properties.insert(
                "Values".to_string(),
                crate::value::ToValue::to_value(&self.values),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-segmentdefinition-group.html
    pub struct Group_ {
        pub dimensions: Option<Vec<Dimension_>>,
        pub source_segments: Option<Vec<SourceSegment_>>,
        pub source_type: Option<crate::value::ExpString>,
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_SegmentDefinition_Group {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::SegmentDefinition.Group"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_SegmentDefinition_Group as Group;
    impl crate::value::ToValue for Group_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dimensions {
                properties.insert(
                    "Dimensions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_segments {
                properties.insert(
                    "SourceSegments".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_type {
                properties.insert(
                    "SourceType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-segmentdefinition-profileattributes.html
    pub struct ProfileAttributes_ {
        pub account_number: Option<Box<ProfileDimension_>>,
        pub additional_information: Option<Box<ExtraLengthValueProfileDimension_>>,
        pub address: Option<Box<AddressDimension_>>,
        pub attributes: Option<std::collections::BTreeMap<String, AttributeDimension_>>,
        pub billing_address: Option<Box<AddressDimension_>>,
        pub birth_date: Option<Box<DateDimension_>>,
        pub business_email_address: Option<Box<ProfileDimension_>>,
        pub business_name: Option<Box<ProfileDimension_>>,
        pub business_phone_number: Option<Box<ProfileDimension_>>,
        pub email_address: Option<Box<ProfileDimension_>>,
        pub first_name: Option<Box<ProfileDimension_>>,
        pub gender_string: Option<Box<ProfileDimension_>>,
        pub home_phone_number: Option<Box<ProfileDimension_>>,
        pub last_name: Option<Box<ProfileDimension_>>,
        pub mailing_address: Option<Box<AddressDimension_>>,
        pub middle_name: Option<Box<ProfileDimension_>>,
        pub mobile_phone_number: Option<Box<ProfileDimension_>>,
        pub party_type_string: Option<Box<ProfileDimension_>>,
        pub personal_email_address: Option<Box<ProfileDimension_>>,
        pub phone_number: Option<Box<ProfileDimension_>>,
        pub profile_type: Option<Box<ProfileTypeDimension_>>,
        pub shipping_address: Option<Box<AddressDimension_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_SegmentDefinition_ProfileAttributes {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::SegmentDefinition.ProfileAttributes"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_SegmentDefinition_ProfileAttributes as ProfileAttributes;
    impl crate::value::ToValue for ProfileAttributes_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.account_number {
                properties.insert(
                    "AccountNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.additional_information {
                properties.insert(
                    "AdditionalInformation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.address {
                properties.insert(
                    "Address".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.attributes {
                properties.insert(
                    "Attributes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.billing_address {
                properties.insert(
                    "BillingAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.birth_date {
                properties.insert(
                    "BirthDate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.business_email_address {
                properties.insert(
                    "BusinessEmailAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.business_name {
                properties.insert(
                    "BusinessName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.business_phone_number {
                properties.insert(
                    "BusinessPhoneNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.email_address {
                properties.insert(
                    "EmailAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.first_name {
                properties.insert(
                    "FirstName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.gender_string {
                properties.insert(
                    "GenderString".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.home_phone_number {
                properties.insert(
                    "HomePhoneNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.last_name {
                properties.insert(
                    "LastName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mailing_address {
                properties.insert(
                    "MailingAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.middle_name {
                properties.insert(
                    "MiddleName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mobile_phone_number {
                properties.insert(
                    "MobilePhoneNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.party_type_string {
                properties.insert(
                    "PartyTypeString".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.personal_email_address {
                properties.insert(
                    "PersonalEmailAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.phone_number {
                properties.insert(
                    "PhoneNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.profile_type {
                properties.insert(
                    "ProfileType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.shipping_address {
                properties.insert(
                    "ShippingAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-segmentdefinition-profiledimension.html
    pub struct ProfileDimension_ {
        pub dimension_type: crate::value::ExpString,
        pub values: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_SegmentDefinition_ProfileDimension {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::SegmentDefinition.ProfileDimension"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_SegmentDefinition_ProfileDimension as ProfileDimension;
    impl crate::value::ToValue for ProfileDimension_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DimensionType".to_string(),
                crate::value::ToValue::to_value(&self.dimension_type),
            );
            properties.insert(
                "Values".to_string(),
                crate::value::ToValue::to_value(&self.values),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-segmentdefinition-profiletypedimension.html
    pub struct ProfileTypeDimension_ {
        pub dimension_type: crate::value::ExpString,
        pub values: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_SegmentDefinition_ProfileTypeDimension {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::SegmentDefinition.ProfileTypeDimension"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_SegmentDefinition_ProfileTypeDimension as ProfileTypeDimension;
    impl crate::value::ToValue for ProfileTypeDimension_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DimensionType".to_string(),
                crate::value::ToValue::to_value(&self.dimension_type),
            );
            properties.insert(
                "Values".to_string(),
                crate::value::ToValue::to_value(&self.values),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-segmentdefinition-rangeoverride.html
    pub struct RangeOverride_ {
        pub end: Option<i64>,
        pub start: i64,
        pub unit: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_SegmentDefinition_RangeOverride {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::SegmentDefinition.RangeOverride"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_SegmentDefinition_RangeOverride as RangeOverride;
    impl crate::value::ToValue for RangeOverride_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.end {
                properties.insert("End".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Start".to_string(),
                crate::value::ToValue::to_value(&self.start),
            );
            properties.insert(
                "Unit".to_string(),
                crate::value::ToValue::to_value(&self.unit),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-segmentdefinition-segmentgroup.html
    pub struct SegmentGroup_ {
        pub groups: Option<Vec<Group_>>,
        pub include: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_SegmentDefinition_SegmentGroup {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::SegmentDefinition.SegmentGroup"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_SegmentDefinition_SegmentGroup as SegmentGroup;
    impl crate::value::ToValue for SegmentGroup_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.groups {
                properties.insert("Groups".to_string(), crate::value::ToValue::to_value(value));
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-segmentdefinition-sourcesegment.html
    pub struct SourceSegment_ {
        pub segment_definition_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_customerprofiles_SegmentDefinition_SourceSegment {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CustomerProfiles::SegmentDefinition.SourceSegment"
            $($field $value)*)
        };
    }
    pub use crate::__aws_customerprofiles_SegmentDefinition_SourceSegment as SourceSegment;
    impl crate::value::ToValue for SourceSegment_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.segment_definition_name {
                properties.insert(
                    "SegmentDefinitionName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-calculatedattributedefinition.html
pub struct CalculatedAttributeDefinition_ {
    pub attribute_details:
        super::customerprofiles::calculatedattributedefinition::AttributeDetails_,
    pub calculated_attribute_name: crate::value::ExpString,
    pub conditions: Option<super::customerprofiles::calculatedattributedefinition::Conditions_>,
    pub description: Option<crate::value::ExpString>,
    pub display_name: Option<crate::value::ExpString>,
    pub domain_name: crate::value::ExpString,
    pub statistic: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub use_historical_data: Option<crate::value::ExpBool>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_customerprofiles_CalculatedAttributeDefinition {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CustomerProfiles::CalculatedAttributeDefinition"
        $($field $value)*)
    };
}
pub use crate::__aws_customerprofiles_CalculatedAttributeDefinition as CalculatedAttributeDefinition;
impl crate::template::ToResource for CalculatedAttributeDefinition_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CustomerProfiles"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "CalculatedAttributeDefinition",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AttributeDetails".to_string(),
            crate::value::ToValue::to_value(&self.attribute_details),
        );
        properties.insert(
            "CalculatedAttributeName".to_string(),
            crate::value::ToValue::to_value(&self.calculated_attribute_name),
        );
        if let Some(ref value) = self.conditions {
            properties.insert(
                "Conditions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.display_name {
            properties.insert(
                "DisplayName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DomainName".to_string(),
            crate::value::ToValue::to_value(&self.domain_name),
        );
        properties.insert(
            "Statistic".to_string(),
            crate::value::ToValue::to_value(&self.statistic),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.use_historical_data {
            properties.insert(
                "UseHistoricalData".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-domain.html
pub struct Domain_ {
    pub dead_letter_queue_url: Option<crate::value::ExpString>,
    pub default_encryption_key: Option<crate::value::ExpString>,
    pub default_expiration_days: i64,
    pub domain_name: crate::value::ExpString,
    pub matching: Option<super::customerprofiles::domain::Matching_>,
    pub rule_based_matching: Option<super::customerprofiles::domain::RuleBasedMatching_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_customerprofiles_Domain {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CustomerProfiles::Domain"
        $($field $value)*)
    };
}
pub use crate::__aws_customerprofiles_Domain as Domain;
impl crate::template::ToResource for Domain_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CustomerProfiles"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Domain"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.dead_letter_queue_url {
            properties.insert(
                "DeadLetterQueueUrl".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.default_encryption_key {
            properties.insert(
                "DefaultEncryptionKey".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DefaultExpirationDays".to_string(),
            crate::value::ToValue::to_value(&self.default_expiration_days),
        );
        properties.insert(
            "DomainName".to_string(),
            crate::value::ToValue::to_value(&self.domain_name),
        );
        if let Some(ref value) = self.matching {
            properties.insert(
                "Matching".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.rule_based_matching {
            properties.insert(
                "RuleBasedMatching".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-eventstream.html
pub struct EventStream_ {
    pub domain_name: crate::value::ExpString,
    pub event_stream_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub uri: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_customerprofiles_EventStream {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CustomerProfiles::EventStream"
        $($field $value)*)
    };
}
pub use crate::__aws_customerprofiles_EventStream as EventStream;
impl crate::template::ToResource for EventStream_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CustomerProfiles"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("EventStream"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DomainName".to_string(),
            crate::value::ToValue::to_value(&self.domain_name),
        );
        properties.insert(
            "EventStreamName".to_string(),
            crate::value::ToValue::to_value(&self.event_stream_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Uri".to_string(),
            crate::value::ToValue::to_value(&self.uri),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-eventtrigger.html
pub struct EventTrigger_ {
    pub description: Option<crate::value::ExpString>,
    pub domain_name: crate::value::ExpString,
    pub event_trigger_conditions:
        Vec<super::customerprofiles::eventtrigger::EventTriggerCondition_>,
    pub event_trigger_limits: Option<super::customerprofiles::eventtrigger::EventTriggerLimits_>,
    pub event_trigger_name: crate::value::ExpString,
    pub object_type_name: crate::value::ExpString,
    pub segment_filter: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_customerprofiles_EventTrigger {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CustomerProfiles::EventTrigger"
        $($field $value)*)
    };
}
pub use crate::__aws_customerprofiles_EventTrigger as EventTrigger;
impl crate::template::ToResource for EventTrigger_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CustomerProfiles"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("EventTrigger"),
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
            "DomainName".to_string(),
            crate::value::ToValue::to_value(&self.domain_name),
        );
        properties.insert(
            "EventTriggerConditions".to_string(),
            crate::value::ToValue::to_value(&self.event_trigger_conditions),
        );
        if let Some(ref value) = self.event_trigger_limits {
            properties.insert(
                "EventTriggerLimits".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "EventTriggerName".to_string(),
            crate::value::ToValue::to_value(&self.event_trigger_name),
        );
        properties.insert(
            "ObjectTypeName".to_string(),
            crate::value::ToValue::to_value(&self.object_type_name),
        );
        if let Some(ref value) = self.segment_filter {
            properties.insert(
                "SegmentFilter".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-integration.html
pub struct Integration_ {
    pub domain_name: crate::value::ExpString,
    pub event_trigger_names: Option<Vec<crate::value::ExpString>>,
    pub flow_definition: Option<super::customerprofiles::integration::FlowDefinition_>,
    pub object_type_name: Option<crate::value::ExpString>,
    pub object_type_names: Option<Vec<super::customerprofiles::integration::ObjectTypeMapping_>>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub uri: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_customerprofiles_Integration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CustomerProfiles::Integration"
        $($field $value)*)
    };
}
pub use crate::__aws_customerprofiles_Integration as Integration;
impl crate::template::ToResource for Integration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CustomerProfiles"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Integration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DomainName".to_string(),
            crate::value::ToValue::to_value(&self.domain_name),
        );
        if let Some(ref value) = self.event_trigger_names {
            properties.insert(
                "EventTriggerNames".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.flow_definition {
            properties.insert(
                "FlowDefinition".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.object_type_name {
            properties.insert(
                "ObjectTypeName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.object_type_names {
            properties.insert(
                "ObjectTypeNames".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.uri {
            properties.insert("Uri".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-objecttype.html
pub struct ObjectType_ {
    pub allow_profile_creation: Option<crate::value::ExpBool>,
    pub description: crate::value::ExpString,
    pub domain_name: crate::value::ExpString,
    pub encryption_key: Option<crate::value::ExpString>,
    pub expiration_days: Option<i64>,
    pub fields: Option<Vec<super::customerprofiles::objecttype::FieldMap_>>,
    pub keys: Option<Vec<super::customerprofiles::objecttype::KeyMap_>>,
    pub max_profile_object_count: Option<i64>,
    pub object_type_name: crate::value::ExpString,
    pub source_last_updated_timestamp_format: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub template_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_customerprofiles_ObjectType {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CustomerProfiles::ObjectType"
        $($field $value)*)
    };
}
pub use crate::__aws_customerprofiles_ObjectType as ObjectType;
impl crate::template::ToResource for ObjectType_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CustomerProfiles"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ObjectType"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.allow_profile_creation {
            properties.insert(
                "AllowProfileCreation".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Description".to_string(),
            crate::value::ToValue::to_value(&self.description),
        );
        properties.insert(
            "DomainName".to_string(),
            crate::value::ToValue::to_value(&self.domain_name),
        );
        if let Some(ref value) = self.encryption_key {
            properties.insert(
                "EncryptionKey".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.expiration_days {
            properties.insert(
                "ExpirationDays".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.fields {
            properties.insert("Fields".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.keys {
            properties.insert("Keys".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.max_profile_object_count {
            properties.insert(
                "MaxProfileObjectCount".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ObjectTypeName".to_string(),
            crate::value::ToValue::to_value(&self.object_type_name),
        );
        if let Some(ref value) = self.source_last_updated_timestamp_format {
            properties.insert(
                "SourceLastUpdatedTimestampFormat".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.template_id {
            properties.insert(
                "TemplateId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-segmentdefinition.html
pub struct SegmentDefinition_ {
    pub description: Option<crate::value::ExpString>,
    pub display_name: crate::value::ExpString,
    pub domain_name: crate::value::ExpString,
    pub segment_definition_name: crate::value::ExpString,
    pub segment_groups: super::customerprofiles::segmentdefinition::SegmentGroup_,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_customerprofiles_SegmentDefinition {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CustomerProfiles::SegmentDefinition"
        $($field $value)*)
    };
}
pub use crate::__aws_customerprofiles_SegmentDefinition as SegmentDefinition;
impl crate::template::ToResource for SegmentDefinition_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CustomerProfiles"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SegmentDefinition"),
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
            "DisplayName".to_string(),
            crate::value::ToValue::to_value(&self.display_name),
        );
        properties.insert(
            "DomainName".to_string(),
            crate::value::ToValue::to_value(&self.domain_name),
        );
        properties.insert(
            "SegmentDefinitionName".to_string(),
            crate::value::ToValue::to_value(&self.segment_definition_name),
        );
        properties.insert(
            "SegmentGroups".to_string(),
            crate::value::ToValue::to_value(&self.segment_groups),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
