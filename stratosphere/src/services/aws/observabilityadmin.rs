pub mod organizationtelemetryrule {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-organizationtelemetryrule-telemetrydestinationconfiguration.html
    pub struct TelemetryDestinationConfiguration_ {
        pub destination_pattern: Option<crate::value::ExpString>,
        pub destination_type: Option<crate::value::ExpString>,
        pub retention_in_days: Option<i32>,
        pub vpc_flow_log_parameters: Option<Box<VPCFlowLogParameters_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_OrganizationTelemetryRule_TelemetryDestinationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::OrganizationTelemetryRule.TelemetryDestinationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_OrganizationTelemetryRule_TelemetryDestinationConfiguration as TelemetryDestinationConfiguration;
    impl crate::value::ToValue for TelemetryDestinationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.destination_pattern {
                properties.insert(
                    "DestinationPattern".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.destination_type {
                properties.insert(
                    "DestinationType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.retention_in_days {
                properties.insert(
                    "RetentionInDays".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpc_flow_log_parameters {
                properties.insert(
                    "VPCFlowLogParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-organizationtelemetryrule-telemetryrule.html
    pub struct TelemetryRule_ {
        pub destination_configuration: Option<Box<TelemetryDestinationConfiguration_>>,
        pub resource_type: crate::value::ExpString,
        pub scope: Option<crate::value::ExpString>,
        pub selection_criteria: Option<crate::value::ExpString>,
        pub telemetry_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_OrganizationTelemetryRule_TelemetryRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::OrganizationTelemetryRule.TelemetryRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_OrganizationTelemetryRule_TelemetryRule as TelemetryRule;
    impl crate::value::ToValue for TelemetryRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.destination_configuration {
                properties.insert(
                    "DestinationConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ResourceType".to_string(),
                crate::value::ToValue::to_value(&self.resource_type),
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
            properties.insert(
                "TelemetryType".to_string(),
                crate::value::ToValue::to_value(&self.telemetry_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-organizationtelemetryrule-vpcflowlogparameters.html
    pub struct VPCFlowLogParameters_ {
        pub log_format: Option<crate::value::ExpString>,
        pub max_aggregation_interval: Option<i32>,
        pub traffic_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_OrganizationTelemetryRule_VPCFlowLogParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::OrganizationTelemetryRule.VPCFlowLogParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_OrganizationTelemetryRule_VPCFlowLogParameters as VPCFlowLogParameters;
    impl crate::value::ToValue for VPCFlowLogParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.log_format {
                properties.insert(
                    "LogFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_aggregation_interval {
                properties.insert(
                    "MaxAggregationInterval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.traffic_type {
                properties.insert(
                    "TrafficType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod telemetryrule {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-telemetryrule-telemetrydestinationconfiguration.html
    pub struct TelemetryDestinationConfiguration_ {
        pub destination_pattern: Option<crate::value::ExpString>,
        pub destination_type: Option<crate::value::ExpString>,
        pub retention_in_days: Option<i32>,
        pub vpc_flow_log_parameters: Option<Box<VPCFlowLogParameters_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_TelemetryRule_TelemetryDestinationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::TelemetryRule.TelemetryDestinationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_TelemetryRule_TelemetryDestinationConfiguration as TelemetryDestinationConfiguration;
    impl crate::value::ToValue for TelemetryDestinationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.destination_pattern {
                properties.insert(
                    "DestinationPattern".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.destination_type {
                properties.insert(
                    "DestinationType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.retention_in_days {
                properties.insert(
                    "RetentionInDays".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpc_flow_log_parameters {
                properties.insert(
                    "VPCFlowLogParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-telemetryrule-telemetryrule.html
    pub struct TelemetryRule_ {
        pub destination_configuration: Option<Box<TelemetryDestinationConfiguration_>>,
        pub resource_type: crate::value::ExpString,
        pub selection_criteria: Option<crate::value::ExpString>,
        pub telemetry_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_TelemetryRule_TelemetryRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::TelemetryRule.TelemetryRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_TelemetryRule_TelemetryRule as TelemetryRule;
    impl crate::value::ToValue for TelemetryRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.destination_configuration {
                properties.insert(
                    "DestinationConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ResourceType".to_string(),
                crate::value::ToValue::to_value(&self.resource_type),
            );
            if let Some(ref value) = self.selection_criteria {
                properties.insert(
                    "SelectionCriteria".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TelemetryType".to_string(),
                crate::value::ToValue::to_value(&self.telemetry_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-observabilityadmin-telemetryrule-vpcflowlogparameters.html
    pub struct VPCFlowLogParameters_ {
        pub log_format: Option<crate::value::ExpString>,
        pub max_aggregation_interval: Option<i32>,
        pub traffic_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_observabilityadmin_TelemetryRule_VPCFlowLogParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ObservabilityAdmin::TelemetryRule.VPCFlowLogParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_observabilityadmin_TelemetryRule_VPCFlowLogParameters as VPCFlowLogParameters;
    impl crate::value::ToValue for VPCFlowLogParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.log_format {
                properties.insert(
                    "LogFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_aggregation_interval {
                properties.insert(
                    "MaxAggregationInterval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.traffic_type {
                properties.insert(
                    "TrafficType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-observabilityadmin-organizationtelemetryrule.html
pub struct OrganizationTelemetryRule_ {
    pub rule: super::observabilityadmin::organizationtelemetryrule::TelemetryRule_,
    pub rule_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_observabilityadmin_OrganizationTelemetryRule {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ObservabilityAdmin::OrganizationTelemetryRule"
        $($field $value)*)
    };
}
pub use crate::__aws_observabilityadmin_OrganizationTelemetryRule as OrganizationTelemetryRule;
impl crate::template::ToResource for OrganizationTelemetryRule_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ObservabilityAdmin"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("OrganizationTelemetryRule"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Rule".to_string(),
            crate::value::ToValue::to_value(&self.rule),
        );
        properties.insert(
            "RuleName".to_string(),
            crate::value::ToValue::to_value(&self.rule_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-observabilityadmin-telemetryrule.html
pub struct TelemetryRule_ {
    pub rule: super::observabilityadmin::telemetryrule::TelemetryRule_,
    pub rule_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_observabilityadmin_TelemetryRule {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ObservabilityAdmin::TelemetryRule"
        $($field $value)*)
    };
}
pub use crate::__aws_observabilityadmin_TelemetryRule as TelemetryRule;
impl crate::template::ToResource for TelemetryRule_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ObservabilityAdmin"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TelemetryRule"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Rule".to_string(),
            crate::value::ToValue::to_value(&self.rule),
        );
        properties.insert(
            "RuleName".to_string(),
            crate::value::ToValue::to_value(&self.rule_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
