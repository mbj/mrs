pub mod publicrepository {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecr-publicrepository-repositorycatalogdata.html>
    pub struct RepositoryCatalogData_ {
        pub about_text: Option<crate::value::ExpString>,
        pub architectures: Option<Vec<crate::value::ExpString>>,
        pub operating_systems: Option<Vec<crate::value::ExpString>>,
        pub repository_description: Option<crate::value::ExpString>,
        pub usage_text: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecr_PublicRepository_RepositoryCatalogData {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ECR::PublicRepository.RepositoryCatalogData"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecr_PublicRepository_RepositoryCatalogData as RepositoryCatalogData;
    impl crate::value::ToValue for RepositoryCatalogData_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.about_text {
                properties.insert(
                    "AboutText".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.architectures {
                properties.insert(
                    "Architectures".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.operating_systems {
                properties.insert(
                    "OperatingSystems".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.repository_description {
                properties.insert(
                    "RepositoryDescription".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.usage_text {
                properties.insert(
                    "UsageText".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod registryscanningconfiguration {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecr-registryscanningconfiguration-repositoryfilter.html>
    pub struct RepositoryFilter_ {
        pub filter: crate::value::ExpString,
        pub filter_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecr_RegistryScanningConfiguration_RepositoryFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ECR::RegistryScanningConfiguration.RepositoryFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecr_RegistryScanningConfiguration_RepositoryFilter as RepositoryFilter;
    impl crate::value::ToValue for RepositoryFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Filter".to_string(),
                crate::value::ToValue::to_value(&self.filter),
            );
            properties.insert(
                "FilterType".to_string(),
                crate::value::ToValue::to_value(&self.filter_type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecr-registryscanningconfiguration-scanningrule.html>
    pub struct ScanningRule_ {
        pub repository_filters: Vec<RepositoryFilter_>,
        pub scan_frequency: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecr_RegistryScanningConfiguration_ScanningRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ECR::RegistryScanningConfiguration.ScanningRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecr_RegistryScanningConfiguration_ScanningRule as ScanningRule;
    impl crate::value::ToValue for ScanningRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RepositoryFilters".to_string(),
                crate::value::ToValue::to_value(&self.repository_filters),
            );
            properties.insert(
                "ScanFrequency".to_string(),
                crate::value::ToValue::to_value(&self.scan_frequency),
            );
            properties.into()
        }
    }
}
pub mod replicationconfiguration {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecr-replicationconfiguration-replicationconfiguration.html>
    pub struct ReplicationConfiguration_ {
        pub rules: Vec<ReplicationRule_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecr_ReplicationConfiguration_ReplicationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ECR::ReplicationConfiguration.ReplicationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecr_ReplicationConfiguration_ReplicationConfiguration as ReplicationConfiguration;
    impl crate::value::ToValue for ReplicationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Rules".to_string(),
                crate::value::ToValue::to_value(&self.rules),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecr-replicationconfiguration-replicationdestination.html>
    pub struct ReplicationDestination_ {
        pub region: crate::value::ExpString,
        pub registry_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecr_ReplicationConfiguration_ReplicationDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ECR::ReplicationConfiguration.ReplicationDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecr_ReplicationConfiguration_ReplicationDestination as ReplicationDestination;
    impl crate::value::ToValue for ReplicationDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Region".to_string(),
                crate::value::ToValue::to_value(&self.region),
            );
            properties.insert(
                "RegistryId".to_string(),
                crate::value::ToValue::to_value(&self.registry_id),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecr-replicationconfiguration-replicationrule.html>
    pub struct ReplicationRule_ {
        pub destinations: Vec<ReplicationDestination_>,
        pub repository_filters: Option<Vec<RepositoryFilter_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecr_ReplicationConfiguration_ReplicationRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ECR::ReplicationConfiguration.ReplicationRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecr_ReplicationConfiguration_ReplicationRule as ReplicationRule;
    impl crate::value::ToValue for ReplicationRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Destinations".to_string(),
                crate::value::ToValue::to_value(&self.destinations),
            );
            if let Some(ref value) = self.repository_filters {
                properties.insert(
                    "RepositoryFilters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecr-replicationconfiguration-repositoryfilter.html>
    pub struct RepositoryFilter_ {
        pub filter: crate::value::ExpString,
        pub filter_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecr_ReplicationConfiguration_RepositoryFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ECR::ReplicationConfiguration.RepositoryFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecr_ReplicationConfiguration_RepositoryFilter as RepositoryFilter;
    impl crate::value::ToValue for RepositoryFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Filter".to_string(),
                crate::value::ToValue::to_value(&self.filter),
            );
            properties.insert(
                "FilterType".to_string(),
                crate::value::ToValue::to_value(&self.filter_type),
            );
            properties.into()
        }
    }
}
pub mod repository {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecr-repository-encryptionconfiguration.html>
    pub struct EncryptionConfiguration_ {
        pub encryption_type: crate::value::ExpString,
        pub kms_key: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecr_Repository_EncryptionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ECR::Repository.EncryptionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecr_Repository_EncryptionConfiguration as EncryptionConfiguration;
    impl crate::value::ToValue for EncryptionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EncryptionType".to_string(),
                crate::value::ToValue::to_value(&self.encryption_type),
            );
            if let Some(ref value) = self.kms_key {
                properties.insert("KmsKey".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecr-repository-imagescanningconfiguration.html>
    pub struct ImageScanningConfiguration_ {
        pub scan_on_push: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecr_Repository_ImageScanningConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ECR::Repository.ImageScanningConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecr_Repository_ImageScanningConfiguration as ImageScanningConfiguration;
    impl crate::value::ToValue for ImageScanningConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.scan_on_push {
                properties.insert(
                    "ScanOnPush".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecr-repository-imagetagmutabilityexclusionfilter.html>
    pub struct ImageTagMutabilityExclusionFilter_ {
        pub image_tag_mutability_exclusion_filter_type: crate::value::ExpString,
        pub image_tag_mutability_exclusion_filter_value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecr_Repository_ImageTagMutabilityExclusionFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ECR::Repository.ImageTagMutabilityExclusionFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecr_Repository_ImageTagMutabilityExclusionFilter as ImageTagMutabilityExclusionFilter;
    impl crate::value::ToValue for ImageTagMutabilityExclusionFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ImageTagMutabilityExclusionFilterType".to_string(),
                crate::value::ToValue::to_value(&self.image_tag_mutability_exclusion_filter_type),
            );
            properties.insert(
                "ImageTagMutabilityExclusionFilterValue".to_string(),
                crate::value::ToValue::to_value(&self.image_tag_mutability_exclusion_filter_value),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecr-repository-lifecyclepolicy.html>
    pub struct LifecyclePolicy_ {
        pub lifecycle_policy_text: Option<crate::value::ExpString>,
        pub registry_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecr_Repository_LifecyclePolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ECR::Repository.LifecyclePolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecr_Repository_LifecyclePolicy as LifecyclePolicy;
    impl crate::value::ToValue for LifecyclePolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.lifecycle_policy_text {
                properties.insert(
                    "LifecyclePolicyText".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.registry_id {
                properties.insert(
                    "RegistryId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod repositorycreationtemplate {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecr-repositorycreationtemplate-encryptionconfiguration.html>
    pub struct EncryptionConfiguration_ {
        pub encryption_type: crate::value::ExpString,
        pub kms_key: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecr_RepositoryCreationTemplate_EncryptionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ECR::RepositoryCreationTemplate.EncryptionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecr_RepositoryCreationTemplate_EncryptionConfiguration as EncryptionConfiguration;
    impl crate::value::ToValue for EncryptionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EncryptionType".to_string(),
                crate::value::ToValue::to_value(&self.encryption_type),
            );
            if let Some(ref value) = self.kms_key {
                properties.insert("KmsKey".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecr-repositorycreationtemplate-imagetagmutabilityexclusionfilter.html>
    pub struct ImageTagMutabilityExclusionFilter_ {
        pub image_tag_mutability_exclusion_filter_type: crate::value::ExpString,
        pub image_tag_mutability_exclusion_filter_value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecr_RepositoryCreationTemplate_ImageTagMutabilityExclusionFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ECR::RepositoryCreationTemplate.ImageTagMutabilityExclusionFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecr_RepositoryCreationTemplate_ImageTagMutabilityExclusionFilter as ImageTagMutabilityExclusionFilter;
    impl crate::value::ToValue for ImageTagMutabilityExclusionFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ImageTagMutabilityExclusionFilterType".to_string(),
                crate::value::ToValue::to_value(&self.image_tag_mutability_exclusion_filter_type),
            );
            properties.insert(
                "ImageTagMutabilityExclusionFilterValue".to_string(),
                crate::value::ToValue::to_value(&self.image_tag_mutability_exclusion_filter_value),
            );
            properties.into()
        }
    }
}
pub mod signingconfiguration {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecr-signingconfiguration-repositoryfilter.html>
    pub struct RepositoryFilter_ {
        pub filter: crate::value::ExpString,
        pub filter_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecr_SigningConfiguration_RepositoryFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ECR::SigningConfiguration.RepositoryFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecr_SigningConfiguration_RepositoryFilter as RepositoryFilter;
    impl crate::value::ToValue for RepositoryFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Filter".to_string(),
                crate::value::ToValue::to_value(&self.filter),
            );
            properties.insert(
                "FilterType".to_string(),
                crate::value::ToValue::to_value(&self.filter_type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecr-signingconfiguration-rule.html>
    pub struct Rule_ {
        pub repository_filters: Option<Vec<RepositoryFilter_>>,
        pub signing_profile_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecr_SigningConfiguration_Rule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ECR::SigningConfiguration.Rule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecr_SigningConfiguration_Rule as Rule;
    impl crate::value::ToValue for Rule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.repository_filters {
                properties.insert(
                    "RepositoryFilters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SigningProfileArn".to_string(),
                crate::value::ToValue::to_value(&self.signing_profile_arn),
            );
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecr-publicrepository.html>
pub struct PublicRepository_ {
    pub repository_catalog_data: Option<super::ecr::publicrepository::RepositoryCatalogData_>,
    pub repository_name: Option<crate::value::ExpString>,
    pub repository_policy_text: Option<serde_json::Value>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ecr_PublicRepository {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ECR::PublicRepository"
        $($field $value)*)
    };
}
pub use crate::__aws_ecr_PublicRepository as PublicRepository;
impl crate::template::ToResource for PublicRepository_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ECR"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PublicRepository"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.repository_catalog_data {
            properties.insert(
                "RepositoryCatalogData".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.repository_name {
            properties.insert(
                "RepositoryName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.repository_policy_text {
            properties.insert(
                "RepositoryPolicyText".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecr-pullthroughcacherule.html>
pub struct PullThroughCacheRule_ {
    pub credential_arn: Option<crate::value::ExpString>,
    pub custom_role_arn: Option<crate::value::ExpString>,
    pub ecr_repository_prefix: Option<crate::value::ExpString>,
    pub upstream_registry: Option<crate::value::ExpString>,
    pub upstream_registry_url: Option<crate::value::ExpString>,
    pub upstream_repository_prefix: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ecr_PullThroughCacheRule {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ECR::PullThroughCacheRule"
        $($field $value)*)
    };
}
pub use crate::__aws_ecr_PullThroughCacheRule as PullThroughCacheRule;
impl crate::template::ToResource for PullThroughCacheRule_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ECR"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PullThroughCacheRule"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.credential_arn {
            properties.insert(
                "CredentialArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.custom_role_arn {
            properties.insert(
                "CustomRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ecr_repository_prefix {
            properties.insert(
                "EcrRepositoryPrefix".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.upstream_registry {
            properties.insert(
                "UpstreamRegistry".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.upstream_registry_url {
            properties.insert(
                "UpstreamRegistryUrl".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.upstream_repository_prefix {
            properties.insert(
                "UpstreamRepositoryPrefix".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecr-pulltimeupdateexclusion.html>
pub struct PullTimeUpdateExclusion_ {
    pub principal_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ecr_PullTimeUpdateExclusion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ECR::PullTimeUpdateExclusion"
        $($field $value)*)
    };
}
pub use crate::__aws_ecr_PullTimeUpdateExclusion as PullTimeUpdateExclusion;
impl crate::template::ToResource for PullTimeUpdateExclusion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ECR"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PullTimeUpdateExclusion"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "PrincipalArn".to_string(),
            crate::value::ToValue::to_value(&self.principal_arn),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecr-registrypolicy.html>
pub struct RegistryPolicy_ {
    pub policy_text: serde_json::Value,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ecr_RegistryPolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ECR::RegistryPolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_ecr_RegistryPolicy as RegistryPolicy;
impl crate::template::ToResource for RegistryPolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ECR"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RegistryPolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "PolicyText".to_string(),
            crate::value::ToValue::to_value(&self.policy_text),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecr-registryscanningconfiguration.html>
pub struct RegistryScanningConfiguration_ {
    pub rules: Vec<super::ecr::registryscanningconfiguration::ScanningRule_>,
    pub scan_type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ecr_RegistryScanningConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ECR::RegistryScanningConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_ecr_RegistryScanningConfiguration as RegistryScanningConfiguration;
impl crate::template::ToResource for RegistryScanningConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ECR"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "RegistryScanningConfiguration",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Rules".to_string(),
            crate::value::ToValue::to_value(&self.rules),
        );
        properties.insert(
            "ScanType".to_string(),
            crate::value::ToValue::to_value(&self.scan_type),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecr-replicationconfiguration.html>
pub struct ReplicationConfiguration_ {
    pub replication_configuration: super::ecr::replicationconfiguration::ReplicationConfiguration_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ecr_ReplicationConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ECR::ReplicationConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_ecr_ReplicationConfiguration as ReplicationConfiguration;
impl crate::template::ToResource for ReplicationConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ECR"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ReplicationConfiguration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ReplicationConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.replication_configuration),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecr-repository.html>
pub struct Repository_ {
    pub empty_on_delete: Option<crate::value::ExpBool>,
    pub encryption_configuration: Option<super::ecr::repository::EncryptionConfiguration_>,
    pub image_scanning_configuration: Option<super::ecr::repository::ImageScanningConfiguration_>,
    pub image_tag_mutability: Option<crate::value::ExpString>,
    pub image_tag_mutability_exclusion_filters:
        Option<Vec<super::ecr::repository::ImageTagMutabilityExclusionFilter_>>,
    pub lifecycle_policy: Option<super::ecr::repository::LifecyclePolicy_>,
    pub repository_name: Option<crate::value::ExpString>,
    pub repository_policy_text: Option<serde_json::Value>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ecr_Repository {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ECR::Repository" $($field
        $value)*)
    };
}
pub use crate::__aws_ecr_Repository as Repository;
impl crate::template::ToResource for Repository_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ECR"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Repository"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.empty_on_delete {
            properties.insert(
                "EmptyOnDelete".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.encryption_configuration {
            properties.insert(
                "EncryptionConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.image_scanning_configuration {
            properties.insert(
                "ImageScanningConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.image_tag_mutability {
            properties.insert(
                "ImageTagMutability".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.image_tag_mutability_exclusion_filters {
            properties.insert(
                "ImageTagMutabilityExclusionFilters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.lifecycle_policy {
            properties.insert(
                "LifecyclePolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.repository_name {
            properties.insert(
                "RepositoryName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.repository_policy_text {
            properties.insert(
                "RepositoryPolicyText".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecr-repositorycreationtemplate.html>
pub struct RepositoryCreationTemplate_ {
    pub applied_for: Vec<crate::value::ExpString>,
    pub custom_role_arn: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub encryption_configuration:
        Option<super::ecr::repositorycreationtemplate::EncryptionConfiguration_>,
    pub image_tag_mutability: Option<crate::value::ExpString>,
    pub image_tag_mutability_exclusion_filters:
        Option<Vec<super::ecr::repositorycreationtemplate::ImageTagMutabilityExclusionFilter_>>,
    pub lifecycle_policy: Option<crate::value::ExpString>,
    pub prefix: crate::value::ExpString,
    pub repository_policy: Option<crate::value::ExpString>,
    pub resource_tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ecr_RepositoryCreationTemplate {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ECR::RepositoryCreationTemplate"
        $($field $value)*)
    };
}
pub use crate::__aws_ecr_RepositoryCreationTemplate as RepositoryCreationTemplate;
impl crate::template::ToResource for RepositoryCreationTemplate_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ECR"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "RepositoryCreationTemplate",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AppliedFor".to_string(),
            crate::value::ToValue::to_value(&self.applied_for),
        );
        if let Some(ref value) = self.custom_role_arn {
            properties.insert(
                "CustomRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.encryption_configuration {
            properties.insert(
                "EncryptionConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.image_tag_mutability {
            properties.insert(
                "ImageTagMutability".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.image_tag_mutability_exclusion_filters {
            properties.insert(
                "ImageTagMutabilityExclusionFilters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.lifecycle_policy {
            properties.insert(
                "LifecyclePolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Prefix".to_string(),
            crate::value::ToValue::to_value(&self.prefix),
        );
        if let Some(ref value) = self.repository_policy {
            properties.insert(
                "RepositoryPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resource_tags {
            properties.insert(
                "ResourceTags".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecr-signingconfiguration.html>
pub struct SigningConfiguration_ {
    pub rules: Vec<super::ecr::signingconfiguration::Rule_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ecr_SigningConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ECR::SigningConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_ecr_SigningConfiguration as SigningConfiguration;
impl crate::template::ToResource for SigningConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ECR"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SigningConfiguration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Rules".to_string(),
            crate::value::ToValue::to_value(&self.rules),
        );
        properties
    }
}
