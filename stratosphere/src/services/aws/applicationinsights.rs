pub mod application {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-alarm.html
    pub struct Alarm_ {
        pub alarm_name: crate::value::ExpString,
        pub severity: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationinsights_Application_Alarm {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationInsights::Application.Alarm"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationinsights_Application_Alarm as Alarm;
    impl crate::value::ToValue for Alarm_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AlarmName".to_string(),
                crate::value::ToValue::to_value(&self.alarm_name),
            );
            if let Some(ref value) = self.severity {
                properties.insert(
                    "Severity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-alarmmetric.html
    pub struct AlarmMetric_ {
        pub alarm_metric_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationinsights_Application_AlarmMetric {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationInsights::Application.AlarmMetric"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationinsights_Application_AlarmMetric as AlarmMetric;
    impl crate::value::ToValue for AlarmMetric_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AlarmMetricName".to_string(),
                crate::value::ToValue::to_value(&self.alarm_metric_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-componentconfiguration.html
    pub struct ComponentConfiguration_ {
        pub configuration_details: Option<Box<ConfigurationDetails_>>,
        pub sub_component_type_configurations: Option<Vec<SubComponentTypeConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationinsights_Application_ComponentConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationInsights::Application.ComponentConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationinsights_Application_ComponentConfiguration as ComponentConfiguration;
    impl crate::value::ToValue for ComponentConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.configuration_details {
                properties.insert(
                    "ConfigurationDetails".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sub_component_type_configurations {
                properties.insert(
                    "SubComponentTypeConfigurations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-componentmonitoringsetting.html
    pub struct ComponentMonitoringSetting_ {
        pub component_arn: Option<crate::value::ExpString>,
        pub component_configuration_mode: crate::value::ExpString,
        pub component_name: Option<crate::value::ExpString>,
        pub custom_component_configuration: Option<Box<ComponentConfiguration_>>,
        pub default_overwrite_component_configuration: Option<Box<ComponentConfiguration_>>,
        pub tier: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationinsights_Application_ComponentMonitoringSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationInsights::Application.ComponentMonitoringSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationinsights_Application_ComponentMonitoringSetting as ComponentMonitoringSetting;
    impl crate::value::ToValue for ComponentMonitoringSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.component_arn {
                properties.insert(
                    "ComponentARN".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ComponentConfigurationMode".to_string(),
                crate::value::ToValue::to_value(&self.component_configuration_mode),
            );
            if let Some(ref value) = self.component_name {
                properties.insert(
                    "ComponentName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_component_configuration {
                properties.insert(
                    "CustomComponentConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.default_overwrite_component_configuration {
                properties.insert(
                    "DefaultOverwriteComponentConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Tier".to_string(),
                crate::value::ToValue::to_value(&self.tier),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-configurationdetails.html
    pub struct ConfigurationDetails_ {
        pub alarm_metrics: Option<Vec<AlarmMetric_>>,
        pub alarms: Option<Vec<Alarm_>>,
        pub ha_cluster_prometheus_exporter: Option<Box<HAClusterPrometheusExporter_>>,
        pub hana_prometheus_exporter: Option<Box<HANAPrometheusExporter_>>,
        pub jmx_prometheus_exporter: Option<Box<JMXPrometheusExporter_>>,
        pub logs: Option<Vec<Log_>>,
        pub net_weaver_prometheus_exporter: Option<Box<NetWeaverPrometheusExporter_>>,
        pub processes: Option<Vec<Process_>>,
        pub sql_server_prometheus_exporter: Option<Box<SQLServerPrometheusExporter_>>,
        pub windows_events: Option<Vec<WindowsEvent_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationinsights_Application_ConfigurationDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationInsights::Application.ConfigurationDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationinsights_Application_ConfigurationDetails as ConfigurationDetails;
    impl crate::value::ToValue for ConfigurationDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.alarm_metrics {
                properties.insert(
                    "AlarmMetrics".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.alarms {
                properties.insert("Alarms".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.ha_cluster_prometheus_exporter {
                properties.insert(
                    "HAClusterPrometheusExporter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hana_prometheus_exporter {
                properties.insert(
                    "HANAPrometheusExporter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.jmx_prometheus_exporter {
                properties.insert(
                    "JMXPrometheusExporter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.logs {
                properties.insert("Logs".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.net_weaver_prometheus_exporter {
                properties.insert(
                    "NetWeaverPrometheusExporter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.processes {
                properties.insert(
                    "Processes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sql_server_prometheus_exporter {
                properties.insert(
                    "SQLServerPrometheusExporter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.windows_events {
                properties.insert(
                    "WindowsEvents".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-customcomponent.html
    pub struct CustomComponent_ {
        pub component_name: crate::value::ExpString,
        pub resource_list: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationinsights_Application_CustomComponent {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationInsights::Application.CustomComponent"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationinsights_Application_CustomComponent as CustomComponent;
    impl crate::value::ToValue for CustomComponent_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ComponentName".to_string(),
                crate::value::ToValue::to_value(&self.component_name),
            );
            properties.insert(
                "ResourceList".to_string(),
                crate::value::ToValue::to_value(&self.resource_list),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-haclusterprometheusexporter.html
    pub struct HAClusterPrometheusExporter_ {
        pub prometheus_port: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationinsights_Application_HAClusterPrometheusExporter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationInsights::Application.HAClusterPrometheusExporter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationinsights_Application_HAClusterPrometheusExporter as HAClusterPrometheusExporter;
    impl crate::value::ToValue for HAClusterPrometheusExporter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.prometheus_port {
                properties.insert(
                    "PrometheusPort".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-hanaprometheusexporter.html
    pub struct HANAPrometheusExporter_ {
        pub agree_to_install_hanadb_client: crate::value::ExpBool,
        pub hana_port: crate::value::ExpString,
        pub hanasid: crate::value::ExpString,
        pub hana_secret_name: crate::value::ExpString,
        pub prometheus_port: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationinsights_Application_HANAPrometheusExporter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationInsights::Application.HANAPrometheusExporter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationinsights_Application_HANAPrometheusExporter as HANAPrometheusExporter;
    impl crate::value::ToValue for HANAPrometheusExporter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AgreeToInstallHANADBClient".to_string(),
                crate::value::ToValue::to_value(&self.agree_to_install_hanadb_client),
            );
            properties.insert(
                "HANAPort".to_string(),
                crate::value::ToValue::to_value(&self.hana_port),
            );
            properties.insert(
                "HANASID".to_string(),
                crate::value::ToValue::to_value(&self.hanasid),
            );
            properties.insert(
                "HANASecretName".to_string(),
                crate::value::ToValue::to_value(&self.hana_secret_name),
            );
            if let Some(ref value) = self.prometheus_port {
                properties.insert(
                    "PrometheusPort".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-jmxprometheusexporter.html
    pub struct JMXPrometheusExporter_ {
        pub host_port: Option<crate::value::ExpString>,
        pub jmxurl: Option<crate::value::ExpString>,
        pub prometheus_port: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationinsights_Application_JMXPrometheusExporter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationInsights::Application.JMXPrometheusExporter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationinsights_Application_JMXPrometheusExporter as JMXPrometheusExporter;
    impl crate::value::ToValue for JMXPrometheusExporter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.host_port {
                properties.insert(
                    "HostPort".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.jmxurl {
                properties.insert("JMXURL".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.prometheus_port {
                properties.insert(
                    "PrometheusPort".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-log.html
    pub struct Log_ {
        pub encoding: Option<crate::value::ExpString>,
        pub log_group_name: Option<crate::value::ExpString>,
        pub log_path: Option<crate::value::ExpString>,
        pub log_type: crate::value::ExpString,
        pub pattern_set: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationinsights_Application_Log {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationInsights::Application.Log"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationinsights_Application_Log as Log;
    impl crate::value::ToValue for Log_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.encoding {
                properties.insert(
                    "Encoding".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.log_group_name {
                properties.insert(
                    "LogGroupName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.log_path {
                properties.insert(
                    "LogPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "LogType".to_string(),
                crate::value::ToValue::to_value(&self.log_type),
            );
            if let Some(ref value) = self.pattern_set {
                properties.insert(
                    "PatternSet".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-logpattern.html
    pub struct LogPattern_ {
        pub pattern: crate::value::ExpString,
        pub pattern_name: crate::value::ExpString,
        pub rank: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationinsights_Application_LogPattern {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationInsights::Application.LogPattern"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationinsights_Application_LogPattern as LogPattern;
    impl crate::value::ToValue for LogPattern_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Pattern".to_string(),
                crate::value::ToValue::to_value(&self.pattern),
            );
            properties.insert(
                "PatternName".to_string(),
                crate::value::ToValue::to_value(&self.pattern_name),
            );
            properties.insert(
                "Rank".to_string(),
                crate::value::ToValue::to_value(&self.rank),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-logpatternset.html
    pub struct LogPatternSet_ {
        pub log_patterns: Vec<LogPattern_>,
        pub pattern_set_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationinsights_Application_LogPatternSet {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationInsights::Application.LogPatternSet"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationinsights_Application_LogPatternSet as LogPatternSet;
    impl crate::value::ToValue for LogPatternSet_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LogPatterns".to_string(),
                crate::value::ToValue::to_value(&self.log_patterns),
            );
            properties.insert(
                "PatternSetName".to_string(),
                crate::value::ToValue::to_value(&self.pattern_set_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-netweaverprometheusexporter.html
    pub struct NetWeaverPrometheusExporter_ {
        pub instance_numbers: Vec<crate::value::ExpString>,
        pub prometheus_port: Option<crate::value::ExpString>,
        pub sapsid: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationinsights_Application_NetWeaverPrometheusExporter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationInsights::Application.NetWeaverPrometheusExporter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationinsights_Application_NetWeaverPrometheusExporter as NetWeaverPrometheusExporter;
    impl crate::value::ToValue for NetWeaverPrometheusExporter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InstanceNumbers".to_string(),
                crate::value::ToValue::to_value(&self.instance_numbers),
            );
            if let Some(ref value) = self.prometheus_port {
                properties.insert(
                    "PrometheusPort".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SAPSID".to_string(),
                crate::value::ToValue::to_value(&self.sapsid),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-process.html
    pub struct Process_ {
        pub alarm_metrics: Vec<AlarmMetric_>,
        pub process_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationinsights_Application_Process {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationInsights::Application.Process"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationinsights_Application_Process as Process;
    impl crate::value::ToValue for Process_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AlarmMetrics".to_string(),
                crate::value::ToValue::to_value(&self.alarm_metrics),
            );
            properties.insert(
                "ProcessName".to_string(),
                crate::value::ToValue::to_value(&self.process_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-sqlserverprometheusexporter.html
    pub struct SQLServerPrometheusExporter_ {
        pub prometheus_port: crate::value::ExpString,
        pub sql_secret_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationinsights_Application_SQLServerPrometheusExporter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationInsights::Application.SQLServerPrometheusExporter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationinsights_Application_SQLServerPrometheusExporter as SQLServerPrometheusExporter;
    impl crate::value::ToValue for SQLServerPrometheusExporter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PrometheusPort".to_string(),
                crate::value::ToValue::to_value(&self.prometheus_port),
            );
            properties.insert(
                "SQLSecretName".to_string(),
                crate::value::ToValue::to_value(&self.sql_secret_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-subcomponentconfigurationdetails.html
    pub struct SubComponentConfigurationDetails_ {
        pub alarm_metrics: Option<Vec<AlarmMetric_>>,
        pub logs: Option<Vec<Log_>>,
        pub processes: Option<Vec<Process_>>,
        pub windows_events: Option<Vec<WindowsEvent_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationinsights_Application_SubComponentConfigurationDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationInsights::Application.SubComponentConfigurationDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationinsights_Application_SubComponentConfigurationDetails as SubComponentConfigurationDetails;
    impl crate::value::ToValue for SubComponentConfigurationDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.alarm_metrics {
                properties.insert(
                    "AlarmMetrics".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.logs {
                properties.insert("Logs".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.processes {
                properties.insert(
                    "Processes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.windows_events {
                properties.insert(
                    "WindowsEvents".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-subcomponenttypeconfiguration.html
    pub struct SubComponentTypeConfiguration_ {
        pub sub_component_configuration_details: Box<SubComponentConfigurationDetails_>,
        pub sub_component_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationinsights_Application_SubComponentTypeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationInsights::Application.SubComponentTypeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationinsights_Application_SubComponentTypeConfiguration as SubComponentTypeConfiguration;
    impl crate::value::ToValue for SubComponentTypeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SubComponentConfigurationDetails".to_string(),
                crate::value::ToValue::to_value(&self.sub_component_configuration_details),
            );
            properties.insert(
                "SubComponentType".to_string(),
                crate::value::ToValue::to_value(&self.sub_component_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationinsights-application-windowsevent.html
    pub struct WindowsEvent_ {
        pub event_levels: Vec<crate::value::ExpString>,
        pub event_name: crate::value::ExpString,
        pub log_group_name: crate::value::ExpString,
        pub pattern_set: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_applicationinsights_Application_WindowsEvent {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApplicationInsights::Application.WindowsEvent"
            $($field $value)*)
        };
    }
    pub use crate::__aws_applicationinsights_Application_WindowsEvent as WindowsEvent;
    impl crate::value::ToValue for WindowsEvent_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EventLevels".to_string(),
                crate::value::ToValue::to_value(&self.event_levels),
            );
            properties.insert(
                "EventName".to_string(),
                crate::value::ToValue::to_value(&self.event_name),
            );
            properties.insert(
                "LogGroupName".to_string(),
                crate::value::ToValue::to_value(&self.log_group_name),
            );
            if let Some(ref value) = self.pattern_set {
                properties.insert(
                    "PatternSet".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationinsights-application.html
pub struct Application_ {
    pub attach_missing_permission: Option<crate::value::ExpBool>,
    pub auto_configuration_enabled: Option<crate::value::ExpBool>,
    pub cwe_monitor_enabled: Option<crate::value::ExpBool>,
    pub component_monitoring_settings:
        Option<Vec<super::applicationinsights::application::ComponentMonitoringSetting_>>,
    pub custom_components: Option<Vec<super::applicationinsights::application::CustomComponent_>>,
    pub grouping_type: Option<crate::value::ExpString>,
    pub log_pattern_sets: Option<Vec<super::applicationinsights::application::LogPatternSet_>>,
    pub ops_center_enabled: Option<crate::value::ExpBool>,
    pub ops_item_sns_topic_arn: Option<crate::value::ExpString>,
    pub resource_group_name: crate::value::ExpString,
    pub sns_notification_arn: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_applicationinsights_Application {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::ApplicationInsights::Application"
        $($field $value)*)
    };
}
pub use crate::__aws_applicationinsights_Application as Application;
impl crate::template::ToResource for Application_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApplicationInsights"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Application"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.attach_missing_permission {
            properties.insert(
                "AttachMissingPermission".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.auto_configuration_enabled {
            properties.insert(
                "AutoConfigurationEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cwe_monitor_enabled {
            properties.insert(
                "CWEMonitorEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.component_monitoring_settings {
            properties.insert(
                "ComponentMonitoringSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.custom_components {
            properties.insert(
                "CustomComponents".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.grouping_type {
            properties.insert(
                "GroupingType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.log_pattern_sets {
            properties.insert(
                "LogPatternSets".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ops_center_enabled {
            properties.insert(
                "OpsCenterEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ops_item_sns_topic_arn {
            properties.insert(
                "OpsItemSNSTopicArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ResourceGroupName".to_string(),
            crate::value::ToValue::to_value(&self.resource_group_name),
        );
        if let Some(ref value) = self.sns_notification_arn {
            properties.insert(
                "SNSNotificationArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
