pub mod application {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-application-attachmentsconfiguration.html>
    pub struct AttachmentsConfiguration_ {
        pub attachments_control_mode: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_Application_AttachmentsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::Application.AttachmentsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_Application_AttachmentsConfiguration as AttachmentsConfiguration;
    impl crate::value::ToValue for AttachmentsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AttachmentsControlMode".to_string(),
                crate::value::ToValue::to_value(&self.attachments_control_mode),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-application-autosubscriptionconfiguration.html>
    pub struct AutoSubscriptionConfiguration_ {
        pub auto_subscribe: crate::value::ExpString,
        pub default_subscription_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_Application_AutoSubscriptionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::Application.AutoSubscriptionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_Application_AutoSubscriptionConfiguration as AutoSubscriptionConfiguration;
    impl crate::value::ToValue for AutoSubscriptionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AutoSubscribe".to_string(),
                crate::value::ToValue::to_value(&self.auto_subscribe),
            );
            if let Some(ref value) = self.default_subscription_type {
                properties.insert(
                    "DefaultSubscriptionType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-application-encryptionconfiguration.html>
    pub struct EncryptionConfiguration_ {
        pub kms_key_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_Application_EncryptionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::Application.EncryptionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_Application_EncryptionConfiguration as EncryptionConfiguration;
    impl crate::value::ToValue for EncryptionConfiguration_ {
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-application-personalizationconfiguration.html>
    pub struct PersonalizationConfiguration_ {
        pub personalization_control_mode: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_Application_PersonalizationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::Application.PersonalizationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_Application_PersonalizationConfiguration as PersonalizationConfiguration;
    impl crate::value::ToValue for PersonalizationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PersonalizationControlMode".to_string(),
                crate::value::ToValue::to_value(&self.personalization_control_mode),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-application-qappsconfiguration.html>
    pub struct QAppsConfiguration_ {
        pub q_apps_control_mode: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_Application_QAppsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::Application.QAppsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_Application_QAppsConfiguration as QAppsConfiguration;
    impl crate::value::ToValue for QAppsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "QAppsControlMode".to_string(),
                crate::value::ToValue::to_value(&self.q_apps_control_mode),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-application-quicksightconfiguration.html>
    pub struct QuickSightConfiguration_ {
        pub client_namespace: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_Application_QuickSightConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::Application.QuickSightConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_Application_QuickSightConfiguration as QuickSightConfiguration;
    impl crate::value::ToValue for QuickSightConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ClientNamespace".to_string(),
                crate::value::ToValue::to_value(&self.client_namespace),
            );
            properties.into()
        }
    }
}
pub mod dataaccessor {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-dataaccessor-actionconfiguration.html>
    pub struct ActionConfiguration_ {
        pub action: crate::value::ExpString,
        pub filter_configuration: Option<Box<ActionFilterConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_DataAccessor_ActionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::DataAccessor.ActionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_DataAccessor_ActionConfiguration as ActionConfiguration;
    impl crate::value::ToValue for ActionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Action".to_string(),
                crate::value::ToValue::to_value(&self.action),
            );
            if let Some(ref value) = self.filter_configuration {
                properties.insert(
                    "FilterConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-dataaccessor-actionfilterconfiguration.html>
    pub struct ActionFilterConfiguration_ {
        pub document_attribute_filter: Box<AttributeFilter_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_DataAccessor_ActionFilterConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::DataAccessor.ActionFilterConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_DataAccessor_ActionFilterConfiguration as ActionFilterConfiguration;
    impl crate::value::ToValue for ActionFilterConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DocumentAttributeFilter".to_string(),
                crate::value::ToValue::to_value(&self.document_attribute_filter),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-dataaccessor-attributefilter.html>
    pub struct AttributeFilter_ {
        pub and_all_filters: Option<Vec<AttributeFilter_>>,
        pub contains_all: Option<Box<DocumentAttribute_>>,
        pub contains_any: Option<Box<DocumentAttribute_>>,
        pub equals_to: Option<Box<DocumentAttribute_>>,
        pub greater_than: Option<Box<DocumentAttribute_>>,
        pub greater_than_or_equals: Option<Box<DocumentAttribute_>>,
        pub less_than: Option<Box<DocumentAttribute_>>,
        pub less_than_or_equals: Option<Box<DocumentAttribute_>>,
        pub not_filter: Option<Box<AttributeFilter_>>,
        pub or_all_filters: Option<Vec<AttributeFilter_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_DataAccessor_AttributeFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::DataAccessor.AttributeFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_DataAccessor_AttributeFilter as AttributeFilter;
    impl crate::value::ToValue for AttributeFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.and_all_filters {
                properties.insert(
                    "AndAllFilters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.contains_all {
                properties.insert(
                    "ContainsAll".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.contains_any {
                properties.insert(
                    "ContainsAny".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.equals_to {
                properties.insert(
                    "EqualsTo".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.greater_than {
                properties.insert(
                    "GreaterThan".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.greater_than_or_equals {
                properties.insert(
                    "GreaterThanOrEquals".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.less_than {
                properties.insert(
                    "LessThan".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.less_than_or_equals {
                properties.insert(
                    "LessThanOrEquals".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.not_filter {
                properties.insert(
                    "NotFilter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.or_all_filters {
                properties.insert(
                    "OrAllFilters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-dataaccessor-dataaccessorauthenticationconfiguration.html>
    pub struct DataAccessorAuthenticationConfiguration_ {
        pub idc_trusted_token_issuer_configuration:
            Box<DataAccessorIdcTrustedTokenIssuerConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_DataAccessor_DataAccessorAuthenticationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::DataAccessor.DataAccessorAuthenticationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_DataAccessor_DataAccessorAuthenticationConfiguration as DataAccessorAuthenticationConfiguration;
    impl crate::value::ToValue for DataAccessorAuthenticationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IdcTrustedTokenIssuerConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.idc_trusted_token_issuer_configuration),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-dataaccessor-dataaccessorauthenticationdetail.html>
    pub struct DataAccessorAuthenticationDetail_ {
        pub authentication_configuration: Option<Box<DataAccessorAuthenticationConfiguration_>>,
        pub authentication_type: crate::value::ExpString,
        pub external_ids: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_DataAccessor_DataAccessorAuthenticationDetail {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::DataAccessor.DataAccessorAuthenticationDetail"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_DataAccessor_DataAccessorAuthenticationDetail as DataAccessorAuthenticationDetail;
    impl crate::value::ToValue for DataAccessorAuthenticationDetail_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.authentication_configuration {
                properties.insert(
                    "AuthenticationConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "AuthenticationType".to_string(),
                crate::value::ToValue::to_value(&self.authentication_type),
            );
            if let Some(ref value) = self.external_ids {
                properties.insert(
                    "ExternalIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-dataaccessor-dataaccessoridctrustedtokenissuerconfiguration.html>
    pub struct DataAccessorIdcTrustedTokenIssuerConfiguration_ {
        pub idc_trusted_token_issuer_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_DataAccessor_DataAccessorIdcTrustedTokenIssuerConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::DataAccessor.DataAccessorIdcTrustedTokenIssuerConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_DataAccessor_DataAccessorIdcTrustedTokenIssuerConfiguration as DataAccessorIdcTrustedTokenIssuerConfiguration;
    impl crate::value::ToValue for DataAccessorIdcTrustedTokenIssuerConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IdcTrustedTokenIssuerArn".to_string(),
                crate::value::ToValue::to_value(&self.idc_trusted_token_issuer_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-dataaccessor-documentattribute.html>
    pub struct DocumentAttribute_ {
        pub name: crate::value::ExpString,
        pub value: Box<DocumentAttributeValue_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_DataAccessor_DocumentAttribute {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::DataAccessor.DocumentAttribute"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_DataAccessor_DocumentAttribute as DocumentAttribute;
    impl crate::value::ToValue for DocumentAttribute_ {
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-dataaccessor-documentattributevalue.html>
    pub struct DocumentAttributeValue_ {
        pub date_value: Option<crate::value::ExpString>,
        pub long_value: Option<f64>,
        pub string_list_value: Option<Vec<crate::value::ExpString>>,
        pub string_value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_DataAccessor_DocumentAttributeValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::DataAccessor.DocumentAttributeValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_DataAccessor_DocumentAttributeValue as DocumentAttributeValue;
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
}
pub mod datasource {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-datasource-audioextractionconfiguration.html>
    pub struct AudioExtractionConfiguration_ {
        pub audio_extraction_status: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_DataSource_AudioExtractionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::DataSource.AudioExtractionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_DataSource_AudioExtractionConfiguration as AudioExtractionConfiguration;
    impl crate::value::ToValue for AudioExtractionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AudioExtractionStatus".to_string(),
                crate::value::ToValue::to_value(&self.audio_extraction_status),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-datasource-datasourcevpcconfiguration.html>
    pub struct DataSourceVpcConfiguration_ {
        pub security_group_ids: Vec<crate::value::ExpString>,
        pub subnet_ids: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_DataSource_DataSourceVpcConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::DataSource.DataSourceVpcConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_DataSource_DataSourceVpcConfiguration as DataSourceVpcConfiguration;
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-datasource-documentattributecondition.html>
    pub struct DocumentAttributeCondition_ {
        pub key: crate::value::ExpString,
        pub operator: crate::value::ExpString,
        pub value: Option<Box<DocumentAttributeValue_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_DataSource_DocumentAttributeCondition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::DataSource.DocumentAttributeCondition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_DataSource_DocumentAttributeCondition as DocumentAttributeCondition;
    impl crate::value::ToValue for DocumentAttributeCondition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            properties.insert(
                "Operator".to_string(),
                crate::value::ToValue::to_value(&self.operator),
            );
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-datasource-documentattributetarget.html>
    pub struct DocumentAttributeTarget_ {
        pub attribute_value_operator: Option<crate::value::ExpString>,
        pub key: crate::value::ExpString,
        pub value: Option<Box<DocumentAttributeValue_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_DataSource_DocumentAttributeTarget {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::DataSource.DocumentAttributeTarget"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_DataSource_DocumentAttributeTarget as DocumentAttributeTarget;
    impl crate::value::ToValue for DocumentAttributeTarget_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attribute_value_operator {
                properties.insert(
                    "AttributeValueOperator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-datasource-documentattributevalue.html>
    pub struct DocumentAttributeValue_ {
        pub date_value: Option<crate::value::ExpString>,
        pub long_value: Option<f64>,
        pub string_list_value: Option<Vec<crate::value::ExpString>>,
        pub string_value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_DataSource_DocumentAttributeValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::DataSource.DocumentAttributeValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_DataSource_DocumentAttributeValue as DocumentAttributeValue;
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-datasource-documentenrichmentconfiguration.html>
    pub struct DocumentEnrichmentConfiguration_ {
        pub inline_configurations: Option<Vec<InlineDocumentEnrichmentConfiguration_>>,
        pub post_extraction_hook_configuration: Option<Box<HookConfiguration_>>,
        pub pre_extraction_hook_configuration: Option<Box<HookConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_DataSource_DocumentEnrichmentConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::DataSource.DocumentEnrichmentConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_DataSource_DocumentEnrichmentConfiguration as DocumentEnrichmentConfiguration;
    impl crate::value::ToValue for DocumentEnrichmentConfiguration_ {
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
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-datasource-hookconfiguration.html>
    pub struct HookConfiguration_ {
        pub invocation_condition: Option<Box<DocumentAttributeCondition_>>,
        pub lambda_arn: Option<crate::value::ExpString>,
        pub role_arn: Option<crate::value::ExpString>,
        pub s3_bucket_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_DataSource_HookConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::DataSource.HookConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_DataSource_HookConfiguration as HookConfiguration;
    impl crate::value::ToValue for HookConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.invocation_condition {
                properties.insert(
                    "InvocationCondition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lambda_arn {
                properties.insert(
                    "LambdaArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.role_arn {
                properties.insert(
                    "RoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_bucket_name {
                properties.insert(
                    "S3BucketName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-datasource-imageextractionconfiguration.html>
    pub struct ImageExtractionConfiguration_ {
        pub image_extraction_status: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_DataSource_ImageExtractionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::DataSource.ImageExtractionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_DataSource_ImageExtractionConfiguration as ImageExtractionConfiguration;
    impl crate::value::ToValue for ImageExtractionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ImageExtractionStatus".to_string(),
                crate::value::ToValue::to_value(&self.image_extraction_status),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-datasource-inlinedocumentenrichmentconfiguration.html>
    pub struct InlineDocumentEnrichmentConfiguration_ {
        pub condition: Option<Box<DocumentAttributeCondition_>>,
        pub document_content_operator: Option<crate::value::ExpString>,
        pub target: Option<Box<DocumentAttributeTarget_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_DataSource_InlineDocumentEnrichmentConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::DataSource.InlineDocumentEnrichmentConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_DataSource_InlineDocumentEnrichmentConfiguration as InlineDocumentEnrichmentConfiguration;
    impl crate::value::ToValue for InlineDocumentEnrichmentConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.condition {
                properties.insert(
                    "Condition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.document_content_operator {
                properties.insert(
                    "DocumentContentOperator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target {
                properties.insert("Target".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-datasource-mediaextractionconfiguration.html>
    pub struct MediaExtractionConfiguration_ {
        pub audio_extraction_configuration: Option<Box<AudioExtractionConfiguration_>>,
        pub image_extraction_configuration: Option<Box<ImageExtractionConfiguration_>>,
        pub video_extraction_configuration: Option<Box<VideoExtractionConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_DataSource_MediaExtractionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::DataSource.MediaExtractionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_DataSource_MediaExtractionConfiguration as MediaExtractionConfiguration;
    impl crate::value::ToValue for MediaExtractionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.audio_extraction_configuration {
                properties.insert(
                    "AudioExtractionConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.image_extraction_configuration {
                properties.insert(
                    "ImageExtractionConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.video_extraction_configuration {
                properties.insert(
                    "VideoExtractionConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-datasource-videoextractionconfiguration.html>
    pub struct VideoExtractionConfiguration_ {
        pub video_extraction_status: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_DataSource_VideoExtractionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::DataSource.VideoExtractionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_DataSource_VideoExtractionConfiguration as VideoExtractionConfiguration;
    impl crate::value::ToValue for VideoExtractionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "VideoExtractionStatus".to_string(),
                crate::value::ToValue::to_value(&self.video_extraction_status),
            );
            properties.into()
        }
    }
}
pub mod index {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-index-documentattributeconfiguration.html>
    pub struct DocumentAttributeConfiguration_ {
        pub name: Option<crate::value::ExpString>,
        pub search: Option<crate::value::ExpString>,
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_Index_DocumentAttributeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::Index.DocumentAttributeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_Index_DocumentAttributeConfiguration as DocumentAttributeConfiguration;
    impl crate::value::ToValue for DocumentAttributeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.search {
                properties.insert("Search".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-index-indexcapacityconfiguration.html>
    pub struct IndexCapacityConfiguration_ {
        pub units: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_Index_IndexCapacityConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::Index.IndexCapacityConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_Index_IndexCapacityConfiguration as IndexCapacityConfiguration;
    impl crate::value::ToValue for IndexCapacityConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.units {
                properties.insert("Units".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-index-indexstatistics.html>
    pub struct IndexStatistics_ {
        pub text_document_statistics: Option<Box<TextDocumentStatistics_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_Index_IndexStatistics {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::Index.IndexStatistics"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_Index_IndexStatistics as IndexStatistics;
    impl crate::value::ToValue for IndexStatistics_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.text_document_statistics {
                properties.insert(
                    "TextDocumentStatistics".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-index-textdocumentstatistics.html>
    pub struct TextDocumentStatistics_ {
        pub indexed_text_bytes: Option<f64>,
        pub indexed_text_document_count: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_Index_TextDocumentStatistics {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::Index.TextDocumentStatistics"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_Index_TextDocumentStatistics as TextDocumentStatistics;
    impl crate::value::ToValue for TextDocumentStatistics_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.indexed_text_bytes {
                properties.insert(
                    "IndexedTextBytes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.indexed_text_document_count {
                properties.insert(
                    "IndexedTextDocumentCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod permission {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-permission-condition.html>
    pub struct Condition_ {
        pub condition_key: crate::value::ExpString,
        pub condition_operator: crate::value::ExpString,
        pub condition_values: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_Permission_Condition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::Permission.Condition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_Permission_Condition as Condition;
    impl crate::value::ToValue for Condition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ConditionKey".to_string(),
                crate::value::ToValue::to_value(&self.condition_key),
            );
            properties.insert(
                "ConditionOperator".to_string(),
                crate::value::ToValue::to_value(&self.condition_operator),
            );
            properties.insert(
                "ConditionValues".to_string(),
                crate::value::ToValue::to_value(&self.condition_values),
            );
            properties.into()
        }
    }
}
pub mod plugin {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-plugin-apischema.html>
    pub struct APISchema_ {
        pub payload: Option<crate::value::ExpString>,
        pub s3: Option<Box<S3_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_Plugin_APISchema {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::Plugin.APISchema"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_Plugin_APISchema as APISchema;
    impl crate::value::ToValue for APISchema_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.payload {
                properties.insert(
                    "Payload".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3 {
                properties.insert("S3".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-plugin-basicauthconfiguration.html>
    pub struct BasicAuthConfiguration_ {
        pub role_arn: crate::value::ExpString,
        pub secret_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_Plugin_BasicAuthConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::Plugin.BasicAuthConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_Plugin_BasicAuthConfiguration as BasicAuthConfiguration;
    impl crate::value::ToValue for BasicAuthConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.insert(
                "SecretArn".to_string(),
                crate::value::ToValue::to_value(&self.secret_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-plugin-custompluginconfiguration.html>
    pub struct CustomPluginConfiguration_ {
        pub api_schema: Box<APISchema_>,
        pub api_schema_type: crate::value::ExpString,
        pub description: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_Plugin_CustomPluginConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::Plugin.CustomPluginConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_Plugin_CustomPluginConfiguration as CustomPluginConfiguration;
    impl crate::value::ToValue for CustomPluginConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ApiSchema".to_string(),
                crate::value::ToValue::to_value(&self.api_schema),
            );
            properties.insert(
                "ApiSchemaType".to_string(),
                crate::value::ToValue::to_value(&self.api_schema_type),
            );
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(&self.description),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-plugin-oauth2clientcredentialconfiguration.html>
    pub struct OAuth2ClientCredentialConfiguration_ {
        pub authorization_url: Option<crate::value::ExpString>,
        pub role_arn: crate::value::ExpString,
        pub secret_arn: crate::value::ExpString,
        pub token_url: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_Plugin_OAuth2ClientCredentialConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::Plugin.OAuth2ClientCredentialConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_Plugin_OAuth2ClientCredentialConfiguration as OAuth2ClientCredentialConfiguration;
    impl crate::value::ToValue for OAuth2ClientCredentialConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.authorization_url {
                properties.insert(
                    "AuthorizationUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.insert(
                "SecretArn".to_string(),
                crate::value::ToValue::to_value(&self.secret_arn),
            );
            if let Some(ref value) = self.token_url {
                properties.insert(
                    "TokenUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-plugin-pluginauthconfiguration.html>
    pub struct PluginAuthConfiguration_ {
        pub basic_auth_configuration: Option<Box<BasicAuthConfiguration_>>,
        pub no_auth_configuration: Option<serde_json::Value>,
        pub o_auth2_client_credential_configuration:
            Option<Box<OAuth2ClientCredentialConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_Plugin_PluginAuthConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::Plugin.PluginAuthConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_Plugin_PluginAuthConfiguration as PluginAuthConfiguration;
    impl crate::value::ToValue for PluginAuthConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.basic_auth_configuration {
                properties.insert(
                    "BasicAuthConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.no_auth_configuration {
                properties.insert(
                    "NoAuthConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.o_auth2_client_credential_configuration {
                properties.insert(
                    "OAuth2ClientCredentialConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-plugin-s3.html>
    pub struct S3_ {
        pub bucket: crate::value::ExpString,
        pub key: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_Plugin_S3 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::Plugin.S3"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_Plugin_S3 as S3;
    impl crate::value::ToValue for S3_ {
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
pub mod retriever {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-retriever-kendraindexconfiguration.html>
    pub struct KendraIndexConfiguration_ {
        pub index_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_Retriever_KendraIndexConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::Retriever.KendraIndexConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_Retriever_KendraIndexConfiguration as KendraIndexConfiguration;
    impl crate::value::ToValue for KendraIndexConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IndexId".to_string(),
                crate::value::ToValue::to_value(&self.index_id),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-retriever-nativeindexconfiguration.html>
    pub struct NativeIndexConfiguration_ {
        pub index_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_Retriever_NativeIndexConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::Retriever.NativeIndexConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_Retriever_NativeIndexConfiguration as NativeIndexConfiguration;
    impl crate::value::ToValue for NativeIndexConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IndexId".to_string(),
                crate::value::ToValue::to_value(&self.index_id),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-retriever-retrieverconfiguration.html>
    pub struct RetrieverConfiguration_ {
        pub kendra_index_configuration: Option<Box<KendraIndexConfiguration_>>,
        pub native_index_configuration: Option<Box<NativeIndexConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_Retriever_RetrieverConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::Retriever.RetrieverConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_Retriever_RetrieverConfiguration as RetrieverConfiguration;
    impl crate::value::ToValue for RetrieverConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kendra_index_configuration {
                properties.insert(
                    "KendraIndexConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.native_index_configuration {
                properties.insert(
                    "NativeIndexConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod webexperience {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-webexperience-browserextensionconfiguration.html>
    pub struct BrowserExtensionConfiguration_ {
        pub enabled_browser_extensions: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_WebExperience_BrowserExtensionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::WebExperience.BrowserExtensionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_WebExperience_BrowserExtensionConfiguration as BrowserExtensionConfiguration;
    impl crate::value::ToValue for BrowserExtensionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EnabledBrowserExtensions".to_string(),
                crate::value::ToValue::to_value(&self.enabled_browser_extensions),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-webexperience-customizationconfiguration.html>
    pub struct CustomizationConfiguration_ {
        pub custom_css_url: Option<crate::value::ExpString>,
        pub favicon_url: Option<crate::value::ExpString>,
        pub font_url: Option<crate::value::ExpString>,
        pub logo_url: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_WebExperience_CustomizationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::WebExperience.CustomizationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_WebExperience_CustomizationConfiguration as CustomizationConfiguration;
    impl crate::value::ToValue for CustomizationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_css_url {
                properties.insert(
                    "CustomCSSUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.favicon_url {
                properties.insert(
                    "FaviconUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.font_url {
                properties.insert(
                    "FontUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.logo_url {
                properties.insert(
                    "LogoUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-webexperience-identityproviderconfiguration.html>
    pub struct IdentityProviderConfiguration_ {
        pub open_id_connect_configuration: Option<Box<OpenIDConnectProviderConfiguration_>>,
        pub saml_configuration: Option<Box<SamlProviderConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_WebExperience_IdentityProviderConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::WebExperience.IdentityProviderConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_WebExperience_IdentityProviderConfiguration as IdentityProviderConfiguration;
    impl crate::value::ToValue for IdentityProviderConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.open_id_connect_configuration {
                properties.insert(
                    "OpenIDConnectConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.saml_configuration {
                properties.insert(
                    "SamlConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-webexperience-openidconnectproviderconfiguration.html>
    pub struct OpenIDConnectProviderConfiguration_ {
        pub secrets_arn: crate::value::ExpString,
        pub secrets_role: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_WebExperience_OpenIDConnectProviderConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::WebExperience.OpenIDConnectProviderConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_WebExperience_OpenIDConnectProviderConfiguration as OpenIDConnectProviderConfiguration;
    impl crate::value::ToValue for OpenIDConnectProviderConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SecretsArn".to_string(),
                crate::value::ToValue::to_value(&self.secrets_arn),
            );
            properties.insert(
                "SecretsRole".to_string(),
                crate::value::ToValue::to_value(&self.secrets_role),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qbusiness-webexperience-samlproviderconfiguration.html>
    pub struct SamlProviderConfiguration_ {
        pub authentication_url: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_qbusiness_WebExperience_SamlProviderConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::QBusiness::WebExperience.SamlProviderConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_qbusiness_WebExperience_SamlProviderConfiguration as SamlProviderConfiguration;
    impl crate::value::ToValue for SamlProviderConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AuthenticationUrl".to_string(),
                crate::value::ToValue::to_value(&self.authentication_url),
            );
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-qbusiness-application.html>
pub struct Application_ {
    pub attachments_configuration: Option<super::qbusiness::application::AttachmentsConfiguration_>,
    pub auto_subscription_configuration:
        Option<super::qbusiness::application::AutoSubscriptionConfiguration_>,
    pub client_ids_for_oidc: Option<Vec<crate::value::ExpString>>,
    pub description: Option<crate::value::ExpString>,
    pub display_name: crate::value::ExpString,
    pub encryption_configuration: Option<super::qbusiness::application::EncryptionConfiguration_>,
    pub iam_identity_provider_arn: Option<crate::value::ExpString>,
    pub identity_center_instance_arn: Option<crate::value::ExpString>,
    pub identity_type: Option<crate::value::ExpString>,
    pub personalization_configuration:
        Option<super::qbusiness::application::PersonalizationConfiguration_>,
    pub q_apps_configuration: Option<super::qbusiness::application::QAppsConfiguration_>,
    pub quick_sight_configuration: Option<super::qbusiness::application::QuickSightConfiguration_>,
    pub role_arn: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_qbusiness_Application {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::QBusiness::Application"
        $($field $value)*)
    };
}
pub use crate::__aws_qbusiness_Application as Application;
impl crate::template::ToResource for Application_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("QBusiness"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Application"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.attachments_configuration {
            properties.insert(
                "AttachmentsConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.auto_subscription_configuration {
            properties.insert(
                "AutoSubscriptionConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.client_ids_for_oidc {
            properties.insert(
                "ClientIdsForOIDC".to_string(),
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
        if let Some(ref value) = self.encryption_configuration {
            properties.insert(
                "EncryptionConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.iam_identity_provider_arn {
            properties.insert(
                "IamIdentityProviderArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.identity_center_instance_arn {
            properties.insert(
                "IdentityCenterInstanceArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.identity_type {
            properties.insert(
                "IdentityType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.personalization_configuration {
            properties.insert(
                "PersonalizationConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.q_apps_configuration {
            properties.insert(
                "QAppsConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.quick_sight_configuration {
            properties.insert(
                "QuickSightConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.role_arn {
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-qbusiness-dataaccessor.html>
pub struct DataAccessor_ {
    pub action_configurations: Vec<super::qbusiness::dataaccessor::ActionConfiguration_>,
    pub application_id: crate::value::ExpString,
    pub authentication_detail:
        Option<super::qbusiness::dataaccessor::DataAccessorAuthenticationDetail_>,
    pub display_name: crate::value::ExpString,
    pub principal: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_qbusiness_DataAccessor {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::QBusiness::DataAccessor"
        $($field $value)*)
    };
}
pub use crate::__aws_qbusiness_DataAccessor as DataAccessor;
impl crate::template::ToResource for DataAccessor_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("QBusiness"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DataAccessor"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ActionConfigurations".to_string(),
            crate::value::ToValue::to_value(&self.action_configurations),
        );
        properties.insert(
            "ApplicationId".to_string(),
            crate::value::ToValue::to_value(&self.application_id),
        );
        if let Some(ref value) = self.authentication_detail {
            properties.insert(
                "AuthenticationDetail".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DisplayName".to_string(),
            crate::value::ToValue::to_value(&self.display_name),
        );
        properties.insert(
            "Principal".to_string(),
            crate::value::ToValue::to_value(&self.principal),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-qbusiness-datasource.html>
pub struct DataSource_ {
    pub application_id: crate::value::ExpString,
    pub configuration: serde_json::Value,
    pub description: Option<crate::value::ExpString>,
    pub display_name: crate::value::ExpString,
    pub document_enrichment_configuration:
        Option<super::qbusiness::datasource::DocumentEnrichmentConfiguration_>,
    pub index_id: crate::value::ExpString,
    pub media_extraction_configuration:
        Option<super::qbusiness::datasource::MediaExtractionConfiguration_>,
    pub role_arn: Option<crate::value::ExpString>,
    pub sync_schedule: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_configuration: Option<super::qbusiness::datasource::DataSourceVpcConfiguration_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_qbusiness_DataSource {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::QBusiness::DataSource"
        $($field $value)*)
    };
}
pub use crate::__aws_qbusiness_DataSource as DataSource;
impl crate::template::ToResource for DataSource_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("QBusiness"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DataSource"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApplicationId".to_string(),
            crate::value::ToValue::to_value(&self.application_id),
        );
        properties.insert(
            "Configuration".to_string(),
            crate::value::ToValue::to_value(&self.configuration),
        );
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
        if let Some(ref value) = self.document_enrichment_configuration {
            properties.insert(
                "DocumentEnrichmentConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "IndexId".to_string(),
            crate::value::ToValue::to_value(&self.index_id),
        );
        if let Some(ref value) = self.media_extraction_configuration {
            properties.insert(
                "MediaExtractionConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.role_arn {
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sync_schedule {
            properties.insert(
                "SyncSchedule".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.vpc_configuration {
            properties.insert(
                "VpcConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-qbusiness-index.html>
pub struct Index_ {
    pub application_id: crate::value::ExpString,
    pub capacity_configuration: Option<super::qbusiness::index::IndexCapacityConfiguration_>,
    pub description: Option<crate::value::ExpString>,
    pub display_name: crate::value::ExpString,
    pub document_attribute_configurations:
        Option<Vec<super::qbusiness::index::DocumentAttributeConfiguration_>>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub r#type: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_qbusiness_Index {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::QBusiness::Index"
        $($field $value)*)
    };
}
pub use crate::__aws_qbusiness_Index as Index;
impl crate::template::ToResource for Index_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("QBusiness"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Index"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApplicationId".to_string(),
            crate::value::ToValue::to_value(&self.application_id),
        );
        if let Some(ref value) = self.capacity_configuration {
            properties.insert(
                "CapacityConfiguration".to_string(),
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
        if let Some(ref value) = self.document_attribute_configurations {
            properties.insert(
                "DocumentAttributeConfigurations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.r#type {
            properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-qbusiness-permission.html>
pub struct Permission_ {
    pub actions: Vec<crate::value::ExpString>,
    pub application_id: crate::value::ExpString,
    pub conditions: Option<Vec<super::qbusiness::permission::Condition_>>,
    pub principal: crate::value::ExpString,
    pub statement_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_qbusiness_Permission {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::QBusiness::Permission"
        $($field $value)*)
    };
}
pub use crate::__aws_qbusiness_Permission as Permission;
impl crate::template::ToResource for Permission_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("QBusiness"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Permission"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Actions".to_string(),
            crate::value::ToValue::to_value(&self.actions),
        );
        properties.insert(
            "ApplicationId".to_string(),
            crate::value::ToValue::to_value(&self.application_id),
        );
        if let Some(ref value) = self.conditions {
            properties.insert(
                "Conditions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Principal".to_string(),
            crate::value::ToValue::to_value(&self.principal),
        );
        properties.insert(
            "StatementId".to_string(),
            crate::value::ToValue::to_value(&self.statement_id),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-qbusiness-plugin.html>
pub struct Plugin_ {
    pub application_id: Option<crate::value::ExpString>,
    pub auth_configuration: super::qbusiness::plugin::PluginAuthConfiguration_,
    pub custom_plugin_configuration: Option<super::qbusiness::plugin::CustomPluginConfiguration_>,
    pub display_name: crate::value::ExpString,
    pub server_url: Option<crate::value::ExpString>,
    pub state: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_qbusiness_Plugin {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::QBusiness::Plugin"
        $($field $value)*)
    };
}
pub use crate::__aws_qbusiness_Plugin as Plugin;
impl crate::template::ToResource for Plugin_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("QBusiness"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Plugin"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.application_id {
            properties.insert(
                "ApplicationId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "AuthConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.auth_configuration),
        );
        if let Some(ref value) = self.custom_plugin_configuration {
            properties.insert(
                "CustomPluginConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DisplayName".to_string(),
            crate::value::ToValue::to_value(&self.display_name),
        );
        if let Some(ref value) = self.server_url {
            properties.insert(
                "ServerUrl".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.state {
            properties.insert("State".to_string(), crate::value::ToValue::to_value(value));
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
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-qbusiness-retriever.html>
pub struct Retriever_ {
    pub application_id: crate::value::ExpString,
    pub configuration: super::qbusiness::retriever::RetrieverConfiguration_,
    pub display_name: crate::value::ExpString,
    pub role_arn: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_qbusiness_Retriever {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::QBusiness::Retriever"
        $($field $value)*)
    };
}
pub use crate::__aws_qbusiness_Retriever as Retriever;
impl crate::template::ToResource for Retriever_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("QBusiness"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Retriever"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApplicationId".to_string(),
            crate::value::ToValue::to_value(&self.application_id),
        );
        properties.insert(
            "Configuration".to_string(),
            crate::value::ToValue::to_value(&self.configuration),
        );
        properties.insert(
            "DisplayName".to_string(),
            crate::value::ToValue::to_value(&self.display_name),
        );
        if let Some(ref value) = self.role_arn {
            properties.insert(
                "RoleArn".to_string(),
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
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-qbusiness-webexperience.html>
pub struct WebExperience_ {
    pub application_id: crate::value::ExpString,
    pub browser_extension_configuration:
        Option<super::qbusiness::webexperience::BrowserExtensionConfiguration_>,
    pub customization_configuration:
        Option<super::qbusiness::webexperience::CustomizationConfiguration_>,
    pub identity_provider_configuration:
        Option<super::qbusiness::webexperience::IdentityProviderConfiguration_>,
    pub origins: Option<Vec<crate::value::ExpString>>,
    pub role_arn: Option<crate::value::ExpString>,
    pub sample_prompts_control_mode: Option<crate::value::ExpString>,
    pub subtitle: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub title: Option<crate::value::ExpString>,
    pub welcome_message: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_qbusiness_WebExperience {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::QBusiness::WebExperience"
        $($field $value)*)
    };
}
pub use crate::__aws_qbusiness_WebExperience as WebExperience;
impl crate::template::ToResource for WebExperience_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("QBusiness"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("WebExperience"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApplicationId".to_string(),
            crate::value::ToValue::to_value(&self.application_id),
        );
        if let Some(ref value) = self.browser_extension_configuration {
            properties.insert(
                "BrowserExtensionConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.customization_configuration {
            properties.insert(
                "CustomizationConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.identity_provider_configuration {
            properties.insert(
                "IdentityProviderConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.origins {
            properties.insert(
                "Origins".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.role_arn {
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sample_prompts_control_mode {
            properties.insert(
                "SamplePromptsControlMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.subtitle {
            properties.insert(
                "Subtitle".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.title {
            properties.insert("Title".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.welcome_message {
            properties.insert(
                "WelcomeMessage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
