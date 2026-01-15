pub mod rotationschedule {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-secretsmanager-rotationschedule-hostedrotationlambda.html
    pub struct HostedRotationLambda_ {
        pub exclude_characters: Option<crate::value::ExpString>,
        pub kms_key_arn: Option<crate::value::ExpString>,
        pub master_secret_arn: Option<crate::value::ExpString>,
        pub master_secret_kms_key_arn: Option<crate::value::ExpString>,
        pub rotation_lambda_name: Option<crate::value::ExpString>,
        pub rotation_type: crate::value::ExpString,
        pub runtime: Option<crate::value::ExpString>,
        pub superuser_secret_arn: Option<crate::value::ExpString>,
        pub superuser_secret_kms_key_arn: Option<crate::value::ExpString>,
        pub vpc_security_group_ids: Option<crate::value::ExpString>,
        pub vpc_subnet_ids: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_secretsmanager_RotationSchedule_HostedRotationLambda {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SecretsManager::RotationSchedule.HostedRotationLambda"
            $($field $value)*)
        };
    }
    pub use crate::__aws_secretsmanager_RotationSchedule_HostedRotationLambda as HostedRotationLambda;
    impl crate::value::ToValue for HostedRotationLambda_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.exclude_characters {
                properties.insert(
                    "ExcludeCharacters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kms_key_arn {
                properties.insert(
                    "KmsKeyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.master_secret_arn {
                properties.insert(
                    "MasterSecretArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.master_secret_kms_key_arn {
                properties.insert(
                    "MasterSecretKmsKeyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rotation_lambda_name {
                properties.insert(
                    "RotationLambdaName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RotationType".to_string(),
                crate::value::ToValue::to_value(&self.rotation_type),
            );
            if let Some(ref value) = self.runtime {
                properties.insert(
                    "Runtime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.superuser_secret_arn {
                properties.insert(
                    "SuperuserSecretArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.superuser_secret_kms_key_arn {
                properties.insert(
                    "SuperuserSecretKmsKeyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpc_security_group_ids {
                properties.insert(
                    "VpcSecurityGroupIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpc_subnet_ids {
                properties.insert(
                    "VpcSubnetIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-secretsmanager-rotationschedule-rotationrules.html
    pub struct RotationRules_ {
        pub automatically_after_days: Option<i64>,
        pub duration: Option<crate::value::ExpString>,
        pub schedule_expression: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_secretsmanager_RotationSchedule_RotationRules {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SecretsManager::RotationSchedule.RotationRules"
            $($field $value)*)
        };
    }
    pub use crate::__aws_secretsmanager_RotationSchedule_RotationRules as RotationRules;
    impl crate::value::ToValue for RotationRules_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.automatically_after_days {
                properties.insert(
                    "AutomaticallyAfterDays".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.duration {
                properties.insert(
                    "Duration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.schedule_expression {
                properties.insert(
                    "ScheduleExpression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod secret {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-secretsmanager-secret-generatesecretstring.html
    pub struct GenerateSecretString_ {
        pub exclude_characters: Option<crate::value::ExpString>,
        pub exclude_lowercase: Option<crate::value::ExpBool>,
        pub exclude_numbers: Option<crate::value::ExpBool>,
        pub exclude_punctuation: Option<crate::value::ExpBool>,
        pub exclude_uppercase: Option<crate::value::ExpBool>,
        pub generate_string_key: Option<crate::value::ExpString>,
        pub include_space: Option<crate::value::ExpBool>,
        pub password_length: Option<i64>,
        pub require_each_included_type: Option<crate::value::ExpBool>,
        pub secret_string_template: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_secretsmanager_Secret_GenerateSecretString {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SecretsManager::Secret.GenerateSecretString"
            $($field $value)*)
        };
    }
    pub use crate::__aws_secretsmanager_Secret_GenerateSecretString as GenerateSecretString;
    impl crate::value::ToValue for GenerateSecretString_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.exclude_characters {
                properties.insert(
                    "ExcludeCharacters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exclude_lowercase {
                properties.insert(
                    "ExcludeLowercase".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exclude_numbers {
                properties.insert(
                    "ExcludeNumbers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exclude_punctuation {
                properties.insert(
                    "ExcludePunctuation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exclude_uppercase {
                properties.insert(
                    "ExcludeUppercase".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.generate_string_key {
                properties.insert(
                    "GenerateStringKey".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_space {
                properties.insert(
                    "IncludeSpace".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.password_length {
                properties.insert(
                    "PasswordLength".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.require_each_included_type {
                properties.insert(
                    "RequireEachIncludedType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secret_string_template {
                properties.insert(
                    "SecretStringTemplate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-secretsmanager-secret-replicaregion.html
    pub struct ReplicaRegion_ {
        pub kms_key_id: Option<crate::value::ExpString>,
        pub region: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_secretsmanager_Secret_ReplicaRegion {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SecretsManager::Secret.ReplicaRegion"
            $($field $value)*)
        };
    }
    pub use crate::__aws_secretsmanager_Secret_ReplicaRegion as ReplicaRegion;
    impl crate::value::ToValue for ReplicaRegion_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Region".to_string(),
                crate::value::ToValue::to_value(&self.region),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-secretsmanager-resourcepolicy.html
pub struct ResourcePolicy_ {
    pub block_public_policy: Option<crate::value::ExpBool>,
    pub resource_policy: serde_json::Value,
    pub secret_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_secretsmanager_ResourcePolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SecretsManager::ResourcePolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_secretsmanager_ResourcePolicy as ResourcePolicy;
impl crate::template::ToResource for ResourcePolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SecretsManager"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ResourcePolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.block_public_policy {
            properties.insert(
                "BlockPublicPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ResourcePolicy".to_string(),
            crate::value::ToValue::to_value(&self.resource_policy),
        );
        properties.insert(
            "SecretId".to_string(),
            crate::value::ToValue::to_value(&self.secret_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-secretsmanager-rotationschedule.html
pub struct RotationSchedule_ {
    pub hosted_rotation_lambda:
        Option<super::secretsmanager::rotationschedule::HostedRotationLambda_>,
    pub rotate_immediately_on_update: Option<crate::value::ExpBool>,
    pub rotation_lambda_arn: Option<crate::value::ExpString>,
    pub rotation_rules: Option<super::secretsmanager::rotationschedule::RotationRules_>,
    pub secret_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_secretsmanager_RotationSchedule {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SecretsManager::RotationSchedule"
        $($field $value)*)
    };
}
pub use crate::__aws_secretsmanager_RotationSchedule as RotationSchedule;
impl crate::template::ToResource for RotationSchedule_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SecretsManager"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RotationSchedule"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.hosted_rotation_lambda {
            properties.insert(
                "HostedRotationLambda".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.rotate_immediately_on_update {
            properties.insert(
                "RotateImmediatelyOnUpdate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.rotation_lambda_arn {
            properties.insert(
                "RotationLambdaARN".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.rotation_rules {
            properties.insert(
                "RotationRules".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SecretId".to_string(),
            crate::value::ToValue::to_value(&self.secret_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-secretsmanager-secret.html
pub struct Secret_ {
    pub description: Option<crate::value::ExpString>,
    pub generate_secret_string: Option<super::secretsmanager::secret::GenerateSecretString_>,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub replica_regions: Option<Vec<super::secretsmanager::secret::ReplicaRegion_>>,
    pub secret_string: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_secretsmanager_Secret {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SecretsManager::Secret"
        $($field $value)*)
    };
}
pub use crate::__aws_secretsmanager_Secret as Secret;
impl crate::template::ToResource for Secret_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SecretsManager"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Secret"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.generate_secret_string {
            properties.insert(
                "GenerateSecretString".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_id {
            properties.insert(
                "KmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.replica_regions {
            properties.insert(
                "ReplicaRegions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.secret_string {
            properties.insert(
                "SecretString".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-secretsmanager-secrettargetattachment.html
pub struct SecretTargetAttachment_ {
    pub secret_id: crate::value::ExpString,
    pub target_id: crate::value::ExpString,
    pub target_type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_secretsmanager_SecretTargetAttachment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SecretsManager::SecretTargetAttachment"
        $($field $value)*)
    };
}
pub use crate::__aws_secretsmanager_SecretTargetAttachment as SecretTargetAttachment;
impl crate::template::ToResource for SecretTargetAttachment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SecretsManager"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SecretTargetAttachment"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "SecretId".to_string(),
            crate::value::ToValue::to_value(&self.secret_id),
        );
        properties.insert(
            "TargetId".to_string(),
            crate::value::ToValue::to_value(&self.target_id),
        );
        properties.insert(
            "TargetType".to_string(),
            crate::value::ToValue::to_value(&self.target_type),
        );
        properties
    }
}
