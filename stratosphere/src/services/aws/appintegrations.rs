pub mod application {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appintegrations-application-applicationconfig.html
    pub struct ApplicationConfig_ {
        pub contact_handling: Option<Box<ContactHandling_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appintegrations_Application_ApplicationConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppIntegrations::Application.ApplicationConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appintegrations_Application_ApplicationConfig as ApplicationConfig;
    impl crate::value::ToValue for ApplicationConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.contact_handling {
                properties.insert(
                    "ContactHandling".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appintegrations-application-applicationsourceconfig.html
    pub struct ApplicationSourceConfig_ {
        pub external_url_config: Box<ExternalUrlConfig_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appintegrations_Application_ApplicationSourceConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppIntegrations::Application.ApplicationSourceConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appintegrations_Application_ApplicationSourceConfig as ApplicationSourceConfig;
    impl crate::value::ToValue for ApplicationSourceConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ExternalUrlConfig".to_string(),
                crate::value::ToValue::to_value(&self.external_url_config),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appintegrations-application-contacthandling.html
    pub struct ContactHandling_ {
        pub scope: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appintegrations_Application_ContactHandling {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppIntegrations::Application.ContactHandling"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appintegrations_Application_ContactHandling as ContactHandling;
    impl crate::value::ToValue for ContactHandling_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Scope".to_string(),
                crate::value::ToValue::to_value(&self.scope),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appintegrations-application-externalurlconfig.html
    pub struct ExternalUrlConfig_ {
        pub access_url: crate::value::ExpString,
        pub approved_origins: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appintegrations_Application_ExternalUrlConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppIntegrations::Application.ExternalUrlConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appintegrations_Application_ExternalUrlConfig as ExternalUrlConfig;
    impl crate::value::ToValue for ExternalUrlConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AccessUrl".to_string(),
                crate::value::ToValue::to_value(&self.access_url),
            );
            if let Some(ref value) = self.approved_origins {
                properties.insert(
                    "ApprovedOrigins".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appintegrations-application-iframeconfig.html
    pub struct IframeConfig_ {
        pub allow: Option<Vec<crate::value::ExpString>>,
        pub sandbox: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appintegrations_Application_IframeConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppIntegrations::Application.IframeConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appintegrations_Application_IframeConfig as IframeConfig;
    impl crate::value::ToValue for IframeConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allow {
                properties.insert("Allow".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.sandbox {
                properties.insert(
                    "Sandbox".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod dataintegration {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appintegrations-dataintegration-fileconfiguration.html
    pub struct FileConfiguration_ {
        pub filters: Option<serde_json::Value>,
        pub folders: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appintegrations_DataIntegration_FileConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppIntegrations::DataIntegration.FileConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appintegrations_DataIntegration_FileConfiguration as FileConfiguration;
    impl crate::value::ToValue for FileConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.filters {
                properties.insert(
                    "Filters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Folders".to_string(),
                crate::value::ToValue::to_value(&self.folders),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appintegrations-dataintegration-scheduleconfig.html
    pub struct ScheduleConfig_ {
        pub first_execution_from: Option<crate::value::ExpString>,
        pub object: Option<crate::value::ExpString>,
        pub schedule_expression: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appintegrations_DataIntegration_ScheduleConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppIntegrations::DataIntegration.ScheduleConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appintegrations_DataIntegration_ScheduleConfig as ScheduleConfig;
    impl crate::value::ToValue for ScheduleConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.first_execution_from {
                properties.insert(
                    "FirstExecutionFrom".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.object {
                properties.insert("Object".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "ScheduleExpression".to_string(),
                crate::value::ToValue::to_value(&self.schedule_expression),
            );
            properties.into()
        }
    }
}
pub mod eventintegration {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appintegrations-eventintegration-eventfilter.html
    pub struct EventFilter_ {
        pub source: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appintegrations_EventIntegration_EventFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppIntegrations::EventIntegration.EventFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appintegrations_EventIntegration_EventFilter as EventFilter;
    impl crate::value::ToValue for EventFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Source".to_string(),
                crate::value::ToValue::to_value(&self.source),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appintegrations-application.html
pub struct Application_ {
    pub application_config: Option<super::appintegrations::application::ApplicationConfig_>,
    pub application_source_config: super::appintegrations::application::ApplicationSourceConfig_,
    pub description: crate::value::ExpString,
    pub iframe_config: Option<super::appintegrations::application::IframeConfig_>,
    pub initialization_timeout: Option<i64>,
    pub is_service: Option<crate::value::ExpBool>,
    pub name: crate::value::ExpString,
    pub namespace: crate::value::ExpString,
    pub permissions: Option<Vec<crate::value::ExpString>>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appintegrations_Application {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::AppIntegrations::Application"
        $($field $value)*)
    };
}
pub use crate::__aws_appintegrations_Application as Application;
impl crate::template::ToResource for Application_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppIntegrations"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Application"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.application_config {
            properties.insert(
                "ApplicationConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ApplicationSourceConfig".to_string(),
            crate::value::ToValue::to_value(&self.application_source_config),
        );
        properties.insert(
            "Description".to_string(),
            crate::value::ToValue::to_value(&self.description),
        );
        if let Some(ref value) = self.iframe_config {
            properties.insert(
                "IframeConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.initialization_timeout {
            properties.insert(
                "InitializationTimeout".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.is_service {
            properties.insert(
                "IsService".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "Namespace".to_string(),
            crate::value::ToValue::to_value(&self.namespace),
        );
        if let Some(ref value) = self.permissions {
            properties.insert(
                "Permissions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appintegrations-dataintegration.html
pub struct DataIntegration_ {
    pub description: Option<crate::value::ExpString>,
    pub file_configuration: Option<super::appintegrations::dataintegration::FileConfiguration_>,
    pub kms_key: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub object_configuration: Option<serde_json::Value>,
    pub schedule_config: Option<super::appintegrations::dataintegration::ScheduleConfig_>,
    pub source_uri: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appintegrations_DataIntegration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::AppIntegrations::DataIntegration"
        $($field $value)*)
    };
}
pub use crate::__aws_appintegrations_DataIntegration as DataIntegration;
impl crate::template::ToResource for DataIntegration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppIntegrations"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DataIntegration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.file_configuration {
            properties.insert(
                "FileConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "KmsKey".to_string(),
            crate::value::ToValue::to_value(&self.kms_key),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.object_configuration {
            properties.insert(
                "ObjectConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.schedule_config {
            properties.insert(
                "ScheduleConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SourceURI".to_string(),
            crate::value::ToValue::to_value(&self.source_uri),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appintegrations-eventintegration.html
pub struct EventIntegration_ {
    pub description: Option<crate::value::ExpString>,
    pub event_bridge_bus: crate::value::ExpString,
    pub event_filter: super::appintegrations::eventintegration::EventFilter_,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appintegrations_EventIntegration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::AppIntegrations::EventIntegration"
        $($field $value)*)
    };
}
pub use crate::__aws_appintegrations_EventIntegration as EventIntegration;
impl crate::template::ToResource for EventIntegration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppIntegrations"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("EventIntegration"),
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
            "EventBridgeBus".to_string(),
            crate::value::ToValue::to_value(&self.event_bridge_bus),
        );
        properties.insert(
            "EventFilter".to_string(),
            crate::value::ToValue::to_value(&self.event_filter),
        );
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
