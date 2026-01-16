pub mod firewall {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewall-availabilityzonemapping.html
    pub struct AvailabilityZoneMapping_ {
        pub availability_zone: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_Firewall_AvailabilityZoneMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::Firewall.AvailabilityZoneMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_Firewall_AvailabilityZoneMapping as AvailabilityZoneMapping;
    impl crate::value::ToValue for AvailabilityZoneMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AvailabilityZone".to_string(),
                crate::value::ToValue::to_value(&self.availability_zone),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewall-subnetmapping.html
    pub struct SubnetMapping_ {
        pub ip_address_type: Option<crate::value::ExpString>,
        pub subnet_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_Firewall_SubnetMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::Firewall.SubnetMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_Firewall_SubnetMapping as SubnetMapping;
    impl crate::value::ToValue for SubnetMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ip_address_type {
                properties.insert(
                    "IPAddressType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SubnetId".to_string(),
                crate::value::ToValue::to_value(&self.subnet_id),
            );
            properties.into()
        }
    }
}
pub mod firewallpolicy {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-actiondefinition.html
    pub struct ActionDefinition_ {
        pub publish_metric_action: Option<Box<PublishMetricAction_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_FirewallPolicy_ActionDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::FirewallPolicy.ActionDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_FirewallPolicy_ActionDefinition as ActionDefinition;
    impl crate::value::ToValue for ActionDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.publish_metric_action {
                properties.insert(
                    "PublishMetricAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-customaction.html
    pub struct CustomAction_ {
        pub action_definition: Box<ActionDefinition_>,
        pub action_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_FirewallPolicy_CustomAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::FirewallPolicy.CustomAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_FirewallPolicy_CustomAction as CustomAction;
    impl crate::value::ToValue for CustomAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ActionDefinition".to_string(),
                crate::value::ToValue::to_value(&self.action_definition),
            );
            properties.insert(
                "ActionName".to_string(),
                crate::value::ToValue::to_value(&self.action_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-dimension.html
    pub struct Dimension_ {
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_FirewallPolicy_Dimension {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::FirewallPolicy.Dimension"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_FirewallPolicy_Dimension as Dimension;
    impl crate::value::ToValue for Dimension_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-firewallpolicy.html
    pub struct FirewallPolicy_ {
        pub policy_variables: Option<Box<PolicyVariables_>>,
        pub stateful_default_actions: Option<Vec<crate::value::ExpString>>,
        pub stateful_engine_options: Option<Box<StatefulEngineOptions_>>,
        pub stateful_rule_group_references: Option<Vec<StatefulRuleGroupReference_>>,
        pub stateless_custom_actions: Option<Vec<CustomAction_>>,
        pub stateless_default_actions: Vec<crate::value::ExpString>,
        pub stateless_fragment_default_actions: Vec<crate::value::ExpString>,
        pub stateless_rule_group_references: Option<Vec<StatelessRuleGroupReference_>>,
        pub tls_inspection_configuration_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_FirewallPolicy_FirewallPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::FirewallPolicy.FirewallPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_FirewallPolicy_FirewallPolicy as FirewallPolicy;
    impl crate::value::ToValue for FirewallPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.policy_variables {
                properties.insert(
                    "PolicyVariables".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stateful_default_actions {
                properties.insert(
                    "StatefulDefaultActions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stateful_engine_options {
                properties.insert(
                    "StatefulEngineOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stateful_rule_group_references {
                properties.insert(
                    "StatefulRuleGroupReferences".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stateless_custom_actions {
                properties.insert(
                    "StatelessCustomActions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "StatelessDefaultActions".to_string(),
                crate::value::ToValue::to_value(&self.stateless_default_actions),
            );
            properties.insert(
                "StatelessFragmentDefaultActions".to_string(),
                crate::value::ToValue::to_value(&self.stateless_fragment_default_actions),
            );
            if let Some(ref value) = self.stateless_rule_group_references {
                properties.insert(
                    "StatelessRuleGroupReferences".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tls_inspection_configuration_arn {
                properties.insert(
                    "TLSInspectionConfigurationArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-flowtimeouts.html
    pub struct FlowTimeouts_ {
        pub tcp_idle_timeout_seconds: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_FirewallPolicy_FlowTimeouts {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::FirewallPolicy.FlowTimeouts"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_FirewallPolicy_FlowTimeouts as FlowTimeouts;
    impl crate::value::ToValue for FlowTimeouts_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.tcp_idle_timeout_seconds {
                properties.insert(
                    "TcpIdleTimeoutSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-ipset.html
    pub struct IPSet_ {
        pub definition: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_FirewallPolicy_IPSet {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::FirewallPolicy.IPSet"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_FirewallPolicy_IPSet as IPSet;
    impl crate::value::ToValue for IPSet_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.definition {
                properties.insert(
                    "Definition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-policyvariables.html
    pub struct PolicyVariables_ {
        pub rule_variables: Option<std::collections::BTreeMap<String, IPSet_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_FirewallPolicy_PolicyVariables {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::FirewallPolicy.PolicyVariables"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_FirewallPolicy_PolicyVariables as PolicyVariables;
    impl crate::value::ToValue for PolicyVariables_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.rule_variables {
                properties.insert(
                    "RuleVariables".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-publishmetricaction.html
    pub struct PublishMetricAction_ {
        pub dimensions: Vec<Dimension_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_FirewallPolicy_PublishMetricAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::FirewallPolicy.PublishMetricAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_FirewallPolicy_PublishMetricAction as PublishMetricAction;
    impl crate::value::ToValue for PublishMetricAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Dimensions".to_string(),
                crate::value::ToValue::to_value(&self.dimensions),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-statefulengineoptions.html
    pub struct StatefulEngineOptions_ {
        pub flow_timeouts: Option<Box<FlowTimeouts_>>,
        pub rule_order: Option<crate::value::ExpString>,
        pub stream_exception_policy: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_FirewallPolicy_StatefulEngineOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::FirewallPolicy.StatefulEngineOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_FirewallPolicy_StatefulEngineOptions as StatefulEngineOptions;
    impl crate::value::ToValue for StatefulEngineOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.flow_timeouts {
                properties.insert(
                    "FlowTimeouts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rule_order {
                properties.insert(
                    "RuleOrder".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stream_exception_policy {
                properties.insert(
                    "StreamExceptionPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-statefulrulegroupoverride.html
    pub struct StatefulRuleGroupOverride_ {
        pub action: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_FirewallPolicy_StatefulRuleGroupOverride {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::FirewallPolicy.StatefulRuleGroupOverride"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_FirewallPolicy_StatefulRuleGroupOverride as StatefulRuleGroupOverride;
    impl crate::value::ToValue for StatefulRuleGroupOverride_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.action {
                properties.insert("Action".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-statefulrulegroupreference.html
    pub struct StatefulRuleGroupReference_ {
        pub deep_threat_inspection: Option<crate::value::ExpBool>,
        pub r#override: Option<Box<StatefulRuleGroupOverride_>>,
        pub priority: Option<i64>,
        pub resource_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_FirewallPolicy_StatefulRuleGroupReference {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::FirewallPolicy.StatefulRuleGroupReference"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_FirewallPolicy_StatefulRuleGroupReference as StatefulRuleGroupReference;
    impl crate::value::ToValue for StatefulRuleGroupReference_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.deep_threat_inspection {
                properties.insert(
                    "DeepThreatInspection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.r#override {
                properties.insert(
                    "Override".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.priority {
                properties.insert(
                    "Priority".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ResourceArn".to_string(),
                crate::value::ToValue::to_value(&self.resource_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-statelessrulegroupreference.html
    pub struct StatelessRuleGroupReference_ {
        pub priority: i64,
        pub resource_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_FirewallPolicy_StatelessRuleGroupReference {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::FirewallPolicy.StatelessRuleGroupReference"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_FirewallPolicy_StatelessRuleGroupReference as StatelessRuleGroupReference;
    impl crate::value::ToValue for StatelessRuleGroupReference_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Priority".to_string(),
                crate::value::ToValue::to_value(&self.priority),
            );
            properties.insert(
                "ResourceArn".to_string(),
                crate::value::ToValue::to_value(&self.resource_arn),
            );
            properties.into()
        }
    }
}
pub mod loggingconfiguration {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-loggingconfiguration-logdestinationconfig.html
    pub struct LogDestinationConfig_ {
        pub log_destination: std::collections::BTreeMap<String, crate::value::ExpString>,
        pub log_destination_type: crate::value::ExpString,
        pub log_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_LoggingConfiguration_LogDestinationConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::LoggingConfiguration.LogDestinationConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_LoggingConfiguration_LogDestinationConfig as LogDestinationConfig;
    impl crate::value::ToValue for LogDestinationConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LogDestination".to_string(),
                crate::value::ToValue::to_value(&self.log_destination),
            );
            properties.insert(
                "LogDestinationType".to_string(),
                crate::value::ToValue::to_value(&self.log_destination_type),
            );
            properties.insert(
                "LogType".to_string(),
                crate::value::ToValue::to_value(&self.log_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-loggingconfiguration-loggingconfiguration.html
    pub struct LoggingConfiguration_ {
        pub log_destination_configs: Vec<LogDestinationConfig_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_LoggingConfiguration_LoggingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::LoggingConfiguration.LoggingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_LoggingConfiguration_LoggingConfiguration as LoggingConfiguration;
    impl crate::value::ToValue for LoggingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LogDestinationConfigs".to_string(),
                crate::value::ToValue::to_value(&self.log_destination_configs),
            );
            properties.into()
        }
    }
}
pub mod rulegroup {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-actiondefinition.html
    pub struct ActionDefinition_ {
        pub publish_metric_action: Option<Box<PublishMetricAction_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_RuleGroup_ActionDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::RuleGroup.ActionDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_RuleGroup_ActionDefinition as ActionDefinition;
    impl crate::value::ToValue for ActionDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.publish_metric_action {
                properties.insert(
                    "PublishMetricAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-address.html
    pub struct Address_ {
        pub address_definition: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_RuleGroup_Address {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::RuleGroup.Address"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_RuleGroup_Address as Address;
    impl crate::value::ToValue for Address_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AddressDefinition".to_string(),
                crate::value::ToValue::to_value(&self.address_definition),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-customaction.html
    pub struct CustomAction_ {
        pub action_definition: Box<ActionDefinition_>,
        pub action_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_RuleGroup_CustomAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::RuleGroup.CustomAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_RuleGroup_CustomAction as CustomAction;
    impl crate::value::ToValue for CustomAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ActionDefinition".to_string(),
                crate::value::ToValue::to_value(&self.action_definition),
            );
            properties.insert(
                "ActionName".to_string(),
                crate::value::ToValue::to_value(&self.action_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-dimension.html
    pub struct Dimension_ {
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_RuleGroup_Dimension {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::RuleGroup.Dimension"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_RuleGroup_Dimension as Dimension;
    impl crate::value::ToValue for Dimension_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-header.html
    pub struct Header_ {
        pub destination: crate::value::ExpString,
        pub destination_port: crate::value::ExpString,
        pub direction: crate::value::ExpString,
        pub protocol: crate::value::ExpString,
        pub source: crate::value::ExpString,
        pub source_port: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_RuleGroup_Header {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::RuleGroup.Header"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_RuleGroup_Header as Header;
    impl crate::value::ToValue for Header_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Destination".to_string(),
                crate::value::ToValue::to_value(&self.destination),
            );
            properties.insert(
                "DestinationPort".to_string(),
                crate::value::ToValue::to_value(&self.destination_port),
            );
            properties.insert(
                "Direction".to_string(),
                crate::value::ToValue::to_value(&self.direction),
            );
            properties.insert(
                "Protocol".to_string(),
                crate::value::ToValue::to_value(&self.protocol),
            );
            properties.insert(
                "Source".to_string(),
                crate::value::ToValue::to_value(&self.source),
            );
            properties.insert(
                "SourcePort".to_string(),
                crate::value::ToValue::to_value(&self.source_port),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-ipset.html
    pub struct IPSet_ {
        pub definition: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_RuleGroup_IPSet {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::RuleGroup.IPSet"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_RuleGroup_IPSet as IPSet;
    impl crate::value::ToValue for IPSet_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.definition {
                properties.insert(
                    "Definition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-ipsetreference.html
    pub struct IPSetReference_ {
        pub reference_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_RuleGroup_IPSetReference {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::RuleGroup.IPSetReference"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_RuleGroup_IPSetReference as IPSetReference;
    impl crate::value::ToValue for IPSetReference_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.reference_arn {
                properties.insert(
                    "ReferenceArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-matchattributes.html
    pub struct MatchAttributes_ {
        pub destination_ports: Option<Vec<PortRange_>>,
        pub destinations: Option<Vec<Address_>>,
        pub protocols: Option<Vec<i64>>,
        pub source_ports: Option<Vec<PortRange_>>,
        pub sources: Option<Vec<Address_>>,
        pub tcp_flags: Option<Vec<TCPFlagField_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_RuleGroup_MatchAttributes {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::RuleGroup.MatchAttributes"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_RuleGroup_MatchAttributes as MatchAttributes;
    impl crate::value::ToValue for MatchAttributes_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.destination_ports {
                properties.insert(
                    "DestinationPorts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.destinations {
                properties.insert(
                    "Destinations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.protocols {
                properties.insert(
                    "Protocols".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_ports {
                properties.insert(
                    "SourcePorts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sources {
                properties.insert(
                    "Sources".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tcp_flags {
                properties.insert(
                    "TCPFlags".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-portrange.html
    pub struct PortRange_ {
        pub from_port: i64,
        pub to_port: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_RuleGroup_PortRange {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::RuleGroup.PortRange"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_RuleGroup_PortRange as PortRange;
    impl crate::value::ToValue for PortRange_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FromPort".to_string(),
                crate::value::ToValue::to_value(&self.from_port),
            );
            properties.insert(
                "ToPort".to_string(),
                crate::value::ToValue::to_value(&self.to_port),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-portset.html
    pub struct PortSet_ {
        pub definition: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_RuleGroup_PortSet {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::RuleGroup.PortSet"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_RuleGroup_PortSet as PortSet;
    impl crate::value::ToValue for PortSet_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.definition {
                properties.insert(
                    "Definition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-publishmetricaction.html
    pub struct PublishMetricAction_ {
        pub dimensions: Vec<Dimension_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_RuleGroup_PublishMetricAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::RuleGroup.PublishMetricAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_RuleGroup_PublishMetricAction as PublishMetricAction;
    impl crate::value::ToValue for PublishMetricAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Dimensions".to_string(),
                crate::value::ToValue::to_value(&self.dimensions),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-referencesets.html
    pub struct ReferenceSets_ {
        pub ip_set_references: Option<std::collections::BTreeMap<String, IPSetReference_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_RuleGroup_ReferenceSets {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::RuleGroup.ReferenceSets"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_RuleGroup_ReferenceSets as ReferenceSets;
    impl crate::value::ToValue for ReferenceSets_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ip_set_references {
                properties.insert(
                    "IPSetReferences".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-ruledefinition.html
    pub struct RuleDefinition_ {
        pub actions: Vec<crate::value::ExpString>,
        pub match_attributes: Box<MatchAttributes_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_RuleGroup_RuleDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::RuleGroup.RuleDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_RuleGroup_RuleDefinition as RuleDefinition;
    impl crate::value::ToValue for RuleDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Actions".to_string(),
                crate::value::ToValue::to_value(&self.actions),
            );
            properties.insert(
                "MatchAttributes".to_string(),
                crate::value::ToValue::to_value(&self.match_attributes),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-rulegroup.html
    pub struct RuleGroup_ {
        pub reference_sets: Option<Box<ReferenceSets_>>,
        pub rule_variables: Option<Box<RuleVariables_>>,
        pub rules_source: Box<RulesSource_>,
        pub stateful_rule_options: Option<Box<StatefulRuleOptions_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_RuleGroup_RuleGroup {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::RuleGroup.RuleGroup"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_RuleGroup_RuleGroup as RuleGroup;
    impl crate::value::ToValue for RuleGroup_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.reference_sets {
                properties.insert(
                    "ReferenceSets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rule_variables {
                properties.insert(
                    "RuleVariables".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RulesSource".to_string(),
                crate::value::ToValue::to_value(&self.rules_source),
            );
            if let Some(ref value) = self.stateful_rule_options {
                properties.insert(
                    "StatefulRuleOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-ruleoption.html
    pub struct RuleOption_ {
        pub keyword: crate::value::ExpString,
        pub settings: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_RuleGroup_RuleOption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::RuleGroup.RuleOption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_RuleGroup_RuleOption as RuleOption;
    impl crate::value::ToValue for RuleOption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Keyword".to_string(),
                crate::value::ToValue::to_value(&self.keyword),
            );
            if let Some(ref value) = self.settings {
                properties.insert(
                    "Settings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-rulevariables.html
    pub struct RuleVariables_ {
        pub ip_sets: Option<std::collections::BTreeMap<String, IPSet_>>,
        pub port_sets: Option<std::collections::BTreeMap<String, PortSet_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_RuleGroup_RuleVariables {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::RuleGroup.RuleVariables"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_RuleGroup_RuleVariables as RuleVariables;
    impl crate::value::ToValue for RuleVariables_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ip_sets {
                properties.insert("IPSets".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.port_sets {
                properties.insert(
                    "PortSets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-rulessource.html
    pub struct RulesSource_ {
        pub rules_source_list: Option<Box<RulesSourceList_>>,
        pub rules_string: Option<crate::value::ExpString>,
        pub stateful_rules: Option<Vec<StatefulRule_>>,
        pub stateless_rules_and_custom_actions: Option<Box<StatelessRulesAndCustomActions_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_RuleGroup_RulesSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::RuleGroup.RulesSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_RuleGroup_RulesSource as RulesSource;
    impl crate::value::ToValue for RulesSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.rules_source_list {
                properties.insert(
                    "RulesSourceList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rules_string {
                properties.insert(
                    "RulesString".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stateful_rules {
                properties.insert(
                    "StatefulRules".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stateless_rules_and_custom_actions {
                properties.insert(
                    "StatelessRulesAndCustomActions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-rulessourcelist.html
    pub struct RulesSourceList_ {
        pub generated_rules_type: crate::value::ExpString,
        pub target_types: Vec<crate::value::ExpString>,
        pub targets: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_RuleGroup_RulesSourceList {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::RuleGroup.RulesSourceList"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_RuleGroup_RulesSourceList as RulesSourceList;
    impl crate::value::ToValue for RulesSourceList_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "GeneratedRulesType".to_string(),
                crate::value::ToValue::to_value(&self.generated_rules_type),
            );
            properties.insert(
                "TargetTypes".to_string(),
                crate::value::ToValue::to_value(&self.target_types),
            );
            properties.insert(
                "Targets".to_string(),
                crate::value::ToValue::to_value(&self.targets),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-statefulrule.html
    pub struct StatefulRule_ {
        pub action: crate::value::ExpString,
        pub header: Box<Header_>,
        pub rule_options: Vec<RuleOption_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_RuleGroup_StatefulRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::RuleGroup.StatefulRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_RuleGroup_StatefulRule as StatefulRule;
    impl crate::value::ToValue for StatefulRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Action".to_string(),
                crate::value::ToValue::to_value(&self.action),
            );
            properties.insert(
                "Header".to_string(),
                crate::value::ToValue::to_value(&self.header),
            );
            properties.insert(
                "RuleOptions".to_string(),
                crate::value::ToValue::to_value(&self.rule_options),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-statefulruleoptions.html
    pub struct StatefulRuleOptions_ {
        pub rule_order: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_RuleGroup_StatefulRuleOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::RuleGroup.StatefulRuleOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_RuleGroup_StatefulRuleOptions as StatefulRuleOptions;
    impl crate::value::ToValue for StatefulRuleOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.rule_order {
                properties.insert(
                    "RuleOrder".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-statelessrule.html
    pub struct StatelessRule_ {
        pub priority: i64,
        pub rule_definition: Box<RuleDefinition_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_RuleGroup_StatelessRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::RuleGroup.StatelessRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_RuleGroup_StatelessRule as StatelessRule;
    impl crate::value::ToValue for StatelessRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Priority".to_string(),
                crate::value::ToValue::to_value(&self.priority),
            );
            properties.insert(
                "RuleDefinition".to_string(),
                crate::value::ToValue::to_value(&self.rule_definition),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-statelessrulesandcustomactions.html
    pub struct StatelessRulesAndCustomActions_ {
        pub custom_actions: Option<Vec<CustomAction_>>,
        pub stateless_rules: Vec<StatelessRule_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_RuleGroup_StatelessRulesAndCustomActions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::RuleGroup.StatelessRulesAndCustomActions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_RuleGroup_StatelessRulesAndCustomActions as StatelessRulesAndCustomActions;
    impl crate::value::ToValue for StatelessRulesAndCustomActions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_actions {
                properties.insert(
                    "CustomActions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "StatelessRules".to_string(),
                crate::value::ToValue::to_value(&self.stateless_rules),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-summaryconfiguration.html
    pub struct SummaryConfiguration_ {
        pub rule_options: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_RuleGroup_SummaryConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::RuleGroup.SummaryConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_RuleGroup_SummaryConfiguration as SummaryConfiguration;
    impl crate::value::ToValue for SummaryConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.rule_options {
                properties.insert(
                    "RuleOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-tcpflagfield.html
    pub struct TCPFlagField_ {
        pub flags: Vec<crate::value::ExpString>,
        pub masks: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_RuleGroup_TCPFlagField {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::RuleGroup.TCPFlagField"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_RuleGroup_TCPFlagField as TCPFlagField;
    impl crate::value::ToValue for TCPFlagField_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Flags".to_string(),
                crate::value::ToValue::to_value(&self.flags),
            );
            if let Some(ref value) = self.masks {
                properties.insert("Masks".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod tlsinspectionconfiguration {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-tlsinspectionconfiguration-address.html
    pub struct Address_ {
        pub address_definition: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_TLSInspectionConfiguration_Address {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::TLSInspectionConfiguration.Address"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_TLSInspectionConfiguration_Address as Address;
    impl crate::value::ToValue for Address_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AddressDefinition".to_string(),
                crate::value::ToValue::to_value(&self.address_definition),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-tlsinspectionconfiguration-checkcertificaterevocationstatus.html
    pub struct CheckCertificateRevocationStatus_ {
        pub revoked_status_action: Option<crate::value::ExpString>,
        pub unknown_status_action: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_TLSInspectionConfiguration_CheckCertificateRevocationStatus {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::TLSInspectionConfiguration.CheckCertificateRevocationStatus"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_TLSInspectionConfiguration_CheckCertificateRevocationStatus as CheckCertificateRevocationStatus;
    impl crate::value::ToValue for CheckCertificateRevocationStatus_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.revoked_status_action {
                properties.insert(
                    "RevokedStatusAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.unknown_status_action {
                properties.insert(
                    "UnknownStatusAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-tlsinspectionconfiguration-portrange.html
    pub struct PortRange_ {
        pub from_port: i64,
        pub to_port: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_TLSInspectionConfiguration_PortRange {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::TLSInspectionConfiguration.PortRange"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_TLSInspectionConfiguration_PortRange as PortRange;
    impl crate::value::ToValue for PortRange_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FromPort".to_string(),
                crate::value::ToValue::to_value(&self.from_port),
            );
            properties.insert(
                "ToPort".to_string(),
                crate::value::ToValue::to_value(&self.to_port),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-tlsinspectionconfiguration-servercertificate.html
    pub struct ServerCertificate_ {
        pub resource_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_TLSInspectionConfiguration_ServerCertificate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::TLSInspectionConfiguration.ServerCertificate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_TLSInspectionConfiguration_ServerCertificate as ServerCertificate;
    impl crate::value::ToValue for ServerCertificate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.resource_arn {
                properties.insert(
                    "ResourceArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-tlsinspectionconfiguration-servercertificateconfiguration.html
    pub struct ServerCertificateConfiguration_ {
        pub certificate_authority_arn: Option<crate::value::ExpString>,
        pub check_certificate_revocation_status: Option<Box<CheckCertificateRevocationStatus_>>,
        pub scopes: Option<Vec<ServerCertificateScope_>>,
        pub server_certificates: Option<Vec<ServerCertificate_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_TLSInspectionConfiguration_ServerCertificateConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::TLSInspectionConfiguration.ServerCertificateConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_TLSInspectionConfiguration_ServerCertificateConfiguration as ServerCertificateConfiguration;
    impl crate::value::ToValue for ServerCertificateConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.certificate_authority_arn {
                properties.insert(
                    "CertificateAuthorityArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.check_certificate_revocation_status {
                properties.insert(
                    "CheckCertificateRevocationStatus".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scopes {
                properties.insert("Scopes".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.server_certificates {
                properties.insert(
                    "ServerCertificates".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-tlsinspectionconfiguration-servercertificatescope.html
    pub struct ServerCertificateScope_ {
        pub destination_ports: Option<Vec<PortRange_>>,
        pub destinations: Option<Vec<Address_>>,
        pub protocols: Option<Vec<i64>>,
        pub source_ports: Option<Vec<PortRange_>>,
        pub sources: Option<Vec<Address_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_TLSInspectionConfiguration_ServerCertificateScope {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::TLSInspectionConfiguration.ServerCertificateScope"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_TLSInspectionConfiguration_ServerCertificateScope as ServerCertificateScope;
    impl crate::value::ToValue for ServerCertificateScope_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.destination_ports {
                properties.insert(
                    "DestinationPorts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.destinations {
                properties.insert(
                    "Destinations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.protocols {
                properties.insert(
                    "Protocols".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_ports {
                properties.insert(
                    "SourcePorts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sources {
                properties.insert(
                    "Sources".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-tlsinspectionconfiguration-tlsinspectionconfiguration.html
    pub struct TLSInspectionConfiguration_ {
        pub server_certificate_configurations: Option<Vec<ServerCertificateConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_TLSInspectionConfiguration_TLSInspectionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::TLSInspectionConfiguration.TLSInspectionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_TLSInspectionConfiguration_TLSInspectionConfiguration as TLSInspectionConfiguration;
    impl crate::value::ToValue for TLSInspectionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.server_certificate_configurations {
                properties.insert(
                    "ServerCertificateConfigurations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod vpcendpointassociation {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-vpcendpointassociation-subnetmapping.html
    pub struct SubnetMapping_ {
        pub ip_address_type: Option<crate::value::ExpString>,
        pub subnet_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_networkfirewall_VpcEndpointAssociation_SubnetMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NetworkFirewall::VpcEndpointAssociation.SubnetMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_networkfirewall_VpcEndpointAssociation_SubnetMapping as SubnetMapping;
    impl crate::value::ToValue for SubnetMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ip_address_type {
                properties.insert(
                    "IPAddressType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SubnetId".to_string(),
                crate::value::ToValue::to_value(&self.subnet_id),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkfirewall-firewall.html
pub struct Firewall_ {
    pub availability_zone_change_protection: Option<crate::value::ExpBool>,
    pub availability_zone_mappings:
        Option<Vec<super::networkfirewall::firewall::AvailabilityZoneMapping_>>,
    pub delete_protection: Option<crate::value::ExpBool>,
    pub description: Option<crate::value::ExpString>,
    pub enabled_analysis_types: Option<Vec<crate::value::ExpString>>,
    pub firewall_name: crate::value::ExpString,
    pub firewall_policy_arn: crate::value::ExpString,
    pub firewall_policy_change_protection: Option<crate::value::ExpBool>,
    pub subnet_change_protection: Option<crate::value::ExpBool>,
    pub subnet_mappings: Option<Vec<super::networkfirewall::firewall::SubnetMapping_>>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub transit_gateway_id: Option<crate::value::ExpString>,
    pub vpc_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_networkfirewall_Firewall {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::NetworkFirewall::Firewall"
        $($field $value)*)
    };
}
pub use crate::__aws_networkfirewall_Firewall as Firewall;
impl crate::template::ToResource for Firewall_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("NetworkFirewall"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Firewall"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.availability_zone_change_protection {
            properties.insert(
                "AvailabilityZoneChangeProtection".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.availability_zone_mappings {
            properties.insert(
                "AvailabilityZoneMappings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.delete_protection {
            properties.insert(
                "DeleteProtection".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enabled_analysis_types {
            properties.insert(
                "EnabledAnalysisTypes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "FirewallName".to_string(),
            crate::value::ToValue::to_value(&self.firewall_name),
        );
        properties.insert(
            "FirewallPolicyArn".to_string(),
            crate::value::ToValue::to_value(&self.firewall_policy_arn),
        );
        if let Some(ref value) = self.firewall_policy_change_protection {
            properties.insert(
                "FirewallPolicyChangeProtection".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.subnet_change_protection {
            properties.insert(
                "SubnetChangeProtection".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.subnet_mappings {
            properties.insert(
                "SubnetMappings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.transit_gateway_id {
            properties.insert(
                "TransitGatewayId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.vpc_id {
            properties.insert("VpcId".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkfirewall-firewallpolicy.html
pub struct FirewallPolicy_ {
    pub description: Option<crate::value::ExpString>,
    pub firewall_policy: super::networkfirewall::firewallpolicy::FirewallPolicy_,
    pub firewall_policy_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_networkfirewall_FirewallPolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::NetworkFirewall::FirewallPolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_networkfirewall_FirewallPolicy as FirewallPolicy;
impl crate::template::ToResource for FirewallPolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("NetworkFirewall"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("FirewallPolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "FirewallPolicy".to_string(),
            crate::value::ToValue::to_value(&self.firewall_policy),
        );
        properties.insert(
            "FirewallPolicyName".to_string(),
            crate::value::ToValue::to_value(&self.firewall_policy_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkfirewall-loggingconfiguration.html
pub struct LoggingConfiguration_ {
    pub enable_monitoring_dashboard: Option<crate::value::ExpBool>,
    pub firewall_arn: crate::value::ExpString,
    pub firewall_name: Option<crate::value::ExpString>,
    pub logging_configuration: super::networkfirewall::loggingconfiguration::LoggingConfiguration_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_networkfirewall_LoggingConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::NetworkFirewall::LoggingConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_networkfirewall_LoggingConfiguration as LoggingConfiguration;
impl crate::template::ToResource for LoggingConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("NetworkFirewall"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LoggingConfiguration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.enable_monitoring_dashboard {
            properties.insert(
                "EnableMonitoringDashboard".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "FirewallArn".to_string(),
            crate::value::ToValue::to_value(&self.firewall_arn),
        );
        if let Some(ref value) = self.firewall_name {
            properties.insert(
                "FirewallName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "LoggingConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.logging_configuration),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkfirewall-rulegroup.html
pub struct RuleGroup_ {
    pub capacity: i64,
    pub description: Option<crate::value::ExpString>,
    pub rule_group: Option<super::networkfirewall::rulegroup::RuleGroup_>,
    pub rule_group_name: crate::value::ExpString,
    pub summary_configuration: Option<super::networkfirewall::rulegroup::SummaryConfiguration_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_networkfirewall_RuleGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::NetworkFirewall::RuleGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_networkfirewall_RuleGroup as RuleGroup;
impl crate::template::ToResource for RuleGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("NetworkFirewall"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RuleGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Capacity".to_string(),
            crate::value::ToValue::to_value(&self.capacity),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.rule_group {
            properties.insert(
                "RuleGroup".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RuleGroupName".to_string(),
            crate::value::ToValue::to_value(&self.rule_group_name),
        );
        if let Some(ref value) = self.summary_configuration {
            properties.insert(
                "SummaryConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkfirewall-tlsinspectionconfiguration.html
pub struct TLSInspectionConfiguration_ {
    pub description: Option<crate::value::ExpString>,
    pub tls_inspection_configuration:
        super::networkfirewall::tlsinspectionconfiguration::TLSInspectionConfiguration_,
    pub tls_inspection_configuration_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_networkfirewall_TLSInspectionConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::NetworkFirewall::TLSInspectionConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_networkfirewall_TLSInspectionConfiguration as TLSInspectionConfiguration;
impl crate::template::ToResource for TLSInspectionConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("NetworkFirewall"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "TLSInspectionConfiguration",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "TLSInspectionConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.tls_inspection_configuration),
        );
        properties.insert(
            "TLSInspectionConfigurationName".to_string(),
            crate::value::ToValue::to_value(&self.tls_inspection_configuration_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkfirewall-vpcendpointassociation.html
pub struct VpcEndpointAssociation_ {
    pub description: Option<crate::value::ExpString>,
    pub firewall_arn: crate::value::ExpString,
    pub subnet_mapping: super::networkfirewall::vpcendpointassociation::SubnetMapping_,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_networkfirewall_VpcEndpointAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::NetworkFirewall::VpcEndpointAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_networkfirewall_VpcEndpointAssociation as VpcEndpointAssociation;
impl crate::template::ToResource for VpcEndpointAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("NetworkFirewall"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("VpcEndpointAssociation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "FirewallArn".to_string(),
            crate::value::ToValue::to_value(&self.firewall_arn),
        );
        properties.insert(
            "SubnetMapping".to_string(),
            crate::value::ToValue::to_value(&self.subnet_mapping),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "VpcId".to_string(),
            crate::value::ToValue::to_value(&self.vpc_id),
        );
        properties
    }
}
