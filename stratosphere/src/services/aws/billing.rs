pub mod billingview {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billing-billingview-datafilterexpression.html>
    pub struct DataFilterExpression_ {
        pub dimensions: Option<Box<Dimensions_>>,
        pub tags: Option<Box<Tags_>>,
        pub time_range: Option<Box<TimeRange_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_billing_BillingView_DataFilterExpression {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Billing::BillingView.DataFilterExpression"
            $($field $value)*)
        };
    }
    pub use crate::__aws_billing_BillingView_DataFilterExpression as DataFilterExpression;
    impl crate::value::ToValue for DataFilterExpression_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dimensions {
                properties.insert(
                    "Dimensions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tags {
                properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.time_range {
                properties.insert(
                    "TimeRange".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billing-billingview-dimensions.html>
    pub struct Dimensions_ {
        pub key: Option<crate::value::ExpString>,
        pub values: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_billing_BillingView_Dimensions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Billing::BillingView.Dimensions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_billing_BillingView_Dimensions as Dimensions;
    impl crate::value::ToValue for Dimensions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.key {
                properties.insert("Key".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.values {
                properties.insert("Values".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billing-billingview-tags.html>
    pub struct Tags_ {
        pub key: Option<crate::value::ExpString>,
        pub values: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_billing_BillingView_Tags {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Billing::BillingView.Tags"
            $($field $value)*)
        };
    }
    pub use crate::__aws_billing_BillingView_Tags as Tags;
    impl crate::value::ToValue for Tags_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.key {
                properties.insert("Key".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.values {
                properties.insert("Values".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billing-billingview-timerange.html>
    pub struct TimeRange_ {
        pub begin_date_inclusive: Option<crate::value::ExpString>,
        pub end_date_inclusive: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_billing_BillingView_TimeRange {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Billing::BillingView.TimeRange"
            $($field $value)*)
        };
    }
    pub use crate::__aws_billing_BillingView_TimeRange as TimeRange;
    impl crate::value::ToValue for TimeRange_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.begin_date_inclusive {
                properties.insert(
                    "BeginDateInclusive".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.end_date_inclusive {
                properties.insert(
                    "EndDateInclusive".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-billing-billingview.html>
pub struct BillingView_ {
    pub data_filter_expression: Option<super::billing::billingview::DataFilterExpression_>,
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub source_views: Vec<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_billing_BillingView {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Billing::BillingView"
        $($field $value)*)
    };
}
pub use crate::__aws_billing_BillingView as BillingView;
impl crate::template::ToResource for BillingView_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Billing"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("BillingView"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.data_filter_expression {
            properties.insert(
                "DataFilterExpression".to_string(),
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
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "SourceViews".to_string(),
            crate::value::ToValue::to_value(&self.source_views),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
