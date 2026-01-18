pub mod dataprotectionsettings {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesweb-dataprotectionsettings-custompattern.html
    pub struct CustomPattern_ {
        pub keyword_regex: Option<crate::value::ExpString>,
        pub pattern_description: Option<crate::value::ExpString>,
        pub pattern_name: crate::value::ExpString,
        pub pattern_regex: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspacesweb_DataProtectionSettings_CustomPattern {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WorkSpacesWeb::DataProtectionSettings.CustomPattern"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspacesweb_DataProtectionSettings_CustomPattern as CustomPattern;
    impl crate::value::ToValue for CustomPattern_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.keyword_regex {
                properties.insert(
                    "KeywordRegex".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pattern_description {
                properties.insert(
                    "PatternDescription".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "PatternName".to_string(),
                crate::value::ToValue::to_value(&self.pattern_name),
            );
            properties.insert(
                "PatternRegex".to_string(),
                crate::value::ToValue::to_value(&self.pattern_regex),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesweb-dataprotectionsettings-inlineredactionconfiguration.html
    pub struct InlineRedactionConfiguration_ {
        pub global_confidence_level: Option<f64>,
        pub global_enforced_urls: Option<Vec<crate::value::ExpString>>,
        pub global_exempt_urls: Option<Vec<crate::value::ExpString>>,
        pub inline_redaction_patterns: Vec<InlineRedactionPattern_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspacesweb_DataProtectionSettings_InlineRedactionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WorkSpacesWeb::DataProtectionSettings.InlineRedactionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspacesweb_DataProtectionSettings_InlineRedactionConfiguration as InlineRedactionConfiguration;
    impl crate::value::ToValue for InlineRedactionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.global_confidence_level {
                properties.insert(
                    "GlobalConfidenceLevel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.global_enforced_urls {
                properties.insert(
                    "GlobalEnforcedUrls".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.global_exempt_urls {
                properties.insert(
                    "GlobalExemptUrls".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "InlineRedactionPatterns".to_string(),
                crate::value::ToValue::to_value(&self.inline_redaction_patterns),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesweb-dataprotectionsettings-inlineredactionpattern.html
    pub struct InlineRedactionPattern_ {
        pub built_in_pattern_id: Option<crate::value::ExpString>,
        pub confidence_level: Option<f64>,
        pub custom_pattern: Option<Box<CustomPattern_>>,
        pub enforced_urls: Option<Vec<crate::value::ExpString>>,
        pub exempt_urls: Option<Vec<crate::value::ExpString>>,
        pub redaction_place_holder: Box<RedactionPlaceHolder_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspacesweb_DataProtectionSettings_InlineRedactionPattern {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WorkSpacesWeb::DataProtectionSettings.InlineRedactionPattern"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspacesweb_DataProtectionSettings_InlineRedactionPattern as InlineRedactionPattern;
    impl crate::value::ToValue for InlineRedactionPattern_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.built_in_pattern_id {
                properties.insert(
                    "BuiltInPatternId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.confidence_level {
                properties.insert(
                    "ConfidenceLevel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_pattern {
                properties.insert(
                    "CustomPattern".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enforced_urls {
                properties.insert(
                    "EnforcedUrls".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exempt_urls {
                properties.insert(
                    "ExemptUrls".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RedactionPlaceHolder".to_string(),
                crate::value::ToValue::to_value(&self.redaction_place_holder),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesweb-dataprotectionsettings-redactionplaceholder.html
    pub struct RedactionPlaceHolder_ {
        pub redaction_place_holder_text: Option<crate::value::ExpString>,
        pub redaction_place_holder_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspacesweb_DataProtectionSettings_RedactionPlaceHolder {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WorkSpacesWeb::DataProtectionSettings.RedactionPlaceHolder"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspacesweb_DataProtectionSettings_RedactionPlaceHolder as RedactionPlaceHolder;
    impl crate::value::ToValue for RedactionPlaceHolder_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.redaction_place_holder_text {
                properties.insert(
                    "RedactionPlaceHolderText".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RedactionPlaceHolderType".to_string(),
                crate::value::ToValue::to_value(&self.redaction_place_holder_type),
            );
            properties.into()
        }
    }
}
pub mod ipaccesssettings {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesweb-ipaccesssettings-iprule.html
    pub struct IpRule_ {
        pub description: Option<crate::value::ExpString>,
        pub ip_range: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspacesweb_IpAccessSettings_IpRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WorkSpacesWeb::IpAccessSettings.IpRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspacesweb_IpAccessSettings_IpRule as IpRule;
    impl crate::value::ToValue for IpRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "IpRange".to_string(),
                crate::value::ToValue::to_value(&self.ip_range),
            );
            properties.into()
        }
    }
}
pub mod sessionlogger {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesweb-sessionlogger-eventfilter.html
    pub struct EventFilter_ {
        pub all: Option<serde_json::Value>,
        pub include: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspacesweb_SessionLogger_EventFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WorkSpacesWeb::SessionLogger.EventFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspacesweb_SessionLogger_EventFilter as EventFilter;
    impl crate::value::ToValue for EventFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.all {
                properties.insert("All".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.include {
                properties.insert(
                    "Include".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesweb-sessionlogger-logconfiguration.html
    pub struct LogConfiguration_ {
        pub s3: Option<Box<S3LogConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspacesweb_SessionLogger_LogConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WorkSpacesWeb::SessionLogger.LogConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspacesweb_SessionLogger_LogConfiguration as LogConfiguration;
    impl crate::value::ToValue for LogConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.s3 {
                properties.insert("S3".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesweb-sessionlogger-s3logconfiguration.html
    pub struct S3LogConfiguration_ {
        pub bucket: crate::value::ExpString,
        pub bucket_owner: Option<crate::value::ExpString>,
        pub folder_structure: crate::value::ExpString,
        pub key_prefix: Option<crate::value::ExpString>,
        pub log_file_format: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspacesweb_SessionLogger_S3LogConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WorkSpacesWeb::SessionLogger.S3LogConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspacesweb_SessionLogger_S3LogConfiguration as S3LogConfiguration;
    impl crate::value::ToValue for S3LogConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Bucket".to_string(),
                crate::value::ToValue::to_value(&self.bucket),
            );
            if let Some(ref value) = self.bucket_owner {
                properties.insert(
                    "BucketOwner".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "FolderStructure".to_string(),
                crate::value::ToValue::to_value(&self.folder_structure),
            );
            if let Some(ref value) = self.key_prefix {
                properties.insert(
                    "KeyPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "LogFileFormat".to_string(),
                crate::value::ToValue::to_value(&self.log_file_format),
            );
            properties.into()
        }
    }
}
pub mod usersettings {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesweb-usersettings-cookiespecification.html
    pub struct CookieSpecification_ {
        pub domain: crate::value::ExpString,
        pub name: Option<crate::value::ExpString>,
        pub path: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspacesweb_UserSettings_CookieSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WorkSpacesWeb::UserSettings.CookieSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspacesweb_UserSettings_CookieSpecification as CookieSpecification;
    impl crate::value::ToValue for CookieSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Domain".to_string(),
                crate::value::ToValue::to_value(&self.domain),
            );
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.path {
                properties.insert("Path".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesweb-usersettings-cookiesynchronizationconfiguration.html
    pub struct CookieSynchronizationConfiguration_ {
        pub allowlist: Vec<CookieSpecification_>,
        pub blocklist: Option<Vec<CookieSpecification_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspacesweb_UserSettings_CookieSynchronizationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WorkSpacesWeb::UserSettings.CookieSynchronizationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspacesweb_UserSettings_CookieSynchronizationConfiguration as CookieSynchronizationConfiguration;
    impl crate::value::ToValue for CookieSynchronizationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Allowlist".to_string(),
                crate::value::ToValue::to_value(&self.allowlist),
            );
            if let Some(ref value) = self.blocklist {
                properties.insert(
                    "Blocklist".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesweb-usersettings-toolbarconfiguration.html
    pub struct ToolbarConfiguration_ {
        pub hidden_toolbar_items: Option<Vec<crate::value::ExpString>>,
        pub max_display_resolution: Option<crate::value::ExpString>,
        pub toolbar_type: Option<crate::value::ExpString>,
        pub visual_mode: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspacesweb_UserSettings_ToolbarConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WorkSpacesWeb::UserSettings.ToolbarConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspacesweb_UserSettings_ToolbarConfiguration as ToolbarConfiguration;
    impl crate::value::ToValue for ToolbarConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.hidden_toolbar_items {
                properties.insert(
                    "HiddenToolbarItems".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_display_resolution {
                properties.insert(
                    "MaxDisplayResolution".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.toolbar_type {
                properties.insert(
                    "ToolbarType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.visual_mode {
                properties.insert(
                    "VisualMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-browsersettings.html
pub struct BrowserSettings_ {
    pub additional_encryption_context:
        Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub browser_policy: Option<crate::value::ExpString>,
    pub customer_managed_key: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_workspacesweb_BrowserSettings {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::WorkSpacesWeb::BrowserSettings"
        $($field $value)*)
    };
}
pub use crate::__aws_workspacesweb_BrowserSettings as BrowserSettings;
impl crate::template::ToResource for BrowserSettings_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WorkSpacesWeb"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("BrowserSettings"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.additional_encryption_context {
            properties.insert(
                "AdditionalEncryptionContext".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.browser_policy {
            properties.insert(
                "BrowserPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.customer_managed_key {
            properties.insert(
                "CustomerManagedKey".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-dataprotectionsettings.html
pub struct DataProtectionSettings_ {
    pub additional_encryption_context:
        Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub customer_managed_key: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub display_name: Option<crate::value::ExpString>,
    pub inline_redaction_configuration:
        Option<super::workspacesweb::dataprotectionsettings::InlineRedactionConfiguration_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_workspacesweb_DataProtectionSettings {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::WorkSpacesWeb::DataProtectionSettings"
        $($field $value)*)
    };
}
pub use crate::__aws_workspacesweb_DataProtectionSettings as DataProtectionSettings;
impl crate::template::ToResource for DataProtectionSettings_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WorkSpacesWeb"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DataProtectionSettings"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.additional_encryption_context {
            properties.insert(
                "AdditionalEncryptionContext".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.customer_managed_key {
            properties.insert(
                "CustomerManagedKey".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.display_name {
            properties.insert(
                "DisplayName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.inline_redaction_configuration {
            properties.insert(
                "InlineRedactionConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-identityprovider.html
pub struct IdentityProvider_ {
    pub identity_provider_details: std::collections::BTreeMap<String, crate::value::ExpString>,
    pub identity_provider_name: crate::value::ExpString,
    pub identity_provider_type: crate::value::ExpString,
    pub portal_arn: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_workspacesweb_IdentityProvider {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::WorkSpacesWeb::IdentityProvider"
        $($field $value)*)
    };
}
pub use crate::__aws_workspacesweb_IdentityProvider as IdentityProvider;
impl crate::template::ToResource for IdentityProvider_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WorkSpacesWeb"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("IdentityProvider"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "IdentityProviderDetails".to_string(),
            crate::value::ToValue::to_value(&self.identity_provider_details),
        );
        properties.insert(
            "IdentityProviderName".to_string(),
            crate::value::ToValue::to_value(&self.identity_provider_name),
        );
        properties.insert(
            "IdentityProviderType".to_string(),
            crate::value::ToValue::to_value(&self.identity_provider_type),
        );
        if let Some(ref value) = self.portal_arn {
            properties.insert(
                "PortalArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-ipaccesssettings.html
pub struct IpAccessSettings_ {
    pub additional_encryption_context:
        Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub customer_managed_key: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub display_name: Option<crate::value::ExpString>,
    pub ip_rules: Vec<super::workspacesweb::ipaccesssettings::IpRule_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_workspacesweb_IpAccessSettings {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::WorkSpacesWeb::IpAccessSettings"
        $($field $value)*)
    };
}
pub use crate::__aws_workspacesweb_IpAccessSettings as IpAccessSettings;
impl crate::template::ToResource for IpAccessSettings_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WorkSpacesWeb"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("IpAccessSettings"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.additional_encryption_context {
            properties.insert(
                "AdditionalEncryptionContext".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.customer_managed_key {
            properties.insert(
                "CustomerManagedKey".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.display_name {
            properties.insert(
                "DisplayName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "IpRules".to_string(),
            crate::value::ToValue::to_value(&self.ip_rules),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-networksettings.html
pub struct NetworkSettings_ {
    pub security_group_ids: Vec<crate::value::ExpString>,
    pub subnet_ids: Vec<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_workspacesweb_NetworkSettings {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::WorkSpacesWeb::NetworkSettings"
        $($field $value)*)
    };
}
pub use crate::__aws_workspacesweb_NetworkSettings as NetworkSettings;
impl crate::template::ToResource for NetworkSettings_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WorkSpacesWeb"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("NetworkSettings"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "SecurityGroupIds".to_string(),
            crate::value::ToValue::to_value(&self.security_group_ids),
        );
        properties.insert(
            "SubnetIds".to_string(),
            crate::value::ToValue::to_value(&self.subnet_ids),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "VpcId".to_string(),
            crate::value::ToValue::to_value(&self.vpc_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-portal.html
pub struct Portal_ {
    pub additional_encryption_context:
        Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub authentication_type: Option<crate::value::ExpString>,
    pub browser_settings_arn: Option<crate::value::ExpString>,
    pub customer_managed_key: Option<crate::value::ExpString>,
    pub data_protection_settings_arn: Option<crate::value::ExpString>,
    pub display_name: Option<crate::value::ExpString>,
    pub instance_type: Option<crate::value::ExpString>,
    pub ip_access_settings_arn: Option<crate::value::ExpString>,
    pub max_concurrent_sessions: Option<f64>,
    pub network_settings_arn: Option<crate::value::ExpString>,
    pub session_logger_arn: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub trust_store_arn: Option<crate::value::ExpString>,
    pub user_access_logging_settings_arn: Option<crate::value::ExpString>,
    pub user_settings_arn: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_workspacesweb_Portal {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::WorkSpacesWeb::Portal"
        $($field $value)*)
    };
}
pub use crate::__aws_workspacesweb_Portal as Portal;
impl crate::template::ToResource for Portal_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WorkSpacesWeb"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Portal"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.additional_encryption_context {
            properties.insert(
                "AdditionalEncryptionContext".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.authentication_type {
            properties.insert(
                "AuthenticationType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.browser_settings_arn {
            properties.insert(
                "BrowserSettingsArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.customer_managed_key {
            properties.insert(
                "CustomerManagedKey".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.data_protection_settings_arn {
            properties.insert(
                "DataProtectionSettingsArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.display_name {
            properties.insert(
                "DisplayName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.instance_type {
            properties.insert(
                "InstanceType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ip_access_settings_arn {
            properties.insert(
                "IpAccessSettingsArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.max_concurrent_sessions {
            properties.insert(
                "MaxConcurrentSessions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.network_settings_arn {
            properties.insert(
                "NetworkSettingsArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.session_logger_arn {
            properties.insert(
                "SessionLoggerArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.trust_store_arn {
            properties.insert(
                "TrustStoreArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.user_access_logging_settings_arn {
            properties.insert(
                "UserAccessLoggingSettingsArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.user_settings_arn {
            properties.insert(
                "UserSettingsArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-sessionlogger.html
pub struct SessionLogger_ {
    pub additional_encryption_context:
        Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub customer_managed_key: Option<crate::value::ExpString>,
    pub display_name: Option<crate::value::ExpString>,
    pub event_filter: super::workspacesweb::sessionlogger::EventFilter_,
    pub log_configuration: super::workspacesweb::sessionlogger::LogConfiguration_,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_workspacesweb_SessionLogger {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::WorkSpacesWeb::SessionLogger"
        $($field $value)*)
    };
}
pub use crate::__aws_workspacesweb_SessionLogger as SessionLogger;
impl crate::template::ToResource for SessionLogger_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WorkSpacesWeb"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SessionLogger"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.additional_encryption_context {
            properties.insert(
                "AdditionalEncryptionContext".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.customer_managed_key {
            properties.insert(
                "CustomerManagedKey".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.display_name {
            properties.insert(
                "DisplayName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "EventFilter".to_string(),
            crate::value::ToValue::to_value(&self.event_filter),
        );
        properties.insert(
            "LogConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.log_configuration),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-truststore.html
pub struct TrustStore_ {
    pub certificate_list: Vec<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_workspacesweb_TrustStore {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::WorkSpacesWeb::TrustStore"
        $($field $value)*)
    };
}
pub use crate::__aws_workspacesweb_TrustStore as TrustStore;
impl crate::template::ToResource for TrustStore_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WorkSpacesWeb"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TrustStore"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "CertificateList".to_string(),
            crate::value::ToValue::to_value(&self.certificate_list),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-useraccessloggingsettings.html
pub struct UserAccessLoggingSettings_ {
    pub kinesis_stream_arn: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_workspacesweb_UserAccessLoggingSettings {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::WorkSpacesWeb::UserAccessLoggingSettings"
        $($field $value)*)
    };
}
pub use crate::__aws_workspacesweb_UserAccessLoggingSettings as UserAccessLoggingSettings;
impl crate::template::ToResource for UserAccessLoggingSettings_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WorkSpacesWeb"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("UserAccessLoggingSettings"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "KinesisStreamArn".to_string(),
            crate::value::ToValue::to_value(&self.kinesis_stream_arn),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-usersettings.html
pub struct UserSettings_ {
    pub additional_encryption_context:
        Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub cookie_synchronization_configuration:
        Option<super::workspacesweb::usersettings::CookieSynchronizationConfiguration_>,
    pub copy_allowed: crate::value::ExpString,
    pub customer_managed_key: Option<crate::value::ExpString>,
    pub deep_link_allowed: Option<crate::value::ExpString>,
    pub disconnect_timeout_in_minutes: Option<f64>,
    pub download_allowed: crate::value::ExpString,
    pub idle_disconnect_timeout_in_minutes: Option<f64>,
    pub paste_allowed: crate::value::ExpString,
    pub print_allowed: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub toolbar_configuration: Option<super::workspacesweb::usersettings::ToolbarConfiguration_>,
    pub upload_allowed: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_workspacesweb_UserSettings {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::WorkSpacesWeb::UserSettings"
        $($field $value)*)
    };
}
pub use crate::__aws_workspacesweb_UserSettings as UserSettings;
impl crate::template::ToResource for UserSettings_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WorkSpacesWeb"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("UserSettings"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.additional_encryption_context {
            properties.insert(
                "AdditionalEncryptionContext".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cookie_synchronization_configuration {
            properties.insert(
                "CookieSynchronizationConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "CopyAllowed".to_string(),
            crate::value::ToValue::to_value(&self.copy_allowed),
        );
        if let Some(ref value) = self.customer_managed_key {
            properties.insert(
                "CustomerManagedKey".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.deep_link_allowed {
            properties.insert(
                "DeepLinkAllowed".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.disconnect_timeout_in_minutes {
            properties.insert(
                "DisconnectTimeoutInMinutes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DownloadAllowed".to_string(),
            crate::value::ToValue::to_value(&self.download_allowed),
        );
        if let Some(ref value) = self.idle_disconnect_timeout_in_minutes {
            properties.insert(
                "IdleDisconnectTimeoutInMinutes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "PasteAllowed".to_string(),
            crate::value::ToValue::to_value(&self.paste_allowed),
        );
        properties.insert(
            "PrintAllowed".to_string(),
            crate::value::ToValue::to_value(&self.print_allowed),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.toolbar_configuration {
            properties.insert(
                "ToolbarConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "UploadAllowed".to_string(),
            crate::value::ToValue::to_value(&self.upload_allowed),
        );
        properties
    }
}
