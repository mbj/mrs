pub mod allowlist {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-macie-allowlist-criteria.html
    pub struct Criteria_ {
        pub regex: Option<crate::value::ExpString>,
        pub s3_words_list: Option<Box<S3WordsList_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_macie_AllowList_Criteria {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Macie::AllowList.Criteria"
            $($field $value)*)
        };
    }
    pub use crate::__aws_macie_AllowList_Criteria as Criteria;
    impl crate::value::ToValue for Criteria_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.regex {
                properties.insert("Regex".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.s3_words_list {
                properties.insert(
                    "S3WordsList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-macie-allowlist-s3wordslist.html
    pub struct S3WordsList_ {
        pub bucket_name: crate::value::ExpString,
        pub object_key: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_macie_AllowList_S3WordsList {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Macie::AllowList.S3WordsList"
            $($field $value)*)
        };
    }
    pub use crate::__aws_macie_AllowList_S3WordsList as S3WordsList;
    impl crate::value::ToValue for S3WordsList_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BucketName".to_string(),
                crate::value::ToValue::to_value(&self.bucket_name),
            );
            properties.insert(
                "ObjectKey".to_string(),
                crate::value::ToValue::to_value(&self.object_key),
            );
            properties.into()
        }
    }
}
pub mod findingsfilter {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-macie-findingsfilter-criterionadditionalproperties.html
    pub struct CriterionAdditionalProperties_ {
        pub eq: Option<Vec<crate::value::ExpString>>,
        pub gt: Option<i32>,
        pub gte: Option<i32>,
        pub lt: Option<i32>,
        pub lte: Option<i32>,
        pub neq: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_macie_FindingsFilter_CriterionAdditionalProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Macie::FindingsFilter.CriterionAdditionalProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_macie_FindingsFilter_CriterionAdditionalProperties as CriterionAdditionalProperties;
    impl crate::value::ToValue for CriterionAdditionalProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.eq {
                properties.insert("eq".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.gt {
                properties.insert("gt".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.gte {
                properties.insert("gte".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.lt {
                properties.insert("lt".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.lte {
                properties.insert("lte".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.neq {
                properties.insert("neq".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-macie-findingsfilter-findingcriteria.html
    pub struct FindingCriteria_ {
        pub criterion: Option<std::collections::BTreeMap<String, CriterionAdditionalProperties_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_macie_FindingsFilter_FindingCriteria {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Macie::FindingsFilter.FindingCriteria"
            $($field $value)*)
        };
    }
    pub use crate::__aws_macie_FindingsFilter_FindingCriteria as FindingCriteria;
    impl crate::value::ToValue for FindingCriteria_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.criterion {
                properties.insert(
                    "Criterion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-macie-allowlist.html
pub struct AllowList_ {
    pub criteria: super::macie::allowlist::Criteria_,
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_macie_AllowList {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Macie::AllowList"
        $($field $value)*)
    };
}
pub use crate::__aws_macie_AllowList as AllowList;
impl crate::template::ToResource for AllowList_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Macie"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AllowList"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Criteria".to_string(),
            crate::value::ToValue::to_value(&self.criteria),
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
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-macie-customdataidentifier.html
pub struct CustomDataIdentifier_ {
    pub description: Option<crate::value::ExpString>,
    pub ignore_words: Option<Vec<crate::value::ExpString>>,
    pub keywords: Option<Vec<crate::value::ExpString>>,
    pub maximum_match_distance: Option<i32>,
    pub name: crate::value::ExpString,
    pub regex: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_macie_CustomDataIdentifier {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Macie::CustomDataIdentifier"
        $($field $value)*)
    };
}
pub use crate::__aws_macie_CustomDataIdentifier as CustomDataIdentifier;
impl crate::template::ToResource for CustomDataIdentifier_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Macie"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("CustomDataIdentifier"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ignore_words {
            properties.insert(
                "IgnoreWords".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.keywords {
            properties.insert(
                "Keywords".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.maximum_match_distance {
            properties.insert(
                "MaximumMatchDistance".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "Regex".to_string(),
            crate::value::ToValue::to_value(&self.regex),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-macie-findingsfilter.html
pub struct FindingsFilter_ {
    pub action: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub finding_criteria: super::macie::findingsfilter::FindingCriteria_,
    pub name: crate::value::ExpString,
    pub position: Option<i32>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_macie_FindingsFilter {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Macie::FindingsFilter"
        $($field $value)*)
    };
}
pub use crate::__aws_macie_FindingsFilter as FindingsFilter;
impl crate::template::ToResource for FindingsFilter_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Macie"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("FindingsFilter"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.action {
            properties.insert("Action".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "FindingCriteria".to_string(),
            crate::value::ToValue::to_value(&self.finding_criteria),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.position {
            properties.insert(
                "Position".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-macie-session.html
pub struct Session_ {
    pub finding_publishing_frequency: Option<crate::value::ExpString>,
    pub status: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_macie_Session {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Macie::Session" $($field
        $value)*)
    };
}
pub use crate::__aws_macie_Session as Session;
impl crate::template::ToResource for Session_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Macie"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Session"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.finding_publishing_frequency {
            properties.insert(
                "FindingPublishingFrequency".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.status {
            properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
