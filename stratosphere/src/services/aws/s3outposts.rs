pub mod accesspoint {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3outposts-accesspoint-vpcconfiguration.html
    pub struct VpcConfiguration_ {
        pub vpc_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3outposts_AccessPoint_VpcConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3Outposts::AccessPoint.VpcConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3outposts_AccessPoint_VpcConfiguration as VpcConfiguration;
    impl crate::value::ToValue for VpcConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.vpc_id {
                properties.insert("VpcId".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod bucket {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3outposts-bucket-abortincompletemultipartupload.html
    pub struct AbortIncompleteMultipartUpload_ {
        pub days_after_initiation: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3outposts_Bucket_AbortIncompleteMultipartUpload {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3Outposts::Bucket.AbortIncompleteMultipartUpload"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3outposts_Bucket_AbortIncompleteMultipartUpload as AbortIncompleteMultipartUpload;
    impl crate::value::ToValue for AbortIncompleteMultipartUpload_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DaysAfterInitiation".to_string(),
                crate::value::ToValue::to_value(&self.days_after_initiation),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3outposts-bucket-filter.html
    pub struct Filter_ {
        pub and_operator: Option<Box<FilterAndOperator_>>,
        pub prefix: Option<crate::value::ExpString>,
        pub tag: Option<Box<FilterTag_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3outposts_Bucket_Filter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3Outposts::Bucket.Filter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3outposts_Bucket_Filter as Filter;
    impl crate::value::ToValue for Filter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.and_operator {
                properties.insert(
                    "AndOperator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.prefix {
                properties.insert("Prefix".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.tag {
                properties.insert("Tag".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3outposts-bucket-filterandoperator.html
    pub struct FilterAndOperator_ {
        pub prefix: Option<crate::value::ExpString>,
        pub tags: Vec<FilterTag_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3outposts_Bucket_FilterAndOperator {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3Outposts::Bucket.FilterAndOperator"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3outposts_Bucket_FilterAndOperator as FilterAndOperator;
    impl crate::value::ToValue for FilterAndOperator_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.prefix {
                properties.insert("Prefix".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Tags".to_string(),
                crate::value::ToValue::to_value(&self.tags),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3outposts-bucket-filtertag.html
    pub struct FilterTag_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3outposts_Bucket_FilterTag {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3Outposts::Bucket.FilterTag"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3outposts_Bucket_FilterTag as FilterTag;
    impl crate::value::ToValue for FilterTag_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3outposts-bucket-lifecycleconfiguration.html
    pub struct LifecycleConfiguration_ {
        pub rules: Vec<Rule_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3outposts_Bucket_LifecycleConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3Outposts::Bucket.LifecycleConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3outposts_Bucket_LifecycleConfiguration as LifecycleConfiguration;
    impl crate::value::ToValue for LifecycleConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Rules".to_string(),
                crate::value::ToValue::to_value(&self.rules),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3outposts-bucket-rule.html
    pub struct Rule_ {
        pub abort_incomplete_multipart_upload: Option<Box<AbortIncompleteMultipartUpload_>>,
        pub expiration_date: Option<crate::value::ExpString>,
        pub expiration_in_days: Option<i64>,
        pub filter: Option<Box<Filter_>>,
        pub id: Option<crate::value::ExpString>,
        pub status: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3outposts_Bucket_Rule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3Outposts::Bucket.Rule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3outposts_Bucket_Rule as Rule;
    impl crate::value::ToValue for Rule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.abort_incomplete_multipart_upload {
                properties.insert(
                    "AbortIncompleteMultipartUpload".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.expiration_date {
                properties.insert(
                    "ExpirationDate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.expiration_in_days {
                properties.insert(
                    "ExpirationInDays".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.filter {
                properties.insert("Filter".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.id {
                properties.insert("Id".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Status".to_string(),
                crate::value::ToValue::to_value(&self.status),
            );
            properties.into()
        }
    }
}
pub mod endpoint {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3outposts-endpoint-failedreason.html
    pub struct FailedReason_ {
        pub error_code: Option<crate::value::ExpString>,
        pub message: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3outposts_Endpoint_FailedReason {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3Outposts::Endpoint.FailedReason"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3outposts_Endpoint_FailedReason as FailedReason;
    impl crate::value::ToValue for FailedReason_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.error_code {
                properties.insert(
                    "ErrorCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.message {
                properties.insert(
                    "Message".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3outposts-endpoint-networkinterface.html
    pub struct NetworkInterface_ {
        pub network_interface_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3outposts_Endpoint_NetworkInterface {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3Outposts::Endpoint.NetworkInterface"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3outposts_Endpoint_NetworkInterface as NetworkInterface;
    impl crate::value::ToValue for NetworkInterface_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "NetworkInterfaceId".to_string(),
                crate::value::ToValue::to_value(&self.network_interface_id),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-s3outposts-accesspoint.html
pub struct AccessPoint_ {
    pub bucket: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub policy: Option<serde_json::Value>,
    pub vpc_configuration: super::s3outposts::accesspoint::VpcConfiguration_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_s3outposts_AccessPoint {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::S3Outposts::AccessPoint"
        $($field $value)*)
    };
}
pub use crate::__aws_s3outposts_AccessPoint as AccessPoint;
impl crate::template::ToResource for AccessPoint_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("S3Outposts"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AccessPoint"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Bucket".to_string(),
            crate::value::ToValue::to_value(&self.bucket),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.policy {
            properties.insert("Policy".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "VpcConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.vpc_configuration),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-s3outposts-bucket.html
pub struct Bucket_ {
    pub bucket_name: crate::value::ExpString,
    pub lifecycle_configuration: Option<super::s3outposts::bucket::LifecycleConfiguration_>,
    pub outpost_id: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_s3outposts_Bucket {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::S3Outposts::Bucket"
        $($field $value)*)
    };
}
pub use crate::__aws_s3outposts_Bucket as Bucket;
impl crate::template::ToResource for Bucket_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("S3Outposts"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Bucket"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "BucketName".to_string(),
            crate::value::ToValue::to_value(&self.bucket_name),
        );
        if let Some(ref value) = self.lifecycle_configuration {
            properties.insert(
                "LifecycleConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "OutpostId".to_string(),
            crate::value::ToValue::to_value(&self.outpost_id),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-s3outposts-bucketpolicy.html
pub struct BucketPolicy_ {
    pub bucket: crate::value::ExpString,
    pub policy_document: serde_json::Value,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_s3outposts_BucketPolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::S3Outposts::BucketPolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_s3outposts_BucketPolicy as BucketPolicy;
impl crate::template::ToResource for BucketPolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("S3Outposts"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("BucketPolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Bucket".to_string(),
            crate::value::ToValue::to_value(&self.bucket),
        );
        properties.insert(
            "PolicyDocument".to_string(),
            crate::value::ToValue::to_value(&self.policy_document),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-s3outposts-endpoint.html
pub struct Endpoint_ {
    pub access_type: Option<crate::value::ExpString>,
    pub customer_owned_ipv4_pool: Option<crate::value::ExpString>,
    pub failed_reason: Option<super::s3outposts::endpoint::FailedReason_>,
    pub outpost_id: crate::value::ExpString,
    pub security_group_id: crate::value::ExpString,
    pub subnet_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_s3outposts_Endpoint {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::S3Outposts::Endpoint"
        $($field $value)*)
    };
}
pub use crate::__aws_s3outposts_Endpoint as Endpoint;
impl crate::template::ToResource for Endpoint_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("S3Outposts"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Endpoint"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.access_type {
            properties.insert(
                "AccessType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.customer_owned_ipv4_pool {
            properties.insert(
                "CustomerOwnedIpv4Pool".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.failed_reason {
            properties.insert(
                "FailedReason".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "OutpostId".to_string(),
            crate::value::ToValue::to_value(&self.outpost_id),
        );
        properties.insert(
            "SecurityGroupId".to_string(),
            crate::value::ToValue::to_value(&self.security_group_id),
        );
        properties.insert(
            "SubnetId".to_string(),
            crate::value::ToValue::to_value(&self.subnet_id),
        );
        properties
    }
}
