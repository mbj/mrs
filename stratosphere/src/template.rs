use crate::resource_specification::ResourceTypeName;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize)]
pub struct LogicalResourceName(pub String);

impl std::fmt::Display for LogicalResourceName {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(formatter, "{}", self.0)
    }
}

pub type ResourceProperties = serde_json::Map<String, serde_json::Value>;

pub trait ToResource {
    const RESOURCE_TYPE_NAME: ResourceTypeName<'static>;
    fn to_resource_properties(&self) -> ResourceProperties;
}

#[derive(serde::Serialize)]
pub struct Resource<'a> {
    #[serde(rename = "Type")]
    resource_type_name: ResourceTypeName<'a>,
    #[serde(rename = "Properties")]
    resource_properties: ResourceProperties,
}

trait ToLogicalResourceName {
    fn to_logical_resource_name(&self) -> LogicalResourceName;
}

impl ToLogicalResourceName for &str {
    fn to_logical_resource_name(&self) -> LogicalResourceName {
        LogicalResourceName(self.to_string())
    }
}

enum Version {
    V2010_09_09,
}

impl serde::Serialize for Version {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Version::V2010_09_09 => serializer.serialize_str("2010-09-09"),
        }
    }
}

#[derive(serde::Serialize)]
pub struct Template<'a> {
    #[serde(rename = "Version")]
    version: Version,
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(rename = "Resources")]
    resources: std::collections::BTreeMap<LogicalResourceName, Resource<'a>>,
}

impl Default for Template<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl Template<'_> {
    pub fn new() -> Self {
        Self {
            version: Version::V2010_09_09,
            description: None,
            resources: std::collections::BTreeMap::new(),
        }
    }

    pub fn resource<R: ToResource>(
        mut self,
        logical_resource_name: impl ToLogicalResourceName,
        resource: R,
    ) -> Self {
        let logical_resource_name = logical_resource_name.to_logical_resource_name();

        let resource = Resource {
            resource_type_name: R::RESOURCE_TYPE_NAME,
            resource_properties: resource.to_resource_properties(),
        };

        match self
            .resources
            .insert(logical_resource_name.clone(), resource)
        {
            None => self,
            Some(_existing) => {
                panic!("Logical resource with name: {logical_resource_name} already exists")
            }
        }
    }
}
