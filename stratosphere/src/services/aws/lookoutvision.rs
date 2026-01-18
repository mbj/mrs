///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lookoutvision-project.html
pub struct Project_ {
    pub project_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lookoutvision_Project {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::LookoutVision::Project"
        $($field $value)*)
    };
}
pub use crate::__aws_lookoutvision_Project as Project;
impl crate::template::ToResource for Project_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("LookoutVision"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Project"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ProjectName".to_string(),
            crate::value::ToValue::to_value(&self.project_name),
        );
        properties
    }
}
