pub mod experimenttemplate {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-cloudwatchdashboard.html>
    pub struct CloudWatchDashboard_ {
        pub dashboard_identifier: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fis_ExperimentTemplate_CloudWatchDashboard {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FIS::ExperimentTemplate.CloudWatchDashboard"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fis_ExperimentTemplate_CloudWatchDashboard as CloudWatchDashboard;
    impl crate::value::ToValue for CloudWatchDashboard_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DashboardIdentifier".to_string(),
                crate::value::ToValue::to_value(&self.dashboard_identifier),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-cloudwatchlogsconfiguration.html>
    pub struct CloudWatchLogsConfiguration_ {
        pub log_group_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fis_ExperimentTemplate_CloudWatchLogsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FIS::ExperimentTemplate.CloudWatchLogsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fis_ExperimentTemplate_CloudWatchLogsConfiguration as CloudWatchLogsConfiguration;
    impl crate::value::ToValue for CloudWatchLogsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LogGroupArn".to_string(),
                crate::value::ToValue::to_value(&self.log_group_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-datasources.html>
    pub struct DataSources_ {
        pub cloud_watch_dashboards: Option<Vec<CloudWatchDashboard_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fis_ExperimentTemplate_DataSources {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FIS::ExperimentTemplate.DataSources"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fis_ExperimentTemplate_DataSources as DataSources;
    impl crate::value::ToValue for DataSources_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cloud_watch_dashboards {
                properties.insert(
                    "CloudWatchDashboards".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-experimentreports3configuration.html>
    pub struct ExperimentReportS3Configuration_ {
        pub bucket_name: crate::value::ExpString,
        pub prefix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fis_ExperimentTemplate_ExperimentReportS3Configuration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FIS::ExperimentTemplate.ExperimentReportS3Configuration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fis_ExperimentTemplate_ExperimentReportS3Configuration as ExperimentReportS3Configuration;
    impl crate::value::ToValue for ExperimentReportS3Configuration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BucketName".to_string(),
                crate::value::ToValue::to_value(&self.bucket_name),
            );
            if let Some(ref value) = self.prefix {
                properties.insert("Prefix".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-experimenttemplateaction.html>
    pub struct ExperimentTemplateAction_ {
        pub action_id: crate::value::ExpString,
        pub description: Option<crate::value::ExpString>,
        pub parameters: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub start_after: Option<Vec<crate::value::ExpString>>,
        pub targets: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fis_ExperimentTemplate_ExperimentTemplateAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FIS::ExperimentTemplate.ExperimentTemplateAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fis_ExperimentTemplate_ExperimentTemplateAction as ExperimentTemplateAction;
    impl crate::value::ToValue for ExperimentTemplateAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ActionId".to_string(),
                crate::value::ToValue::to_value(&self.action_id),
            );
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parameters {
                properties.insert(
                    "Parameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.start_after {
                properties.insert(
                    "StartAfter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.targets {
                properties.insert(
                    "Targets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-experimenttemplateexperimentoptions.html>
    pub struct ExperimentTemplateExperimentOptions_ {
        pub account_targeting: Option<crate::value::ExpString>,
        pub empty_target_resolution_mode: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fis_ExperimentTemplate_ExperimentTemplateExperimentOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FIS::ExperimentTemplate.ExperimentTemplateExperimentOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fis_ExperimentTemplate_ExperimentTemplateExperimentOptions as ExperimentTemplateExperimentOptions;
    impl crate::value::ToValue for ExperimentTemplateExperimentOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.account_targeting {
                properties.insert(
                    "AccountTargeting".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.empty_target_resolution_mode {
                properties.insert(
                    "EmptyTargetResolutionMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-experimenttemplateexperimentreportconfiguration.html>
    pub struct ExperimentTemplateExperimentReportConfiguration_ {
        pub data_sources: Option<Box<DataSources_>>,
        pub outputs: Box<Outputs_>,
        pub post_experiment_duration: Option<crate::value::ExpString>,
        pub pre_experiment_duration: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fis_ExperimentTemplate_ExperimentTemplateExperimentReportConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FIS::ExperimentTemplate.ExperimentTemplateExperimentReportConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fis_ExperimentTemplate_ExperimentTemplateExperimentReportConfiguration as ExperimentTemplateExperimentReportConfiguration;
    impl crate::value::ToValue for ExperimentTemplateExperimentReportConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.data_sources {
                properties.insert(
                    "DataSources".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Outputs".to_string(),
                crate::value::ToValue::to_value(&self.outputs),
            );
            if let Some(ref value) = self.post_experiment_duration {
                properties.insert(
                    "PostExperimentDuration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pre_experiment_duration {
                properties.insert(
                    "PreExperimentDuration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-experimenttemplatelogconfiguration.html>
    pub struct ExperimentTemplateLogConfiguration_ {
        pub cloud_watch_logs_configuration: Option<Box<CloudWatchLogsConfiguration_>>,
        pub log_schema_version: i32,
        pub s3_configuration: Option<Box<S3Configuration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fis_ExperimentTemplate_ExperimentTemplateLogConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FIS::ExperimentTemplate.ExperimentTemplateLogConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fis_ExperimentTemplate_ExperimentTemplateLogConfiguration as ExperimentTemplateLogConfiguration;
    impl crate::value::ToValue for ExperimentTemplateLogConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cloud_watch_logs_configuration {
                properties.insert(
                    "CloudWatchLogsConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "LogSchemaVersion".to_string(),
                crate::value::ToValue::to_value(&self.log_schema_version),
            );
            if let Some(ref value) = self.s3_configuration {
                properties.insert(
                    "S3Configuration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-experimenttemplatestopcondition.html>
    pub struct ExperimentTemplateStopCondition_ {
        pub source: crate::value::ExpString,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fis_ExperimentTemplate_ExperimentTemplateStopCondition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FIS::ExperimentTemplate.ExperimentTemplateStopCondition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fis_ExperimentTemplate_ExperimentTemplateStopCondition as ExperimentTemplateStopCondition;
    impl crate::value::ToValue for ExperimentTemplateStopCondition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Source".to_string(),
                crate::value::ToValue::to_value(&self.source),
            );
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-experimenttemplatetarget.html>
    pub struct ExperimentTemplateTarget_ {
        pub filters: Option<Vec<ExperimentTemplateTargetFilter_>>,
        pub parameters: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub resource_arns: Option<Vec<crate::value::ExpString>>,
        pub resource_tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub resource_type: crate::value::ExpString,
        pub selection_mode: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fis_ExperimentTemplate_ExperimentTemplateTarget {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FIS::ExperimentTemplate.ExperimentTemplateTarget"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fis_ExperimentTemplate_ExperimentTemplateTarget as ExperimentTemplateTarget;
    impl crate::value::ToValue for ExperimentTemplateTarget_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.filters {
                properties.insert(
                    "Filters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parameters {
                properties.insert(
                    "Parameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_arns {
                properties.insert(
                    "ResourceArns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_tags {
                properties.insert(
                    "ResourceTags".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ResourceType".to_string(),
                crate::value::ToValue::to_value(&self.resource_type),
            );
            properties.insert(
                "SelectionMode".to_string(),
                crate::value::ToValue::to_value(&self.selection_mode),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-experimenttemplatetargetfilter.html>
    pub struct ExperimentTemplateTargetFilter_ {
        pub path: crate::value::ExpString,
        pub values: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fis_ExperimentTemplate_ExperimentTemplateTargetFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FIS::ExperimentTemplate.ExperimentTemplateTargetFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fis_ExperimentTemplate_ExperimentTemplateTargetFilter as ExperimentTemplateTargetFilter;
    impl crate::value::ToValue for ExperimentTemplateTargetFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Path".to_string(),
                crate::value::ToValue::to_value(&self.path),
            );
            properties.insert(
                "Values".to_string(),
                crate::value::ToValue::to_value(&self.values),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-outputs.html>
    pub struct Outputs_ {
        pub experiment_report_s3_configuration: Box<ExperimentReportS3Configuration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fis_ExperimentTemplate_Outputs {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FIS::ExperimentTemplate.Outputs"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fis_ExperimentTemplate_Outputs as Outputs;
    impl crate::value::ToValue for Outputs_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ExperimentReportS3Configuration".to_string(),
                crate::value::ToValue::to_value(&self.experiment_report_s3_configuration),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fis-experimenttemplate-s3configuration.html>
    pub struct S3Configuration_ {
        pub bucket_name: crate::value::ExpString,
        pub prefix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fis_ExperimentTemplate_S3Configuration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FIS::ExperimentTemplate.S3Configuration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fis_ExperimentTemplate_S3Configuration as S3Configuration;
    impl crate::value::ToValue for S3Configuration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BucketName".to_string(),
                crate::value::ToValue::to_value(&self.bucket_name),
            );
            if let Some(ref value) = self.prefix {
                properties.insert("Prefix".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fis-experimenttemplate.html>
pub struct ExperimentTemplate_ {
    pub actions: Option<
        std::collections::BTreeMap<
            String,
            super::fis::experimenttemplate::ExperimentTemplateAction_,
        >,
    >,
    pub description: crate::value::ExpString,
    pub experiment_options:
        Option<super::fis::experimenttemplate::ExperimentTemplateExperimentOptions_>,
    pub experiment_report_configuration:
        Option<super::fis::experimenttemplate::ExperimentTemplateExperimentReportConfiguration_>,
    pub log_configuration:
        Option<super::fis::experimenttemplate::ExperimentTemplateLogConfiguration_>,
    pub role_arn: crate::value::ExpString,
    pub stop_conditions: Vec<super::fis::experimenttemplate::ExperimentTemplateStopCondition_>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub targets: std::collections::BTreeMap<
        String,
        super::fis::experimenttemplate::ExperimentTemplateTarget_,
    >,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_fis_ExperimentTemplate {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::FIS::ExperimentTemplate"
        $($field $value)*)
    };
}
pub use crate::__aws_fis_ExperimentTemplate as ExperimentTemplate;
impl crate::template::ToResource for ExperimentTemplate_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("FIS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ExperimentTemplate"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.actions {
            properties.insert(
                "Actions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Description".to_string(),
            crate::value::ToValue::to_value(&self.description),
        );
        if let Some(ref value) = self.experiment_options {
            properties.insert(
                "ExperimentOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.experiment_report_configuration {
            properties.insert(
                "ExperimentReportConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.log_configuration {
            properties.insert(
                "LogConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        properties.insert(
            "StopConditions".to_string(),
            crate::value::ToValue::to_value(&self.stop_conditions),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Targets".to_string(),
            crate::value::ToValue::to_value(&self.targets),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fis-targetaccountconfiguration.html>
pub struct TargetAccountConfiguration_ {
    pub account_id: crate::value::ExpString,
    pub description: Option<crate::value::ExpString>,
    pub experiment_template_id: crate::value::ExpString,
    pub role_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_fis_TargetAccountConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::FIS::TargetAccountConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_fis_TargetAccountConfiguration as TargetAccountConfiguration;
impl crate::template::ToResource for TargetAccountConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("FIS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "TargetAccountConfiguration",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AccountId".to_string(),
            crate::value::ToValue::to_value(&self.account_id),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ExperimentTemplateId".to_string(),
            crate::value::ToValue::to_value(&self.experiment_template_id),
        );
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        properties
    }
}
