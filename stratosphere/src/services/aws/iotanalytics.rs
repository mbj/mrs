pub mod channel {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-channel-channelstorage.html>
    pub struct ChannelStorage_ {
        pub customer_managed_s3: Option<Box<CustomerManagedS3_>>,
        pub service_managed_s3: Option<serde_json::Value>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Channel_ChannelStorage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Channel.ChannelStorage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Channel_ChannelStorage as ChannelStorage;
    impl crate::value::ToValue for ChannelStorage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.customer_managed_s3 {
                properties.insert(
                    "CustomerManagedS3".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.service_managed_s3 {
                properties.insert(
                    "ServiceManagedS3".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-channel-customermanageds3.html>
    pub struct CustomerManagedS3_ {
        pub bucket: crate::value::ExpString,
        pub key_prefix: Option<crate::value::ExpString>,
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Channel_CustomerManagedS3 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Channel.CustomerManagedS3"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Channel_CustomerManagedS3 as CustomerManagedS3;
    impl crate::value::ToValue for CustomerManagedS3_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Bucket".to_string(),
                crate::value::ToValue::to_value(&self.bucket),
            );
            if let Some(ref value) = self.key_prefix {
                properties.insert(
                    "KeyPrefix".to_string(),
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-channel-retentionperiod.html>
    pub struct RetentionPeriod_ {
        pub number_of_days: Option<i32>,
        pub unlimited: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Channel_RetentionPeriod {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Channel.RetentionPeriod"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Channel_RetentionPeriod as RetentionPeriod;
    impl crate::value::ToValue for RetentionPeriod_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.number_of_days {
                properties.insert(
                    "NumberOfDays".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.unlimited {
                properties.insert(
                    "Unlimited".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod dataset {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-action.html>
    pub struct Action_ {
        pub action_name: crate::value::ExpString,
        pub container_action: Option<Box<ContainerAction_>>,
        pub query_action: Option<Box<QueryAction_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Dataset_Action {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Dataset.Action"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Dataset_Action as Action;
    impl crate::value::ToValue for Action_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ActionName".to_string(),
                crate::value::ToValue::to_value(&self.action_name),
            );
            if let Some(ref value) = self.container_action {
                properties.insert(
                    "ContainerAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.query_action {
                properties.insert(
                    "QueryAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-containeraction.html>
    pub struct ContainerAction_ {
        pub execution_role_arn: crate::value::ExpString,
        pub image: crate::value::ExpString,
        pub resource_configuration: Box<ResourceConfiguration_>,
        pub variables: Option<Vec<Variable_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Dataset_ContainerAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Dataset.ContainerAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Dataset_ContainerAction as ContainerAction;
    impl crate::value::ToValue for ContainerAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ExecutionRoleArn".to_string(),
                crate::value::ToValue::to_value(&self.execution_role_arn),
            );
            properties.insert(
                "Image".to_string(),
                crate::value::ToValue::to_value(&self.image),
            );
            properties.insert(
                "ResourceConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.resource_configuration),
            );
            if let Some(ref value) = self.variables {
                properties.insert(
                    "Variables".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-datasetcontentdeliveryrule.html>
    pub struct DatasetContentDeliveryRule_ {
        pub destination: Box<DatasetContentDeliveryRuleDestination_>,
        pub entry_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Dataset_DatasetContentDeliveryRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Dataset.DatasetContentDeliveryRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Dataset_DatasetContentDeliveryRule as DatasetContentDeliveryRule;
    impl crate::value::ToValue for DatasetContentDeliveryRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Destination".to_string(),
                crate::value::ToValue::to_value(&self.destination),
            );
            if let Some(ref value) = self.entry_name {
                properties.insert(
                    "EntryName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-datasetcontentdeliveryruledestination.html>
    pub struct DatasetContentDeliveryRuleDestination_ {
        pub iot_events_destination_configuration: Option<Box<IotEventsDestinationConfiguration_>>,
        pub s3_destination_configuration: Option<Box<S3DestinationConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Dataset_DatasetContentDeliveryRuleDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Dataset.DatasetContentDeliveryRuleDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Dataset_DatasetContentDeliveryRuleDestination as DatasetContentDeliveryRuleDestination;
    impl crate::value::ToValue for DatasetContentDeliveryRuleDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.iot_events_destination_configuration {
                properties.insert(
                    "IotEventsDestinationConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_destination_configuration {
                properties.insert(
                    "S3DestinationConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-datasetcontentversionvalue.html>
    pub struct DatasetContentVersionValue_ {
        pub dataset_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Dataset_DatasetContentVersionValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Dataset.DatasetContentVersionValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Dataset_DatasetContentVersionValue as DatasetContentVersionValue;
    impl crate::value::ToValue for DatasetContentVersionValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DatasetName".to_string(),
                crate::value::ToValue::to_value(&self.dataset_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-deltatime.html>
    pub struct DeltaTime_ {
        pub offset_seconds: i32,
        pub time_expression: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Dataset_DeltaTime {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Dataset.DeltaTime"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Dataset_DeltaTime as DeltaTime;
    impl crate::value::ToValue for DeltaTime_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "OffsetSeconds".to_string(),
                crate::value::ToValue::to_value(&self.offset_seconds),
            );
            properties.insert(
                "TimeExpression".to_string(),
                crate::value::ToValue::to_value(&self.time_expression),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-deltatimesessionwindowconfiguration.html>
    pub struct DeltaTimeSessionWindowConfiguration_ {
        pub timeout_in_minutes: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Dataset_DeltaTimeSessionWindowConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Dataset.DeltaTimeSessionWindowConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Dataset_DeltaTimeSessionWindowConfiguration as DeltaTimeSessionWindowConfiguration;
    impl crate::value::ToValue for DeltaTimeSessionWindowConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TimeoutInMinutes".to_string(),
                crate::value::ToValue::to_value(&self.timeout_in_minutes),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-filter.html>
    pub struct Filter_ {
        pub delta_time: Option<Box<DeltaTime_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Dataset_Filter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Dataset.Filter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Dataset_Filter as Filter;
    impl crate::value::ToValue for Filter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.delta_time {
                properties.insert(
                    "DeltaTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-glueconfiguration.html>
    pub struct GlueConfiguration_ {
        pub database_name: crate::value::ExpString,
        pub table_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Dataset_GlueConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Dataset.GlueConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Dataset_GlueConfiguration as GlueConfiguration;
    impl crate::value::ToValue for GlueConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DatabaseName".to_string(),
                crate::value::ToValue::to_value(&self.database_name),
            );
            properties.insert(
                "TableName".to_string(),
                crate::value::ToValue::to_value(&self.table_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-ioteventsdestinationconfiguration.html>
    pub struct IotEventsDestinationConfiguration_ {
        pub input_name: crate::value::ExpString,
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Dataset_IotEventsDestinationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Dataset.IotEventsDestinationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Dataset_IotEventsDestinationConfiguration as IotEventsDestinationConfiguration;
    impl crate::value::ToValue for IotEventsDestinationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InputName".to_string(),
                crate::value::ToValue::to_value(&self.input_name),
            );
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-latedatarule.html>
    pub struct LateDataRule_ {
        pub rule_configuration: Box<LateDataRuleConfiguration_>,
        pub rule_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Dataset_LateDataRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Dataset.LateDataRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Dataset_LateDataRule as LateDataRule;
    impl crate::value::ToValue for LateDataRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RuleConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.rule_configuration),
            );
            if let Some(ref value) = self.rule_name {
                properties.insert(
                    "RuleName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-latedataruleconfiguration.html>
    pub struct LateDataRuleConfiguration_ {
        pub delta_time_session_window_configuration:
            Option<Box<DeltaTimeSessionWindowConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Dataset_LateDataRuleConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Dataset.LateDataRuleConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Dataset_LateDataRuleConfiguration as LateDataRuleConfiguration;
    impl crate::value::ToValue for LateDataRuleConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.delta_time_session_window_configuration {
                properties.insert(
                    "DeltaTimeSessionWindowConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-outputfileurivalue.html>
    pub struct OutputFileUriValue_ {
        pub file_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Dataset_OutputFileUriValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Dataset.OutputFileUriValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Dataset_OutputFileUriValue as OutputFileUriValue;
    impl crate::value::ToValue for OutputFileUriValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FileName".to_string(),
                crate::value::ToValue::to_value(&self.file_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-queryaction.html>
    pub struct QueryAction_ {
        pub filters: Option<Vec<Filter_>>,
        pub sql_query: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Dataset_QueryAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Dataset.QueryAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Dataset_QueryAction as QueryAction;
    impl crate::value::ToValue for QueryAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.filters {
                properties.insert(
                    "Filters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SqlQuery".to_string(),
                crate::value::ToValue::to_value(&self.sql_query),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-resourceconfiguration.html>
    pub struct ResourceConfiguration_ {
        pub compute_type: crate::value::ExpString,
        pub volume_size_in_gb: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Dataset_ResourceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Dataset.ResourceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Dataset_ResourceConfiguration as ResourceConfiguration;
    impl crate::value::ToValue for ResourceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ComputeType".to_string(),
                crate::value::ToValue::to_value(&self.compute_type),
            );
            properties.insert(
                "VolumeSizeInGB".to_string(),
                crate::value::ToValue::to_value(&self.volume_size_in_gb),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-retentionperiod.html>
    pub struct RetentionPeriod_ {
        pub number_of_days: Option<i32>,
        pub unlimited: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Dataset_RetentionPeriod {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Dataset.RetentionPeriod"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Dataset_RetentionPeriod as RetentionPeriod;
    impl crate::value::ToValue for RetentionPeriod_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.number_of_days {
                properties.insert(
                    "NumberOfDays".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.unlimited {
                properties.insert(
                    "Unlimited".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-s3destinationconfiguration.html>
    pub struct S3DestinationConfiguration_ {
        pub bucket: crate::value::ExpString,
        pub glue_configuration: Option<Box<GlueConfiguration_>>,
        pub key: crate::value::ExpString,
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Dataset_S3DestinationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Dataset.S3DestinationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Dataset_S3DestinationConfiguration as S3DestinationConfiguration;
    impl crate::value::ToValue for S3DestinationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Bucket".to_string(),
                crate::value::ToValue::to_value(&self.bucket),
            );
            if let Some(ref value) = self.glue_configuration {
                properties.insert(
                    "GlueConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-schedule.html>
    pub struct Schedule_ {
        pub schedule_expression: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Dataset_Schedule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Dataset.Schedule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Dataset_Schedule as Schedule;
    impl crate::value::ToValue for Schedule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ScheduleExpression".to_string(),
                crate::value::ToValue::to_value(&self.schedule_expression),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-trigger.html>
    pub struct Trigger_ {
        pub schedule: Option<Box<Schedule_>>,
        pub triggering_dataset: Option<Box<TriggeringDataset_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Dataset_Trigger {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Dataset.Trigger"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Dataset_Trigger as Trigger;
    impl crate::value::ToValue for Trigger_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.schedule {
                properties.insert(
                    "Schedule".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.triggering_dataset {
                properties.insert(
                    "TriggeringDataset".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-triggeringdataset.html>
    pub struct TriggeringDataset_ {
        pub dataset_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Dataset_TriggeringDataset {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Dataset.TriggeringDataset"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Dataset_TriggeringDataset as TriggeringDataset;
    impl crate::value::ToValue for TriggeringDataset_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DatasetName".to_string(),
                crate::value::ToValue::to_value(&self.dataset_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-variable.html>
    pub struct Variable_ {
        pub dataset_content_version_value: Option<Box<DatasetContentVersionValue_>>,
        pub double_value: Option<f64>,
        pub output_file_uri_value: Option<Box<OutputFileUriValue_>>,
        pub string_value: Option<crate::value::ExpString>,
        pub variable_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Dataset_Variable {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Dataset.Variable"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Dataset_Variable as Variable;
    impl crate::value::ToValue for Variable_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dataset_content_version_value {
                properties.insert(
                    "DatasetContentVersionValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.double_value {
                properties.insert(
                    "DoubleValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.output_file_uri_value {
                properties.insert(
                    "OutputFileUriValue".to_string(),
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
                "VariableName".to_string(),
                crate::value::ToValue::to_value(&self.variable_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-dataset-versioningconfiguration.html>
    pub struct VersioningConfiguration_ {
        pub max_versions: Option<i32>,
        pub unlimited: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Dataset_VersioningConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Dataset.VersioningConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Dataset_VersioningConfiguration as VersioningConfiguration;
    impl crate::value::ToValue for VersioningConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max_versions {
                properties.insert(
                    "MaxVersions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.unlimited {
                properties.insert(
                    "Unlimited".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod datastore {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-column.html>
    pub struct Column_ {
        pub name: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Datastore_Column {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Datastore.Column"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Datastore_Column as Column;
    impl crate::value::ToValue for Column_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-customermanageds3.html>
    pub struct CustomerManagedS3_ {
        pub bucket: crate::value::ExpString,
        pub key_prefix: Option<crate::value::ExpString>,
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Datastore_CustomerManagedS3 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Datastore.CustomerManagedS3"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Datastore_CustomerManagedS3 as CustomerManagedS3;
    impl crate::value::ToValue for CustomerManagedS3_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Bucket".to_string(),
                crate::value::ToValue::to_value(&self.bucket),
            );
            if let Some(ref value) = self.key_prefix {
                properties.insert(
                    "KeyPrefix".to_string(),
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-customermanageds3storage.html>
    pub struct CustomerManagedS3Storage_ {
        pub bucket: crate::value::ExpString,
        pub key_prefix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Datastore_CustomerManagedS3Storage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Datastore.CustomerManagedS3Storage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Datastore_CustomerManagedS3Storage as CustomerManagedS3Storage;
    impl crate::value::ToValue for CustomerManagedS3Storage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Bucket".to_string(),
                crate::value::ToValue::to_value(&self.bucket),
            );
            if let Some(ref value) = self.key_prefix {
                properties.insert(
                    "KeyPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-datastorepartition.html>
    pub struct DatastorePartition_ {
        pub partition: Option<Box<Partition_>>,
        pub timestamp_partition: Option<Box<TimestampPartition_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Datastore_DatastorePartition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Datastore.DatastorePartition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Datastore_DatastorePartition as DatastorePartition;
    impl crate::value::ToValue for DatastorePartition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.partition {
                properties.insert(
                    "Partition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timestamp_partition {
                properties.insert(
                    "TimestampPartition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-datastorepartitions.html>
    pub struct DatastorePartitions_ {
        pub partitions: Option<Vec<DatastorePartition_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Datastore_DatastorePartitions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Datastore.DatastorePartitions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Datastore_DatastorePartitions as DatastorePartitions;
    impl crate::value::ToValue for DatastorePartitions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.partitions {
                properties.insert(
                    "Partitions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-datastorestorage.html>
    pub struct DatastoreStorage_ {
        pub customer_managed_s3: Option<Box<CustomerManagedS3_>>,
        pub iot_site_wise_multi_layer_storage: Option<Box<IotSiteWiseMultiLayerStorage_>>,
        pub service_managed_s3: Option<serde_json::Value>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Datastore_DatastoreStorage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Datastore.DatastoreStorage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Datastore_DatastoreStorage as DatastoreStorage;
    impl crate::value::ToValue for DatastoreStorage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.customer_managed_s3 {
                properties.insert(
                    "CustomerManagedS3".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.iot_site_wise_multi_layer_storage {
                properties.insert(
                    "IotSiteWiseMultiLayerStorage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.service_managed_s3 {
                properties.insert(
                    "ServiceManagedS3".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-fileformatconfiguration.html>
    pub struct FileFormatConfiguration_ {
        pub json_configuration: Option<serde_json::Value>,
        pub parquet_configuration: Option<Box<ParquetConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Datastore_FileFormatConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Datastore.FileFormatConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Datastore_FileFormatConfiguration as FileFormatConfiguration;
    impl crate::value::ToValue for FileFormatConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.json_configuration {
                properties.insert(
                    "JsonConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parquet_configuration {
                properties.insert(
                    "ParquetConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-iotsitewisemultilayerstorage.html>
    pub struct IotSiteWiseMultiLayerStorage_ {
        pub customer_managed_s3_storage: Option<Box<CustomerManagedS3Storage_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Datastore_IotSiteWiseMultiLayerStorage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Datastore.IotSiteWiseMultiLayerStorage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Datastore_IotSiteWiseMultiLayerStorage as IotSiteWiseMultiLayerStorage;
    impl crate::value::ToValue for IotSiteWiseMultiLayerStorage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.customer_managed_s3_storage {
                properties.insert(
                    "CustomerManagedS3Storage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-parquetconfiguration.html>
    pub struct ParquetConfiguration_ {
        pub schema_definition: Option<Box<SchemaDefinition_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Datastore_ParquetConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Datastore.ParquetConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Datastore_ParquetConfiguration as ParquetConfiguration;
    impl crate::value::ToValue for ParquetConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.schema_definition {
                properties.insert(
                    "SchemaDefinition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-partition.html>
    pub struct Partition_ {
        pub attribute_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Datastore_Partition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Datastore.Partition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Datastore_Partition as Partition;
    impl crate::value::ToValue for Partition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AttributeName".to_string(),
                crate::value::ToValue::to_value(&self.attribute_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-retentionperiod.html>
    pub struct RetentionPeriod_ {
        pub number_of_days: Option<i32>,
        pub unlimited: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Datastore_RetentionPeriod {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Datastore.RetentionPeriod"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Datastore_RetentionPeriod as RetentionPeriod;
    impl crate::value::ToValue for RetentionPeriod_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.number_of_days {
                properties.insert(
                    "NumberOfDays".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.unlimited {
                properties.insert(
                    "Unlimited".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-schemadefinition.html>
    pub struct SchemaDefinition_ {
        pub columns: Option<Vec<Column_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Datastore_SchemaDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Datastore.SchemaDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Datastore_SchemaDefinition as SchemaDefinition;
    impl crate::value::ToValue for SchemaDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.columns {
                properties.insert(
                    "Columns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-datastore-timestamppartition.html>
    pub struct TimestampPartition_ {
        pub attribute_name: crate::value::ExpString,
        pub timestamp_format: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Datastore_TimestampPartition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Datastore.TimestampPartition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Datastore_TimestampPartition as TimestampPartition;
    impl crate::value::ToValue for TimestampPartition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AttributeName".to_string(),
                crate::value::ToValue::to_value(&self.attribute_name),
            );
            if let Some(ref value) = self.timestamp_format {
                properties.insert(
                    "TimestampFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod pipeline {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-activity.html>
    pub struct Activity_ {
        pub add_attributes: Option<Box<AddAttributes_>>,
        pub channel: Option<Box<Channel_>>,
        pub datastore: Option<Box<Datastore_>>,
        pub device_registry_enrich: Option<Box<DeviceRegistryEnrich_>>,
        pub device_shadow_enrich: Option<Box<DeviceShadowEnrich_>>,
        pub filter: Option<Box<Filter_>>,
        pub lambda: Option<Box<Lambda_>>,
        pub math: Option<Box<Math_>>,
        pub remove_attributes: Option<Box<RemoveAttributes_>>,
        pub select_attributes: Option<Box<SelectAttributes_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Pipeline_Activity {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Pipeline.Activity"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Pipeline_Activity as Activity;
    impl crate::value::ToValue for Activity_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.add_attributes {
                properties.insert(
                    "AddAttributes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.channel {
                properties.insert(
                    "Channel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.datastore {
                properties.insert(
                    "Datastore".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.device_registry_enrich {
                properties.insert(
                    "DeviceRegistryEnrich".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.device_shadow_enrich {
                properties.insert(
                    "DeviceShadowEnrich".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.filter {
                properties.insert("Filter".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.lambda {
                properties.insert("Lambda".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.math {
                properties.insert("Math".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.remove_attributes {
                properties.insert(
                    "RemoveAttributes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.select_attributes {
                properties.insert(
                    "SelectAttributes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-addattributes.html>
    pub struct AddAttributes_ {
        pub attributes: std::collections::BTreeMap<String, crate::value::ExpString>,
        pub name: crate::value::ExpString,
        pub next: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Pipeline_AddAttributes {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Pipeline.AddAttributes"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Pipeline_AddAttributes as AddAttributes;
    impl crate::value::ToValue for AddAttributes_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Attributes".to_string(),
                crate::value::ToValue::to_value(&self.attributes),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.next {
                properties.insert("Next".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-channel.html>
    pub struct Channel_ {
        pub channel_name: crate::value::ExpString,
        pub name: crate::value::ExpString,
        pub next: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Pipeline_Channel {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Pipeline.Channel"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Pipeline_Channel as Channel;
    impl crate::value::ToValue for Channel_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ChannelName".to_string(),
                crate::value::ToValue::to_value(&self.channel_name),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.next {
                properties.insert("Next".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-datastore.html>
    pub struct Datastore_ {
        pub datastore_name: crate::value::ExpString,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Pipeline_Datastore {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Pipeline.Datastore"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Pipeline_Datastore as Datastore;
    impl crate::value::ToValue for Datastore_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DatastoreName".to_string(),
                crate::value::ToValue::to_value(&self.datastore_name),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-deviceregistryenrich.html>
    pub struct DeviceRegistryEnrich_ {
        pub attribute: crate::value::ExpString,
        pub name: crate::value::ExpString,
        pub next: Option<crate::value::ExpString>,
        pub role_arn: crate::value::ExpString,
        pub thing_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Pipeline_DeviceRegistryEnrich {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Pipeline.DeviceRegistryEnrich"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Pipeline_DeviceRegistryEnrich as DeviceRegistryEnrich;
    impl crate::value::ToValue for DeviceRegistryEnrich_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Attribute".to_string(),
                crate::value::ToValue::to_value(&self.attribute),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.next {
                properties.insert("Next".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.insert(
                "ThingName".to_string(),
                crate::value::ToValue::to_value(&self.thing_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-deviceshadowenrich.html>
    pub struct DeviceShadowEnrich_ {
        pub attribute: crate::value::ExpString,
        pub name: crate::value::ExpString,
        pub next: Option<crate::value::ExpString>,
        pub role_arn: crate::value::ExpString,
        pub thing_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Pipeline_DeviceShadowEnrich {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Pipeline.DeviceShadowEnrich"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Pipeline_DeviceShadowEnrich as DeviceShadowEnrich;
    impl crate::value::ToValue for DeviceShadowEnrich_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Attribute".to_string(),
                crate::value::ToValue::to_value(&self.attribute),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.next {
                properties.insert("Next".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.insert(
                "ThingName".to_string(),
                crate::value::ToValue::to_value(&self.thing_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-filter.html>
    pub struct Filter_ {
        pub filter: crate::value::ExpString,
        pub name: crate::value::ExpString,
        pub next: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Pipeline_Filter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Pipeline.Filter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Pipeline_Filter as Filter;
    impl crate::value::ToValue for Filter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Filter".to_string(),
                crate::value::ToValue::to_value(&self.filter),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.next {
                properties.insert("Next".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-lambda.html>
    pub struct Lambda_ {
        pub batch_size: i32,
        pub lambda_name: crate::value::ExpString,
        pub name: crate::value::ExpString,
        pub next: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Pipeline_Lambda {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Pipeline.Lambda"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Pipeline_Lambda as Lambda;
    impl crate::value::ToValue for Lambda_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BatchSize".to_string(),
                crate::value::ToValue::to_value(&self.batch_size),
            );
            properties.insert(
                "LambdaName".to_string(),
                crate::value::ToValue::to_value(&self.lambda_name),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.next {
                properties.insert("Next".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-math.html>
    pub struct Math_ {
        pub attribute: crate::value::ExpString,
        pub math: crate::value::ExpString,
        pub name: crate::value::ExpString,
        pub next: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Pipeline_Math {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Pipeline.Math"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Pipeline_Math as Math;
    impl crate::value::ToValue for Math_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Attribute".to_string(),
                crate::value::ToValue::to_value(&self.attribute),
            );
            properties.insert(
                "Math".to_string(),
                crate::value::ToValue::to_value(&self.math),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.next {
                properties.insert("Next".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-removeattributes.html>
    pub struct RemoveAttributes_ {
        pub attributes: Vec<crate::value::ExpString>,
        pub name: crate::value::ExpString,
        pub next: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Pipeline_RemoveAttributes {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Pipeline.RemoveAttributes"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Pipeline_RemoveAttributes as RemoveAttributes;
    impl crate::value::ToValue for RemoveAttributes_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Attributes".to_string(),
                crate::value::ToValue::to_value(&self.attributes),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.next {
                properties.insert("Next".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotanalytics-pipeline-selectattributes.html>
    pub struct SelectAttributes_ {
        pub attributes: Vec<crate::value::ExpString>,
        pub name: crate::value::ExpString,
        pub next: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotanalytics_Pipeline_SelectAttributes {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTAnalytics::Pipeline.SelectAttributes"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotanalytics_Pipeline_SelectAttributes as SelectAttributes;
    impl crate::value::ToValue for SelectAttributes_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Attributes".to_string(),
                crate::value::ToValue::to_value(&self.attributes),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.next {
                properties.insert("Next".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotanalytics-channel.html>
pub struct Channel_ {
    pub channel_name: Option<crate::value::ExpString>,
    pub channel_storage: Option<super::iotanalytics::channel::ChannelStorage_>,
    pub retention_period: Option<super::iotanalytics::channel::RetentionPeriod_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iotanalytics_Channel {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTAnalytics::Channel"
        $($field $value)*)
    };
}
pub use crate::__aws_iotanalytics_Channel as Channel;
impl crate::template::ToResource for Channel_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTAnalytics"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Channel"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.channel_name {
            properties.insert(
                "ChannelName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.channel_storage {
            properties.insert(
                "ChannelStorage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.retention_period {
            properties.insert(
                "RetentionPeriod".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotanalytics-dataset.html>
pub struct Dataset_ {
    pub actions: Vec<super::iotanalytics::dataset::Action_>,
    pub content_delivery_rules:
        Option<Vec<super::iotanalytics::dataset::DatasetContentDeliveryRule_>>,
    pub dataset_name: Option<crate::value::ExpString>,
    pub late_data_rules: Option<Vec<super::iotanalytics::dataset::LateDataRule_>>,
    pub retention_period: Option<super::iotanalytics::dataset::RetentionPeriod_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub triggers: Option<Vec<super::iotanalytics::dataset::Trigger_>>,
    pub versioning_configuration: Option<super::iotanalytics::dataset::VersioningConfiguration_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iotanalytics_Dataset {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTAnalytics::Dataset"
        $($field $value)*)
    };
}
pub use crate::__aws_iotanalytics_Dataset as Dataset;
impl crate::template::ToResource for Dataset_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTAnalytics"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Dataset"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Actions".to_string(),
            crate::value::ToValue::to_value(&self.actions),
        );
        if let Some(ref value) = self.content_delivery_rules {
            properties.insert(
                "ContentDeliveryRules".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.dataset_name {
            properties.insert(
                "DatasetName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.late_data_rules {
            properties.insert(
                "LateDataRules".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.retention_period {
            properties.insert(
                "RetentionPeriod".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.triggers {
            properties.insert(
                "Triggers".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.versioning_configuration {
            properties.insert(
                "VersioningConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotanalytics-datastore.html>
pub struct Datastore_ {
    pub datastore_name: Option<crate::value::ExpString>,
    pub datastore_partitions: Option<super::iotanalytics::datastore::DatastorePartitions_>,
    pub datastore_storage: Option<super::iotanalytics::datastore::DatastoreStorage_>,
    pub file_format_configuration: Option<super::iotanalytics::datastore::FileFormatConfiguration_>,
    pub retention_period: Option<super::iotanalytics::datastore::RetentionPeriod_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iotanalytics_Datastore {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTAnalytics::Datastore"
        $($field $value)*)
    };
}
pub use crate::__aws_iotanalytics_Datastore as Datastore;
impl crate::template::ToResource for Datastore_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTAnalytics"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Datastore"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.datastore_name {
            properties.insert(
                "DatastoreName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.datastore_partitions {
            properties.insert(
                "DatastorePartitions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.datastore_storage {
            properties.insert(
                "DatastoreStorage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.file_format_configuration {
            properties.insert(
                "FileFormatConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.retention_period {
            properties.insert(
                "RetentionPeriod".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotanalytics-pipeline.html>
pub struct Pipeline_ {
    pub pipeline_activities: Vec<super::iotanalytics::pipeline::Activity_>,
    pub pipeline_name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iotanalytics_Pipeline {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTAnalytics::Pipeline"
        $($field $value)*)
    };
}
pub use crate::__aws_iotanalytics_Pipeline as Pipeline;
impl crate::template::ToResource for Pipeline_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTAnalytics"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Pipeline"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "PipelineActivities".to_string(),
            crate::value::ToValue::to_value(&self.pipeline_activities),
        );
        if let Some(ref value) = self.pipeline_name {
            properties.insert(
                "PipelineName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
