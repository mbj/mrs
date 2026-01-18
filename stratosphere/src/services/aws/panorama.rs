pub mod applicationinstance {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-panorama-applicationinstance-manifestoverridespayload.html
    pub struct ManifestOverridesPayload_ {
        pub payload_data: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_panorama_ApplicationInstance_ManifestOverridesPayload {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Panorama::ApplicationInstance.ManifestOverridesPayload"
            $($field $value)*)
        };
    }
    pub use crate::__aws_panorama_ApplicationInstance_ManifestOverridesPayload as ManifestOverridesPayload;
    impl crate::value::ToValue for ManifestOverridesPayload_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.payload_data {
                properties.insert(
                    "PayloadData".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-panorama-applicationinstance-manifestpayload.html
    pub struct ManifestPayload_ {
        pub payload_data: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_panorama_ApplicationInstance_ManifestPayload {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Panorama::ApplicationInstance.ManifestPayload"
            $($field $value)*)
        };
    }
    pub use crate::__aws_panorama_ApplicationInstance_ManifestPayload as ManifestPayload;
    impl crate::value::ToValue for ManifestPayload_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.payload_data {
                properties.insert(
                    "PayloadData".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod package {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-panorama-package-storagelocation.html
    pub struct StorageLocation_ {
        pub binary_prefix_location: Option<crate::value::ExpString>,
        pub bucket: Option<crate::value::ExpString>,
        pub generated_prefix_location: Option<crate::value::ExpString>,
        pub manifest_prefix_location: Option<crate::value::ExpString>,
        pub repo_prefix_location: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_panorama_Package_StorageLocation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Panorama::Package.StorageLocation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_panorama_Package_StorageLocation as StorageLocation;
    impl crate::value::ToValue for StorageLocation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.binary_prefix_location {
                properties.insert(
                    "BinaryPrefixLocation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bucket {
                properties.insert("Bucket".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.generated_prefix_location {
                properties.insert(
                    "GeneratedPrefixLocation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.manifest_prefix_location {
                properties.insert(
                    "ManifestPrefixLocation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.repo_prefix_location {
                properties.insert(
                    "RepoPrefixLocation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-panorama-applicationinstance.html
pub struct ApplicationInstance_ {
    pub application_instance_id_to_replace: Option<crate::value::ExpString>,
    pub default_runtime_context_device: crate::value::ExpString,
    pub description: Option<crate::value::ExpString>,
    pub manifest_overrides_payload:
        Option<super::panorama::applicationinstance::ManifestOverridesPayload_>,
    pub manifest_payload: super::panorama::applicationinstance::ManifestPayload_,
    pub name: Option<crate::value::ExpString>,
    pub runtime_role_arn: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_panorama_ApplicationInstance {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Panorama::ApplicationInstance"
        $($field $value)*)
    };
}
pub use crate::__aws_panorama_ApplicationInstance as ApplicationInstance;
impl crate::template::ToResource for ApplicationInstance_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Panorama"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ApplicationInstance"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.application_instance_id_to_replace {
            properties.insert(
                "ApplicationInstanceIdToReplace".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DefaultRuntimeContextDevice".to_string(),
            crate::value::ToValue::to_value(&self.default_runtime_context_device),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.manifest_overrides_payload {
            properties.insert(
                "ManifestOverridesPayload".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ManifestPayload".to_string(),
            crate::value::ToValue::to_value(&self.manifest_payload),
        );
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.runtime_role_arn {
            properties.insert(
                "RuntimeRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-panorama-package.html
pub struct Package_ {
    pub package_name: crate::value::ExpString,
    pub storage_location: Option<super::panorama::package::StorageLocation_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_panorama_Package {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Panorama::Package"
        $($field $value)*)
    };
}
pub use crate::__aws_panorama_Package as Package;
impl crate::template::ToResource for Package_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Panorama"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Package"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "PackageName".to_string(),
            crate::value::ToValue::to_value(&self.package_name),
        );
        if let Some(ref value) = self.storage_location {
            properties.insert(
                "StorageLocation".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-panorama-packageversion.html
pub struct PackageVersion_ {
    pub mark_latest: Option<crate::value::ExpBool>,
    pub owner_account: Option<crate::value::ExpString>,
    pub package_id: crate::value::ExpString,
    pub package_version: crate::value::ExpString,
    pub patch_version: crate::value::ExpString,
    pub updated_latest_patch_version: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_panorama_PackageVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Panorama::PackageVersion"
        $($field $value)*)
    };
}
pub use crate::__aws_panorama_PackageVersion as PackageVersion;
impl crate::template::ToResource for PackageVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Panorama"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PackageVersion"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.mark_latest {
            properties.insert(
                "MarkLatest".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.owner_account {
            properties.insert(
                "OwnerAccount".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "PackageId".to_string(),
            crate::value::ToValue::to_value(&self.package_id),
        );
        properties.insert(
            "PackageVersion".to_string(),
            crate::value::ToValue::to_value(&self.package_version),
        );
        properties.insert(
            "PatchVersion".to_string(),
            crate::value::ToValue::to_value(&self.patch_version),
        );
        if let Some(ref value) = self.updated_latest_patch_version {
            properties.insert(
                "UpdatedLatestPatchVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
