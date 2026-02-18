pub mod approvalteam {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mpa-approvalteam-approvalstrategy.html>
    pub struct ApprovalStrategy_ {
        pub mof_n: Box<MofNApprovalStrategy_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mpa_ApprovalTeam_ApprovalStrategy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MPA::ApprovalTeam.ApprovalStrategy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mpa_ApprovalTeam_ApprovalStrategy as ApprovalStrategy;
    impl crate::value::ToValue for ApprovalStrategy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MofN".to_string(),
                crate::value::ToValue::to_value(&self.mof_n),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mpa-approvalteam-approver.html>
    pub struct Approver_ {
        pub approver_id: Option<crate::value::ExpString>,
        pub primary_identity_id: crate::value::ExpString,
        pub primary_identity_source_arn: crate::value::ExpString,
        pub primary_identity_status: Option<crate::value::ExpString>,
        pub response_time: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mpa_ApprovalTeam_Approver {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MPA::ApprovalTeam.Approver"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mpa_ApprovalTeam_Approver as Approver;
    impl crate::value::ToValue for Approver_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.approver_id {
                properties.insert(
                    "ApproverId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "PrimaryIdentityId".to_string(),
                crate::value::ToValue::to_value(&self.primary_identity_id),
            );
            properties.insert(
                "PrimaryIdentitySourceArn".to_string(),
                crate::value::ToValue::to_value(&self.primary_identity_source_arn),
            );
            if let Some(ref value) = self.primary_identity_status {
                properties.insert(
                    "PrimaryIdentityStatus".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.response_time {
                properties.insert(
                    "ResponseTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mpa-approvalteam-mofnapprovalstrategy.html>
    pub struct MofNApprovalStrategy_ {
        pub min_approvals_required: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mpa_ApprovalTeam_MofNApprovalStrategy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MPA::ApprovalTeam.MofNApprovalStrategy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mpa_ApprovalTeam_MofNApprovalStrategy as MofNApprovalStrategy;
    impl crate::value::ToValue for MofNApprovalStrategy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MinApprovalsRequired".to_string(),
                crate::value::ToValue::to_value(&self.min_approvals_required),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mpa-approvalteam-policy.html>
    pub struct Policy_ {
        pub policy_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mpa_ApprovalTeam_Policy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MPA::ApprovalTeam.Policy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mpa_ApprovalTeam_Policy as Policy;
    impl crate::value::ToValue for Policy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PolicyArn".to_string(),
                crate::value::ToValue::to_value(&self.policy_arn),
            );
            properties.into()
        }
    }
}
pub mod identitysource {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mpa-identitysource-iamidentitycenter.html>
    pub struct IamIdentityCenter_ {
        pub approval_portal_url: Option<crate::value::ExpString>,
        pub instance_arn: crate::value::ExpString,
        pub region: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mpa_IdentitySource_IamIdentityCenter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MPA::IdentitySource.IamIdentityCenter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mpa_IdentitySource_IamIdentityCenter as IamIdentityCenter;
    impl crate::value::ToValue for IamIdentityCenter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.approval_portal_url {
                properties.insert(
                    "ApprovalPortalUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "InstanceArn".to_string(),
                crate::value::ToValue::to_value(&self.instance_arn),
            );
            properties.insert(
                "Region".to_string(),
                crate::value::ToValue::to_value(&self.region),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mpa-identitysource-identitysourceparameters.html>
    pub struct IdentitySourceParameters_ {
        pub iam_identity_center: Box<IamIdentityCenter_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mpa_IdentitySource_IdentitySourceParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MPA::IdentitySource.IdentitySourceParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mpa_IdentitySource_IdentitySourceParameters as IdentitySourceParameters;
    impl crate::value::ToValue for IdentitySourceParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IamIdentityCenter".to_string(),
                crate::value::ToValue::to_value(&self.iam_identity_center),
            );
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mpa-approvalteam.html>
pub struct ApprovalTeam_ {
    pub approval_strategy: super::mpa::approvalteam::ApprovalStrategy_,
    pub approvers: Vec<super::mpa::approvalteam::Approver_>,
    pub description: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub policies: Vec<super::mpa::approvalteam::Policy_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_mpa_ApprovalTeam {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MPA::ApprovalTeam"
        $($field $value)*)
    };
}
pub use crate::__aws_mpa_ApprovalTeam as ApprovalTeam;
impl crate::template::ToResource for ApprovalTeam_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MPA"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ApprovalTeam"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApprovalStrategy".to_string(),
            crate::value::ToValue::to_value(&self.approval_strategy),
        );
        properties.insert(
            "Approvers".to_string(),
            crate::value::ToValue::to_value(&self.approvers),
        );
        properties.insert(
            "Description".to_string(),
            crate::value::ToValue::to_value(&self.description),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "Policies".to_string(),
            crate::value::ToValue::to_value(&self.policies),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mpa-identitysource.html>
pub struct IdentitySource_ {
    pub identity_source_parameters: super::mpa::identitysource::IdentitySourceParameters_,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_mpa_IdentitySource {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MPA::IdentitySource"
        $($field $value)*)
    };
}
pub use crate::__aws_mpa_IdentitySource as IdentitySource;
impl crate::template::ToResource for IdentitySource_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MPA"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("IdentitySource"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "IdentitySourceParameters".to_string(),
            crate::value::ToValue::to_value(&self.identity_source_parameters),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
