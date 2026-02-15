pub mod analyzer {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-accessanalyzer-analyzer-analysisrule.html
    pub struct AnalysisRule_ {
        pub exclusions: Option<Vec<AnalysisRuleCriteria_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_accessanalyzer_Analyzer_AnalysisRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AccessAnalyzer::Analyzer.AnalysisRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_accessanalyzer_Analyzer_AnalysisRule as AnalysisRule;
    impl crate::value::ToValue for AnalysisRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.exclusions {
                properties.insert(
                    "Exclusions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-accessanalyzer-analyzer-analysisrulecriteria.html
    pub struct AnalysisRuleCriteria_ {
        pub account_ids: Option<Vec<crate::value::ExpString>>,
        pub resource_tags: Option<serde_json::Value>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_accessanalyzer_Analyzer_AnalysisRuleCriteria {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AccessAnalyzer::Analyzer.AnalysisRuleCriteria"
            $($field $value)*)
        };
    }
    pub use crate::__aws_accessanalyzer_Analyzer_AnalysisRuleCriteria as AnalysisRuleCriteria;
    impl crate::value::ToValue for AnalysisRuleCriteria_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.account_ids {
                properties.insert(
                    "AccountIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_tags {
                properties.insert(
                    "ResourceTags".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-accessanalyzer-analyzer-analyzerconfiguration.html
    pub struct AnalyzerConfiguration_ {
        pub internal_access_configuration: Option<Box<InternalAccessConfiguration_>>,
        pub unused_access_configuration: Option<Box<UnusedAccessConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_accessanalyzer_Analyzer_AnalyzerConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AccessAnalyzer::Analyzer.AnalyzerConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_accessanalyzer_Analyzer_AnalyzerConfiguration as AnalyzerConfiguration;
    impl crate::value::ToValue for AnalyzerConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.internal_access_configuration {
                properties.insert(
                    "InternalAccessConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.unused_access_configuration {
                properties.insert(
                    "UnusedAccessConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-accessanalyzer-analyzer-archiverule.html
    pub struct ArchiveRule_ {
        pub filter: Vec<Filter_>,
        pub rule_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_accessanalyzer_Analyzer_ArchiveRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AccessAnalyzer::Analyzer.ArchiveRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_accessanalyzer_Analyzer_ArchiveRule as ArchiveRule;
    impl crate::value::ToValue for ArchiveRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Filter".to_string(),
                crate::value::ToValue::to_value(&self.filter),
            );
            properties.insert(
                "RuleName".to_string(),
                crate::value::ToValue::to_value(&self.rule_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-accessanalyzer-analyzer-filter.html
    pub struct Filter_ {
        pub contains: Option<Vec<crate::value::ExpString>>,
        pub eq: Option<Vec<crate::value::ExpString>>,
        pub exists: Option<crate::value::ExpBool>,
        pub neq: Option<Vec<crate::value::ExpString>>,
        pub property: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_accessanalyzer_Analyzer_Filter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AccessAnalyzer::Analyzer.Filter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_accessanalyzer_Analyzer_Filter as Filter;
    impl crate::value::ToValue for Filter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.contains {
                properties.insert(
                    "Contains".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.eq {
                properties.insert("Eq".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.exists {
                properties.insert("Exists".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.neq {
                properties.insert("Neq".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Property".to_string(),
                crate::value::ToValue::to_value(&self.property),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-accessanalyzer-analyzer-internalaccessanalysisrule.html
    pub struct InternalAccessAnalysisRule_ {
        pub inclusions: Option<Vec<InternalAccessAnalysisRuleCriteria_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_accessanalyzer_Analyzer_InternalAccessAnalysisRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AccessAnalyzer::Analyzer.InternalAccessAnalysisRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_accessanalyzer_Analyzer_InternalAccessAnalysisRule as InternalAccessAnalysisRule;
    impl crate::value::ToValue for InternalAccessAnalysisRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.inclusions {
                properties.insert(
                    "Inclusions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-accessanalyzer-analyzer-internalaccessanalysisrulecriteria.html
    pub struct InternalAccessAnalysisRuleCriteria_ {
        pub account_ids: Option<Vec<crate::value::ExpString>>,
        pub resource_arns: Option<Vec<crate::value::ExpString>>,
        pub resource_types: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_accessanalyzer_Analyzer_InternalAccessAnalysisRuleCriteria {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AccessAnalyzer::Analyzer.InternalAccessAnalysisRuleCriteria"
            $($field $value)*)
        };
    }
    pub use crate::__aws_accessanalyzer_Analyzer_InternalAccessAnalysisRuleCriteria as InternalAccessAnalysisRuleCriteria;
    impl crate::value::ToValue for InternalAccessAnalysisRuleCriteria_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.account_ids {
                properties.insert(
                    "AccountIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_arns {
                properties.insert(
                    "ResourceArns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_types {
                properties.insert(
                    "ResourceTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-accessanalyzer-analyzer-internalaccessconfiguration.html
    pub struct InternalAccessConfiguration_ {
        pub internal_access_analysis_rule: Option<Box<InternalAccessAnalysisRule_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_accessanalyzer_Analyzer_InternalAccessConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AccessAnalyzer::Analyzer.InternalAccessConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_accessanalyzer_Analyzer_InternalAccessConfiguration as InternalAccessConfiguration;
    impl crate::value::ToValue for InternalAccessConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.internal_access_analysis_rule {
                properties.insert(
                    "InternalAccessAnalysisRule".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-accessanalyzer-analyzer-unusedaccessconfiguration.html
    pub struct UnusedAccessConfiguration_ {
        pub analysis_rule: Option<Box<AnalysisRule_>>,
        pub unused_access_age: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_accessanalyzer_Analyzer_UnusedAccessConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AccessAnalyzer::Analyzer.UnusedAccessConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_accessanalyzer_Analyzer_UnusedAccessConfiguration as UnusedAccessConfiguration;
    impl crate::value::ToValue for UnusedAccessConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.analysis_rule {
                properties.insert(
                    "AnalysisRule".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.unused_access_age {
                properties.insert(
                    "UnusedAccessAge".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-accessanalyzer-analyzer.html
pub struct Analyzer_ {
    pub analyzer_configuration: Option<super::accessanalyzer::analyzer::AnalyzerConfiguration_>,
    pub analyzer_name: Option<crate::value::ExpString>,
    pub archive_rules: Option<Vec<super::accessanalyzer::analyzer::ArchiveRule_>>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_accessanalyzer_Analyzer {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::AccessAnalyzer::Analyzer"
        $($field $value)*)
    };
}
pub use crate::__aws_accessanalyzer_Analyzer as Analyzer;
impl crate::template::ToResource for Analyzer_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AccessAnalyzer"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Analyzer"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.analyzer_configuration {
            properties.insert(
                "AnalyzerConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.analyzer_name {
            properties.insert(
                "AnalyzerName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.archive_rules {
            properties.insert(
                "ArchiveRules".to_string(),
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
