pub mod datasource {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-accesscontrollistconfiguration.html
    pub struct AccessControlListConfiguration_ {
        pub key_path: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_AccessControlListConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.AccessControlListConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_AccessControlListConfiguration as AccessControlListConfiguration;
    impl crate::value::ToValue for AccessControlListConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.key_path {
                properties.insert(
                    "KeyPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-aclconfiguration.html
    pub struct AclConfiguration_ {
        pub allowed_groups_column_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_AclConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.AclConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_AclConfiguration as AclConfiguration;
    impl crate::value::ToValue for AclConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AllowedGroupsColumnName".to_string(),
                crate::value::ToValue::to_value(&self.allowed_groups_column_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-columnconfiguration.html
    pub struct ColumnConfiguration_ {
        pub change_detecting_columns: Vec<crate::value::ExpString>,
        pub document_data_column_name: crate::value::ExpString,
        pub document_id_column_name: crate::value::ExpString,
        pub document_title_column_name: Option<crate::value::ExpString>,
        pub field_mappings: Option<Vec<DataSourceToIndexFieldMapping_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_ColumnConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.ColumnConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_ColumnConfiguration as ColumnConfiguration;
    impl crate::value::ToValue for ColumnConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ChangeDetectingColumns".to_string(),
                crate::value::ToValue::to_value(&self.change_detecting_columns),
            );
            properties.insert(
                "DocumentDataColumnName".to_string(),
                crate::value::ToValue::to_value(&self.document_data_column_name),
            );
            properties.insert(
                "DocumentIdColumnName".to_string(),
                crate::value::ToValue::to_value(&self.document_id_column_name),
            );
            if let Some(ref value) = self.document_title_column_name {
                properties.insert(
                    "DocumentTitleColumnName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.field_mappings {
                properties.insert(
                    "FieldMappings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluenceattachmentconfiguration.html
    pub struct ConfluenceAttachmentConfiguration_ {
        pub attachment_field_mappings: Option<Vec<ConfluenceAttachmentToIndexFieldMapping_>>,
        pub crawl_attachments: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_ConfluenceAttachmentConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.ConfluenceAttachmentConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_ConfluenceAttachmentConfiguration as ConfluenceAttachmentConfiguration;
    impl crate::value::ToValue for ConfluenceAttachmentConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attachment_field_mappings {
                properties.insert(
                    "AttachmentFieldMappings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.crawl_attachments {
                properties.insert(
                    "CrawlAttachments".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluenceattachmenttoindexfieldmapping.html
    pub struct ConfluenceAttachmentToIndexFieldMapping_ {
        pub data_source_field_name: crate::value::ExpString,
        pub date_field_format: Option<crate::value::ExpString>,
        pub index_field_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_ConfluenceAttachmentToIndexFieldMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.ConfluenceAttachmentToIndexFieldMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_ConfluenceAttachmentToIndexFieldMapping as ConfluenceAttachmentToIndexFieldMapping;
    impl crate::value::ToValue for ConfluenceAttachmentToIndexFieldMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DataSourceFieldName".to_string(),
                crate::value::ToValue::to_value(&self.data_source_field_name),
            );
            if let Some(ref value) = self.date_field_format {
                properties.insert(
                    "DateFieldFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "IndexFieldName".to_string(),
                crate::value::ToValue::to_value(&self.index_field_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluenceblogconfiguration.html
    pub struct ConfluenceBlogConfiguration_ {
        pub blog_field_mappings: Option<Vec<ConfluenceBlogToIndexFieldMapping_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_ConfluenceBlogConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.ConfluenceBlogConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_ConfluenceBlogConfiguration as ConfluenceBlogConfiguration;
    impl crate::value::ToValue for ConfluenceBlogConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.blog_field_mappings {
                properties.insert(
                    "BlogFieldMappings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluenceblogtoindexfieldmapping.html
    pub struct ConfluenceBlogToIndexFieldMapping_ {
        pub data_source_field_name: crate::value::ExpString,
        pub date_field_format: Option<crate::value::ExpString>,
        pub index_field_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_ConfluenceBlogToIndexFieldMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.ConfluenceBlogToIndexFieldMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_ConfluenceBlogToIndexFieldMapping as ConfluenceBlogToIndexFieldMapping;
    impl crate::value::ToValue for ConfluenceBlogToIndexFieldMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DataSourceFieldName".to_string(),
                crate::value::ToValue::to_value(&self.data_source_field_name),
            );
            if let Some(ref value) = self.date_field_format {
                properties.insert(
                    "DateFieldFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "IndexFieldName".to_string(),
                crate::value::ToValue::to_value(&self.index_field_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluenceconfiguration.html
    pub struct ConfluenceConfiguration_ {
        pub attachment_configuration: Option<Box<ConfluenceAttachmentConfiguration_>>,
        pub blog_configuration: Option<Box<ConfluenceBlogConfiguration_>>,
        pub exclusion_patterns: Option<Vec<crate::value::ExpString>>,
        pub inclusion_patterns: Option<Vec<crate::value::ExpString>>,
        pub page_configuration: Option<Box<ConfluencePageConfiguration_>>,
        pub secret_arn: crate::value::ExpString,
        pub server_url: crate::value::ExpString,
        pub space_configuration: Option<Box<ConfluenceSpaceConfiguration_>>,
        pub version: crate::value::ExpString,
        pub vpc_configuration: Option<Box<DataSourceVpcConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_ConfluenceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.ConfluenceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_ConfluenceConfiguration as ConfluenceConfiguration;
    impl crate::value::ToValue for ConfluenceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attachment_configuration {
                properties.insert(
                    "AttachmentConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.blog_configuration {
                properties.insert(
                    "BlogConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exclusion_patterns {
                properties.insert(
                    "ExclusionPatterns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inclusion_patterns {
                properties.insert(
                    "InclusionPatterns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.page_configuration {
                properties.insert(
                    "PageConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SecretArn".to_string(),
                crate::value::ToValue::to_value(&self.secret_arn),
            );
            properties.insert(
                "ServerUrl".to_string(),
                crate::value::ToValue::to_value(&self.server_url),
            );
            if let Some(ref value) = self.space_configuration {
                properties.insert(
                    "SpaceConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Version".to_string(),
                crate::value::ToValue::to_value(&self.version),
            );
            if let Some(ref value) = self.vpc_configuration {
                properties.insert(
                    "VpcConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluencepageconfiguration.html
    pub struct ConfluencePageConfiguration_ {
        pub page_field_mappings: Option<Vec<ConfluencePageToIndexFieldMapping_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_ConfluencePageConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.ConfluencePageConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_ConfluencePageConfiguration as ConfluencePageConfiguration;
    impl crate::value::ToValue for ConfluencePageConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.page_field_mappings {
                properties.insert(
                    "PageFieldMappings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluencepagetoindexfieldmapping.html
    pub struct ConfluencePageToIndexFieldMapping_ {
        pub data_source_field_name: crate::value::ExpString,
        pub date_field_format: Option<crate::value::ExpString>,
        pub index_field_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_ConfluencePageToIndexFieldMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.ConfluencePageToIndexFieldMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_ConfluencePageToIndexFieldMapping as ConfluencePageToIndexFieldMapping;
    impl crate::value::ToValue for ConfluencePageToIndexFieldMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DataSourceFieldName".to_string(),
                crate::value::ToValue::to_value(&self.data_source_field_name),
            );
            if let Some(ref value) = self.date_field_format {
                properties.insert(
                    "DateFieldFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "IndexFieldName".to_string(),
                crate::value::ToValue::to_value(&self.index_field_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluencespaceconfiguration.html
    pub struct ConfluenceSpaceConfiguration_ {
        pub crawl_archived_spaces: Option<crate::value::ExpBool>,
        pub crawl_personal_spaces: Option<crate::value::ExpBool>,
        pub exclude_spaces: Option<Vec<crate::value::ExpString>>,
        pub include_spaces: Option<Vec<crate::value::ExpString>>,
        pub space_field_mappings: Option<Vec<ConfluenceSpaceToIndexFieldMapping_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_ConfluenceSpaceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.ConfluenceSpaceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_ConfluenceSpaceConfiguration as ConfluenceSpaceConfiguration;
    impl crate::value::ToValue for ConfluenceSpaceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.crawl_archived_spaces {
                properties.insert(
                    "CrawlArchivedSpaces".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.crawl_personal_spaces {
                properties.insert(
                    "CrawlPersonalSpaces".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exclude_spaces {
                properties.insert(
                    "ExcludeSpaces".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_spaces {
                properties.insert(
                    "IncludeSpaces".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.space_field_mappings {
                properties.insert(
                    "SpaceFieldMappings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluencespacetoindexfieldmapping.html
    pub struct ConfluenceSpaceToIndexFieldMapping_ {
        pub data_source_field_name: crate::value::ExpString,
        pub date_field_format: Option<crate::value::ExpString>,
        pub index_field_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_ConfluenceSpaceToIndexFieldMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.ConfluenceSpaceToIndexFieldMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_ConfluenceSpaceToIndexFieldMapping as ConfluenceSpaceToIndexFieldMapping;
    impl crate::value::ToValue for ConfluenceSpaceToIndexFieldMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DataSourceFieldName".to_string(),
                crate::value::ToValue::to_value(&self.data_source_field_name),
            );
            if let Some(ref value) = self.date_field_format {
                properties.insert(
                    "DateFieldFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "IndexFieldName".to_string(),
                crate::value::ToValue::to_value(&self.index_field_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-connectionconfiguration.html
    pub struct ConnectionConfiguration_ {
        pub database_host: crate::value::ExpString,
        pub database_name: crate::value::ExpString,
        pub database_port: i64,
        pub secret_arn: crate::value::ExpString,
        pub table_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_ConnectionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.ConnectionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_ConnectionConfiguration as ConnectionConfiguration;
    impl crate::value::ToValue for ConnectionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DatabaseHost".to_string(),
                crate::value::ToValue::to_value(&self.database_host),
            );
            properties.insert(
                "DatabaseName".to_string(),
                crate::value::ToValue::to_value(&self.database_name),
            );
            properties.insert(
                "DatabasePort".to_string(),
                crate::value::ToValue::to_value(&self.database_port),
            );
            properties.insert(
                "SecretArn".to_string(),
                crate::value::ToValue::to_value(&self.secret_arn),
            );
            properties.insert(
                "TableName".to_string(),
                crate::value::ToValue::to_value(&self.table_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-customdocumentenrichmentconfiguration.html
    pub struct CustomDocumentEnrichmentConfiguration_ {
        pub inline_configurations: Option<Vec<InlineCustomDocumentEnrichmentConfiguration_>>,
        pub post_extraction_hook_configuration: Option<Box<HookConfiguration_>>,
        pub pre_extraction_hook_configuration: Option<Box<HookConfiguration_>>,
        pub role_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_CustomDocumentEnrichmentConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.CustomDocumentEnrichmentConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_CustomDocumentEnrichmentConfiguration as CustomDocumentEnrichmentConfiguration;
    impl crate::value::ToValue for CustomDocumentEnrichmentConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.inline_configurations {
                properties.insert(
                    "InlineConfigurations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.post_extraction_hook_configuration {
                properties.insert(
                    "PostExtractionHookConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pre_extraction_hook_configuration {
                properties.insert(
                    "PreExtractionHookConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.role_arn {
                properties.insert(
                    "RoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-datasourceconfiguration.html
    pub struct DataSourceConfiguration_ {
        pub confluence_configuration: Option<Box<ConfluenceConfiguration_>>,
        pub database_configuration: Option<Box<DatabaseConfiguration_>>,
        pub google_drive_configuration: Option<Box<GoogleDriveConfiguration_>>,
        pub one_drive_configuration: Option<Box<OneDriveConfiguration_>>,
        pub s3_configuration: Option<Box<S3DataSourceConfiguration_>>,
        pub salesforce_configuration: Option<Box<SalesforceConfiguration_>>,
        pub service_now_configuration: Option<Box<ServiceNowConfiguration_>>,
        pub share_point_configuration: Option<Box<SharePointConfiguration_>>,
        pub template_configuration: Option<Box<TemplateConfiguration_>>,
        pub web_crawler_configuration: Option<Box<WebCrawlerConfiguration_>>,
        pub work_docs_configuration: Option<Box<WorkDocsConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_DataSourceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.DataSourceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_DataSourceConfiguration as DataSourceConfiguration;
    impl crate::value::ToValue for DataSourceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.confluence_configuration {
                properties.insert(
                    "ConfluenceConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.database_configuration {
                properties.insert(
                    "DatabaseConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.google_drive_configuration {
                properties.insert(
                    "GoogleDriveConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.one_drive_configuration {
                properties.insert(
                    "OneDriveConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_configuration {
                properties.insert(
                    "S3Configuration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.salesforce_configuration {
                properties.insert(
                    "SalesforceConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.service_now_configuration {
                properties.insert(
                    "ServiceNowConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.share_point_configuration {
                properties.insert(
                    "SharePointConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.template_configuration {
                properties.insert(
                    "TemplateConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.web_crawler_configuration {
                properties.insert(
                    "WebCrawlerConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.work_docs_configuration {
                properties.insert(
                    "WorkDocsConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-datasourcetoindexfieldmapping.html
    pub struct DataSourceToIndexFieldMapping_ {
        pub data_source_field_name: crate::value::ExpString,
        pub date_field_format: Option<crate::value::ExpString>,
        pub index_field_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_DataSourceToIndexFieldMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.DataSourceToIndexFieldMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_DataSourceToIndexFieldMapping as DataSourceToIndexFieldMapping;
    impl crate::value::ToValue for DataSourceToIndexFieldMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DataSourceFieldName".to_string(),
                crate::value::ToValue::to_value(&self.data_source_field_name),
            );
            if let Some(ref value) = self.date_field_format {
                properties.insert(
                    "DateFieldFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "IndexFieldName".to_string(),
                crate::value::ToValue::to_value(&self.index_field_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-datasourcevpcconfiguration.html
    pub struct DataSourceVpcConfiguration_ {
        pub security_group_ids: Vec<crate::value::ExpString>,
        pub subnet_ids: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_DataSourceVpcConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.DataSourceVpcConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_DataSourceVpcConfiguration as DataSourceVpcConfiguration;
    impl crate::value::ToValue for DataSourceVpcConfiguration_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-databaseconfiguration.html
    pub struct DatabaseConfiguration_ {
        pub acl_configuration: Option<Box<AclConfiguration_>>,
        pub column_configuration: Box<ColumnConfiguration_>,
        pub connection_configuration: Box<ConnectionConfiguration_>,
        pub database_engine_type: crate::value::ExpString,
        pub sql_configuration: Option<Box<SqlConfiguration_>>,
        pub vpc_configuration: Option<Box<DataSourceVpcConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_DatabaseConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.DatabaseConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_DatabaseConfiguration as DatabaseConfiguration;
    impl crate::value::ToValue for DatabaseConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.acl_configuration {
                properties.insert(
                    "AclConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ColumnConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.column_configuration),
            );
            properties.insert(
                "ConnectionConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.connection_configuration),
            );
            properties.insert(
                "DatabaseEngineType".to_string(),
                crate::value::ToValue::to_value(&self.database_engine_type),
            );
            if let Some(ref value) = self.sql_configuration {
                properties.insert(
                    "SqlConfiguration".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-documentattributecondition.html
    pub struct DocumentAttributeCondition_ {
        pub condition_document_attribute_key: crate::value::ExpString,
        pub condition_on_value: Option<Box<DocumentAttributeValue_>>,
        pub operator: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_DocumentAttributeCondition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.DocumentAttributeCondition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_DocumentAttributeCondition as DocumentAttributeCondition;
    impl crate::value::ToValue for DocumentAttributeCondition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ConditionDocumentAttributeKey".to_string(),
                crate::value::ToValue::to_value(&self.condition_document_attribute_key),
            );
            if let Some(ref value) = self.condition_on_value {
                properties.insert(
                    "ConditionOnValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Operator".to_string(),
                crate::value::ToValue::to_value(&self.operator),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-documentattributetarget.html
    pub struct DocumentAttributeTarget_ {
        pub target_document_attribute_key: crate::value::ExpString,
        pub target_document_attribute_value: Option<Box<DocumentAttributeValue_>>,
        pub target_document_attribute_value_deletion: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_DocumentAttributeTarget {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.DocumentAttributeTarget"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_DocumentAttributeTarget as DocumentAttributeTarget;
    impl crate::value::ToValue for DocumentAttributeTarget_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TargetDocumentAttributeKey".to_string(),
                crate::value::ToValue::to_value(&self.target_document_attribute_key),
            );
            if let Some(ref value) = self.target_document_attribute_value {
                properties.insert(
                    "TargetDocumentAttributeValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_document_attribute_value_deletion {
                properties.insert(
                    "TargetDocumentAttributeValueDeletion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-documentattributevalue.html
    pub struct DocumentAttributeValue_ {
        pub date_value: Option<crate::value::ExpString>,
        pub long_value: Option<i64>,
        pub string_list_value: Option<Vec<crate::value::ExpString>>,
        pub string_value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_DocumentAttributeValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.DocumentAttributeValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_DocumentAttributeValue as DocumentAttributeValue;
    impl crate::value::ToValue for DocumentAttributeValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.date_value {
                properties.insert(
                    "DateValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.long_value {
                properties.insert(
                    "LongValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.string_list_value {
                properties.insert(
                    "StringListValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.string_value {
                properties.insert(
                    "StringValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-documentsmetadataconfiguration.html
    pub struct DocumentsMetadataConfiguration_ {
        pub s3_prefix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_DocumentsMetadataConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.DocumentsMetadataConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_DocumentsMetadataConfiguration as DocumentsMetadataConfiguration;
    impl crate::value::ToValue for DocumentsMetadataConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.s3_prefix {
                properties.insert(
                    "S3Prefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-googledriveconfiguration.html
    pub struct GoogleDriveConfiguration_ {
        pub exclude_mime_types: Option<Vec<crate::value::ExpString>>,
        pub exclude_shared_drives: Option<Vec<crate::value::ExpString>>,
        pub exclude_user_accounts: Option<Vec<crate::value::ExpString>>,
        pub exclusion_patterns: Option<Vec<crate::value::ExpString>>,
        pub field_mappings: Option<Vec<DataSourceToIndexFieldMapping_>>,
        pub inclusion_patterns: Option<Vec<crate::value::ExpString>>,
        pub secret_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_GoogleDriveConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.GoogleDriveConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_GoogleDriveConfiguration as GoogleDriveConfiguration;
    impl crate::value::ToValue for GoogleDriveConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.exclude_mime_types {
                properties.insert(
                    "ExcludeMimeTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exclude_shared_drives {
                properties.insert(
                    "ExcludeSharedDrives".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exclude_user_accounts {
                properties.insert(
                    "ExcludeUserAccounts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exclusion_patterns {
                properties.insert(
                    "ExclusionPatterns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.field_mappings {
                properties.insert(
                    "FieldMappings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inclusion_patterns {
                properties.insert(
                    "InclusionPatterns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SecretArn".to_string(),
                crate::value::ToValue::to_value(&self.secret_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-hookconfiguration.html
    pub struct HookConfiguration_ {
        pub invocation_condition: Option<Box<DocumentAttributeCondition_>>,
        pub lambda_arn: crate::value::ExpString,
        pub s3_bucket: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_HookConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.HookConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_HookConfiguration as HookConfiguration;
    impl crate::value::ToValue for HookConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.invocation_condition {
                properties.insert(
                    "InvocationCondition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "LambdaArn".to_string(),
                crate::value::ToValue::to_value(&self.lambda_arn),
            );
            properties.insert(
                "S3Bucket".to_string(),
                crate::value::ToValue::to_value(&self.s3_bucket),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-inlinecustomdocumentenrichmentconfiguration.html
    pub struct InlineCustomDocumentEnrichmentConfiguration_ {
        pub condition: Option<Box<DocumentAttributeCondition_>>,
        pub document_content_deletion: Option<crate::value::ExpBool>,
        pub target: Option<Box<DocumentAttributeTarget_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_InlineCustomDocumentEnrichmentConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.InlineCustomDocumentEnrichmentConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_InlineCustomDocumentEnrichmentConfiguration as InlineCustomDocumentEnrichmentConfiguration;
    impl crate::value::ToValue for InlineCustomDocumentEnrichmentConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.condition {
                properties.insert(
                    "Condition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.document_content_deletion {
                properties.insert(
                    "DocumentContentDeletion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target {
                properties.insert("Target".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-onedriveconfiguration.html
    pub struct OneDriveConfiguration_ {
        pub disable_local_groups: Option<crate::value::ExpBool>,
        pub exclusion_patterns: Option<Vec<crate::value::ExpString>>,
        pub field_mappings: Option<Vec<DataSourceToIndexFieldMapping_>>,
        pub inclusion_patterns: Option<Vec<crate::value::ExpString>>,
        pub one_drive_users: Box<OneDriveUsers_>,
        pub secret_arn: crate::value::ExpString,
        pub tenant_domain: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_OneDriveConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.OneDriveConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_OneDriveConfiguration as OneDriveConfiguration;
    impl crate::value::ToValue for OneDriveConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.disable_local_groups {
                properties.insert(
                    "DisableLocalGroups".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exclusion_patterns {
                properties.insert(
                    "ExclusionPatterns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.field_mappings {
                properties.insert(
                    "FieldMappings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inclusion_patterns {
                properties.insert(
                    "InclusionPatterns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "OneDriveUsers".to_string(),
                crate::value::ToValue::to_value(&self.one_drive_users),
            );
            properties.insert(
                "SecretArn".to_string(),
                crate::value::ToValue::to_value(&self.secret_arn),
            );
            properties.insert(
                "TenantDomain".to_string(),
                crate::value::ToValue::to_value(&self.tenant_domain),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-onedriveusers.html
    pub struct OneDriveUsers_ {
        pub one_drive_user_list: Option<Vec<crate::value::ExpString>>,
        pub one_drive_user_s3_path: Option<Box<S3Path_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_OneDriveUsers {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.OneDriveUsers"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_OneDriveUsers as OneDriveUsers;
    impl crate::value::ToValue for OneDriveUsers_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.one_drive_user_list {
                properties.insert(
                    "OneDriveUserList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.one_drive_user_s3_path {
                properties.insert(
                    "OneDriveUserS3Path".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-proxyconfiguration.html
    pub struct ProxyConfiguration_ {
        pub credentials: Option<crate::value::ExpString>,
        pub host: crate::value::ExpString,
        pub port: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_ProxyConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.ProxyConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_ProxyConfiguration as ProxyConfiguration;
    impl crate::value::ToValue for ProxyConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.credentials {
                properties.insert(
                    "Credentials".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Host".to_string(),
                crate::value::ToValue::to_value(&self.host),
            );
            properties.insert(
                "Port".to_string(),
                crate::value::ToValue::to_value(&self.port),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-s3datasourceconfiguration.html
    pub struct S3DataSourceConfiguration_ {
        pub access_control_list_configuration: Option<Box<AccessControlListConfiguration_>>,
        pub bucket_name: crate::value::ExpString,
        pub documents_metadata_configuration: Option<Box<DocumentsMetadataConfiguration_>>,
        pub exclusion_patterns: Option<Vec<crate::value::ExpString>>,
        pub inclusion_patterns: Option<Vec<crate::value::ExpString>>,
        pub inclusion_prefixes: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_S3DataSourceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.S3DataSourceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_S3DataSourceConfiguration as S3DataSourceConfiguration;
    impl crate::value::ToValue for S3DataSourceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.access_control_list_configuration {
                properties.insert(
                    "AccessControlListConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "BucketName".to_string(),
                crate::value::ToValue::to_value(&self.bucket_name),
            );
            if let Some(ref value) = self.documents_metadata_configuration {
                properties.insert(
                    "DocumentsMetadataConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exclusion_patterns {
                properties.insert(
                    "ExclusionPatterns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inclusion_patterns {
                properties.insert(
                    "InclusionPatterns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inclusion_prefixes {
                properties.insert(
                    "InclusionPrefixes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-s3path.html
    pub struct S3Path_ {
        pub bucket: crate::value::ExpString,
        pub key: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_S3Path {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.S3Path"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_S3Path as S3Path;
    impl crate::value::ToValue for S3Path_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Bucket".to_string(),
                crate::value::ToValue::to_value(&self.bucket),
            );
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforcechatterfeedconfiguration.html
    pub struct SalesforceChatterFeedConfiguration_ {
        pub document_data_field_name: crate::value::ExpString,
        pub document_title_field_name: Option<crate::value::ExpString>,
        pub field_mappings: Option<Vec<DataSourceToIndexFieldMapping_>>,
        pub include_filter_types: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_SalesforceChatterFeedConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.SalesforceChatterFeedConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_SalesforceChatterFeedConfiguration as SalesforceChatterFeedConfiguration;
    impl crate::value::ToValue for SalesforceChatterFeedConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DocumentDataFieldName".to_string(),
                crate::value::ToValue::to_value(&self.document_data_field_name),
            );
            if let Some(ref value) = self.document_title_field_name {
                properties.insert(
                    "DocumentTitleFieldName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.field_mappings {
                properties.insert(
                    "FieldMappings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_filter_types {
                properties.insert(
                    "IncludeFilterTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforceconfiguration.html
    pub struct SalesforceConfiguration_ {
        pub chatter_feed_configuration: Option<Box<SalesforceChatterFeedConfiguration_>>,
        pub crawl_attachments: Option<crate::value::ExpBool>,
        pub exclude_attachment_file_patterns: Option<Vec<crate::value::ExpString>>,
        pub include_attachment_file_patterns: Option<Vec<crate::value::ExpString>>,
        pub knowledge_article_configuration: Option<Box<SalesforceKnowledgeArticleConfiguration_>>,
        pub secret_arn: crate::value::ExpString,
        pub server_url: crate::value::ExpString,
        pub standard_object_attachment_configuration:
            Option<Box<SalesforceStandardObjectAttachmentConfiguration_>>,
        pub standard_object_configurations: Option<Vec<SalesforceStandardObjectConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_SalesforceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.SalesforceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_SalesforceConfiguration as SalesforceConfiguration;
    impl crate::value::ToValue for SalesforceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.chatter_feed_configuration {
                properties.insert(
                    "ChatterFeedConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.crawl_attachments {
                properties.insert(
                    "CrawlAttachments".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exclude_attachment_file_patterns {
                properties.insert(
                    "ExcludeAttachmentFilePatterns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_attachment_file_patterns {
                properties.insert(
                    "IncludeAttachmentFilePatterns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.knowledge_article_configuration {
                properties.insert(
                    "KnowledgeArticleConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SecretArn".to_string(),
                crate::value::ToValue::to_value(&self.secret_arn),
            );
            properties.insert(
                "ServerUrl".to_string(),
                crate::value::ToValue::to_value(&self.server_url),
            );
            if let Some(ref value) = self.standard_object_attachment_configuration {
                properties.insert(
                    "StandardObjectAttachmentConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.standard_object_configurations {
                properties.insert(
                    "StandardObjectConfigurations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforcecustomknowledgearticletypeconfiguration.html
    pub struct SalesforceCustomKnowledgeArticleTypeConfiguration_ {
        pub document_data_field_name: crate::value::ExpString,
        pub document_title_field_name: Option<crate::value::ExpString>,
        pub field_mappings: Option<Vec<DataSourceToIndexFieldMapping_>>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_SalesforceCustomKnowledgeArticleTypeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.SalesforceCustomKnowledgeArticleTypeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_SalesforceCustomKnowledgeArticleTypeConfiguration as SalesforceCustomKnowledgeArticleTypeConfiguration;
    impl crate::value::ToValue for SalesforceCustomKnowledgeArticleTypeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DocumentDataFieldName".to_string(),
                crate::value::ToValue::to_value(&self.document_data_field_name),
            );
            if let Some(ref value) = self.document_title_field_name {
                properties.insert(
                    "DocumentTitleFieldName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.field_mappings {
                properties.insert(
                    "FieldMappings".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforceknowledgearticleconfiguration.html
    pub struct SalesforceKnowledgeArticleConfiguration_ {
        pub custom_knowledge_article_type_configurations:
            Option<Vec<SalesforceCustomKnowledgeArticleTypeConfiguration_>>,
        pub included_states: Vec<crate::value::ExpString>,
        pub standard_knowledge_article_type_configuration:
            Option<Box<SalesforceStandardKnowledgeArticleTypeConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_SalesforceKnowledgeArticleConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.SalesforceKnowledgeArticleConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_SalesforceKnowledgeArticleConfiguration as SalesforceKnowledgeArticleConfiguration;
    impl crate::value::ToValue for SalesforceKnowledgeArticleConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_knowledge_article_type_configurations {
                properties.insert(
                    "CustomKnowledgeArticleTypeConfigurations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "IncludedStates".to_string(),
                crate::value::ToValue::to_value(&self.included_states),
            );
            if let Some(ref value) = self.standard_knowledge_article_type_configuration {
                properties.insert(
                    "StandardKnowledgeArticleTypeConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforcestandardknowledgearticletypeconfiguration.html
    pub struct SalesforceStandardKnowledgeArticleTypeConfiguration_ {
        pub document_data_field_name: crate::value::ExpString,
        pub document_title_field_name: Option<crate::value::ExpString>,
        pub field_mappings: Option<Vec<DataSourceToIndexFieldMapping_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_SalesforceStandardKnowledgeArticleTypeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.SalesforceStandardKnowledgeArticleTypeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_SalesforceStandardKnowledgeArticleTypeConfiguration as SalesforceStandardKnowledgeArticleTypeConfiguration;
    impl crate::value::ToValue for SalesforceStandardKnowledgeArticleTypeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DocumentDataFieldName".to_string(),
                crate::value::ToValue::to_value(&self.document_data_field_name),
            );
            if let Some(ref value) = self.document_title_field_name {
                properties.insert(
                    "DocumentTitleFieldName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.field_mappings {
                properties.insert(
                    "FieldMappings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforcestandardobjectattachmentconfiguration.html
    pub struct SalesforceStandardObjectAttachmentConfiguration_ {
        pub document_title_field_name: Option<crate::value::ExpString>,
        pub field_mappings: Option<Vec<DataSourceToIndexFieldMapping_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_SalesforceStandardObjectAttachmentConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.SalesforceStandardObjectAttachmentConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_SalesforceStandardObjectAttachmentConfiguration as SalesforceStandardObjectAttachmentConfiguration;
    impl crate::value::ToValue for SalesforceStandardObjectAttachmentConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.document_title_field_name {
                properties.insert(
                    "DocumentTitleFieldName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.field_mappings {
                properties.insert(
                    "FieldMappings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforcestandardobjectconfiguration.html
    pub struct SalesforceStandardObjectConfiguration_ {
        pub document_data_field_name: crate::value::ExpString,
        pub document_title_field_name: Option<crate::value::ExpString>,
        pub field_mappings: Option<Vec<DataSourceToIndexFieldMapping_>>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_SalesforceStandardObjectConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.SalesforceStandardObjectConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_SalesforceStandardObjectConfiguration as SalesforceStandardObjectConfiguration;
    impl crate::value::ToValue for SalesforceStandardObjectConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DocumentDataFieldName".to_string(),
                crate::value::ToValue::to_value(&self.document_data_field_name),
            );
            if let Some(ref value) = self.document_title_field_name {
                properties.insert(
                    "DocumentTitleFieldName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.field_mappings {
                properties.insert(
                    "FieldMappings".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-servicenowconfiguration.html
    pub struct ServiceNowConfiguration_ {
        pub authentication_type: Option<crate::value::ExpString>,
        pub host_url: crate::value::ExpString,
        pub knowledge_article_configuration: Option<Box<ServiceNowKnowledgeArticleConfiguration_>>,
        pub secret_arn: crate::value::ExpString,
        pub service_catalog_configuration: Option<Box<ServiceNowServiceCatalogConfiguration_>>,
        pub service_now_build_version: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_ServiceNowConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.ServiceNowConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_ServiceNowConfiguration as ServiceNowConfiguration;
    impl crate::value::ToValue for ServiceNowConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.authentication_type {
                properties.insert(
                    "AuthenticationType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "HostUrl".to_string(),
                crate::value::ToValue::to_value(&self.host_url),
            );
            if let Some(ref value) = self.knowledge_article_configuration {
                properties.insert(
                    "KnowledgeArticleConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SecretArn".to_string(),
                crate::value::ToValue::to_value(&self.secret_arn),
            );
            if let Some(ref value) = self.service_catalog_configuration {
                properties.insert(
                    "ServiceCatalogConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ServiceNowBuildVersion".to_string(),
                crate::value::ToValue::to_value(&self.service_now_build_version),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-servicenowknowledgearticleconfiguration.html
    pub struct ServiceNowKnowledgeArticleConfiguration_ {
        pub crawl_attachments: Option<crate::value::ExpBool>,
        pub document_data_field_name: crate::value::ExpString,
        pub document_title_field_name: Option<crate::value::ExpString>,
        pub exclude_attachment_file_patterns: Option<Vec<crate::value::ExpString>>,
        pub field_mappings: Option<Vec<DataSourceToIndexFieldMapping_>>,
        pub filter_query: Option<crate::value::ExpString>,
        pub include_attachment_file_patterns: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_ServiceNowKnowledgeArticleConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.ServiceNowKnowledgeArticleConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_ServiceNowKnowledgeArticleConfiguration as ServiceNowKnowledgeArticleConfiguration;
    impl crate::value::ToValue for ServiceNowKnowledgeArticleConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.crawl_attachments {
                properties.insert(
                    "CrawlAttachments".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DocumentDataFieldName".to_string(),
                crate::value::ToValue::to_value(&self.document_data_field_name),
            );
            if let Some(ref value) = self.document_title_field_name {
                properties.insert(
                    "DocumentTitleFieldName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exclude_attachment_file_patterns {
                properties.insert(
                    "ExcludeAttachmentFilePatterns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.field_mappings {
                properties.insert(
                    "FieldMappings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.filter_query {
                properties.insert(
                    "FilterQuery".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_attachment_file_patterns {
                properties.insert(
                    "IncludeAttachmentFilePatterns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-servicenowservicecatalogconfiguration.html
    pub struct ServiceNowServiceCatalogConfiguration_ {
        pub crawl_attachments: Option<crate::value::ExpBool>,
        pub document_data_field_name: crate::value::ExpString,
        pub document_title_field_name: Option<crate::value::ExpString>,
        pub exclude_attachment_file_patterns: Option<Vec<crate::value::ExpString>>,
        pub field_mappings: Option<Vec<DataSourceToIndexFieldMapping_>>,
        pub include_attachment_file_patterns: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_ServiceNowServiceCatalogConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.ServiceNowServiceCatalogConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_ServiceNowServiceCatalogConfiguration as ServiceNowServiceCatalogConfiguration;
    impl crate::value::ToValue for ServiceNowServiceCatalogConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.crawl_attachments {
                properties.insert(
                    "CrawlAttachments".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DocumentDataFieldName".to_string(),
                crate::value::ToValue::to_value(&self.document_data_field_name),
            );
            if let Some(ref value) = self.document_title_field_name {
                properties.insert(
                    "DocumentTitleFieldName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exclude_attachment_file_patterns {
                properties.insert(
                    "ExcludeAttachmentFilePatterns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.field_mappings {
                properties.insert(
                    "FieldMappings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_attachment_file_patterns {
                properties.insert(
                    "IncludeAttachmentFilePatterns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-sharepointconfiguration.html
    pub struct SharePointConfiguration_ {
        pub crawl_attachments: Option<crate::value::ExpBool>,
        pub disable_local_groups: Option<crate::value::ExpBool>,
        pub document_title_field_name: Option<crate::value::ExpString>,
        pub exclusion_patterns: Option<Vec<crate::value::ExpString>>,
        pub field_mappings: Option<Vec<DataSourceToIndexFieldMapping_>>,
        pub inclusion_patterns: Option<Vec<crate::value::ExpString>>,
        pub secret_arn: crate::value::ExpString,
        pub share_point_version: crate::value::ExpString,
        pub ssl_certificate_s3_path: Option<Box<S3Path_>>,
        pub urls: Vec<crate::value::ExpString>,
        pub use_change_log: Option<crate::value::ExpBool>,
        pub vpc_configuration: Option<Box<DataSourceVpcConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_SharePointConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.SharePointConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_SharePointConfiguration as SharePointConfiguration;
    impl crate::value::ToValue for SharePointConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.crawl_attachments {
                properties.insert(
                    "CrawlAttachments".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.disable_local_groups {
                properties.insert(
                    "DisableLocalGroups".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.document_title_field_name {
                properties.insert(
                    "DocumentTitleFieldName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exclusion_patterns {
                properties.insert(
                    "ExclusionPatterns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.field_mappings {
                properties.insert(
                    "FieldMappings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inclusion_patterns {
                properties.insert(
                    "InclusionPatterns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SecretArn".to_string(),
                crate::value::ToValue::to_value(&self.secret_arn),
            );
            properties.insert(
                "SharePointVersion".to_string(),
                crate::value::ToValue::to_value(&self.share_point_version),
            );
            if let Some(ref value) = self.ssl_certificate_s3_path {
                properties.insert(
                    "SslCertificateS3Path".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Urls".to_string(),
                crate::value::ToValue::to_value(&self.urls),
            );
            if let Some(ref value) = self.use_change_log {
                properties.insert(
                    "UseChangeLog".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-sqlconfiguration.html
    pub struct SqlConfiguration_ {
        pub query_identifiers_enclosing_option: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_SqlConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.SqlConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_SqlConfiguration as SqlConfiguration;
    impl crate::value::ToValue for SqlConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.query_identifiers_enclosing_option {
                properties.insert(
                    "QueryIdentifiersEnclosingOption".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-templateconfiguration.html
    pub struct TemplateConfiguration_ {
        pub template: serde_json::Value,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_TemplateConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.TemplateConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_TemplateConfiguration as TemplateConfiguration;
    impl crate::value::ToValue for TemplateConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Template".to_string(),
                crate::value::ToValue::to_value(&self.template),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-webcrawlerauthenticationconfiguration.html
    pub struct WebCrawlerAuthenticationConfiguration_ {
        pub basic_authentication: Option<Vec<WebCrawlerBasicAuthentication_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_WebCrawlerAuthenticationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.WebCrawlerAuthenticationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_WebCrawlerAuthenticationConfiguration as WebCrawlerAuthenticationConfiguration;
    impl crate::value::ToValue for WebCrawlerAuthenticationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.basic_authentication {
                properties.insert(
                    "BasicAuthentication".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-webcrawlerbasicauthentication.html
    pub struct WebCrawlerBasicAuthentication_ {
        pub credentials: crate::value::ExpString,
        pub host: crate::value::ExpString,
        pub port: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_WebCrawlerBasicAuthentication {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.WebCrawlerBasicAuthentication"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_WebCrawlerBasicAuthentication as WebCrawlerBasicAuthentication;
    impl crate::value::ToValue for WebCrawlerBasicAuthentication_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Credentials".to_string(),
                crate::value::ToValue::to_value(&self.credentials),
            );
            properties.insert(
                "Host".to_string(),
                crate::value::ToValue::to_value(&self.host),
            );
            properties.insert(
                "Port".to_string(),
                crate::value::ToValue::to_value(&self.port),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-webcrawlerconfiguration.html
    pub struct WebCrawlerConfiguration_ {
        pub authentication_configuration: Option<Box<WebCrawlerAuthenticationConfiguration_>>,
        pub crawl_depth: Option<i64>,
        pub max_content_size_per_page_in_mega_bytes: Option<f64>,
        pub max_links_per_page: Option<i64>,
        pub max_urls_per_minute_crawl_rate: Option<i64>,
        pub proxy_configuration: Option<Box<ProxyConfiguration_>>,
        pub url_exclusion_patterns: Option<Vec<crate::value::ExpString>>,
        pub url_inclusion_patterns: Option<Vec<crate::value::ExpString>>,
        pub urls: Box<WebCrawlerUrls_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_WebCrawlerConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.WebCrawlerConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_WebCrawlerConfiguration as WebCrawlerConfiguration;
    impl crate::value::ToValue for WebCrawlerConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.authentication_configuration {
                properties.insert(
                    "AuthenticationConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.crawl_depth {
                properties.insert(
                    "CrawlDepth".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_content_size_per_page_in_mega_bytes {
                properties.insert(
                    "MaxContentSizePerPageInMegaBytes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_links_per_page {
                properties.insert(
                    "MaxLinksPerPage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_urls_per_minute_crawl_rate {
                properties.insert(
                    "MaxUrlsPerMinuteCrawlRate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.proxy_configuration {
                properties.insert(
                    "ProxyConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.url_exclusion_patterns {
                properties.insert(
                    "UrlExclusionPatterns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.url_inclusion_patterns {
                properties.insert(
                    "UrlInclusionPatterns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Urls".to_string(),
                crate::value::ToValue::to_value(&self.urls),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-webcrawlerseedurlconfiguration.html
    pub struct WebCrawlerSeedUrlConfiguration_ {
        pub seed_urls: Vec<crate::value::ExpString>,
        pub web_crawler_mode: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_WebCrawlerSeedUrlConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.WebCrawlerSeedUrlConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_WebCrawlerSeedUrlConfiguration as WebCrawlerSeedUrlConfiguration;
    impl crate::value::ToValue for WebCrawlerSeedUrlConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SeedUrls".to_string(),
                crate::value::ToValue::to_value(&self.seed_urls),
            );
            if let Some(ref value) = self.web_crawler_mode {
                properties.insert(
                    "WebCrawlerMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-webcrawlersitemapsconfiguration.html
    pub struct WebCrawlerSiteMapsConfiguration_ {
        pub site_maps: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_WebCrawlerSiteMapsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.WebCrawlerSiteMapsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_WebCrawlerSiteMapsConfiguration as WebCrawlerSiteMapsConfiguration;
    impl crate::value::ToValue for WebCrawlerSiteMapsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SiteMaps".to_string(),
                crate::value::ToValue::to_value(&self.site_maps),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-webcrawlerurls.html
    pub struct WebCrawlerUrls_ {
        pub seed_url_configuration: Option<Box<WebCrawlerSeedUrlConfiguration_>>,
        pub site_maps_configuration: Option<Box<WebCrawlerSiteMapsConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_WebCrawlerUrls {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.WebCrawlerUrls"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_WebCrawlerUrls as WebCrawlerUrls;
    impl crate::value::ToValue for WebCrawlerUrls_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.seed_url_configuration {
                properties.insert(
                    "SeedUrlConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.site_maps_configuration {
                properties.insert(
                    "SiteMapsConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-workdocsconfiguration.html
    pub struct WorkDocsConfiguration_ {
        pub crawl_comments: Option<crate::value::ExpBool>,
        pub exclusion_patterns: Option<Vec<crate::value::ExpString>>,
        pub field_mappings: Option<Vec<DataSourceToIndexFieldMapping_>>,
        pub inclusion_patterns: Option<Vec<crate::value::ExpString>>,
        pub organization_id: crate::value::ExpString,
        pub use_change_log: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_WorkDocsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::DataSource.WorkDocsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_WorkDocsConfiguration as WorkDocsConfiguration;
    impl crate::value::ToValue for WorkDocsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.crawl_comments {
                properties.insert(
                    "CrawlComments".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exclusion_patterns {
                properties.insert(
                    "ExclusionPatterns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.field_mappings {
                properties.insert(
                    "FieldMappings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inclusion_patterns {
                properties.insert(
                    "InclusionPatterns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "OrganizationId".to_string(),
                crate::value::ToValue::to_value(&self.organization_id),
            );
            if let Some(ref value) = self.use_change_log {
                properties.insert(
                    "UseChangeLog".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod faq {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-faq-s3path.html
    pub struct S3Path_ {
        pub bucket: crate::value::ExpString,
        pub key: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_Faq_S3Path {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::Faq.S3Path"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_Faq_S3Path as S3Path;
    impl crate::value::ToValue for S3Path_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Bucket".to_string(),
                crate::value::ToValue::to_value(&self.bucket),
            );
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            properties.into()
        }
    }
}
pub mod index {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-capacityunitsconfiguration.html
    pub struct CapacityUnitsConfiguration_ {
        pub query_capacity_units: i64,
        pub storage_capacity_units: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_Index_CapacityUnitsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::Index.CapacityUnitsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_Index_CapacityUnitsConfiguration as CapacityUnitsConfiguration;
    impl crate::value::ToValue for CapacityUnitsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "QueryCapacityUnits".to_string(),
                crate::value::ToValue::to_value(&self.query_capacity_units),
            );
            properties.insert(
                "StorageCapacityUnits".to_string(),
                crate::value::ToValue::to_value(&self.storage_capacity_units),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-documentmetadataconfiguration.html
    pub struct DocumentMetadataConfiguration_ {
        pub name: crate::value::ExpString,
        pub relevance: Option<Box<Relevance_>>,
        pub search: Option<Box<Search_>>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_Index_DocumentMetadataConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::Index.DocumentMetadataConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_Index_DocumentMetadataConfiguration as DocumentMetadataConfiguration;
    impl crate::value::ToValue for DocumentMetadataConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.relevance {
                properties.insert(
                    "Relevance".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.search {
                properties.insert("Search".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-jsontokentypeconfiguration.html
    pub struct JsonTokenTypeConfiguration_ {
        pub group_attribute_field: crate::value::ExpString,
        pub user_name_attribute_field: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_Index_JsonTokenTypeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::Index.JsonTokenTypeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_Index_JsonTokenTypeConfiguration as JsonTokenTypeConfiguration;
    impl crate::value::ToValue for JsonTokenTypeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "GroupAttributeField".to_string(),
                crate::value::ToValue::to_value(&self.group_attribute_field),
            );
            properties.insert(
                "UserNameAttributeField".to_string(),
                crate::value::ToValue::to_value(&self.user_name_attribute_field),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-jwttokentypeconfiguration.html
    pub struct JwtTokenTypeConfiguration_ {
        pub claim_regex: Option<crate::value::ExpString>,
        pub group_attribute_field: Option<crate::value::ExpString>,
        pub issuer: Option<crate::value::ExpString>,
        pub key_location: crate::value::ExpString,
        pub secret_manager_arn: Option<crate::value::ExpString>,
        pub url: Option<crate::value::ExpString>,
        pub user_name_attribute_field: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_Index_JwtTokenTypeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::Index.JwtTokenTypeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_Index_JwtTokenTypeConfiguration as JwtTokenTypeConfiguration;
    impl crate::value::ToValue for JwtTokenTypeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.claim_regex {
                properties.insert(
                    "ClaimRegex".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.group_attribute_field {
                properties.insert(
                    "GroupAttributeField".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.issuer {
                properties.insert("Issuer".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "KeyLocation".to_string(),
                crate::value::ToValue::to_value(&self.key_location),
            );
            if let Some(ref value) = self.secret_manager_arn {
                properties.insert(
                    "SecretManagerArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.url {
                properties.insert("URL".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.user_name_attribute_field {
                properties.insert(
                    "UserNameAttributeField".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-relevance.html
    pub struct Relevance_ {
        pub duration: Option<crate::value::ExpString>,
        pub freshness: Option<crate::value::ExpBool>,
        pub importance: Option<i64>,
        pub rank_order: Option<crate::value::ExpString>,
        pub value_importance_items: Option<Vec<ValueImportanceItem_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_Index_Relevance {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::Index.Relevance"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_Index_Relevance as Relevance;
    impl crate::value::ToValue for Relevance_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.duration {
                properties.insert(
                    "Duration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.freshness {
                properties.insert(
                    "Freshness".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.importance {
                properties.insert(
                    "Importance".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rank_order {
                properties.insert(
                    "RankOrder".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.value_importance_items {
                properties.insert(
                    "ValueImportanceItems".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-search.html
    pub struct Search_ {
        pub displayable: Option<crate::value::ExpBool>,
        pub facetable: Option<crate::value::ExpBool>,
        pub searchable: Option<crate::value::ExpBool>,
        pub sortable: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_Index_Search {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::Index.Search"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_Index_Search as Search;
    impl crate::value::ToValue for Search_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.displayable {
                properties.insert(
                    "Displayable".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.facetable {
                properties.insert(
                    "Facetable".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.searchable {
                properties.insert(
                    "Searchable".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sortable {
                properties.insert(
                    "Sortable".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-serversideencryptionconfiguration.html
    pub struct ServerSideEncryptionConfiguration_ {
        pub kms_key_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_Index_ServerSideEncryptionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::Index.ServerSideEncryptionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_Index_ServerSideEncryptionConfiguration as ServerSideEncryptionConfiguration;
    impl crate::value::ToValue for ServerSideEncryptionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-usertokenconfiguration.html
    pub struct UserTokenConfiguration_ {
        pub json_token_type_configuration: Option<Box<JsonTokenTypeConfiguration_>>,
        pub jwt_token_type_configuration: Option<Box<JwtTokenTypeConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_Index_UserTokenConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::Index.UserTokenConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_Index_UserTokenConfiguration as UserTokenConfiguration;
    impl crate::value::ToValue for UserTokenConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.json_token_type_configuration {
                properties.insert(
                    "JsonTokenTypeConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.jwt_token_type_configuration {
                properties.insert(
                    "JwtTokenTypeConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-valueimportanceitem.html
    pub struct ValueImportanceItem_ {
        pub key: Option<crate::value::ExpString>,
        pub value: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_Index_ValueImportanceItem {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Kendra::Index.ValueImportanceItem"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_Index_ValueImportanceItem as ValueImportanceItem;
    impl crate::value::ToValue for ValueImportanceItem_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.key {
                properties.insert("Key".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kendra-datasource.html
pub struct DataSource_ {
    pub custom_document_enrichment_configuration:
        Option<super::kendra::datasource::CustomDocumentEnrichmentConfiguration_>,
    pub data_source_configuration: Option<super::kendra::datasource::DataSourceConfiguration_>,
    pub description: Option<crate::value::ExpString>,
    pub index_id: crate::value::ExpString,
    pub language_code: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub role_arn: Option<crate::value::ExpString>,
    pub schedule: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_kendra_DataSource {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Kendra::DataSource"
        $($field $value)*)
    };
}
pub use crate::__aws_kendra_DataSource as DataSource;
impl crate::template::ToResource for DataSource_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Kendra"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DataSource"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.custom_document_enrichment_configuration {
            properties.insert(
                "CustomDocumentEnrichmentConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.data_source_configuration {
            properties.insert(
                "DataSourceConfiguration".to_string(),
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
            "IndexId".to_string(),
            crate::value::ToValue::to_value(&self.index_id),
        );
        if let Some(ref value) = self.language_code {
            properties.insert(
                "LanguageCode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.role_arn {
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.schedule {
            properties.insert(
                "Schedule".to_string(),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kendra-faq.html
pub struct Faq_ {
    pub description: Option<crate::value::ExpString>,
    pub file_format: Option<crate::value::ExpString>,
    pub index_id: crate::value::ExpString,
    pub language_code: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub role_arn: crate::value::ExpString,
    pub s3_path: super::kendra::faq::S3Path_,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_kendra_Faq {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Kendra::Faq" $($field
        $value)*)
    };
}
pub use crate::__aws_kendra_Faq as Faq;
impl crate::template::ToResource for Faq_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Kendra"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Faq"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.file_format {
            properties.insert(
                "FileFormat".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "IndexId".to_string(),
            crate::value::ToValue::to_value(&self.index_id),
        );
        if let Some(ref value) = self.language_code {
            properties.insert(
                "LanguageCode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        properties.insert(
            "S3Path".to_string(),
            crate::value::ToValue::to_value(&self.s3_path),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kendra-index.html
pub struct Index_ {
    pub capacity_units: Option<super::kendra::index::CapacityUnitsConfiguration_>,
    pub description: Option<crate::value::ExpString>,
    pub document_metadata_configurations:
        Option<Vec<super::kendra::index::DocumentMetadataConfiguration_>>,
    pub edition: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub role_arn: crate::value::ExpString,
    pub server_side_encryption_configuration:
        Option<super::kendra::index::ServerSideEncryptionConfiguration_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub user_context_policy: Option<crate::value::ExpString>,
    pub user_token_configurations: Option<Vec<super::kendra::index::UserTokenConfiguration_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_kendra_Index {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Kendra::Index" $($field
        $value)*)
    };
}
pub use crate::__aws_kendra_Index as Index;
impl crate::template::ToResource for Index_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Kendra"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Index"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.capacity_units {
            properties.insert(
                "CapacityUnits".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.document_metadata_configurations {
            properties.insert(
                "DocumentMetadataConfigurations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Edition".to_string(),
            crate::value::ToValue::to_value(&self.edition),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        if let Some(ref value) = self.server_side_encryption_configuration {
            properties.insert(
                "ServerSideEncryptionConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.user_context_policy {
            properties.insert(
                "UserContextPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.user_token_configurations {
            properties.insert(
                "UserTokenConfigurations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
