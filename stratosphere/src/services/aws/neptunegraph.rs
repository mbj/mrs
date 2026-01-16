pub mod graph {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-neptunegraph-graph-vectorsearchconfiguration.html
    pub struct VectorSearchConfiguration_ {
        pub vector_search_dimension: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_neptunegraph_Graph_VectorSearchConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::NeptuneGraph::Graph.VectorSearchConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_neptunegraph_Graph_VectorSearchConfiguration as VectorSearchConfiguration;
    impl crate::value::ToValue for VectorSearchConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "VectorSearchDimension".to_string(),
                crate::value::ToValue::to_value(&self.vector_search_dimension),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-neptunegraph-graph.html
pub struct Graph_ {
    pub deletion_protection: Option<crate::value::ExpBool>,
    pub graph_name: Option<crate::value::ExpString>,
    pub provisioned_memory: i64,
    pub public_connectivity: Option<crate::value::ExpBool>,
    pub replica_count: Option<i64>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vector_search_configuration: Option<super::neptunegraph::graph::VectorSearchConfiguration_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_neptunegraph_Graph {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::NeptuneGraph::Graph"
        $($field $value)*)
    };
}
pub use crate::__aws_neptunegraph_Graph as Graph;
impl crate::template::ToResource for Graph_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("NeptuneGraph"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Graph"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.deletion_protection {
            properties.insert(
                "DeletionProtection".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.graph_name {
            properties.insert(
                "GraphName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ProvisionedMemory".to_string(),
            crate::value::ToValue::to_value(&self.provisioned_memory),
        );
        if let Some(ref value) = self.public_connectivity {
            properties.insert(
                "PublicConnectivity".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.replica_count {
            properties.insert(
                "ReplicaCount".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.vector_search_configuration {
            properties.insert(
                "VectorSearchConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-neptunegraph-privategraphendpoint.html
pub struct PrivateGraphEndpoint_ {
    pub graph_identifier: crate::value::ExpString,
    pub security_group_ids: Option<Vec<crate::value::ExpString>>,
    pub subnet_ids: Option<Vec<crate::value::ExpString>>,
    pub vpc_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_neptunegraph_PrivateGraphEndpoint {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::NeptuneGraph::PrivateGraphEndpoint"
        $($field $value)*)
    };
}
pub use crate::__aws_neptunegraph_PrivateGraphEndpoint as PrivateGraphEndpoint;
impl crate::template::ToResource for PrivateGraphEndpoint_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("NeptuneGraph"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PrivateGraphEndpoint"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "GraphIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.graph_identifier),
        );
        if let Some(ref value) = self.security_group_ids {
            properties.insert(
                "SecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.subnet_ids {
            properties.insert(
                "SubnetIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "VpcId".to_string(),
            crate::value::ToValue::to_value(&self.vpc_id),
        );
        properties
    }
}
