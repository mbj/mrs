pub mod configurationmanager {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmquicksetup-configurationmanager-configurationdefinition.html
    pub struct ConfigurationDefinition_ {
        pub local_deployment_administration_role_arn: Option<crate::value::ExpString>,
        pub local_deployment_execution_role_name: Option<crate::value::ExpString>,
        pub parameters: std::collections::BTreeMap<String, crate::value::ExpString>,
        pub r#type: crate::value::ExpString,
        pub type_version: Option<crate::value::ExpString>,
        pub id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssmquicksetup_ConfigurationManager_ConfigurationDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SSMQuickSetup::ConfigurationManager.ConfigurationDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssmquicksetup_ConfigurationManager_ConfigurationDefinition as ConfigurationDefinition;
    impl crate::value::ToValue for ConfigurationDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.local_deployment_administration_role_arn {
                properties.insert(
                    "LocalDeploymentAdministrationRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.local_deployment_execution_role_name {
                properties.insert(
                    "LocalDeploymentExecutionRoleName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Parameters".to_string(),
                crate::value::ToValue::to_value(&self.parameters),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            if let Some(ref value) = self.type_version {
                properties.insert(
                    "TypeVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.id {
                properties.insert("id".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmquicksetup-configurationmanager-statussummary.html
    pub struct StatusSummary_ {
        pub last_updated_at: crate::value::ExpString,
        pub status: Option<crate::value::ExpString>,
        pub status_details: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub status_message: Option<crate::value::ExpString>,
        pub status_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssmquicksetup_ConfigurationManager_StatusSummary {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SSMQuickSetup::ConfigurationManager.StatusSummary"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssmquicksetup_ConfigurationManager_StatusSummary as StatusSummary;
    impl crate::value::ToValue for StatusSummary_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LastUpdatedAt".to_string(),
                crate::value::ToValue::to_value(&self.last_updated_at),
            );
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.status_details {
                properties.insert(
                    "StatusDetails".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.status_message {
                properties.insert(
                    "StatusMessage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "StatusType".to_string(),
                crate::value::ToValue::to_value(&self.status_type),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmquicksetup-configurationmanager.html
pub struct ConfigurationManager_ {
    pub configuration_definitions:
        Vec<super::ssmquicksetup::configurationmanager::ConfigurationDefinition_>,
    pub description: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ssmquicksetup_ConfigurationManager {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SSMQuickSetup::ConfigurationManager"
        $($field $value)*)
    };
}
pub use crate::__aws_ssmquicksetup_ConfigurationManager as ConfigurationManager;
impl crate::template::ToResource for ConfigurationManager_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SSMQuickSetup"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ConfigurationManager"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ConfigurationDefinitions".to_string(),
            crate::value::ToValue::to_value(&self.configuration_definitions),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
