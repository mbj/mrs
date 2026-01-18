pub mod repository {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codecommit-repository-code.html
    pub struct Code_ {
        pub branch_name: Option<crate::value::ExpString>,
        pub s3: Box<S3_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codecommit_Repository_Code {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeCommit::Repository.Code"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codecommit_Repository_Code as Code;
    impl crate::value::ToValue for Code_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.branch_name {
                properties.insert(
                    "BranchName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert("S3".to_string(), crate::value::ToValue::to_value(&self.s3));
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codecommit-repository-repositorytrigger.html
    pub struct RepositoryTrigger_ {
        pub branches: Option<Vec<crate::value::ExpString>>,
        pub custom_data: Option<crate::value::ExpString>,
        pub destination_arn: crate::value::ExpString,
        pub events: Vec<crate::value::ExpString>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codecommit_Repository_RepositoryTrigger {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeCommit::Repository.RepositoryTrigger"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codecommit_Repository_RepositoryTrigger as RepositoryTrigger;
    impl crate::value::ToValue for RepositoryTrigger_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.branches {
                properties.insert(
                    "Branches".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_data {
                properties.insert(
                    "CustomData".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DestinationArn".to_string(),
                crate::value::ToValue::to_value(&self.destination_arn),
            );
            properties.insert(
                "Events".to_string(),
                crate::value::ToValue::to_value(&self.events),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codecommit-repository-s3.html
    pub struct S3_ {
        pub bucket: crate::value::ExpString,
        pub key: crate::value::ExpString,
        pub object_version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codecommit_Repository_S3 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeCommit::Repository.S3"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codecommit_Repository_S3 as S3;
    impl crate::value::ToValue for S3_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Bucket".to_string(),
                crate::value::ToValue::to_value(&self.bucket),
            );
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            if let Some(ref value) = self.object_version {
                properties.insert(
                    "ObjectVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codecommit-repository.html
pub struct Repository_ {
    pub code: Option<super::codecommit::repository::Code_>,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub repository_description: Option<crate::value::ExpString>,
    pub repository_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub triggers: Option<Vec<super::codecommit::repository::RepositoryTrigger_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_codecommit_Repository {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CodeCommit::Repository"
        $($field $value)*)
    };
}
pub use crate::__aws_codecommit_Repository as Repository;
impl crate::template::ToResource for Repository_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CodeCommit"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Repository"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.code {
            properties.insert("Code".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.kms_key_id {
            properties.insert(
                "KmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.repository_description {
            properties.insert(
                "RepositoryDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RepositoryName".to_string(),
            crate::value::ToValue::to_value(&self.repository_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.triggers {
            properties.insert(
                "Triggers".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
