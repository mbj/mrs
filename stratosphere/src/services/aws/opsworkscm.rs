pub mod server {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworkscm-server-engineattribute.html
    pub struct EngineAttribute_ {
        pub name: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opsworkscm_Server_EngineAttribute {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::OpsWorksCM::Server.EngineAttribute"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opsworkscm_Server_EngineAttribute as EngineAttribute;
    impl crate::value::ToValue for EngineAttribute_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworkscm-server.html
pub struct Server_ {
    pub associate_public_ip_address: Option<crate::value::ExpBool>,
    pub backup_id: Option<crate::value::ExpString>,
    pub backup_retention_count: Option<i64>,
    pub custom_certificate: Option<crate::value::ExpString>,
    pub custom_domain: Option<crate::value::ExpString>,
    pub custom_private_key: Option<crate::value::ExpString>,
    pub disable_automated_backup: Option<crate::value::ExpBool>,
    pub engine: Option<crate::value::ExpString>,
    pub engine_attributes: Option<Vec<super::opsworkscm::server::EngineAttribute_>>,
    pub engine_model: Option<crate::value::ExpString>,
    pub engine_version: Option<crate::value::ExpString>,
    pub instance_profile_arn: crate::value::ExpString,
    pub instance_type: crate::value::ExpString,
    pub key_pair: Option<crate::value::ExpString>,
    pub preferred_backup_window: Option<crate::value::ExpString>,
    pub preferred_maintenance_window: Option<crate::value::ExpString>,
    pub security_group_ids: Option<Vec<crate::value::ExpString>>,
    pub server_name: Option<crate::value::ExpString>,
    pub service_role_arn: crate::value::ExpString,
    pub subnet_ids: Option<Vec<crate::value::ExpString>>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_opsworkscm_Server {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::OpsWorksCM::Server"
        $($field $value)*)
    };
}
pub use crate::__aws_opsworkscm_Server as Server;
impl crate::template::ToResource for Server_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("OpsWorksCM"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Server"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.associate_public_ip_address {
            properties.insert(
                "AssociatePublicIpAddress".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.backup_id {
            properties.insert(
                "BackupId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.backup_retention_count {
            properties.insert(
                "BackupRetentionCount".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.custom_certificate {
            properties.insert(
                "CustomCertificate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.custom_domain {
            properties.insert(
                "CustomDomain".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.custom_private_key {
            properties.insert(
                "CustomPrivateKey".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.disable_automated_backup {
            properties.insert(
                "DisableAutomatedBackup".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.engine {
            properties.insert("Engine".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.engine_attributes {
            properties.insert(
                "EngineAttributes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.engine_model {
            properties.insert(
                "EngineModel".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.engine_version {
            properties.insert(
                "EngineVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "InstanceProfileArn".to_string(),
            crate::value::ToValue::to_value(&self.instance_profile_arn),
        );
        properties.insert(
            "InstanceType".to_string(),
            crate::value::ToValue::to_value(&self.instance_type),
        );
        if let Some(ref value) = self.key_pair {
            properties.insert(
                "KeyPair".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.preferred_backup_window {
            properties.insert(
                "PreferredBackupWindow".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.preferred_maintenance_window {
            properties.insert(
                "PreferredMaintenanceWindow".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.security_group_ids {
            properties.insert(
                "SecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.server_name {
            properties.insert(
                "ServerName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ServiceRoleArn".to_string(),
            crate::value::ToValue::to_value(&self.service_role_arn),
        );
        if let Some(ref value) = self.subnet_ids {
            properties.insert(
                "SubnetIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
