pub mod githubrepository {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codestar-githubrepository-code.html
    pub struct Code_ {
        pub s3: Box<S3_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codestar_GitHubRepository_Code {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeStar::GitHubRepository.Code"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codestar_GitHubRepository_Code as Code;
    impl crate::value::ToValue for Code_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert("S3".to_string(), crate::value::ToValue::to_value(&self.s3));
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codestar-githubrepository-s3.html
    pub struct S3_ {
        pub bucket: crate::value::ExpString,
        pub key: crate::value::ExpString,
        pub object_version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codestar_GitHubRepository_S3 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeStar::GitHubRepository.S3"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codestar_GitHubRepository_S3 as S3;
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestar-githubrepository.html
pub struct GitHubRepository_ {
    pub code: Option<super::codestar::githubrepository::Code_>,
    pub connection_arn: Option<crate::value::ExpString>,
    pub enable_issues: Option<crate::value::ExpBool>,
    pub is_private: Option<crate::value::ExpBool>,
    pub repository_access_token: Option<crate::value::ExpString>,
    pub repository_description: Option<crate::value::ExpString>,
    pub repository_name: crate::value::ExpString,
    pub repository_owner: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_codestar_GitHubRepository {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CodeStar::GitHubRepository"
        $($field $value)*)
    };
}
pub use crate::__aws_codestar_GitHubRepository as GitHubRepository;
impl crate::template::ToResource for GitHubRepository_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CodeStar"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("GitHubRepository"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.code {
            properties.insert("Code".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.connection_arn {
            properties.insert(
                "ConnectionArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enable_issues {
            properties.insert(
                "EnableIssues".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.is_private {
            properties.insert(
                "IsPrivate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.repository_access_token {
            properties.insert(
                "RepositoryAccessToken".to_string(),
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
        properties.insert(
            "RepositoryOwner".to_string(),
            crate::value::ToValue::to_value(&self.repository_owner),
        );
        properties
    }
}
