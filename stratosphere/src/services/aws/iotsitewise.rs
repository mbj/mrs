pub mod accesspolicy {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-accesspolicy-accesspolicyidentity.html>
    pub struct AccessPolicyIdentity_ {
        pub iam_role: Option<Box<IamRole_>>,
        pub iam_user: Option<Box<IamUser_>>,
        pub user: Option<Box<User_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotsitewise_AccessPolicy_AccessPolicyIdentity {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTSiteWise::AccessPolicy.AccessPolicyIdentity"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotsitewise_AccessPolicy_AccessPolicyIdentity as AccessPolicyIdentity;
    impl crate::value::ToValue for AccessPolicyIdentity_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.iam_role {
                properties.insert(
                    "IamRole".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.iam_user {
                properties.insert(
                    "IamUser".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user {
                properties.insert("User".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-accesspolicy-accesspolicyresource.html>
    pub struct AccessPolicyResource_ {
        pub portal: Option<Box<Portal_>>,
        pub project: Option<Box<Project_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotsitewise_AccessPolicy_AccessPolicyResource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTSiteWise::AccessPolicy.AccessPolicyResource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotsitewise_AccessPolicy_AccessPolicyResource as AccessPolicyResource;
    impl crate::value::ToValue for AccessPolicyResource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.portal {
                properties.insert("Portal".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.project {
                properties.insert(
                    "Project".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-accesspolicy-iamrole.html>
    pub struct IamRole_ {
        pub arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotsitewise_AccessPolicy_IamRole {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTSiteWise::AccessPolicy.IamRole"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotsitewise_AccessPolicy_IamRole as IamRole;
    impl crate::value::ToValue for IamRole_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.arn {
                properties.insert("arn".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-accesspolicy-iamuser.html>
    pub struct IamUser_ {
        pub arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotsitewise_AccessPolicy_IamUser {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTSiteWise::AccessPolicy.IamUser"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotsitewise_AccessPolicy_IamUser as IamUser;
    impl crate::value::ToValue for IamUser_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.arn {
                properties.insert("arn".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-accesspolicy-portal.html>
    pub struct Portal_ {
        pub id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotsitewise_AccessPolicy_Portal {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTSiteWise::AccessPolicy.Portal"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotsitewise_AccessPolicy_Portal as Portal;
    impl crate::value::ToValue for Portal_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.id {
                properties.insert("id".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-accesspolicy-project.html>
    pub struct Project_ {
        pub id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotsitewise_AccessPolicy_Project {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTSiteWise::AccessPolicy.Project"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotsitewise_AccessPolicy_Project as Project;
    impl crate::value::ToValue for Project_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.id {
                properties.insert("id".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-accesspolicy-user.html>
    pub struct User_ {
        pub id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotsitewise_AccessPolicy_User {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTSiteWise::AccessPolicy.User"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotsitewise_AccessPolicy_User as User;
    impl crate::value::ToValue for User_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.id {
                properties.insert("id".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod asset {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-asset-assethierarchy.html>
    pub struct AssetHierarchy_ {
        pub child_asset_id: crate::value::ExpString,
        pub external_id: Option<crate::value::ExpString>,
        pub id: Option<crate::value::ExpString>,
        pub logical_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotsitewise_Asset_AssetHierarchy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTSiteWise::Asset.AssetHierarchy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotsitewise_Asset_AssetHierarchy as AssetHierarchy;
    impl crate::value::ToValue for AssetHierarchy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ChildAssetId".to_string(),
                crate::value::ToValue::to_value(&self.child_asset_id),
            );
            if let Some(ref value) = self.external_id {
                properties.insert(
                    "ExternalId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.id {
                properties.insert("Id".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.logical_id {
                properties.insert(
                    "LogicalId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-asset-assetproperty.html>
    pub struct AssetProperty_ {
        pub alias: Option<crate::value::ExpString>,
        pub external_id: Option<crate::value::ExpString>,
        pub id: Option<crate::value::ExpString>,
        pub logical_id: Option<crate::value::ExpString>,
        pub notification_state: Option<crate::value::ExpString>,
        pub unit: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotsitewise_Asset_AssetProperty {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTSiteWise::Asset.AssetProperty"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotsitewise_Asset_AssetProperty as AssetProperty;
    impl crate::value::ToValue for AssetProperty_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.alias {
                properties.insert("Alias".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.external_id {
                properties.insert(
                    "ExternalId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.id {
                properties.insert("Id".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.logical_id {
                properties.insert(
                    "LogicalId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.notification_state {
                properties.insert(
                    "NotificationState".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.unit {
                properties.insert("Unit".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod assetmodel {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-assetmodelcompositemodel.html>
    pub struct AssetModelCompositeModel_ {
        pub composed_asset_model_id: Option<crate::value::ExpString>,
        pub composite_model_properties: Option<Vec<AssetModelProperty_>>,
        pub description: Option<crate::value::ExpString>,
        pub external_id: Option<crate::value::ExpString>,
        pub id: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
        pub parent_asset_model_composite_model_external_id: Option<crate::value::ExpString>,
        pub path: Option<Vec<crate::value::ExpString>>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotsitewise_AssetModel_AssetModelCompositeModel {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTSiteWise::AssetModel.AssetModelCompositeModel"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotsitewise_AssetModel_AssetModelCompositeModel as AssetModelCompositeModel;
    impl crate::value::ToValue for AssetModelCompositeModel_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.composed_asset_model_id {
                properties.insert(
                    "ComposedAssetModelId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.composite_model_properties {
                properties.insert(
                    "CompositeModelProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.external_id {
                properties.insert(
                    "ExternalId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.id {
                properties.insert("Id".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.parent_asset_model_composite_model_external_id {
                properties.insert(
                    "ParentAssetModelCompositeModelExternalId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.path {
                properties.insert("Path".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-assetmodelhierarchy.html>
    pub struct AssetModelHierarchy_ {
        pub child_asset_model_id: crate::value::ExpString,
        pub external_id: Option<crate::value::ExpString>,
        pub id: Option<crate::value::ExpString>,
        pub logical_id: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotsitewise_AssetModel_AssetModelHierarchy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTSiteWise::AssetModel.AssetModelHierarchy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotsitewise_AssetModel_AssetModelHierarchy as AssetModelHierarchy;
    impl crate::value::ToValue for AssetModelHierarchy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ChildAssetModelId".to_string(),
                crate::value::ToValue::to_value(&self.child_asset_model_id),
            );
            if let Some(ref value) = self.external_id {
                properties.insert(
                    "ExternalId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.id {
                properties.insert("Id".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.logical_id {
                properties.insert(
                    "LogicalId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-assetmodelproperty.html>
    pub struct AssetModelProperty_ {
        pub data_type: crate::value::ExpString,
        pub data_type_spec: Option<crate::value::ExpString>,
        pub external_id: Option<crate::value::ExpString>,
        pub id: Option<crate::value::ExpString>,
        pub logical_id: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
        pub r#type: Box<PropertyType_>,
        pub unit: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotsitewise_AssetModel_AssetModelProperty {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTSiteWise::AssetModel.AssetModelProperty"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotsitewise_AssetModel_AssetModelProperty as AssetModelProperty;
    impl crate::value::ToValue for AssetModelProperty_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DataType".to_string(),
                crate::value::ToValue::to_value(&self.data_type),
            );
            if let Some(ref value) = self.data_type_spec {
                properties.insert(
                    "DataTypeSpec".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.external_id {
                properties.insert(
                    "ExternalId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.id {
                properties.insert("Id".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.logical_id {
                properties.insert(
                    "LogicalId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            if let Some(ref value) = self.unit {
                properties.insert("Unit".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-attribute.html>
    pub struct Attribute_ {
        pub default_value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotsitewise_AssetModel_Attribute {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTSiteWise::AssetModel.Attribute"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotsitewise_AssetModel_Attribute as Attribute;
    impl crate::value::ToValue for Attribute_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.default_value {
                properties.insert(
                    "DefaultValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-enforcedassetmodelinterfacepropertymapping.html>
    pub struct EnforcedAssetModelInterfacePropertyMapping_ {
        pub asset_model_property_external_id: Option<crate::value::ExpString>,
        pub asset_model_property_logical_id: Option<crate::value::ExpString>,
        pub interface_asset_model_property_external_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotsitewise_AssetModel_EnforcedAssetModelInterfacePropertyMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTSiteWise::AssetModel.EnforcedAssetModelInterfacePropertyMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotsitewise_AssetModel_EnforcedAssetModelInterfacePropertyMapping as EnforcedAssetModelInterfacePropertyMapping;
    impl crate::value::ToValue for EnforcedAssetModelInterfacePropertyMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.asset_model_property_external_id {
                properties.insert(
                    "AssetModelPropertyExternalId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.asset_model_property_logical_id {
                properties.insert(
                    "AssetModelPropertyLogicalId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "InterfaceAssetModelPropertyExternalId".to_string(),
                crate::value::ToValue::to_value(&self.interface_asset_model_property_external_id),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-enforcedassetmodelinterfacerelationship.html>
    pub struct EnforcedAssetModelInterfaceRelationship_ {
        pub interface_asset_model_id: Option<crate::value::ExpString>,
        pub property_mappings: Option<Vec<EnforcedAssetModelInterfacePropertyMapping_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotsitewise_AssetModel_EnforcedAssetModelInterfaceRelationship {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTSiteWise::AssetModel.EnforcedAssetModelInterfaceRelationship"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotsitewise_AssetModel_EnforcedAssetModelInterfaceRelationship as EnforcedAssetModelInterfaceRelationship;
    impl crate::value::ToValue for EnforcedAssetModelInterfaceRelationship_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.interface_asset_model_id {
                properties.insert(
                    "InterfaceAssetModelId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.property_mappings {
                properties.insert(
                    "PropertyMappings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-expressionvariable.html>
    pub struct ExpressionVariable_ {
        pub name: crate::value::ExpString,
        pub value: Box<VariableValue_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotsitewise_AssetModel_ExpressionVariable {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTSiteWise::AssetModel.ExpressionVariable"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotsitewise_AssetModel_ExpressionVariable as ExpressionVariable;
    impl crate::value::ToValue for ExpressionVariable_ {
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-metric.html>
    pub struct Metric_ {
        pub expression: crate::value::ExpString,
        pub variables: Vec<ExpressionVariable_>,
        pub window: Box<MetricWindow_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotsitewise_AssetModel_Metric {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTSiteWise::AssetModel.Metric"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotsitewise_AssetModel_Metric as Metric;
    impl crate::value::ToValue for Metric_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Expression".to_string(),
                crate::value::ToValue::to_value(&self.expression),
            );
            properties.insert(
                "Variables".to_string(),
                crate::value::ToValue::to_value(&self.variables),
            );
            properties.insert(
                "Window".to_string(),
                crate::value::ToValue::to_value(&self.window),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-metricwindow.html>
    pub struct MetricWindow_ {
        pub tumbling: Option<Box<TumblingWindow_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotsitewise_AssetModel_MetricWindow {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTSiteWise::AssetModel.MetricWindow"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotsitewise_AssetModel_MetricWindow as MetricWindow;
    impl crate::value::ToValue for MetricWindow_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.tumbling {
                properties.insert(
                    "Tumbling".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-propertypathdefinition.html>
    pub struct PropertyPathDefinition_ {
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotsitewise_AssetModel_PropertyPathDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTSiteWise::AssetModel.PropertyPathDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotsitewise_AssetModel_PropertyPathDefinition as PropertyPathDefinition;
    impl crate::value::ToValue for PropertyPathDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-propertytype.html>
    pub struct PropertyType_ {
        pub attribute: Option<Box<Attribute_>>,
        pub metric: Option<Box<Metric_>>,
        pub transform: Option<Box<Transform_>>,
        pub type_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotsitewise_AssetModel_PropertyType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTSiteWise::AssetModel.PropertyType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotsitewise_AssetModel_PropertyType as PropertyType;
    impl crate::value::ToValue for PropertyType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attribute {
                properties.insert(
                    "Attribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metric {
                properties.insert("Metric".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.transform {
                properties.insert(
                    "Transform".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TypeName".to_string(),
                crate::value::ToValue::to_value(&self.type_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-transform.html>
    pub struct Transform_ {
        pub expression: crate::value::ExpString,
        pub variables: Vec<ExpressionVariable_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotsitewise_AssetModel_Transform {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTSiteWise::AssetModel.Transform"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotsitewise_AssetModel_Transform as Transform;
    impl crate::value::ToValue for Transform_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Expression".to_string(),
                crate::value::ToValue::to_value(&self.expression),
            );
            properties.insert(
                "Variables".to_string(),
                crate::value::ToValue::to_value(&self.variables),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-tumblingwindow.html>
    pub struct TumblingWindow_ {
        pub interval: crate::value::ExpString,
        pub offset: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotsitewise_AssetModel_TumblingWindow {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTSiteWise::AssetModel.TumblingWindow"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotsitewise_AssetModel_TumblingWindow as TumblingWindow;
    impl crate::value::ToValue for TumblingWindow_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Interval".to_string(),
                crate::value::ToValue::to_value(&self.interval),
            );
            if let Some(ref value) = self.offset {
                properties.insert("Offset".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-assetmodel-variablevalue.html>
    pub struct VariableValue_ {
        pub hierarchy_external_id: Option<crate::value::ExpString>,
        pub hierarchy_id: Option<crate::value::ExpString>,
        pub hierarchy_logical_id: Option<crate::value::ExpString>,
        pub property_external_id: Option<crate::value::ExpString>,
        pub property_id: Option<crate::value::ExpString>,
        pub property_logical_id: Option<crate::value::ExpString>,
        pub property_path: Option<Vec<PropertyPathDefinition_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotsitewise_AssetModel_VariableValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTSiteWise::AssetModel.VariableValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotsitewise_AssetModel_VariableValue as VariableValue;
    impl crate::value::ToValue for VariableValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.hierarchy_external_id {
                properties.insert(
                    "HierarchyExternalId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hierarchy_id {
                properties.insert(
                    "HierarchyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hierarchy_logical_id {
                properties.insert(
                    "HierarchyLogicalId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.property_external_id {
                properties.insert(
                    "PropertyExternalId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.property_id {
                properties.insert(
                    "PropertyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.property_logical_id {
                properties.insert(
                    "PropertyLogicalId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.property_path {
                properties.insert(
                    "PropertyPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod computationmodel {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-computationmodel-anomalydetectioncomputationmodelconfiguration.html>
    pub struct AnomalyDetectionComputationModelConfiguration_ {
        pub input_properties: crate::value::ExpString,
        pub result_property: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotsitewise_ComputationModel_AnomalyDetectionComputationModelConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTSiteWise::ComputationModel.AnomalyDetectionComputationModelConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotsitewise_ComputationModel_AnomalyDetectionComputationModelConfiguration as AnomalyDetectionComputationModelConfiguration;
    impl crate::value::ToValue for AnomalyDetectionComputationModelConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InputProperties".to_string(),
                crate::value::ToValue::to_value(&self.input_properties),
            );
            properties.insert(
                "ResultProperty".to_string(),
                crate::value::ToValue::to_value(&self.result_property),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-computationmodel-assetmodelpropertybindingvalue.html>
    pub struct AssetModelPropertyBindingValue_ {
        pub asset_model_id: crate::value::ExpString,
        pub property_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotsitewise_ComputationModel_AssetModelPropertyBindingValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTSiteWise::ComputationModel.AssetModelPropertyBindingValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotsitewise_ComputationModel_AssetModelPropertyBindingValue as AssetModelPropertyBindingValue;
    impl crate::value::ToValue for AssetModelPropertyBindingValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AssetModelId".to_string(),
                crate::value::ToValue::to_value(&self.asset_model_id),
            );
            properties.insert(
                "PropertyId".to_string(),
                crate::value::ToValue::to_value(&self.property_id),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-computationmodel-assetpropertybindingvalue.html>
    pub struct AssetPropertyBindingValue_ {
        pub asset_id: crate::value::ExpString,
        pub property_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotsitewise_ComputationModel_AssetPropertyBindingValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTSiteWise::ComputationModel.AssetPropertyBindingValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotsitewise_ComputationModel_AssetPropertyBindingValue as AssetPropertyBindingValue;
    impl crate::value::ToValue for AssetPropertyBindingValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AssetId".to_string(),
                crate::value::ToValue::to_value(&self.asset_id),
            );
            properties.insert(
                "PropertyId".to_string(),
                crate::value::ToValue::to_value(&self.property_id),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-computationmodel-computationmodelconfiguration.html>
    pub struct ComputationModelConfiguration_ {
        pub anomaly_detection: Option<Box<AnomalyDetectionComputationModelConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotsitewise_ComputationModel_ComputationModelConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTSiteWise::ComputationModel.ComputationModelConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotsitewise_ComputationModel_ComputationModelConfiguration as ComputationModelConfiguration;
    impl crate::value::ToValue for ComputationModelConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.anomaly_detection {
                properties.insert(
                    "AnomalyDetection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-computationmodel-computationmodeldatabindingvalue.html>
    pub struct ComputationModelDataBindingValue_ {
        pub asset_model_property: Option<Box<AssetModelPropertyBindingValue_>>,
        pub asset_property: Option<Box<AssetPropertyBindingValue_>>,
        pub list: Option<Vec<ComputationModelDataBindingValue_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotsitewise_ComputationModel_ComputationModelDataBindingValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTSiteWise::ComputationModel.ComputationModelDataBindingValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotsitewise_ComputationModel_ComputationModelDataBindingValue as ComputationModelDataBindingValue;
    impl crate::value::ToValue for ComputationModelDataBindingValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.asset_model_property {
                properties.insert(
                    "AssetModelProperty".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.asset_property {
                properties.insert(
                    "AssetProperty".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.list {
                properties.insert("List".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod dataset {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-dataset-datasetsource.html>
    pub struct DatasetSource_ {
        pub source_detail: Option<Box<SourceDetail_>>,
        pub source_format: crate::value::ExpString,
        pub source_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotsitewise_Dataset_DatasetSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTSiteWise::Dataset.DatasetSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotsitewise_Dataset_DatasetSource as DatasetSource;
    impl crate::value::ToValue for DatasetSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.source_detail {
                properties.insert(
                    "SourceDetail".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SourceFormat".to_string(),
                crate::value::ToValue::to_value(&self.source_format),
            );
            properties.insert(
                "SourceType".to_string(),
                crate::value::ToValue::to_value(&self.source_type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-dataset-kendrasourcedetail.html>
    pub struct KendraSourceDetail_ {
        pub knowledge_base_arn: crate::value::ExpString,
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotsitewise_Dataset_KendraSourceDetail {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTSiteWise::Dataset.KendraSourceDetail"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotsitewise_Dataset_KendraSourceDetail as KendraSourceDetail;
    impl crate::value::ToValue for KendraSourceDetail_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "KnowledgeBaseArn".to_string(),
                crate::value::ToValue::to_value(&self.knowledge_base_arn),
            );
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-dataset-sourcedetail.html>
    pub struct SourceDetail_ {
        pub kendra: Option<Box<KendraSourceDetail_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotsitewise_Dataset_SourceDetail {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTSiteWise::Dataset.SourceDetail"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotsitewise_Dataset_SourceDetail as SourceDetail;
    impl crate::value::ToValue for SourceDetail_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kendra {
                properties.insert("Kendra".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod gateway {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-gateway-gatewaycapabilitysummary.html>
    pub struct GatewayCapabilitySummary_ {
        pub capability_configuration: Option<crate::value::ExpString>,
        pub capability_namespace: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotsitewise_Gateway_GatewayCapabilitySummary {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTSiteWise::Gateway.GatewayCapabilitySummary"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotsitewise_Gateway_GatewayCapabilitySummary as GatewayCapabilitySummary;
    impl crate::value::ToValue for GatewayCapabilitySummary_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.capability_configuration {
                properties.insert(
                    "CapabilityConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "CapabilityNamespace".to_string(),
                crate::value::ToValue::to_value(&self.capability_namespace),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-gateway-gatewayplatform.html>
    pub struct GatewayPlatform_ {
        pub greengrass_v2: Option<Box<GreengrassV2_>>,
        pub siemens_ie: Option<Box<SiemensIE_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotsitewise_Gateway_GatewayPlatform {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTSiteWise::Gateway.GatewayPlatform"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotsitewise_Gateway_GatewayPlatform as GatewayPlatform;
    impl crate::value::ToValue for GatewayPlatform_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.greengrass_v2 {
                properties.insert(
                    "GreengrassV2".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.siemens_ie {
                properties.insert(
                    "SiemensIE".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-gateway-greengrassv2.html>
    pub struct GreengrassV2_ {
        pub core_device_operating_system: Option<crate::value::ExpString>,
        pub core_device_thing_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotsitewise_Gateway_GreengrassV2 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTSiteWise::Gateway.GreengrassV2"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotsitewise_Gateway_GreengrassV2 as GreengrassV2;
    impl crate::value::ToValue for GreengrassV2_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.core_device_operating_system {
                properties.insert(
                    "CoreDeviceOperatingSystem".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "CoreDeviceThingName".to_string(),
                crate::value::ToValue::to_value(&self.core_device_thing_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-gateway-siemensie.html>
    pub struct SiemensIE_ {
        pub iot_core_thing_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotsitewise_Gateway_SiemensIE {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTSiteWise::Gateway.SiemensIE"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotsitewise_Gateway_SiemensIE as SiemensIE;
    impl crate::value::ToValue for SiemensIE_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IotCoreThingName".to_string(),
                crate::value::ToValue::to_value(&self.iot_core_thing_name),
            );
            properties.into()
        }
    }
}
pub mod portal {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-portal-alarms.html>
    pub struct Alarms_ {
        pub alarm_role_arn: Option<crate::value::ExpString>,
        pub notification_lambda_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotsitewise_Portal_Alarms {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTSiteWise::Portal.Alarms"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotsitewise_Portal_Alarms as Alarms;
    impl crate::value::ToValue for Alarms_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.alarm_role_arn {
                properties.insert(
                    "AlarmRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.notification_lambda_arn {
                properties.insert(
                    "NotificationLambdaArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotsitewise-portal-portaltypeentry.html>
    pub struct PortalTypeEntry_ {
        pub portal_tools: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotsitewise_Portal_PortalTypeEntry {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTSiteWise::Portal.PortalTypeEntry"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotsitewise_Portal_PortalTypeEntry as PortalTypeEntry;
    impl crate::value::ToValue for PortalTypeEntry_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PortalTools".to_string(),
                crate::value::ToValue::to_value(&self.portal_tools),
            );
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-accesspolicy.html>
pub struct AccessPolicy_ {
    pub access_policy_identity: super::iotsitewise::accesspolicy::AccessPolicyIdentity_,
    pub access_policy_permission: crate::value::ExpString,
    pub access_policy_resource: super::iotsitewise::accesspolicy::AccessPolicyResource_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iotsitewise_AccessPolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTSiteWise::AccessPolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_iotsitewise_AccessPolicy as AccessPolicy;
impl crate::template::ToResource for AccessPolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTSiteWise"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AccessPolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AccessPolicyIdentity".to_string(),
            crate::value::ToValue::to_value(&self.access_policy_identity),
        );
        properties.insert(
            "AccessPolicyPermission".to_string(),
            crate::value::ToValue::to_value(&self.access_policy_permission),
        );
        properties.insert(
            "AccessPolicyResource".to_string(),
            crate::value::ToValue::to_value(&self.access_policy_resource),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-asset.html>
pub struct Asset_ {
    pub asset_description: Option<crate::value::ExpString>,
    pub asset_external_id: Option<crate::value::ExpString>,
    pub asset_hierarchies: Option<Vec<super::iotsitewise::asset::AssetHierarchy_>>,
    pub asset_model_id: crate::value::ExpString,
    pub asset_name: crate::value::ExpString,
    pub asset_properties: Option<Vec<super::iotsitewise::asset::AssetProperty_>>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iotsitewise_Asset {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTSiteWise::Asset"
        $($field $value)*)
    };
}
pub use crate::__aws_iotsitewise_Asset as Asset;
impl crate::template::ToResource for Asset_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTSiteWise"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Asset"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.asset_description {
            properties.insert(
                "AssetDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.asset_external_id {
            properties.insert(
                "AssetExternalId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.asset_hierarchies {
            properties.insert(
                "AssetHierarchies".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "AssetModelId".to_string(),
            crate::value::ToValue::to_value(&self.asset_model_id),
        );
        properties.insert(
            "AssetName".to_string(),
            crate::value::ToValue::to_value(&self.asset_name),
        );
        if let Some(ref value) = self.asset_properties {
            properties.insert(
                "AssetProperties".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-assetmodel.html>
pub struct AssetModel_ {
    pub asset_model_composite_models:
        Option<Vec<super::iotsitewise::assetmodel::AssetModelCompositeModel_>>,
    pub asset_model_description: Option<crate::value::ExpString>,
    pub asset_model_external_id: Option<crate::value::ExpString>,
    pub asset_model_hierarchies: Option<Vec<super::iotsitewise::assetmodel::AssetModelHierarchy_>>,
    pub asset_model_name: crate::value::ExpString,
    pub asset_model_properties: Option<Vec<super::iotsitewise::assetmodel::AssetModelProperty_>>,
    pub asset_model_type: Option<crate::value::ExpString>,
    pub enforced_asset_model_interface_relationships:
        Option<Vec<super::iotsitewise::assetmodel::EnforcedAssetModelInterfaceRelationship_>>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iotsitewise_AssetModel {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTSiteWise::AssetModel"
        $($field $value)*)
    };
}
pub use crate::__aws_iotsitewise_AssetModel as AssetModel;
impl crate::template::ToResource for AssetModel_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTSiteWise"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AssetModel"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.asset_model_composite_models {
            properties.insert(
                "AssetModelCompositeModels".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.asset_model_description {
            properties.insert(
                "AssetModelDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.asset_model_external_id {
            properties.insert(
                "AssetModelExternalId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.asset_model_hierarchies {
            properties.insert(
                "AssetModelHierarchies".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "AssetModelName".to_string(),
            crate::value::ToValue::to_value(&self.asset_model_name),
        );
        if let Some(ref value) = self.asset_model_properties {
            properties.insert(
                "AssetModelProperties".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.asset_model_type {
            properties.insert(
                "AssetModelType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enforced_asset_model_interface_relationships {
            properties.insert(
                "EnforcedAssetModelInterfaceRelationships".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-computationmodel.html>
pub struct ComputationModel_ {
    pub computation_model_configuration:
        super::iotsitewise::computationmodel::ComputationModelConfiguration_,
    pub computation_model_data_binding: std::collections::BTreeMap<
        String,
        super::iotsitewise::computationmodel::ComputationModelDataBindingValue_,
    >,
    pub computation_model_description: Option<crate::value::ExpString>,
    pub computation_model_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iotsitewise_ComputationModel {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTSiteWise::ComputationModel"
        $($field $value)*)
    };
}
pub use crate::__aws_iotsitewise_ComputationModel as ComputationModel;
impl crate::template::ToResource for ComputationModel_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTSiteWise"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ComputationModel"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ComputationModelConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.computation_model_configuration),
        );
        properties.insert(
            "ComputationModelDataBinding".to_string(),
            crate::value::ToValue::to_value(&self.computation_model_data_binding),
        );
        if let Some(ref value) = self.computation_model_description {
            properties.insert(
                "ComputationModelDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ComputationModelName".to_string(),
            crate::value::ToValue::to_value(&self.computation_model_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-dashboard.html>
pub struct Dashboard_ {
    pub dashboard_definition: crate::value::ExpString,
    pub dashboard_description: crate::value::ExpString,
    pub dashboard_name: crate::value::ExpString,
    pub project_id: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iotsitewise_Dashboard {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTSiteWise::Dashboard"
        $($field $value)*)
    };
}
pub use crate::__aws_iotsitewise_Dashboard as Dashboard;
impl crate::template::ToResource for Dashboard_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTSiteWise"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Dashboard"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DashboardDefinition".to_string(),
            crate::value::ToValue::to_value(&self.dashboard_definition),
        );
        properties.insert(
            "DashboardDescription".to_string(),
            crate::value::ToValue::to_value(&self.dashboard_description),
        );
        properties.insert(
            "DashboardName".to_string(),
            crate::value::ToValue::to_value(&self.dashboard_name),
        );
        if let Some(ref value) = self.project_id {
            properties.insert(
                "ProjectId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-dataset.html>
pub struct Dataset_ {
    pub dataset_description: Option<crate::value::ExpString>,
    pub dataset_name: crate::value::ExpString,
    pub dataset_source: super::iotsitewise::dataset::DatasetSource_,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iotsitewise_Dataset {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTSiteWise::Dataset"
        $($field $value)*)
    };
}
pub use crate::__aws_iotsitewise_Dataset as Dataset;
impl crate::template::ToResource for Dataset_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTSiteWise"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Dataset"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.dataset_description {
            properties.insert(
                "DatasetDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DatasetName".to_string(),
            crate::value::ToValue::to_value(&self.dataset_name),
        );
        properties.insert(
            "DatasetSource".to_string(),
            crate::value::ToValue::to_value(&self.dataset_source),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-gateway.html>
pub struct Gateway_ {
    pub gateway_capability_summaries:
        Option<Vec<super::iotsitewise::gateway::GatewayCapabilitySummary_>>,
    pub gateway_name: crate::value::ExpString,
    pub gateway_platform: super::iotsitewise::gateway::GatewayPlatform_,
    pub gateway_version: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iotsitewise_Gateway {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTSiteWise::Gateway"
        $($field $value)*)
    };
}
pub use crate::__aws_iotsitewise_Gateway as Gateway;
impl crate::template::ToResource for Gateway_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTSiteWise"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Gateway"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.gateway_capability_summaries {
            properties.insert(
                "GatewayCapabilitySummaries".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "GatewayName".to_string(),
            crate::value::ToValue::to_value(&self.gateway_name),
        );
        properties.insert(
            "GatewayPlatform".to_string(),
            crate::value::ToValue::to_value(&self.gateway_platform),
        );
        if let Some(ref value) = self.gateway_version {
            properties.insert(
                "GatewayVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-portal.html>
pub struct Portal_ {
    pub alarms: Option<super::iotsitewise::portal::Alarms_>,
    pub notification_sender_email: Option<crate::value::ExpString>,
    pub portal_auth_mode: Option<crate::value::ExpString>,
    pub portal_contact_email: crate::value::ExpString,
    pub portal_description: Option<crate::value::ExpString>,
    pub portal_name: crate::value::ExpString,
    pub portal_type: Option<crate::value::ExpString>,
    pub portal_type_configuration:
        Option<std::collections::BTreeMap<String, super::iotsitewise::portal::PortalTypeEntry_>>,
    pub role_arn: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iotsitewise_Portal {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTSiteWise::Portal"
        $($field $value)*)
    };
}
pub use crate::__aws_iotsitewise_Portal as Portal;
impl crate::template::ToResource for Portal_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTSiteWise"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Portal"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.alarms {
            properties.insert("Alarms".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.notification_sender_email {
            properties.insert(
                "NotificationSenderEmail".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.portal_auth_mode {
            properties.insert(
                "PortalAuthMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "PortalContactEmail".to_string(),
            crate::value::ToValue::to_value(&self.portal_contact_email),
        );
        if let Some(ref value) = self.portal_description {
            properties.insert(
                "PortalDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "PortalName".to_string(),
            crate::value::ToValue::to_value(&self.portal_name),
        );
        if let Some(ref value) = self.portal_type {
            properties.insert(
                "PortalType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.portal_type_configuration {
            properties.insert(
                "PortalTypeConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotsitewise-project.html>
pub struct Project_ {
    pub asset_ids: Option<Vec<crate::value::ExpString>>,
    pub portal_id: crate::value::ExpString,
    pub project_description: Option<crate::value::ExpString>,
    pub project_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iotsitewise_Project {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTSiteWise::Project"
        $($field $value)*)
    };
}
pub use crate::__aws_iotsitewise_Project as Project;
impl crate::template::ToResource for Project_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTSiteWise"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Project"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.asset_ids {
            properties.insert(
                "AssetIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "PortalId".to_string(),
            crate::value::ToValue::to_value(&self.portal_id),
        );
        if let Some(ref value) = self.project_description {
            properties.insert(
                "ProjectDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ProjectName".to_string(),
            crate::value::ToValue::to_value(&self.project_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
