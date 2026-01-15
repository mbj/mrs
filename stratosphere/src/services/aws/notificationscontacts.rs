pub mod emailcontact {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-notificationscontacts-emailcontact-emailcontact.html
    pub struct EmailContact_ {
        pub address: crate::value::ExpString,
        pub arn: crate::value::ExpString,
        pub creation_time: crate::value::ExpString,
        pub name: crate::value::ExpString,
        pub status: crate::value::ExpString,
        pub update_time: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_notificationscontacts_EmailContact_EmailContact {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NotificationsContacts::EmailContact.EmailContact"
            $($field $value)*)
        };
    }
    pub use crate::__aws_notificationscontacts_EmailContact_EmailContact as EmailContact;
    impl crate::value::ToValue for EmailContact_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Address".to_string(),
                crate::value::ToValue::to_value(&self.address),
            );
            properties.insert(
                "Arn".to_string(),
                crate::value::ToValue::to_value(&self.arn),
            );
            properties.insert(
                "CreationTime".to_string(),
                crate::value::ToValue::to_value(&self.creation_time),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "Status".to_string(),
                crate::value::ToValue::to_value(&self.status),
            );
            properties.insert(
                "UpdateTime".to_string(),
                crate::value::ToValue::to_value(&self.update_time),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-notificationscontacts-emailcontact.html
pub struct EmailContact_ {
    pub email_address: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_notificationscontacts_EmailContact {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::NotificationsContacts::EmailContact"
        $($field $value)*)
    };
}
pub use crate::__aws_notificationscontacts_EmailContact as EmailContact;
impl crate::template::ToResource for EmailContact_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("NotificationsContacts"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("EmailContact"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "EmailAddress".to_string(),
            crate::value::ToValue::to_value(&self.email_address),
        );
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
