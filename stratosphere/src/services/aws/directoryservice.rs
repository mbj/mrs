pub mod microsoftad {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-directoryservice-microsoftad-vpcsettings.html
    pub struct VpcSettings_ {
        pub subnet_ids: Vec<crate::value::ExpString>,
        pub vpc_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_directoryservice_MicrosoftAD_VpcSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DirectoryService::MicrosoftAD.VpcSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_directoryservice_MicrosoftAD_VpcSettings as VpcSettings;
    impl crate::value::ToValue for VpcSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SubnetIds".to_string(),
                crate::value::ToValue::to_value(&self.subnet_ids),
            );
            properties.insert(
                "VpcId".to_string(),
                crate::value::ToValue::to_value(&self.vpc_id),
            );
            properties.into()
        }
    }
}
pub mod simplead {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-directoryservice-simplead-vpcsettings.html
    pub struct VpcSettings_ {
        pub subnet_ids: Vec<crate::value::ExpString>,
        pub vpc_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_directoryservice_SimpleAD_VpcSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DirectoryService::SimpleAD.VpcSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_directoryservice_SimpleAD_VpcSettings as VpcSettings;
    impl crate::value::ToValue for VpcSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SubnetIds".to_string(),
                crate::value::ToValue::to_value(&self.subnet_ids),
            );
            properties.insert(
                "VpcId".to_string(),
                crate::value::ToValue::to_value(&self.vpc_id),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-directoryservice-microsoftad.html
pub struct MicrosoftAD_ {
    pub create_alias: Option<crate::value::ExpBool>,
    pub edition: Option<crate::value::ExpString>,
    pub enable_sso: Option<crate::value::ExpBool>,
    pub name: crate::value::ExpString,
    pub password: crate::value::ExpString,
    pub short_name: Option<crate::value::ExpString>,
    pub vpc_settings: super::directoryservice::microsoftad::VpcSettings_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_directoryservice_MicrosoftAD {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DirectoryService::MicrosoftAD"
        $($field $value)*)
    };
}
pub use crate::__aws_directoryservice_MicrosoftAD as MicrosoftAD;
impl crate::template::ToResource for MicrosoftAD_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DirectoryService"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("MicrosoftAD"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.create_alias {
            properties.insert(
                "CreateAlias".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.edition {
            properties.insert(
                "Edition".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enable_sso {
            properties.insert(
                "EnableSso".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "Password".to_string(),
            crate::value::ToValue::to_value(&self.password),
        );
        if let Some(ref value) = self.short_name {
            properties.insert(
                "ShortName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "VpcSettings".to_string(),
            crate::value::ToValue::to_value(&self.vpc_settings),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-directoryservice-simplead.html
pub struct SimpleAD_ {
    pub create_alias: Option<crate::value::ExpBool>,
    pub description: Option<crate::value::ExpString>,
    pub enable_sso: Option<crate::value::ExpBool>,
    pub name: crate::value::ExpString,
    pub password: Option<crate::value::ExpString>,
    pub short_name: Option<crate::value::ExpString>,
    pub size: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_settings: super::directoryservice::simplead::VpcSettings_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_directoryservice_SimpleAD {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DirectoryService::SimpleAD"
        $($field $value)*)
    };
}
pub use crate::__aws_directoryservice_SimpleAD as SimpleAD;
impl crate::template::ToResource for SimpleAD_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DirectoryService"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SimpleAD"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.create_alias {
            properties.insert(
                "CreateAlias".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enable_sso {
            properties.insert(
                "EnableSso".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.password {
            properties.insert(
                "Password".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.short_name {
            properties.insert(
                "ShortName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Size".to_string(),
            crate::value::ToValue::to_value(&self.size),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "VpcSettings".to_string(),
            crate::value::ToValue::to_value(&self.vpc_settings),
        );
        properties
    }
}
