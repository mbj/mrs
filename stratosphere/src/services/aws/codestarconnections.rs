///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarconnections-connection.html
pub struct Connection_ {
    pub connection_name: crate::value::ExpString,
    pub host_arn: Option<crate::value::ExpString>,
    pub provider_type: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_codestarconnections_Connection {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CodeStarConnections::Connection"
        $($field $value)*)
    };
}
pub use crate::__aws_codestarconnections_Connection as Connection;
impl crate::template::ToResource for Connection_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CodeStarConnections"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Connection"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ConnectionName".to_string(),
            crate::value::ToValue::to_value(&self.connection_name),
        );
        if let Some(ref value) = self.host_arn {
            properties.insert(
                "HostArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.provider_type {
            properties.insert(
                "ProviderType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarconnections-repositorylink.html
pub struct RepositoryLink_ {
    pub connection_arn: crate::value::ExpString,
    pub encryption_key_arn: Option<crate::value::ExpString>,
    pub owner_id: crate::value::ExpString,
    pub repository_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_codestarconnections_RepositoryLink {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CodeStarConnections::RepositoryLink"
        $($field $value)*)
    };
}
pub use crate::__aws_codestarconnections_RepositoryLink as RepositoryLink;
impl crate::template::ToResource for RepositoryLink_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CodeStarConnections"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RepositoryLink"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ConnectionArn".to_string(),
            crate::value::ToValue::to_value(&self.connection_arn),
        );
        if let Some(ref value) = self.encryption_key_arn {
            properties.insert(
                "EncryptionKeyArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "OwnerId".to_string(),
            crate::value::ToValue::to_value(&self.owner_id),
        );
        properties.insert(
            "RepositoryName".to_string(),
            crate::value::ToValue::to_value(&self.repository_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarconnections-syncconfiguration.html
pub struct SyncConfiguration_ {
    pub branch: crate::value::ExpString,
    pub config_file: crate::value::ExpString,
    pub publish_deployment_status: Option<crate::value::ExpString>,
    pub repository_link_id: crate::value::ExpString,
    pub resource_name: crate::value::ExpString,
    pub role_arn: crate::value::ExpString,
    pub sync_type: crate::value::ExpString,
    pub trigger_resource_update_on: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_codestarconnections_SyncConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CodeStarConnections::SyncConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_codestarconnections_SyncConfiguration as SyncConfiguration;
impl crate::template::ToResource for SyncConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CodeStarConnections"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SyncConfiguration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Branch".to_string(),
            crate::value::ToValue::to_value(&self.branch),
        );
        properties.insert(
            "ConfigFile".to_string(),
            crate::value::ToValue::to_value(&self.config_file),
        );
        if let Some(ref value) = self.publish_deployment_status {
            properties.insert(
                "PublishDeploymentStatus".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RepositoryLinkId".to_string(),
            crate::value::ToValue::to_value(&self.repository_link_id),
        );
        properties.insert(
            "ResourceName".to_string(),
            crate::value::ToValue::to_value(&self.resource_name),
        );
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        properties.insert(
            "SyncType".to_string(),
            crate::value::ToValue::to_value(&self.sync_type),
        );
        if let Some(ref value) = self.trigger_resource_update_on {
            properties.insert(
                "TriggerResourceUpdateOn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
