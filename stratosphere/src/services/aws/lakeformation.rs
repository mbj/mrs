pub mod datacellsfilter {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-datacellsfilter-columnwildcard.html
    pub struct ColumnWildcard_ {
        pub excluded_column_names: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lakeformation_DataCellsFilter_ColumnWildcard {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LakeFormation::DataCellsFilter.ColumnWildcard"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lakeformation_DataCellsFilter_ColumnWildcard as ColumnWildcard;
    impl crate::value::ToValue for ColumnWildcard_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.excluded_column_names {
                properties.insert(
                    "ExcludedColumnNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-datacellsfilter-rowfilter.html
    pub struct RowFilter_ {
        pub all_rows_wildcard: Option<serde_json::Value>,
        pub filter_expression: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lakeformation_DataCellsFilter_RowFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LakeFormation::DataCellsFilter.RowFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lakeformation_DataCellsFilter_RowFilter as RowFilter;
    impl crate::value::ToValue for RowFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.all_rows_wildcard {
                properties.insert(
                    "AllRowsWildcard".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.filter_expression {
                properties.insert(
                    "FilterExpression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod datalakesettings {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-datalakesettings-admins.html
    pub struct Admins_ {}
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lakeformation_DataLakeSettings_Admins {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LakeFormation::DataLakeSettings.Admins"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lakeformation_DataLakeSettings_Admins as Admins;
    impl crate::value::ToValue for Admins_ {
        fn to_value(&self) -> serde_json::Value {
            serde_json::Value::Object(serde_json::Map::new())
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-datalakesettings-createdatabasedefaultpermissions.html
    pub struct CreateDatabaseDefaultPermissions_ {}
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lakeformation_DataLakeSettings_CreateDatabaseDefaultPermissions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LakeFormation::DataLakeSettings.CreateDatabaseDefaultPermissions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lakeformation_DataLakeSettings_CreateDatabaseDefaultPermissions as CreateDatabaseDefaultPermissions;
    impl crate::value::ToValue for CreateDatabaseDefaultPermissions_ {
        fn to_value(&self) -> serde_json::Value {
            serde_json::Value::Object(serde_json::Map::new())
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-datalakesettings-createtabledefaultpermissions.html
    pub struct CreateTableDefaultPermissions_ {}
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lakeformation_DataLakeSettings_CreateTableDefaultPermissions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LakeFormation::DataLakeSettings.CreateTableDefaultPermissions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lakeformation_DataLakeSettings_CreateTableDefaultPermissions as CreateTableDefaultPermissions;
    impl crate::value::ToValue for CreateTableDefaultPermissions_ {
        fn to_value(&self) -> serde_json::Value {
            serde_json::Value::Object(serde_json::Map::new())
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-datalakesettings-datalakeprincipal.html
    pub struct DataLakePrincipal_ {
        pub data_lake_principal_identifier: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lakeformation_DataLakeSettings_DataLakePrincipal {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LakeFormation::DataLakeSettings.DataLakePrincipal"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lakeformation_DataLakeSettings_DataLakePrincipal as DataLakePrincipal;
    impl crate::value::ToValue for DataLakePrincipal_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DataLakePrincipalIdentifier".to_string(),
                crate::value::ToValue::to_value(&self.data_lake_principal_identifier),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-datalakesettings-externaldatafilteringallowlist.html
    pub struct ExternalDataFilteringAllowList_ {}
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lakeformation_DataLakeSettings_ExternalDataFilteringAllowList {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LakeFormation::DataLakeSettings.ExternalDataFilteringAllowList"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lakeformation_DataLakeSettings_ExternalDataFilteringAllowList as ExternalDataFilteringAllowList;
    impl crate::value::ToValue for ExternalDataFilteringAllowList_ {
        fn to_value(&self) -> serde_json::Value {
            serde_json::Value::Object(serde_json::Map::new())
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-datalakesettings-principalpermissions.html
    pub struct PrincipalPermissions_ {
        pub permissions: Vec<crate::value::ExpString>,
        pub principal: Box<DataLakePrincipal_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lakeformation_DataLakeSettings_PrincipalPermissions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LakeFormation::DataLakeSettings.PrincipalPermissions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lakeformation_DataLakeSettings_PrincipalPermissions as PrincipalPermissions;
    impl crate::value::ToValue for PrincipalPermissions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Permissions".to_string(),
                crate::value::ToValue::to_value(&self.permissions),
            );
            properties.insert(
                "Principal".to_string(),
                crate::value::ToValue::to_value(&self.principal),
            );
            properties.into()
        }
    }
}
pub mod permissions {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-columnwildcard.html
    pub struct ColumnWildcard_ {
        pub excluded_column_names: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lakeformation_Permissions_ColumnWildcard {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LakeFormation::Permissions.ColumnWildcard"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lakeformation_Permissions_ColumnWildcard as ColumnWildcard;
    impl crate::value::ToValue for ColumnWildcard_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.excluded_column_names {
                properties.insert(
                    "ExcludedColumnNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-datalakeprincipal.html
    pub struct DataLakePrincipal_ {
        pub data_lake_principal_identifier: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lakeformation_Permissions_DataLakePrincipal {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LakeFormation::Permissions.DataLakePrincipal"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lakeformation_Permissions_DataLakePrincipal as DataLakePrincipal;
    impl crate::value::ToValue for DataLakePrincipal_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.data_lake_principal_identifier {
                properties.insert(
                    "DataLakePrincipalIdentifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-datalocationresource.html
    pub struct DataLocationResource_ {
        pub catalog_id: Option<crate::value::ExpString>,
        pub s3_resource: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lakeformation_Permissions_DataLocationResource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LakeFormation::Permissions.DataLocationResource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lakeformation_Permissions_DataLocationResource as DataLocationResource;
    impl crate::value::ToValue for DataLocationResource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.catalog_id {
                properties.insert(
                    "CatalogId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_resource {
                properties.insert(
                    "S3Resource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-databaseresource.html
    pub struct DatabaseResource_ {
        pub catalog_id: Option<crate::value::ExpString>,
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lakeformation_Permissions_DatabaseResource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LakeFormation::Permissions.DatabaseResource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lakeformation_Permissions_DatabaseResource as DatabaseResource;
    impl crate::value::ToValue for DatabaseResource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.catalog_id {
                properties.insert(
                    "CatalogId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-resource.html
    pub struct Resource_ {
        pub data_location_resource: Option<Box<DataLocationResource_>>,
        pub database_resource: Option<Box<DatabaseResource_>>,
        pub table_resource: Option<Box<TableResource_>>,
        pub table_with_columns_resource: Option<Box<TableWithColumnsResource_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lakeformation_Permissions_Resource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LakeFormation::Permissions.Resource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lakeformation_Permissions_Resource as Resource;
    impl crate::value::ToValue for Resource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.data_location_resource {
                properties.insert(
                    "DataLocationResource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.database_resource {
                properties.insert(
                    "DatabaseResource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.table_resource {
                properties.insert(
                    "TableResource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.table_with_columns_resource {
                properties.insert(
                    "TableWithColumnsResource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-tableresource.html
    pub struct TableResource_ {
        pub catalog_id: Option<crate::value::ExpString>,
        pub database_name: Option<crate::value::ExpString>,
        pub name: Option<crate::value::ExpString>,
        pub table_wildcard: Option<Box<TableWildcard_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lakeformation_Permissions_TableResource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LakeFormation::Permissions.TableResource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lakeformation_Permissions_TableResource as TableResource;
    impl crate::value::ToValue for TableResource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.catalog_id {
                properties.insert(
                    "CatalogId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.database_name {
                properties.insert(
                    "DatabaseName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.table_wildcard {
                properties.insert(
                    "TableWildcard".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-tablewildcard.html
    pub struct TableWildcard_ {}
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lakeformation_Permissions_TableWildcard {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LakeFormation::Permissions.TableWildcard"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lakeformation_Permissions_TableWildcard as TableWildcard;
    impl crate::value::ToValue for TableWildcard_ {
        fn to_value(&self) -> serde_json::Value {
            serde_json::Value::Object(serde_json::Map::new())
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-tablewithcolumnsresource.html
    pub struct TableWithColumnsResource_ {
        pub catalog_id: Option<crate::value::ExpString>,
        pub column_names: Option<Vec<crate::value::ExpString>>,
        pub column_wildcard: Option<Box<ColumnWildcard_>>,
        pub database_name: Option<crate::value::ExpString>,
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lakeformation_Permissions_TableWithColumnsResource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LakeFormation::Permissions.TableWithColumnsResource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lakeformation_Permissions_TableWithColumnsResource as TableWithColumnsResource;
    impl crate::value::ToValue for TableWithColumnsResource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.catalog_id {
                properties.insert(
                    "CatalogId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.column_names {
                properties.insert(
                    "ColumnNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.column_wildcard {
                properties.insert(
                    "ColumnWildcard".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.database_name {
                properties.insert(
                    "DatabaseName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod principalpermissions {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-columnwildcard.html
    pub struct ColumnWildcard_ {
        pub excluded_column_names: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lakeformation_PrincipalPermissions_ColumnWildcard {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LakeFormation::PrincipalPermissions.ColumnWildcard"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lakeformation_PrincipalPermissions_ColumnWildcard as ColumnWildcard;
    impl crate::value::ToValue for ColumnWildcard_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.excluded_column_names {
                properties.insert(
                    "ExcludedColumnNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-datacellsfilterresource.html
    pub struct DataCellsFilterResource_ {
        pub database_name: crate::value::ExpString,
        pub name: crate::value::ExpString,
        pub table_catalog_id: crate::value::ExpString,
        pub table_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lakeformation_PrincipalPermissions_DataCellsFilterResource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LakeFormation::PrincipalPermissions.DataCellsFilterResource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lakeformation_PrincipalPermissions_DataCellsFilterResource as DataCellsFilterResource;
    impl crate::value::ToValue for DataCellsFilterResource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DatabaseName".to_string(),
                crate::value::ToValue::to_value(&self.database_name),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "TableCatalogId".to_string(),
                crate::value::ToValue::to_value(&self.table_catalog_id),
            );
            properties.insert(
                "TableName".to_string(),
                crate::value::ToValue::to_value(&self.table_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-datalakeprincipal.html
    pub struct DataLakePrincipal_ {
        pub data_lake_principal_identifier: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lakeformation_PrincipalPermissions_DataLakePrincipal {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LakeFormation::PrincipalPermissions.DataLakePrincipal"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lakeformation_PrincipalPermissions_DataLakePrincipal as DataLakePrincipal;
    impl crate::value::ToValue for DataLakePrincipal_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.data_lake_principal_identifier {
                properties.insert(
                    "DataLakePrincipalIdentifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-datalocationresource.html
    pub struct DataLocationResource_ {
        pub catalog_id: crate::value::ExpString,
        pub resource_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lakeformation_PrincipalPermissions_DataLocationResource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LakeFormation::PrincipalPermissions.DataLocationResource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lakeformation_PrincipalPermissions_DataLocationResource as DataLocationResource;
    impl crate::value::ToValue for DataLocationResource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CatalogId".to_string(),
                crate::value::ToValue::to_value(&self.catalog_id),
            );
            properties.insert(
                "ResourceArn".to_string(),
                crate::value::ToValue::to_value(&self.resource_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-databaseresource.html
    pub struct DatabaseResource_ {
        pub catalog_id: crate::value::ExpString,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lakeformation_PrincipalPermissions_DatabaseResource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LakeFormation::PrincipalPermissions.DatabaseResource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lakeformation_PrincipalPermissions_DatabaseResource as DatabaseResource;
    impl crate::value::ToValue for DatabaseResource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CatalogId".to_string(),
                crate::value::ToValue::to_value(&self.catalog_id),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-lftag.html
    pub struct LFTag_ {
        pub tag_key: Option<crate::value::ExpString>,
        pub tag_values: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lakeformation_PrincipalPermissions_LFTag {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LakeFormation::PrincipalPermissions.LFTag"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lakeformation_PrincipalPermissions_LFTag as LFTag;
    impl crate::value::ToValue for LFTag_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.tag_key {
                properties.insert("TagKey".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.tag_values {
                properties.insert(
                    "TagValues".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-lftagkeyresource.html
    pub struct LFTagKeyResource_ {
        pub catalog_id: crate::value::ExpString,
        pub tag_key: crate::value::ExpString,
        pub tag_values: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lakeformation_PrincipalPermissions_LFTagKeyResource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LakeFormation::PrincipalPermissions.LFTagKeyResource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lakeformation_PrincipalPermissions_LFTagKeyResource as LFTagKeyResource;
    impl crate::value::ToValue for LFTagKeyResource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CatalogId".to_string(),
                crate::value::ToValue::to_value(&self.catalog_id),
            );
            properties.insert(
                "TagKey".to_string(),
                crate::value::ToValue::to_value(&self.tag_key),
            );
            properties.insert(
                "TagValues".to_string(),
                crate::value::ToValue::to_value(&self.tag_values),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-lftagpolicyresource.html
    pub struct LFTagPolicyResource_ {
        pub catalog_id: crate::value::ExpString,
        pub expression: Vec<LFTag_>,
        pub resource_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lakeformation_PrincipalPermissions_LFTagPolicyResource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LakeFormation::PrincipalPermissions.LFTagPolicyResource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lakeformation_PrincipalPermissions_LFTagPolicyResource as LFTagPolicyResource;
    impl crate::value::ToValue for LFTagPolicyResource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CatalogId".to_string(),
                crate::value::ToValue::to_value(&self.catalog_id),
            );
            properties.insert(
                "Expression".to_string(),
                crate::value::ToValue::to_value(&self.expression),
            );
            properties.insert(
                "ResourceType".to_string(),
                crate::value::ToValue::to_value(&self.resource_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-resource.html
    pub struct Resource_ {
        pub catalog: Option<serde_json::Value>,
        pub data_cells_filter: Option<Box<DataCellsFilterResource_>>,
        pub data_location: Option<Box<DataLocationResource_>>,
        pub database: Option<Box<DatabaseResource_>>,
        pub lf_tag: Option<Box<LFTagKeyResource_>>,
        pub lf_tag_policy: Option<Box<LFTagPolicyResource_>>,
        pub table: Option<Box<TableResource_>>,
        pub table_with_columns: Option<Box<TableWithColumnsResource_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lakeformation_PrincipalPermissions_Resource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LakeFormation::PrincipalPermissions.Resource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lakeformation_PrincipalPermissions_Resource as Resource;
    impl crate::value::ToValue for Resource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.catalog {
                properties.insert(
                    "Catalog".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.data_cells_filter {
                properties.insert(
                    "DataCellsFilter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.data_location {
                properties.insert(
                    "DataLocation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.database {
                properties.insert(
                    "Database".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lf_tag {
                properties.insert("LFTag".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.lf_tag_policy {
                properties.insert(
                    "LFTagPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.table {
                properties.insert("Table".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.table_with_columns {
                properties.insert(
                    "TableWithColumns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-tableresource.html
    pub struct TableResource_ {
        pub catalog_id: crate::value::ExpString,
        pub database_name: crate::value::ExpString,
        pub name: Option<crate::value::ExpString>,
        pub table_wildcard: Option<serde_json::Value>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lakeformation_PrincipalPermissions_TableResource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LakeFormation::PrincipalPermissions.TableResource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lakeformation_PrincipalPermissions_TableResource as TableResource;
    impl crate::value::ToValue for TableResource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CatalogId".to_string(),
                crate::value::ToValue::to_value(&self.catalog_id),
            );
            properties.insert(
                "DatabaseName".to_string(),
                crate::value::ToValue::to_value(&self.database_name),
            );
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.table_wildcard {
                properties.insert(
                    "TableWildcard".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-tablewithcolumnsresource.html
    pub struct TableWithColumnsResource_ {
        pub catalog_id: crate::value::ExpString,
        pub column_names: Option<Vec<crate::value::ExpString>>,
        pub column_wildcard: Option<Box<ColumnWildcard_>>,
        pub database_name: crate::value::ExpString,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lakeformation_PrincipalPermissions_TableWithColumnsResource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LakeFormation::PrincipalPermissions.TableWithColumnsResource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lakeformation_PrincipalPermissions_TableWithColumnsResource as TableWithColumnsResource;
    impl crate::value::ToValue for TableWithColumnsResource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CatalogId".to_string(),
                crate::value::ToValue::to_value(&self.catalog_id),
            );
            if let Some(ref value) = self.column_names {
                properties.insert(
                    "ColumnNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.column_wildcard {
                properties.insert(
                    "ColumnWildcard".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DatabaseName".to_string(),
                crate::value::ToValue::to_value(&self.database_name),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
}
pub mod tagassociation {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-tagassociation-databaseresource.html
    pub struct DatabaseResource_ {
        pub catalog_id: crate::value::ExpString,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lakeformation_TagAssociation_DatabaseResource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LakeFormation::TagAssociation.DatabaseResource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lakeformation_TagAssociation_DatabaseResource as DatabaseResource;
    impl crate::value::ToValue for DatabaseResource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CatalogId".to_string(),
                crate::value::ToValue::to_value(&self.catalog_id),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-tagassociation-lftagpair.html
    pub struct LFTagPair_ {
        pub catalog_id: crate::value::ExpString,
        pub tag_key: crate::value::ExpString,
        pub tag_values: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lakeformation_TagAssociation_LFTagPair {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LakeFormation::TagAssociation.LFTagPair"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lakeformation_TagAssociation_LFTagPair as LFTagPair;
    impl crate::value::ToValue for LFTagPair_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CatalogId".to_string(),
                crate::value::ToValue::to_value(&self.catalog_id),
            );
            properties.insert(
                "TagKey".to_string(),
                crate::value::ToValue::to_value(&self.tag_key),
            );
            properties.insert(
                "TagValues".to_string(),
                crate::value::ToValue::to_value(&self.tag_values),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-tagassociation-resource.html
    pub struct Resource_ {
        pub catalog: Option<serde_json::Value>,
        pub database: Option<Box<DatabaseResource_>>,
        pub table: Option<Box<TableResource_>>,
        pub table_with_columns: Option<Box<TableWithColumnsResource_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lakeformation_TagAssociation_Resource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LakeFormation::TagAssociation.Resource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lakeformation_TagAssociation_Resource as Resource;
    impl crate::value::ToValue for Resource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.catalog {
                properties.insert(
                    "Catalog".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.database {
                properties.insert(
                    "Database".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.table {
                properties.insert("Table".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.table_with_columns {
                properties.insert(
                    "TableWithColumns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-tagassociation-tableresource.html
    pub struct TableResource_ {
        pub catalog_id: crate::value::ExpString,
        pub database_name: crate::value::ExpString,
        pub name: Option<crate::value::ExpString>,
        pub table_wildcard: Option<serde_json::Value>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lakeformation_TagAssociation_TableResource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LakeFormation::TagAssociation.TableResource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lakeformation_TagAssociation_TableResource as TableResource;
    impl crate::value::ToValue for TableResource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CatalogId".to_string(),
                crate::value::ToValue::to_value(&self.catalog_id),
            );
            properties.insert(
                "DatabaseName".to_string(),
                crate::value::ToValue::to_value(&self.database_name),
            );
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.table_wildcard {
                properties.insert(
                    "TableWildcard".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-tagassociation-tablewithcolumnsresource.html
    pub struct TableWithColumnsResource_ {
        pub catalog_id: crate::value::ExpString,
        pub column_names: Vec<crate::value::ExpString>,
        pub database_name: crate::value::ExpString,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lakeformation_TagAssociation_TableWithColumnsResource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LakeFormation::TagAssociation.TableWithColumnsResource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lakeformation_TagAssociation_TableWithColumnsResource as TableWithColumnsResource;
    impl crate::value::ToValue for TableWithColumnsResource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CatalogId".to_string(),
                crate::value::ToValue::to_value(&self.catalog_id),
            );
            properties.insert(
                "ColumnNames".to_string(),
                crate::value::ToValue::to_value(&self.column_names),
            );
            properties.insert(
                "DatabaseName".to_string(),
                crate::value::ToValue::to_value(&self.database_name),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-datacellsfilter.html
pub struct DataCellsFilter_ {
    pub column_names: Option<Vec<crate::value::ExpString>>,
    pub column_wildcard: Option<super::lakeformation::datacellsfilter::ColumnWildcard_>,
    pub database_name: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub row_filter: Option<super::lakeformation::datacellsfilter::RowFilter_>,
    pub table_catalog_id: crate::value::ExpString,
    pub table_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lakeformation_DataCellsFilter {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::LakeFormation::DataCellsFilter"
        $($field $value)*)
    };
}
pub use crate::__aws_lakeformation_DataCellsFilter as DataCellsFilter;
impl crate::template::ToResource for DataCellsFilter_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("LakeFormation"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DataCellsFilter"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.column_names {
            properties.insert(
                "ColumnNames".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.column_wildcard {
            properties.insert(
                "ColumnWildcard".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DatabaseName".to_string(),
            crate::value::ToValue::to_value(&self.database_name),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.row_filter {
            properties.insert(
                "RowFilter".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "TableCatalogId".to_string(),
            crate::value::ToValue::to_value(&self.table_catalog_id),
        );
        properties.insert(
            "TableName".to_string(),
            crate::value::ToValue::to_value(&self.table_name),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-datalakesettings.html
pub struct DataLakeSettings_ {
    pub admins: Option<super::lakeformation::datalakesettings::Admins_>,
    pub allow_external_data_filtering: Option<crate::value::ExpBool>,
    pub allow_full_table_external_data_access: Option<crate::value::ExpBool>,
    pub authorized_session_tag_value_list: Option<Vec<crate::value::ExpString>>,
    pub create_database_default_permissions:
        Option<super::lakeformation::datalakesettings::CreateDatabaseDefaultPermissions_>,
    pub create_table_default_permissions:
        Option<super::lakeformation::datalakesettings::CreateTableDefaultPermissions_>,
    pub external_data_filtering_allow_list:
        Option<super::lakeformation::datalakesettings::ExternalDataFilteringAllowList_>,
    pub mutation_type: Option<crate::value::ExpString>,
    pub parameters: Option<serde_json::Value>,
    pub trusted_resource_owners: Option<Vec<crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lakeformation_DataLakeSettings {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::LakeFormation::DataLakeSettings"
        $($field $value)*)
    };
}
pub use crate::__aws_lakeformation_DataLakeSettings as DataLakeSettings;
impl crate::template::ToResource for DataLakeSettings_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("LakeFormation"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DataLakeSettings"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.admins {
            properties.insert("Admins".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.allow_external_data_filtering {
            properties.insert(
                "AllowExternalDataFiltering".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.allow_full_table_external_data_access {
            properties.insert(
                "AllowFullTableExternalDataAccess".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.authorized_session_tag_value_list {
            properties.insert(
                "AuthorizedSessionTagValueList".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.create_database_default_permissions {
            properties.insert(
                "CreateDatabaseDefaultPermissions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.create_table_default_permissions {
            properties.insert(
                "CreateTableDefaultPermissions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.external_data_filtering_allow_list {
            properties.insert(
                "ExternalDataFilteringAllowList".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.mutation_type {
            properties.insert(
                "MutationType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.parameters {
            properties.insert(
                "Parameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.trusted_resource_owners {
            properties.insert(
                "TrustedResourceOwners".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-permissions.html
pub struct Permissions_ {
    pub data_lake_principal: super::lakeformation::permissions::DataLakePrincipal_,
    pub permissions: Option<Vec<crate::value::ExpString>>,
    pub permissions_with_grant_option: Option<Vec<crate::value::ExpString>>,
    pub resource: super::lakeformation::permissions::Resource_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lakeformation_Permissions {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::LakeFormation::Permissions"
        $($field $value)*)
    };
}
pub use crate::__aws_lakeformation_Permissions as Permissions;
impl crate::template::ToResource for Permissions_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("LakeFormation"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Permissions"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DataLakePrincipal".to_string(),
            crate::value::ToValue::to_value(&self.data_lake_principal),
        );
        if let Some(ref value) = self.permissions {
            properties.insert(
                "Permissions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.permissions_with_grant_option {
            properties.insert(
                "PermissionsWithGrantOption".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Resource".to_string(),
            crate::value::ToValue::to_value(&self.resource),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-principalpermissions.html
pub struct PrincipalPermissions_ {
    pub catalog: Option<crate::value::ExpString>,
    pub permissions: Vec<crate::value::ExpString>,
    pub permissions_with_grant_option: Vec<crate::value::ExpString>,
    pub principal: super::lakeformation::principalpermissions::DataLakePrincipal_,
    pub resource: super::lakeformation::principalpermissions::Resource_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lakeformation_PrincipalPermissions {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::LakeFormation::PrincipalPermissions"
        $($field $value)*)
    };
}
pub use crate::__aws_lakeformation_PrincipalPermissions as PrincipalPermissions;
impl crate::template::ToResource for PrincipalPermissions_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("LakeFormation"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PrincipalPermissions"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.catalog {
            properties.insert(
                "Catalog".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Permissions".to_string(),
            crate::value::ToValue::to_value(&self.permissions),
        );
        properties.insert(
            "PermissionsWithGrantOption".to_string(),
            crate::value::ToValue::to_value(&self.permissions_with_grant_option),
        );
        properties.insert(
            "Principal".to_string(),
            crate::value::ToValue::to_value(&self.principal),
        );
        properties.insert(
            "Resource".to_string(),
            crate::value::ToValue::to_value(&self.resource),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-resource.html
pub struct Resource_ {
    pub hybrid_access_enabled: Option<crate::value::ExpBool>,
    pub resource_arn: crate::value::ExpString,
    pub role_arn: Option<crate::value::ExpString>,
    pub use_service_linked_role: crate::value::ExpBool,
    pub with_federation: Option<crate::value::ExpBool>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lakeformation_Resource {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::LakeFormation::Resource"
        $($field $value)*)
    };
}
pub use crate::__aws_lakeformation_Resource as Resource;
impl crate::template::ToResource for Resource_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("LakeFormation"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Resource"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.hybrid_access_enabled {
            properties.insert(
                "HybridAccessEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ResourceArn".to_string(),
            crate::value::ToValue::to_value(&self.resource_arn),
        );
        if let Some(ref value) = self.role_arn {
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "UseServiceLinkedRole".to_string(),
            crate::value::ToValue::to_value(&self.use_service_linked_role),
        );
        if let Some(ref value) = self.with_federation {
            properties.insert(
                "WithFederation".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-tag.html
pub struct Tag_ {
    pub catalog_id: Option<crate::value::ExpString>,
    pub tag_key: crate::value::ExpString,
    pub tag_values: Vec<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lakeformation_Tag {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::LakeFormation::Tag"
        $($field $value)*)
    };
}
pub use crate::__aws_lakeformation_Tag as Tag;
impl crate::template::ToResource for Tag_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("LakeFormation"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Tag"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.catalog_id {
            properties.insert(
                "CatalogId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "TagKey".to_string(),
            crate::value::ToValue::to_value(&self.tag_key),
        );
        properties.insert(
            "TagValues".to_string(),
            crate::value::ToValue::to_value(&self.tag_values),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-tagassociation.html
pub struct TagAssociation_ {
    pub lf_tags: Vec<super::lakeformation::tagassociation::LFTagPair_>,
    pub resource: super::lakeformation::tagassociation::Resource_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lakeformation_TagAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::LakeFormation::TagAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_lakeformation_TagAssociation as TagAssociation;
impl crate::template::ToResource for TagAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("LakeFormation"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TagAssociation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "LFTags".to_string(),
            crate::value::ToValue::to_value(&self.lf_tags),
        );
        properties.insert(
            "Resource".to_string(),
            crate::value::ToValue::to_value(&self.resource),
        );
        properties
    }
}
