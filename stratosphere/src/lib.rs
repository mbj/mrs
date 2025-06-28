#![feature(default_field_values)]

use serde::ser::SerializeMap;

pub mod ec2;
pub mod value;

use crate::value::*;

stratosphere_generator::gen_service_enum!();

struct ServiceResourceType(&'static str);

struct ResourceType {
    service: Service,
    service_resource_type: ServiceResourceType,
}

impl serde::Serialize for ResourceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!(
            "AWS::{}::{}",
            <&Service as Into<&'static str>>::into(&self.service),
            self.service_resource_type.0
        ))
    }
}

pub struct Resource {
    r#type: ResourceType,
    logical_name: LogicalName,
    properties: ResourceProperties,
}

impl serde::Serialize for Resource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut resource = serializer.serialize_map(Some(1))?;
        resource.serialize_entry(self.logical_name.as_str(), &Wrapper(self))?;
        resource.end()
    }
}

struct Wrapper<'a>(&'a Resource);

impl serde::Serialize for Wrapper<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut resource = serializer.serialize_map(Some(2))?;
        resource.serialize_entry("Type", &self.0.r#type)?;
        resource.serialize_entry("Properties", &self.0.properties)?;
        resource.end()
    }
}

#[derive(serde::Serialize)]
struct ResourceProperties(serde_json::Map<String, serde_json::Value>);

fn resource<T: ToLogicalName, R: ToResource>(logical_name_value: T, resource_value: R) -> Resource {
    Resource {
        r#type: R::RESOURCE_TYPE,
        logical_name: logical_name_value.to_logical_name(),
        properties: resource_value.to_resource_properties(),
    }
}

trait ToResource {
    const RESOURCE_TYPE: ResourceType;

    fn to_resource_properties(&self) -> ResourceProperties;
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_services() {
        Service::EC2;
    }
}
