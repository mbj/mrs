pub mod connectionalias {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspaces-connectionalias-connectionaliasassociation.html
    pub struct ConnectionAliasAssociation_ {
        pub associated_account_id: Option<crate::value::ExpString>,
        pub association_status: Option<crate::value::ExpString>,
        pub connection_identifier: Option<crate::value::ExpString>,
        pub resource_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspaces_ConnectionAlias_ConnectionAliasAssociation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::WorkSpaces::ConnectionAlias.ConnectionAliasAssociation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspaces_ConnectionAlias_ConnectionAliasAssociation as ConnectionAliasAssociation;
    impl crate::value::ToValue for ConnectionAliasAssociation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.associated_account_id {
                properties.insert(
                    "AssociatedAccountId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.association_status {
                properties.insert(
                    "AssociationStatus".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.connection_identifier {
                properties.insert(
                    "ConnectionIdentifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_id {
                properties.insert(
                    "ResourceId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod workspace {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspaces-workspace-workspaceproperties.html
    pub struct WorkspaceProperties_ {
        pub compute_type_name: Option<crate::value::ExpString>,
        pub root_volume_size_gib: Option<i64>,
        pub running_mode: Option<crate::value::ExpString>,
        pub running_mode_auto_stop_timeout_in_minutes: Option<i64>,
        pub user_volume_size_gib: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspaces_Workspace_WorkspaceProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::WorkSpaces::Workspace.WorkspaceProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspaces_Workspace_WorkspaceProperties as WorkspaceProperties;
    impl crate::value::ToValue for WorkspaceProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.compute_type_name {
                properties.insert(
                    "ComputeTypeName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.root_volume_size_gib {
                properties.insert(
                    "RootVolumeSizeGib".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.running_mode {
                properties.insert(
                    "RunningMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.running_mode_auto_stop_timeout_in_minutes {
                properties.insert(
                    "RunningModeAutoStopTimeoutInMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_volume_size_gib {
                properties.insert(
                    "UserVolumeSizeGib".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod workspacespool {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspaces-workspacespool-applicationsettings.html
    pub struct ApplicationSettings_ {
        pub settings_group: Option<crate::value::ExpString>,
        pub status: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspaces_WorkspacesPool_ApplicationSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::WorkSpaces::WorkspacesPool.ApplicationSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspaces_WorkspacesPool_ApplicationSettings as ApplicationSettings;
    impl crate::value::ToValue for ApplicationSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.settings_group {
                properties.insert(
                    "SettingsGroup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Status".to_string(),
                crate::value::ToValue::to_value(&self.status),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspaces-workspacespool-capacity.html
    pub struct Capacity_ {
        pub desired_user_sessions: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspaces_WorkspacesPool_Capacity {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::WorkSpaces::WorkspacesPool.Capacity"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspaces_WorkspacesPool_Capacity as Capacity;
    impl crate::value::ToValue for Capacity_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DesiredUserSessions".to_string(),
                crate::value::ToValue::to_value(&self.desired_user_sessions),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspaces-workspacespool-timeoutsettings.html
    pub struct TimeoutSettings_ {
        pub disconnect_timeout_in_seconds: Option<i64>,
        pub idle_disconnect_timeout_in_seconds: Option<i64>,
        pub max_user_duration_in_seconds: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspaces_WorkspacesPool_TimeoutSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::WorkSpaces::WorkspacesPool.TimeoutSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspaces_WorkspacesPool_TimeoutSettings as TimeoutSettings;
    impl crate::value::ToValue for TimeoutSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.disconnect_timeout_in_seconds {
                properties.insert(
                    "DisconnectTimeoutInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.idle_disconnect_timeout_in_seconds {
                properties.insert(
                    "IdleDisconnectTimeoutInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_user_duration_in_seconds {
                properties.insert(
                    "MaxUserDurationInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspaces-connectionalias.html
pub struct ConnectionAlias_ {
    pub connection_string: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_workspaces_ConnectionAlias {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::WorkSpaces::ConnectionAlias"
        $($field $value)*)
    };
}
pub use crate::__aws_workspaces_ConnectionAlias as ConnectionAlias;
impl crate::template::ToResource for ConnectionAlias_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WorkSpaces"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ConnectionAlias"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ConnectionString".to_string(),
            crate::value::ToValue::to_value(&self.connection_string),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspaces-workspace.html
pub struct Workspace_ {
    pub bundle_id: crate::value::ExpString,
    pub directory_id: crate::value::ExpString,
    pub root_volume_encryption_enabled: Option<crate::value::ExpBool>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub user_name: crate::value::ExpString,
    pub user_volume_encryption_enabled: Option<crate::value::ExpBool>,
    pub volume_encryption_key: Option<crate::value::ExpString>,
    pub workspace_properties: Option<super::workspaces::workspace::WorkspaceProperties_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_workspaces_Workspace {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::WorkSpaces::Workspace"
        $($field $value)*)
    };
}
pub use crate::__aws_workspaces_Workspace as Workspace;
impl crate::template::ToResource for Workspace_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WorkSpaces"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Workspace"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "BundleId".to_string(),
            crate::value::ToValue::to_value(&self.bundle_id),
        );
        properties.insert(
            "DirectoryId".to_string(),
            crate::value::ToValue::to_value(&self.directory_id),
        );
        if let Some(ref value) = self.root_volume_encryption_enabled {
            properties.insert(
                "RootVolumeEncryptionEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "UserName".to_string(),
            crate::value::ToValue::to_value(&self.user_name),
        );
        if let Some(ref value) = self.user_volume_encryption_enabled {
            properties.insert(
                "UserVolumeEncryptionEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.volume_encryption_key {
            properties.insert(
                "VolumeEncryptionKey".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.workspace_properties {
            properties.insert(
                "WorkspaceProperties".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspaces-workspacespool.html
pub struct WorkspacesPool_ {
    pub application_settings: Option<super::workspaces::workspacespool::ApplicationSettings_>,
    pub bundle_id: crate::value::ExpString,
    pub capacity: super::workspaces::workspacespool::Capacity_,
    pub description: Option<crate::value::ExpString>,
    pub directory_id: crate::value::ExpString,
    pub pool_name: crate::value::ExpString,
    pub running_mode: Option<crate::value::ExpString>,
    pub timeout_settings: Option<super::workspaces::workspacespool::TimeoutSettings_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_workspaces_WorkspacesPool {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::WorkSpaces::WorkspacesPool"
        $($field $value)*)
    };
}
pub use crate::__aws_workspaces_WorkspacesPool as WorkspacesPool;
impl crate::template::ToResource for WorkspacesPool_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WorkSpaces"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("WorkspacesPool"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.application_settings {
            properties.insert(
                "ApplicationSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "BundleId".to_string(),
            crate::value::ToValue::to_value(&self.bundle_id),
        );
        properties.insert(
            "Capacity".to_string(),
            crate::value::ToValue::to_value(&self.capacity),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DirectoryId".to_string(),
            crate::value::ToValue::to_value(&self.directory_id),
        );
        properties.insert(
            "PoolName".to_string(),
            crate::value::ToValue::to_value(&self.pool_name),
        );
        if let Some(ref value) = self.running_mode {
            properties.insert(
                "RunningMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.timeout_settings {
            properties.insert(
                "TimeoutSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
