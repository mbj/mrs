pub mod deliverydestination {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-deliverydestination-destinationpolicy.html>
    pub struct DestinationPolicy_ {
        pub delivery_destination_name: Option<crate::value::ExpString>,
        pub delivery_destination_policy: Option<serde_json::Value>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_logs_DeliveryDestination_DestinationPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Logs::DeliveryDestination.DestinationPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_logs_DeliveryDestination_DestinationPolicy as DestinationPolicy;
    impl crate::value::ToValue for DestinationPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.delivery_destination_name {
                properties.insert(
                    "DeliveryDestinationName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.delivery_destination_policy {
                properties.insert(
                    "DeliveryDestinationPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod integration {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-integration-opensearchresourceconfig.html>
    pub struct OpenSearchResourceConfig_ {
        pub application_arn: Option<crate::value::ExpString>,
        pub dashboard_viewer_principals: Vec<crate::value::ExpString>,
        pub data_source_role_arn: crate::value::ExpString,
        pub kms_key_arn: Option<crate::value::ExpString>,
        pub retention_days: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_logs_Integration_OpenSearchResourceConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Logs::Integration.OpenSearchResourceConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_logs_Integration_OpenSearchResourceConfig as OpenSearchResourceConfig;
    impl crate::value::ToValue for OpenSearchResourceConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.application_arn {
                properties.insert(
                    "ApplicationARN".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DashboardViewerPrincipals".to_string(),
                crate::value::ToValue::to_value(&self.dashboard_viewer_principals),
            );
            properties.insert(
                "DataSourceRoleArn".to_string(),
                crate::value::ToValue::to_value(&self.data_source_role_arn),
            );
            if let Some(ref value) = self.kms_key_arn {
                properties.insert(
                    "KmsKeyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.retention_days {
                properties.insert(
                    "RetentionDays".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-integration-resourceconfig.html>
    pub struct ResourceConfig_ {
        pub open_search_resource_config: Option<Box<OpenSearchResourceConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_logs_Integration_ResourceConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Logs::Integration.ResourceConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_logs_Integration_ResourceConfig as ResourceConfig;
    impl crate::value::ToValue for ResourceConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.open_search_resource_config {
                properties.insert(
                    "OpenSearchResourceConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod metricfilter {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-metricfilter-dimension.html>
    pub struct Dimension_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_logs_MetricFilter_Dimension {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Logs::MetricFilter.Dimension"
            $($field $value)*)
        };
    }
    pub use crate::__aws_logs_MetricFilter_Dimension as Dimension;
    impl crate::value::ToValue for Dimension_ {
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-metricfilter-metrictransformation.html>
    pub struct MetricTransformation_ {
        pub default_value: Option<f64>,
        pub dimensions: Option<Vec<Dimension_>>,
        pub metric_name: crate::value::ExpString,
        pub metric_namespace: crate::value::ExpString,
        pub metric_value: crate::value::ExpString,
        pub unit: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_logs_MetricFilter_MetricTransformation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Logs::MetricFilter.MetricTransformation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_logs_MetricFilter_MetricTransformation as MetricTransformation;
    impl crate::value::ToValue for MetricTransformation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.default_value {
                properties.insert(
                    "DefaultValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
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
                "MetricNamespace".to_string(),
                crate::value::ToValue::to_value(&self.metric_namespace),
            );
            properties.insert(
                "MetricValue".to_string(),
                crate::value::ToValue::to_value(&self.metric_value),
            );
            if let Some(ref value) = self.unit {
                properties.insert("Unit".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod scheduledquery {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-scheduledquery-destinationconfiguration.html>
    pub struct DestinationConfiguration_ {
        pub s3_configuration: Option<Box<S3Configuration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_logs_ScheduledQuery_DestinationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Logs::ScheduledQuery.DestinationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_logs_ScheduledQuery_DestinationConfiguration as DestinationConfiguration;
    impl crate::value::ToValue for DestinationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.s3_configuration {
                properties.insert(
                    "S3Configuration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-scheduledquery-s3configuration.html>
    pub struct S3Configuration_ {
        pub destination_identifier: crate::value::ExpString,
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_logs_ScheduledQuery_S3Configuration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Logs::ScheduledQuery.S3Configuration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_logs_ScheduledQuery_S3Configuration as S3Configuration;
    impl crate::value::ToValue for S3Configuration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DestinationIdentifier".to_string(),
                crate::value::ToValue::to_value(&self.destination_identifier),
            );
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-scheduledquery-tagsitems.html>
    pub struct TagsItems_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_logs_ScheduledQuery_TagsItems {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Logs::ScheduledQuery.TagsItems"
            $($field $value)*)
        };
    }
    pub use crate::__aws_logs_ScheduledQuery_TagsItems as TagsItems;
    impl crate::value::ToValue for TagsItems_ {
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
}
pub mod transformer {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-transformer-addkeyentry.html>
    pub struct AddKeyEntry_ {
        pub key: crate::value::ExpString,
        pub overwrite_if_exists: Option<crate::value::ExpBool>,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_logs_Transformer_AddKeyEntry {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Logs::Transformer.AddKeyEntry"
            $($field $value)*)
        };
    }
    pub use crate::__aws_logs_Transformer_AddKeyEntry as AddKeyEntry;
    impl crate::value::ToValue for AddKeyEntry_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            if let Some(ref value) = self.overwrite_if_exists {
                properties.insert(
                    "OverwriteIfExists".to_string(),
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-transformer-addkeys.html>
    pub struct AddKeys_ {
        pub entries: Vec<AddKeyEntry_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_logs_Transformer_AddKeys {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Logs::Transformer.AddKeys"
            $($field $value)*)
        };
    }
    pub use crate::__aws_logs_Transformer_AddKeys as AddKeys;
    impl crate::value::ToValue for AddKeys_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Entries".to_string(),
                crate::value::ToValue::to_value(&self.entries),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-transformer-copyvalue.html>
    pub struct CopyValue_ {
        pub entries: Vec<CopyValueEntry_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_logs_Transformer_CopyValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Logs::Transformer.CopyValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_logs_Transformer_CopyValue as CopyValue;
    impl crate::value::ToValue for CopyValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Entries".to_string(),
                crate::value::ToValue::to_value(&self.entries),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-transformer-copyvalueentry.html>
    pub struct CopyValueEntry_ {
        pub overwrite_if_exists: Option<crate::value::ExpBool>,
        pub source: crate::value::ExpString,
        pub target: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_logs_Transformer_CopyValueEntry {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Logs::Transformer.CopyValueEntry"
            $($field $value)*)
        };
    }
    pub use crate::__aws_logs_Transformer_CopyValueEntry as CopyValueEntry;
    impl crate::value::ToValue for CopyValueEntry_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.overwrite_if_exists {
                properties.insert(
                    "OverwriteIfExists".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Source".to_string(),
                crate::value::ToValue::to_value(&self.source),
            );
            properties.insert(
                "Target".to_string(),
                crate::value::ToValue::to_value(&self.target),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-transformer-csv.html>
    pub struct Csv_ {
        pub columns: Option<Vec<crate::value::ExpString>>,
        pub delimiter: Option<crate::value::ExpString>,
        pub quote_character: Option<crate::value::ExpString>,
        pub source: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_logs_Transformer_Csv {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Logs::Transformer.Csv"
            $($field $value)*)
        };
    }
    pub use crate::__aws_logs_Transformer_Csv as Csv;
    impl crate::value::ToValue for Csv_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.columns {
                properties.insert(
                    "Columns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.delimiter {
                properties.insert(
                    "Delimiter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.quote_character {
                properties.insert(
                    "QuoteCharacter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source {
                properties.insert("Source".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-transformer-datetimeconverter.html>
    pub struct DateTimeConverter_ {
        pub locale: Option<crate::value::ExpString>,
        pub match_patterns: Vec<crate::value::ExpString>,
        pub source: crate::value::ExpString,
        pub source_timezone: Option<crate::value::ExpString>,
        pub target: crate::value::ExpString,
        pub target_format: Option<crate::value::ExpString>,
        pub target_timezone: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_logs_Transformer_DateTimeConverter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Logs::Transformer.DateTimeConverter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_logs_Transformer_DateTimeConverter as DateTimeConverter;
    impl crate::value::ToValue for DateTimeConverter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.locale {
                properties.insert("Locale".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "MatchPatterns".to_string(),
                crate::value::ToValue::to_value(&self.match_patterns),
            );
            properties.insert(
                "Source".to_string(),
                crate::value::ToValue::to_value(&self.source),
            );
            if let Some(ref value) = self.source_timezone {
                properties.insert(
                    "SourceTimezone".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Target".to_string(),
                crate::value::ToValue::to_value(&self.target),
            );
            if let Some(ref value) = self.target_format {
                properties.insert(
                    "TargetFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_timezone {
                properties.insert(
                    "TargetTimezone".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-transformer-deletekeys.html>
    pub struct DeleteKeys_ {
        pub with_keys: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_logs_Transformer_DeleteKeys {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Logs::Transformer.DeleteKeys"
            $($field $value)*)
        };
    }
    pub use crate::__aws_logs_Transformer_DeleteKeys as DeleteKeys;
    impl crate::value::ToValue for DeleteKeys_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "WithKeys".to_string(),
                crate::value::ToValue::to_value(&self.with_keys),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-transformer-grok.html>
    pub struct Grok_ {
        pub r#match: crate::value::ExpString,
        pub source: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_logs_Transformer_Grok {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Logs::Transformer.Grok"
            $($field $value)*)
        };
    }
    pub use crate::__aws_logs_Transformer_Grok as Grok;
    impl crate::value::ToValue for Grok_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Match".to_string(),
                crate::value::ToValue::to_value(&self.r#match),
            );
            if let Some(ref value) = self.source {
                properties.insert("Source".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-transformer-listtomap.html>
    pub struct ListToMap_ {
        pub flatten: Option<crate::value::ExpBool>,
        pub flattened_element: Option<crate::value::ExpString>,
        pub key: crate::value::ExpString,
        pub source: crate::value::ExpString,
        pub target: Option<crate::value::ExpString>,
        pub value_key: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_logs_Transformer_ListToMap {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Logs::Transformer.ListToMap"
            $($field $value)*)
        };
    }
    pub use crate::__aws_logs_Transformer_ListToMap as ListToMap;
    impl crate::value::ToValue for ListToMap_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.flatten {
                properties.insert(
                    "Flatten".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.flattened_element {
                properties.insert(
                    "FlattenedElement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            properties.insert(
                "Source".to_string(),
                crate::value::ToValue::to_value(&self.source),
            );
            if let Some(ref value) = self.target {
                properties.insert("Target".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value_key {
                properties.insert(
                    "ValueKey".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-transformer-lowercasestring.html>
    pub struct LowerCaseString_ {
        pub with_keys: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_logs_Transformer_LowerCaseString {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Logs::Transformer.LowerCaseString"
            $($field $value)*)
        };
    }
    pub use crate::__aws_logs_Transformer_LowerCaseString as LowerCaseString;
    impl crate::value::ToValue for LowerCaseString_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "WithKeys".to_string(),
                crate::value::ToValue::to_value(&self.with_keys),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-transformer-movekeyentry.html>
    pub struct MoveKeyEntry_ {
        pub overwrite_if_exists: Option<crate::value::ExpBool>,
        pub source: crate::value::ExpString,
        pub target: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_logs_Transformer_MoveKeyEntry {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Logs::Transformer.MoveKeyEntry"
            $($field $value)*)
        };
    }
    pub use crate::__aws_logs_Transformer_MoveKeyEntry as MoveKeyEntry;
    impl crate::value::ToValue for MoveKeyEntry_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.overwrite_if_exists {
                properties.insert(
                    "OverwriteIfExists".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Source".to_string(),
                crate::value::ToValue::to_value(&self.source),
            );
            properties.insert(
                "Target".to_string(),
                crate::value::ToValue::to_value(&self.target),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-transformer-movekeys.html>
    pub struct MoveKeys_ {
        pub entries: Vec<MoveKeyEntry_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_logs_Transformer_MoveKeys {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Logs::Transformer.MoveKeys"
            $($field $value)*)
        };
    }
    pub use crate::__aws_logs_Transformer_MoveKeys as MoveKeys;
    impl crate::value::ToValue for MoveKeys_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Entries".to_string(),
                crate::value::ToValue::to_value(&self.entries),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-transformer-parsecloudfront.html>
    pub struct ParseCloudfront_ {
        pub source: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_logs_Transformer_ParseCloudfront {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Logs::Transformer.ParseCloudfront"
            $($field $value)*)
        };
    }
    pub use crate::__aws_logs_Transformer_ParseCloudfront as ParseCloudfront;
    impl crate::value::ToValue for ParseCloudfront_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.source {
                properties.insert("Source".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-transformer-parsejson.html>
    pub struct ParseJSON_ {
        pub destination: Option<crate::value::ExpString>,
        pub source: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_logs_Transformer_ParseJSON {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Logs::Transformer.ParseJSON"
            $($field $value)*)
        };
    }
    pub use crate::__aws_logs_Transformer_ParseJSON as ParseJSON;
    impl crate::value::ToValue for ParseJSON_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.destination {
                properties.insert(
                    "Destination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source {
                properties.insert("Source".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-transformer-parsekeyvalue.html>
    pub struct ParseKeyValue_ {
        pub destination: Option<crate::value::ExpString>,
        pub field_delimiter: Option<crate::value::ExpString>,
        pub key_prefix: Option<crate::value::ExpString>,
        pub key_value_delimiter: Option<crate::value::ExpString>,
        pub non_match_value: Option<crate::value::ExpString>,
        pub overwrite_if_exists: Option<crate::value::ExpBool>,
        pub source: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_logs_Transformer_ParseKeyValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Logs::Transformer.ParseKeyValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_logs_Transformer_ParseKeyValue as ParseKeyValue;
    impl crate::value::ToValue for ParseKeyValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.destination {
                properties.insert(
                    "Destination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.field_delimiter {
                properties.insert(
                    "FieldDelimiter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.key_prefix {
                properties.insert(
                    "KeyPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.key_value_delimiter {
                properties.insert(
                    "KeyValueDelimiter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.non_match_value {
                properties.insert(
                    "NonMatchValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.overwrite_if_exists {
                properties.insert(
                    "OverwriteIfExists".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source {
                properties.insert("Source".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-transformer-parsepostgres.html>
    pub struct ParsePostgres_ {
        pub source: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_logs_Transformer_ParsePostgres {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Logs::Transformer.ParsePostgres"
            $($field $value)*)
        };
    }
    pub use crate::__aws_logs_Transformer_ParsePostgres as ParsePostgres;
    impl crate::value::ToValue for ParsePostgres_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.source {
                properties.insert("Source".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-transformer-parseroute53.html>
    pub struct ParseRoute53_ {
        pub source: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_logs_Transformer_ParseRoute53 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Logs::Transformer.ParseRoute53"
            $($field $value)*)
        };
    }
    pub use crate::__aws_logs_Transformer_ParseRoute53 as ParseRoute53;
    impl crate::value::ToValue for ParseRoute53_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.source {
                properties.insert("Source".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-transformer-parsetoocsf.html>
    pub struct ParseToOCSF_ {
        pub event_source: crate::value::ExpString,
        pub mapping_version: Option<crate::value::ExpString>,
        pub ocsf_version: crate::value::ExpString,
        pub source: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_logs_Transformer_ParseToOCSF {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Logs::Transformer.ParseToOCSF"
            $($field $value)*)
        };
    }
    pub use crate::__aws_logs_Transformer_ParseToOCSF as ParseToOCSF;
    impl crate::value::ToValue for ParseToOCSF_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EventSource".to_string(),
                crate::value::ToValue::to_value(&self.event_source),
            );
            if let Some(ref value) = self.mapping_version {
                properties.insert(
                    "MappingVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "OcsfVersion".to_string(),
                crate::value::ToValue::to_value(&self.ocsf_version),
            );
            if let Some(ref value) = self.source {
                properties.insert("Source".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-transformer-parsevpc.html>
    pub struct ParseVPC_ {
        pub source: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_logs_Transformer_ParseVPC {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Logs::Transformer.ParseVPC"
            $($field $value)*)
        };
    }
    pub use crate::__aws_logs_Transformer_ParseVPC as ParseVPC;
    impl crate::value::ToValue for ParseVPC_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.source {
                properties.insert("Source".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-transformer-parsewaf.html>
    pub struct ParseWAF_ {
        pub source: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_logs_Transformer_ParseWAF {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Logs::Transformer.ParseWAF"
            $($field $value)*)
        };
    }
    pub use crate::__aws_logs_Transformer_ParseWAF as ParseWAF;
    impl crate::value::ToValue for ParseWAF_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.source {
                properties.insert("Source".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-transformer-processor.html>
    pub struct Processor_ {
        pub add_keys: Option<Box<AddKeys_>>,
        pub copy_value: Option<Box<CopyValue_>>,
        pub csv: Option<Box<Csv_>>,
        pub date_time_converter: Option<Box<DateTimeConverter_>>,
        pub delete_keys: Option<Box<DeleteKeys_>>,
        pub grok: Option<Box<Grok_>>,
        pub list_to_map: Option<Box<ListToMap_>>,
        pub lower_case_string: Option<Box<LowerCaseString_>>,
        pub move_keys: Option<Box<MoveKeys_>>,
        pub parse_cloudfront: Option<Box<ParseCloudfront_>>,
        pub parse_json: Option<Box<ParseJSON_>>,
        pub parse_key_value: Option<Box<ParseKeyValue_>>,
        pub parse_postgres: Option<Box<ParsePostgres_>>,
        pub parse_route53: Option<Box<ParseRoute53_>>,
        pub parse_to_ocsf: Option<Box<ParseToOCSF_>>,
        pub parse_vpc: Option<Box<ParseVPC_>>,
        pub parse_waf: Option<Box<ParseWAF_>>,
        pub rename_keys: Option<Box<RenameKeys_>>,
        pub split_string: Option<Box<SplitString_>>,
        pub substitute_string: Option<Box<SubstituteString_>>,
        pub trim_string: Option<Box<TrimString_>>,
        pub type_converter: Option<Box<TypeConverter_>>,
        pub upper_case_string: Option<Box<UpperCaseString_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_logs_Transformer_Processor {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Logs::Transformer.Processor"
            $($field $value)*)
        };
    }
    pub use crate::__aws_logs_Transformer_Processor as Processor;
    impl crate::value::ToValue for Processor_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.add_keys {
                properties.insert(
                    "AddKeys".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.copy_value {
                properties.insert(
                    "CopyValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.csv {
                properties.insert("Csv".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.date_time_converter {
                properties.insert(
                    "DateTimeConverter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.delete_keys {
                properties.insert(
                    "DeleteKeys".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.grok {
                properties.insert("Grok".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.list_to_map {
                properties.insert(
                    "ListToMap".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lower_case_string {
                properties.insert(
                    "LowerCaseString".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.move_keys {
                properties.insert(
                    "MoveKeys".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parse_cloudfront {
                properties.insert(
                    "ParseCloudfront".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parse_json {
                properties.insert(
                    "ParseJSON".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parse_key_value {
                properties.insert(
                    "ParseKeyValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parse_postgres {
                properties.insert(
                    "ParsePostgres".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parse_route53 {
                properties.insert(
                    "ParseRoute53".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parse_to_ocsf {
                properties.insert(
                    "ParseToOCSF".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parse_vpc {
                properties.insert(
                    "ParseVPC".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parse_waf {
                properties.insert(
                    "ParseWAF".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rename_keys {
                properties.insert(
                    "RenameKeys".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.split_string {
                properties.insert(
                    "SplitString".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.substitute_string {
                properties.insert(
                    "SubstituteString".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.trim_string {
                properties.insert(
                    "TrimString".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.type_converter {
                properties.insert(
                    "TypeConverter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.upper_case_string {
                properties.insert(
                    "UpperCaseString".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-transformer-renamekeyentry.html>
    pub struct RenameKeyEntry_ {
        pub key: crate::value::ExpString,
        pub overwrite_if_exists: Option<crate::value::ExpBool>,
        pub rename_to: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_logs_Transformer_RenameKeyEntry {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Logs::Transformer.RenameKeyEntry"
            $($field $value)*)
        };
    }
    pub use crate::__aws_logs_Transformer_RenameKeyEntry as RenameKeyEntry;
    impl crate::value::ToValue for RenameKeyEntry_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            if let Some(ref value) = self.overwrite_if_exists {
                properties.insert(
                    "OverwriteIfExists".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RenameTo".to_string(),
                crate::value::ToValue::to_value(&self.rename_to),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-transformer-renamekeys.html>
    pub struct RenameKeys_ {
        pub entries: Vec<RenameKeyEntry_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_logs_Transformer_RenameKeys {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Logs::Transformer.RenameKeys"
            $($field $value)*)
        };
    }
    pub use crate::__aws_logs_Transformer_RenameKeys as RenameKeys;
    impl crate::value::ToValue for RenameKeys_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Entries".to_string(),
                crate::value::ToValue::to_value(&self.entries),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-transformer-splitstring.html>
    pub struct SplitString_ {
        pub entries: Vec<SplitStringEntry_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_logs_Transformer_SplitString {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Logs::Transformer.SplitString"
            $($field $value)*)
        };
    }
    pub use crate::__aws_logs_Transformer_SplitString as SplitString;
    impl crate::value::ToValue for SplitString_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Entries".to_string(),
                crate::value::ToValue::to_value(&self.entries),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-transformer-splitstringentry.html>
    pub struct SplitStringEntry_ {
        pub delimiter: crate::value::ExpString,
        pub source: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_logs_Transformer_SplitStringEntry {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Logs::Transformer.SplitStringEntry"
            $($field $value)*)
        };
    }
    pub use crate::__aws_logs_Transformer_SplitStringEntry as SplitStringEntry;
    impl crate::value::ToValue for SplitStringEntry_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Delimiter".to_string(),
                crate::value::ToValue::to_value(&self.delimiter),
            );
            properties.insert(
                "Source".to_string(),
                crate::value::ToValue::to_value(&self.source),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-transformer-substitutestring.html>
    pub struct SubstituteString_ {
        pub entries: Vec<SubstituteStringEntry_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_logs_Transformer_SubstituteString {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Logs::Transformer.SubstituteString"
            $($field $value)*)
        };
    }
    pub use crate::__aws_logs_Transformer_SubstituteString as SubstituteString;
    impl crate::value::ToValue for SubstituteString_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Entries".to_string(),
                crate::value::ToValue::to_value(&self.entries),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-transformer-substitutestringentry.html>
    pub struct SubstituteStringEntry_ {
        pub from: crate::value::ExpString,
        pub source: crate::value::ExpString,
        pub to: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_logs_Transformer_SubstituteStringEntry {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Logs::Transformer.SubstituteStringEntry"
            $($field $value)*)
        };
    }
    pub use crate::__aws_logs_Transformer_SubstituteStringEntry as SubstituteStringEntry;
    impl crate::value::ToValue for SubstituteStringEntry_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "From".to_string(),
                crate::value::ToValue::to_value(&self.from),
            );
            properties.insert(
                "Source".to_string(),
                crate::value::ToValue::to_value(&self.source),
            );
            properties.insert("To".to_string(), crate::value::ToValue::to_value(&self.to));
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-transformer-trimstring.html>
    pub struct TrimString_ {
        pub with_keys: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_logs_Transformer_TrimString {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Logs::Transformer.TrimString"
            $($field $value)*)
        };
    }
    pub use crate::__aws_logs_Transformer_TrimString as TrimString;
    impl crate::value::ToValue for TrimString_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "WithKeys".to_string(),
                crate::value::ToValue::to_value(&self.with_keys),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-transformer-typeconverter.html>
    pub struct TypeConverter_ {
        pub entries: Vec<TypeConverterEntry_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_logs_Transformer_TypeConverter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Logs::Transformer.TypeConverter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_logs_Transformer_TypeConverter as TypeConverter;
    impl crate::value::ToValue for TypeConverter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Entries".to_string(),
                crate::value::ToValue::to_value(&self.entries),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-transformer-typeconverterentry.html>
    pub struct TypeConverterEntry_ {
        pub key: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_logs_Transformer_TypeConverterEntry {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Logs::Transformer.TypeConverterEntry"
            $($field $value)*)
        };
    }
    pub use crate::__aws_logs_Transformer_TypeConverterEntry as TypeConverterEntry;
    impl crate::value::ToValue for TypeConverterEntry_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-transformer-uppercasestring.html>
    pub struct UpperCaseString_ {
        pub with_keys: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_logs_Transformer_UpperCaseString {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Logs::Transformer.UpperCaseString"
            $($field $value)*)
        };
    }
    pub use crate::__aws_logs_Transformer_UpperCaseString as UpperCaseString;
    impl crate::value::ToValue for UpperCaseString_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "WithKeys".to_string(),
                crate::value::ToValue::to_value(&self.with_keys),
            );
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-accountpolicy.html>
pub struct AccountPolicy_ {
    pub policy_document: crate::value::ExpString,
    pub policy_name: crate::value::ExpString,
    pub policy_type: crate::value::ExpString,
    pub scope: Option<crate::value::ExpString>,
    pub selection_criteria: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_logs_AccountPolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Logs::AccountPolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_logs_AccountPolicy as AccountPolicy;
impl crate::template::ToResource for AccountPolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Logs"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AccountPolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "PolicyDocument".to_string(),
            crate::value::ToValue::to_value(&self.policy_document),
        );
        properties.insert(
            "PolicyName".to_string(),
            crate::value::ToValue::to_value(&self.policy_name),
        );
        properties.insert(
            "PolicyType".to_string(),
            crate::value::ToValue::to_value(&self.policy_type),
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
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-delivery.html>
pub struct Delivery_ {
    pub delivery_destination_arn: crate::value::ExpString,
    pub delivery_source_name: crate::value::ExpString,
    pub field_delimiter: Option<crate::value::ExpString>,
    pub record_fields: Option<Vec<crate::value::ExpString>>,
    pub s3_enable_hive_compatible_path: Option<crate::value::ExpBool>,
    pub s3_suffix_path: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_logs_Delivery {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Logs::Delivery" $($field
        $value)*)
    };
}
pub use crate::__aws_logs_Delivery as Delivery;
impl crate::template::ToResource for Delivery_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Logs"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Delivery"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DeliveryDestinationArn".to_string(),
            crate::value::ToValue::to_value(&self.delivery_destination_arn),
        );
        properties.insert(
            "DeliverySourceName".to_string(),
            crate::value::ToValue::to_value(&self.delivery_source_name),
        );
        if let Some(ref value) = self.field_delimiter {
            properties.insert(
                "FieldDelimiter".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.record_fields {
            properties.insert(
                "RecordFields".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.s3_enable_hive_compatible_path {
            properties.insert(
                "S3EnableHiveCompatiblePath".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.s3_suffix_path {
            properties.insert(
                "S3SuffixPath".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-deliverydestination.html>
pub struct DeliveryDestination_ {
    pub delivery_destination_policy: Option<super::logs::deliverydestination::DestinationPolicy_>,
    pub delivery_destination_type: Option<crate::value::ExpString>,
    pub destination_resource_arn: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub output_format: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_logs_DeliveryDestination {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Logs::DeliveryDestination"
        $($field $value)*)
    };
}
pub use crate::__aws_logs_DeliveryDestination as DeliveryDestination;
impl crate::template::ToResource for DeliveryDestination_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Logs"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DeliveryDestination"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.delivery_destination_policy {
            properties.insert(
                "DeliveryDestinationPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.delivery_destination_type {
            properties.insert(
                "DeliveryDestinationType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.destination_resource_arn {
            properties.insert(
                "DestinationResourceArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.output_format {
            properties.insert(
                "OutputFormat".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-deliverysource.html>
pub struct DeliverySource_ {
    pub log_type: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub resource_arn: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_logs_DeliverySource {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Logs::DeliverySource"
        $($field $value)*)
    };
}
pub use crate::__aws_logs_DeliverySource as DeliverySource;
impl crate::template::ToResource for DeliverySource_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Logs"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DeliverySource"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.log_type {
            properties.insert(
                "LogType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.resource_arn {
            properties.insert(
                "ResourceArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-destination.html>
pub struct Destination_ {
    pub destination_name: crate::value::ExpString,
    pub destination_policy: Option<crate::value::ExpString>,
    pub role_arn: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub target_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_logs_Destination {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Logs::Destination"
        $($field $value)*)
    };
}
pub use crate::__aws_logs_Destination as Destination;
impl crate::template::ToResource for Destination_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Logs"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Destination"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DestinationName".to_string(),
            crate::value::ToValue::to_value(&self.destination_name),
        );
        if let Some(ref value) = self.destination_policy {
            properties.insert(
                "DestinationPolicy".to_string(),
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
        properties.insert(
            "TargetArn".to_string(),
            crate::value::ToValue::to_value(&self.target_arn),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-integration.html>
pub struct Integration_ {
    pub integration_name: crate::value::ExpString,
    pub integration_type: crate::value::ExpString,
    pub resource_config: super::logs::integration::ResourceConfig_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_logs_Integration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Logs::Integration"
        $($field $value)*)
    };
}
pub use crate::__aws_logs_Integration as Integration;
impl crate::template::ToResource for Integration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Logs"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Integration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "IntegrationName".to_string(),
            crate::value::ToValue::to_value(&self.integration_name),
        );
        properties.insert(
            "IntegrationType".to_string(),
            crate::value::ToValue::to_value(&self.integration_type),
        );
        properties.insert(
            "ResourceConfig".to_string(),
            crate::value::ToValue::to_value(&self.resource_config),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-loganomalydetector.html>
pub struct LogAnomalyDetector_ {
    pub account_id: Option<crate::value::ExpString>,
    pub anomaly_visibility_time: Option<f64>,
    pub detector_name: Option<crate::value::ExpString>,
    pub evaluation_frequency: Option<crate::value::ExpString>,
    pub filter_pattern: Option<crate::value::ExpString>,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub log_group_arn_list: Option<Vec<crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_logs_LogAnomalyDetector {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Logs::LogAnomalyDetector"
        $($field $value)*)
    };
}
pub use crate::__aws_logs_LogAnomalyDetector as LogAnomalyDetector;
impl crate::template::ToResource for LogAnomalyDetector_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Logs"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LogAnomalyDetector"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.account_id {
            properties.insert(
                "AccountId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.anomaly_visibility_time {
            properties.insert(
                "AnomalyVisibilityTime".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.detector_name {
            properties.insert(
                "DetectorName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.evaluation_frequency {
            properties.insert(
                "EvaluationFrequency".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.filter_pattern {
            properties.insert(
                "FilterPattern".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_id {
            properties.insert(
                "KmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.log_group_arn_list {
            properties.insert(
                "LogGroupArnList".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-loggroup.html>
pub struct LogGroup_ {
    pub data_protection_policy: Option<serde_json::Value>,
    pub deletion_protection_enabled: Option<crate::value::ExpBool>,
    pub field_index_policies: Option<Vec<serde_json::Value>>,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub log_group_class: Option<crate::value::ExpString>,
    pub log_group_name: Option<crate::value::ExpString>,
    pub resource_policy_document: Option<serde_json::Value>,
    pub retention_in_days: Option<i32>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_logs_LogGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Logs::LogGroup" $($field
        $value)*)
    };
}
pub use crate::__aws_logs_LogGroup as LogGroup;
impl crate::template::ToResource for LogGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Logs"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LogGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.data_protection_policy {
            properties.insert(
                "DataProtectionPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.deletion_protection_enabled {
            properties.insert(
                "DeletionProtectionEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.field_index_policies {
            properties.insert(
                "FieldIndexPolicies".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_id {
            properties.insert(
                "KmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.log_group_class {
            properties.insert(
                "LogGroupClass".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.log_group_name {
            properties.insert(
                "LogGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resource_policy_document {
            properties.insert(
                "ResourcePolicyDocument".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.retention_in_days {
            properties.insert(
                "RetentionInDays".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-logstream.html>
pub struct LogStream_ {
    pub log_group_name: crate::value::ExpString,
    pub log_stream_name: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_logs_LogStream {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Logs::LogStream" $($field
        $value)*)
    };
}
pub use crate::__aws_logs_LogStream as LogStream;
impl crate::template::ToResource for LogStream_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Logs"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LogStream"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "LogGroupName".to_string(),
            crate::value::ToValue::to_value(&self.log_group_name),
        );
        if let Some(ref value) = self.log_stream_name {
            properties.insert(
                "LogStreamName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-metricfilter.html>
pub struct MetricFilter_ {
    pub apply_on_transformed_logs: Option<crate::value::ExpBool>,
    pub emit_system_field_dimensions: Option<Vec<crate::value::ExpString>>,
    pub field_selection_criteria: Option<crate::value::ExpString>,
    pub filter_name: Option<crate::value::ExpString>,
    pub filter_pattern: crate::value::ExpString,
    pub log_group_name: crate::value::ExpString,
    pub metric_transformations: Vec<super::logs::metricfilter::MetricTransformation_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_logs_MetricFilter {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Logs::MetricFilter"
        $($field $value)*)
    };
}
pub use crate::__aws_logs_MetricFilter as MetricFilter;
impl crate::template::ToResource for MetricFilter_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Logs"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("MetricFilter"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.apply_on_transformed_logs {
            properties.insert(
                "ApplyOnTransformedLogs".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.emit_system_field_dimensions {
            properties.insert(
                "EmitSystemFieldDimensions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.field_selection_criteria {
            properties.insert(
                "FieldSelectionCriteria".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.filter_name {
            properties.insert(
                "FilterName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "FilterPattern".to_string(),
            crate::value::ToValue::to_value(&self.filter_pattern),
        );
        properties.insert(
            "LogGroupName".to_string(),
            crate::value::ToValue::to_value(&self.log_group_name),
        );
        properties.insert(
            "MetricTransformations".to_string(),
            crate::value::ToValue::to_value(&self.metric_transformations),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-querydefinition.html>
pub struct QueryDefinition_ {
    pub log_group_names: Option<Vec<crate::value::ExpString>>,
    pub name: crate::value::ExpString,
    pub query_language: Option<crate::value::ExpString>,
    pub query_string: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_logs_QueryDefinition {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Logs::QueryDefinition"
        $($field $value)*)
    };
}
pub use crate::__aws_logs_QueryDefinition as QueryDefinition;
impl crate::template::ToResource for QueryDefinition_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Logs"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("QueryDefinition"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.log_group_names {
            properties.insert(
                "LogGroupNames".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.query_language {
            properties.insert(
                "QueryLanguage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "QueryString".to_string(),
            crate::value::ToValue::to_value(&self.query_string),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-resourcepolicy.html>
pub struct ResourcePolicy_ {
    pub policy_document: crate::value::ExpString,
    pub policy_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_logs_ResourcePolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Logs::ResourcePolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_logs_ResourcePolicy as ResourcePolicy;
impl crate::template::ToResource for ResourcePolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Logs"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ResourcePolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "PolicyDocument".to_string(),
            crate::value::ToValue::to_value(&self.policy_document),
        );
        properties.insert(
            "PolicyName".to_string(),
            crate::value::ToValue::to_value(&self.policy_name),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-scheduledquery.html>
pub struct ScheduledQuery_ {
    pub description: Option<crate::value::ExpString>,
    pub destination_configuration: Option<super::logs::scheduledquery::DestinationConfiguration_>,
    pub execution_role_arn: crate::value::ExpString,
    pub log_group_identifiers: Option<Vec<crate::value::ExpString>>,
    pub name: crate::value::ExpString,
    pub query_language: crate::value::ExpString,
    pub query_string: crate::value::ExpString,
    pub schedule_end_time: Option<f64>,
    pub schedule_expression: crate::value::ExpString,
    pub schedule_start_time: Option<f64>,
    pub start_time_offset: Option<i32>,
    pub state: Option<crate::value::ExpString>,
    pub tags: Option<Vec<super::logs::scheduledquery::TagsItems_>>,
    pub timezone: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_logs_ScheduledQuery {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Logs::ScheduledQuery"
        $($field $value)*)
    };
}
pub use crate::__aws_logs_ScheduledQuery as ScheduledQuery;
impl crate::template::ToResource for ScheduledQuery_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Logs"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ScheduledQuery"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.destination_configuration {
            properties.insert(
                "DestinationConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ExecutionRoleArn".to_string(),
            crate::value::ToValue::to_value(&self.execution_role_arn),
        );
        if let Some(ref value) = self.log_group_identifiers {
            properties.insert(
                "LogGroupIdentifiers".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "QueryLanguage".to_string(),
            crate::value::ToValue::to_value(&self.query_language),
        );
        properties.insert(
            "QueryString".to_string(),
            crate::value::ToValue::to_value(&self.query_string),
        );
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
        if let Some(ref value) = self.schedule_start_time {
            properties.insert(
                "ScheduleStartTime".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.start_time_offset {
            properties.insert(
                "StartTimeOffset".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.state {
            properties.insert("State".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.timezone {
            properties.insert(
                "Timezone".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-subscriptionfilter.html>
pub struct SubscriptionFilter_ {
    pub apply_on_transformed_logs: Option<crate::value::ExpBool>,
    pub destination_arn: crate::value::ExpString,
    pub distribution: Option<crate::value::ExpString>,
    pub emit_system_fields: Option<Vec<crate::value::ExpString>>,
    pub field_selection_criteria: Option<crate::value::ExpString>,
    pub filter_name: Option<crate::value::ExpString>,
    pub filter_pattern: crate::value::ExpString,
    pub log_group_name: crate::value::ExpString,
    pub role_arn: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_logs_SubscriptionFilter {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Logs::SubscriptionFilter"
        $($field $value)*)
    };
}
pub use crate::__aws_logs_SubscriptionFilter as SubscriptionFilter;
impl crate::template::ToResource for SubscriptionFilter_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Logs"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SubscriptionFilter"),
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
            "DestinationArn".to_string(),
            crate::value::ToValue::to_value(&self.destination_arn),
        );
        if let Some(ref value) = self.distribution {
            properties.insert(
                "Distribution".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.emit_system_fields {
            properties.insert(
                "EmitSystemFields".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.field_selection_criteria {
            properties.insert(
                "FieldSelectionCriteria".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.filter_name {
            properties.insert(
                "FilterName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "FilterPattern".to_string(),
            crate::value::ToValue::to_value(&self.filter_pattern),
        );
        properties.insert(
            "LogGroupName".to_string(),
            crate::value::ToValue::to_value(&self.log_group_name),
        );
        if let Some(ref value) = self.role_arn {
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-transformer.html>
pub struct Transformer_ {
    pub log_group_identifier: crate::value::ExpString,
    pub transformer_config: Vec<super::logs::transformer::Processor_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_logs_Transformer {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Logs::Transformer"
        $($field $value)*)
    };
}
pub use crate::__aws_logs_Transformer as Transformer;
impl crate::template::ToResource for Transformer_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Logs"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Transformer"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "LogGroupIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.log_group_identifier),
        );
        properties.insert(
            "TransformerConfig".to_string(),
            crate::value::ToValue::to_value(&self.transformer_config),
        );
        properties
    }
}
