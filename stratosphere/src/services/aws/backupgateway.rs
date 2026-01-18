///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-backupgateway-hypervisor.html
pub struct Hypervisor_ {
    pub host: Option<crate::value::ExpString>,
    pub kms_key_arn: Option<crate::value::ExpString>,
    pub log_group_arn: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub password: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub username: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_backupgateway_Hypervisor {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::BackupGateway::Hypervisor"
        $($field $value)*)
    };
}
pub use crate::__aws_backupgateway_Hypervisor as Hypervisor;
impl crate::template::ToResource for Hypervisor_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("BackupGateway"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Hypervisor"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.host {
            properties.insert("Host".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.kms_key_arn {
            properties.insert(
                "KmsKeyArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.log_group_arn {
            properties.insert(
                "LogGroupArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.password {
            properties.insert(
                "Password".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.username {
            properties.insert(
                "Username".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
