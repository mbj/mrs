pub mod accesspoint {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3objectlambda-accesspoint-alias.html>
    pub struct Alias_ {
        pub status: Option<crate::value::ExpString>,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3objectlambda_AccessPoint_Alias {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3ObjectLambda::AccessPoint.Alias"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3objectlambda_AccessPoint_Alias as Alias;
    impl crate::value::ToValue for Alias_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3objectlambda-accesspoint-awslambda.html>
    pub struct AwsLambda_ {
        pub function_arn: crate::value::ExpString,
        pub function_payload: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3objectlambda_AccessPoint_AwsLambda {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3ObjectLambda::AccessPoint.AwsLambda"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3objectlambda_AccessPoint_AwsLambda as AwsLambda;
    impl crate::value::ToValue for AwsLambda_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FunctionArn".to_string(),
                crate::value::ToValue::to_value(&self.function_arn),
            );
            if let Some(ref value) = self.function_payload {
                properties.insert(
                    "FunctionPayload".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3objectlambda-accesspoint-contenttransformation.html>
    pub struct ContentTransformation_ {
        pub aws_lambda: Box<AwsLambda_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3objectlambda_AccessPoint_ContentTransformation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3ObjectLambda::AccessPoint.ContentTransformation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3objectlambda_AccessPoint_ContentTransformation as ContentTransformation;
    impl crate::value::ToValue for ContentTransformation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AwsLambda".to_string(),
                crate::value::ToValue::to_value(&self.aws_lambda),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3objectlambda-accesspoint-objectlambdaconfiguration.html>
    pub struct ObjectLambdaConfiguration_ {
        pub allowed_features: Option<Vec<crate::value::ExpString>>,
        pub cloud_watch_metrics_enabled: Option<crate::value::ExpBool>,
        pub supporting_access_point: crate::value::ExpString,
        pub transformation_configurations: Vec<TransformationConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3objectlambda_AccessPoint_ObjectLambdaConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3ObjectLambda::AccessPoint.ObjectLambdaConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3objectlambda_AccessPoint_ObjectLambdaConfiguration as ObjectLambdaConfiguration;
    impl crate::value::ToValue for ObjectLambdaConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allowed_features {
                properties.insert(
                    "AllowedFeatures".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cloud_watch_metrics_enabled {
                properties.insert(
                    "CloudWatchMetricsEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SupportingAccessPoint".to_string(),
                crate::value::ToValue::to_value(&self.supporting_access_point),
            );
            properties.insert(
                "TransformationConfigurations".to_string(),
                crate::value::ToValue::to_value(&self.transformation_configurations),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3objectlambda-accesspoint-publicaccessblockconfiguration.html>
    pub struct PublicAccessBlockConfiguration_ {
        pub block_public_acls: Option<crate::value::ExpBool>,
        pub block_public_policy: Option<crate::value::ExpBool>,
        pub ignore_public_acls: Option<crate::value::ExpBool>,
        pub restrict_public_buckets: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3objectlambda_AccessPoint_PublicAccessBlockConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3ObjectLambda::AccessPoint.PublicAccessBlockConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3objectlambda_AccessPoint_PublicAccessBlockConfiguration as PublicAccessBlockConfiguration;
    impl crate::value::ToValue for PublicAccessBlockConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.block_public_acls {
                properties.insert(
                    "BlockPublicAcls".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.block_public_policy {
                properties.insert(
                    "BlockPublicPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ignore_public_acls {
                properties.insert(
                    "IgnorePublicAcls".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.restrict_public_buckets {
                properties.insert(
                    "RestrictPublicBuckets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3objectlambda-accesspoint-transformationconfiguration.html>
    pub struct TransformationConfiguration_ {
        pub actions: Vec<crate::value::ExpString>,
        pub content_transformation: Box<ContentTransformation_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3objectlambda_AccessPoint_TransformationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3ObjectLambda::AccessPoint.TransformationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3objectlambda_AccessPoint_TransformationConfiguration as TransformationConfiguration;
    impl crate::value::ToValue for TransformationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Actions".to_string(),
                crate::value::ToValue::to_value(&self.actions),
            );
            properties.insert(
                "ContentTransformation".to_string(),
                crate::value::ToValue::to_value(&self.content_transformation),
            );
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-s3objectlambda-accesspoint.html>
pub struct AccessPoint_ {
    pub name: Option<crate::value::ExpString>,
    pub object_lambda_configuration: super::s3objectlambda::accesspoint::ObjectLambdaConfiguration_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_s3objectlambda_AccessPoint {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::S3ObjectLambda::AccessPoint"
        $($field $value)*)
    };
}
pub use crate::__aws_s3objectlambda_AccessPoint as AccessPoint;
impl crate::template::ToResource for AccessPoint_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("S3ObjectLambda"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AccessPoint"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "ObjectLambdaConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.object_lambda_configuration),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-s3objectlambda-accesspointpolicy.html>
pub struct AccessPointPolicy_ {
    pub object_lambda_access_point: crate::value::ExpString,
    pub policy_document: serde_json::Value,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_s3objectlambda_AccessPointPolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::S3ObjectLambda::AccessPointPolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_s3objectlambda_AccessPointPolicy as AccessPointPolicy;
impl crate::template::ToResource for AccessPointPolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("S3ObjectLambda"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AccessPointPolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ObjectLambdaAccessPoint".to_string(),
            crate::value::ToValue::to_value(&self.object_lambda_access_point),
        );
        properties.insert(
            "PolicyDocument".to_string(),
            crate::value::ToValue::to_value(&self.policy_document),
        );
        properties
    }
}
