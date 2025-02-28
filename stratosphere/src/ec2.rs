/// TODO all of this would be generated
use crate::value::*;
use crate::*;

/*
const SERVICE: Service = Service::EC2;

pub struct SecurityGroup {
    pub group_description: ExpString,
    pub group_name: Option<ExpString> = None,
    pub security_group_ingress: Option<security_group::Ingress> = None,
}

impl ToResource for SecurityGroup {
    const RESOURCE_TYPE: ResourceType = ResourceType {
        service: SERVICE,
        service_resource_type: ServiceResourceType("SecurityGroup"),
    };

    fn to_resource_properties(&self) -> ResourceProperties {
        let mut map = serde_json::Map::new();

        map.insert(
            "GroupDescription".to_string(),
            self.group_description.to_cf_value(),
        );

        if let Some(security_group_ingress) = &self.security_group_ingress {
            map.insert(
                "SecurityGroupIngress".to_string(),
                security_group_ingress.to_cf_value(),
            );
        }

        ResourceProperties(map)
    }
}

pub mod security_group {
    use crate::value::*;

    pub struct Ingress {
        pub cidr_ip: Option<ExpString> = None
    }

    impl Ingress {
        pub fn to_cf_value(&self) -> serde_json::Value {
            let mut map = serde_json::Map::new();

            if let Some(cidr_ip) = &self.cidr_ip {
                map.insert("CidrIp".to_string(), cidr_ip.to_cf_value());
            }

            serde_json::Value::Object(map)
        }
    }
}

/// Example security group
///
/// ```
/// # use stratosphere::*;
/// assert_eq!(
///   serde_json::to_value(example_security_group()).unwrap(),
///   serde_json::json!(
///     {
///       "SecurityGroupA": {
///         "Type": "AWS::EC2::SecurityGroup",
///         "Properties": {
///           "GroupDescription": "Security Group Id A",
///           "SecurityGroupIngress": {
///             "CidrIp": "10.0.0.0/16"
///           }
///         }
///       }
///     }
///   )
/// )
/// ```
pub fn example_security_group() -> Resource {
    resource(
        "SecurityGroupA",
        ec2::SecurityGroup {
            group_description: "Security Group Id A".to_exp(),
            security_group_ingress: ec2::security_group::Ingress {
                cidr_ip: "10.0.0.0/16".to_exp().into(),
            }
            .into(),
            ..
        },
    )
}
*/
