pub mod cluster {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoverycontrol-cluster-clusterendpoint.html
    pub struct ClusterEndpoint_ {
        pub endpoint: Option<crate::value::ExpString>,
        pub region: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_route53recoverycontrol_Cluster_ClusterEndpoint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Route53RecoveryControl::Cluster.ClusterEndpoint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_route53recoverycontrol_Cluster_ClusterEndpoint as ClusterEndpoint;
    impl crate::value::ToValue for ClusterEndpoint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.endpoint {
                properties.insert(
                    "Endpoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.region {
                properties.insert("Region".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod safetyrule {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoverycontrol-safetyrule-assertionrule.html
    pub struct AssertionRule_ {
        pub asserted_controls: Vec<crate::value::ExpString>,
        pub wait_period_ms: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_route53recoverycontrol_SafetyRule_AssertionRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Route53RecoveryControl::SafetyRule.AssertionRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_route53recoverycontrol_SafetyRule_AssertionRule as AssertionRule;
    impl crate::value::ToValue for AssertionRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AssertedControls".to_string(),
                crate::value::ToValue::to_value(&self.asserted_controls),
            );
            properties.insert(
                "WaitPeriodMs".to_string(),
                crate::value::ToValue::to_value(&self.wait_period_ms),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoverycontrol-safetyrule-gatingrule.html
    pub struct GatingRule_ {
        pub gating_controls: Vec<crate::value::ExpString>,
        pub target_controls: Vec<crate::value::ExpString>,
        pub wait_period_ms: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_route53recoverycontrol_SafetyRule_GatingRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Route53RecoveryControl::SafetyRule.GatingRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_route53recoverycontrol_SafetyRule_GatingRule as GatingRule;
    impl crate::value::ToValue for GatingRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "GatingControls".to_string(),
                crate::value::ToValue::to_value(&self.gating_controls),
            );
            properties.insert(
                "TargetControls".to_string(),
                crate::value::ToValue::to_value(&self.target_controls),
            );
            properties.insert(
                "WaitPeriodMs".to_string(),
                crate::value::ToValue::to_value(&self.wait_period_ms),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoverycontrol-safetyrule-ruleconfig.html
    pub struct RuleConfig_ {
        pub inverted: crate::value::ExpBool,
        pub threshold: i64,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_route53recoverycontrol_SafetyRule_RuleConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Route53RecoveryControl::SafetyRule.RuleConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_route53recoverycontrol_SafetyRule_RuleConfig as RuleConfig;
    impl crate::value::ToValue for RuleConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Inverted".to_string(),
                crate::value::ToValue::to_value(&self.inverted),
            );
            properties.insert(
                "Threshold".to_string(),
                crate::value::ToValue::to_value(&self.threshold),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoverycontrol-cluster.html
pub struct Cluster_ {
    pub name: crate::value::ExpString,
    pub network_type: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_route53recoverycontrol_Cluster {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Route53RecoveryControl::Cluster"
        $($field $value)*)
    };
}
pub use crate::__aws_route53recoverycontrol_Cluster as Cluster;
impl crate::template::ToResource for Cluster_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Route53RecoveryControl"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Cluster"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.network_type {
            properties.insert(
                "NetworkType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoverycontrol-controlpanel.html
pub struct ControlPanel_ {
    pub cluster_arn: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_route53recoverycontrol_ControlPanel {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Route53RecoveryControl::ControlPanel"
        $($field $value)*)
    };
}
pub use crate::__aws_route53recoverycontrol_ControlPanel as ControlPanel;
impl crate::template::ToResource for ControlPanel_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Route53RecoveryControl"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ControlPanel"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.cluster_arn {
            properties.insert(
                "ClusterArn".to_string(),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoverycontrol-routingcontrol.html
pub struct RoutingControl_ {
    pub cluster_arn: Option<crate::value::ExpString>,
    pub control_panel_arn: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_route53recoverycontrol_RoutingControl {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Route53RecoveryControl::RoutingControl"
        $($field $value)*)
    };
}
pub use crate::__aws_route53recoverycontrol_RoutingControl as RoutingControl;
impl crate::template::ToResource for RoutingControl_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Route53RecoveryControl"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RoutingControl"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.cluster_arn {
            properties.insert(
                "ClusterArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.control_panel_arn {
            properties.insert(
                "ControlPanelArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoverycontrol-safetyrule.html
pub struct SafetyRule_ {
    pub assertion_rule: Option<super::route53recoverycontrol::safetyrule::AssertionRule_>,
    pub control_panel_arn: crate::value::ExpString,
    pub gating_rule: Option<super::route53recoverycontrol::safetyrule::GatingRule_>,
    pub name: crate::value::ExpString,
    pub rule_config: super::route53recoverycontrol::safetyrule::RuleConfig_,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_route53recoverycontrol_SafetyRule {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Route53RecoveryControl::SafetyRule"
        $($field $value)*)
    };
}
pub use crate::__aws_route53recoverycontrol_SafetyRule as SafetyRule;
impl crate::template::ToResource for SafetyRule_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Route53RecoveryControl"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SafetyRule"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.assertion_rule {
            properties.insert(
                "AssertionRule".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ControlPanelArn".to_string(),
            crate::value::ToValue::to_value(&self.control_panel_arn),
        );
        if let Some(ref value) = self.gating_rule {
            properties.insert(
                "GatingRule".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "RuleConfig".to_string(),
            crate::value::ToValue::to_value(&self.rule_config),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
