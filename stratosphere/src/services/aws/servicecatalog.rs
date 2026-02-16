pub mod cloudformationproduct {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicecatalog-cloudformationproduct-codestarparameters.html
    pub struct CodeStarParameters_ {
        pub artifact_path: crate::value::ExpString,
        pub branch: crate::value::ExpString,
        pub connection_arn: crate::value::ExpString,
        pub repository: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_servicecatalog_CloudFormationProduct_CodeStarParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ServiceCatalog::CloudFormationProduct.CodeStarParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_servicecatalog_CloudFormationProduct_CodeStarParameters as CodeStarParameters;
    impl crate::value::ToValue for CodeStarParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ArtifactPath".to_string(),
                crate::value::ToValue::to_value(&self.artifact_path),
            );
            properties.insert(
                "Branch".to_string(),
                crate::value::ToValue::to_value(&self.branch),
            );
            properties.insert(
                "ConnectionArn".to_string(),
                crate::value::ToValue::to_value(&self.connection_arn),
            );
            properties.insert(
                "Repository".to_string(),
                crate::value::ToValue::to_value(&self.repository),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicecatalog-cloudformationproduct-sourceconnection-connectionparameters.html
    pub struct ConnectionParameters_ {
        pub code_star: Option<Box<CodeStarParameters_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_servicecatalog_CloudFormationProduct_ConnectionParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ServiceCatalog::CloudFormationProduct.ConnectionParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_servicecatalog_CloudFormationProduct_ConnectionParameters as ConnectionParameters;
    impl crate::value::ToValue for ConnectionParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.code_star {
                properties.insert(
                    "CodeStar".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicecatalog-cloudformationproduct-provisioningartifactproperties.html
    pub struct ProvisioningArtifactProperties_ {
        pub description: Option<crate::value::ExpString>,
        pub disable_template_validation: Option<crate::value::ExpBool>,
        pub info: serde_json::Value,
        pub name: Option<crate::value::ExpString>,
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_servicecatalog_CloudFormationProduct_ProvisioningArtifactProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ServiceCatalog::CloudFormationProduct.ProvisioningArtifactProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_servicecatalog_CloudFormationProduct_ProvisioningArtifactProperties as ProvisioningArtifactProperties;
    impl crate::value::ToValue for ProvisioningArtifactProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.disable_template_validation {
                properties.insert(
                    "DisableTemplateValidation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Info".to_string(),
                crate::value::ToValue::to_value(&self.info),
            );
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicecatalog-cloudformationproduct-sourceconnection.html
    pub struct SourceConnection_ {
        pub connection_parameters: Box<ConnectionParameters_>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_servicecatalog_CloudFormationProduct_SourceConnection {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ServiceCatalog::CloudFormationProduct.SourceConnection"
            $($field $value)*)
        };
    }
    pub use crate::__aws_servicecatalog_CloudFormationProduct_SourceConnection as SourceConnection;
    impl crate::value::ToValue for SourceConnection_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ConnectionParameters".to_string(),
                crate::value::ToValue::to_value(&self.connection_parameters),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
}
pub mod cloudformationprovisionedproduct {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicecatalog-cloudformationprovisionedproduct-provisioningparameter.html
    pub struct ProvisioningParameter_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_servicecatalog_CloudFormationProvisionedProduct_ProvisioningParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ServiceCatalog::CloudFormationProvisionedProduct.ProvisioningParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_servicecatalog_CloudFormationProvisionedProduct_ProvisioningParameter as ProvisioningParameter;
    impl crate::value::ToValue for ProvisioningParameter_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicecatalog-cloudformationprovisionedproduct-provisioningpreferences.html
    pub struct ProvisioningPreferences_ {
        pub stack_set_accounts: Option<Vec<crate::value::ExpString>>,
        pub stack_set_failure_tolerance_count: Option<i32>,
        pub stack_set_failure_tolerance_percentage: Option<i32>,
        pub stack_set_max_concurrency_count: Option<i32>,
        pub stack_set_max_concurrency_percentage: Option<i32>,
        pub stack_set_operation_type: Option<crate::value::ExpString>,
        pub stack_set_regions: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_servicecatalog_CloudFormationProvisionedProduct_ProvisioningPreferences {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ServiceCatalog::CloudFormationProvisionedProduct.ProvisioningPreferences"
            $($field $value)*)
        };
    }
    pub use crate::__aws_servicecatalog_CloudFormationProvisionedProduct_ProvisioningPreferences as ProvisioningPreferences;
    impl crate::value::ToValue for ProvisioningPreferences_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.stack_set_accounts {
                properties.insert(
                    "StackSetAccounts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stack_set_failure_tolerance_count {
                properties.insert(
                    "StackSetFailureToleranceCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stack_set_failure_tolerance_percentage {
                properties.insert(
                    "StackSetFailureTolerancePercentage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stack_set_max_concurrency_count {
                properties.insert(
                    "StackSetMaxConcurrencyCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stack_set_max_concurrency_percentage {
                properties.insert(
                    "StackSetMaxConcurrencyPercentage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stack_set_operation_type {
                properties.insert(
                    "StackSetOperationType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stack_set_regions {
                properties.insert(
                    "StackSetRegions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod serviceaction {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicecatalog-serviceaction-definitionparameter.html
    pub struct DefinitionParameter_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_servicecatalog_ServiceAction_DefinitionParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ServiceCatalog::ServiceAction.DefinitionParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_servicecatalog_ServiceAction_DefinitionParameter as DefinitionParameter;
    impl crate::value::ToValue for DefinitionParameter_ {
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
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-acceptedportfolioshare.html
pub struct AcceptedPortfolioShare_ {
    pub accept_language: Option<crate::value::ExpString>,
    pub portfolio_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_servicecatalog_AcceptedPortfolioShare {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ServiceCatalog::AcceptedPortfolioShare"
        $($field $value)*)
    };
}
pub use crate::__aws_servicecatalog_AcceptedPortfolioShare as AcceptedPortfolioShare;
impl crate::template::ToResource for AcceptedPortfolioShare_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ServiceCatalog"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AcceptedPortfolioShare"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.accept_language {
            properties.insert(
                "AcceptLanguage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "PortfolioId".to_string(),
            crate::value::ToValue::to_value(&self.portfolio_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-cloudformationproduct.html
pub struct CloudFormationProduct_ {
    pub accept_language: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub distributor: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub owner: crate::value::ExpString,
    pub product_type: Option<crate::value::ExpString>,
    pub provisioning_artifact_parameters:
        Option<Vec<super::servicecatalog::cloudformationproduct::ProvisioningArtifactProperties_>>,
    pub replace_provisioning_artifacts: Option<crate::value::ExpBool>,
    pub source_connection: Option<super::servicecatalog::cloudformationproduct::SourceConnection_>,
    pub support_description: Option<crate::value::ExpString>,
    pub support_email: Option<crate::value::ExpString>,
    pub support_url: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_servicecatalog_CloudFormationProduct {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ServiceCatalog::CloudFormationProduct"
        $($field $value)*)
    };
}
pub use crate::__aws_servicecatalog_CloudFormationProduct as CloudFormationProduct;
impl crate::template::ToResource for CloudFormationProduct_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ServiceCatalog"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("CloudFormationProduct"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.accept_language {
            properties.insert(
                "AcceptLanguage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.distributor {
            properties.insert(
                "Distributor".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "Owner".to_string(),
            crate::value::ToValue::to_value(&self.owner),
        );
        if let Some(ref value) = self.product_type {
            properties.insert(
                "ProductType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.provisioning_artifact_parameters {
            properties.insert(
                "ProvisioningArtifactParameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.replace_provisioning_artifacts {
            properties.insert(
                "ReplaceProvisioningArtifacts".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_connection {
            properties.insert(
                "SourceConnection".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.support_description {
            properties.insert(
                "SupportDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.support_email {
            properties.insert(
                "SupportEmail".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.support_url {
            properties.insert(
                "SupportUrl".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-cloudformationprovisionedproduct.html
pub struct CloudFormationProvisionedProduct_ {
    pub accept_language: Option<crate::value::ExpString>,
    pub notification_arns: Option<Vec<crate::value::ExpString>>,
    pub path_id: Option<crate::value::ExpString>,
    pub path_name: Option<crate::value::ExpString>,
    pub product_id: Option<crate::value::ExpString>,
    pub product_name: Option<crate::value::ExpString>,
    pub provisioned_product_name: Option<crate::value::ExpString>,
    pub provisioning_artifact_id: Option<crate::value::ExpString>,
    pub provisioning_artifact_name: Option<crate::value::ExpString>,
    pub provisioning_parameters: Option<
        Vec<super::servicecatalog::cloudformationprovisionedproduct::ProvisioningParameter_>,
    >,
    pub provisioning_preferences:
        Option<super::servicecatalog::cloudformationprovisionedproduct::ProvisioningPreferences_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_servicecatalog_CloudFormationProvisionedProduct {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ServiceCatalog::CloudFormationProvisionedProduct"
        $($field $value)*)
    };
}
pub use crate::__aws_servicecatalog_CloudFormationProvisionedProduct as CloudFormationProvisionedProduct;
impl crate::template::ToResource for CloudFormationProvisionedProduct_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ServiceCatalog"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "CloudFormationProvisionedProduct",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.accept_language {
            properties.insert(
                "AcceptLanguage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.notification_arns {
            properties.insert(
                "NotificationArns".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.path_id {
            properties.insert("PathId".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.path_name {
            properties.insert(
                "PathName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.product_id {
            properties.insert(
                "ProductId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.product_name {
            properties.insert(
                "ProductName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.provisioned_product_name {
            properties.insert(
                "ProvisionedProductName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.provisioning_artifact_id {
            properties.insert(
                "ProvisioningArtifactId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.provisioning_artifact_name {
            properties.insert(
                "ProvisioningArtifactName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.provisioning_parameters {
            properties.insert(
                "ProvisioningParameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.provisioning_preferences {
            properties.insert(
                "ProvisioningPreferences".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-launchnotificationconstraint.html
pub struct LaunchNotificationConstraint_ {
    pub accept_language: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub notification_arns: Vec<crate::value::ExpString>,
    pub portfolio_id: crate::value::ExpString,
    pub product_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_servicecatalog_LaunchNotificationConstraint {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ServiceCatalog::LaunchNotificationConstraint"
        $($field $value)*)
    };
}
pub use crate::__aws_servicecatalog_LaunchNotificationConstraint as LaunchNotificationConstraint;
impl crate::template::ToResource for LaunchNotificationConstraint_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ServiceCatalog"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "LaunchNotificationConstraint",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.accept_language {
            properties.insert(
                "AcceptLanguage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "NotificationArns".to_string(),
            crate::value::ToValue::to_value(&self.notification_arns),
        );
        properties.insert(
            "PortfolioId".to_string(),
            crate::value::ToValue::to_value(&self.portfolio_id),
        );
        properties.insert(
            "ProductId".to_string(),
            crate::value::ToValue::to_value(&self.product_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-launchroleconstraint.html
pub struct LaunchRoleConstraint_ {
    pub accept_language: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub local_role_name: Option<crate::value::ExpString>,
    pub portfolio_id: crate::value::ExpString,
    pub product_id: crate::value::ExpString,
    pub role_arn: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_servicecatalog_LaunchRoleConstraint {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ServiceCatalog::LaunchRoleConstraint"
        $($field $value)*)
    };
}
pub use crate::__aws_servicecatalog_LaunchRoleConstraint as LaunchRoleConstraint;
impl crate::template::ToResource for LaunchRoleConstraint_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ServiceCatalog"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LaunchRoleConstraint"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.accept_language {
            properties.insert(
                "AcceptLanguage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.local_role_name {
            properties.insert(
                "LocalRoleName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "PortfolioId".to_string(),
            crate::value::ToValue::to_value(&self.portfolio_id),
        );
        properties.insert(
            "ProductId".to_string(),
            crate::value::ToValue::to_value(&self.product_id),
        );
        if let Some(ref value) = self.role_arn {
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-launchtemplateconstraint.html
pub struct LaunchTemplateConstraint_ {
    pub accept_language: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub portfolio_id: crate::value::ExpString,
    pub product_id: crate::value::ExpString,
    pub rules: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_servicecatalog_LaunchTemplateConstraint {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ServiceCatalog::LaunchTemplateConstraint"
        $($field $value)*)
    };
}
pub use crate::__aws_servicecatalog_LaunchTemplateConstraint as LaunchTemplateConstraint;
impl crate::template::ToResource for LaunchTemplateConstraint_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ServiceCatalog"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LaunchTemplateConstraint"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.accept_language {
            properties.insert(
                "AcceptLanguage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "PortfolioId".to_string(),
            crate::value::ToValue::to_value(&self.portfolio_id),
        );
        properties.insert(
            "ProductId".to_string(),
            crate::value::ToValue::to_value(&self.product_id),
        );
        properties.insert(
            "Rules".to_string(),
            crate::value::ToValue::to_value(&self.rules),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-portfolio.html
pub struct Portfolio_ {
    pub accept_language: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub display_name: crate::value::ExpString,
    pub provider_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_servicecatalog_Portfolio {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ServiceCatalog::Portfolio"
        $($field $value)*)
    };
}
pub use crate::__aws_servicecatalog_Portfolio as Portfolio;
impl crate::template::ToResource for Portfolio_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ServiceCatalog"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Portfolio"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.accept_language {
            properties.insert(
                "AcceptLanguage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DisplayName".to_string(),
            crate::value::ToValue::to_value(&self.display_name),
        );
        properties.insert(
            "ProviderName".to_string(),
            crate::value::ToValue::to_value(&self.provider_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-portfolioprincipalassociation.html
pub struct PortfolioPrincipalAssociation_ {
    pub accept_language: Option<crate::value::ExpString>,
    pub portfolio_id: Option<crate::value::ExpString>,
    pub principal_arn: Option<crate::value::ExpString>,
    pub principal_type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_servicecatalog_PortfolioPrincipalAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ServiceCatalog::PortfolioPrincipalAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_servicecatalog_PortfolioPrincipalAssociation as PortfolioPrincipalAssociation;
impl crate::template::ToResource for PortfolioPrincipalAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ServiceCatalog"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "PortfolioPrincipalAssociation",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.accept_language {
            properties.insert(
                "AcceptLanguage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.portfolio_id {
            properties.insert(
                "PortfolioId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.principal_arn {
            properties.insert(
                "PrincipalARN".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "PrincipalType".to_string(),
            crate::value::ToValue::to_value(&self.principal_type),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-portfolioproductassociation.html
pub struct PortfolioProductAssociation_ {
    pub accept_language: Option<crate::value::ExpString>,
    pub portfolio_id: Option<crate::value::ExpString>,
    pub product_id: Option<crate::value::ExpString>,
    pub source_portfolio_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_servicecatalog_PortfolioProductAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ServiceCatalog::PortfolioProductAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_servicecatalog_PortfolioProductAssociation as PortfolioProductAssociation;
impl crate::template::ToResource for PortfolioProductAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ServiceCatalog"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "PortfolioProductAssociation",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.accept_language {
            properties.insert(
                "AcceptLanguage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.portfolio_id {
            properties.insert(
                "PortfolioId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.product_id {
            properties.insert(
                "ProductId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_portfolio_id {
            properties.insert(
                "SourcePortfolioId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-portfolioshare.html
pub struct PortfolioShare_ {
    pub accept_language: Option<crate::value::ExpString>,
    pub account_id: crate::value::ExpString,
    pub portfolio_id: crate::value::ExpString,
    pub share_tag_options: Option<crate::value::ExpBool>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_servicecatalog_PortfolioShare {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ServiceCatalog::PortfolioShare"
        $($field $value)*)
    };
}
pub use crate::__aws_servicecatalog_PortfolioShare as PortfolioShare;
impl crate::template::ToResource for PortfolioShare_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ServiceCatalog"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PortfolioShare"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.accept_language {
            properties.insert(
                "AcceptLanguage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "AccountId".to_string(),
            crate::value::ToValue::to_value(&self.account_id),
        );
        properties.insert(
            "PortfolioId".to_string(),
            crate::value::ToValue::to_value(&self.portfolio_id),
        );
        if let Some(ref value) = self.share_tag_options {
            properties.insert(
                "ShareTagOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-resourceupdateconstraint.html
pub struct ResourceUpdateConstraint_ {
    pub accept_language: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub portfolio_id: crate::value::ExpString,
    pub product_id: crate::value::ExpString,
    pub tag_update_on_provisioned_product: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_servicecatalog_ResourceUpdateConstraint {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ServiceCatalog::ResourceUpdateConstraint"
        $($field $value)*)
    };
}
pub use crate::__aws_servicecatalog_ResourceUpdateConstraint as ResourceUpdateConstraint;
impl crate::template::ToResource for ResourceUpdateConstraint_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ServiceCatalog"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ResourceUpdateConstraint"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.accept_language {
            properties.insert(
                "AcceptLanguage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "PortfolioId".to_string(),
            crate::value::ToValue::to_value(&self.portfolio_id),
        );
        properties.insert(
            "ProductId".to_string(),
            crate::value::ToValue::to_value(&self.product_id),
        );
        properties.insert(
            "TagUpdateOnProvisionedProduct".to_string(),
            crate::value::ToValue::to_value(&self.tag_update_on_provisioned_product),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-serviceaction.html
pub struct ServiceAction_ {
    pub accept_language: Option<crate::value::ExpString>,
    pub definition: Vec<super::servicecatalog::serviceaction::DefinitionParameter_>,
    pub definition_type: crate::value::ExpString,
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_servicecatalog_ServiceAction {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ServiceCatalog::ServiceAction"
        $($field $value)*)
    };
}
pub use crate::__aws_servicecatalog_ServiceAction as ServiceAction;
impl crate::template::ToResource for ServiceAction_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ServiceCatalog"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ServiceAction"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.accept_language {
            properties.insert(
                "AcceptLanguage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Definition".to_string(),
            crate::value::ToValue::to_value(&self.definition),
        );
        properties.insert(
            "DefinitionType".to_string(),
            crate::value::ToValue::to_value(&self.definition_type),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-serviceactionassociation.html
pub struct ServiceActionAssociation_ {
    pub product_id: crate::value::ExpString,
    pub provisioning_artifact_id: crate::value::ExpString,
    pub service_action_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_servicecatalog_ServiceActionAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ServiceCatalog::ServiceActionAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_servicecatalog_ServiceActionAssociation as ServiceActionAssociation;
impl crate::template::ToResource for ServiceActionAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ServiceCatalog"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ServiceActionAssociation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ProductId".to_string(),
            crate::value::ToValue::to_value(&self.product_id),
        );
        properties.insert(
            "ProvisioningArtifactId".to_string(),
            crate::value::ToValue::to_value(&self.provisioning_artifact_id),
        );
        properties.insert(
            "ServiceActionId".to_string(),
            crate::value::ToValue::to_value(&self.service_action_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-stacksetconstraint.html
pub struct StackSetConstraint_ {
    pub accept_language: Option<crate::value::ExpString>,
    pub account_list: Vec<crate::value::ExpString>,
    pub admin_role: crate::value::ExpString,
    pub description: crate::value::ExpString,
    pub execution_role: crate::value::ExpString,
    pub portfolio_id: crate::value::ExpString,
    pub product_id: crate::value::ExpString,
    pub region_list: Vec<crate::value::ExpString>,
    pub stack_instance_control: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_servicecatalog_StackSetConstraint {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ServiceCatalog::StackSetConstraint"
        $($field $value)*)
    };
}
pub use crate::__aws_servicecatalog_StackSetConstraint as StackSetConstraint;
impl crate::template::ToResource for StackSetConstraint_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ServiceCatalog"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("StackSetConstraint"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.accept_language {
            properties.insert(
                "AcceptLanguage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "AccountList".to_string(),
            crate::value::ToValue::to_value(&self.account_list),
        );
        properties.insert(
            "AdminRole".to_string(),
            crate::value::ToValue::to_value(&self.admin_role),
        );
        properties.insert(
            "Description".to_string(),
            crate::value::ToValue::to_value(&self.description),
        );
        properties.insert(
            "ExecutionRole".to_string(),
            crate::value::ToValue::to_value(&self.execution_role),
        );
        properties.insert(
            "PortfolioId".to_string(),
            crate::value::ToValue::to_value(&self.portfolio_id),
        );
        properties.insert(
            "ProductId".to_string(),
            crate::value::ToValue::to_value(&self.product_id),
        );
        properties.insert(
            "RegionList".to_string(),
            crate::value::ToValue::to_value(&self.region_list),
        );
        properties.insert(
            "StackInstanceControl".to_string(),
            crate::value::ToValue::to_value(&self.stack_instance_control),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-tagoption.html
pub struct TagOption_ {
    pub active: Option<crate::value::ExpBool>,
    pub key: crate::value::ExpString,
    pub value: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_servicecatalog_TagOption {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ServiceCatalog::TagOption"
        $($field $value)*)
    };
}
pub use crate::__aws_servicecatalog_TagOption as TagOption;
impl crate::template::ToResource for TagOption_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ServiceCatalog"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TagOption"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.active {
            properties.insert("Active".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Key".to_string(),
            crate::value::ToValue::to_value(&self.key),
        );
        properties.insert(
            "Value".to_string(),
            crate::value::ToValue::to_value(&self.value),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-tagoptionassociation.html
pub struct TagOptionAssociation_ {
    pub resource_id: Option<crate::value::ExpString>,
    pub tag_option_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_servicecatalog_TagOptionAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ServiceCatalog::TagOptionAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_servicecatalog_TagOptionAssociation as TagOptionAssociation;
impl crate::template::ToResource for TagOptionAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ServiceCatalog"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TagOptionAssociation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.resource_id {
            properties.insert(
                "ResourceId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tag_option_id {
            properties.insert(
                "TagOptionId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
