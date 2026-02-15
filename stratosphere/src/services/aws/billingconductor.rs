pub mod billinggroup {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billingconductor-billinggroup-accountgrouping.html
    pub struct AccountGrouping_ {
        pub auto_associate: Option<crate::value::ExpBool>,
        pub linked_account_ids: Option<Vec<crate::value::ExpString>>,
        pub responsibility_transfer_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_billingconductor_BillingGroup_AccountGrouping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BillingConductor::BillingGroup.AccountGrouping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_billingconductor_BillingGroup_AccountGrouping as AccountGrouping;
    impl crate::value::ToValue for AccountGrouping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auto_associate {
                properties.insert(
                    "AutoAssociate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.linked_account_ids {
                properties.insert(
                    "LinkedAccountIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.responsibility_transfer_arn {
                properties.insert(
                    "ResponsibilityTransferArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billingconductor-billinggroup-computationpreference.html
    pub struct ComputationPreference_ {
        pub pricing_plan_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_billingconductor_BillingGroup_ComputationPreference {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BillingConductor::BillingGroup.ComputationPreference"
            $($field $value)*)
        };
    }
    pub use crate::__aws_billingconductor_BillingGroup_ComputationPreference as ComputationPreference;
    impl crate::value::ToValue for ComputationPreference_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PricingPlanArn".to_string(),
                crate::value::ToValue::to_value(&self.pricing_plan_arn),
            );
            properties.into()
        }
    }
}
pub mod customlineitem {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billingconductor-customlineitem-billingperiodrange.html
    pub struct BillingPeriodRange_ {
        pub exclusive_end_billing_period: Option<crate::value::ExpString>,
        pub inclusive_start_billing_period: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_billingconductor_CustomLineItem_BillingPeriodRange {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BillingConductor::CustomLineItem.BillingPeriodRange"
            $($field $value)*)
        };
    }
    pub use crate::__aws_billingconductor_CustomLineItem_BillingPeriodRange as BillingPeriodRange;
    impl crate::value::ToValue for BillingPeriodRange_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.exclusive_end_billing_period {
                properties.insert(
                    "ExclusiveEndBillingPeriod".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inclusive_start_billing_period {
                properties.insert(
                    "InclusiveStartBillingPeriod".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billingconductor-customlineitem-customlineitemchargedetails.html
    pub struct CustomLineItemChargeDetails_ {
        pub flat: Option<Box<CustomLineItemFlatChargeDetails_>>,
        pub line_item_filters: Option<Vec<LineItemFilter_>>,
        pub percentage: Option<Box<CustomLineItemPercentageChargeDetails_>>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_billingconductor_CustomLineItem_CustomLineItemChargeDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BillingConductor::CustomLineItem.CustomLineItemChargeDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_billingconductor_CustomLineItem_CustomLineItemChargeDetails as CustomLineItemChargeDetails;
    impl crate::value::ToValue for CustomLineItemChargeDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.flat {
                properties.insert("Flat".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.line_item_filters {
                properties.insert(
                    "LineItemFilters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.percentage {
                properties.insert(
                    "Percentage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billingconductor-customlineitem-customlineitemflatchargedetails.html
    pub struct CustomLineItemFlatChargeDetails_ {
        pub charge_value: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_billingconductor_CustomLineItem_CustomLineItemFlatChargeDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BillingConductor::CustomLineItem.CustomLineItemFlatChargeDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_billingconductor_CustomLineItem_CustomLineItemFlatChargeDetails as CustomLineItemFlatChargeDetails;
    impl crate::value::ToValue for CustomLineItemFlatChargeDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ChargeValue".to_string(),
                crate::value::ToValue::to_value(&self.charge_value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billingconductor-customlineitem-customlineitempercentagechargedetails.html
    pub struct CustomLineItemPercentageChargeDetails_ {
        pub child_associated_resources: Option<Vec<crate::value::ExpString>>,
        pub percentage_value: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_billingconductor_CustomLineItem_CustomLineItemPercentageChargeDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BillingConductor::CustomLineItem.CustomLineItemPercentageChargeDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_billingconductor_CustomLineItem_CustomLineItemPercentageChargeDetails as CustomLineItemPercentageChargeDetails;
    impl crate::value::ToValue for CustomLineItemPercentageChargeDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.child_associated_resources {
                properties.insert(
                    "ChildAssociatedResources".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "PercentageValue".to_string(),
                crate::value::ToValue::to_value(&self.percentage_value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billingconductor-customlineitem-lineitemfilter.html
    pub struct LineItemFilter_ {
        pub attribute: crate::value::ExpString,
        pub attribute_values: Option<Vec<crate::value::ExpString>>,
        pub match_option: crate::value::ExpString,
        pub values: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_billingconductor_CustomLineItem_LineItemFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BillingConductor::CustomLineItem.LineItemFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_billingconductor_CustomLineItem_LineItemFilter as LineItemFilter;
    impl crate::value::ToValue for LineItemFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Attribute".to_string(),
                crate::value::ToValue::to_value(&self.attribute),
            );
            if let Some(ref value) = self.attribute_values {
                properties.insert(
                    "AttributeValues".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MatchOption".to_string(),
                crate::value::ToValue::to_value(&self.match_option),
            );
            if let Some(ref value) = self.values {
                properties.insert("Values".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billingconductor-customlineitem-presentationdetails.html
    pub struct PresentationDetails_ {
        pub service: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_billingconductor_CustomLineItem_PresentationDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BillingConductor::CustomLineItem.PresentationDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_billingconductor_CustomLineItem_PresentationDetails as PresentationDetails;
    impl crate::value::ToValue for PresentationDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Service".to_string(),
                crate::value::ToValue::to_value(&self.service),
            );
            properties.into()
        }
    }
}
pub mod pricingrule {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billingconductor-pricingrule-freetier.html
    pub struct FreeTier_ {
        pub activated: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_billingconductor_PricingRule_FreeTier {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BillingConductor::PricingRule.FreeTier"
            $($field $value)*)
        };
    }
    pub use crate::__aws_billingconductor_PricingRule_FreeTier as FreeTier;
    impl crate::value::ToValue for FreeTier_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Activated".to_string(),
                crate::value::ToValue::to_value(&self.activated),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billingconductor-pricingrule-tiering.html
    pub struct Tiering_ {
        pub free_tier: Option<Box<FreeTier_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_billingconductor_PricingRule_Tiering {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BillingConductor::PricingRule.Tiering"
            $($field $value)*)
        };
    }
    pub use crate::__aws_billingconductor_PricingRule_Tiering as Tiering;
    impl crate::value::ToValue for Tiering_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.free_tier {
                properties.insert(
                    "FreeTier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-billingconductor-billinggroup.html
pub struct BillingGroup_ {
    pub account_grouping: super::billingconductor::billinggroup::AccountGrouping_,
    pub computation_preference: super::billingconductor::billinggroup::ComputationPreference_,
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub primary_account_id: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_billingconductor_BillingGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::BillingConductor::BillingGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_billingconductor_BillingGroup as BillingGroup;
impl crate::template::ToResource for BillingGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("BillingConductor"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("BillingGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AccountGrouping".to_string(),
            crate::value::ToValue::to_value(&self.account_grouping),
        );
        properties.insert(
            "ComputationPreference".to_string(),
            crate::value::ToValue::to_value(&self.computation_preference),
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
        if let Some(ref value) = self.primary_account_id {
            properties.insert(
                "PrimaryAccountId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-billingconductor-customlineitem.html
pub struct CustomLineItem_ {
    pub account_id: Option<crate::value::ExpString>,
    pub billing_group_arn: crate::value::ExpString,
    pub billing_period_range: Option<super::billingconductor::customlineitem::BillingPeriodRange_>,
    pub computation_rule: Option<crate::value::ExpString>,
    pub custom_line_item_charge_details:
        Option<super::billingconductor::customlineitem::CustomLineItemChargeDetails_>,
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub presentation_details: Option<super::billingconductor::customlineitem::PresentationDetails_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_billingconductor_CustomLineItem {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::BillingConductor::CustomLineItem"
        $($field $value)*)
    };
}
pub use crate::__aws_billingconductor_CustomLineItem as CustomLineItem;
impl crate::template::ToResource for CustomLineItem_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("BillingConductor"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("CustomLineItem"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.account_id {
            properties.insert(
                "AccountId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "BillingGroupArn".to_string(),
            crate::value::ToValue::to_value(&self.billing_group_arn),
        );
        if let Some(ref value) = self.billing_period_range {
            properties.insert(
                "BillingPeriodRange".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.computation_rule {
            properties.insert(
                "ComputationRule".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.custom_line_item_charge_details {
            properties.insert(
                "CustomLineItemChargeDetails".to_string(),
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
        if let Some(ref value) = self.presentation_details {
            properties.insert(
                "PresentationDetails".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-billingconductor-pricingplan.html
pub struct PricingPlan_ {
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub pricing_rule_arns: Option<Vec<crate::value::ExpString>>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_billingconductor_PricingPlan {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::BillingConductor::PricingPlan"
        $($field $value)*)
    };
}
pub use crate::__aws_billingconductor_PricingPlan as PricingPlan;
impl crate::template::ToResource for PricingPlan_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("BillingConductor"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PricingPlan"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
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
        if let Some(ref value) = self.pricing_rule_arns {
            properties.insert(
                "PricingRuleArns".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-billingconductor-pricingrule.html
pub struct PricingRule_ {
    pub billing_entity: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub modifier_percentage: Option<f64>,
    pub name: crate::value::ExpString,
    pub operation: Option<crate::value::ExpString>,
    pub scope: crate::value::ExpString,
    pub service: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub tiering: Option<super::billingconductor::pricingrule::Tiering_>,
    pub r#type: crate::value::ExpString,
    pub usage_type: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_billingconductor_PricingRule {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::BillingConductor::PricingRule"
        $($field $value)*)
    };
}
pub use crate::__aws_billingconductor_PricingRule as PricingRule;
impl crate::template::ToResource for PricingRule_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("BillingConductor"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PricingRule"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.billing_entity {
            properties.insert(
                "BillingEntity".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.modifier_percentage {
            properties.insert(
                "ModifierPercentage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.operation {
            properties.insert(
                "Operation".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Scope".to_string(),
            crate::value::ToValue::to_value(&self.scope),
        );
        if let Some(ref value) = self.service {
            properties.insert(
                "Service".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tiering {
            properties.insert(
                "Tiering".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        if let Some(ref value) = self.usage_type {
            properties.insert(
                "UsageType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
