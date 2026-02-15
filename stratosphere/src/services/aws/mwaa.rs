pub mod environment {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mwaa-environment-loggingconfiguration.html
    pub struct LoggingConfiguration_ {
        pub dag_processing_logs: Option<Box<ModuleLoggingConfiguration_>>,
        pub scheduler_logs: Option<Box<ModuleLoggingConfiguration_>>,
        pub task_logs: Option<Box<ModuleLoggingConfiguration_>>,
        pub webserver_logs: Option<Box<ModuleLoggingConfiguration_>>,
        pub worker_logs: Option<Box<ModuleLoggingConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mwaa_Environment_LoggingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MWAA::Environment.LoggingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mwaa_Environment_LoggingConfiguration as LoggingConfiguration;
    impl crate::value::ToValue for LoggingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dag_processing_logs {
                properties.insert(
                    "DagProcessingLogs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scheduler_logs {
                properties.insert(
                    "SchedulerLogs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.task_logs {
                properties.insert(
                    "TaskLogs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.webserver_logs {
                properties.insert(
                    "WebserverLogs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.worker_logs {
                properties.insert(
                    "WorkerLogs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mwaa-environment-moduleloggingconfiguration.html
    pub struct ModuleLoggingConfiguration_ {
        pub cloud_watch_log_group_arn: Option<crate::value::ExpString>,
        pub enabled: Option<crate::value::ExpBool>,
        pub log_level: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mwaa_Environment_ModuleLoggingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MWAA::Environment.ModuleLoggingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mwaa_Environment_ModuleLoggingConfiguration as ModuleLoggingConfiguration;
    impl crate::value::ToValue for ModuleLoggingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cloud_watch_log_group_arn {
                properties.insert(
                    "CloudWatchLogGroupArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.log_level {
                properties.insert(
                    "LogLevel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mwaa-environment-networkconfiguration.html
    pub struct NetworkConfiguration_ {
        pub security_group_ids: Option<Vec<crate::value::ExpString>>,
        pub subnet_ids: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mwaa_Environment_NetworkConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MWAA::Environment.NetworkConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mwaa_Environment_NetworkConfiguration as NetworkConfiguration;
    impl crate::value::ToValue for NetworkConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.security_group_ids {
                properties.insert(
                    "SecurityGroupIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.subnet_ids {
                properties.insert(
                    "SubnetIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mwaa-environment.html
pub struct Environment_ {
    pub airflow_configuration_options: Option<serde_json::Value>,
    pub airflow_version: Option<crate::value::ExpString>,
    pub dag_s3_path: Option<crate::value::ExpString>,
    pub endpoint_management: Option<crate::value::ExpString>,
    pub environment_class: Option<crate::value::ExpString>,
    pub execution_role_arn: Option<crate::value::ExpString>,
    pub kms_key: Option<crate::value::ExpString>,
    pub logging_configuration: Option<super::mwaa::environment::LoggingConfiguration_>,
    pub max_webservers: Option<i32>,
    pub max_workers: Option<i32>,
    pub min_webservers: Option<i32>,
    pub min_workers: Option<i32>,
    pub name: crate::value::ExpString,
    pub network_configuration: Option<super::mwaa::environment::NetworkConfiguration_>,
    pub plugins_s3_object_version: Option<crate::value::ExpString>,
    pub plugins_s3_path: Option<crate::value::ExpString>,
    pub requirements_s3_object_version: Option<crate::value::ExpString>,
    pub requirements_s3_path: Option<crate::value::ExpString>,
    pub schedulers: Option<i32>,
    pub source_bucket_arn: Option<crate::value::ExpString>,
    pub startup_script_s3_object_version: Option<crate::value::ExpString>,
    pub startup_script_s3_path: Option<crate::value::ExpString>,
    pub tags: Option<serde_json::Value>,
    pub webserver_access_mode: Option<crate::value::ExpString>,
    pub weekly_maintenance_window_start: Option<crate::value::ExpString>,
    pub worker_replacement_strategy: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_mwaa_Environment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MWAA::Environment"
        $($field $value)*)
    };
}
pub use crate::__aws_mwaa_Environment as Environment;
impl crate::template::ToResource for Environment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MWAA"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Environment"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.airflow_configuration_options {
            properties.insert(
                "AirflowConfigurationOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.airflow_version {
            properties.insert(
                "AirflowVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.dag_s3_path {
            properties.insert(
                "DagS3Path".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.endpoint_management {
            properties.insert(
                "EndpointManagement".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.environment_class {
            properties.insert(
                "EnvironmentClass".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.execution_role_arn {
            properties.insert(
                "ExecutionRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key {
            properties.insert("KmsKey".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.logging_configuration {
            properties.insert(
                "LoggingConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.max_webservers {
            properties.insert(
                "MaxWebservers".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.max_workers {
            properties.insert(
                "MaxWorkers".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.min_webservers {
            properties.insert(
                "MinWebservers".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.min_workers {
            properties.insert(
                "MinWorkers".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.network_configuration {
            properties.insert(
                "NetworkConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.plugins_s3_object_version {
            properties.insert(
                "PluginsS3ObjectVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.plugins_s3_path {
            properties.insert(
                "PluginsS3Path".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.requirements_s3_object_version {
            properties.insert(
                "RequirementsS3ObjectVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.requirements_s3_path {
            properties.insert(
                "RequirementsS3Path".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.schedulers {
            properties.insert(
                "Schedulers".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_bucket_arn {
            properties.insert(
                "SourceBucketArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.startup_script_s3_object_version {
            properties.insert(
                "StartupScriptS3ObjectVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.startup_script_s3_path {
            properties.insert(
                "StartupScriptS3Path".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.webserver_access_mode {
            properties.insert(
                "WebserverAccessMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.weekly_maintenance_window_start {
            properties.insert(
                "WeeklyMaintenanceWindowStart".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.worker_replacement_strategy {
            properties.insert(
                "WorkerReplacementStrategy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
