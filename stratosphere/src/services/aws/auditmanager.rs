pub mod assessment {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-auditmanager-assessment-awsaccount.html
    pub struct AWSAccount_ {
        pub email_address: Option<crate::value::ExpString>,
        pub id: Option<crate::value::ExpString>,
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_auditmanager_Assessment_AWSAccount {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AuditManager::Assessment.AWSAccount"
            $($field $value)*)
        };
    }
    pub use crate::__aws_auditmanager_Assessment_AWSAccount as AWSAccount;
    impl crate::value::ToValue for AWSAccount_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.email_address {
                properties.insert(
                    "EmailAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.id {
                properties.insert("Id".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-auditmanager-assessment-awsservice.html
    pub struct AWSService_ {
        pub service_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_auditmanager_Assessment_AWSService {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AuditManager::Assessment.AWSService"
            $($field $value)*)
        };
    }
    pub use crate::__aws_auditmanager_Assessment_AWSService as AWSService;
    impl crate::value::ToValue for AWSService_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.service_name {
                properties.insert(
                    "ServiceName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-auditmanager-assessment-assessmentreportsdestination.html
    pub struct AssessmentReportsDestination_ {
        pub destination: Option<crate::value::ExpString>,
        pub destination_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_auditmanager_Assessment_AssessmentReportsDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AuditManager::Assessment.AssessmentReportsDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_auditmanager_Assessment_AssessmentReportsDestination as AssessmentReportsDestination;
    impl crate::value::ToValue for AssessmentReportsDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.destination {
                properties.insert(
                    "Destination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.destination_type {
                properties.insert(
                    "DestinationType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-auditmanager-assessment-delegation.html
    pub struct Delegation_ {
        pub assessment_id: Option<crate::value::ExpString>,
        pub assessment_name: Option<crate::value::ExpString>,
        pub comment: Option<crate::value::ExpString>,
        pub control_set_id: Option<crate::value::ExpString>,
        pub created_by: Option<crate::value::ExpString>,
        pub creation_time: Option<f64>,
        pub id: Option<crate::value::ExpString>,
        pub last_updated: Option<f64>,
        pub role_arn: Option<crate::value::ExpString>,
        pub role_type: Option<crate::value::ExpString>,
        pub status: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_auditmanager_Assessment_Delegation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AuditManager::Assessment.Delegation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_auditmanager_Assessment_Delegation as Delegation;
    impl crate::value::ToValue for Delegation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.assessment_id {
                properties.insert(
                    "AssessmentId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.assessment_name {
                properties.insert(
                    "AssessmentName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.comment {
                properties.insert(
                    "Comment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.control_set_id {
                properties.insert(
                    "ControlSetId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.created_by {
                properties.insert(
                    "CreatedBy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.creation_time {
                properties.insert(
                    "CreationTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.id {
                properties.insert("Id".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.last_updated {
                properties.insert(
                    "LastUpdated".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.role_arn {
                properties.insert(
                    "RoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.role_type {
                properties.insert(
                    "RoleType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-auditmanager-assessment-role.html
    pub struct Role_ {
        pub role_arn: Option<crate::value::ExpString>,
        pub role_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_auditmanager_Assessment_Role {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AuditManager::Assessment.Role"
            $($field $value)*)
        };
    }
    pub use crate::__aws_auditmanager_Assessment_Role as Role;
    impl crate::value::ToValue for Role_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.role_arn {
                properties.insert(
                    "RoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.role_type {
                properties.insert(
                    "RoleType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-auditmanager-assessment-scope.html
    pub struct Scope_ {
        pub aws_accounts: Option<Vec<AWSAccount_>>,
        pub aws_services: Option<Vec<AWSService_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_auditmanager_Assessment_Scope {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AuditManager::Assessment.Scope"
            $($field $value)*)
        };
    }
    pub use crate::__aws_auditmanager_Assessment_Scope as Scope;
    impl crate::value::ToValue for Scope_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.aws_accounts {
                properties.insert(
                    "AwsAccounts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.aws_services {
                properties.insert(
                    "AwsServices".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-auditmanager-assessment.html
pub struct Assessment_ {
    pub assessment_reports_destination:
        Option<super::auditmanager::assessment::AssessmentReportsDestination_>,
    pub aws_account: Option<super::auditmanager::assessment::AWSAccount_>,
    pub delegations: Option<Vec<super::auditmanager::assessment::Delegation_>>,
    pub description: Option<crate::value::ExpString>,
    pub framework_id: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub roles: Option<Vec<super::auditmanager::assessment::Role_>>,
    pub scope: Option<super::auditmanager::assessment::Scope_>,
    pub status: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_auditmanager_Assessment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::AuditManager::Assessment"
        $($field $value)*)
    };
}
pub use crate::__aws_auditmanager_Assessment as Assessment;
impl crate::template::ToResource for Assessment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AuditManager"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Assessment"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.assessment_reports_destination {
            properties.insert(
                "AssessmentReportsDestination".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.aws_account {
            properties.insert(
                "AwsAccount".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.delegations {
            properties.insert(
                "Delegations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.framework_id {
            properties.insert(
                "FrameworkId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.roles {
            properties.insert("Roles".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.scope {
            properties.insert("Scope".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.status {
            properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
