pub mod invoiceunit {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-invoicing-invoiceunit-resourcetag.html
    pub struct ResourceTag_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_invoicing_InvoiceUnit_ResourceTag {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Invoicing::InvoiceUnit.ResourceTag"
            $($field $value)*)
        };
    }
    pub use crate::__aws_invoicing_InvoiceUnit_ResourceTag as ResourceTag;
    impl crate::value::ToValue for ResourceTag_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-invoicing-invoiceunit-rule.html
    pub struct Rule_ {
        pub linked_accounts: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_invoicing_InvoiceUnit_Rule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Invoicing::InvoiceUnit.Rule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_invoicing_InvoiceUnit_Rule as Rule;
    impl crate::value::ToValue for Rule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LinkedAccounts".to_string(),
                crate::value::ToValue::to_value(&self.linked_accounts),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-invoicing-invoiceunit.html
pub struct InvoiceUnit_ {
    pub description: Option<crate::value::ExpString>,
    pub invoice_receiver: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub resource_tags: Option<Vec<super::invoicing::invoiceunit::ResourceTag_>>,
    pub rule: super::invoicing::invoiceunit::Rule_,
    pub tax_inheritance_disabled: Option<crate::value::ExpBool>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_invoicing_InvoiceUnit {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Invoicing::InvoiceUnit"
        $($field $value)*)
    };
}
pub use crate::__aws_invoicing_InvoiceUnit as InvoiceUnit;
impl crate::template::ToResource for InvoiceUnit_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Invoicing"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("InvoiceUnit"),
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
            "InvoiceReceiver".to_string(),
            crate::value::ToValue::to_value(&self.invoice_receiver),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.resource_tags {
            properties.insert(
                "ResourceTags".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Rule".to_string(),
            crate::value::ToValue::to_value(&self.rule),
        );
        if let Some(ref value) = self.tax_inheritance_disabled {
            properties.insert(
                "TaxInheritanceDisabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
