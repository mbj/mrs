use crate::resource_specification::ResourceTypeName;
use crate::value;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize)]
pub struct LogicalResourceName(pub String);

impl From<&str> for LogicalResourceName {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<&LogicalResourceName> for LogicalResourceName {
    fn from(value: &Self) -> Self {
        value.clone()
    }
}

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

#[derive(Debug, PartialEq, serde::Serialize)]
pub struct Resource<'a> {
    #[serde(rename = "Type")]
    resource_type_identifier: ResourceTypeName<'a>,
    #[serde(rename = "Properties")]
    resource_properties: ResourceProperties,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Version {
    V2010_09_09,
}

impl serde::Serialize for Version {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Version::V2010_09_09 => serializer.serialize_str("2010-09-09"),
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize)]
pub struct ParameterKey(pub String);

impl std::fmt::Display for ParameterKey {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(formatter, "{}", self.0)
    }
}

impl From<&str> for ParameterKey {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl From<ParameterKey> for value::ExpString {
    fn from(value: ParameterKey) -> Self {
        (&value).into()
    }
}

impl From<&ParameterKey> for value::ExpString {
    fn from(value: &ParameterKey) -> Self {
        Self::Ref(value.0.as_str().into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub enum ParameterType {
    String,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub struct Parameter {
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Type")]
    pub r#type: ParameterType,
    #[serde(rename = "AllowedPattern", skip_serializing_if = "Option::is_none")]
    pub allowed_pattern: Option<String>,
}

pub type ParameterKeys = std::collections::BTreeSet<ParameterKey>;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize)]
pub struct OutputKey(pub String);

impl std::fmt::Display for OutputKey {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(formatter, "{}", self.0)
    }
}

impl From<&str> for OutputKey {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl OutputKey {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize)]
pub struct MapName(pub String);

impl std::fmt::Display for MapName {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(formatter, "{}", self.0)
    }
}

impl From<&str> for MapName {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl From<&MapName> for MapName {
    fn from(value: &MapName) -> Self {
        value.clone()
    }
}

impl From<MapName> for String {
    fn from(value: MapName) -> Self {
        value.0
    }
}

impl From<&MapName> for String {
    fn from(value: &MapName) -> Self {
        value.0.clone()
    }
}

impl MapName {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub type Mapping =
    std::collections::BTreeMap<String, std::collections::BTreeMap<String, serde_json::Value>>;

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct Output {
    #[serde(rename = "Description")]
    pub description: value::ExpString,
    #[serde(rename = "Value")]
    pub value: value::ExpString,
    #[serde(rename = "Export", skip_serializing_if = "Option::is_none")]
    pub export: Option<OutputExport>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct OutputExport {
    #[serde(rename = "Name")]
    pub name: value::ExpString,
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<value::ExpString>,
}

#[derive(Debug, PartialEq, serde::Serialize)]
pub struct Template<'a> {
    #[serde(rename = "AWSTemplateFormatVersion")]
    version: Version,
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(
        rename = "Mappings",
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    mappings: std::collections::BTreeMap<MapName, Mapping>,
    #[serde(
        rename = "Outputs",
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    outputs: std::collections::BTreeMap<OutputKey, Output>,
    #[serde(
        rename = "Parameters",
        skip_serializing_if = "std::collections::BTreeMap::is_empty"
    )]
    parameters: std::collections::BTreeMap<ParameterKey, Parameter>,
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
            description: None,
            mappings: std::collections::BTreeMap::new(),
            outputs: std::collections::BTreeMap::new(),
            parameters: std::collections::BTreeMap::new(),
            resources: std::collections::BTreeMap::new(),
            version: Version::V2010_09_09,
        }
    }

    pub fn build(action: fn(&mut Self) -> ()) -> Self {
        let mut template = Self::new();

        action(&mut template);

        template
    }

    pub fn resource<R: ToResource>(
        &mut self,
        logical_resource_name: impl Into<LogicalResourceName>,
        resource: R,
    ) -> LogicalResourceName {
        let logical_resource_name = logical_resource_name.into();

        let resource = Resource {
            resource_type_identifier: R::RESOURCE_TYPE_NAME,
            resource_properties: resource.to_resource_properties(),
        };

        match self
            .resources
            .insert(logical_resource_name.clone(), resource)
        {
            None => logical_resource_name,
            Some(_existing) => {
                panic!("Logical resource with name: {logical_resource_name} already exists")
            }
        }
    }

    pub fn parameter(
        &mut self,
        parameter_key: impl Into<ParameterKey>,
        parameter: Parameter,
    ) -> ParameterKey {
        let parameter_key = parameter_key.into();

        if let Some(_existing) = self.parameters.insert(parameter_key.clone(), parameter) {
            panic!("Parameter key: {parameter_key} already exists")
        }

        parameter_key
    }

    pub fn parameter_(
        mut self,
        parameter_key: impl Into<ParameterKey>,
        parameter: Parameter,
    ) -> Self {
        self.parameter(parameter_key, parameter);
        self
    }

    pub fn mapping(&mut self, map_name: impl Into<MapName>, mapping: Mapping) -> MapName {
        let map_name = map_name.into();

        if let Some(_existing) = self.mappings.insert(map_name.clone(), mapping) {
            panic!("Mapping with name: {map_name} already exists")
        }

        map_name
    }

    pub fn mapping_(mut self, map_name: impl Into<MapName>, mapping: Mapping) -> Self {
        self.mapping(map_name, mapping);
        self
    }

    pub fn output(&mut self, output_key: impl Into<OutputKey>, output: Output) -> OutputKey {
        let output_key = output_key.into();

        if let Some(_old) = self.outputs.insert(output_key.clone(), output) {
            panic!("Output with name: {output_key} already exists")
        }

        output_key
    }

    pub fn output_(mut self, output_key: impl Into<OutputKey>, output: Output) -> Self {
        self.output(output_key, output);
        self
    }

    pub fn output_export(
        &mut self,
        output_key: impl Into<OutputKey>,
        description: &str,
        value: impl Into<value::ExpString>,
    ) -> OutputKey {
        let output_key = output_key.into();

        self.output(
            output_key.clone(),
            Output {
                description: description.into(),
                value: value.into(),
                export: Some(OutputExport {
                    name: value::join(":", [value::AWS_STACK_NAME, output_key.as_str().into()]),
                    value: None,
                }),
            },
        )
    }

    pub fn resource_<R: ToResource>(
        mut self,
        logical_resource_name: impl Into<LogicalResourceName>,
        resource: R,
    ) -> Self {
        self.resource(logical_resource_name, resource);
        self
    }

    pub fn render_json_pretty(&self) -> String {
        let mut string = serde_json::to_string_pretty(&self).unwrap();
        string.push('\n');
        string
    }

    pub fn render_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn parameter_keys(&self) -> std::collections::BTreeSet<ParameterKey> {
        self.parameters.keys().cloned().collect()
    }
}
