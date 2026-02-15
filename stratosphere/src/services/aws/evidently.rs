pub mod experiment {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-experiment-metricgoalobject.html
    pub struct MetricGoalObject_ {
        pub desired_change: crate::value::ExpString,
        pub entity_id_key: crate::value::ExpString,
        pub event_pattern: Option<crate::value::ExpString>,
        pub metric_name: crate::value::ExpString,
        pub unit_label: Option<crate::value::ExpString>,
        pub value_key: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_evidently_Experiment_MetricGoalObject {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Evidently::Experiment.MetricGoalObject"
            $($field $value)*)
        };
    }
    pub use crate::__aws_evidently_Experiment_MetricGoalObject as MetricGoalObject;
    impl crate::value::ToValue for MetricGoalObject_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DesiredChange".to_string(),
                crate::value::ToValue::to_value(&self.desired_change),
            );
            properties.insert(
                "EntityIdKey".to_string(),
                crate::value::ToValue::to_value(&self.entity_id_key),
            );
            if let Some(ref value) = self.event_pattern {
                properties.insert(
                    "EventPattern".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MetricName".to_string(),
                crate::value::ToValue::to_value(&self.metric_name),
            );
            if let Some(ref value) = self.unit_label {
                properties.insert(
                    "UnitLabel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ValueKey".to_string(),
                crate::value::ToValue::to_value(&self.value_key),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-experiment-onlineabconfigobject.html
    pub struct OnlineAbConfigObject_ {
        pub control_treatment_name: Option<crate::value::ExpString>,
        pub treatment_weights: Option<Vec<TreatmentToWeight_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_evidently_Experiment_OnlineAbConfigObject {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Evidently::Experiment.OnlineAbConfigObject"
            $($field $value)*)
        };
    }
    pub use crate::__aws_evidently_Experiment_OnlineAbConfigObject as OnlineAbConfigObject;
    impl crate::value::ToValue for OnlineAbConfigObject_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.control_treatment_name {
                properties.insert(
                    "ControlTreatmentName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.treatment_weights {
                properties.insert(
                    "TreatmentWeights".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-experiment-runningstatusobject.html
    pub struct RunningStatusObject_ {
        pub analysis_complete_time: Option<crate::value::ExpString>,
        pub desired_state: Option<crate::value::ExpString>,
        pub reason: Option<crate::value::ExpString>,
        pub status: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_evidently_Experiment_RunningStatusObject {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Evidently::Experiment.RunningStatusObject"
            $($field $value)*)
        };
    }
    pub use crate::__aws_evidently_Experiment_RunningStatusObject as RunningStatusObject;
    impl crate::value::ToValue for RunningStatusObject_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.analysis_complete_time {
                properties.insert(
                    "AnalysisCompleteTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.desired_state {
                properties.insert(
                    "DesiredState".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.reason {
                properties.insert("Reason".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Status".to_string(),
                crate::value::ToValue::to_value(&self.status),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-experiment-treatmentobject.html
    pub struct TreatmentObject_ {
        pub description: Option<crate::value::ExpString>,
        pub feature: crate::value::ExpString,
        pub treatment_name: crate::value::ExpString,
        pub variation: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_evidently_Experiment_TreatmentObject {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Evidently::Experiment.TreatmentObject"
            $($field $value)*)
        };
    }
    pub use crate::__aws_evidently_Experiment_TreatmentObject as TreatmentObject;
    impl crate::value::ToValue for TreatmentObject_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Feature".to_string(),
                crate::value::ToValue::to_value(&self.feature),
            );
            properties.insert(
                "TreatmentName".to_string(),
                crate::value::ToValue::to_value(&self.treatment_name),
            );
            properties.insert(
                "Variation".to_string(),
                crate::value::ToValue::to_value(&self.variation),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-experiment-treatmenttoweight.html
    pub struct TreatmentToWeight_ {
        pub split_weight: i32,
        pub treatment: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_evidently_Experiment_TreatmentToWeight {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Evidently::Experiment.TreatmentToWeight"
            $($field $value)*)
        };
    }
    pub use crate::__aws_evidently_Experiment_TreatmentToWeight as TreatmentToWeight;
    impl crate::value::ToValue for TreatmentToWeight_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SplitWeight".to_string(),
                crate::value::ToValue::to_value(&self.split_weight),
            );
            properties.insert(
                "Treatment".to_string(),
                crate::value::ToValue::to_value(&self.treatment),
            );
            properties.into()
        }
    }
}
pub mod feature {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-feature-entityoverride.html
    pub struct EntityOverride_ {
        pub entity_id: Option<crate::value::ExpString>,
        pub variation: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_evidently_Feature_EntityOverride {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Evidently::Feature.EntityOverride"
            $($field $value)*)
        };
    }
    pub use crate::__aws_evidently_Feature_EntityOverride as EntityOverride;
    impl crate::value::ToValue for EntityOverride_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.entity_id {
                properties.insert(
                    "EntityId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.variation {
                properties.insert(
                    "Variation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-feature-variationobject.html
    pub struct VariationObject_ {
        pub boolean_value: Option<crate::value::ExpBool>,
        pub double_value: Option<f64>,
        pub long_value: Option<f64>,
        pub string_value: Option<crate::value::ExpString>,
        pub variation_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_evidently_Feature_VariationObject {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Evidently::Feature.VariationObject"
            $($field $value)*)
        };
    }
    pub use crate::__aws_evidently_Feature_VariationObject as VariationObject;
    impl crate::value::ToValue for VariationObject_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.boolean_value {
                properties.insert(
                    "BooleanValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.double_value {
                properties.insert(
                    "DoubleValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.long_value {
                properties.insert(
                    "LongValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.string_value {
                properties.insert(
                    "StringValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "VariationName".to_string(),
                crate::value::ToValue::to_value(&self.variation_name),
            );
            properties.into()
        }
    }
}
pub mod launch {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-launch-executionstatusobject.html
    pub struct ExecutionStatusObject_ {
        pub desired_state: Option<crate::value::ExpString>,
        pub reason: Option<crate::value::ExpString>,
        pub status: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_evidently_Launch_ExecutionStatusObject {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Evidently::Launch.ExecutionStatusObject"
            $($field $value)*)
        };
    }
    pub use crate::__aws_evidently_Launch_ExecutionStatusObject as ExecutionStatusObject;
    impl crate::value::ToValue for ExecutionStatusObject_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.desired_state {
                properties.insert(
                    "DesiredState".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.reason {
                properties.insert("Reason".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Status".to_string(),
                crate::value::ToValue::to_value(&self.status),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-launch-grouptoweight.html
    pub struct GroupToWeight_ {
        pub group_name: crate::value::ExpString,
        pub split_weight: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_evidently_Launch_GroupToWeight {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Evidently::Launch.GroupToWeight"
            $($field $value)*)
        };
    }
    pub use crate::__aws_evidently_Launch_GroupToWeight as GroupToWeight;
    impl crate::value::ToValue for GroupToWeight_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "GroupName".to_string(),
                crate::value::ToValue::to_value(&self.group_name),
            );
            properties.insert(
                "SplitWeight".to_string(),
                crate::value::ToValue::to_value(&self.split_weight),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-launch-launchgroupobject.html
    pub struct LaunchGroupObject_ {
        pub description: Option<crate::value::ExpString>,
        pub feature: crate::value::ExpString,
        pub group_name: crate::value::ExpString,
        pub variation: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_evidently_Launch_LaunchGroupObject {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Evidently::Launch.LaunchGroupObject"
            $($field $value)*)
        };
    }
    pub use crate::__aws_evidently_Launch_LaunchGroupObject as LaunchGroupObject;
    impl crate::value::ToValue for LaunchGroupObject_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Feature".to_string(),
                crate::value::ToValue::to_value(&self.feature),
            );
            properties.insert(
                "GroupName".to_string(),
                crate::value::ToValue::to_value(&self.group_name),
            );
            properties.insert(
                "Variation".to_string(),
                crate::value::ToValue::to_value(&self.variation),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-launch-metricdefinitionobject.html
    pub struct MetricDefinitionObject_ {
        pub entity_id_key: crate::value::ExpString,
        pub event_pattern: Option<crate::value::ExpString>,
        pub metric_name: crate::value::ExpString,
        pub unit_label: Option<crate::value::ExpString>,
        pub value_key: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_evidently_Launch_MetricDefinitionObject {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Evidently::Launch.MetricDefinitionObject"
            $($field $value)*)
        };
    }
    pub use crate::__aws_evidently_Launch_MetricDefinitionObject as MetricDefinitionObject;
    impl crate::value::ToValue for MetricDefinitionObject_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EntityIdKey".to_string(),
                crate::value::ToValue::to_value(&self.entity_id_key),
            );
            if let Some(ref value) = self.event_pattern {
                properties.insert(
                    "EventPattern".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MetricName".to_string(),
                crate::value::ToValue::to_value(&self.metric_name),
            );
            if let Some(ref value) = self.unit_label {
                properties.insert(
                    "UnitLabel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ValueKey".to_string(),
                crate::value::ToValue::to_value(&self.value_key),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-launch-segmentoverride.html
    pub struct SegmentOverride_ {
        pub evaluation_order: i32,
        pub segment: crate::value::ExpString,
        pub weights: Vec<GroupToWeight_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_evidently_Launch_SegmentOverride {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Evidently::Launch.SegmentOverride"
            $($field $value)*)
        };
    }
    pub use crate::__aws_evidently_Launch_SegmentOverride as SegmentOverride;
    impl crate::value::ToValue for SegmentOverride_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EvaluationOrder".to_string(),
                crate::value::ToValue::to_value(&self.evaluation_order),
            );
            properties.insert(
                "Segment".to_string(),
                crate::value::ToValue::to_value(&self.segment),
            );
            properties.insert(
                "Weights".to_string(),
                crate::value::ToValue::to_value(&self.weights),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-launch-stepconfig.html
    pub struct StepConfig_ {
        pub group_weights: Vec<GroupToWeight_>,
        pub segment_overrides: Option<Vec<SegmentOverride_>>,
        pub start_time: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_evidently_Launch_StepConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Evidently::Launch.StepConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_evidently_Launch_StepConfig as StepConfig;
    impl crate::value::ToValue for StepConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "GroupWeights".to_string(),
                crate::value::ToValue::to_value(&self.group_weights),
            );
            if let Some(ref value) = self.segment_overrides {
                properties.insert(
                    "SegmentOverrides".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "StartTime".to_string(),
                crate::value::ToValue::to_value(&self.start_time),
            );
            properties.into()
        }
    }
}
pub mod project {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-project-appconfigresourceobject.html
    pub struct AppConfigResourceObject_ {
        pub application_id: crate::value::ExpString,
        pub environment_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_evidently_Project_AppConfigResourceObject {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Evidently::Project.AppConfigResourceObject"
            $($field $value)*)
        };
    }
    pub use crate::__aws_evidently_Project_AppConfigResourceObject as AppConfigResourceObject;
    impl crate::value::ToValue for AppConfigResourceObject_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ApplicationId".to_string(),
                crate::value::ToValue::to_value(&self.application_id),
            );
            properties.insert(
                "EnvironmentId".to_string(),
                crate::value::ToValue::to_value(&self.environment_id),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-project-datadeliveryobject.html
    pub struct DataDeliveryObject_ {
        pub log_group: Option<crate::value::ExpString>,
        pub s3: Option<Box<S3Destination_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_evidently_Project_DataDeliveryObject {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Evidently::Project.DataDeliveryObject"
            $($field $value)*)
        };
    }
    pub use crate::__aws_evidently_Project_DataDeliveryObject as DataDeliveryObject;
    impl crate::value::ToValue for DataDeliveryObject_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.log_group {
                properties.insert(
                    "LogGroup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3 {
                properties.insert("S3".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-project-s3destination.html
    pub struct S3Destination_ {
        pub bucket_name: crate::value::ExpString,
        pub prefix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_evidently_Project_S3Destination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Evidently::Project.S3Destination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_evidently_Project_S3Destination as S3Destination;
    impl crate::value::ToValue for S3Destination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BucketName".to_string(),
                crate::value::ToValue::to_value(&self.bucket_name),
            );
            if let Some(ref value) = self.prefix {
                properties.insert("Prefix".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-experiment.html
pub struct Experiment_ {
    pub description: Option<crate::value::ExpString>,
    pub metric_goals: Vec<super::evidently::experiment::MetricGoalObject_>,
    pub name: crate::value::ExpString,
    pub online_ab_config: super::evidently::experiment::OnlineAbConfigObject_,
    pub project: crate::value::ExpString,
    pub randomization_salt: Option<crate::value::ExpString>,
    pub remove_segment: Option<crate::value::ExpBool>,
    pub running_status: Option<super::evidently::experiment::RunningStatusObject_>,
    pub sampling_rate: Option<i32>,
    pub segment: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub treatments: Vec<super::evidently::experiment::TreatmentObject_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_evidently_Experiment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Evidently::Experiment"
        $($field $value)*)
    };
}
pub use crate::__aws_evidently_Experiment as Experiment;
impl crate::template::ToResource for Experiment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Evidently"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Experiment"),
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
            "MetricGoals".to_string(),
            crate::value::ToValue::to_value(&self.metric_goals),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "OnlineAbConfig".to_string(),
            crate::value::ToValue::to_value(&self.online_ab_config),
        );
        properties.insert(
            "Project".to_string(),
            crate::value::ToValue::to_value(&self.project),
        );
        if let Some(ref value) = self.randomization_salt {
            properties.insert(
                "RandomizationSalt".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.remove_segment {
            properties.insert(
                "RemoveSegment".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.running_status {
            properties.insert(
                "RunningStatus".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sampling_rate {
            properties.insert(
                "SamplingRate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.segment {
            properties.insert(
                "Segment".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Treatments".to_string(),
            crate::value::ToValue::to_value(&self.treatments),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-feature.html
pub struct Feature_ {
    pub default_variation: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub entity_overrides: Option<Vec<super::evidently::feature::EntityOverride_>>,
    pub evaluation_strategy: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub project: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub variations: Vec<super::evidently::feature::VariationObject_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_evidently_Feature {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Evidently::Feature"
        $($field $value)*)
    };
}
pub use crate::__aws_evidently_Feature as Feature;
impl crate::template::ToResource for Feature_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Evidently"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Feature"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.default_variation {
            properties.insert(
                "DefaultVariation".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.entity_overrides {
            properties.insert(
                "EntityOverrides".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.evaluation_strategy {
            properties.insert(
                "EvaluationStrategy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "Project".to_string(),
            crate::value::ToValue::to_value(&self.project),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Variations".to_string(),
            crate::value::ToValue::to_value(&self.variations),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-launch.html
pub struct Launch_ {
    pub description: Option<crate::value::ExpString>,
    pub execution_status: Option<super::evidently::launch::ExecutionStatusObject_>,
    pub groups: Vec<super::evidently::launch::LaunchGroupObject_>,
    pub metric_monitors: Option<Vec<super::evidently::launch::MetricDefinitionObject_>>,
    pub name: crate::value::ExpString,
    pub project: crate::value::ExpString,
    pub randomization_salt: Option<crate::value::ExpString>,
    pub scheduled_splits_config: Vec<super::evidently::launch::StepConfig_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_evidently_Launch {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Evidently::Launch"
        $($field $value)*)
    };
}
pub use crate::__aws_evidently_Launch as Launch;
impl crate::template::ToResource for Launch_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Evidently"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Launch"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.execution_status {
            properties.insert(
                "ExecutionStatus".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Groups".to_string(),
            crate::value::ToValue::to_value(&self.groups),
        );
        if let Some(ref value) = self.metric_monitors {
            properties.insert(
                "MetricMonitors".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "Project".to_string(),
            crate::value::ToValue::to_value(&self.project),
        );
        if let Some(ref value) = self.randomization_salt {
            properties.insert(
                "RandomizationSalt".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ScheduledSplitsConfig".to_string(),
            crate::value::ToValue::to_value(&self.scheduled_splits_config),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-project.html
pub struct Project_ {
    pub app_config_resource: Option<super::evidently::project::AppConfigResourceObject_>,
    pub data_delivery: Option<super::evidently::project::DataDeliveryObject_>,
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_evidently_Project {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Evidently::Project"
        $($field $value)*)
    };
}
pub use crate::__aws_evidently_Project as Project;
impl crate::template::ToResource for Project_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Evidently"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Project"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.app_config_resource {
            properties.insert(
                "AppConfigResource".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.data_delivery {
            properties.insert(
                "DataDelivery".to_string(),
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
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-segment.html
pub struct Segment_ {
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub pattern: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_evidently_Segment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Evidently::Segment"
        $($field $value)*)
    };
}
pub use crate::__aws_evidently_Segment as Segment;
impl crate::template::ToResource for Segment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Evidently"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Segment"),
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
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.pattern {
            properties.insert(
                "Pattern".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
