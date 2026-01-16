pub mod campaign {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaigns-campaign-agentlessdialerconfig.html
    pub struct AgentlessDialerConfig_ {
        pub dialing_capacity: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connectcampaigns_Campaign_AgentlessDialerConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ConnectCampaigns::Campaign.AgentlessDialerConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connectcampaigns_Campaign_AgentlessDialerConfig as AgentlessDialerConfig;
    impl crate::value::ToValue for AgentlessDialerConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dialing_capacity {
                properties.insert(
                    "DialingCapacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaigns-campaign-answermachinedetectionconfig.html
    pub struct AnswerMachineDetectionConfig_ {
        pub await_answer_machine_prompt: Option<crate::value::ExpBool>,
        pub enable_answer_machine_detection: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connectcampaigns_Campaign_AnswerMachineDetectionConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ConnectCampaigns::Campaign.AnswerMachineDetectionConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connectcampaigns_Campaign_AnswerMachineDetectionConfig as AnswerMachineDetectionConfig;
    impl crate::value::ToValue for AnswerMachineDetectionConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.await_answer_machine_prompt {
                properties.insert(
                    "AwaitAnswerMachinePrompt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "EnableAnswerMachineDetection".to_string(),
                crate::value::ToValue::to_value(&self.enable_answer_machine_detection),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaigns-campaign-dialerconfig.html
    pub struct DialerConfig_ {
        pub agentless_dialer_config: Option<Box<AgentlessDialerConfig_>>,
        pub predictive_dialer_config: Option<Box<PredictiveDialerConfig_>>,
        pub progressive_dialer_config: Option<Box<ProgressiveDialerConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connectcampaigns_Campaign_DialerConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ConnectCampaigns::Campaign.DialerConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connectcampaigns_Campaign_DialerConfig as DialerConfig;
    impl crate::value::ToValue for DialerConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.agentless_dialer_config {
                properties.insert(
                    "AgentlessDialerConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.predictive_dialer_config {
                properties.insert(
                    "PredictiveDialerConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.progressive_dialer_config {
                properties.insert(
                    "ProgressiveDialerConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaigns-campaign-outboundcallconfig.html
    pub struct OutboundCallConfig_ {
        pub answer_machine_detection_config: Option<Box<AnswerMachineDetectionConfig_>>,
        pub connect_contact_flow_arn: crate::value::ExpString,
        pub connect_queue_arn: Option<crate::value::ExpString>,
        pub connect_source_phone_number: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connectcampaigns_Campaign_OutboundCallConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ConnectCampaigns::Campaign.OutboundCallConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connectcampaigns_Campaign_OutboundCallConfig as OutboundCallConfig;
    impl crate::value::ToValue for OutboundCallConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.answer_machine_detection_config {
                properties.insert(
                    "AnswerMachineDetectionConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ConnectContactFlowArn".to_string(),
                crate::value::ToValue::to_value(&self.connect_contact_flow_arn),
            );
            if let Some(ref value) = self.connect_queue_arn {
                properties.insert(
                    "ConnectQueueArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.connect_source_phone_number {
                properties.insert(
                    "ConnectSourcePhoneNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaigns-campaign-predictivedialerconfig.html
    pub struct PredictiveDialerConfig_ {
        pub bandwidth_allocation: f64,
        pub dialing_capacity: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connectcampaigns_Campaign_PredictiveDialerConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ConnectCampaigns::Campaign.PredictiveDialerConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connectcampaigns_Campaign_PredictiveDialerConfig as PredictiveDialerConfig;
    impl crate::value::ToValue for PredictiveDialerConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BandwidthAllocation".to_string(),
                crate::value::ToValue::to_value(&self.bandwidth_allocation),
            );
            if let Some(ref value) = self.dialing_capacity {
                properties.insert(
                    "DialingCapacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaigns-campaign-progressivedialerconfig.html
    pub struct ProgressiveDialerConfig_ {
        pub bandwidth_allocation: f64,
        pub dialing_capacity: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connectcampaigns_Campaign_ProgressiveDialerConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ConnectCampaigns::Campaign.ProgressiveDialerConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connectcampaigns_Campaign_ProgressiveDialerConfig as ProgressiveDialerConfig;
    impl crate::value::ToValue for ProgressiveDialerConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BandwidthAllocation".to_string(),
                crate::value::ToValue::to_value(&self.bandwidth_allocation),
            );
            if let Some(ref value) = self.dialing_capacity {
                properties.insert(
                    "DialingCapacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connectcampaigns-campaign.html
pub struct Campaign_ {
    pub connect_instance_arn: crate::value::ExpString,
    pub dialer_config: super::connectcampaigns::campaign::DialerConfig_,
    pub name: crate::value::ExpString,
    pub outbound_call_config: super::connectcampaigns::campaign::OutboundCallConfig_,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_connectcampaigns_Campaign {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::ConnectCampaigns::Campaign"
        $($field $value)*)
    };
}
pub use crate::__aws_connectcampaigns_Campaign as Campaign;
impl crate::template::ToResource for Campaign_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ConnectCampaigns"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Campaign"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ConnectInstanceArn".to_string(),
            crate::value::ToValue::to_value(&self.connect_instance_arn),
        );
        properties.insert(
            "DialerConfig".to_string(),
            crate::value::ToValue::to_value(&self.dialer_config),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "OutboundCallConfig".to_string(),
            crate::value::ToValue::to_value(&self.outbound_call_config),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
