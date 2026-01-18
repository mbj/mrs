pub mod license {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-licensemanager-license-borrowconfiguration.html
    pub struct BorrowConfiguration_ {
        pub allow_early_check_in: crate::value::ExpBool,
        pub max_time_to_live_in_minutes: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_licensemanager_License_BorrowConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LicenseManager::License.BorrowConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_licensemanager_License_BorrowConfiguration as BorrowConfiguration;
    impl crate::value::ToValue for BorrowConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AllowEarlyCheckIn".to_string(),
                crate::value::ToValue::to_value(&self.allow_early_check_in),
            );
            properties.insert(
                "MaxTimeToLiveInMinutes".to_string(),
                crate::value::ToValue::to_value(&self.max_time_to_live_in_minutes),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-licensemanager-license-consumptionconfiguration.html
    pub struct ConsumptionConfiguration_ {
        pub borrow_configuration: Option<Box<BorrowConfiguration_>>,
        pub provisional_configuration: Option<Box<ProvisionalConfiguration_>>,
        pub renew_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_licensemanager_License_ConsumptionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LicenseManager::License.ConsumptionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_licensemanager_License_ConsumptionConfiguration as ConsumptionConfiguration;
    impl crate::value::ToValue for ConsumptionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.borrow_configuration {
                properties.insert(
                    "BorrowConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.provisional_configuration {
                properties.insert(
                    "ProvisionalConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.renew_type {
                properties.insert(
                    "RenewType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-licensemanager-license-entitlement.html
    pub struct Entitlement_ {
        pub allow_check_in: Option<crate::value::ExpBool>,
        pub max_count: Option<i64>,
        pub name: crate::value::ExpString,
        pub overage: Option<crate::value::ExpBool>,
        pub unit: crate::value::ExpString,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_licensemanager_License_Entitlement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LicenseManager::License.Entitlement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_licensemanager_License_Entitlement as Entitlement;
    impl crate::value::ToValue for Entitlement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allow_check_in {
                properties.insert(
                    "AllowCheckIn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_count {
                properties.insert(
                    "MaxCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.overage {
                properties.insert(
                    "Overage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Unit".to_string(),
                crate::value::ToValue::to_value(&self.unit),
            );
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-licensemanager-license-issuerdata.html
    pub struct IssuerData_ {
        pub name: crate::value::ExpString,
        pub sign_key: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_licensemanager_License_IssuerData {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LicenseManager::License.IssuerData"
            $($field $value)*)
        };
    }
    pub use crate::__aws_licensemanager_License_IssuerData as IssuerData;
    impl crate::value::ToValue for IssuerData_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.sign_key {
                properties.insert(
                    "SignKey".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-licensemanager-license-metadata.html
    pub struct Metadata_ {
        pub name: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_licensemanager_License_Metadata {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LicenseManager::License.Metadata"
            $($field $value)*)
        };
    }
    pub use crate::__aws_licensemanager_License_Metadata as Metadata;
    impl crate::value::ToValue for Metadata_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-licensemanager-license-provisionalconfiguration.html
    pub struct ProvisionalConfiguration_ {
        pub max_time_to_live_in_minutes: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_licensemanager_License_ProvisionalConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LicenseManager::License.ProvisionalConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_licensemanager_License_ProvisionalConfiguration as ProvisionalConfiguration;
    impl crate::value::ToValue for ProvisionalConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MaxTimeToLiveInMinutes".to_string(),
                crate::value::ToValue::to_value(&self.max_time_to_live_in_minutes),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-licensemanager-license-validitydateformat.html
    pub struct ValidityDateFormat_ {
        pub begin: crate::value::ExpString,
        pub end: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_licensemanager_License_ValidityDateFormat {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LicenseManager::License.ValidityDateFormat"
            $($field $value)*)
        };
    }
    pub use crate::__aws_licensemanager_License_ValidityDateFormat as ValidityDateFormat;
    impl crate::value::ToValue for ValidityDateFormat_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Begin".to_string(),
                crate::value::ToValue::to_value(&self.begin),
            );
            properties.insert(
                "End".to_string(),
                crate::value::ToValue::to_value(&self.end),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-licensemanager-grant.html
pub struct Grant_ {
    pub allowed_operations: Option<Vec<crate::value::ExpString>>,
    pub grant_name: Option<crate::value::ExpString>,
    pub home_region: Option<crate::value::ExpString>,
    pub license_arn: Option<crate::value::ExpString>,
    pub principals: Option<Vec<crate::value::ExpString>>,
    pub status: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_licensemanager_Grant {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::LicenseManager::Grant"
        $($field $value)*)
    };
}
pub use crate::__aws_licensemanager_Grant as Grant;
impl crate::template::ToResource for Grant_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("LicenseManager"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Grant"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.allowed_operations {
            properties.insert(
                "AllowedOperations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.grant_name {
            properties.insert(
                "GrantName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.home_region {
            properties.insert(
                "HomeRegion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.license_arn {
            properties.insert(
                "LicenseArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.principals {
            properties.insert(
                "Principals".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.status {
            properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-licensemanager-license.html
pub struct License_ {
    pub beneficiary: Option<crate::value::ExpString>,
    pub consumption_configuration: super::licensemanager::license::ConsumptionConfiguration_,
    pub entitlements: Vec<super::licensemanager::license::Entitlement_>,
    pub home_region: crate::value::ExpString,
    pub issuer: super::licensemanager::license::IssuerData_,
    pub license_metadata: Option<Vec<super::licensemanager::license::Metadata_>>,
    pub license_name: crate::value::ExpString,
    pub product_name: crate::value::ExpString,
    pub product_sku: Option<crate::value::ExpString>,
    pub status: Option<crate::value::ExpString>,
    pub validity: super::licensemanager::license::ValidityDateFormat_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_licensemanager_License {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::LicenseManager::License"
        $($field $value)*)
    };
}
pub use crate::__aws_licensemanager_License as License;
impl crate::template::ToResource for License_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("LicenseManager"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("License"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.beneficiary {
            properties.insert(
                "Beneficiary".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ConsumptionConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.consumption_configuration),
        );
        properties.insert(
            "Entitlements".to_string(),
            crate::value::ToValue::to_value(&self.entitlements),
        );
        properties.insert(
            "HomeRegion".to_string(),
            crate::value::ToValue::to_value(&self.home_region),
        );
        properties.insert(
            "Issuer".to_string(),
            crate::value::ToValue::to_value(&self.issuer),
        );
        if let Some(ref value) = self.license_metadata {
            properties.insert(
                "LicenseMetadata".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "LicenseName".to_string(),
            crate::value::ToValue::to_value(&self.license_name),
        );
        properties.insert(
            "ProductName".to_string(),
            crate::value::ToValue::to_value(&self.product_name),
        );
        if let Some(ref value) = self.product_sku {
            properties.insert(
                "ProductSKU".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.status {
            properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Validity".to_string(),
            crate::value::ToValue::to_value(&self.validity),
        );
        properties
    }
}
