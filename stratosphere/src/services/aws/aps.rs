pub mod anomalydetector {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-aps-anomalydetector-anomalydetectorconfiguration.html
    pub struct AnomalyDetectorConfiguration_ {
        pub random_cut_forest: Box<RandomCutForestConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_aps_AnomalyDetector_AnomalyDetectorConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::APS::AnomalyDetector.AnomalyDetectorConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_aps_AnomalyDetector_AnomalyDetectorConfiguration as AnomalyDetectorConfiguration;
    impl crate::value::ToValue for AnomalyDetectorConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RandomCutForest".to_string(),
                crate::value::ToValue::to_value(&self.random_cut_forest),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-aps-anomalydetector-ignorenearexpected.html
    pub struct IgnoreNearExpected_ {
        pub amount: Option<f64>,
        pub ratio: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_aps_AnomalyDetector_IgnoreNearExpected {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::APS::AnomalyDetector.IgnoreNearExpected"
            $($field $value)*)
        };
    }
    pub use crate::__aws_aps_AnomalyDetector_IgnoreNearExpected as IgnoreNearExpected;
    impl crate::value::ToValue for IgnoreNearExpected_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.amount {
                properties.insert("Amount".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.ratio {
                properties.insert("Ratio".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-aps-anomalydetector-label.html
    pub struct Label_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_aps_AnomalyDetector_Label {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::APS::AnomalyDetector.Label"
            $($field $value)*)
        };
    }
    pub use crate::__aws_aps_AnomalyDetector_Label as Label;
    impl crate::value::ToValue for Label_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-aps-anomalydetector-missingdataaction.html
    pub struct MissingDataAction_ {
        pub mark_as_anomaly: Option<crate::value::ExpBool>,
        pub skip: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_aps_AnomalyDetector_MissingDataAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::APS::AnomalyDetector.MissingDataAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_aps_AnomalyDetector_MissingDataAction as MissingDataAction;
    impl crate::value::ToValue for MissingDataAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.mark_as_anomaly {
                properties.insert(
                    "MarkAsAnomaly".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.skip {
                properties.insert("Skip".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-aps-anomalydetector-randomcutforestconfiguration.html
    pub struct RandomCutForestConfiguration_ {
        pub ignore_near_expected_from_above: Option<Box<IgnoreNearExpected_>>,
        pub ignore_near_expected_from_below: Option<Box<IgnoreNearExpected_>>,
        pub query: crate::value::ExpString,
        pub sample_size: Option<i32>,
        pub shingle_size: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_aps_AnomalyDetector_RandomCutForestConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::APS::AnomalyDetector.RandomCutForestConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_aps_AnomalyDetector_RandomCutForestConfiguration as RandomCutForestConfiguration;
    impl crate::value::ToValue for RandomCutForestConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ignore_near_expected_from_above {
                properties.insert(
                    "IgnoreNearExpectedFromAbove".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ignore_near_expected_from_below {
                properties.insert(
                    "IgnoreNearExpectedFromBelow".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Query".to_string(),
                crate::value::ToValue::to_value(&self.query),
            );
            if let Some(ref value) = self.sample_size {
                properties.insert(
                    "SampleSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.shingle_size {
                properties.insert(
                    "ShingleSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod scraper {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-aps-scraper-ampconfiguration.html
    pub struct AmpConfiguration_ {
        pub workspace_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_aps_Scraper_AmpConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::APS::Scraper.AmpConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_aps_Scraper_AmpConfiguration as AmpConfiguration;
    impl crate::value::ToValue for AmpConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "WorkspaceArn".to_string(),
                crate::value::ToValue::to_value(&self.workspace_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-aps-scraper-cloudwatchlogdestination.html
    pub struct CloudWatchLogDestination_ {
        pub log_group_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_aps_Scraper_CloudWatchLogDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::APS::Scraper.CloudWatchLogDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_aps_Scraper_CloudWatchLogDestination as CloudWatchLogDestination;
    impl crate::value::ToValue for CloudWatchLogDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.log_group_arn {
                properties.insert(
                    "LogGroupArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-aps-scraper-componentconfig.html
    pub struct ComponentConfig_ {
        pub options: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_aps_Scraper_ComponentConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::APS::Scraper.ComponentConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_aps_Scraper_ComponentConfig as ComponentConfig;
    impl crate::value::ToValue for ComponentConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.options {
                properties.insert(
                    "Options".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-aps-scraper-destination.html
    pub struct Destination_ {
        pub amp_configuration: Box<AmpConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_aps_Scraper_Destination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::APS::Scraper.Destination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_aps_Scraper_Destination as Destination;
    impl crate::value::ToValue for Destination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AmpConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.amp_configuration),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-aps-scraper-eksconfiguration.html
    pub struct EksConfiguration_ {
        pub cluster_arn: crate::value::ExpString,
        pub security_group_ids: Option<Vec<crate::value::ExpString>>,
        pub subnet_ids: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_aps_Scraper_EksConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::APS::Scraper.EksConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_aps_Scraper_EksConfiguration as EksConfiguration;
    impl crate::value::ToValue for EksConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ClusterArn".to_string(),
                crate::value::ToValue::to_value(&self.cluster_arn),
            );
            if let Some(ref value) = self.security_group_ids {
                properties.insert(
                    "SecurityGroupIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SubnetIds".to_string(),
                crate::value::ToValue::to_value(&self.subnet_ids),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-aps-scraper-roleconfiguration.html
    pub struct RoleConfiguration_ {
        pub source_role_arn: Option<crate::value::ExpString>,
        pub target_role_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_aps_Scraper_RoleConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::APS::Scraper.RoleConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_aps_Scraper_RoleConfiguration as RoleConfiguration;
    impl crate::value::ToValue for RoleConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.source_role_arn {
                properties.insert(
                    "SourceRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_role_arn {
                properties.insert(
                    "TargetRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-aps-scraper-scrapeconfiguration.html
    pub struct ScrapeConfiguration_ {
        pub configuration_blob: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_aps_Scraper_ScrapeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::APS::Scraper.ScrapeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_aps_Scraper_ScrapeConfiguration as ScrapeConfiguration;
    impl crate::value::ToValue for ScrapeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ConfigurationBlob".to_string(),
                crate::value::ToValue::to_value(&self.configuration_blob),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-aps-scraper-scrapercomponent.html
    pub struct ScraperComponent_ {
        pub config: Option<Box<ComponentConfig_>>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_aps_Scraper_ScraperComponent {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::APS::Scraper.ScraperComponent"
            $($field $value)*)
        };
    }
    pub use crate::__aws_aps_Scraper_ScraperComponent as ScraperComponent;
    impl crate::value::ToValue for ScraperComponent_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.config {
                properties.insert("Config".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-aps-scraper-scraperloggingconfiguration.html
    pub struct ScraperLoggingConfiguration_ {
        pub logging_destination: Box<ScraperLoggingDestination_>,
        pub scraper_components: Vec<ScraperComponent_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_aps_Scraper_ScraperLoggingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::APS::Scraper.ScraperLoggingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_aps_Scraper_ScraperLoggingConfiguration as ScraperLoggingConfiguration;
    impl crate::value::ToValue for ScraperLoggingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LoggingDestination".to_string(),
                crate::value::ToValue::to_value(&self.logging_destination),
            );
            properties.insert(
                "ScraperComponents".to_string(),
                crate::value::ToValue::to_value(&self.scraper_components),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-aps-scraper-scraperloggingdestination.html
    pub struct ScraperLoggingDestination_ {
        pub cloud_watch_logs: Option<Box<CloudWatchLogDestination_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_aps_Scraper_ScraperLoggingDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::APS::Scraper.ScraperLoggingDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_aps_Scraper_ScraperLoggingDestination as ScraperLoggingDestination;
    impl crate::value::ToValue for ScraperLoggingDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cloud_watch_logs {
                properties.insert(
                    "CloudWatchLogs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-aps-scraper-source.html
    pub struct Source_ {
        pub eks_configuration: Option<Box<EksConfiguration_>>,
        pub vpc_configuration: Option<Box<VpcConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_aps_Scraper_Source {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::APS::Scraper.Source"
            $($field $value)*)
        };
    }
    pub use crate::__aws_aps_Scraper_Source as Source;
    impl crate::value::ToValue for Source_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.eks_configuration {
                properties.insert(
                    "EksConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpc_configuration {
                properties.insert(
                    "VpcConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-aps-scraper-vpcconfiguration.html
    pub struct VpcConfiguration_ {
        pub security_group_ids: Vec<crate::value::ExpString>,
        pub subnet_ids: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_aps_Scraper_VpcConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::APS::Scraper.VpcConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_aps_Scraper_VpcConfiguration as VpcConfiguration;
    impl crate::value::ToValue for VpcConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(&self.security_group_ids),
            );
            properties.insert(
                "SubnetIds".to_string(),
                crate::value::ToValue::to_value(&self.subnet_ids),
            );
            properties.into()
        }
    }
}
pub mod workspace {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-aps-workspace-cloudwatchlogdestination.html
    pub struct CloudWatchLogDestination_ {
        pub log_group_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_aps_Workspace_CloudWatchLogDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::APS::Workspace.CloudWatchLogDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_aps_Workspace_CloudWatchLogDestination as CloudWatchLogDestination;
    impl crate::value::ToValue for CloudWatchLogDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LogGroupArn".to_string(),
                crate::value::ToValue::to_value(&self.log_group_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-aps-workspace-label.html
    pub struct Label_ {
        pub name: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_aps_Workspace_Label {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::APS::Workspace.Label"
            $($field $value)*)
        };
    }
    pub use crate::__aws_aps_Workspace_Label as Label;
    impl crate::value::ToValue for Label_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-aps-workspace-limitsperlabelset.html
    pub struct LimitsPerLabelSet_ {
        pub label_set: Vec<Label_>,
        pub limits: Box<LimitsPerLabelSetEntry_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_aps_Workspace_LimitsPerLabelSet {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::APS::Workspace.LimitsPerLabelSet"
            $($field $value)*)
        };
    }
    pub use crate::__aws_aps_Workspace_LimitsPerLabelSet as LimitsPerLabelSet;
    impl crate::value::ToValue for LimitsPerLabelSet_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LabelSet".to_string(),
                crate::value::ToValue::to_value(&self.label_set),
            );
            properties.insert(
                "Limits".to_string(),
                crate::value::ToValue::to_value(&self.limits),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-aps-workspace-limitsperlabelsetentry.html
    pub struct LimitsPerLabelSetEntry_ {
        pub max_series: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_aps_Workspace_LimitsPerLabelSetEntry {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::APS::Workspace.LimitsPerLabelSetEntry"
            $($field $value)*)
        };
    }
    pub use crate::__aws_aps_Workspace_LimitsPerLabelSetEntry as LimitsPerLabelSetEntry;
    impl crate::value::ToValue for LimitsPerLabelSetEntry_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max_series {
                properties.insert(
                    "MaxSeries".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-aps-workspace-loggingconfiguration.html
    pub struct LoggingConfiguration_ {
        pub log_group_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_aps_Workspace_LoggingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::APS::Workspace.LoggingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_aps_Workspace_LoggingConfiguration as LoggingConfiguration;
    impl crate::value::ToValue for LoggingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.log_group_arn {
                properties.insert(
                    "LogGroupArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-aps-workspace-loggingdestination.html
    pub struct LoggingDestination_ {
        pub cloud_watch_logs: Box<CloudWatchLogDestination_>,
        pub filters: Box<LoggingFilter_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_aps_Workspace_LoggingDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::APS::Workspace.LoggingDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_aps_Workspace_LoggingDestination as LoggingDestination;
    impl crate::value::ToValue for LoggingDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CloudWatchLogs".to_string(),
                crate::value::ToValue::to_value(&self.cloud_watch_logs),
            );
            properties.insert(
                "Filters".to_string(),
                crate::value::ToValue::to_value(&self.filters),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-aps-workspace-loggingfilter.html
    pub struct LoggingFilter_ {
        pub qsp_threshold: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_aps_Workspace_LoggingFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::APS::Workspace.LoggingFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_aps_Workspace_LoggingFilter as LoggingFilter;
    impl crate::value::ToValue for LoggingFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "QspThreshold".to_string(),
                crate::value::ToValue::to_value(&self.qsp_threshold),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-aps-workspace-queryloggingconfiguration.html
    pub struct QueryLoggingConfiguration_ {
        pub destinations: Vec<LoggingDestination_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_aps_Workspace_QueryLoggingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::APS::Workspace.QueryLoggingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_aps_Workspace_QueryLoggingConfiguration as QueryLoggingConfiguration;
    impl crate::value::ToValue for QueryLoggingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Destinations".to_string(),
                crate::value::ToValue::to_value(&self.destinations),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-aps-workspace-workspaceconfiguration.html
    pub struct WorkspaceConfiguration_ {
        pub limits_per_label_sets: Option<Vec<LimitsPerLabelSet_>>,
        pub retention_period_in_days: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_aps_Workspace_WorkspaceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::APS::Workspace.WorkspaceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_aps_Workspace_WorkspaceConfiguration as WorkspaceConfiguration;
    impl crate::value::ToValue for WorkspaceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.limits_per_label_sets {
                properties.insert(
                    "LimitsPerLabelSets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.retention_period_in_days {
                properties.insert(
                    "RetentionPeriodInDays".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-aps-anomalydetector.html
pub struct AnomalyDetector_ {
    pub alias: crate::value::ExpString,
    pub configuration: super::aps::anomalydetector::AnomalyDetectorConfiguration_,
    pub evaluation_interval_in_seconds: Option<i32>,
    pub labels: Option<Vec<super::aps::anomalydetector::Label_>>,
    pub missing_data_action: Option<super::aps::anomalydetector::MissingDataAction_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub workspace: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_aps_AnomalyDetector {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::APS::AnomalyDetector"
        $($field $value)*)
    };
}
pub use crate::__aws_aps_AnomalyDetector as AnomalyDetector;
impl crate::template::ToResource for AnomalyDetector_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("APS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AnomalyDetector"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Alias".to_string(),
            crate::value::ToValue::to_value(&self.alias),
        );
        properties.insert(
            "Configuration".to_string(),
            crate::value::ToValue::to_value(&self.configuration),
        );
        if let Some(ref value) = self.evaluation_interval_in_seconds {
            properties.insert(
                "EvaluationIntervalInSeconds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.labels {
            properties.insert("Labels".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.missing_data_action {
            properties.insert(
                "MissingDataAction".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Workspace".to_string(),
            crate::value::ToValue::to_value(&self.workspace),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-aps-resourcepolicy.html
pub struct ResourcePolicy_ {
    pub policy_document: crate::value::ExpString,
    pub workspace_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_aps_ResourcePolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::APS::ResourcePolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_aps_ResourcePolicy as ResourcePolicy;
impl crate::template::ToResource for ResourcePolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("APS"),
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
            "WorkspaceArn".to_string(),
            crate::value::ToValue::to_value(&self.workspace_arn),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-aps-rulegroupsnamespace.html
pub struct RuleGroupsNamespace_ {
    pub data: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub workspace: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_aps_RuleGroupsNamespace {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::APS::RuleGroupsNamespace"
        $($field $value)*)
    };
}
pub use crate::__aws_aps_RuleGroupsNamespace as RuleGroupsNamespace;
impl crate::template::ToResource for RuleGroupsNamespace_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("APS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RuleGroupsNamespace"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Data".to_string(),
            crate::value::ToValue::to_value(&self.data),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Workspace".to_string(),
            crate::value::ToValue::to_value(&self.workspace),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-aps-scraper.html
pub struct Scraper_ {
    pub alias: Option<crate::value::ExpString>,
    pub destination: super::aps::scraper::Destination_,
    pub role_configuration: Option<super::aps::scraper::RoleConfiguration_>,
    pub scrape_configuration: super::aps::scraper::ScrapeConfiguration_,
    pub scraper_logging_configuration: Option<super::aps::scraper::ScraperLoggingConfiguration_>,
    pub source: super::aps::scraper::Source_,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_aps_Scraper {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::APS::Scraper" $($field
        $value)*)
    };
}
pub use crate::__aws_aps_Scraper as Scraper;
impl crate::template::ToResource for Scraper_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("APS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Scraper"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.alias {
            properties.insert("Alias".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Destination".to_string(),
            crate::value::ToValue::to_value(&self.destination),
        );
        if let Some(ref value) = self.role_configuration {
            properties.insert(
                "RoleConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ScrapeConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.scrape_configuration),
        );
        if let Some(ref value) = self.scraper_logging_configuration {
            properties.insert(
                "ScraperLoggingConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Source".to_string(),
            crate::value::ToValue::to_value(&self.source),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-aps-workspace.html
pub struct Workspace_ {
    pub alert_manager_definition: Option<crate::value::ExpString>,
    pub alias: Option<crate::value::ExpString>,
    pub kms_key_arn: Option<crate::value::ExpString>,
    pub logging_configuration: Option<super::aps::workspace::LoggingConfiguration_>,
    pub query_logging_configuration: Option<super::aps::workspace::QueryLoggingConfiguration_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub workspace_configuration: Option<super::aps::workspace::WorkspaceConfiguration_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_aps_Workspace {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::APS::Workspace" $($field
        $value)*)
    };
}
pub use crate::__aws_aps_Workspace as Workspace;
impl crate::template::ToResource for Workspace_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("APS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Workspace"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.alert_manager_definition {
            properties.insert(
                "AlertManagerDefinition".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.alias {
            properties.insert("Alias".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.kms_key_arn {
            properties.insert(
                "KmsKeyArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.logging_configuration {
            properties.insert(
                "LoggingConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.query_logging_configuration {
            properties.insert(
                "QueryLoggingConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.workspace_configuration {
            properties.insert(
                "WorkspaceConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
