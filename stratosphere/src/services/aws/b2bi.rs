pub mod capability {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-b2bi-capability-capabilityconfiguration.html>
    pub struct CapabilityConfiguration_ {
        pub edi: Box<EdiConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_b2bi_Capability_CapabilityConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::B2BI::Capability.CapabilityConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_b2bi_Capability_CapabilityConfiguration as CapabilityConfiguration;
    impl crate::value::ToValue for CapabilityConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Edi".to_string(),
                crate::value::ToValue::to_value(&self.edi),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-b2bi-capability-ediconfiguration.html>
    pub struct EdiConfiguration_ {
        pub capability_direction: Option<crate::value::ExpString>,
        pub input_location: Box<S3Location_>,
        pub output_location: Box<S3Location_>,
        pub transformer_id: crate::value::ExpString,
        pub r#type: Box<EdiType_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_b2bi_Capability_EdiConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::B2BI::Capability.EdiConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_b2bi_Capability_EdiConfiguration as EdiConfiguration;
    impl crate::value::ToValue for EdiConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.capability_direction {
                properties.insert(
                    "CapabilityDirection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "InputLocation".to_string(),
                crate::value::ToValue::to_value(&self.input_location),
            );
            properties.insert(
                "OutputLocation".to_string(),
                crate::value::ToValue::to_value(&self.output_location),
            );
            properties.insert(
                "TransformerId".to_string(),
                crate::value::ToValue::to_value(&self.transformer_id),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-b2bi-capability-editype.html>
    pub struct EdiType_ {
        pub x12_details: Box<X12Details_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_b2bi_Capability_EdiType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::B2BI::Capability.EdiType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_b2bi_Capability_EdiType as EdiType;
    impl crate::value::ToValue for EdiType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "X12Details".to_string(),
                crate::value::ToValue::to_value(&self.x12_details),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-b2bi-capability-s3location.html>
    pub struct S3Location_ {
        pub bucket_name: Option<crate::value::ExpString>,
        pub key: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_b2bi_Capability_S3Location {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::B2BI::Capability.S3Location"
            $($field $value)*)
        };
    }
    pub use crate::__aws_b2bi_Capability_S3Location as S3Location;
    impl crate::value::ToValue for S3Location_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bucket_name {
                properties.insert(
                    "BucketName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.key {
                properties.insert("Key".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-b2bi-capability-x12details.html>
    pub struct X12Details_ {
        pub transaction_set: Option<crate::value::ExpString>,
        pub version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_b2bi_Capability_X12Details {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::B2BI::Capability.X12Details"
            $($field $value)*)
        };
    }
    pub use crate::__aws_b2bi_Capability_X12Details as X12Details;
    impl crate::value::ToValue for X12Details_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.transaction_set {
                properties.insert(
                    "TransactionSet".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.version {
                properties.insert(
                    "Version".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod partnership {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-b2bi-partnership-capabilityoptions.html>
    pub struct CapabilityOptions_ {
        pub inbound_edi: Option<Box<InboundEdiOptions_>>,
        pub outbound_edi: Option<Box<OutboundEdiOptions_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_b2bi_Partnership_CapabilityOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::B2BI::Partnership.CapabilityOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_b2bi_Partnership_CapabilityOptions as CapabilityOptions;
    impl crate::value::ToValue for CapabilityOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.inbound_edi {
                properties.insert(
                    "InboundEdi".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.outbound_edi {
                properties.insert(
                    "OutboundEdi".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-b2bi-partnership-inboundedioptions.html>
    pub struct InboundEdiOptions_ {
        pub x12: Option<Box<X12InboundEdiOptions_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_b2bi_Partnership_InboundEdiOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::B2BI::Partnership.InboundEdiOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_b2bi_Partnership_InboundEdiOptions as InboundEdiOptions;
    impl crate::value::ToValue for InboundEdiOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.x12 {
                properties.insert("X12".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-b2bi-partnership-outboundedioptions.html>
    pub struct OutboundEdiOptions_ {
        pub x12: Box<X12Envelope_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_b2bi_Partnership_OutboundEdiOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::B2BI::Partnership.OutboundEdiOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_b2bi_Partnership_OutboundEdiOptions as OutboundEdiOptions;
    impl crate::value::ToValue for OutboundEdiOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "X12".to_string(),
                crate::value::ToValue::to_value(&self.x12),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-b2bi-partnership-wrapoptions.html>
    pub struct WrapOptions_ {
        pub line_length: Option<f64>,
        pub line_terminator: Option<crate::value::ExpString>,
        pub wrap_by: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_b2bi_Partnership_WrapOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::B2BI::Partnership.WrapOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_b2bi_Partnership_WrapOptions as WrapOptions;
    impl crate::value::ToValue for WrapOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.line_length {
                properties.insert(
                    "LineLength".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.line_terminator {
                properties.insert(
                    "LineTerminator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.wrap_by {
                properties.insert("WrapBy".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-b2bi-partnership-x12acknowledgmentoptions.html>
    pub struct X12AcknowledgmentOptions_ {
        pub functional_acknowledgment: crate::value::ExpString,
        pub technical_acknowledgment: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_b2bi_Partnership_X12AcknowledgmentOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::B2BI::Partnership.X12AcknowledgmentOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_b2bi_Partnership_X12AcknowledgmentOptions as X12AcknowledgmentOptions;
    impl crate::value::ToValue for X12AcknowledgmentOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FunctionalAcknowledgment".to_string(),
                crate::value::ToValue::to_value(&self.functional_acknowledgment),
            );
            properties.insert(
                "TechnicalAcknowledgment".to_string(),
                crate::value::ToValue::to_value(&self.technical_acknowledgment),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-b2bi-partnership-x12controlnumbers.html>
    pub struct X12ControlNumbers_ {
        pub starting_functional_group_control_number: Option<f64>,
        pub starting_interchange_control_number: Option<f64>,
        pub starting_transaction_set_control_number: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_b2bi_Partnership_X12ControlNumbers {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::B2BI::Partnership.X12ControlNumbers"
            $($field $value)*)
        };
    }
    pub use crate::__aws_b2bi_Partnership_X12ControlNumbers as X12ControlNumbers;
    impl crate::value::ToValue for X12ControlNumbers_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.starting_functional_group_control_number {
                properties.insert(
                    "StartingFunctionalGroupControlNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.starting_interchange_control_number {
                properties.insert(
                    "StartingInterchangeControlNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.starting_transaction_set_control_number {
                properties.insert(
                    "StartingTransactionSetControlNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-b2bi-partnership-x12delimiters.html>
    pub struct X12Delimiters_ {
        pub component_separator: Option<crate::value::ExpString>,
        pub data_element_separator: Option<crate::value::ExpString>,
        pub segment_terminator: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_b2bi_Partnership_X12Delimiters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::B2BI::Partnership.X12Delimiters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_b2bi_Partnership_X12Delimiters as X12Delimiters;
    impl crate::value::ToValue for X12Delimiters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.component_separator {
                properties.insert(
                    "ComponentSeparator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.data_element_separator {
                properties.insert(
                    "DataElementSeparator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.segment_terminator {
                properties.insert(
                    "SegmentTerminator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-b2bi-partnership-x12envelope.html>
    pub struct X12Envelope_ {
        pub common: Option<Box<X12OutboundEdiHeaders_>>,
        pub wrap_options: Option<Box<WrapOptions_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_b2bi_Partnership_X12Envelope {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::B2BI::Partnership.X12Envelope"
            $($field $value)*)
        };
    }
    pub use crate::__aws_b2bi_Partnership_X12Envelope as X12Envelope;
    impl crate::value::ToValue for X12Envelope_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.common {
                properties.insert("Common".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.wrap_options {
                properties.insert(
                    "WrapOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-b2bi-partnership-x12functionalgroupheaders.html>
    pub struct X12FunctionalGroupHeaders_ {
        pub application_receiver_code: Option<crate::value::ExpString>,
        pub application_sender_code: Option<crate::value::ExpString>,
        pub responsible_agency_code: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_b2bi_Partnership_X12FunctionalGroupHeaders {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::B2BI::Partnership.X12FunctionalGroupHeaders"
            $($field $value)*)
        };
    }
    pub use crate::__aws_b2bi_Partnership_X12FunctionalGroupHeaders as X12FunctionalGroupHeaders;
    impl crate::value::ToValue for X12FunctionalGroupHeaders_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.application_receiver_code {
                properties.insert(
                    "ApplicationReceiverCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.application_sender_code {
                properties.insert(
                    "ApplicationSenderCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.responsible_agency_code {
                properties.insert(
                    "ResponsibleAgencyCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-b2bi-partnership-x12inboundedioptions.html>
    pub struct X12InboundEdiOptions_ {
        pub acknowledgment_options: Option<Box<X12AcknowledgmentOptions_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_b2bi_Partnership_X12InboundEdiOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::B2BI::Partnership.X12InboundEdiOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_b2bi_Partnership_X12InboundEdiOptions as X12InboundEdiOptions;
    impl crate::value::ToValue for X12InboundEdiOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.acknowledgment_options {
                properties.insert(
                    "AcknowledgmentOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-b2bi-partnership-x12interchangecontrolheaders.html>
    pub struct X12InterchangeControlHeaders_ {
        pub acknowledgment_requested_code: Option<crate::value::ExpString>,
        pub receiver_id: Option<crate::value::ExpString>,
        pub receiver_id_qualifier: Option<crate::value::ExpString>,
        pub repetition_separator: Option<crate::value::ExpString>,
        pub sender_id: Option<crate::value::ExpString>,
        pub sender_id_qualifier: Option<crate::value::ExpString>,
        pub usage_indicator_code: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_b2bi_Partnership_X12InterchangeControlHeaders {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::B2BI::Partnership.X12InterchangeControlHeaders"
            $($field $value)*)
        };
    }
    pub use crate::__aws_b2bi_Partnership_X12InterchangeControlHeaders as X12InterchangeControlHeaders;
    impl crate::value::ToValue for X12InterchangeControlHeaders_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.acknowledgment_requested_code {
                properties.insert(
                    "AcknowledgmentRequestedCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.receiver_id {
                properties.insert(
                    "ReceiverId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.receiver_id_qualifier {
                properties.insert(
                    "ReceiverIdQualifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.repetition_separator {
                properties.insert(
                    "RepetitionSeparator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sender_id {
                properties.insert(
                    "SenderId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sender_id_qualifier {
                properties.insert(
                    "SenderIdQualifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.usage_indicator_code {
                properties.insert(
                    "UsageIndicatorCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-b2bi-partnership-x12outboundediheaders.html>
    pub struct X12OutboundEdiHeaders_ {
        pub control_numbers: Option<Box<X12ControlNumbers_>>,
        pub delimiters: Option<Box<X12Delimiters_>>,
        pub functional_group_headers: Option<Box<X12FunctionalGroupHeaders_>>,
        pub gs05_time_format: Option<crate::value::ExpString>,
        pub interchange_control_headers: Option<Box<X12InterchangeControlHeaders_>>,
        pub validate_edi: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_b2bi_Partnership_X12OutboundEdiHeaders {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::B2BI::Partnership.X12OutboundEdiHeaders"
            $($field $value)*)
        };
    }
    pub use crate::__aws_b2bi_Partnership_X12OutboundEdiHeaders as X12OutboundEdiHeaders;
    impl crate::value::ToValue for X12OutboundEdiHeaders_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.control_numbers {
                properties.insert(
                    "ControlNumbers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.delimiters {
                properties.insert(
                    "Delimiters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.functional_group_headers {
                properties.insert(
                    "FunctionalGroupHeaders".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.gs05_time_format {
                properties.insert(
                    "Gs05TimeFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.interchange_control_headers {
                properties.insert(
                    "InterchangeControlHeaders".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.validate_edi {
                properties.insert(
                    "ValidateEdi".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod transformer {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-b2bi-transformer-advancedoptions.html>
    pub struct AdvancedOptions_ {
        pub x12: Option<Box<X12AdvancedOptions_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_b2bi_Transformer_AdvancedOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::B2BI::Transformer.AdvancedOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_b2bi_Transformer_AdvancedOptions as AdvancedOptions;
    impl crate::value::ToValue for AdvancedOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.x12 {
                properties.insert("X12".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-b2bi-transformer-formatoptions.html>
    pub struct FormatOptions_ {
        pub x12: Box<X12Details_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_b2bi_Transformer_FormatOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::B2BI::Transformer.FormatOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_b2bi_Transformer_FormatOptions as FormatOptions;
    impl crate::value::ToValue for FormatOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "X12".to_string(),
                crate::value::ToValue::to_value(&self.x12),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-b2bi-transformer-inputconversion.html>
    pub struct InputConversion_ {
        pub advanced_options: Option<Box<AdvancedOptions_>>,
        pub format_options: Option<Box<FormatOptions_>>,
        pub from_format: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_b2bi_Transformer_InputConversion {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::B2BI::Transformer.InputConversion"
            $($field $value)*)
        };
    }
    pub use crate::__aws_b2bi_Transformer_InputConversion as InputConversion;
    impl crate::value::ToValue for InputConversion_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.advanced_options {
                properties.insert(
                    "AdvancedOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.format_options {
                properties.insert(
                    "FormatOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "FromFormat".to_string(),
                crate::value::ToValue::to_value(&self.from_format),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-b2bi-transformer-mapping.html>
    pub struct Mapping_ {
        pub template: Option<crate::value::ExpString>,
        pub template_language: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_b2bi_Transformer_Mapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::B2BI::Transformer.Mapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_b2bi_Transformer_Mapping as Mapping;
    impl crate::value::ToValue for Mapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.template {
                properties.insert(
                    "Template".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TemplateLanguage".to_string(),
                crate::value::ToValue::to_value(&self.template_language),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-b2bi-transformer-outputconversion.html>
    pub struct OutputConversion_ {
        pub advanced_options: Option<Box<AdvancedOptions_>>,
        pub format_options: Option<Box<FormatOptions_>>,
        pub to_format: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_b2bi_Transformer_OutputConversion {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::B2BI::Transformer.OutputConversion"
            $($field $value)*)
        };
    }
    pub use crate::__aws_b2bi_Transformer_OutputConversion as OutputConversion;
    impl crate::value::ToValue for OutputConversion_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.advanced_options {
                properties.insert(
                    "AdvancedOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.format_options {
                properties.insert(
                    "FormatOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ToFormat".to_string(),
                crate::value::ToValue::to_value(&self.to_format),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-b2bi-transformer-sampledocumentkeys.html>
    pub struct SampleDocumentKeys_ {
        pub input: Option<crate::value::ExpString>,
        pub output: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_b2bi_Transformer_SampleDocumentKeys {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::B2BI::Transformer.SampleDocumentKeys"
            $($field $value)*)
        };
    }
    pub use crate::__aws_b2bi_Transformer_SampleDocumentKeys as SampleDocumentKeys;
    impl crate::value::ToValue for SampleDocumentKeys_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.input {
                properties.insert("Input".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.output {
                properties.insert("Output".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-b2bi-transformer-sampledocuments.html>
    pub struct SampleDocuments_ {
        pub bucket_name: crate::value::ExpString,
        pub keys: Vec<SampleDocumentKeys_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_b2bi_Transformer_SampleDocuments {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::B2BI::Transformer.SampleDocuments"
            $($field $value)*)
        };
    }
    pub use crate::__aws_b2bi_Transformer_SampleDocuments as SampleDocuments;
    impl crate::value::ToValue for SampleDocuments_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BucketName".to_string(),
                crate::value::ToValue::to_value(&self.bucket_name),
            );
            properties.insert(
                "Keys".to_string(),
                crate::value::ToValue::to_value(&self.keys),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-b2bi-transformer-x12advancedoptions.html>
    pub struct X12AdvancedOptions_ {
        pub split_options: Option<Box<X12SplitOptions_>>,
        pub validation_options: Option<Box<X12ValidationOptions_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_b2bi_Transformer_X12AdvancedOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::B2BI::Transformer.X12AdvancedOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_b2bi_Transformer_X12AdvancedOptions as X12AdvancedOptions;
    impl crate::value::ToValue for X12AdvancedOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.split_options {
                properties.insert(
                    "SplitOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.validation_options {
                properties.insert(
                    "ValidationOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-b2bi-transformer-x12codelistvalidationrule.html>
    pub struct X12CodeListValidationRule_ {
        pub codes_to_add: Option<Vec<crate::value::ExpString>>,
        pub codes_to_remove: Option<Vec<crate::value::ExpString>>,
        pub element_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_b2bi_Transformer_X12CodeListValidationRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::B2BI::Transformer.X12CodeListValidationRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_b2bi_Transformer_X12CodeListValidationRule as X12CodeListValidationRule;
    impl crate::value::ToValue for X12CodeListValidationRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.codes_to_add {
                properties.insert(
                    "CodesToAdd".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.codes_to_remove {
                properties.insert(
                    "CodesToRemove".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ElementId".to_string(),
                crate::value::ToValue::to_value(&self.element_id),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-b2bi-transformer-x12details.html>
    pub struct X12Details_ {
        pub transaction_set: Option<crate::value::ExpString>,
        pub version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_b2bi_Transformer_X12Details {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::B2BI::Transformer.X12Details"
            $($field $value)*)
        };
    }
    pub use crate::__aws_b2bi_Transformer_X12Details as X12Details;
    impl crate::value::ToValue for X12Details_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.transaction_set {
                properties.insert(
                    "TransactionSet".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.version {
                properties.insert(
                    "Version".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-b2bi-transformer-x12elementlengthvalidationrule.html>
    pub struct X12ElementLengthValidationRule_ {
        pub element_id: crate::value::ExpString,
        pub max_length: f64,
        pub min_length: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_b2bi_Transformer_X12ElementLengthValidationRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::B2BI::Transformer.X12ElementLengthValidationRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_b2bi_Transformer_X12ElementLengthValidationRule as X12ElementLengthValidationRule;
    impl crate::value::ToValue for X12ElementLengthValidationRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ElementId".to_string(),
                crate::value::ToValue::to_value(&self.element_id),
            );
            properties.insert(
                "MaxLength".to_string(),
                crate::value::ToValue::to_value(&self.max_length),
            );
            properties.insert(
                "MinLength".to_string(),
                crate::value::ToValue::to_value(&self.min_length),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-b2bi-transformer-x12elementrequirementvalidationrule.html>
    pub struct X12ElementRequirementValidationRule_ {
        pub element_position: crate::value::ExpString,
        pub requirement: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_b2bi_Transformer_X12ElementRequirementValidationRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::B2BI::Transformer.X12ElementRequirementValidationRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_b2bi_Transformer_X12ElementRequirementValidationRule as X12ElementRequirementValidationRule;
    impl crate::value::ToValue for X12ElementRequirementValidationRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ElementPosition".to_string(),
                crate::value::ToValue::to_value(&self.element_position),
            );
            properties.insert(
                "Requirement".to_string(),
                crate::value::ToValue::to_value(&self.requirement),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-b2bi-transformer-x12splitoptions.html>
    pub struct X12SplitOptions_ {
        pub split_by: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_b2bi_Transformer_X12SplitOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::B2BI::Transformer.X12SplitOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_b2bi_Transformer_X12SplitOptions as X12SplitOptions;
    impl crate::value::ToValue for X12SplitOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.split_by {
                properties.insert(
                    "SplitBy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-b2bi-transformer-x12validationoptions.html>
    pub struct X12ValidationOptions_ {
        pub validation_rules: Option<Vec<X12ValidationRule_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_b2bi_Transformer_X12ValidationOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::B2BI::Transformer.X12ValidationOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_b2bi_Transformer_X12ValidationOptions as X12ValidationOptions;
    impl crate::value::ToValue for X12ValidationOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.validation_rules {
                properties.insert(
                    "ValidationRules".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-b2bi-transformer-x12validationrule.html>
    pub struct X12ValidationRule_ {
        pub code_list_validation_rule: Option<Box<X12CodeListValidationRule_>>,
        pub element_length_validation_rule: Option<Box<X12ElementLengthValidationRule_>>,
        pub element_requirement_validation_rule: Option<Box<X12ElementRequirementValidationRule_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_b2bi_Transformer_X12ValidationRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::B2BI::Transformer.X12ValidationRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_b2bi_Transformer_X12ValidationRule as X12ValidationRule;
    impl crate::value::ToValue for X12ValidationRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.code_list_validation_rule {
                properties.insert(
                    "CodeListValidationRule".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.element_length_validation_rule {
                properties.insert(
                    "ElementLengthValidationRule".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.element_requirement_validation_rule {
                properties.insert(
                    "ElementRequirementValidationRule".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-b2bi-capability.html>
pub struct Capability_ {
    pub configuration: super::b2bi::capability::CapabilityConfiguration_,
    pub instructions_documents: Option<Vec<super::b2bi::capability::S3Location_>>,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_b2bi_Capability {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::B2BI::Capability"
        $($field $value)*)
    };
}
pub use crate::__aws_b2bi_Capability as Capability;
impl crate::template::ToResource for Capability_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("B2BI"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Capability"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Configuration".to_string(),
            crate::value::ToValue::to_value(&self.configuration),
        );
        if let Some(ref value) = self.instructions_documents {
            properties.insert(
                "InstructionsDocuments".to_string(),
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
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-b2bi-partnership.html>
pub struct Partnership_ {
    pub capabilities: Vec<crate::value::ExpString>,
    pub capability_options: Option<super::b2bi::partnership::CapabilityOptions_>,
    pub email: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub phone: Option<crate::value::ExpString>,
    pub profile_id: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_b2bi_Partnership {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::B2BI::Partnership"
        $($field $value)*)
    };
}
pub use crate::__aws_b2bi_Partnership as Partnership;
impl crate::template::ToResource for Partnership_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("B2BI"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Partnership"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Capabilities".to_string(),
            crate::value::ToValue::to_value(&self.capabilities),
        );
        if let Some(ref value) = self.capability_options {
            properties.insert(
                "CapabilityOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Email".to_string(),
            crate::value::ToValue::to_value(&self.email),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.phone {
            properties.insert("Phone".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "ProfileId".to_string(),
            crate::value::ToValue::to_value(&self.profile_id),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-b2bi-profile.html>
pub struct Profile_ {
    pub business_name: crate::value::ExpString,
    pub email: Option<crate::value::ExpString>,
    pub logging: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub phone: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_b2bi_Profile {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::B2BI::Profile" $($field
        $value)*)
    };
}
pub use crate::__aws_b2bi_Profile as Profile;
impl crate::template::ToResource for Profile_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("B2BI"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Profile"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "BusinessName".to_string(),
            crate::value::ToValue::to_value(&self.business_name),
        );
        if let Some(ref value) = self.email {
            properties.insert("Email".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Logging".to_string(),
            crate::value::ToValue::to_value(&self.logging),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "Phone".to_string(),
            crate::value::ToValue::to_value(&self.phone),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-b2bi-transformer.html>
pub struct Transformer_ {
    pub input_conversion: Option<super::b2bi::transformer::InputConversion_>,
    pub mapping: Option<super::b2bi::transformer::Mapping_>,
    pub name: crate::value::ExpString,
    pub output_conversion: Option<super::b2bi::transformer::OutputConversion_>,
    pub sample_documents: Option<super::b2bi::transformer::SampleDocuments_>,
    pub status: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_b2bi_Transformer {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::B2BI::Transformer"
        $($field $value)*)
    };
}
pub use crate::__aws_b2bi_Transformer as Transformer;
impl crate::template::ToResource for Transformer_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("B2BI"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Transformer"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.input_conversion {
            properties.insert(
                "InputConversion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.mapping {
            properties.insert(
                "Mapping".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.output_conversion {
            properties.insert(
                "OutputConversion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sample_documents {
            properties.insert(
                "SampleDocuments".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Status".to_string(),
            crate::value::ToValue::to_value(&self.status),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
