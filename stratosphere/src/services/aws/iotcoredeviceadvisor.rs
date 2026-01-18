pub mod suitedefinition {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotcoredeviceadvisor-suitedefinition-deviceundertest.html
    pub struct DeviceUnderTest_ {
        pub certificate_arn: Option<crate::value::ExpString>,
        pub thing_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotcoredeviceadvisor_SuiteDefinition_DeviceUnderTest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTCoreDeviceAdvisor::SuiteDefinition.DeviceUnderTest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotcoredeviceadvisor_SuiteDefinition_DeviceUnderTest as DeviceUnderTest;
    impl crate::value::ToValue for DeviceUnderTest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.certificate_arn {
                properties.insert(
                    "CertificateArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.thing_arn {
                properties.insert(
                    "ThingArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotcoredeviceadvisor-suitedefinition-suitedefinitionconfiguration.html
    pub struct SuiteDefinitionConfiguration_ {
        pub device_permission_role_arn: crate::value::ExpString,
        pub devices: Option<Vec<DeviceUnderTest_>>,
        pub intended_for_qualification: Option<crate::value::ExpBool>,
        pub root_group: crate::value::ExpString,
        pub suite_definition_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotcoredeviceadvisor_SuiteDefinition_SuiteDefinitionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTCoreDeviceAdvisor::SuiteDefinition.SuiteDefinitionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotcoredeviceadvisor_SuiteDefinition_SuiteDefinitionConfiguration as SuiteDefinitionConfiguration;
    impl crate::value::ToValue for SuiteDefinitionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DevicePermissionRoleArn".to_string(),
                crate::value::ToValue::to_value(&self.device_permission_role_arn),
            );
            if let Some(ref value) = self.devices {
                properties.insert(
                    "Devices".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.intended_for_qualification {
                properties.insert(
                    "IntendedForQualification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RootGroup".to_string(),
                crate::value::ToValue::to_value(&self.root_group),
            );
            if let Some(ref value) = self.suite_definition_name {
                properties.insert(
                    "SuiteDefinitionName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotcoredeviceadvisor-suitedefinition.html
pub struct SuiteDefinition_ {
    pub suite_definition_configuration:
        super::iotcoredeviceadvisor::suitedefinition::SuiteDefinitionConfiguration_,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iotcoredeviceadvisor_SuiteDefinition {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTCoreDeviceAdvisor::SuiteDefinition"
        $($field $value)*)
    };
}
pub use crate::__aws_iotcoredeviceadvisor_SuiteDefinition as SuiteDefinition;
impl crate::template::ToResource for SuiteDefinition_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTCoreDeviceAdvisor"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SuiteDefinition"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "SuiteDefinitionConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.suite_definition_configuration),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
