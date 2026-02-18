///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdbelastic-cluster.html>
pub struct Cluster_ {
    pub admin_user_name: crate::value::ExpString,
    pub admin_user_password: Option<crate::value::ExpString>,
    pub auth_type: crate::value::ExpString,
    pub backup_retention_period: Option<i32>,
    pub cluster_name: crate::value::ExpString,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub preferred_backup_window: Option<crate::value::ExpString>,
    pub preferred_maintenance_window: Option<crate::value::ExpString>,
    pub shard_capacity: i32,
    pub shard_count: i32,
    pub shard_instance_count: Option<i32>,
    pub subnet_ids: Option<Vec<crate::value::ExpString>>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_security_group_ids: Option<Vec<crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_docdbelastic_Cluster {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DocDBElastic::Cluster"
        $($field $value)*)
    };
}
pub use crate::__aws_docdbelastic_Cluster as Cluster;
impl crate::template::ToResource for Cluster_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DocDBElastic"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Cluster"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AdminUserName".to_string(),
            crate::value::ToValue::to_value(&self.admin_user_name),
        );
        if let Some(ref value) = self.admin_user_password {
            properties.insert(
                "AdminUserPassword".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "AuthType".to_string(),
            crate::value::ToValue::to_value(&self.auth_type),
        );
        if let Some(ref value) = self.backup_retention_period {
            properties.insert(
                "BackupRetentionPeriod".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ClusterName".to_string(),
            crate::value::ToValue::to_value(&self.cluster_name),
        );
        if let Some(ref value) = self.kms_key_id {
            properties.insert(
                "KmsKeyId".to_string(),
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
        properties.insert(
            "ShardCapacity".to_string(),
            crate::value::ToValue::to_value(&self.shard_capacity),
        );
        properties.insert(
            "ShardCount".to_string(),
            crate::value::ToValue::to_value(&self.shard_count),
        );
        if let Some(ref value) = self.shard_instance_count {
            properties.insert(
                "ShardInstanceCount".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.subnet_ids {
            properties.insert(
                "SubnetIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.vpc_security_group_ids {
            properties.insert(
                "VpcSecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
