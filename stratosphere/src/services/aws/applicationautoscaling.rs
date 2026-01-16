pub mod scalabletarget {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalabletarget-scalabletargetaction.html
    pub struct ScalableTargetAction_ {
        pub max_capacity: Option<i64>,
        pub min_capacity: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationautoscaling_ScalableTarget_ScalableTargetAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationAutoScaling::ScalableTarget.ScalableTargetAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationautoscaling_ScalableTarget_ScalableTargetAction as ScalableTargetAction;
    impl crate::value::ToValue for ScalableTargetAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max_capacity {
                properties.insert(
                    "MaxCapacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.min_capacity {
                properties.insert(
                    "MinCapacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalabletarget-scheduledaction.html
    pub struct ScheduledAction_ {
        pub end_time: Option<chrono::DateTime<chrono::Utc>>,
        pub scalable_target_action: Option<Box<ScalableTargetAction_>>,
        pub schedule: crate::value::ExpString,
        pub scheduled_action_name: crate::value::ExpString,
        pub start_time: Option<chrono::DateTime<chrono::Utc>>,
        pub timezone: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationautoscaling_ScalableTarget_ScheduledAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationAutoScaling::ScalableTarget.ScheduledAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationautoscaling_ScalableTarget_ScheduledAction as ScheduledAction;
    impl crate::value::ToValue for ScheduledAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.end_time {
                properties.insert(
                    "EndTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scalable_target_action {
                properties.insert(
                    "ScalableTargetAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Schedule".to_string(),
                crate::value::ToValue::to_value(&self.schedule),
            );
            properties.insert(
                "ScheduledActionName".to_string(),
                crate::value::ToValue::to_value(&self.scheduled_action_name),
            );
            if let Some(ref value) = self.start_time {
                properties.insert(
                    "StartTime".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalabletarget-suspendedstate.html
    pub struct SuspendedState_ {
        pub dynamic_scaling_in_suspended: Option<crate::value::ExpBool>,
        pub dynamic_scaling_out_suspended: Option<crate::value::ExpBool>,
        pub scheduled_scaling_suspended: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationautoscaling_ScalableTarget_SuspendedState {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationAutoScaling::ScalableTarget.SuspendedState"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationautoscaling_ScalableTarget_SuspendedState as SuspendedState;
    impl crate::value::ToValue for SuspendedState_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dynamic_scaling_in_suspended {
                properties.insert(
                    "DynamicScalingInSuspended".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dynamic_scaling_out_suspended {
                properties.insert(
                    "DynamicScalingOutSuspended".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scheduled_scaling_suspended {
                properties.insert(
                    "ScheduledScalingSuspended".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod scalingpolicy {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-customizedmetricspecification.html
    pub struct CustomizedMetricSpecification_ {
        pub dimensions: Option<Vec<MetricDimension_>>,
        pub metric_name: Option<crate::value::ExpString>,
        pub metrics: Option<Vec<TargetTrackingMetricDataQuery_>>,
        pub namespace: Option<crate::value::ExpString>,
        pub statistic: Option<crate::value::ExpString>,
        pub unit: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationautoscaling_ScalingPolicy_CustomizedMetricSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationAutoScaling::ScalingPolicy.CustomizedMetricSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationautoscaling_ScalingPolicy_CustomizedMetricSpecification as CustomizedMetricSpecification;
    impl crate::value::ToValue for CustomizedMetricSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dimensions {
                properties.insert(
                    "Dimensions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metric_name {
                properties.insert(
                    "MetricName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metrics {
                properties.insert(
                    "Metrics".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.namespace {
                properties.insert(
                    "Namespace".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.statistic {
                properties.insert(
                    "Statistic".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.unit {
                properties.insert("Unit".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-metricdimension.html
    pub struct MetricDimension_ {
        pub name: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationautoscaling_ScalingPolicy_MetricDimension {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationAutoScaling::ScalingPolicy.MetricDimension"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationautoscaling_ScalingPolicy_MetricDimension as MetricDimension;
    impl crate::value::ToValue for MetricDimension_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-predefinedmetricspecification.html
    pub struct PredefinedMetricSpecification_ {
        pub predefined_metric_type: crate::value::ExpString,
        pub resource_label: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationautoscaling_ScalingPolicy_PredefinedMetricSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationAutoScaling::ScalingPolicy.PredefinedMetricSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationautoscaling_ScalingPolicy_PredefinedMetricSpecification as PredefinedMetricSpecification;
    impl crate::value::ToValue for PredefinedMetricSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PredefinedMetricType".to_string(),
                crate::value::ToValue::to_value(&self.predefined_metric_type),
            );
            if let Some(ref value) = self.resource_label {
                properties.insert(
                    "ResourceLabel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-predictivescalingcustomizedcapacitymetric.html
    pub struct PredictiveScalingCustomizedCapacityMetric_ {
        pub metric_data_queries: Vec<PredictiveScalingMetricDataQuery_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationautoscaling_ScalingPolicy_PredictiveScalingCustomizedCapacityMetric {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationAutoScaling::ScalingPolicy.PredictiveScalingCustomizedCapacityMetric"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationautoscaling_ScalingPolicy_PredictiveScalingCustomizedCapacityMetric as PredictiveScalingCustomizedCapacityMetric;
    impl crate::value::ToValue for PredictiveScalingCustomizedCapacityMetric_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MetricDataQueries".to_string(),
                crate::value::ToValue::to_value(&self.metric_data_queries),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-predictivescalingcustomizedloadmetric.html
    pub struct PredictiveScalingCustomizedLoadMetric_ {
        pub metric_data_queries: Vec<PredictiveScalingMetricDataQuery_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationautoscaling_ScalingPolicy_PredictiveScalingCustomizedLoadMetric {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationAutoScaling::ScalingPolicy.PredictiveScalingCustomizedLoadMetric"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationautoscaling_ScalingPolicy_PredictiveScalingCustomizedLoadMetric as PredictiveScalingCustomizedLoadMetric;
    impl crate::value::ToValue for PredictiveScalingCustomizedLoadMetric_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MetricDataQueries".to_string(),
                crate::value::ToValue::to_value(&self.metric_data_queries),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-predictivescalingcustomizedscalingmetric.html
    pub struct PredictiveScalingCustomizedScalingMetric_ {
        pub metric_data_queries: Vec<PredictiveScalingMetricDataQuery_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationautoscaling_ScalingPolicy_PredictiveScalingCustomizedScalingMetric {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationAutoScaling::ScalingPolicy.PredictiveScalingCustomizedScalingMetric"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationautoscaling_ScalingPolicy_PredictiveScalingCustomizedScalingMetric as PredictiveScalingCustomizedScalingMetric;
    impl crate::value::ToValue for PredictiveScalingCustomizedScalingMetric_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MetricDataQueries".to_string(),
                crate::value::ToValue::to_value(&self.metric_data_queries),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-predictivescalingmetric.html
    pub struct PredictiveScalingMetric_ {
        pub dimensions: Option<Vec<PredictiveScalingMetricDimension_>>,
        pub metric_name: Option<crate::value::ExpString>,
        pub namespace: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationautoscaling_ScalingPolicy_PredictiveScalingMetric {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationAutoScaling::ScalingPolicy.PredictiveScalingMetric"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationautoscaling_ScalingPolicy_PredictiveScalingMetric as PredictiveScalingMetric;
    impl crate::value::ToValue for PredictiveScalingMetric_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dimensions {
                properties.insert(
                    "Dimensions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metric_name {
                properties.insert(
                    "MetricName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.namespace {
                properties.insert(
                    "Namespace".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-predictivescalingmetricdataquery.html
    pub struct PredictiveScalingMetricDataQuery_ {
        pub expression: Option<crate::value::ExpString>,
        pub id: Option<crate::value::ExpString>,
        pub label: Option<crate::value::ExpString>,
        pub metric_stat: Option<Box<PredictiveScalingMetricStat_>>,
        pub return_data: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationautoscaling_ScalingPolicy_PredictiveScalingMetricDataQuery {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationAutoScaling::ScalingPolicy.PredictiveScalingMetricDataQuery"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationautoscaling_ScalingPolicy_PredictiveScalingMetricDataQuery as PredictiveScalingMetricDataQuery;
    impl crate::value::ToValue for PredictiveScalingMetricDataQuery_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.expression {
                properties.insert(
                    "Expression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.id {
                properties.insert("Id".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.label {
                properties.insert("Label".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.metric_stat {
                properties.insert(
                    "MetricStat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.return_data {
                properties.insert(
                    "ReturnData".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-predictivescalingmetricdimension.html
    pub struct PredictiveScalingMetricDimension_ {
        pub name: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationautoscaling_ScalingPolicy_PredictiveScalingMetricDimension {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationAutoScaling::ScalingPolicy.PredictiveScalingMetricDimension"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationautoscaling_ScalingPolicy_PredictiveScalingMetricDimension as PredictiveScalingMetricDimension;
    impl crate::value::ToValue for PredictiveScalingMetricDimension_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-predictivescalingmetricspecification.html
    pub struct PredictiveScalingMetricSpecification_ {
        pub customized_capacity_metric_specification:
            Option<Box<PredictiveScalingCustomizedCapacityMetric_>>,
        pub customized_load_metric_specification:
            Option<Box<PredictiveScalingCustomizedLoadMetric_>>,
        pub customized_scaling_metric_specification:
            Option<Box<PredictiveScalingCustomizedScalingMetric_>>,
        pub predefined_load_metric_specification:
            Option<Box<PredictiveScalingPredefinedLoadMetric_>>,
        pub predefined_metric_pair_specification:
            Option<Box<PredictiveScalingPredefinedMetricPair_>>,
        pub predefined_scaling_metric_specification:
            Option<Box<PredictiveScalingPredefinedScalingMetric_>>,
        pub target_value: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationautoscaling_ScalingPolicy_PredictiveScalingMetricSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationAutoScaling::ScalingPolicy.PredictiveScalingMetricSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationautoscaling_ScalingPolicy_PredictiveScalingMetricSpecification as PredictiveScalingMetricSpecification;
    impl crate::value::ToValue for PredictiveScalingMetricSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.customized_capacity_metric_specification {
                properties.insert(
                    "CustomizedCapacityMetricSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.customized_load_metric_specification {
                properties.insert(
                    "CustomizedLoadMetricSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.customized_scaling_metric_specification {
                properties.insert(
                    "CustomizedScalingMetricSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.predefined_load_metric_specification {
                properties.insert(
                    "PredefinedLoadMetricSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.predefined_metric_pair_specification {
                properties.insert(
                    "PredefinedMetricPairSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.predefined_scaling_metric_specification {
                properties.insert(
                    "PredefinedScalingMetricSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TargetValue".to_string(),
                crate::value::ToValue::to_value(&self.target_value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-predictivescalingmetricstat.html
    pub struct PredictiveScalingMetricStat_ {
        pub metric: Option<Box<PredictiveScalingMetric_>>,
        pub stat: Option<crate::value::ExpString>,
        pub unit: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationautoscaling_ScalingPolicy_PredictiveScalingMetricStat {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationAutoScaling::ScalingPolicy.PredictiveScalingMetricStat"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationautoscaling_ScalingPolicy_PredictiveScalingMetricStat as PredictiveScalingMetricStat;
    impl crate::value::ToValue for PredictiveScalingMetricStat_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.metric {
                properties.insert("Metric".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.stat {
                properties.insert("Stat".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.unit {
                properties.insert("Unit".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-predictivescalingpolicyconfiguration.html
    pub struct PredictiveScalingPolicyConfiguration_ {
        pub max_capacity_breach_behavior: Option<crate::value::ExpString>,
        pub max_capacity_buffer: Option<i64>,
        pub metric_specifications: Vec<PredictiveScalingMetricSpecification_>,
        pub mode: Option<crate::value::ExpString>,
        pub scheduling_buffer_time: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationautoscaling_ScalingPolicy_PredictiveScalingPolicyConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationAutoScaling::ScalingPolicy.PredictiveScalingPolicyConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationautoscaling_ScalingPolicy_PredictiveScalingPolicyConfiguration as PredictiveScalingPolicyConfiguration;
    impl crate::value::ToValue for PredictiveScalingPolicyConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max_capacity_breach_behavior {
                properties.insert(
                    "MaxCapacityBreachBehavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_capacity_buffer {
                properties.insert(
                    "MaxCapacityBuffer".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MetricSpecifications".to_string(),
                crate::value::ToValue::to_value(&self.metric_specifications),
            );
            if let Some(ref value) = self.mode {
                properties.insert("Mode".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.scheduling_buffer_time {
                properties.insert(
                    "SchedulingBufferTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-predictivescalingpredefinedloadmetric.html
    pub struct PredictiveScalingPredefinedLoadMetric_ {
        pub predefined_metric_type: crate::value::ExpString,
        pub resource_label: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationautoscaling_ScalingPolicy_PredictiveScalingPredefinedLoadMetric {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationAutoScaling::ScalingPolicy.PredictiveScalingPredefinedLoadMetric"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationautoscaling_ScalingPolicy_PredictiveScalingPredefinedLoadMetric as PredictiveScalingPredefinedLoadMetric;
    impl crate::value::ToValue for PredictiveScalingPredefinedLoadMetric_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PredefinedMetricType".to_string(),
                crate::value::ToValue::to_value(&self.predefined_metric_type),
            );
            if let Some(ref value) = self.resource_label {
                properties.insert(
                    "ResourceLabel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-predictivescalingpredefinedmetricpair.html
    pub struct PredictiveScalingPredefinedMetricPair_ {
        pub predefined_metric_type: crate::value::ExpString,
        pub resource_label: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationautoscaling_ScalingPolicy_PredictiveScalingPredefinedMetricPair {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationAutoScaling::ScalingPolicy.PredictiveScalingPredefinedMetricPair"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationautoscaling_ScalingPolicy_PredictiveScalingPredefinedMetricPair as PredictiveScalingPredefinedMetricPair;
    impl crate::value::ToValue for PredictiveScalingPredefinedMetricPair_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PredefinedMetricType".to_string(),
                crate::value::ToValue::to_value(&self.predefined_metric_type),
            );
            if let Some(ref value) = self.resource_label {
                properties.insert(
                    "ResourceLabel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-predictivescalingpredefinedscalingmetric.html
    pub struct PredictiveScalingPredefinedScalingMetric_ {
        pub predefined_metric_type: crate::value::ExpString,
        pub resource_label: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationautoscaling_ScalingPolicy_PredictiveScalingPredefinedScalingMetric {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationAutoScaling::ScalingPolicy.PredictiveScalingPredefinedScalingMetric"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationautoscaling_ScalingPolicy_PredictiveScalingPredefinedScalingMetric as PredictiveScalingPredefinedScalingMetric;
    impl crate::value::ToValue for PredictiveScalingPredefinedScalingMetric_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PredefinedMetricType".to_string(),
                crate::value::ToValue::to_value(&self.predefined_metric_type),
            );
            if let Some(ref value) = self.resource_label {
                properties.insert(
                    "ResourceLabel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-stepadjustment.html
    pub struct StepAdjustment_ {
        pub metric_interval_lower_bound: Option<f64>,
        pub metric_interval_upper_bound: Option<f64>,
        pub scaling_adjustment: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationautoscaling_ScalingPolicy_StepAdjustment {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationAutoScaling::ScalingPolicy.StepAdjustment"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationautoscaling_ScalingPolicy_StepAdjustment as StepAdjustment;
    impl crate::value::ToValue for StepAdjustment_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.metric_interval_lower_bound {
                properties.insert(
                    "MetricIntervalLowerBound".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metric_interval_upper_bound {
                properties.insert(
                    "MetricIntervalUpperBound".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ScalingAdjustment".to_string(),
                crate::value::ToValue::to_value(&self.scaling_adjustment),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-stepscalingpolicyconfiguration.html
    pub struct StepScalingPolicyConfiguration_ {
        pub adjustment_type: Option<crate::value::ExpString>,
        pub cooldown: Option<i64>,
        pub metric_aggregation_type: Option<crate::value::ExpString>,
        pub min_adjustment_magnitude: Option<i64>,
        pub step_adjustments: Option<Vec<StepAdjustment_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationautoscaling_ScalingPolicy_StepScalingPolicyConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationAutoScaling::ScalingPolicy.StepScalingPolicyConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationautoscaling_ScalingPolicy_StepScalingPolicyConfiguration as StepScalingPolicyConfiguration;
    impl crate::value::ToValue for StepScalingPolicyConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.adjustment_type {
                properties.insert(
                    "AdjustmentType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cooldown {
                properties.insert(
                    "Cooldown".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metric_aggregation_type {
                properties.insert(
                    "MetricAggregationType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.min_adjustment_magnitude {
                properties.insert(
                    "MinAdjustmentMagnitude".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.step_adjustments {
                properties.insert(
                    "StepAdjustments".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-targettrackingmetric.html
    pub struct TargetTrackingMetric_ {
        pub dimensions: Option<Vec<TargetTrackingMetricDimension_>>,
        pub metric_name: Option<crate::value::ExpString>,
        pub namespace: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationautoscaling_ScalingPolicy_TargetTrackingMetric {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationAutoScaling::ScalingPolicy.TargetTrackingMetric"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationautoscaling_ScalingPolicy_TargetTrackingMetric as TargetTrackingMetric;
    impl crate::value::ToValue for TargetTrackingMetric_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dimensions {
                properties.insert(
                    "Dimensions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metric_name {
                properties.insert(
                    "MetricName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.namespace {
                properties.insert(
                    "Namespace".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-targettrackingmetricdataquery.html
    pub struct TargetTrackingMetricDataQuery_ {
        pub expression: Option<crate::value::ExpString>,
        pub id: Option<crate::value::ExpString>,
        pub label: Option<crate::value::ExpString>,
        pub metric_stat: Option<Box<TargetTrackingMetricStat_>>,
        pub return_data: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationautoscaling_ScalingPolicy_TargetTrackingMetricDataQuery {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationAutoScaling::ScalingPolicy.TargetTrackingMetricDataQuery"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationautoscaling_ScalingPolicy_TargetTrackingMetricDataQuery as TargetTrackingMetricDataQuery;
    impl crate::value::ToValue for TargetTrackingMetricDataQuery_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.expression {
                properties.insert(
                    "Expression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.id {
                properties.insert("Id".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.label {
                properties.insert("Label".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.metric_stat {
                properties.insert(
                    "MetricStat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.return_data {
                properties.insert(
                    "ReturnData".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-targettrackingmetricdimension.html
    pub struct TargetTrackingMetricDimension_ {
        pub name: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationautoscaling_ScalingPolicy_TargetTrackingMetricDimension {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationAutoScaling::ScalingPolicy.TargetTrackingMetricDimension"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationautoscaling_ScalingPolicy_TargetTrackingMetricDimension as TargetTrackingMetricDimension;
    impl crate::value::ToValue for TargetTrackingMetricDimension_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-targettrackingmetricstat.html
    pub struct TargetTrackingMetricStat_ {
        pub metric: Option<Box<TargetTrackingMetric_>>,
        pub stat: Option<crate::value::ExpString>,
        pub unit: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationautoscaling_ScalingPolicy_TargetTrackingMetricStat {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationAutoScaling::ScalingPolicy.TargetTrackingMetricStat"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationautoscaling_ScalingPolicy_TargetTrackingMetricStat as TargetTrackingMetricStat;
    impl crate::value::ToValue for TargetTrackingMetricStat_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.metric {
                properties.insert("Metric".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.stat {
                properties.insert("Stat".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.unit {
                properties.insert("Unit".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-targettrackingscalingpolicyconfiguration.html
    pub struct TargetTrackingScalingPolicyConfiguration_ {
        pub customized_metric_specification: Option<Box<CustomizedMetricSpecification_>>,
        pub disable_scale_in: Option<crate::value::ExpBool>,
        pub predefined_metric_specification: Option<Box<PredefinedMetricSpecification_>>,
        pub scale_in_cooldown: Option<i64>,
        pub scale_out_cooldown: Option<i64>,
        pub target_value: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationautoscaling_ScalingPolicy_TargetTrackingScalingPolicyConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationAutoScaling::ScalingPolicy.TargetTrackingScalingPolicyConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationautoscaling_ScalingPolicy_TargetTrackingScalingPolicyConfiguration as TargetTrackingScalingPolicyConfiguration;
    impl crate::value::ToValue for TargetTrackingScalingPolicyConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.customized_metric_specification {
                properties.insert(
                    "CustomizedMetricSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.disable_scale_in {
                properties.insert(
                    "DisableScaleIn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.predefined_metric_specification {
                properties.insert(
                    "PredefinedMetricSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scale_in_cooldown {
                properties.insert(
                    "ScaleInCooldown".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scale_out_cooldown {
                properties.insert(
                    "ScaleOutCooldown".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TargetValue".to_string(),
                crate::value::ToValue::to_value(&self.target_value),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationautoscaling-scalabletarget.html
pub struct ScalableTarget_ {
    pub max_capacity: i64,
    pub min_capacity: i64,
    pub resource_id: crate::value::ExpString,
    pub role_arn: Option<crate::value::ExpString>,
    pub scalable_dimension: crate::value::ExpString,
    pub scheduled_actions:
        Option<Vec<super::applicationautoscaling::scalabletarget::ScheduledAction_>>,
    pub service_namespace: crate::value::ExpString,
    pub suspended_state: Option<super::applicationautoscaling::scalabletarget::SuspendedState_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_applicationautoscaling_ScalableTarget {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::ApplicationAutoScaling::ScalableTarget"
        $($field $value)*)
    };
}
pub use crate::__aws_applicationautoscaling_ScalableTarget as ScalableTarget;
impl crate::template::ToResource for ScalableTarget_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApplicationAutoScaling"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ScalableTarget"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "MaxCapacity".to_string(),
            crate::value::ToValue::to_value(&self.max_capacity),
        );
        properties.insert(
            "MinCapacity".to_string(),
            crate::value::ToValue::to_value(&self.min_capacity),
        );
        properties.insert(
            "ResourceId".to_string(),
            crate::value::ToValue::to_value(&self.resource_id),
        );
        if let Some(ref value) = self.role_arn {
            properties.insert(
                "RoleARN".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ScalableDimension".to_string(),
            crate::value::ToValue::to_value(&self.scalable_dimension),
        );
        if let Some(ref value) = self.scheduled_actions {
            properties.insert(
                "ScheduledActions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ServiceNamespace".to_string(),
            crate::value::ToValue::to_value(&self.service_namespace),
        );
        if let Some(ref value) = self.suspended_state {
            properties.insert(
                "SuspendedState".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationautoscaling-scalingpolicy.html
pub struct ScalingPolicy_ {
    pub policy_name: crate::value::ExpString,
    pub policy_type: crate::value::ExpString,
    pub predictive_scaling_policy_configuration:
        Option<super::applicationautoscaling::scalingpolicy::PredictiveScalingPolicyConfiguration_>,
    pub resource_id: Option<crate::value::ExpString>,
    pub scalable_dimension: Option<crate::value::ExpString>,
    pub scaling_target_id: Option<crate::value::ExpString>,
    pub service_namespace: Option<crate::value::ExpString>,
    pub step_scaling_policy_configuration:
        Option<super::applicationautoscaling::scalingpolicy::StepScalingPolicyConfiguration_>,
    pub target_tracking_scaling_policy_configuration: Option<
        super::applicationautoscaling::scalingpolicy::TargetTrackingScalingPolicyConfiguration_,
    >,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_applicationautoscaling_ScalingPolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::ApplicationAutoScaling::ScalingPolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_applicationautoscaling_ScalingPolicy as ScalingPolicy;
impl crate::template::ToResource for ScalingPolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApplicationAutoScaling"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ScalingPolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "PolicyName".to_string(),
            crate::value::ToValue::to_value(&self.policy_name),
        );
        properties.insert(
            "PolicyType".to_string(),
            crate::value::ToValue::to_value(&self.policy_type),
        );
        if let Some(ref value) = self.predictive_scaling_policy_configuration {
            properties.insert(
                "PredictiveScalingPolicyConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resource_id {
            properties.insert(
                "ResourceId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.scalable_dimension {
            properties.insert(
                "ScalableDimension".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.scaling_target_id {
            properties.insert(
                "ScalingTargetId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.service_namespace {
            properties.insert(
                "ServiceNamespace".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.step_scaling_policy_configuration {
            properties.insert(
                "StepScalingPolicyConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.target_tracking_scaling_policy_configuration {
            properties.insert(
                "TargetTrackingScalingPolicyConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
