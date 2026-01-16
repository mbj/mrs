///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-detective-graph.html
pub struct Graph_ {
    pub auto_enable_members: Option<crate::value::ExpBool>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_detective_Graph {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Detective::Graph" $($field
        $value)*)
    };
}
pub use crate::__aws_detective_Graph as Graph;
impl crate::template::ToResource for Graph_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Detective"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Graph"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.auto_enable_members {
            properties.insert(
                "AutoEnableMembers".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-detective-memberinvitation.html
pub struct MemberInvitation_ {
    pub disable_email_notification: Option<crate::value::ExpBool>,
    pub graph_arn: crate::value::ExpString,
    pub member_email_address: crate::value::ExpString,
    pub member_id: crate::value::ExpString,
    pub message: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_detective_MemberInvitation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Detective::MemberInvitation"
        $($field $value)*)
    };
}
pub use crate::__aws_detective_MemberInvitation as MemberInvitation;
impl crate::template::ToResource for MemberInvitation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Detective"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("MemberInvitation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.disable_email_notification {
            properties.insert(
                "DisableEmailNotification".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "GraphArn".to_string(),
            crate::value::ToValue::to_value(&self.graph_arn),
        );
        properties.insert(
            "MemberEmailAddress".to_string(),
            crate::value::ToValue::to_value(&self.member_email_address),
        );
        properties.insert(
            "MemberId".to_string(),
            crate::value::ToValue::to_value(&self.member_id),
        );
        if let Some(ref value) = self.message {
            properties.insert(
                "Message".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-detective-organizationadmin.html
pub struct OrganizationAdmin_ {
    pub account_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_detective_OrganizationAdmin {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Detective::OrganizationAdmin"
        $($field $value)*)
    };
}
pub use crate::__aws_detective_OrganizationAdmin as OrganizationAdmin;
impl crate::template::ToResource for OrganizationAdmin_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Detective"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("OrganizationAdmin"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AccountId".to_string(),
            crate::value::ToValue::to_value(&self.account_id),
        );
        properties
    }
}
