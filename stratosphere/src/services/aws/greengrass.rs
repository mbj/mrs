pub mod connectordefinition {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-connectordefinition-connector.html
    pub struct Connector_ {
        pub connector_arn: crate::value::ExpString,
        pub id: crate::value::ExpString,
        pub parameters: Option<serde_json::Value>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_ConnectorDefinition_Connector {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::ConnectorDefinition.Connector"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_ConnectorDefinition_Connector as Connector;
    impl crate::value::ToValue for Connector_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ConnectorArn".to_string(),
                crate::value::ToValue::to_value(&self.connector_arn),
            );
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            if let Some(ref value) = self.parameters {
                properties.insert(
                    "Parameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-connectordefinition-connectordefinitionversion.html
    pub struct ConnectorDefinitionVersion_ {
        pub connectors: Vec<Connector_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_ConnectorDefinition_ConnectorDefinitionVersion {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::ConnectorDefinition.ConnectorDefinitionVersion"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_ConnectorDefinition_ConnectorDefinitionVersion as ConnectorDefinitionVersion;
    impl crate::value::ToValue for ConnectorDefinitionVersion_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Connectors".to_string(),
                crate::value::ToValue::to_value(&self.connectors),
            );
            properties.into()
        }
    }
}
pub mod connectordefinitionversion {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-connectordefinitionversion-connector.html
    pub struct Connector_ {
        pub connector_arn: crate::value::ExpString,
        pub id: crate::value::ExpString,
        pub parameters: Option<serde_json::Value>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_ConnectorDefinitionVersion_Connector {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::ConnectorDefinitionVersion.Connector"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_ConnectorDefinitionVersion_Connector as Connector;
    impl crate::value::ToValue for Connector_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ConnectorArn".to_string(),
                crate::value::ToValue::to_value(&self.connector_arn),
            );
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            if let Some(ref value) = self.parameters {
                properties.insert(
                    "Parameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod coredefinition {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-coredefinition-core.html
    pub struct Core_ {
        pub certificate_arn: crate::value::ExpString,
        pub id: crate::value::ExpString,
        pub sync_shadow: Option<crate::value::ExpBool>,
        pub thing_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_CoreDefinition_Core {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::CoreDefinition.Core"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_CoreDefinition_Core as Core;
    impl crate::value::ToValue for Core_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CertificateArn".to_string(),
                crate::value::ToValue::to_value(&self.certificate_arn),
            );
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            if let Some(ref value) = self.sync_shadow {
                properties.insert(
                    "SyncShadow".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ThingArn".to_string(),
                crate::value::ToValue::to_value(&self.thing_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-coredefinition-coredefinitionversion.html
    pub struct CoreDefinitionVersion_ {
        pub cores: Vec<Core_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_CoreDefinition_CoreDefinitionVersion {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::CoreDefinition.CoreDefinitionVersion"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_CoreDefinition_CoreDefinitionVersion as CoreDefinitionVersion;
    impl crate::value::ToValue for CoreDefinitionVersion_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Cores".to_string(),
                crate::value::ToValue::to_value(&self.cores),
            );
            properties.into()
        }
    }
}
pub mod coredefinitionversion {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-coredefinitionversion-core.html
    pub struct Core_ {
        pub certificate_arn: crate::value::ExpString,
        pub id: crate::value::ExpString,
        pub sync_shadow: Option<crate::value::ExpBool>,
        pub thing_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_CoreDefinitionVersion_Core {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::CoreDefinitionVersion.Core"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_CoreDefinitionVersion_Core as Core;
    impl crate::value::ToValue for Core_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CertificateArn".to_string(),
                crate::value::ToValue::to_value(&self.certificate_arn),
            );
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            if let Some(ref value) = self.sync_shadow {
                properties.insert(
                    "SyncShadow".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ThingArn".to_string(),
                crate::value::ToValue::to_value(&self.thing_arn),
            );
            properties.into()
        }
    }
}
pub mod devicedefinition {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-devicedefinition-device.html
    pub struct Device_ {
        pub certificate_arn: crate::value::ExpString,
        pub id: crate::value::ExpString,
        pub sync_shadow: Option<crate::value::ExpBool>,
        pub thing_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_DeviceDefinition_Device {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::DeviceDefinition.Device"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_DeviceDefinition_Device as Device;
    impl crate::value::ToValue for Device_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CertificateArn".to_string(),
                crate::value::ToValue::to_value(&self.certificate_arn),
            );
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            if let Some(ref value) = self.sync_shadow {
                properties.insert(
                    "SyncShadow".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ThingArn".to_string(),
                crate::value::ToValue::to_value(&self.thing_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-devicedefinition-devicedefinitionversion.html
    pub struct DeviceDefinitionVersion_ {
        pub devices: Vec<Device_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_DeviceDefinition_DeviceDefinitionVersion {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::DeviceDefinition.DeviceDefinitionVersion"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_DeviceDefinition_DeviceDefinitionVersion as DeviceDefinitionVersion;
    impl crate::value::ToValue for DeviceDefinitionVersion_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Devices".to_string(),
                crate::value::ToValue::to_value(&self.devices),
            );
            properties.into()
        }
    }
}
pub mod devicedefinitionversion {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-devicedefinitionversion-device.html
    pub struct Device_ {
        pub certificate_arn: crate::value::ExpString,
        pub id: crate::value::ExpString,
        pub sync_shadow: Option<crate::value::ExpBool>,
        pub thing_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_DeviceDefinitionVersion_Device {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::DeviceDefinitionVersion.Device"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_DeviceDefinitionVersion_Device as Device;
    impl crate::value::ToValue for Device_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CertificateArn".to_string(),
                crate::value::ToValue::to_value(&self.certificate_arn),
            );
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            if let Some(ref value) = self.sync_shadow {
                properties.insert(
                    "SyncShadow".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ThingArn".to_string(),
                crate::value::ToValue::to_value(&self.thing_arn),
            );
            properties.into()
        }
    }
}
pub mod functiondefinition {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinition-defaultconfig.html
    pub struct DefaultConfig_ {
        pub execution: Box<Execution_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_FunctionDefinition_DefaultConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::FunctionDefinition.DefaultConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_FunctionDefinition_DefaultConfig as DefaultConfig;
    impl crate::value::ToValue for DefaultConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Execution".to_string(),
                crate::value::ToValue::to_value(&self.execution),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinition-environment.html
    pub struct Environment_ {
        pub access_sysfs: Option<crate::value::ExpBool>,
        pub execution: Option<Box<Execution_>>,
        pub resource_access_policies: Option<Vec<ResourceAccessPolicy_>>,
        pub variables: Option<serde_json::Value>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_FunctionDefinition_Environment {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::FunctionDefinition.Environment"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_FunctionDefinition_Environment as Environment;
    impl crate::value::ToValue for Environment_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.access_sysfs {
                properties.insert(
                    "AccessSysfs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.execution {
                properties.insert(
                    "Execution".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_access_policies {
                properties.insert(
                    "ResourceAccessPolicies".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.variables {
                properties.insert(
                    "Variables".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinition-execution.html
    pub struct Execution_ {
        pub isolation_mode: Option<crate::value::ExpString>,
        pub run_as: Option<Box<RunAs_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_FunctionDefinition_Execution {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::FunctionDefinition.Execution"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_FunctionDefinition_Execution as Execution;
    impl crate::value::ToValue for Execution_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.isolation_mode {
                properties.insert(
                    "IsolationMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.run_as {
                properties.insert("RunAs".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinition-function.html
    pub struct Function_ {
        pub function_arn: crate::value::ExpString,
        pub function_configuration: Box<FunctionConfiguration_>,
        pub id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_FunctionDefinition_Function {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::FunctionDefinition.Function"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_FunctionDefinition_Function as Function;
    impl crate::value::ToValue for Function_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FunctionArn".to_string(),
                crate::value::ToValue::to_value(&self.function_arn),
            );
            properties.insert(
                "FunctionConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.function_configuration),
            );
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinition-functionconfiguration.html
    pub struct FunctionConfiguration_ {
        pub encoding_type: Option<crate::value::ExpString>,
        pub environment: Option<Box<Environment_>>,
        pub exec_args: Option<crate::value::ExpString>,
        pub executable: Option<crate::value::ExpString>,
        pub memory_size: Option<i64>,
        pub pinned: Option<crate::value::ExpBool>,
        pub timeout: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_FunctionDefinition_FunctionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::FunctionDefinition.FunctionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_FunctionDefinition_FunctionConfiguration as FunctionConfiguration;
    impl crate::value::ToValue for FunctionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.encoding_type {
                properties.insert(
                    "EncodingType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.environment {
                properties.insert(
                    "Environment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exec_args {
                properties.insert(
                    "ExecArgs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.executable {
                properties.insert(
                    "Executable".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.memory_size {
                properties.insert(
                    "MemorySize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pinned {
                properties.insert("Pinned".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.timeout {
                properties.insert(
                    "Timeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinition-functiondefinitionversion.html
    pub struct FunctionDefinitionVersion_ {
        pub default_config: Option<Box<DefaultConfig_>>,
        pub functions: Vec<Function_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_FunctionDefinition_FunctionDefinitionVersion {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::FunctionDefinition.FunctionDefinitionVersion"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_FunctionDefinition_FunctionDefinitionVersion as FunctionDefinitionVersion;
    impl crate::value::ToValue for FunctionDefinitionVersion_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.default_config {
                properties.insert(
                    "DefaultConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Functions".to_string(),
                crate::value::ToValue::to_value(&self.functions),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinition-resourceaccesspolicy.html
    pub struct ResourceAccessPolicy_ {
        pub permission: Option<crate::value::ExpString>,
        pub resource_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_FunctionDefinition_ResourceAccessPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::FunctionDefinition.ResourceAccessPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_FunctionDefinition_ResourceAccessPolicy as ResourceAccessPolicy;
    impl crate::value::ToValue for ResourceAccessPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.permission {
                properties.insert(
                    "Permission".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ResourceId".to_string(),
                crate::value::ToValue::to_value(&self.resource_id),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinition-runas.html
    pub struct RunAs_ {
        pub gid: Option<i64>,
        pub uid: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_FunctionDefinition_RunAs {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::FunctionDefinition.RunAs"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_FunctionDefinition_RunAs as RunAs;
    impl crate::value::ToValue for RunAs_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.gid {
                properties.insert("Gid".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.uid {
                properties.insert("Uid".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod functiondefinitionversion {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinitionversion-defaultconfig.html
    pub struct DefaultConfig_ {
        pub execution: Box<Execution_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_FunctionDefinitionVersion_DefaultConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::FunctionDefinitionVersion.DefaultConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_FunctionDefinitionVersion_DefaultConfig as DefaultConfig;
    impl crate::value::ToValue for DefaultConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Execution".to_string(),
                crate::value::ToValue::to_value(&self.execution),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinitionversion-environment.html
    pub struct Environment_ {
        pub access_sysfs: Option<crate::value::ExpBool>,
        pub execution: Option<Box<Execution_>>,
        pub resource_access_policies: Option<Vec<ResourceAccessPolicy_>>,
        pub variables: Option<serde_json::Value>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_FunctionDefinitionVersion_Environment {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::FunctionDefinitionVersion.Environment"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_FunctionDefinitionVersion_Environment as Environment;
    impl crate::value::ToValue for Environment_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.access_sysfs {
                properties.insert(
                    "AccessSysfs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.execution {
                properties.insert(
                    "Execution".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_access_policies {
                properties.insert(
                    "ResourceAccessPolicies".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.variables {
                properties.insert(
                    "Variables".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinitionversion-execution.html
    pub struct Execution_ {
        pub isolation_mode: Option<crate::value::ExpString>,
        pub run_as: Option<Box<RunAs_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_FunctionDefinitionVersion_Execution {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::FunctionDefinitionVersion.Execution"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_FunctionDefinitionVersion_Execution as Execution;
    impl crate::value::ToValue for Execution_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.isolation_mode {
                properties.insert(
                    "IsolationMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.run_as {
                properties.insert("RunAs".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinitionversion-function.html
    pub struct Function_ {
        pub function_arn: crate::value::ExpString,
        pub function_configuration: Box<FunctionConfiguration_>,
        pub id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_FunctionDefinitionVersion_Function {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::FunctionDefinitionVersion.Function"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_FunctionDefinitionVersion_Function as Function;
    impl crate::value::ToValue for Function_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FunctionArn".to_string(),
                crate::value::ToValue::to_value(&self.function_arn),
            );
            properties.insert(
                "FunctionConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.function_configuration),
            );
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinitionversion-functionconfiguration.html
    pub struct FunctionConfiguration_ {
        pub encoding_type: Option<crate::value::ExpString>,
        pub environment: Option<Box<Environment_>>,
        pub exec_args: Option<crate::value::ExpString>,
        pub executable: Option<crate::value::ExpString>,
        pub memory_size: Option<i64>,
        pub pinned: Option<crate::value::ExpBool>,
        pub timeout: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_FunctionDefinitionVersion_FunctionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::FunctionDefinitionVersion.FunctionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_FunctionDefinitionVersion_FunctionConfiguration as FunctionConfiguration;
    impl crate::value::ToValue for FunctionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.encoding_type {
                properties.insert(
                    "EncodingType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.environment {
                properties.insert(
                    "Environment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exec_args {
                properties.insert(
                    "ExecArgs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.executable {
                properties.insert(
                    "Executable".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.memory_size {
                properties.insert(
                    "MemorySize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pinned {
                properties.insert("Pinned".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.timeout {
                properties.insert(
                    "Timeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinitionversion-resourceaccesspolicy.html
    pub struct ResourceAccessPolicy_ {
        pub permission: Option<crate::value::ExpString>,
        pub resource_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_FunctionDefinitionVersion_ResourceAccessPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::FunctionDefinitionVersion.ResourceAccessPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_FunctionDefinitionVersion_ResourceAccessPolicy as ResourceAccessPolicy;
    impl crate::value::ToValue for ResourceAccessPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.permission {
                properties.insert(
                    "Permission".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ResourceId".to_string(),
                crate::value::ToValue::to_value(&self.resource_id),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-functiondefinitionversion-runas.html
    pub struct RunAs_ {
        pub gid: Option<i64>,
        pub uid: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_FunctionDefinitionVersion_RunAs {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::FunctionDefinitionVersion.RunAs"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_FunctionDefinitionVersion_RunAs as RunAs;
    impl crate::value::ToValue for RunAs_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.gid {
                properties.insert("Gid".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.uid {
                properties.insert("Uid".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod group {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-group-groupversion.html
    pub struct GroupVersion_ {
        pub connector_definition_version_arn: Option<crate::value::ExpString>,
        pub core_definition_version_arn: Option<crate::value::ExpString>,
        pub device_definition_version_arn: Option<crate::value::ExpString>,
        pub function_definition_version_arn: Option<crate::value::ExpString>,
        pub logger_definition_version_arn: Option<crate::value::ExpString>,
        pub resource_definition_version_arn: Option<crate::value::ExpString>,
        pub subscription_definition_version_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_Group_GroupVersion {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::Group.GroupVersion"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_Group_GroupVersion as GroupVersion;
    impl crate::value::ToValue for GroupVersion_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.connector_definition_version_arn {
                properties.insert(
                    "ConnectorDefinitionVersionArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.core_definition_version_arn {
                properties.insert(
                    "CoreDefinitionVersionArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.device_definition_version_arn {
                properties.insert(
                    "DeviceDefinitionVersionArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.function_definition_version_arn {
                properties.insert(
                    "FunctionDefinitionVersionArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.logger_definition_version_arn {
                properties.insert(
                    "LoggerDefinitionVersionArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_definition_version_arn {
                properties.insert(
                    "ResourceDefinitionVersionArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.subscription_definition_version_arn {
                properties.insert(
                    "SubscriptionDefinitionVersionArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod loggerdefinition {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-loggerdefinition-logger.html
    pub struct Logger_ {
        pub component: crate::value::ExpString,
        pub id: crate::value::ExpString,
        pub level: crate::value::ExpString,
        pub space: Option<i64>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_LoggerDefinition_Logger {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::LoggerDefinition.Logger"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_LoggerDefinition_Logger as Logger;
    impl crate::value::ToValue for Logger_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Component".to_string(),
                crate::value::ToValue::to_value(&self.component),
            );
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            properties.insert(
                "Level".to_string(),
                crate::value::ToValue::to_value(&self.level),
            );
            if let Some(ref value) = self.space {
                properties.insert("Space".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-loggerdefinition-loggerdefinitionversion.html
    pub struct LoggerDefinitionVersion_ {
        pub loggers: Vec<Logger_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_LoggerDefinition_LoggerDefinitionVersion {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::LoggerDefinition.LoggerDefinitionVersion"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_LoggerDefinition_LoggerDefinitionVersion as LoggerDefinitionVersion;
    impl crate::value::ToValue for LoggerDefinitionVersion_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Loggers".to_string(),
                crate::value::ToValue::to_value(&self.loggers),
            );
            properties.into()
        }
    }
}
pub mod loggerdefinitionversion {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-loggerdefinitionversion-logger.html
    pub struct Logger_ {
        pub component: crate::value::ExpString,
        pub id: crate::value::ExpString,
        pub level: crate::value::ExpString,
        pub space: Option<i64>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_LoggerDefinitionVersion_Logger {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::LoggerDefinitionVersion.Logger"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_LoggerDefinitionVersion_Logger as Logger;
    impl crate::value::ToValue for Logger_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Component".to_string(),
                crate::value::ToValue::to_value(&self.component),
            );
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            properties.insert(
                "Level".to_string(),
                crate::value::ToValue::to_value(&self.level),
            );
            if let Some(ref value) = self.space {
                properties.insert("Space".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
}
pub mod resourcedefinition {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-groupownersetting.html
    pub struct GroupOwnerSetting_ {
        pub auto_add_group_owner: crate::value::ExpBool,
        pub group_owner: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_ResourceDefinition_GroupOwnerSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::ResourceDefinition.GroupOwnerSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_ResourceDefinition_GroupOwnerSetting as GroupOwnerSetting;
    impl crate::value::ToValue for GroupOwnerSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AutoAddGroupOwner".to_string(),
                crate::value::ToValue::to_value(&self.auto_add_group_owner),
            );
            if let Some(ref value) = self.group_owner {
                properties.insert(
                    "GroupOwner".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-localdeviceresourcedata.html
    pub struct LocalDeviceResourceData_ {
        pub group_owner_setting: Option<Box<GroupOwnerSetting_>>,
        pub source_path: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_ResourceDefinition_LocalDeviceResourceData {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::ResourceDefinition.LocalDeviceResourceData"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_ResourceDefinition_LocalDeviceResourceData as LocalDeviceResourceData;
    impl crate::value::ToValue for LocalDeviceResourceData_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.group_owner_setting {
                properties.insert(
                    "GroupOwnerSetting".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SourcePath".to_string(),
                crate::value::ToValue::to_value(&self.source_path),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-localvolumeresourcedata.html
    pub struct LocalVolumeResourceData_ {
        pub destination_path: crate::value::ExpString,
        pub group_owner_setting: Option<Box<GroupOwnerSetting_>>,
        pub source_path: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_ResourceDefinition_LocalVolumeResourceData {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::ResourceDefinition.LocalVolumeResourceData"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_ResourceDefinition_LocalVolumeResourceData as LocalVolumeResourceData;
    impl crate::value::ToValue for LocalVolumeResourceData_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DestinationPath".to_string(),
                crate::value::ToValue::to_value(&self.destination_path),
            );
            if let Some(ref value) = self.group_owner_setting {
                properties.insert(
                    "GroupOwnerSetting".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SourcePath".to_string(),
                crate::value::ToValue::to_value(&self.source_path),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-resourcedatacontainer.html
    pub struct ResourceDataContainer_ {
        pub local_device_resource_data: Option<Box<LocalDeviceResourceData_>>,
        pub local_volume_resource_data: Option<Box<LocalVolumeResourceData_>>,
        pub s3_machine_learning_model_resource_data:
            Option<Box<S3MachineLearningModelResourceData_>>,
        pub sage_maker_machine_learning_model_resource_data:
            Option<Box<SageMakerMachineLearningModelResourceData_>>,
        pub secrets_manager_secret_resource_data: Option<Box<SecretsManagerSecretResourceData_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_ResourceDefinition_ResourceDataContainer {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::ResourceDefinition.ResourceDataContainer"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_ResourceDefinition_ResourceDataContainer as ResourceDataContainer;
    impl crate::value::ToValue for ResourceDataContainer_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.local_device_resource_data {
                properties.insert(
                    "LocalDeviceResourceData".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.local_volume_resource_data {
                properties.insert(
                    "LocalVolumeResourceData".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_machine_learning_model_resource_data {
                properties.insert(
                    "S3MachineLearningModelResourceData".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sage_maker_machine_learning_model_resource_data {
                properties.insert(
                    "SageMakerMachineLearningModelResourceData".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secrets_manager_secret_resource_data {
                properties.insert(
                    "SecretsManagerSecretResourceData".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-resourcedefinitionversion.html
    pub struct ResourceDefinitionVersion_ {
        pub resources: Vec<ResourceInstance_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_ResourceDefinition_ResourceDefinitionVersion {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::ResourceDefinition.ResourceDefinitionVersion"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_ResourceDefinition_ResourceDefinitionVersion as ResourceDefinitionVersion;
    impl crate::value::ToValue for ResourceDefinitionVersion_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Resources".to_string(),
                crate::value::ToValue::to_value(&self.resources),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-resourcedownloadownersetting.html
    pub struct ResourceDownloadOwnerSetting_ {
        pub group_owner: crate::value::ExpString,
        pub group_permission: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_ResourceDefinition_ResourceDownloadOwnerSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::ResourceDefinition.ResourceDownloadOwnerSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_ResourceDefinition_ResourceDownloadOwnerSetting as ResourceDownloadOwnerSetting;
    impl crate::value::ToValue for ResourceDownloadOwnerSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "GroupOwner".to_string(),
                crate::value::ToValue::to_value(&self.group_owner),
            );
            properties.insert(
                "GroupPermission".to_string(),
                crate::value::ToValue::to_value(&self.group_permission),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-resourceinstance.html
    pub struct ResourceInstance_ {
        pub id: crate::value::ExpString,
        pub name: crate::value::ExpString,
        pub resource_data_container: Box<ResourceDataContainer_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_ResourceDefinition_ResourceInstance {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::ResourceDefinition.ResourceInstance"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_ResourceDefinition_ResourceInstance as ResourceInstance;
    impl crate::value::ToValue for ResourceInstance_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "ResourceDataContainer".to_string(),
                crate::value::ToValue::to_value(&self.resource_data_container),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-s3machinelearningmodelresourcedata.html
    pub struct S3MachineLearningModelResourceData_ {
        pub destination_path: crate::value::ExpString,
        pub owner_setting: Option<Box<ResourceDownloadOwnerSetting_>>,
        pub s3_uri: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_ResourceDefinition_S3MachineLearningModelResourceData {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::ResourceDefinition.S3MachineLearningModelResourceData"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_ResourceDefinition_S3MachineLearningModelResourceData as S3MachineLearningModelResourceData;
    impl crate::value::ToValue for S3MachineLearningModelResourceData_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DestinationPath".to_string(),
                crate::value::ToValue::to_value(&self.destination_path),
            );
            if let Some(ref value) = self.owner_setting {
                properties.insert(
                    "OwnerSetting".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "S3Uri".to_string(),
                crate::value::ToValue::to_value(&self.s3_uri),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-sagemakermachinelearningmodelresourcedata.html
    pub struct SageMakerMachineLearningModelResourceData_ {
        pub destination_path: crate::value::ExpString,
        pub owner_setting: Option<Box<ResourceDownloadOwnerSetting_>>,
        pub sage_maker_job_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_ResourceDefinition_SageMakerMachineLearningModelResourceData {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::ResourceDefinition.SageMakerMachineLearningModelResourceData"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_ResourceDefinition_SageMakerMachineLearningModelResourceData as SageMakerMachineLearningModelResourceData;
    impl crate::value::ToValue for SageMakerMachineLearningModelResourceData_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DestinationPath".to_string(),
                crate::value::ToValue::to_value(&self.destination_path),
            );
            if let Some(ref value) = self.owner_setting {
                properties.insert(
                    "OwnerSetting".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SageMakerJobArn".to_string(),
                crate::value::ToValue::to_value(&self.sage_maker_job_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinition-secretsmanagersecretresourcedata.html
    pub struct SecretsManagerSecretResourceData_ {
        pub arn: crate::value::ExpString,
        pub additional_staging_labels_to_download: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_ResourceDefinition_SecretsManagerSecretResourceData {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::ResourceDefinition.SecretsManagerSecretResourceData"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_ResourceDefinition_SecretsManagerSecretResourceData as SecretsManagerSecretResourceData;
    impl crate::value::ToValue for SecretsManagerSecretResourceData_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ARN".to_string(),
                crate::value::ToValue::to_value(&self.arn),
            );
            if let Some(ref value) = self.additional_staging_labels_to_download {
                properties.insert(
                    "AdditionalStagingLabelsToDownload".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod resourcedefinitionversion {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-groupownersetting.html
    pub struct GroupOwnerSetting_ {
        pub auto_add_group_owner: crate::value::ExpBool,
        pub group_owner: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_ResourceDefinitionVersion_GroupOwnerSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::ResourceDefinitionVersion.GroupOwnerSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_ResourceDefinitionVersion_GroupOwnerSetting as GroupOwnerSetting;
    impl crate::value::ToValue for GroupOwnerSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AutoAddGroupOwner".to_string(),
                crate::value::ToValue::to_value(&self.auto_add_group_owner),
            );
            if let Some(ref value) = self.group_owner {
                properties.insert(
                    "GroupOwner".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-localdeviceresourcedata.html
    pub struct LocalDeviceResourceData_ {
        pub group_owner_setting: Option<Box<GroupOwnerSetting_>>,
        pub source_path: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_ResourceDefinitionVersion_LocalDeviceResourceData {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::ResourceDefinitionVersion.LocalDeviceResourceData"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_ResourceDefinitionVersion_LocalDeviceResourceData as LocalDeviceResourceData;
    impl crate::value::ToValue for LocalDeviceResourceData_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.group_owner_setting {
                properties.insert(
                    "GroupOwnerSetting".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SourcePath".to_string(),
                crate::value::ToValue::to_value(&self.source_path),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-localvolumeresourcedata.html
    pub struct LocalVolumeResourceData_ {
        pub destination_path: crate::value::ExpString,
        pub group_owner_setting: Option<Box<GroupOwnerSetting_>>,
        pub source_path: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_ResourceDefinitionVersion_LocalVolumeResourceData {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::ResourceDefinitionVersion.LocalVolumeResourceData"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_ResourceDefinitionVersion_LocalVolumeResourceData as LocalVolumeResourceData;
    impl crate::value::ToValue for LocalVolumeResourceData_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DestinationPath".to_string(),
                crate::value::ToValue::to_value(&self.destination_path),
            );
            if let Some(ref value) = self.group_owner_setting {
                properties.insert(
                    "GroupOwnerSetting".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SourcePath".to_string(),
                crate::value::ToValue::to_value(&self.source_path),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-resourcedatacontainer.html
    pub struct ResourceDataContainer_ {
        pub local_device_resource_data: Option<Box<LocalDeviceResourceData_>>,
        pub local_volume_resource_data: Option<Box<LocalVolumeResourceData_>>,
        pub s3_machine_learning_model_resource_data:
            Option<Box<S3MachineLearningModelResourceData_>>,
        pub sage_maker_machine_learning_model_resource_data:
            Option<Box<SageMakerMachineLearningModelResourceData_>>,
        pub secrets_manager_secret_resource_data: Option<Box<SecretsManagerSecretResourceData_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_ResourceDefinitionVersion_ResourceDataContainer {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::ResourceDefinitionVersion.ResourceDataContainer"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_ResourceDefinitionVersion_ResourceDataContainer as ResourceDataContainer;
    impl crate::value::ToValue for ResourceDataContainer_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.local_device_resource_data {
                properties.insert(
                    "LocalDeviceResourceData".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.local_volume_resource_data {
                properties.insert(
                    "LocalVolumeResourceData".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_machine_learning_model_resource_data {
                properties.insert(
                    "S3MachineLearningModelResourceData".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sage_maker_machine_learning_model_resource_data {
                properties.insert(
                    "SageMakerMachineLearningModelResourceData".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secrets_manager_secret_resource_data {
                properties.insert(
                    "SecretsManagerSecretResourceData".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-resourcedownloadownersetting.html
    pub struct ResourceDownloadOwnerSetting_ {
        pub group_owner: crate::value::ExpString,
        pub group_permission: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_ResourceDefinitionVersion_ResourceDownloadOwnerSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::ResourceDefinitionVersion.ResourceDownloadOwnerSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_ResourceDefinitionVersion_ResourceDownloadOwnerSetting as ResourceDownloadOwnerSetting;
    impl crate::value::ToValue for ResourceDownloadOwnerSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "GroupOwner".to_string(),
                crate::value::ToValue::to_value(&self.group_owner),
            );
            properties.insert(
                "GroupPermission".to_string(),
                crate::value::ToValue::to_value(&self.group_permission),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-resourceinstance.html
    pub struct ResourceInstance_ {
        pub id: crate::value::ExpString,
        pub name: crate::value::ExpString,
        pub resource_data_container: Box<ResourceDataContainer_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_ResourceDefinitionVersion_ResourceInstance {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::ResourceDefinitionVersion.ResourceInstance"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_ResourceDefinitionVersion_ResourceInstance as ResourceInstance;
    impl crate::value::ToValue for ResourceInstance_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "ResourceDataContainer".to_string(),
                crate::value::ToValue::to_value(&self.resource_data_container),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-s3machinelearningmodelresourcedata.html
    pub struct S3MachineLearningModelResourceData_ {
        pub destination_path: crate::value::ExpString,
        pub owner_setting: Option<Box<ResourceDownloadOwnerSetting_>>,
        pub s3_uri: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_ResourceDefinitionVersion_S3MachineLearningModelResourceData {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::ResourceDefinitionVersion.S3MachineLearningModelResourceData"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_ResourceDefinitionVersion_S3MachineLearningModelResourceData as S3MachineLearningModelResourceData;
    impl crate::value::ToValue for S3MachineLearningModelResourceData_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DestinationPath".to_string(),
                crate::value::ToValue::to_value(&self.destination_path),
            );
            if let Some(ref value) = self.owner_setting {
                properties.insert(
                    "OwnerSetting".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "S3Uri".to_string(),
                crate::value::ToValue::to_value(&self.s3_uri),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-sagemakermachinelearningmodelresourcedata.html
    pub struct SageMakerMachineLearningModelResourceData_ {
        pub destination_path: crate::value::ExpString,
        pub owner_setting: Option<Box<ResourceDownloadOwnerSetting_>>,
        pub sage_maker_job_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_ResourceDefinitionVersion_SageMakerMachineLearningModelResourceData {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::ResourceDefinitionVersion.SageMakerMachineLearningModelResourceData"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_ResourceDefinitionVersion_SageMakerMachineLearningModelResourceData as SageMakerMachineLearningModelResourceData;
    impl crate::value::ToValue for SageMakerMachineLearningModelResourceData_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DestinationPath".to_string(),
                crate::value::ToValue::to_value(&self.destination_path),
            );
            if let Some(ref value) = self.owner_setting {
                properties.insert(
                    "OwnerSetting".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SageMakerJobArn".to_string(),
                crate::value::ToValue::to_value(&self.sage_maker_job_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-resourcedefinitionversion-secretsmanagersecretresourcedata.html
    pub struct SecretsManagerSecretResourceData_ {
        pub arn: crate::value::ExpString,
        pub additional_staging_labels_to_download: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_ResourceDefinitionVersion_SecretsManagerSecretResourceData {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::ResourceDefinitionVersion.SecretsManagerSecretResourceData"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_ResourceDefinitionVersion_SecretsManagerSecretResourceData as SecretsManagerSecretResourceData;
    impl crate::value::ToValue for SecretsManagerSecretResourceData_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ARN".to_string(),
                crate::value::ToValue::to_value(&self.arn),
            );
            if let Some(ref value) = self.additional_staging_labels_to_download {
                properties.insert(
                    "AdditionalStagingLabelsToDownload".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod subscriptiondefinition {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-subscriptiondefinition-subscription.html
    pub struct Subscription_ {
        pub id: crate::value::ExpString,
        pub source: crate::value::ExpString,
        pub subject: crate::value::ExpString,
        pub target: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_SubscriptionDefinition_Subscription {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::SubscriptionDefinition.Subscription"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_SubscriptionDefinition_Subscription as Subscription;
    impl crate::value::ToValue for Subscription_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            properties.insert(
                "Source".to_string(),
                crate::value::ToValue::to_value(&self.source),
            );
            properties.insert(
                "Subject".to_string(),
                crate::value::ToValue::to_value(&self.subject),
            );
            properties.insert(
                "Target".to_string(),
                crate::value::ToValue::to_value(&self.target),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-subscriptiondefinition-subscriptiondefinitionversion.html
    pub struct SubscriptionDefinitionVersion_ {
        pub subscriptions: Vec<Subscription_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_SubscriptionDefinition_SubscriptionDefinitionVersion {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::SubscriptionDefinition.SubscriptionDefinitionVersion"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_SubscriptionDefinition_SubscriptionDefinitionVersion as SubscriptionDefinitionVersion;
    impl crate::value::ToValue for SubscriptionDefinitionVersion_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Subscriptions".to_string(),
                crate::value::ToValue::to_value(&self.subscriptions),
            );
            properties.into()
        }
    }
}
pub mod subscriptiondefinitionversion {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrass-subscriptiondefinitionversion-subscription.html
    pub struct Subscription_ {
        pub id: crate::value::ExpString,
        pub source: crate::value::ExpString,
        pub subject: crate::value::ExpString,
        pub target: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrass_SubscriptionDefinitionVersion_Subscription {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Greengrass::SubscriptionDefinitionVersion.Subscription"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrass_SubscriptionDefinitionVersion_Subscription as Subscription;
    impl crate::value::ToValue for Subscription_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            properties.insert(
                "Source".to_string(),
                crate::value::ToValue::to_value(&self.source),
            );
            properties.insert(
                "Subject".to_string(),
                crate::value::ToValue::to_value(&self.subject),
            );
            properties.insert(
                "Target".to_string(),
                crate::value::ToValue::to_value(&self.target),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-connectordefinition.html
pub struct ConnectorDefinition_ {
    pub initial_version:
        Option<super::greengrass::connectordefinition::ConnectorDefinitionVersion_>,
    pub name: crate::value::ExpString,
    pub tags: Option<serde_json::Value>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_greengrass_ConnectorDefinition {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Greengrass::ConnectorDefinition"
        $($field $value)*)
    };
}
pub use crate::__aws_greengrass_ConnectorDefinition as ConnectorDefinition;
impl crate::template::ToResource for ConnectorDefinition_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Greengrass"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ConnectorDefinition"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.initial_version {
            properties.insert(
                "InitialVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-connectordefinitionversion.html
pub struct ConnectorDefinitionVersion_ {
    pub connector_definition_id: crate::value::ExpString,
    pub connectors: Vec<super::greengrass::connectordefinitionversion::Connector_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_greengrass_ConnectorDefinitionVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Greengrass::ConnectorDefinitionVersion"
        $($field $value)*)
    };
}
pub use crate::__aws_greengrass_ConnectorDefinitionVersion as ConnectorDefinitionVersion;
impl crate::template::ToResource for ConnectorDefinitionVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Greengrass"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "ConnectorDefinitionVersion",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ConnectorDefinitionId".to_string(),
            crate::value::ToValue::to_value(&self.connector_definition_id),
        );
        properties.insert(
            "Connectors".to_string(),
            crate::value::ToValue::to_value(&self.connectors),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-coredefinition.html
pub struct CoreDefinition_ {
    pub initial_version: Option<super::greengrass::coredefinition::CoreDefinitionVersion_>,
    pub name: crate::value::ExpString,
    pub tags: Option<serde_json::Value>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_greengrass_CoreDefinition {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Greengrass::CoreDefinition"
        $($field $value)*)
    };
}
pub use crate::__aws_greengrass_CoreDefinition as CoreDefinition;
impl crate::template::ToResource for CoreDefinition_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Greengrass"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("CoreDefinition"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.initial_version {
            properties.insert(
                "InitialVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-coredefinitionversion.html
pub struct CoreDefinitionVersion_ {
    pub core_definition_id: crate::value::ExpString,
    pub cores: Vec<super::greengrass::coredefinitionversion::Core_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_greengrass_CoreDefinitionVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Greengrass::CoreDefinitionVersion"
        $($field $value)*)
    };
}
pub use crate::__aws_greengrass_CoreDefinitionVersion as CoreDefinitionVersion;
impl crate::template::ToResource for CoreDefinitionVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Greengrass"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("CoreDefinitionVersion"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "CoreDefinitionId".to_string(),
            crate::value::ToValue::to_value(&self.core_definition_id),
        );
        properties.insert(
            "Cores".to_string(),
            crate::value::ToValue::to_value(&self.cores),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-devicedefinition.html
pub struct DeviceDefinition_ {
    pub initial_version: Option<super::greengrass::devicedefinition::DeviceDefinitionVersion_>,
    pub name: crate::value::ExpString,
    pub tags: Option<serde_json::Value>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_greengrass_DeviceDefinition {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Greengrass::DeviceDefinition"
        $($field $value)*)
    };
}
pub use crate::__aws_greengrass_DeviceDefinition as DeviceDefinition;
impl crate::template::ToResource for DeviceDefinition_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Greengrass"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DeviceDefinition"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.initial_version {
            properties.insert(
                "InitialVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-devicedefinitionversion.html
pub struct DeviceDefinitionVersion_ {
    pub device_definition_id: crate::value::ExpString,
    pub devices: Vec<super::greengrass::devicedefinitionversion::Device_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_greengrass_DeviceDefinitionVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Greengrass::DeviceDefinitionVersion"
        $($field $value)*)
    };
}
pub use crate::__aws_greengrass_DeviceDefinitionVersion as DeviceDefinitionVersion;
impl crate::template::ToResource for DeviceDefinitionVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Greengrass"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DeviceDefinitionVersion"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DeviceDefinitionId".to_string(),
            crate::value::ToValue::to_value(&self.device_definition_id),
        );
        properties.insert(
            "Devices".to_string(),
            crate::value::ToValue::to_value(&self.devices),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-functiondefinition.html
pub struct FunctionDefinition_ {
    pub initial_version: Option<super::greengrass::functiondefinition::FunctionDefinitionVersion_>,
    pub name: crate::value::ExpString,
    pub tags: Option<serde_json::Value>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_greengrass_FunctionDefinition {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Greengrass::FunctionDefinition"
        $($field $value)*)
    };
}
pub use crate::__aws_greengrass_FunctionDefinition as FunctionDefinition;
impl crate::template::ToResource for FunctionDefinition_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Greengrass"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("FunctionDefinition"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.initial_version {
            properties.insert(
                "InitialVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-functiondefinitionversion.html
pub struct FunctionDefinitionVersion_ {
    pub default_config: Option<super::greengrass::functiondefinitionversion::DefaultConfig_>,
    pub function_definition_id: crate::value::ExpString,
    pub functions: Vec<super::greengrass::functiondefinitionversion::Function_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_greengrass_FunctionDefinitionVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Greengrass::FunctionDefinitionVersion"
        $($field $value)*)
    };
}
pub use crate::__aws_greengrass_FunctionDefinitionVersion as FunctionDefinitionVersion;
impl crate::template::ToResource for FunctionDefinitionVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Greengrass"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("FunctionDefinitionVersion"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.default_config {
            properties.insert(
                "DefaultConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "FunctionDefinitionId".to_string(),
            crate::value::ToValue::to_value(&self.function_definition_id),
        );
        properties.insert(
            "Functions".to_string(),
            crate::value::ToValue::to_value(&self.functions),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-group.html
pub struct Group_ {
    pub initial_version: Option<super::greengrass::group::GroupVersion_>,
    pub name: crate::value::ExpString,
    pub role_arn: Option<crate::value::ExpString>,
    pub tags: Option<serde_json::Value>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_greengrass_Group {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Greengrass::Group"
        $($field $value)*)
    };
}
pub use crate::__aws_greengrass_Group as Group;
impl crate::template::ToResource for Group_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Greengrass"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Group"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.initial_version {
            properties.insert(
                "InitialVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.role_arn {
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-groupversion.html
pub struct GroupVersion_ {
    pub connector_definition_version_arn: Option<crate::value::ExpString>,
    pub core_definition_version_arn: Option<crate::value::ExpString>,
    pub device_definition_version_arn: Option<crate::value::ExpString>,
    pub function_definition_version_arn: Option<crate::value::ExpString>,
    pub group_id: crate::value::ExpString,
    pub logger_definition_version_arn: Option<crate::value::ExpString>,
    pub resource_definition_version_arn: Option<crate::value::ExpString>,
    pub subscription_definition_version_arn: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_greengrass_GroupVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Greengrass::GroupVersion"
        $($field $value)*)
    };
}
pub use crate::__aws_greengrass_GroupVersion as GroupVersion;
impl crate::template::ToResource for GroupVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Greengrass"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("GroupVersion"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.connector_definition_version_arn {
            properties.insert(
                "ConnectorDefinitionVersionArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.core_definition_version_arn {
            properties.insert(
                "CoreDefinitionVersionArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.device_definition_version_arn {
            properties.insert(
                "DeviceDefinitionVersionArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.function_definition_version_arn {
            properties.insert(
                "FunctionDefinitionVersionArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "GroupId".to_string(),
            crate::value::ToValue::to_value(&self.group_id),
        );
        if let Some(ref value) = self.logger_definition_version_arn {
            properties.insert(
                "LoggerDefinitionVersionArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resource_definition_version_arn {
            properties.insert(
                "ResourceDefinitionVersionArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.subscription_definition_version_arn {
            properties.insert(
                "SubscriptionDefinitionVersionArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-loggerdefinition.html
pub struct LoggerDefinition_ {
    pub initial_version: Option<super::greengrass::loggerdefinition::LoggerDefinitionVersion_>,
    pub name: crate::value::ExpString,
    pub tags: Option<serde_json::Value>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_greengrass_LoggerDefinition {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Greengrass::LoggerDefinition"
        $($field $value)*)
    };
}
pub use crate::__aws_greengrass_LoggerDefinition as LoggerDefinition;
impl crate::template::ToResource for LoggerDefinition_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Greengrass"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LoggerDefinition"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.initial_version {
            properties.insert(
                "InitialVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-loggerdefinitionversion.html
pub struct LoggerDefinitionVersion_ {
    pub logger_definition_id: crate::value::ExpString,
    pub loggers: Vec<super::greengrass::loggerdefinitionversion::Logger_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_greengrass_LoggerDefinitionVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Greengrass::LoggerDefinitionVersion"
        $($field $value)*)
    };
}
pub use crate::__aws_greengrass_LoggerDefinitionVersion as LoggerDefinitionVersion;
impl crate::template::ToResource for LoggerDefinitionVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Greengrass"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LoggerDefinitionVersion"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "LoggerDefinitionId".to_string(),
            crate::value::ToValue::to_value(&self.logger_definition_id),
        );
        properties.insert(
            "Loggers".to_string(),
            crate::value::ToValue::to_value(&self.loggers),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-resourcedefinition.html
pub struct ResourceDefinition_ {
    pub initial_version: Option<super::greengrass::resourcedefinition::ResourceDefinitionVersion_>,
    pub name: crate::value::ExpString,
    pub tags: Option<serde_json::Value>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_greengrass_ResourceDefinition {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Greengrass::ResourceDefinition"
        $($field $value)*)
    };
}
pub use crate::__aws_greengrass_ResourceDefinition as ResourceDefinition;
impl crate::template::ToResource for ResourceDefinition_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Greengrass"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ResourceDefinition"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.initial_version {
            properties.insert(
                "InitialVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-resourcedefinitionversion.html
pub struct ResourceDefinitionVersion_ {
    pub resource_definition_id: crate::value::ExpString,
    pub resources: Vec<super::greengrass::resourcedefinitionversion::ResourceInstance_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_greengrass_ResourceDefinitionVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Greengrass::ResourceDefinitionVersion"
        $($field $value)*)
    };
}
pub use crate::__aws_greengrass_ResourceDefinitionVersion as ResourceDefinitionVersion;
impl crate::template::ToResource for ResourceDefinitionVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Greengrass"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ResourceDefinitionVersion"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ResourceDefinitionId".to_string(),
            crate::value::ToValue::to_value(&self.resource_definition_id),
        );
        properties.insert(
            "Resources".to_string(),
            crate::value::ToValue::to_value(&self.resources),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-subscriptiondefinition.html
pub struct SubscriptionDefinition_ {
    pub initial_version:
        Option<super::greengrass::subscriptiondefinition::SubscriptionDefinitionVersion_>,
    pub name: crate::value::ExpString,
    pub tags: Option<serde_json::Value>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_greengrass_SubscriptionDefinition {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Greengrass::SubscriptionDefinition"
        $($field $value)*)
    };
}
pub use crate::__aws_greengrass_SubscriptionDefinition as SubscriptionDefinition;
impl crate::template::ToResource for SubscriptionDefinition_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Greengrass"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SubscriptionDefinition"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.initial_version {
            properties.insert(
                "InitialVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrass-subscriptiondefinitionversion.html
pub struct SubscriptionDefinitionVersion_ {
    pub subscription_definition_id: crate::value::ExpString,
    pub subscriptions: Vec<super::greengrass::subscriptiondefinitionversion::Subscription_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_greengrass_SubscriptionDefinitionVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Greengrass::SubscriptionDefinitionVersion"
        $($field $value)*)
    };
}
pub use crate::__aws_greengrass_SubscriptionDefinitionVersion as SubscriptionDefinitionVersion;
impl crate::template::ToResource for SubscriptionDefinitionVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Greengrass"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "SubscriptionDefinitionVersion",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "SubscriptionDefinitionId".to_string(),
            crate::value::ToValue::to_value(&self.subscription_definition_id),
        );
        properties.insert(
            "Subscriptions".to_string(),
            crate::value::ToValue::to_value(&self.subscriptions),
        );
        properties
    }
}
