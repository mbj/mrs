pub mod cluster {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dax-cluster-ssespecification.html
    pub struct SSESpecification_ {
        pub sse_enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dax_Cluster_SSESpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DAX::Cluster.SSESpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dax_Cluster_SSESpecification as SSESpecification;
    impl crate::value::ToValue for SSESpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.sse_enabled {
                properties.insert(
                    "SSEEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dax-cluster.html
pub struct Cluster_ {
    pub availability_zones: Option<Vec<crate::value::ExpString>>,
    pub cluster_endpoint_encryption_type: Option<crate::value::ExpString>,
    pub cluster_name: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub iam_role_arn: crate::value::ExpString,
    pub network_type: Option<crate::value::ExpString>,
    pub node_type: crate::value::ExpString,
    pub notification_topic_arn: Option<crate::value::ExpString>,
    pub parameter_group_name: Option<crate::value::ExpString>,
    pub preferred_maintenance_window: Option<crate::value::ExpString>,
    pub replication_factor: i64,
    pub sse_specification: Option<super::dax::cluster::SSESpecification_>,
    pub security_group_ids: Option<Vec<crate::value::ExpString>>,
    pub subnet_group_name: Option<crate::value::ExpString>,
    pub tags: Option<serde_json::Value>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_dax_Cluster {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DAX::Cluster" $($field
        $value)*)
    };
}
pub use crate::__aws_dax_Cluster as Cluster;
impl crate::template::ToResource for Cluster_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DAX"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Cluster"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.availability_zones {
            properties.insert(
                "AvailabilityZones".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cluster_endpoint_encryption_type {
            properties.insert(
                "ClusterEndpointEncryptionType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cluster_name {
            properties.insert(
                "ClusterName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "IAMRoleARN".to_string(),
            crate::value::ToValue::to_value(&self.iam_role_arn),
        );
        if let Some(ref value) = self.network_type {
            properties.insert(
                "NetworkType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "NodeType".to_string(),
            crate::value::ToValue::to_value(&self.node_type),
        );
        if let Some(ref value) = self.notification_topic_arn {
            properties.insert(
                "NotificationTopicARN".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.parameter_group_name {
            properties.insert(
                "ParameterGroupName".to_string(),
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
            "ReplicationFactor".to_string(),
            crate::value::ToValue::to_value(&self.replication_factor),
        );
        if let Some(ref value) = self.sse_specification {
            properties.insert(
                "SSESpecification".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.security_group_ids {
            properties.insert(
                "SecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.subnet_group_name {
            properties.insert(
                "SubnetGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dax-parametergroup.html
pub struct ParameterGroup_ {
    pub description: Option<crate::value::ExpString>,
    pub parameter_group_name: Option<crate::value::ExpString>,
    pub parameter_name_values: Option<serde_json::Value>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_dax_ParameterGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DAX::ParameterGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_dax_ParameterGroup as ParameterGroup;
impl crate::template::ToResource for ParameterGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DAX"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ParameterGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.parameter_group_name {
            properties.insert(
                "ParameterGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.parameter_name_values {
            properties.insert(
                "ParameterNameValues".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dax-subnetgroup.html
pub struct SubnetGroup_ {
    pub description: Option<crate::value::ExpString>,
    pub subnet_group_name: Option<crate::value::ExpString>,
    pub subnet_ids: Vec<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_dax_SubnetGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DAX::SubnetGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_dax_SubnetGroup as SubnetGroup;
impl crate::template::ToResource for SubnetGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DAX"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SubnetGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.subnet_group_name {
            properties.insert(
                "SubnetGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SubnetIds".to_string(),
            crate::value::ToValue::to_value(&self.subnet_ids),
        );
        properties
    }
}
