pub mod application {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-systemsmanagersap-application-componentinfo.html>
    pub struct ComponentInfo_ {
        pub component_type: Option<crate::value::ExpString>,
        pub ec2_instance_id: Option<crate::value::ExpString>,
        pub sid: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_systemsmanagersap_Application_ComponentInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SystemsManagerSAP::Application.ComponentInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_systemsmanagersap_Application_ComponentInfo as ComponentInfo;
    impl crate::value::ToValue for ComponentInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.component_type {
                properties.insert(
                    "ComponentType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ec2_instance_id {
                properties.insert(
                    "Ec2InstanceId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sid {
                properties.insert("Sid".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-systemsmanagersap-application-credential.html>
    pub struct Credential_ {
        pub credential_type: Option<crate::value::ExpString>,
        pub database_name: Option<crate::value::ExpString>,
        pub secret_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_systemsmanagersap_Application_Credential {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SystemsManagerSAP::Application.Credential"
            $($field $value)*)
        };
    }
    pub use crate::__aws_systemsmanagersap_Application_Credential as Credential;
    impl crate::value::ToValue for Credential_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.credential_type {
                properties.insert(
                    "CredentialType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.database_name {
                properties.insert(
                    "DatabaseName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secret_id {
                properties.insert(
                    "SecretId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-systemsmanagersap-application.html>
pub struct Application_ {
    pub application_id: crate::value::ExpString,
    pub application_type: crate::value::ExpString,
    pub components_info: Option<Vec<super::systemsmanagersap::application::ComponentInfo_>>,
    pub credentials: Option<Vec<super::systemsmanagersap::application::Credential_>>,
    pub database_arn: Option<crate::value::ExpString>,
    pub instances: Option<Vec<crate::value::ExpString>>,
    pub sap_instance_number: Option<crate::value::ExpString>,
    pub sid: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_systemsmanagersap_Application {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SystemsManagerSAP::Application"
        $($field $value)*)
    };
}
pub use crate::__aws_systemsmanagersap_Application as Application;
impl crate::template::ToResource for Application_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SystemsManagerSAP"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Application"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApplicationId".to_string(),
            crate::value::ToValue::to_value(&self.application_id),
        );
        properties.insert(
            "ApplicationType".to_string(),
            crate::value::ToValue::to_value(&self.application_type),
        );
        if let Some(ref value) = self.components_info {
            properties.insert(
                "ComponentsInfo".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.credentials {
            properties.insert(
                "Credentials".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.database_arn {
            properties.insert(
                "DatabaseArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.instances {
            properties.insert(
                "Instances".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sap_instance_number {
            properties.insert(
                "SapInstanceNumber".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sid {
            properties.insert("Sid".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
