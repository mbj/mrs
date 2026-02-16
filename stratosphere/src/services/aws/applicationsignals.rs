pub mod groupingconfiguration {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationsignals-groupingconfiguration-groupingattributedefinition.html
    pub struct GroupingAttributeDefinition_ {
        pub default_grouping_value: Option<crate::value::ExpString>,
        pub grouping_name: crate::value::ExpString,
        pub grouping_source_keys: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationsignals_GroupingConfiguration_GroupingAttributeDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApplicationSignals::GroupingConfiguration.GroupingAttributeDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationsignals_GroupingConfiguration_GroupingAttributeDefinition as GroupingAttributeDefinition;
    impl crate::value::ToValue for GroupingAttributeDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.default_grouping_value {
                properties.insert(
                    "DefaultGroupingValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "GroupingName".to_string(),
                crate::value::ToValue::to_value(&self.grouping_name),
            );
            properties.insert(
                "GroupingSourceKeys".to_string(),
                crate::value::ToValue::to_value(&self.grouping_source_keys),
            );
            properties.into()
        }
    }
}
pub mod servicelevelobjective {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationsignals-servicelevelobjective-burnrateconfiguration.html
    pub struct BurnRateConfiguration_ {
        pub look_back_window_minutes: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationsignals_ServiceLevelObjective_BurnRateConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApplicationSignals::ServiceLevelObjective.BurnRateConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationsignals_ServiceLevelObjective_BurnRateConfiguration as BurnRateConfiguration;
    impl crate::value::ToValue for BurnRateConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LookBackWindowMinutes".to_string(),
                crate::value::ToValue::to_value(&self.look_back_window_minutes),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationsignals-servicelevelobjective-calendarinterval.html
    pub struct CalendarInterval_ {
        pub duration: i32,
        pub duration_unit: crate::value::ExpString,
        pub start_time: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationsignals_ServiceLevelObjective_CalendarInterval {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApplicationSignals::ServiceLevelObjective.CalendarInterval"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationsignals_ServiceLevelObjective_CalendarInterval as CalendarInterval;
    impl crate::value::ToValue for CalendarInterval_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Duration".to_string(),
                crate::value::ToValue::to_value(&self.duration),
            );
            properties.insert(
                "DurationUnit".to_string(),
                crate::value::ToValue::to_value(&self.duration_unit),
            );
            properties.insert(
                "StartTime".to_string(),
                crate::value::ToValue::to_value(&self.start_time),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationsignals-servicelevelobjective-dependencyconfig.html
    pub struct DependencyConfig_ {
        pub dependency_key_attributes: std::collections::BTreeMap<String, crate::value::ExpString>,
        pub dependency_operation_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationsignals_ServiceLevelObjective_DependencyConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApplicationSignals::ServiceLevelObjective.DependencyConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationsignals_ServiceLevelObjective_DependencyConfig as DependencyConfig;
    impl crate::value::ToValue for DependencyConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DependencyKeyAttributes".to_string(),
                crate::value::ToValue::to_value(&self.dependency_key_attributes),
            );
            properties.insert(
                "DependencyOperationName".to_string(),
                crate::value::ToValue::to_value(&self.dependency_operation_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationsignals-servicelevelobjective-dimension.html
    pub struct Dimension_ {
        pub name: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationsignals_ServiceLevelObjective_Dimension {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApplicationSignals::ServiceLevelObjective.Dimension"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationsignals_ServiceLevelObjective_Dimension as Dimension;
    impl crate::value::ToValue for Dimension_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationsignals-servicelevelobjective-exclusionwindow.html
    pub struct ExclusionWindow_ {
        pub reason: Option<crate::value::ExpString>,
        pub recurrence_rule: Option<Box<RecurrenceRule_>>,
        pub start_time: Option<crate::value::ExpString>,
        pub window: Box<Window_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationsignals_ServiceLevelObjective_ExclusionWindow {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApplicationSignals::ServiceLevelObjective.ExclusionWindow"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationsignals_ServiceLevelObjective_ExclusionWindow as ExclusionWindow;
    impl crate::value::ToValue for ExclusionWindow_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.reason {
                properties.insert("Reason".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.recurrence_rule {
                properties.insert(
                    "RecurrenceRule".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.start_time {
                properties.insert(
                    "StartTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Window".to_string(),
                crate::value::ToValue::to_value(&self.window),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationsignals-servicelevelobjective-goal.html
    pub struct Goal_ {
        pub attainment_goal: Option<f64>,
        pub interval: Option<Box<Interval_>>,
        pub warning_threshold: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationsignals_ServiceLevelObjective_Goal {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApplicationSignals::ServiceLevelObjective.Goal"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationsignals_ServiceLevelObjective_Goal as Goal;
    impl crate::value::ToValue for Goal_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attainment_goal {
                properties.insert(
                    "AttainmentGoal".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.interval {
                properties.insert(
                    "Interval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.warning_threshold {
                properties.insert(
                    "WarningThreshold".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationsignals-servicelevelobjective-interval.html
    pub struct Interval_ {
        pub calendar_interval: Option<Box<CalendarInterval_>>,
        pub rolling_interval: Option<Box<RollingInterval_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationsignals_ServiceLevelObjective_Interval {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApplicationSignals::ServiceLevelObjective.Interval"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationsignals_ServiceLevelObjective_Interval as Interval;
    impl crate::value::ToValue for Interval_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.calendar_interval {
                properties.insert(
                    "CalendarInterval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rolling_interval {
                properties.insert(
                    "RollingInterval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationsignals-servicelevelobjective-metric.html
    pub struct Metric_ {
        pub dimensions: Option<Vec<Dimension_>>,
        pub metric_name: Option<crate::value::ExpString>,
        pub namespace: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationsignals_ServiceLevelObjective_Metric {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApplicationSignals::ServiceLevelObjective.Metric"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationsignals_ServiceLevelObjective_Metric as Metric;
    impl crate::value::ToValue for Metric_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationsignals-servicelevelobjective-metricdataquery.html
    pub struct MetricDataQuery_ {
        pub account_id: Option<crate::value::ExpString>,
        pub expression: Option<crate::value::ExpString>,
        pub id: crate::value::ExpString,
        pub metric_stat: Option<Box<MetricStat_>>,
        pub return_data: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationsignals_ServiceLevelObjective_MetricDataQuery {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApplicationSignals::ServiceLevelObjective.MetricDataQuery"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationsignals_ServiceLevelObjective_MetricDataQuery as MetricDataQuery;
    impl crate::value::ToValue for MetricDataQuery_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.account_id {
                properties.insert(
                    "AccountId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.expression {
                properties.insert(
                    "Expression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationsignals-servicelevelobjective-metricstat.html
    pub struct MetricStat_ {
        pub metric: Box<Metric_>,
        pub period: i32,
        pub stat: crate::value::ExpString,
        pub unit: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationsignals_ServiceLevelObjective_MetricStat {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApplicationSignals::ServiceLevelObjective.MetricStat"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationsignals_ServiceLevelObjective_MetricStat as MetricStat;
    impl crate::value::ToValue for MetricStat_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Metric".to_string(),
                crate::value::ToValue::to_value(&self.metric),
            );
            properties.insert(
                "Period".to_string(),
                crate::value::ToValue::to_value(&self.period),
            );
            properties.insert(
                "Stat".to_string(),
                crate::value::ToValue::to_value(&self.stat),
            );
            if let Some(ref value) = self.unit {
                properties.insert("Unit".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationsignals-servicelevelobjective-monitoredrequestcountmetric.html
    pub struct MonitoredRequestCountMetric_ {
        pub bad_count_metric: Option<Vec<MetricDataQuery_>>,
        pub good_count_metric: Option<Vec<MetricDataQuery_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationsignals_ServiceLevelObjective_MonitoredRequestCountMetric {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApplicationSignals::ServiceLevelObjective.MonitoredRequestCountMetric"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationsignals_ServiceLevelObjective_MonitoredRequestCountMetric as MonitoredRequestCountMetric;
    impl crate::value::ToValue for MonitoredRequestCountMetric_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bad_count_metric {
                properties.insert(
                    "BadCountMetric".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.good_count_metric {
                properties.insert(
                    "GoodCountMetric".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationsignals-servicelevelobjective-recurrencerule.html
    pub struct RecurrenceRule_ {
        pub expression: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationsignals_ServiceLevelObjective_RecurrenceRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApplicationSignals::ServiceLevelObjective.RecurrenceRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationsignals_ServiceLevelObjective_RecurrenceRule as RecurrenceRule;
    impl crate::value::ToValue for RecurrenceRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Expression".to_string(),
                crate::value::ToValue::to_value(&self.expression),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationsignals-servicelevelobjective-requestbasedsli.html
    pub struct RequestBasedSli_ {
        pub comparison_operator: Option<crate::value::ExpString>,
        pub metric_threshold: Option<f64>,
        pub request_based_sli_metric: Box<RequestBasedSliMetric_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationsignals_ServiceLevelObjective_RequestBasedSli {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApplicationSignals::ServiceLevelObjective.RequestBasedSli"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationsignals_ServiceLevelObjective_RequestBasedSli as RequestBasedSli;
    impl crate::value::ToValue for RequestBasedSli_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.comparison_operator {
                properties.insert(
                    "ComparisonOperator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metric_threshold {
                properties.insert(
                    "MetricThreshold".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RequestBasedSliMetric".to_string(),
                crate::value::ToValue::to_value(&self.request_based_sli_metric),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationsignals-servicelevelobjective-requestbasedslimetric.html
    pub struct RequestBasedSliMetric_ {
        pub dependency_config: Option<Box<DependencyConfig_>>,
        pub key_attributes: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub metric_type: Option<crate::value::ExpString>,
        pub monitored_request_count_metric: Option<Box<MonitoredRequestCountMetric_>>,
        pub operation_name: Option<crate::value::ExpString>,
        pub total_request_count_metric: Option<Vec<MetricDataQuery_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationsignals_ServiceLevelObjective_RequestBasedSliMetric {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApplicationSignals::ServiceLevelObjective.RequestBasedSliMetric"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationsignals_ServiceLevelObjective_RequestBasedSliMetric as RequestBasedSliMetric;
    impl crate::value::ToValue for RequestBasedSliMetric_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dependency_config {
                properties.insert(
                    "DependencyConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.key_attributes {
                properties.insert(
                    "KeyAttributes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metric_type {
                properties.insert(
                    "MetricType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.monitored_request_count_metric {
                properties.insert(
                    "MonitoredRequestCountMetric".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.operation_name {
                properties.insert(
                    "OperationName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.total_request_count_metric {
                properties.insert(
                    "TotalRequestCountMetric".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationsignals-servicelevelobjective-rollinginterval.html
    pub struct RollingInterval_ {
        pub duration: i32,
        pub duration_unit: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationsignals_ServiceLevelObjective_RollingInterval {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApplicationSignals::ServiceLevelObjective.RollingInterval"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationsignals_ServiceLevelObjective_RollingInterval as RollingInterval;
    impl crate::value::ToValue for RollingInterval_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Duration".to_string(),
                crate::value::ToValue::to_value(&self.duration),
            );
            properties.insert(
                "DurationUnit".to_string(),
                crate::value::ToValue::to_value(&self.duration_unit),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationsignals-servicelevelobjective-sli.html
    pub struct Sli_ {
        pub comparison_operator: crate::value::ExpString,
        pub metric_threshold: f64,
        pub sli_metric: Box<SliMetric_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationsignals_ServiceLevelObjective_Sli {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApplicationSignals::ServiceLevelObjective.Sli"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationsignals_ServiceLevelObjective_Sli as Sli;
    impl crate::value::ToValue for Sli_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ComparisonOperator".to_string(),
                crate::value::ToValue::to_value(&self.comparison_operator),
            );
            properties.insert(
                "MetricThreshold".to_string(),
                crate::value::ToValue::to_value(&self.metric_threshold),
            );
            properties.insert(
                "SliMetric".to_string(),
                crate::value::ToValue::to_value(&self.sli_metric),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationsignals-servicelevelobjective-slimetric.html
    pub struct SliMetric_ {
        pub dependency_config: Option<Box<DependencyConfig_>>,
        pub key_attributes: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub metric_data_queries: Option<Vec<MetricDataQuery_>>,
        pub metric_type: Option<crate::value::ExpString>,
        pub operation_name: Option<crate::value::ExpString>,
        pub period_seconds: Option<i32>,
        pub statistic: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationsignals_ServiceLevelObjective_SliMetric {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApplicationSignals::ServiceLevelObjective.SliMetric"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationsignals_ServiceLevelObjective_SliMetric as SliMetric;
    impl crate::value::ToValue for SliMetric_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dependency_config {
                properties.insert(
                    "DependencyConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.key_attributes {
                properties.insert(
                    "KeyAttributes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metric_data_queries {
                properties.insert(
                    "MetricDataQueries".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metric_type {
                properties.insert(
                    "MetricType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.operation_name {
                properties.insert(
                    "OperationName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.period_seconds {
                properties.insert(
                    "PeriodSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.statistic {
                properties.insert(
                    "Statistic".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationsignals-servicelevelobjective-window.html
    pub struct Window_ {
        pub duration: i32,
        pub duration_unit: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationsignals_ServiceLevelObjective_Window {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApplicationSignals::ServiceLevelObjective.Window"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationsignals_ServiceLevelObjective_Window as Window;
    impl crate::value::ToValue for Window_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Duration".to_string(),
                crate::value::ToValue::to_value(&self.duration),
            );
            properties.insert(
                "DurationUnit".to_string(),
                crate::value::ToValue::to_value(&self.duration_unit),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationsignals-discovery.html
pub struct Discovery_ {}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_applicationsignals_Discovery {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ApplicationSignals::Discovery"
        $($field $value)*)
    };
}
pub use crate::__aws_applicationsignals_Discovery as Discovery;
impl crate::template::ToResource for Discovery_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApplicationSignals"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Discovery"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        crate::template::ResourceProperties::new()
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationsignals-groupingconfiguration.html
pub struct GroupingConfiguration_ {
    pub grouping_attribute_definitions:
        Vec<super::applicationsignals::groupingconfiguration::GroupingAttributeDefinition_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_applicationsignals_GroupingConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ApplicationSignals::GroupingConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_applicationsignals_GroupingConfiguration as GroupingConfiguration;
impl crate::template::ToResource for GroupingConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApplicationSignals"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("GroupingConfiguration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "GroupingAttributeDefinitions".to_string(),
            crate::value::ToValue::to_value(&self.grouping_attribute_definitions),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationsignals-servicelevelobjective.html
pub struct ServiceLevelObjective_ {
    pub burn_rate_configurations:
        Option<Vec<super::applicationsignals::servicelevelobjective::BurnRateConfiguration_>>,
    pub description: Option<crate::value::ExpString>,
    pub exclusion_windows:
        Option<Vec<super::applicationsignals::servicelevelobjective::ExclusionWindow_>>,
    pub goal: Option<super::applicationsignals::servicelevelobjective::Goal_>,
    pub name: crate::value::ExpString,
    pub request_based_sli:
        Option<super::applicationsignals::servicelevelobjective::RequestBasedSli_>,
    pub sli: Option<super::applicationsignals::servicelevelobjective::Sli_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_applicationsignals_ServiceLevelObjective {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ApplicationSignals::ServiceLevelObjective"
        $($field $value)*)
    };
}
pub use crate::__aws_applicationsignals_ServiceLevelObjective as ServiceLevelObjective;
impl crate::template::ToResource for ServiceLevelObjective_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApplicationSignals"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ServiceLevelObjective"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.burn_rate_configurations {
            properties.insert(
                "BurnRateConfigurations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.exclusion_windows {
            properties.insert(
                "ExclusionWindows".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.goal {
            properties.insert("Goal".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.request_based_sli {
            properties.insert(
                "RequestBasedSli".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sli {
            properties.insert("Sli".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
