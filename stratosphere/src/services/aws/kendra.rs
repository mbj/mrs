pub mod datasource {
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
            stratosphere::generator::construct_property_type!("AWS::Kendra::DataSource.CustomDocumentEnrichmentConfiguration"
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
        pub template_configuration: Option<Box<TemplateConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_DataSourceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Kendra::DataSource.DataSourceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kendra_DataSource_DataSourceConfiguration as DataSourceConfiguration;
    impl crate::value::ToValue for DataSourceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.template_configuration {
                properties.insert(
                    "TemplateConfiguration".to_string(),
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
            stratosphere::generator::construct_property_type!("AWS::Kendra::DataSource.DocumentAttributeCondition"
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
            stratosphere::generator::construct_property_type!("AWS::Kendra::DataSource.DocumentAttributeTarget"
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
            stratosphere::generator::construct_property_type!("AWS::Kendra::DataSource.DocumentAttributeValue"
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
            stratosphere::generator::construct_property_type!("AWS::Kendra::DataSource.HookConfiguration"
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
            stratosphere::generator::construct_property_type!("AWS::Kendra::DataSource.InlineCustomDocumentEnrichmentConfiguration"
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-templateconfiguration.html
    pub struct TemplateConfiguration_ {
        pub template: serde_json::Value,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_DataSource_TemplateConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Kendra::DataSource.TemplateConfiguration"
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
            stratosphere::generator::construct_property_type!("AWS::Kendra::Faq.S3Path"
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
        pub query_capacity_units: i32,
        pub storage_capacity_units: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_Index_CapacityUnitsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Kendra::Index.CapacityUnitsConfiguration"
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
            stratosphere::generator::construct_property_type!("AWS::Kendra::Index.DocumentMetadataConfiguration"
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
            stratosphere::generator::construct_property_type!("AWS::Kendra::Index.JsonTokenTypeConfiguration"
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
            stratosphere::generator::construct_property_type!("AWS::Kendra::Index.JwtTokenTypeConfiguration"
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
        pub importance: Option<i32>,
        pub rank_order: Option<crate::value::ExpString>,
        pub value_importance_items: Option<Vec<ValueImportanceItem_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_Index_Relevance {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Kendra::Index.Relevance"
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
            stratosphere::generator::construct_property_type!("AWS::Kendra::Index.Search"
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
            stratosphere::generator::construct_property_type!("AWS::Kendra::Index.ServerSideEncryptionConfiguration"
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
            stratosphere::generator::construct_property_type!("AWS::Kendra::Index.UserTokenConfiguration"
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
        pub value: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kendra_Index_ValueImportanceItem {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Kendra::Index.ValueImportanceItem"
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
        stratosphere::generator::construct_resource_type!("AWS::Kendra::DataSource"
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
        stratosphere::generator::construct_resource_type!("AWS::Kendra::Faq" $($field
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
        stratosphere::generator::construct_resource_type!("AWS::Kendra::Index" $($field
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
