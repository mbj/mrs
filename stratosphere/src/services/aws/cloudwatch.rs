pub mod alarm {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-alarm-dimension.html>
    pub struct Dimension_ {
        pub name: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudwatch_Alarm_Dimension {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudWatch::Alarm.Dimension"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudwatch_Alarm_Dimension as Dimension;
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-alarm-metric.html>
    pub struct Metric_ {
        pub dimensions: Option<Vec<Dimension_>>,
        pub metric_name: Option<crate::value::ExpString>,
        pub namespace: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudwatch_Alarm_Metric {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudWatch::Alarm.Metric"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudwatch_Alarm_Metric as Metric;
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-alarm-metricdataquery.html>
    pub struct MetricDataQuery_ {
        pub account_id: Option<crate::value::ExpString>,
        pub expression: Option<crate::value::ExpString>,
        pub id: crate::value::ExpString,
        pub label: Option<crate::value::ExpString>,
        pub metric_stat: Option<Box<MetricStat_>>,
        pub period: Option<i32>,
        pub return_data: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudwatch_Alarm_MetricDataQuery {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudWatch::Alarm.MetricDataQuery"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudwatch_Alarm_MetricDataQuery as MetricDataQuery;
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
            if let Some(ref value) = self.label {
                properties.insert("Label".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.metric_stat {
                properties.insert(
                    "MetricStat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.period {
                properties.insert("Period".to_string(), crate::value::ToValue::to_value(value));
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-alarm-metricstat.html>
    pub struct MetricStat_ {
        pub metric: Box<Metric_>,
        pub period: i32,
        pub stat: crate::value::ExpString,
        pub unit: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudwatch_Alarm_MetricStat {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudWatch::Alarm.MetricStat"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudwatch_Alarm_MetricStat as MetricStat;
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
}
pub mod anomalydetector {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-anomalydetector-configuration.html>
    pub struct Configuration_ {
        pub excluded_time_ranges: Option<Vec<Range_>>,
        pub metric_time_zone: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudwatch_AnomalyDetector_Configuration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudWatch::AnomalyDetector.Configuration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudwatch_AnomalyDetector_Configuration as Configuration;
    impl crate::value::ToValue for Configuration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.excluded_time_ranges {
                properties.insert(
                    "ExcludedTimeRanges".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metric_time_zone {
                properties.insert(
                    "MetricTimeZone".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-anomalydetector-dimension.html>
    pub struct Dimension_ {
        pub name: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudwatch_AnomalyDetector_Dimension {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudWatch::AnomalyDetector.Dimension"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudwatch_AnomalyDetector_Dimension as Dimension;
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-anomalydetector-metric.html>
    pub struct Metric_ {
        pub dimensions: Option<Vec<Dimension_>>,
        pub metric_name: crate::value::ExpString,
        pub namespace: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudwatch_AnomalyDetector_Metric {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudWatch::AnomalyDetector.Metric"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudwatch_AnomalyDetector_Metric as Metric;
    impl crate::value::ToValue for Metric_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dimensions {
                properties.insert(
                    "Dimensions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MetricName".to_string(),
                crate::value::ToValue::to_value(&self.metric_name),
            );
            properties.insert(
                "Namespace".to_string(),
                crate::value::ToValue::to_value(&self.namespace),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-anomalydetector-metriccharacteristics.html>
    pub struct MetricCharacteristics_ {
        pub periodic_spikes: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudwatch_AnomalyDetector_MetricCharacteristics {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudWatch::AnomalyDetector.MetricCharacteristics"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudwatch_AnomalyDetector_MetricCharacteristics as MetricCharacteristics;
    impl crate::value::ToValue for MetricCharacteristics_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.periodic_spikes {
                properties.insert(
                    "PeriodicSpikes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-anomalydetector-metricdataqueries.html>
    pub struct MetricDataQueries_ {}
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudwatch_AnomalyDetector_MetricDataQueries {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudWatch::AnomalyDetector.MetricDataQueries"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudwatch_AnomalyDetector_MetricDataQueries as MetricDataQueries;
    impl crate::value::ToValue for MetricDataQueries_ {
        fn to_value(&self) -> serde_json::Value {
            serde_json::Value::Object(serde_json::Map::new())
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-anomalydetector-metricdataquery.html>
    pub struct MetricDataQuery_ {
        pub account_id: Option<crate::value::ExpString>,
        pub expression: Option<crate::value::ExpString>,
        pub id: crate::value::ExpString,
        pub label: Option<crate::value::ExpString>,
        pub metric_stat: Option<Box<MetricStat_>>,
        pub period: Option<i32>,
        pub return_data: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudwatch_AnomalyDetector_MetricDataQuery {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudWatch::AnomalyDetector.MetricDataQuery"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudwatch_AnomalyDetector_MetricDataQuery as MetricDataQuery;
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
            if let Some(ref value) = self.label {
                properties.insert("Label".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.metric_stat {
                properties.insert(
                    "MetricStat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.period {
                properties.insert("Period".to_string(), crate::value::ToValue::to_value(value));
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-anomalydetector-metricmathanomalydetector.html>
    pub struct MetricMathAnomalyDetector_ {
        pub metric_data_queries: Option<Vec<MetricDataQuery_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudwatch_AnomalyDetector_MetricMathAnomalyDetector {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudWatch::AnomalyDetector.MetricMathAnomalyDetector"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudwatch_AnomalyDetector_MetricMathAnomalyDetector as MetricMathAnomalyDetector;
    impl crate::value::ToValue for MetricMathAnomalyDetector_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.metric_data_queries {
                properties.insert(
                    "MetricDataQueries".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-anomalydetector-metricstat.html>
    pub struct MetricStat_ {
        pub metric: Box<Metric_>,
        pub period: i32,
        pub stat: crate::value::ExpString,
        pub unit: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudwatch_AnomalyDetector_MetricStat {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudWatch::AnomalyDetector.MetricStat"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudwatch_AnomalyDetector_MetricStat as MetricStat;
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-anomalydetector-range.html>
    pub struct Range_ {
        pub end_time: crate::value::ExpString,
        pub start_time: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudwatch_AnomalyDetector_Range {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudWatch::AnomalyDetector.Range"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudwatch_AnomalyDetector_Range as Range;
    impl crate::value::ToValue for Range_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EndTime".to_string(),
                crate::value::ToValue::to_value(&self.end_time),
            );
            properties.insert(
                "StartTime".to_string(),
                crate::value::ToValue::to_value(&self.start_time),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-anomalydetector-singlemetricanomalydetector.html>
    pub struct SingleMetricAnomalyDetector_ {
        pub account_id: Option<crate::value::ExpString>,
        pub dimensions: Option<Vec<Dimension_>>,
        pub metric_name: Option<crate::value::ExpString>,
        pub namespace: Option<crate::value::ExpString>,
        pub stat: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudwatch_AnomalyDetector_SingleMetricAnomalyDetector {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudWatch::AnomalyDetector.SingleMetricAnomalyDetector"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudwatch_AnomalyDetector_SingleMetricAnomalyDetector as SingleMetricAnomalyDetector;
    impl crate::value::ToValue for SingleMetricAnomalyDetector_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.account_id {
                properties.insert(
                    "AccountId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
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
            if let Some(ref value) = self.stat {
                properties.insert("Stat".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod insightrule {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-insightrule-tags.html>
    pub struct Tags_ {}
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudwatch_InsightRule_Tags {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudWatch::InsightRule.Tags"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudwatch_InsightRule_Tags as Tags;
    impl crate::value::ToValue for Tags_ {
        fn to_value(&self) -> serde_json::Value {
            serde_json::Value::Object(serde_json::Map::new())
        }
    }
}
pub mod metricstream {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-metricstream-metricstreamfilter.html>
    pub struct MetricStreamFilter_ {
        pub metric_names: Option<Vec<crate::value::ExpString>>,
        pub namespace: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudwatch_MetricStream_MetricStreamFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudWatch::MetricStream.MetricStreamFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudwatch_MetricStream_MetricStreamFilter as MetricStreamFilter;
    impl crate::value::ToValue for MetricStreamFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.metric_names {
                properties.insert(
                    "MetricNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Namespace".to_string(),
                crate::value::ToValue::to_value(&self.namespace),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-metricstream-metricstreamstatisticsconfiguration.html>
    pub struct MetricStreamStatisticsConfiguration_ {
        pub additional_statistics: Vec<crate::value::ExpString>,
        pub include_metrics: Vec<MetricStreamStatisticsMetric_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudwatch_MetricStream_MetricStreamStatisticsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudWatch::MetricStream.MetricStreamStatisticsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudwatch_MetricStream_MetricStreamStatisticsConfiguration as MetricStreamStatisticsConfiguration;
    impl crate::value::ToValue for MetricStreamStatisticsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AdditionalStatistics".to_string(),
                crate::value::ToValue::to_value(&self.additional_statistics),
            );
            properties.insert(
                "IncludeMetrics".to_string(),
                crate::value::ToValue::to_value(&self.include_metrics),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudwatch-metricstream-metricstreamstatisticsmetric.html>
    pub struct MetricStreamStatisticsMetric_ {
        pub metric_name: crate::value::ExpString,
        pub namespace: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudwatch_MetricStream_MetricStreamStatisticsMetric {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudWatch::MetricStream.MetricStreamStatisticsMetric"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudwatch_MetricStream_MetricStreamStatisticsMetric as MetricStreamStatisticsMetric;
    impl crate::value::ToValue for MetricStreamStatisticsMetric_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MetricName".to_string(),
                crate::value::ToValue::to_value(&self.metric_name),
            );
            properties.insert(
                "Namespace".to_string(),
                crate::value::ToValue::to_value(&self.namespace),
            );
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-alarm.html>
pub struct Alarm_ {
    pub actions_enabled: Option<crate::value::ExpBool>,
    pub alarm_actions: Option<Vec<crate::value::ExpString>>,
    pub alarm_description: Option<crate::value::ExpString>,
    pub alarm_name: Option<crate::value::ExpString>,
    pub comparison_operator: crate::value::ExpString,
    pub datapoints_to_alarm: Option<i32>,
    pub dimensions: Option<Vec<super::cloudwatch::alarm::Dimension_>>,
    pub evaluate_low_sample_count_percentile: Option<crate::value::ExpString>,
    pub evaluation_periods: i32,
    pub extended_statistic: Option<crate::value::ExpString>,
    pub insufficient_data_actions: Option<Vec<crate::value::ExpString>>,
    pub metric_name: Option<crate::value::ExpString>,
    pub metrics: Option<Vec<super::cloudwatch::alarm::MetricDataQuery_>>,
    pub namespace: Option<crate::value::ExpString>,
    pub ok_actions: Option<Vec<crate::value::ExpString>>,
    pub period: Option<i32>,
    pub statistic: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub threshold: Option<f64>,
    pub threshold_metric_id: Option<crate::value::ExpString>,
    pub treat_missing_data: Option<crate::value::ExpString>,
    pub unit: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudwatch_Alarm {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CloudWatch::Alarm"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudwatch_Alarm as Alarm;
impl crate::template::ToResource for Alarm_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudWatch"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Alarm"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.actions_enabled {
            properties.insert(
                "ActionsEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.alarm_actions {
            properties.insert(
                "AlarmActions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.alarm_description {
            properties.insert(
                "AlarmDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.alarm_name {
            properties.insert(
                "AlarmName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ComparisonOperator".to_string(),
            crate::value::ToValue::to_value(&self.comparison_operator),
        );
        if let Some(ref value) = self.datapoints_to_alarm {
            properties.insert(
                "DatapointsToAlarm".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.dimensions {
            properties.insert(
                "Dimensions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.evaluate_low_sample_count_percentile {
            properties.insert(
                "EvaluateLowSampleCountPercentile".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "EvaluationPeriods".to_string(),
            crate::value::ToValue::to_value(&self.evaluation_periods),
        );
        if let Some(ref value) = self.extended_statistic {
            properties.insert(
                "ExtendedStatistic".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.insufficient_data_actions {
            properties.insert(
                "InsufficientDataActions".to_string(),
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
        if let Some(ref value) = self.ok_actions {
            properties.insert(
                "OKActions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.period {
            properties.insert("Period".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.statistic {
            properties.insert(
                "Statistic".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.threshold {
            properties.insert(
                "Threshold".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.threshold_metric_id {
            properties.insert(
                "ThresholdMetricId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.treat_missing_data {
            properties.insert(
                "TreatMissingData".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.unit {
            properties.insert("Unit".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-anomalydetector.html>
pub struct AnomalyDetector_ {
    pub configuration: Option<super::cloudwatch::anomalydetector::Configuration_>,
    pub dimensions: Option<Vec<super::cloudwatch::anomalydetector::Dimension_>>,
    pub metric_characteristics: Option<super::cloudwatch::anomalydetector::MetricCharacteristics_>,
    pub metric_math_anomaly_detector:
        Option<super::cloudwatch::anomalydetector::MetricMathAnomalyDetector_>,
    pub metric_name: Option<crate::value::ExpString>,
    pub namespace: Option<crate::value::ExpString>,
    pub single_metric_anomaly_detector:
        Option<super::cloudwatch::anomalydetector::SingleMetricAnomalyDetector_>,
    pub stat: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudwatch_AnomalyDetector {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CloudWatch::AnomalyDetector"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudwatch_AnomalyDetector as AnomalyDetector;
impl crate::template::ToResource for AnomalyDetector_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudWatch"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AnomalyDetector"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.configuration {
            properties.insert(
                "Configuration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.dimensions {
            properties.insert(
                "Dimensions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.metric_characteristics {
            properties.insert(
                "MetricCharacteristics".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.metric_math_anomaly_detector {
            properties.insert(
                "MetricMathAnomalyDetector".to_string(),
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
        if let Some(ref value) = self.single_metric_anomaly_detector {
            properties.insert(
                "SingleMetricAnomalyDetector".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.stat {
            properties.insert("Stat".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-compositealarm.html>
pub struct CompositeAlarm_ {
    pub actions_enabled: Option<crate::value::ExpBool>,
    pub actions_suppressor: Option<crate::value::ExpString>,
    pub actions_suppressor_extension_period: Option<i32>,
    pub actions_suppressor_wait_period: Option<i32>,
    pub alarm_actions: Option<Vec<crate::value::ExpString>>,
    pub alarm_description: Option<crate::value::ExpString>,
    pub alarm_name: Option<crate::value::ExpString>,
    pub alarm_rule: crate::value::ExpString,
    pub insufficient_data_actions: Option<Vec<crate::value::ExpString>>,
    pub ok_actions: Option<Vec<crate::value::ExpString>>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudwatch_CompositeAlarm {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CloudWatch::CompositeAlarm"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudwatch_CompositeAlarm as CompositeAlarm;
impl crate::template::ToResource for CompositeAlarm_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudWatch"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("CompositeAlarm"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.actions_enabled {
            properties.insert(
                "ActionsEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.actions_suppressor {
            properties.insert(
                "ActionsSuppressor".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.actions_suppressor_extension_period {
            properties.insert(
                "ActionsSuppressorExtensionPeriod".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.actions_suppressor_wait_period {
            properties.insert(
                "ActionsSuppressorWaitPeriod".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.alarm_actions {
            properties.insert(
                "AlarmActions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.alarm_description {
            properties.insert(
                "AlarmDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.alarm_name {
            properties.insert(
                "AlarmName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "AlarmRule".to_string(),
            crate::value::ToValue::to_value(&self.alarm_rule),
        );
        if let Some(ref value) = self.insufficient_data_actions {
            properties.insert(
                "InsufficientDataActions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ok_actions {
            properties.insert(
                "OKActions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-dashboard.html>
pub struct Dashboard_ {
    pub dashboard_body: crate::value::ExpString,
    pub dashboard_name: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudwatch_Dashboard {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CloudWatch::Dashboard"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudwatch_Dashboard as Dashboard;
impl crate::template::ToResource for Dashboard_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudWatch"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Dashboard"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DashboardBody".to_string(),
            crate::value::ToValue::to_value(&self.dashboard_body),
        );
        if let Some(ref value) = self.dashboard_name {
            properties.insert(
                "DashboardName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-insightrule.html>
pub struct InsightRule_ {
    pub apply_on_transformed_logs: Option<crate::value::ExpBool>,
    pub rule_body: crate::value::ExpString,
    pub rule_name: crate::value::ExpString,
    pub rule_state: crate::value::ExpString,
    pub tags: Option<super::cloudwatch::insightrule::Tags_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudwatch_InsightRule {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CloudWatch::InsightRule"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudwatch_InsightRule as InsightRule;
impl crate::template::ToResource for InsightRule_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudWatch"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("InsightRule"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.apply_on_transformed_logs {
            properties.insert(
                "ApplyOnTransformedLogs".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RuleBody".to_string(),
            crate::value::ToValue::to_value(&self.rule_body),
        );
        properties.insert(
            "RuleName".to_string(),
            crate::value::ToValue::to_value(&self.rule_name),
        );
        properties.insert(
            "RuleState".to_string(),
            crate::value::ToValue::to_value(&self.rule_state),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudwatch-metricstream.html>
pub struct MetricStream_ {
    pub exclude_filters: Option<Vec<super::cloudwatch::metricstream::MetricStreamFilter_>>,
    pub firehose_arn: crate::value::ExpString,
    pub include_filters: Option<Vec<super::cloudwatch::metricstream::MetricStreamFilter_>>,
    pub include_linked_accounts_metrics: Option<crate::value::ExpBool>,
    pub name: Option<crate::value::ExpString>,
    pub output_format: crate::value::ExpString,
    pub role_arn: crate::value::ExpString,
    pub statistics_configurations:
        Option<Vec<super::cloudwatch::metricstream::MetricStreamStatisticsConfiguration_>>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudwatch_MetricStream {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CloudWatch::MetricStream"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudwatch_MetricStream as MetricStream;
impl crate::template::ToResource for MetricStream_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudWatch"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("MetricStream"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.exclude_filters {
            properties.insert(
                "ExcludeFilters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "FirehoseArn".to_string(),
            crate::value::ToValue::to_value(&self.firehose_arn),
        );
        if let Some(ref value) = self.include_filters {
            properties.insert(
                "IncludeFilters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.include_linked_accounts_metrics {
            properties.insert(
                "IncludeLinkedAccountsMetrics".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "OutputFormat".to_string(),
            crate::value::ToValue::to_value(&self.output_format),
        );
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        if let Some(ref value) = self.statistics_configurations {
            properties.insert(
                "StatisticsConfigurations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
