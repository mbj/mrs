pub mod enabledbaseline {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-controltower-enabledbaseline-parameter.html
    pub struct Parameter_ {
        pub key: Option<crate::value::ExpString>,
        pub value: Option<serde_json::Value>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_controltower_EnabledBaseline_Parameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ControlTower::EnabledBaseline.Parameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_controltower_EnabledBaseline_Parameter as Parameter;
    impl crate::value::ToValue for Parameter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.key {
                properties.insert("Key".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod enabledcontrol {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-controltower-enabledcontrol-enabledcontrolparameter.html
    pub struct EnabledControlParameter_ {
        pub key: crate::value::ExpString,
        pub value: serde_json::Value,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_controltower_EnabledControl_EnabledControlParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ControlTower::EnabledControl.EnabledControlParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_controltower_EnabledControl_EnabledControlParameter as EnabledControlParameter;
    impl crate::value::ToValue for EnabledControlParameter_ {
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
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-controltower-enabledbaseline.html
pub struct EnabledBaseline_ {
    pub baseline_identifier: crate::value::ExpString,
    pub baseline_version: crate::value::ExpString,
    pub parameters: Option<Vec<super::controltower::enabledbaseline::Parameter_>>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub target_identifier: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_controltower_EnabledBaseline {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ControlTower::EnabledBaseline"
        $($field $value)*)
    };
}
pub use crate::__aws_controltower_EnabledBaseline as EnabledBaseline;
impl crate::template::ToResource for EnabledBaseline_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ControlTower"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("EnabledBaseline"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "BaselineIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.baseline_identifier),
        );
        properties.insert(
            "BaselineVersion".to_string(),
            crate::value::ToValue::to_value(&self.baseline_version),
        );
        if let Some(ref value) = self.parameters {
            properties.insert(
                "Parameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TargetIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.target_identifier),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-controltower-enabledcontrol.html
pub struct EnabledControl_ {
    pub control_identifier: crate::value::ExpString,
    pub parameters: Option<Vec<super::controltower::enabledcontrol::EnabledControlParameter_>>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub target_identifier: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_controltower_EnabledControl {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ControlTower::EnabledControl"
        $($field $value)*)
    };
}
pub use crate::__aws_controltower_EnabledControl as EnabledControl;
impl crate::template::ToResource for EnabledControl_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ControlTower"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("EnabledControl"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ControlIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.control_identifier),
        );
        if let Some(ref value) = self.parameters {
            properties.insert(
                "Parameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TargetIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.target_identifier),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-controltower-landingzone.html
pub struct LandingZone_ {
    pub manifest: serde_json::Value,
    pub remediation_types: Option<Vec<crate::value::ExpString>>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub version: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_controltower_LandingZone {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ControlTower::LandingZone"
        $($field $value)*)
    };
}
pub use crate::__aws_controltower_LandingZone as LandingZone;
impl crate::template::ToResource for LandingZone_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ControlTower"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LandingZone"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Manifest".to_string(),
            crate::value::ToValue::to_value(&self.manifest),
        );
        if let Some(ref value) = self.remediation_types {
            properties.insert(
                "RemediationTypes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Version".to_string(),
            crate::value::ToValue::to_value(&self.version),
        );
        properties
    }
}
