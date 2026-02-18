pub mod key {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-paymentcryptography-key-keyattributes.html>
    pub struct KeyAttributes_ {
        pub key_algorithm: crate::value::ExpString,
        pub key_class: crate::value::ExpString,
        pub key_modes_of_use: Box<KeyModesOfUse_>,
        pub key_usage: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_paymentcryptography_Key_KeyAttributes {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::PaymentCryptography::Key.KeyAttributes"
            $($field $value)*)
        };
    }
    pub use crate::__aws_paymentcryptography_Key_KeyAttributes as KeyAttributes;
    impl crate::value::ToValue for KeyAttributes_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "KeyAlgorithm".to_string(),
                crate::value::ToValue::to_value(&self.key_algorithm),
            );
            properties.insert(
                "KeyClass".to_string(),
                crate::value::ToValue::to_value(&self.key_class),
            );
            properties.insert(
                "KeyModesOfUse".to_string(),
                crate::value::ToValue::to_value(&self.key_modes_of_use),
            );
            properties.insert(
                "KeyUsage".to_string(),
                crate::value::ToValue::to_value(&self.key_usage),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-paymentcryptography-key-keymodesofuse.html>
    pub struct KeyModesOfUse_ {
        pub decrypt: Option<crate::value::ExpBool>,
        pub derive_key: Option<crate::value::ExpBool>,
        pub encrypt: Option<crate::value::ExpBool>,
        pub generate: Option<crate::value::ExpBool>,
        pub no_restrictions: Option<crate::value::ExpBool>,
        pub sign: Option<crate::value::ExpBool>,
        pub unwrap: Option<crate::value::ExpBool>,
        pub verify: Option<crate::value::ExpBool>,
        pub wrap: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_paymentcryptography_Key_KeyModesOfUse {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::PaymentCryptography::Key.KeyModesOfUse"
            $($field $value)*)
        };
    }
    pub use crate::__aws_paymentcryptography_Key_KeyModesOfUse as KeyModesOfUse;
    impl crate::value::ToValue for KeyModesOfUse_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.decrypt {
                properties.insert(
                    "Decrypt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.derive_key {
                properties.insert(
                    "DeriveKey".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.encrypt {
                properties.insert(
                    "Encrypt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.generate {
                properties.insert(
                    "Generate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.no_restrictions {
                properties.insert(
                    "NoRestrictions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sign {
                properties.insert("Sign".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.unwrap {
                properties.insert("Unwrap".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.verify {
                properties.insert("Verify".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.wrap {
                properties.insert("Wrap".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-paymentcryptography-key-replicationstatustype.html>
    pub struct ReplicationStatusType_ {
        pub status: crate::value::ExpString,
        pub status_message: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_paymentcryptography_Key_ReplicationStatusType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::PaymentCryptography::Key.ReplicationStatusType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_paymentcryptography_Key_ReplicationStatusType as ReplicationStatusType;
    impl crate::value::ToValue for ReplicationStatusType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Status".to_string(),
                crate::value::ToValue::to_value(&self.status),
            );
            if let Some(ref value) = self.status_message {
                properties.insert(
                    "StatusMessage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-paymentcryptography-alias.html>
pub struct Alias_ {
    pub alias_name: crate::value::ExpString,
    pub key_arn: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_paymentcryptography_Alias {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::PaymentCryptography::Alias"
        $($field $value)*)
    };
}
pub use crate::__aws_paymentcryptography_Alias as Alias;
impl crate::template::ToResource for Alias_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("PaymentCryptography"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Alias"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AliasName".to_string(),
            crate::value::ToValue::to_value(&self.alias_name),
        );
        if let Some(ref value) = self.key_arn {
            properties.insert("KeyArn".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-paymentcryptography-key.html>
pub struct Key_ {
    pub derive_key_usage: Option<crate::value::ExpString>,
    pub enabled: Option<crate::value::ExpBool>,
    pub exportable: crate::value::ExpBool,
    pub key_attributes: super::paymentcryptography::key::KeyAttributes_,
    pub key_check_value_algorithm: Option<crate::value::ExpString>,
    pub replication_regions: Option<Vec<crate::value::ExpString>>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_paymentcryptography_Key {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::PaymentCryptography::Key"
        $($field $value)*)
    };
}
pub use crate::__aws_paymentcryptography_Key as Key;
impl crate::template::ToResource for Key_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("PaymentCryptography"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Key"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.derive_key_usage {
            properties.insert(
                "DeriveKeyUsage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enabled {
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Exportable".to_string(),
            crate::value::ToValue::to_value(&self.exportable),
        );
        properties.insert(
            "KeyAttributes".to_string(),
            crate::value::ToValue::to_value(&self.key_attributes),
        );
        if let Some(ref value) = self.key_check_value_algorithm {
            properties.insert(
                "KeyCheckValueAlgorithm".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.replication_regions {
            properties.insert(
                "ReplicationRegions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
