pub mod robotapplication {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-robomaker-robotapplication-robotsoftwaresuite.html
    pub struct RobotSoftwareSuite_ {
        pub name: crate::value::ExpString,
        pub version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_robomaker_RobotApplication_RobotSoftwareSuite {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RoboMaker::RobotApplication.RobotSoftwareSuite"
            $($field $value)*)
        };
    }
    pub use crate::__aws_robomaker_RobotApplication_RobotSoftwareSuite as RobotSoftwareSuite;
    impl crate::value::ToValue for RobotSoftwareSuite_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.version {
                properties.insert(
                    "Version".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-robomaker-robotapplication-sourceconfig.html
    pub struct SourceConfig_ {
        pub architecture: crate::value::ExpString,
        pub s3_bucket: crate::value::ExpString,
        pub s3_key: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_robomaker_RobotApplication_SourceConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RoboMaker::RobotApplication.SourceConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_robomaker_RobotApplication_SourceConfig as SourceConfig;
    impl crate::value::ToValue for SourceConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Architecture".to_string(),
                crate::value::ToValue::to_value(&self.architecture),
            );
            properties.insert(
                "S3Bucket".to_string(),
                crate::value::ToValue::to_value(&self.s3_bucket),
            );
            properties.insert(
                "S3Key".to_string(),
                crate::value::ToValue::to_value(&self.s3_key),
            );
            properties.into()
        }
    }
}
pub mod simulationapplication {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-robomaker-simulationapplication-renderingengine.html
    pub struct RenderingEngine_ {
        pub name: crate::value::ExpString,
        pub version: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_robomaker_SimulationApplication_RenderingEngine {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RoboMaker::SimulationApplication.RenderingEngine"
            $($field $value)*)
        };
    }
    pub use crate::__aws_robomaker_SimulationApplication_RenderingEngine as RenderingEngine;
    impl crate::value::ToValue for RenderingEngine_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "Version".to_string(),
                crate::value::ToValue::to_value(&self.version),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-robomaker-simulationapplication-robotsoftwaresuite.html
    pub struct RobotSoftwareSuite_ {
        pub name: crate::value::ExpString,
        pub version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_robomaker_SimulationApplication_RobotSoftwareSuite {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RoboMaker::SimulationApplication.RobotSoftwareSuite"
            $($field $value)*)
        };
    }
    pub use crate::__aws_robomaker_SimulationApplication_RobotSoftwareSuite as RobotSoftwareSuite;
    impl crate::value::ToValue for RobotSoftwareSuite_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.version {
                properties.insert(
                    "Version".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-robomaker-simulationapplication-simulationsoftwaresuite.html
    pub struct SimulationSoftwareSuite_ {
        pub name: crate::value::ExpString,
        pub version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_robomaker_SimulationApplication_SimulationSoftwareSuite {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RoboMaker::SimulationApplication.SimulationSoftwareSuite"
            $($field $value)*)
        };
    }
    pub use crate::__aws_robomaker_SimulationApplication_SimulationSoftwareSuite as SimulationSoftwareSuite;
    impl crate::value::ToValue for SimulationSoftwareSuite_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.version {
                properties.insert(
                    "Version".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-robomaker-simulationapplication-sourceconfig.html
    pub struct SourceConfig_ {
        pub architecture: crate::value::ExpString,
        pub s3_bucket: crate::value::ExpString,
        pub s3_key: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_robomaker_SimulationApplication_SourceConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RoboMaker::SimulationApplication.SourceConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_robomaker_SimulationApplication_SourceConfig as SourceConfig;
    impl crate::value::ToValue for SourceConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Architecture".to_string(),
                crate::value::ToValue::to_value(&self.architecture),
            );
            properties.insert(
                "S3Bucket".to_string(),
                crate::value::ToValue::to_value(&self.s3_bucket),
            );
            properties.insert(
                "S3Key".to_string(),
                crate::value::ToValue::to_value(&self.s3_key),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-robomaker-fleet.html
pub struct Fleet_ {
    pub name: Option<crate::value::ExpString>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_robomaker_Fleet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::RoboMaker::Fleet"
        $($field $value)*)
    };
}
pub use crate::__aws_robomaker_Fleet as Fleet;
impl crate::template::ToResource for Fleet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RoboMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Fleet"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-robomaker-robot.html
pub struct Robot_ {
    pub architecture: crate::value::ExpString,
    pub fleet: Option<crate::value::ExpString>,
    pub greengrass_group_id: crate::value::ExpString,
    pub name: Option<crate::value::ExpString>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_robomaker_Robot {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::RoboMaker::Robot"
        $($field $value)*)
    };
}
pub use crate::__aws_robomaker_Robot as Robot;
impl crate::template::ToResource for Robot_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RoboMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Robot"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Architecture".to_string(),
            crate::value::ToValue::to_value(&self.architecture),
        );
        if let Some(ref value) = self.fleet {
            properties.insert("Fleet".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "GreengrassGroupId".to_string(),
            crate::value::ToValue::to_value(&self.greengrass_group_id),
        );
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-robomaker-robotapplication.html
pub struct RobotApplication_ {
    pub current_revision_id: Option<crate::value::ExpString>,
    pub environment: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub robot_software_suite: super::robomaker::robotapplication::RobotSoftwareSuite_,
    pub sources: Option<Vec<super::robomaker::robotapplication::SourceConfig_>>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_robomaker_RobotApplication {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::RoboMaker::RobotApplication"
        $($field $value)*)
    };
}
pub use crate::__aws_robomaker_RobotApplication as RobotApplication;
impl crate::template::ToResource for RobotApplication_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RoboMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RobotApplication"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.current_revision_id {
            properties.insert(
                "CurrentRevisionId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.environment {
            properties.insert(
                "Environment".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "RobotSoftwareSuite".to_string(),
            crate::value::ToValue::to_value(&self.robot_software_suite),
        );
        if let Some(ref value) = self.sources {
            properties.insert(
                "Sources".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-robomaker-robotapplicationversion.html
pub struct RobotApplicationVersion_ {
    pub application: crate::value::ExpString,
    pub current_revision_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_robomaker_RobotApplicationVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::RoboMaker::RobotApplicationVersion"
        $($field $value)*)
    };
}
pub use crate::__aws_robomaker_RobotApplicationVersion as RobotApplicationVersion;
impl crate::template::ToResource for RobotApplicationVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RoboMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RobotApplicationVersion"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Application".to_string(),
            crate::value::ToValue::to_value(&self.application),
        );
        if let Some(ref value) = self.current_revision_id {
            properties.insert(
                "CurrentRevisionId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-robomaker-simulationapplication.html
pub struct SimulationApplication_ {
    pub current_revision_id: Option<crate::value::ExpString>,
    pub environment: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub rendering_engine: Option<super::robomaker::simulationapplication::RenderingEngine_>,
    pub robot_software_suite: super::robomaker::simulationapplication::RobotSoftwareSuite_,
    pub simulation_software_suite:
        super::robomaker::simulationapplication::SimulationSoftwareSuite_,
    pub sources: Option<Vec<super::robomaker::simulationapplication::SourceConfig_>>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_robomaker_SimulationApplication {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::RoboMaker::SimulationApplication"
        $($field $value)*)
    };
}
pub use crate::__aws_robomaker_SimulationApplication as SimulationApplication;
impl crate::template::ToResource for SimulationApplication_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RoboMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SimulationApplication"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.current_revision_id {
            properties.insert(
                "CurrentRevisionId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.environment {
            properties.insert(
                "Environment".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.rendering_engine {
            properties.insert(
                "RenderingEngine".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RobotSoftwareSuite".to_string(),
            crate::value::ToValue::to_value(&self.robot_software_suite),
        );
        properties.insert(
            "SimulationSoftwareSuite".to_string(),
            crate::value::ToValue::to_value(&self.simulation_software_suite),
        );
        if let Some(ref value) = self.sources {
            properties.insert(
                "Sources".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-robomaker-simulationapplicationversion.html
pub struct SimulationApplicationVersion_ {
    pub application: crate::value::ExpString,
    pub current_revision_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_robomaker_SimulationApplicationVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::RoboMaker::SimulationApplicationVersion"
        $($field $value)*)
    };
}
pub use crate::__aws_robomaker_SimulationApplicationVersion as SimulationApplicationVersion;
impl crate::template::ToResource for SimulationApplicationVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RoboMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "SimulationApplicationVersion",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Application".to_string(),
            crate::value::ToValue::to_value(&self.application),
        );
        if let Some(ref value) = self.current_revision_id {
            properties.insert(
                "CurrentRevisionId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
