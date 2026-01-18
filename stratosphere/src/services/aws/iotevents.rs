pub mod alarmmodel {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-alarmmodel-acknowledgeflow.html
    pub struct AcknowledgeFlow_ {
        pub enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_AlarmModel_AcknowledgeFlow {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::AlarmModel.AcknowledgeFlow"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_AlarmModel_AcknowledgeFlow as AcknowledgeFlow;
    impl crate::value::ToValue for AcknowledgeFlow_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-alarmmodel-alarmaction.html
    pub struct AlarmAction_ {
        pub dynamo_db: Option<Box<DynamoDB_>>,
        pub dynamo_d_bv2: Option<Box<DynamoDBv2_>>,
        pub firehose: Option<Box<Firehose_>>,
        pub iot_events: Option<Box<IotEvents_>>,
        pub iot_site_wise: Option<Box<IotSiteWise_>>,
        pub iot_topic_publish: Option<Box<IotTopicPublish_>>,
        pub lambda: Option<Box<Lambda_>>,
        pub sns: Option<Box<Sns_>>,
        pub sqs: Option<Box<Sqs_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_AlarmModel_AlarmAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::AlarmModel.AlarmAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_AlarmModel_AlarmAction as AlarmAction;
    impl crate::value::ToValue for AlarmAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dynamo_db {
                properties.insert(
                    "DynamoDB".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dynamo_d_bv2 {
                properties.insert(
                    "DynamoDBv2".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.firehose {
                properties.insert(
                    "Firehose".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.iot_events {
                properties.insert(
                    "IotEvents".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.iot_site_wise {
                properties.insert(
                    "IotSiteWise".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.iot_topic_publish {
                properties.insert(
                    "IotTopicPublish".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lambda {
                properties.insert("Lambda".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.sns {
                properties.insert("Sns".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.sqs {
                properties.insert("Sqs".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-alarmmodel-alarmcapabilities.html
    pub struct AlarmCapabilities_ {
        pub acknowledge_flow: Option<Box<AcknowledgeFlow_>>,
        pub initialization_configuration: Option<Box<InitializationConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_AlarmModel_AlarmCapabilities {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::AlarmModel.AlarmCapabilities"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_AlarmModel_AlarmCapabilities as AlarmCapabilities;
    impl crate::value::ToValue for AlarmCapabilities_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.acknowledge_flow {
                properties.insert(
                    "AcknowledgeFlow".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.initialization_configuration {
                properties.insert(
                    "InitializationConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-alarmmodel-alarmeventactions.html
    pub struct AlarmEventActions_ {
        pub alarm_actions: Option<Vec<AlarmAction_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_AlarmModel_AlarmEventActions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::AlarmModel.AlarmEventActions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_AlarmModel_AlarmEventActions as AlarmEventActions;
    impl crate::value::ToValue for AlarmEventActions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.alarm_actions {
                properties.insert(
                    "AlarmActions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-alarmmodel-alarmrule.html
    pub struct AlarmRule_ {
        pub simple_rule: Option<Box<SimpleRule_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_AlarmModel_AlarmRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::AlarmModel.AlarmRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_AlarmModel_AlarmRule as AlarmRule;
    impl crate::value::ToValue for AlarmRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.simple_rule {
                properties.insert(
                    "SimpleRule".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-alarmmodel-assetpropertytimestamp.html
    pub struct AssetPropertyTimestamp_ {
        pub offset_in_nanos: Option<crate::value::ExpString>,
        pub time_in_seconds: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_AlarmModel_AssetPropertyTimestamp {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::AlarmModel.AssetPropertyTimestamp"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_AlarmModel_AssetPropertyTimestamp as AssetPropertyTimestamp;
    impl crate::value::ToValue for AssetPropertyTimestamp_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.offset_in_nanos {
                properties.insert(
                    "OffsetInNanos".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TimeInSeconds".to_string(),
                crate::value::ToValue::to_value(&self.time_in_seconds),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-alarmmodel-assetpropertyvalue.html
    pub struct AssetPropertyValue_ {
        pub quality: Option<crate::value::ExpString>,
        pub timestamp: Option<Box<AssetPropertyTimestamp_>>,
        pub value: Box<AssetPropertyVariant_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_AlarmModel_AssetPropertyValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::AlarmModel.AssetPropertyValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_AlarmModel_AssetPropertyValue as AssetPropertyValue;
    impl crate::value::ToValue for AssetPropertyValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.quality {
                properties.insert(
                    "Quality".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timestamp {
                properties.insert(
                    "Timestamp".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-alarmmodel-assetpropertyvariant.html
    pub struct AssetPropertyVariant_ {
        pub boolean_value: Option<crate::value::ExpString>,
        pub double_value: Option<crate::value::ExpString>,
        pub integer_value: Option<crate::value::ExpString>,
        pub string_value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_AlarmModel_AssetPropertyVariant {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::AlarmModel.AssetPropertyVariant"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_AlarmModel_AssetPropertyVariant as AssetPropertyVariant;
    impl crate::value::ToValue for AssetPropertyVariant_ {
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
            if let Some(ref value) = self.integer_value {
                properties.insert(
                    "IntegerValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.string_value {
                properties.insert(
                    "StringValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-alarmmodel-dynamodb.html
    pub struct DynamoDB_ {
        pub hash_key_field: crate::value::ExpString,
        pub hash_key_type: Option<crate::value::ExpString>,
        pub hash_key_value: crate::value::ExpString,
        pub operation: Option<crate::value::ExpString>,
        pub payload: Option<Box<Payload_>>,
        pub payload_field: Option<crate::value::ExpString>,
        pub range_key_field: Option<crate::value::ExpString>,
        pub range_key_type: Option<crate::value::ExpString>,
        pub range_key_value: Option<crate::value::ExpString>,
        pub table_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_AlarmModel_DynamoDB {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::AlarmModel.DynamoDB"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_AlarmModel_DynamoDB as DynamoDB;
    impl crate::value::ToValue for DynamoDB_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "HashKeyField".to_string(),
                crate::value::ToValue::to_value(&self.hash_key_field),
            );
            if let Some(ref value) = self.hash_key_type {
                properties.insert(
                    "HashKeyType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "HashKeyValue".to_string(),
                crate::value::ToValue::to_value(&self.hash_key_value),
            );
            if let Some(ref value) = self.operation {
                properties.insert(
                    "Operation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.payload {
                properties.insert(
                    "Payload".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.payload_field {
                properties.insert(
                    "PayloadField".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.range_key_field {
                properties.insert(
                    "RangeKeyField".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.range_key_type {
                properties.insert(
                    "RangeKeyType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.range_key_value {
                properties.insert(
                    "RangeKeyValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TableName".to_string(),
                crate::value::ToValue::to_value(&self.table_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-alarmmodel-dynamodbv2.html
    pub struct DynamoDBv2_ {
        pub payload: Option<Box<Payload_>>,
        pub table_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_AlarmModel_DynamoDBv2 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::AlarmModel.DynamoDBv2"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_AlarmModel_DynamoDBv2 as DynamoDBv2;
    impl crate::value::ToValue for DynamoDBv2_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.payload {
                properties.insert(
                    "Payload".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TableName".to_string(),
                crate::value::ToValue::to_value(&self.table_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-alarmmodel-firehose.html
    pub struct Firehose_ {
        pub delivery_stream_name: crate::value::ExpString,
        pub payload: Option<Box<Payload_>>,
        pub separator: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_AlarmModel_Firehose {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::AlarmModel.Firehose"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_AlarmModel_Firehose as Firehose;
    impl crate::value::ToValue for Firehose_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DeliveryStreamName".to_string(),
                crate::value::ToValue::to_value(&self.delivery_stream_name),
            );
            if let Some(ref value) = self.payload {
                properties.insert(
                    "Payload".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.separator {
                properties.insert(
                    "Separator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-alarmmodel-initializationconfiguration.html
    pub struct InitializationConfiguration_ {
        pub disabled_on_initialization: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_AlarmModel_InitializationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::AlarmModel.InitializationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_AlarmModel_InitializationConfiguration as InitializationConfiguration;
    impl crate::value::ToValue for InitializationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DisabledOnInitialization".to_string(),
                crate::value::ToValue::to_value(&self.disabled_on_initialization),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-alarmmodel-iotevents.html
    pub struct IotEvents_ {
        pub input_name: crate::value::ExpString,
        pub payload: Option<Box<Payload_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_AlarmModel_IotEvents {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::AlarmModel.IotEvents"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_AlarmModel_IotEvents as IotEvents;
    impl crate::value::ToValue for IotEvents_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InputName".to_string(),
                crate::value::ToValue::to_value(&self.input_name),
            );
            if let Some(ref value) = self.payload {
                properties.insert(
                    "Payload".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-alarmmodel-iotsitewise.html
    pub struct IotSiteWise_ {
        pub asset_id: Option<crate::value::ExpString>,
        pub entry_id: Option<crate::value::ExpString>,
        pub property_alias: Option<crate::value::ExpString>,
        pub property_id: Option<crate::value::ExpString>,
        pub property_value: Option<Box<AssetPropertyValue_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_AlarmModel_IotSiteWise {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::AlarmModel.IotSiteWise"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_AlarmModel_IotSiteWise as IotSiteWise;
    impl crate::value::ToValue for IotSiteWise_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.asset_id {
                properties.insert(
                    "AssetId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.entry_id {
                properties.insert(
                    "EntryId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.property_alias {
                properties.insert(
                    "PropertyAlias".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.property_id {
                properties.insert(
                    "PropertyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.property_value {
                properties.insert(
                    "PropertyValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-alarmmodel-iottopicpublish.html
    pub struct IotTopicPublish_ {
        pub mqtt_topic: crate::value::ExpString,
        pub payload: Option<Box<Payload_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_AlarmModel_IotTopicPublish {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::AlarmModel.IotTopicPublish"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_AlarmModel_IotTopicPublish as IotTopicPublish;
    impl crate::value::ToValue for IotTopicPublish_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MqttTopic".to_string(),
                crate::value::ToValue::to_value(&self.mqtt_topic),
            );
            if let Some(ref value) = self.payload {
                properties.insert(
                    "Payload".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-alarmmodel-lambda.html
    pub struct Lambda_ {
        pub function_arn: crate::value::ExpString,
        pub payload: Option<Box<Payload_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_AlarmModel_Lambda {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::AlarmModel.Lambda"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_AlarmModel_Lambda as Lambda;
    impl crate::value::ToValue for Lambda_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FunctionArn".to_string(),
                crate::value::ToValue::to_value(&self.function_arn),
            );
            if let Some(ref value) = self.payload {
                properties.insert(
                    "Payload".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-alarmmodel-payload.html
    pub struct Payload_ {
        pub content_expression: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_AlarmModel_Payload {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::AlarmModel.Payload"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_AlarmModel_Payload as Payload;
    impl crate::value::ToValue for Payload_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ContentExpression".to_string(),
                crate::value::ToValue::to_value(&self.content_expression),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-alarmmodel-simplerule.html
    pub struct SimpleRule_ {
        pub comparison_operator: crate::value::ExpString,
        pub input_property: crate::value::ExpString,
        pub threshold: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_AlarmModel_SimpleRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::AlarmModel.SimpleRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_AlarmModel_SimpleRule as SimpleRule;
    impl crate::value::ToValue for SimpleRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ComparisonOperator".to_string(),
                crate::value::ToValue::to_value(&self.comparison_operator),
            );
            properties.insert(
                "InputProperty".to_string(),
                crate::value::ToValue::to_value(&self.input_property),
            );
            properties.insert(
                "Threshold".to_string(),
                crate::value::ToValue::to_value(&self.threshold),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-alarmmodel-sns.html
    pub struct Sns_ {
        pub payload: Option<Box<Payload_>>,
        pub target_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_AlarmModel_Sns {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::AlarmModel.Sns"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_AlarmModel_Sns as Sns;
    impl crate::value::ToValue for Sns_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.payload {
                properties.insert(
                    "Payload".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TargetArn".to_string(),
                crate::value::ToValue::to_value(&self.target_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-alarmmodel-sqs.html
    pub struct Sqs_ {
        pub payload: Option<Box<Payload_>>,
        pub queue_url: crate::value::ExpString,
        pub use_base64: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_AlarmModel_Sqs {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::AlarmModel.Sqs"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_AlarmModel_Sqs as Sqs;
    impl crate::value::ToValue for Sqs_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.payload {
                properties.insert(
                    "Payload".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "QueueUrl".to_string(),
                crate::value::ToValue::to_value(&self.queue_url),
            );
            if let Some(ref value) = self.use_base64 {
                properties.insert(
                    "UseBase64".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod detectormodel {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-action.html
    pub struct Action_ {
        pub clear_timer: Option<Box<ClearTimer_>>,
        pub dynamo_db: Option<Box<DynamoDB_>>,
        pub dynamo_d_bv2: Option<Box<DynamoDBv2_>>,
        pub firehose: Option<Box<Firehose_>>,
        pub iot_events: Option<Box<IotEvents_>>,
        pub iot_site_wise: Option<Box<IotSiteWise_>>,
        pub iot_topic_publish: Option<Box<IotTopicPublish_>>,
        pub lambda: Option<Box<Lambda_>>,
        pub reset_timer: Option<Box<ResetTimer_>>,
        pub set_timer: Option<Box<SetTimer_>>,
        pub set_variable: Option<Box<SetVariable_>>,
        pub sns: Option<Box<Sns_>>,
        pub sqs: Option<Box<Sqs_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_DetectorModel_Action {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::DetectorModel.Action"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_DetectorModel_Action as Action;
    impl crate::value::ToValue for Action_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.clear_timer {
                properties.insert(
                    "ClearTimer".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dynamo_db {
                properties.insert(
                    "DynamoDB".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dynamo_d_bv2 {
                properties.insert(
                    "DynamoDBv2".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.firehose {
                properties.insert(
                    "Firehose".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.iot_events {
                properties.insert(
                    "IotEvents".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.iot_site_wise {
                properties.insert(
                    "IotSiteWise".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.iot_topic_publish {
                properties.insert(
                    "IotTopicPublish".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lambda {
                properties.insert("Lambda".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.reset_timer {
                properties.insert(
                    "ResetTimer".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.set_timer {
                properties.insert(
                    "SetTimer".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.set_variable {
                properties.insert(
                    "SetVariable".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sns {
                properties.insert("Sns".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.sqs {
                properties.insert("Sqs".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-assetpropertytimestamp.html
    pub struct AssetPropertyTimestamp_ {
        pub offset_in_nanos: Option<crate::value::ExpString>,
        pub time_in_seconds: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_DetectorModel_AssetPropertyTimestamp {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::DetectorModel.AssetPropertyTimestamp"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_DetectorModel_AssetPropertyTimestamp as AssetPropertyTimestamp;
    impl crate::value::ToValue for AssetPropertyTimestamp_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.offset_in_nanos {
                properties.insert(
                    "OffsetInNanos".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TimeInSeconds".to_string(),
                crate::value::ToValue::to_value(&self.time_in_seconds),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-assetpropertyvalue.html
    pub struct AssetPropertyValue_ {
        pub quality: Option<crate::value::ExpString>,
        pub timestamp: Option<Box<AssetPropertyTimestamp_>>,
        pub value: Box<AssetPropertyVariant_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_DetectorModel_AssetPropertyValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::DetectorModel.AssetPropertyValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_DetectorModel_AssetPropertyValue as AssetPropertyValue;
    impl crate::value::ToValue for AssetPropertyValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.quality {
                properties.insert(
                    "Quality".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timestamp {
                properties.insert(
                    "Timestamp".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-assetpropertyvariant.html
    pub struct AssetPropertyVariant_ {
        pub boolean_value: Option<crate::value::ExpString>,
        pub double_value: Option<crate::value::ExpString>,
        pub integer_value: Option<crate::value::ExpString>,
        pub string_value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_DetectorModel_AssetPropertyVariant {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::DetectorModel.AssetPropertyVariant"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_DetectorModel_AssetPropertyVariant as AssetPropertyVariant;
    impl crate::value::ToValue for AssetPropertyVariant_ {
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
            if let Some(ref value) = self.integer_value {
                properties.insert(
                    "IntegerValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.string_value {
                properties.insert(
                    "StringValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-cleartimer.html
    pub struct ClearTimer_ {
        pub timer_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_DetectorModel_ClearTimer {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::DetectorModel.ClearTimer"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_DetectorModel_ClearTimer as ClearTimer;
    impl crate::value::ToValue for ClearTimer_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TimerName".to_string(),
                crate::value::ToValue::to_value(&self.timer_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-detectormodeldefinition.html
    pub struct DetectorModelDefinition_ {
        pub initial_state_name: crate::value::ExpString,
        pub states: Vec<State_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_DetectorModel_DetectorModelDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::DetectorModel.DetectorModelDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_DetectorModel_DetectorModelDefinition as DetectorModelDefinition;
    impl crate::value::ToValue for DetectorModelDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InitialStateName".to_string(),
                crate::value::ToValue::to_value(&self.initial_state_name),
            );
            properties.insert(
                "States".to_string(),
                crate::value::ToValue::to_value(&self.states),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-dynamodb.html
    pub struct DynamoDB_ {
        pub hash_key_field: crate::value::ExpString,
        pub hash_key_type: Option<crate::value::ExpString>,
        pub hash_key_value: crate::value::ExpString,
        pub operation: Option<crate::value::ExpString>,
        pub payload: Option<Box<Payload_>>,
        pub payload_field: Option<crate::value::ExpString>,
        pub range_key_field: Option<crate::value::ExpString>,
        pub range_key_type: Option<crate::value::ExpString>,
        pub range_key_value: Option<crate::value::ExpString>,
        pub table_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_DetectorModel_DynamoDB {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::DetectorModel.DynamoDB"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_DetectorModel_DynamoDB as DynamoDB;
    impl crate::value::ToValue for DynamoDB_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "HashKeyField".to_string(),
                crate::value::ToValue::to_value(&self.hash_key_field),
            );
            if let Some(ref value) = self.hash_key_type {
                properties.insert(
                    "HashKeyType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "HashKeyValue".to_string(),
                crate::value::ToValue::to_value(&self.hash_key_value),
            );
            if let Some(ref value) = self.operation {
                properties.insert(
                    "Operation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.payload {
                properties.insert(
                    "Payload".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.payload_field {
                properties.insert(
                    "PayloadField".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.range_key_field {
                properties.insert(
                    "RangeKeyField".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.range_key_type {
                properties.insert(
                    "RangeKeyType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.range_key_value {
                properties.insert(
                    "RangeKeyValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TableName".to_string(),
                crate::value::ToValue::to_value(&self.table_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-dynamodbv2.html
    pub struct DynamoDBv2_ {
        pub payload: Option<Box<Payload_>>,
        pub table_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_DetectorModel_DynamoDBv2 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::DetectorModel.DynamoDBv2"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_DetectorModel_DynamoDBv2 as DynamoDBv2;
    impl crate::value::ToValue for DynamoDBv2_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.payload {
                properties.insert(
                    "Payload".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TableName".to_string(),
                crate::value::ToValue::to_value(&self.table_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-event.html
    pub struct Event_ {
        pub actions: Option<Vec<Action_>>,
        pub condition: Option<crate::value::ExpString>,
        pub event_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_DetectorModel_Event {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::DetectorModel.Event"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_DetectorModel_Event as Event;
    impl crate::value::ToValue for Event_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.actions {
                properties.insert(
                    "Actions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.condition {
                properties.insert(
                    "Condition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "EventName".to_string(),
                crate::value::ToValue::to_value(&self.event_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-firehose.html
    pub struct Firehose_ {
        pub delivery_stream_name: crate::value::ExpString,
        pub payload: Option<Box<Payload_>>,
        pub separator: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_DetectorModel_Firehose {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::DetectorModel.Firehose"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_DetectorModel_Firehose as Firehose;
    impl crate::value::ToValue for Firehose_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DeliveryStreamName".to_string(),
                crate::value::ToValue::to_value(&self.delivery_stream_name),
            );
            if let Some(ref value) = self.payload {
                properties.insert(
                    "Payload".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.separator {
                properties.insert(
                    "Separator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-iotevents.html
    pub struct IotEvents_ {
        pub input_name: crate::value::ExpString,
        pub payload: Option<Box<Payload_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_DetectorModel_IotEvents {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::DetectorModel.IotEvents"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_DetectorModel_IotEvents as IotEvents;
    impl crate::value::ToValue for IotEvents_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InputName".to_string(),
                crate::value::ToValue::to_value(&self.input_name),
            );
            if let Some(ref value) = self.payload {
                properties.insert(
                    "Payload".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-iotsitewise.html
    pub struct IotSiteWise_ {
        pub asset_id: Option<crate::value::ExpString>,
        pub entry_id: Option<crate::value::ExpString>,
        pub property_alias: Option<crate::value::ExpString>,
        pub property_id: Option<crate::value::ExpString>,
        pub property_value: Box<AssetPropertyValue_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_DetectorModel_IotSiteWise {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::DetectorModel.IotSiteWise"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_DetectorModel_IotSiteWise as IotSiteWise;
    impl crate::value::ToValue for IotSiteWise_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.asset_id {
                properties.insert(
                    "AssetId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.entry_id {
                properties.insert(
                    "EntryId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.property_alias {
                properties.insert(
                    "PropertyAlias".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.property_id {
                properties.insert(
                    "PropertyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "PropertyValue".to_string(),
                crate::value::ToValue::to_value(&self.property_value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-iottopicpublish.html
    pub struct IotTopicPublish_ {
        pub mqtt_topic: crate::value::ExpString,
        pub payload: Option<Box<Payload_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_DetectorModel_IotTopicPublish {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::DetectorModel.IotTopicPublish"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_DetectorModel_IotTopicPublish as IotTopicPublish;
    impl crate::value::ToValue for IotTopicPublish_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MqttTopic".to_string(),
                crate::value::ToValue::to_value(&self.mqtt_topic),
            );
            if let Some(ref value) = self.payload {
                properties.insert(
                    "Payload".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-lambda.html
    pub struct Lambda_ {
        pub function_arn: crate::value::ExpString,
        pub payload: Option<Box<Payload_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_DetectorModel_Lambda {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::DetectorModel.Lambda"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_DetectorModel_Lambda as Lambda;
    impl crate::value::ToValue for Lambda_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FunctionArn".to_string(),
                crate::value::ToValue::to_value(&self.function_arn),
            );
            if let Some(ref value) = self.payload {
                properties.insert(
                    "Payload".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-onenter.html
    pub struct OnEnter_ {
        pub events: Option<Vec<Event_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_DetectorModel_OnEnter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::DetectorModel.OnEnter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_DetectorModel_OnEnter as OnEnter;
    impl crate::value::ToValue for OnEnter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.events {
                properties.insert("Events".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-onexit.html
    pub struct OnExit_ {
        pub events: Option<Vec<Event_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_DetectorModel_OnExit {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::DetectorModel.OnExit"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_DetectorModel_OnExit as OnExit;
    impl crate::value::ToValue for OnExit_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.events {
                properties.insert("Events".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-oninput.html
    pub struct OnInput_ {
        pub events: Option<Vec<Event_>>,
        pub transition_events: Option<Vec<TransitionEvent_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_DetectorModel_OnInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::DetectorModel.OnInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_DetectorModel_OnInput as OnInput;
    impl crate::value::ToValue for OnInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.events {
                properties.insert("Events".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.transition_events {
                properties.insert(
                    "TransitionEvents".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-payload.html
    pub struct Payload_ {
        pub content_expression: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_DetectorModel_Payload {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::DetectorModel.Payload"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_DetectorModel_Payload as Payload;
    impl crate::value::ToValue for Payload_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ContentExpression".to_string(),
                crate::value::ToValue::to_value(&self.content_expression),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-resettimer.html
    pub struct ResetTimer_ {
        pub timer_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_DetectorModel_ResetTimer {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::DetectorModel.ResetTimer"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_DetectorModel_ResetTimer as ResetTimer;
    impl crate::value::ToValue for ResetTimer_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TimerName".to_string(),
                crate::value::ToValue::to_value(&self.timer_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-settimer.html
    pub struct SetTimer_ {
        pub duration_expression: Option<crate::value::ExpString>,
        pub seconds: Option<i64>,
        pub timer_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_DetectorModel_SetTimer {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::DetectorModel.SetTimer"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_DetectorModel_SetTimer as SetTimer;
    impl crate::value::ToValue for SetTimer_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.duration_expression {
                properties.insert(
                    "DurationExpression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.seconds {
                properties.insert(
                    "Seconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TimerName".to_string(),
                crate::value::ToValue::to_value(&self.timer_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-setvariable.html
    pub struct SetVariable_ {
        pub value: crate::value::ExpString,
        pub variable_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_DetectorModel_SetVariable {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::DetectorModel.SetVariable"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_DetectorModel_SetVariable as SetVariable;
    impl crate::value::ToValue for SetVariable_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.insert(
                "VariableName".to_string(),
                crate::value::ToValue::to_value(&self.variable_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-sns.html
    pub struct Sns_ {
        pub payload: Option<Box<Payload_>>,
        pub target_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_DetectorModel_Sns {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::DetectorModel.Sns"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_DetectorModel_Sns as Sns;
    impl crate::value::ToValue for Sns_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.payload {
                properties.insert(
                    "Payload".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TargetArn".to_string(),
                crate::value::ToValue::to_value(&self.target_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-sqs.html
    pub struct Sqs_ {
        pub payload: Option<Box<Payload_>>,
        pub queue_url: crate::value::ExpString,
        pub use_base64: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_DetectorModel_Sqs {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::DetectorModel.Sqs"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_DetectorModel_Sqs as Sqs;
    impl crate::value::ToValue for Sqs_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.payload {
                properties.insert(
                    "Payload".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "QueueUrl".to_string(),
                crate::value::ToValue::to_value(&self.queue_url),
            );
            if let Some(ref value) = self.use_base64 {
                properties.insert(
                    "UseBase64".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-state.html
    pub struct State_ {
        pub on_enter: Option<Box<OnEnter_>>,
        pub on_exit: Option<Box<OnExit_>>,
        pub on_input: Option<Box<OnInput_>>,
        pub state_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_DetectorModel_State {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::DetectorModel.State"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_DetectorModel_State as State;
    impl crate::value::ToValue for State_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.on_enter {
                properties.insert(
                    "OnEnter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.on_exit {
                properties.insert("OnExit".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.on_input {
                properties.insert(
                    "OnInput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "StateName".to_string(),
                crate::value::ToValue::to_value(&self.state_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-detectormodel-transitionevent.html
    pub struct TransitionEvent_ {
        pub actions: Option<Vec<Action_>>,
        pub condition: crate::value::ExpString,
        pub event_name: crate::value::ExpString,
        pub next_state: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_DetectorModel_TransitionEvent {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::DetectorModel.TransitionEvent"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_DetectorModel_TransitionEvent as TransitionEvent;
    impl crate::value::ToValue for TransitionEvent_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.actions {
                properties.insert(
                    "Actions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Condition".to_string(),
                crate::value::ToValue::to_value(&self.condition),
            );
            properties.insert(
                "EventName".to_string(),
                crate::value::ToValue::to_value(&self.event_name),
            );
            properties.insert(
                "NextState".to_string(),
                crate::value::ToValue::to_value(&self.next_state),
            );
            properties.into()
        }
    }
}
pub mod input {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-input-attribute.html
    pub struct Attribute_ {
        pub json_path: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_Input_Attribute {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::Input.Attribute"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_Input_Attribute as Attribute;
    impl crate::value::ToValue for Attribute_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "JsonPath".to_string(),
                crate::value::ToValue::to_value(&self.json_path),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotevents-input-inputdefinition.html
    pub struct InputDefinition_ {
        pub attributes: Vec<Attribute_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotevents_Input_InputDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTEvents::Input.InputDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotevents_Input_InputDefinition as InputDefinition;
    impl crate::value::ToValue for InputDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Attributes".to_string(),
                crate::value::ToValue::to_value(&self.attributes),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotevents-alarmmodel.html
pub struct AlarmModel_ {
    pub alarm_capabilities: Option<super::iotevents::alarmmodel::AlarmCapabilities_>,
    pub alarm_event_actions: Option<super::iotevents::alarmmodel::AlarmEventActions_>,
    pub alarm_model_description: Option<crate::value::ExpString>,
    pub alarm_model_name: Option<crate::value::ExpString>,
    pub alarm_rule: super::iotevents::alarmmodel::AlarmRule_,
    pub key: Option<crate::value::ExpString>,
    pub role_arn: crate::value::ExpString,
    pub severity: Option<i64>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iotevents_AlarmModel {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTEvents::AlarmModel"
        $($field $value)*)
    };
}
pub use crate::__aws_iotevents_AlarmModel as AlarmModel;
impl crate::template::ToResource for AlarmModel_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTEvents"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AlarmModel"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.alarm_capabilities {
            properties.insert(
                "AlarmCapabilities".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.alarm_event_actions {
            properties.insert(
                "AlarmEventActions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.alarm_model_description {
            properties.insert(
                "AlarmModelDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.alarm_model_name {
            properties.insert(
                "AlarmModelName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "AlarmRule".to_string(),
            crate::value::ToValue::to_value(&self.alarm_rule),
        );
        if let Some(ref value) = self.key {
            properties.insert("Key".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        if let Some(ref value) = self.severity {
            properties.insert(
                "Severity".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotevents-detectormodel.html
pub struct DetectorModel_ {
    pub detector_model_definition: super::iotevents::detectormodel::DetectorModelDefinition_,
    pub detector_model_description: Option<crate::value::ExpString>,
    pub detector_model_name: Option<crate::value::ExpString>,
    pub evaluation_method: Option<crate::value::ExpString>,
    pub key: Option<crate::value::ExpString>,
    pub role_arn: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iotevents_DetectorModel {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTEvents::DetectorModel"
        $($field $value)*)
    };
}
pub use crate::__aws_iotevents_DetectorModel as DetectorModel;
impl crate::template::ToResource for DetectorModel_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTEvents"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DetectorModel"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DetectorModelDefinition".to_string(),
            crate::value::ToValue::to_value(&self.detector_model_definition),
        );
        if let Some(ref value) = self.detector_model_description {
            properties.insert(
                "DetectorModelDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.detector_model_name {
            properties.insert(
                "DetectorModelName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.evaluation_method {
            properties.insert(
                "EvaluationMethod".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.key {
            properties.insert("Key".to_string(), crate::value::ToValue::to_value(value));
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotevents-input.html
pub struct Input_ {
    pub input_definition: super::iotevents::input::InputDefinition_,
    pub input_description: Option<crate::value::ExpString>,
    pub input_name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iotevents_Input {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTEvents::Input"
        $($field $value)*)
    };
}
pub use crate::__aws_iotevents_Input as Input;
impl crate::template::ToResource for Input_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTEvents"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Input"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "InputDefinition".to_string(),
            crate::value::ToValue::to_value(&self.input_definition),
        );
        if let Some(ref value) = self.input_description {
            properties.insert(
                "InputDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.input_name {
            properties.insert(
                "InputName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
