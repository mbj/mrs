pub mod documentclassifier {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-documentclassifier-augmentedmanifestslistitem.html
    pub struct AugmentedManifestsListItem_ {
        pub attribute_names: Vec<crate::value::ExpString>,
        pub s3_uri: crate::value::ExpString,
        pub split: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_comprehend_DocumentClassifier_AugmentedManifestsListItem {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Comprehend::DocumentClassifier.AugmentedManifestsListItem"
            $($field $value)*)
        };
    }
    pub use crate::__aws_comprehend_DocumentClassifier_AugmentedManifestsListItem as AugmentedManifestsListItem;
    impl crate::value::ToValue for AugmentedManifestsListItem_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AttributeNames".to_string(),
                crate::value::ToValue::to_value(&self.attribute_names),
            );
            properties.insert(
                "S3Uri".to_string(),
                crate::value::ToValue::to_value(&self.s3_uri),
            );
            if let Some(ref value) = self.split {
                properties.insert("Split".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-documentclassifier-documentclassifierdocuments.html
    pub struct DocumentClassifierDocuments_ {
        pub s3_uri: crate::value::ExpString,
        pub test_s3_uri: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_comprehend_DocumentClassifier_DocumentClassifierDocuments {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Comprehend::DocumentClassifier.DocumentClassifierDocuments"
            $($field $value)*)
        };
    }
    pub use crate::__aws_comprehend_DocumentClassifier_DocumentClassifierDocuments as DocumentClassifierDocuments;
    impl crate::value::ToValue for DocumentClassifierDocuments_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "S3Uri".to_string(),
                crate::value::ToValue::to_value(&self.s3_uri),
            );
            if let Some(ref value) = self.test_s3_uri {
                properties.insert(
                    "TestS3Uri".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-documentclassifier-documentclassifierinputdataconfig.html
    pub struct DocumentClassifierInputDataConfig_ {
        pub augmented_manifests: Option<Vec<AugmentedManifestsListItem_>>,
        pub data_format: Option<crate::value::ExpString>,
        pub document_reader_config: Option<Box<DocumentReaderConfig_>>,
        pub document_type: Option<crate::value::ExpString>,
        pub documents: Option<Box<DocumentClassifierDocuments_>>,
        pub label_delimiter: Option<crate::value::ExpString>,
        pub s3_uri: Option<crate::value::ExpString>,
        pub test_s3_uri: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_comprehend_DocumentClassifier_DocumentClassifierInputDataConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Comprehend::DocumentClassifier.DocumentClassifierInputDataConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_comprehend_DocumentClassifier_DocumentClassifierInputDataConfig as DocumentClassifierInputDataConfig;
    impl crate::value::ToValue for DocumentClassifierInputDataConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.augmented_manifests {
                properties.insert(
                    "AugmentedManifests".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.data_format {
                properties.insert(
                    "DataFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.document_reader_config {
                properties.insert(
                    "DocumentReaderConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.document_type {
                properties.insert(
                    "DocumentType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.documents {
                properties.insert(
                    "Documents".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.label_delimiter {
                properties.insert(
                    "LabelDelimiter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_uri {
                properties.insert("S3Uri".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.test_s3_uri {
                properties.insert(
                    "TestS3Uri".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-documentclassifier-documentclassifieroutputdataconfig.html
    pub struct DocumentClassifierOutputDataConfig_ {
        pub kms_key_id: Option<crate::value::ExpString>,
        pub s3_uri: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_comprehend_DocumentClassifier_DocumentClassifierOutputDataConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Comprehend::DocumentClassifier.DocumentClassifierOutputDataConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_comprehend_DocumentClassifier_DocumentClassifierOutputDataConfig as DocumentClassifierOutputDataConfig;
    impl crate::value::ToValue for DocumentClassifierOutputDataConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_uri {
                properties.insert("S3Uri".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-documentclassifier-documentreaderconfig.html
    pub struct DocumentReaderConfig_ {
        pub document_read_action: crate::value::ExpString,
        pub document_read_mode: Option<crate::value::ExpString>,
        pub feature_types: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_comprehend_DocumentClassifier_DocumentReaderConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Comprehend::DocumentClassifier.DocumentReaderConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_comprehend_DocumentClassifier_DocumentReaderConfig as DocumentReaderConfig;
    impl crate::value::ToValue for DocumentReaderConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DocumentReadAction".to_string(),
                crate::value::ToValue::to_value(&self.document_read_action),
            );
            if let Some(ref value) = self.document_read_mode {
                properties.insert(
                    "DocumentReadMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.feature_types {
                properties.insert(
                    "FeatureTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-documentclassifier-vpcconfig.html
    pub struct VpcConfig_ {
        pub security_group_ids: Vec<crate::value::ExpString>,
        pub subnets: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_comprehend_DocumentClassifier_VpcConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Comprehend::DocumentClassifier.VpcConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_comprehend_DocumentClassifier_VpcConfig as VpcConfig;
    impl crate::value::ToValue for VpcConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(&self.security_group_ids),
            );
            properties.insert(
                "Subnets".to_string(),
                crate::value::ToValue::to_value(&self.subnets),
            );
            properties.into()
        }
    }
}
pub mod flywheel {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-flywheel-datasecurityconfig.html
    pub struct DataSecurityConfig_ {
        pub data_lake_kms_key_id: Option<crate::value::ExpString>,
        pub model_kms_key_id: Option<crate::value::ExpString>,
        pub volume_kms_key_id: Option<crate::value::ExpString>,
        pub vpc_config: Option<Box<VpcConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_comprehend_Flywheel_DataSecurityConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Comprehend::Flywheel.DataSecurityConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_comprehend_Flywheel_DataSecurityConfig as DataSecurityConfig;
    impl crate::value::ToValue for DataSecurityConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.data_lake_kms_key_id {
                properties.insert(
                    "DataLakeKmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.model_kms_key_id {
                properties.insert(
                    "ModelKmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.volume_kms_key_id {
                properties.insert(
                    "VolumeKmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpc_config {
                properties.insert(
                    "VpcConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-flywheel-documentclassificationconfig.html
    pub struct DocumentClassificationConfig_ {
        pub labels: Option<Vec<crate::value::ExpString>>,
        pub mode: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_comprehend_Flywheel_DocumentClassificationConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Comprehend::Flywheel.DocumentClassificationConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_comprehend_Flywheel_DocumentClassificationConfig as DocumentClassificationConfig;
    impl crate::value::ToValue for DocumentClassificationConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.labels {
                properties.insert("Labels".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Mode".to_string(),
                crate::value::ToValue::to_value(&self.mode),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-flywheel-entityrecognitionconfig.html
    pub struct EntityRecognitionConfig_ {
        pub entity_types: Option<Vec<EntityTypesListItem_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_comprehend_Flywheel_EntityRecognitionConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Comprehend::Flywheel.EntityRecognitionConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_comprehend_Flywheel_EntityRecognitionConfig as EntityRecognitionConfig;
    impl crate::value::ToValue for EntityRecognitionConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.entity_types {
                properties.insert(
                    "EntityTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-flywheel-entitytypeslistitem.html
    pub struct EntityTypesListItem_ {
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_comprehend_Flywheel_EntityTypesListItem {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Comprehend::Flywheel.EntityTypesListItem"
            $($field $value)*)
        };
    }
    pub use crate::__aws_comprehend_Flywheel_EntityTypesListItem as EntityTypesListItem;
    impl crate::value::ToValue for EntityTypesListItem_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-flywheel-taskconfig.html
    pub struct TaskConfig_ {
        pub document_classification_config: Option<Box<DocumentClassificationConfig_>>,
        pub entity_recognition_config: Option<Box<EntityRecognitionConfig_>>,
        pub language_code: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_comprehend_Flywheel_TaskConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Comprehend::Flywheel.TaskConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_comprehend_Flywheel_TaskConfig as TaskConfig;
    impl crate::value::ToValue for TaskConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.document_classification_config {
                properties.insert(
                    "DocumentClassificationConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.entity_recognition_config {
                properties.insert(
                    "EntityRecognitionConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "LanguageCode".to_string(),
                crate::value::ToValue::to_value(&self.language_code),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-flywheel-vpcconfig.html
    pub struct VpcConfig_ {
        pub security_group_ids: Vec<crate::value::ExpString>,
        pub subnets: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_comprehend_Flywheel_VpcConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Comprehend::Flywheel.VpcConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_comprehend_Flywheel_VpcConfig as VpcConfig;
    impl crate::value::ToValue for VpcConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(&self.security_group_ids),
            );
            properties.insert(
                "Subnets".to_string(),
                crate::value::ToValue::to_value(&self.subnets),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-comprehend-documentclassifier.html
pub struct DocumentClassifier_ {
    pub data_access_role_arn: crate::value::ExpString,
    pub document_classifier_name: crate::value::ExpString,
    pub input_data_config:
        super::comprehend::documentclassifier::DocumentClassifierInputDataConfig_,
    pub language_code: crate::value::ExpString,
    pub mode: Option<crate::value::ExpString>,
    pub model_kms_key_id: Option<crate::value::ExpString>,
    pub model_policy: Option<crate::value::ExpString>,
    pub output_data_config:
        Option<super::comprehend::documentclassifier::DocumentClassifierOutputDataConfig_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub version_name: Option<crate::value::ExpString>,
    pub volume_kms_key_id: Option<crate::value::ExpString>,
    pub vpc_config: Option<super::comprehend::documentclassifier::VpcConfig_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_comprehend_DocumentClassifier {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Comprehend::DocumentClassifier"
        $($field $value)*)
    };
}
pub use crate::__aws_comprehend_DocumentClassifier as DocumentClassifier;
impl crate::template::ToResource for DocumentClassifier_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Comprehend"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DocumentClassifier"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DataAccessRoleArn".to_string(),
            crate::value::ToValue::to_value(&self.data_access_role_arn),
        );
        properties.insert(
            "DocumentClassifierName".to_string(),
            crate::value::ToValue::to_value(&self.document_classifier_name),
        );
        properties.insert(
            "InputDataConfig".to_string(),
            crate::value::ToValue::to_value(&self.input_data_config),
        );
        properties.insert(
            "LanguageCode".to_string(),
            crate::value::ToValue::to_value(&self.language_code),
        );
        if let Some(ref value) = self.mode {
            properties.insert("Mode".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.model_kms_key_id {
            properties.insert(
                "ModelKmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.model_policy {
            properties.insert(
                "ModelPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.output_data_config {
            properties.insert(
                "OutputDataConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.version_name {
            properties.insert(
                "VersionName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.volume_kms_key_id {
            properties.insert(
                "VolumeKmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.vpc_config {
            properties.insert(
                "VpcConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-comprehend-flywheel.html
pub struct Flywheel_ {
    pub active_model_arn: Option<crate::value::ExpString>,
    pub data_access_role_arn: crate::value::ExpString,
    pub data_lake_s3_uri: crate::value::ExpString,
    pub data_security_config: Option<super::comprehend::flywheel::DataSecurityConfig_>,
    pub flywheel_name: crate::value::ExpString,
    pub model_type: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub task_config: Option<super::comprehend::flywheel::TaskConfig_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_comprehend_Flywheel {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Comprehend::Flywheel"
        $($field $value)*)
    };
}
pub use crate::__aws_comprehend_Flywheel as Flywheel;
impl crate::template::ToResource for Flywheel_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Comprehend"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Flywheel"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.active_model_arn {
            properties.insert(
                "ActiveModelArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DataAccessRoleArn".to_string(),
            crate::value::ToValue::to_value(&self.data_access_role_arn),
        );
        properties.insert(
            "DataLakeS3Uri".to_string(),
            crate::value::ToValue::to_value(&self.data_lake_s3_uri),
        );
        if let Some(ref value) = self.data_security_config {
            properties.insert(
                "DataSecurityConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "FlywheelName".to_string(),
            crate::value::ToValue::to_value(&self.flywheel_name),
        );
        if let Some(ref value) = self.model_type {
            properties.insert(
                "ModelType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.task_config {
            properties.insert(
                "TaskConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
