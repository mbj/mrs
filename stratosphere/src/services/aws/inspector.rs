///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-inspector-assessmenttarget.html>
pub struct AssessmentTarget_ {
    pub assessment_target_name: Option<crate::value::ExpString>,
    pub resource_group_arn: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_inspector_AssessmentTarget {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Inspector::AssessmentTarget"
        $($field $value)*)
    };
}
pub use crate::__aws_inspector_AssessmentTarget as AssessmentTarget;
impl crate::template::ToResource for AssessmentTarget_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Inspector"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AssessmentTarget"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.assessment_target_name {
            properties.insert(
                "AssessmentTargetName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resource_group_arn {
            properties.insert(
                "ResourceGroupArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-inspector-assessmenttemplate.html>
pub struct AssessmentTemplate_ {
    pub assessment_target_arn: crate::value::ExpString,
    pub assessment_template_name: Option<crate::value::ExpString>,
    pub duration_in_seconds: i32,
    pub rules_package_arns: Vec<crate::value::ExpString>,
    pub user_attributes_for_findings: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_inspector_AssessmentTemplate {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Inspector::AssessmentTemplate"
        $($field $value)*)
    };
}
pub use crate::__aws_inspector_AssessmentTemplate as AssessmentTemplate;
impl crate::template::ToResource for AssessmentTemplate_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Inspector"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AssessmentTemplate"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AssessmentTargetArn".to_string(),
            crate::value::ToValue::to_value(&self.assessment_target_arn),
        );
        if let Some(ref value) = self.assessment_template_name {
            properties.insert(
                "AssessmentTemplateName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DurationInSeconds".to_string(),
            crate::value::ToValue::to_value(&self.duration_in_seconds),
        );
        properties.insert(
            "RulesPackageArns".to_string(),
            crate::value::ToValue::to_value(&self.rules_package_arns),
        );
        if let Some(ref value) = self.user_attributes_for_findings {
            properties.insert(
                "UserAttributesForFindings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-inspector-resourcegroup.html>
pub struct ResourceGroup_ {
    pub resource_group_tags: Vec<crate::Tag_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_inspector_ResourceGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Inspector::ResourceGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_inspector_ResourceGroup as ResourceGroup;
impl crate::template::ToResource for ResourceGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Inspector"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ResourceGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ResourceGroupTags".to_string(),
            crate::value::ToValue::to_value(&self.resource_group_tags),
        );
        properties
    }
}
