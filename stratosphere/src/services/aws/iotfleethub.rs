///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleethub-application.html
pub struct Application_ {
    pub application_description: Option<crate::value::ExpString>,
    pub application_name: crate::value::ExpString,
    pub role_arn: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iotfleethub_Application {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTFleetHub::Application"
        $($field $value)*)
    };
}
pub use crate::__aws_iotfleethub_Application as Application;
impl crate::template::ToResource for Application_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTFleetHub"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Application"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.application_description {
            properties.insert(
                "ApplicationDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ApplicationName".to_string(),
            crate::value::ToValue::to_value(&self.application_name),
        );
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
