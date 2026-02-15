pub mod analysistemplate {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-analysistemplate-analysisparameter.html
    pub struct AnalysisParameter_ {
        pub default_value: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_AnalysisTemplate_AnalysisParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::AnalysisTemplate.AnalysisParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_AnalysisTemplate_AnalysisParameter as AnalysisParameter;
    impl crate::value::ToValue for AnalysisParameter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.default_value {
                properties.insert(
                    "DefaultValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-analysistemplate-analysisschema.html
    pub struct AnalysisSchema_ {
        pub referenced_tables: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_AnalysisTemplate_AnalysisSchema {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::AnalysisTemplate.AnalysisSchema"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_AnalysisTemplate_AnalysisSchema as AnalysisSchema;
    impl crate::value::ToValue for AnalysisSchema_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ReferencedTables".to_string(),
                crate::value::ToValue::to_value(&self.referenced_tables),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-analysistemplate-analysissource.html
    pub struct AnalysisSource_ {
        pub artifacts: Option<Box<AnalysisTemplateArtifacts_>>,
        pub text: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_AnalysisTemplate_AnalysisSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::AnalysisTemplate.AnalysisSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_AnalysisTemplate_AnalysisSource as AnalysisSource;
    impl crate::value::ToValue for AnalysisSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.artifacts {
                properties.insert(
                    "Artifacts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.text {
                properties.insert("Text".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-analysistemplate-analysissourcemetadata.html
    pub struct AnalysisSourceMetadata_ {
        pub artifacts: Box<AnalysisTemplateArtifactMetadata_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_AnalysisTemplate_AnalysisSourceMetadata {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::AnalysisTemplate.AnalysisSourceMetadata"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_AnalysisTemplate_AnalysisSourceMetadata as AnalysisSourceMetadata;
    impl crate::value::ToValue for AnalysisSourceMetadata_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Artifacts".to_string(),
                crate::value::ToValue::to_value(&self.artifacts),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-analysistemplate-analysistemplateartifact.html
    pub struct AnalysisTemplateArtifact_ {
        pub location: Box<S3Location_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_AnalysisTemplate_AnalysisTemplateArtifact {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::AnalysisTemplate.AnalysisTemplateArtifact"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_AnalysisTemplate_AnalysisTemplateArtifact as AnalysisTemplateArtifact;
    impl crate::value::ToValue for AnalysisTemplateArtifact_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Location".to_string(),
                crate::value::ToValue::to_value(&self.location),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-analysistemplate-analysistemplateartifactmetadata.html
    pub struct AnalysisTemplateArtifactMetadata_ {
        pub additional_artifact_hashes: Option<Vec<Hash_>>,
        pub entry_point_hash: Box<Hash_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_AnalysisTemplate_AnalysisTemplateArtifactMetadata {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::AnalysisTemplate.AnalysisTemplateArtifactMetadata"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_AnalysisTemplate_AnalysisTemplateArtifactMetadata as AnalysisTemplateArtifactMetadata;
    impl crate::value::ToValue for AnalysisTemplateArtifactMetadata_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.additional_artifact_hashes {
                properties.insert(
                    "AdditionalArtifactHashes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "EntryPointHash".to_string(),
                crate::value::ToValue::to_value(&self.entry_point_hash),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-analysistemplate-analysistemplateartifacts.html
    pub struct AnalysisTemplateArtifacts_ {
        pub additional_artifacts: Option<Vec<AnalysisTemplateArtifact_>>,
        pub entry_point: Box<AnalysisTemplateArtifact_>,
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_AnalysisTemplate_AnalysisTemplateArtifacts {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::AnalysisTemplate.AnalysisTemplateArtifacts"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_AnalysisTemplate_AnalysisTemplateArtifacts as AnalysisTemplateArtifacts;
    impl crate::value::ToValue for AnalysisTemplateArtifacts_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.additional_artifacts {
                properties.insert(
                    "AdditionalArtifacts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "EntryPoint".to_string(),
                crate::value::ToValue::to_value(&self.entry_point),
            );
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-analysistemplate-columnclassificationdetails.html
    pub struct ColumnClassificationDetails_ {
        pub column_mapping: Vec<SyntheticDataColumnProperties_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_AnalysisTemplate_ColumnClassificationDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::AnalysisTemplate.ColumnClassificationDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_AnalysisTemplate_ColumnClassificationDetails as ColumnClassificationDetails;
    impl crate::value::ToValue for ColumnClassificationDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ColumnMapping".to_string(),
                crate::value::ToValue::to_value(&self.column_mapping),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-analysistemplate-errormessageconfiguration.html
    pub struct ErrorMessageConfiguration_ {
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_AnalysisTemplate_ErrorMessageConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::AnalysisTemplate.ErrorMessageConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_AnalysisTemplate_ErrorMessageConfiguration as ErrorMessageConfiguration;
    impl crate::value::ToValue for ErrorMessageConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-analysistemplate-hash.html
    pub struct Hash_ {
        pub sha256: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_AnalysisTemplate_Hash {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::AnalysisTemplate.Hash"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_AnalysisTemplate_Hash as Hash;
    impl crate::value::ToValue for Hash_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.sha256 {
                properties.insert("Sha256".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-analysistemplate-mlsyntheticdataparameters.html
    pub struct MLSyntheticDataParameters_ {
        pub column_classification: Box<ColumnClassificationDetails_>,
        pub epsilon: f64,
        pub max_membership_inference_attack_score: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_AnalysisTemplate_MLSyntheticDataParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::AnalysisTemplate.MLSyntheticDataParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_AnalysisTemplate_MLSyntheticDataParameters as MLSyntheticDataParameters;
    impl crate::value::ToValue for MLSyntheticDataParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ColumnClassification".to_string(),
                crate::value::ToValue::to_value(&self.column_classification),
            );
            properties.insert(
                "Epsilon".to_string(),
                crate::value::ToValue::to_value(&self.epsilon),
            );
            properties.insert(
                "MaxMembershipInferenceAttackScore".to_string(),
                crate::value::ToValue::to_value(&self.max_membership_inference_attack_score),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-analysistemplate-s3location.html
    pub struct S3Location_ {
        pub bucket: crate::value::ExpString,
        pub key: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_AnalysisTemplate_S3Location {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::AnalysisTemplate.S3Location"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_AnalysisTemplate_S3Location as S3Location;
    impl crate::value::ToValue for S3Location_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Bucket".to_string(),
                crate::value::ToValue::to_value(&self.bucket),
            );
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-analysistemplate-syntheticdatacolumnproperties.html
    pub struct SyntheticDataColumnProperties_ {
        pub column_name: crate::value::ExpString,
        pub column_type: crate::value::ExpString,
        pub is_predictive_value: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_AnalysisTemplate_SyntheticDataColumnProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::AnalysisTemplate.SyntheticDataColumnProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_AnalysisTemplate_SyntheticDataColumnProperties as SyntheticDataColumnProperties;
    impl crate::value::ToValue for SyntheticDataColumnProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ColumnName".to_string(),
                crate::value::ToValue::to_value(&self.column_name),
            );
            properties.insert(
                "ColumnType".to_string(),
                crate::value::ToValue::to_value(&self.column_type),
            );
            properties.insert(
                "IsPredictiveValue".to_string(),
                crate::value::ToValue::to_value(&self.is_predictive_value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-analysistemplate-syntheticdataparameters.html
    pub struct SyntheticDataParameters_ {
        pub ml_synthetic_data_parameters: Box<MLSyntheticDataParameters_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_AnalysisTemplate_SyntheticDataParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::AnalysisTemplate.SyntheticDataParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_AnalysisTemplate_SyntheticDataParameters as SyntheticDataParameters;
    impl crate::value::ToValue for SyntheticDataParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MlSyntheticDataParameters".to_string(),
                crate::value::ToValue::to_value(&self.ml_synthetic_data_parameters),
            );
            properties.into()
        }
    }
}
pub mod collaboration {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-collaboration-dataencryptionmetadata.html
    pub struct DataEncryptionMetadata_ {
        pub allow_cleartext: crate::value::ExpBool,
        pub allow_duplicates: crate::value::ExpBool,
        pub allow_joins_on_columns_with_different_names: crate::value::ExpBool,
        pub preserve_nulls: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_Collaboration_DataEncryptionMetadata {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::Collaboration.DataEncryptionMetadata"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_Collaboration_DataEncryptionMetadata as DataEncryptionMetadata;
    impl crate::value::ToValue for DataEncryptionMetadata_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AllowCleartext".to_string(),
                crate::value::ToValue::to_value(&self.allow_cleartext),
            );
            properties.insert(
                "AllowDuplicates".to_string(),
                crate::value::ToValue::to_value(&self.allow_duplicates),
            );
            properties.insert(
                "AllowJoinsOnColumnsWithDifferentNames".to_string(),
                crate::value::ToValue::to_value(&self.allow_joins_on_columns_with_different_names),
            );
            properties.insert(
                "PreserveNulls".to_string(),
                crate::value::ToValue::to_value(&self.preserve_nulls),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-collaboration-jobcomputepaymentconfig.html
    pub struct JobComputePaymentConfig_ {
        pub is_responsible: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_Collaboration_JobComputePaymentConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::Collaboration.JobComputePaymentConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_Collaboration_JobComputePaymentConfig as JobComputePaymentConfig;
    impl crate::value::ToValue for JobComputePaymentConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IsResponsible".to_string(),
                crate::value::ToValue::to_value(&self.is_responsible),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-collaboration-mlmemberabilities.html
    pub struct MLMemberAbilities_ {
        pub custom_ml_member_abilities: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_Collaboration_MLMemberAbilities {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::Collaboration.MLMemberAbilities"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_Collaboration_MLMemberAbilities as MLMemberAbilities;
    impl crate::value::ToValue for MLMemberAbilities_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CustomMLMemberAbilities".to_string(),
                crate::value::ToValue::to_value(&self.custom_ml_member_abilities),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-collaboration-mlpaymentconfig.html
    pub struct MLPaymentConfig_ {
        pub model_inference: Option<Box<ModelInferencePaymentConfig_>>,
        pub model_training: Option<Box<ModelTrainingPaymentConfig_>>,
        pub synthetic_data_generation: Option<Box<SyntheticDataGenerationPaymentConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_Collaboration_MLPaymentConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::Collaboration.MLPaymentConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_Collaboration_MLPaymentConfig as MLPaymentConfig;
    impl crate::value::ToValue for MLPaymentConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.model_inference {
                properties.insert(
                    "ModelInference".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.model_training {
                properties.insert(
                    "ModelTraining".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.synthetic_data_generation {
                properties.insert(
                    "SyntheticDataGeneration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-collaboration-memberspecification.html
    pub struct MemberSpecification_ {
        pub account_id: crate::value::ExpString,
        pub display_name: crate::value::ExpString,
        pub ml_member_abilities: Option<Box<MLMemberAbilities_>>,
        pub member_abilities: Option<Vec<crate::value::ExpString>>,
        pub payment_configuration: Option<Box<PaymentConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_Collaboration_MemberSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::Collaboration.MemberSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_Collaboration_MemberSpecification as MemberSpecification;
    impl crate::value::ToValue for MemberSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AccountId".to_string(),
                crate::value::ToValue::to_value(&self.account_id),
            );
            properties.insert(
                "DisplayName".to_string(),
                crate::value::ToValue::to_value(&self.display_name),
            );
            if let Some(ref value) = self.ml_member_abilities {
                properties.insert(
                    "MLMemberAbilities".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.member_abilities {
                properties.insert(
                    "MemberAbilities".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.payment_configuration {
                properties.insert(
                    "PaymentConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-collaboration-modelinferencepaymentconfig.html
    pub struct ModelInferencePaymentConfig_ {
        pub is_responsible: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_Collaboration_ModelInferencePaymentConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::Collaboration.ModelInferencePaymentConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_Collaboration_ModelInferencePaymentConfig as ModelInferencePaymentConfig;
    impl crate::value::ToValue for ModelInferencePaymentConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IsResponsible".to_string(),
                crate::value::ToValue::to_value(&self.is_responsible),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-collaboration-modeltrainingpaymentconfig.html
    pub struct ModelTrainingPaymentConfig_ {
        pub is_responsible: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_Collaboration_ModelTrainingPaymentConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::Collaboration.ModelTrainingPaymentConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_Collaboration_ModelTrainingPaymentConfig as ModelTrainingPaymentConfig;
    impl crate::value::ToValue for ModelTrainingPaymentConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IsResponsible".to_string(),
                crate::value::ToValue::to_value(&self.is_responsible),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-collaboration-paymentconfiguration.html
    pub struct PaymentConfiguration_ {
        pub job_compute: Option<Box<JobComputePaymentConfig_>>,
        pub machine_learning: Option<Box<MLPaymentConfig_>>,
        pub query_compute: Box<QueryComputePaymentConfig_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_Collaboration_PaymentConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::Collaboration.PaymentConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_Collaboration_PaymentConfiguration as PaymentConfiguration;
    impl crate::value::ToValue for PaymentConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.job_compute {
                properties.insert(
                    "JobCompute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.machine_learning {
                properties.insert(
                    "MachineLearning".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "QueryCompute".to_string(),
                crate::value::ToValue::to_value(&self.query_compute),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-collaboration-querycomputepaymentconfig.html
    pub struct QueryComputePaymentConfig_ {
        pub is_responsible: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_Collaboration_QueryComputePaymentConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::Collaboration.QueryComputePaymentConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_Collaboration_QueryComputePaymentConfig as QueryComputePaymentConfig;
    impl crate::value::ToValue for QueryComputePaymentConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IsResponsible".to_string(),
                crate::value::ToValue::to_value(&self.is_responsible),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-collaboration-syntheticdatagenerationpaymentconfig.html
    pub struct SyntheticDataGenerationPaymentConfig_ {
        pub is_responsible: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_Collaboration_SyntheticDataGenerationPaymentConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::Collaboration.SyntheticDataGenerationPaymentConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_Collaboration_SyntheticDataGenerationPaymentConfig as SyntheticDataGenerationPaymentConfig;
    impl crate::value::ToValue for SyntheticDataGenerationPaymentConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IsResponsible".to_string(),
                crate::value::ToValue::to_value(&self.is_responsible),
            );
            properties.into()
        }
    }
}
pub mod configuredtable {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-aggregatecolumn.html
    pub struct AggregateColumn_ {
        pub column_names: Vec<crate::value::ExpString>,
        pub function: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_ConfiguredTable_AggregateColumn {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::ConfiguredTable.AggregateColumn"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_ConfiguredTable_AggregateColumn as AggregateColumn;
    impl crate::value::ToValue for AggregateColumn_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ColumnNames".to_string(),
                crate::value::ToValue::to_value(&self.column_names),
            );
            properties.insert(
                "Function".to_string(),
                crate::value::ToValue::to_value(&self.function),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-aggregationconstraint.html
    pub struct AggregationConstraint_ {
        pub column_name: crate::value::ExpString,
        pub minimum: f64,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_ConfiguredTable_AggregationConstraint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::ConfiguredTable.AggregationConstraint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_ConfiguredTable_AggregationConstraint as AggregationConstraint;
    impl crate::value::ToValue for AggregationConstraint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ColumnName".to_string(),
                crate::value::ToValue::to_value(&self.column_name),
            );
            properties.insert(
                "Minimum".to_string(),
                crate::value::ToValue::to_value(&self.minimum),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-analysisrule.html
    pub struct AnalysisRule_ {
        pub policy: Box<ConfiguredTableAnalysisRulePolicy_>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_ConfiguredTable_AnalysisRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::ConfiguredTable.AnalysisRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_ConfiguredTable_AnalysisRule as AnalysisRule;
    impl crate::value::ToValue for AnalysisRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Policy".to_string(),
                crate::value::ToValue::to_value(&self.policy),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-analysisruleaggregation.html
    pub struct AnalysisRuleAggregation_ {
        pub additional_analyses: Option<crate::value::ExpString>,
        pub aggregate_columns: Vec<AggregateColumn_>,
        pub allowed_join_operators: Option<Vec<crate::value::ExpString>>,
        pub dimension_columns: Vec<crate::value::ExpString>,
        pub join_columns: Vec<crate::value::ExpString>,
        pub join_required: Option<crate::value::ExpString>,
        pub output_constraints: Vec<AggregationConstraint_>,
        pub scalar_functions: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_ConfiguredTable_AnalysisRuleAggregation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::ConfiguredTable.AnalysisRuleAggregation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_ConfiguredTable_AnalysisRuleAggregation as AnalysisRuleAggregation;
    impl crate::value::ToValue for AnalysisRuleAggregation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.additional_analyses {
                properties.insert(
                    "AdditionalAnalyses".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "AggregateColumns".to_string(),
                crate::value::ToValue::to_value(&self.aggregate_columns),
            );
            if let Some(ref value) = self.allowed_join_operators {
                properties.insert(
                    "AllowedJoinOperators".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DimensionColumns".to_string(),
                crate::value::ToValue::to_value(&self.dimension_columns),
            );
            properties.insert(
                "JoinColumns".to_string(),
                crate::value::ToValue::to_value(&self.join_columns),
            );
            if let Some(ref value) = self.join_required {
                properties.insert(
                    "JoinRequired".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "OutputConstraints".to_string(),
                crate::value::ToValue::to_value(&self.output_constraints),
            );
            properties.insert(
                "ScalarFunctions".to_string(),
                crate::value::ToValue::to_value(&self.scalar_functions),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-analysisrulecustom.html
    pub struct AnalysisRuleCustom_ {
        pub additional_analyses: Option<crate::value::ExpString>,
        pub allowed_analyses: Vec<crate::value::ExpString>,
        pub allowed_analysis_providers: Option<Vec<crate::value::ExpString>>,
        pub differential_privacy: Option<Box<DifferentialPrivacy_>>,
        pub disallowed_output_columns: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_ConfiguredTable_AnalysisRuleCustom {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::ConfiguredTable.AnalysisRuleCustom"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_ConfiguredTable_AnalysisRuleCustom as AnalysisRuleCustom;
    impl crate::value::ToValue for AnalysisRuleCustom_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.additional_analyses {
                properties.insert(
                    "AdditionalAnalyses".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "AllowedAnalyses".to_string(),
                crate::value::ToValue::to_value(&self.allowed_analyses),
            );
            if let Some(ref value) = self.allowed_analysis_providers {
                properties.insert(
                    "AllowedAnalysisProviders".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.differential_privacy {
                properties.insert(
                    "DifferentialPrivacy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.disallowed_output_columns {
                properties.insert(
                    "DisallowedOutputColumns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-analysisrulelist.html
    pub struct AnalysisRuleList_ {
        pub additional_analyses: Option<crate::value::ExpString>,
        pub allowed_join_operators: Option<Vec<crate::value::ExpString>>,
        pub join_columns: Vec<crate::value::ExpString>,
        pub list_columns: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_ConfiguredTable_AnalysisRuleList {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::ConfiguredTable.AnalysisRuleList"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_ConfiguredTable_AnalysisRuleList as AnalysisRuleList;
    impl crate::value::ToValue for AnalysisRuleList_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.additional_analyses {
                properties.insert(
                    "AdditionalAnalyses".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.allowed_join_operators {
                properties.insert(
                    "AllowedJoinOperators".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "JoinColumns".to_string(),
                crate::value::ToValue::to_value(&self.join_columns),
            );
            properties.insert(
                "ListColumns".to_string(),
                crate::value::ToValue::to_value(&self.list_columns),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-athenatablereference.html
    pub struct AthenaTableReference_ {
        pub database_name: crate::value::ExpString,
        pub output_location: Option<crate::value::ExpString>,
        pub region: Option<crate::value::ExpString>,
        pub table_name: crate::value::ExpString,
        pub work_group: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_ConfiguredTable_AthenaTableReference {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::ConfiguredTable.AthenaTableReference"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_ConfiguredTable_AthenaTableReference as AthenaTableReference;
    impl crate::value::ToValue for AthenaTableReference_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DatabaseName".to_string(),
                crate::value::ToValue::to_value(&self.database_name),
            );
            if let Some(ref value) = self.output_location {
                properties.insert(
                    "OutputLocation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.region {
                properties.insert("Region".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "TableName".to_string(),
                crate::value::ToValue::to_value(&self.table_name),
            );
            properties.insert(
                "WorkGroup".to_string(),
                crate::value::ToValue::to_value(&self.work_group),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-configuredtableanalysisrulepolicy.html
    pub struct ConfiguredTableAnalysisRulePolicy_ {
        pub v1: Box<ConfiguredTableAnalysisRulePolicyV1_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_ConfiguredTable_ConfiguredTableAnalysisRulePolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::ConfiguredTable.ConfiguredTableAnalysisRulePolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_ConfiguredTable_ConfiguredTableAnalysisRulePolicy as ConfiguredTableAnalysisRulePolicy;
    impl crate::value::ToValue for ConfiguredTableAnalysisRulePolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert("V1".to_string(), crate::value::ToValue::to_value(&self.v1));
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-configuredtableanalysisrulepolicyv1.html
    pub struct ConfiguredTableAnalysisRulePolicyV1_ {
        pub aggregation: Option<Box<AnalysisRuleAggregation_>>,
        pub custom: Option<Box<AnalysisRuleCustom_>>,
        pub list: Option<Box<AnalysisRuleList_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_ConfiguredTable_ConfiguredTableAnalysisRulePolicyV1 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::ConfiguredTable.ConfiguredTableAnalysisRulePolicyV1"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_ConfiguredTable_ConfiguredTableAnalysisRulePolicyV1 as ConfiguredTableAnalysisRulePolicyV1;
    impl crate::value::ToValue for ConfiguredTableAnalysisRulePolicyV1_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.aggregation {
                properties.insert(
                    "Aggregation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom {
                properties.insert("Custom".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.list {
                properties.insert("List".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-differentialprivacy.html
    pub struct DifferentialPrivacy_ {
        pub columns: Vec<DifferentialPrivacyColumn_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_ConfiguredTable_DifferentialPrivacy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::ConfiguredTable.DifferentialPrivacy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_ConfiguredTable_DifferentialPrivacy as DifferentialPrivacy;
    impl crate::value::ToValue for DifferentialPrivacy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Columns".to_string(),
                crate::value::ToValue::to_value(&self.columns),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-differentialprivacycolumn.html
    pub struct DifferentialPrivacyColumn_ {
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_ConfiguredTable_DifferentialPrivacyColumn {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::ConfiguredTable.DifferentialPrivacyColumn"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_ConfiguredTable_DifferentialPrivacyColumn as DifferentialPrivacyColumn;
    impl crate::value::ToValue for DifferentialPrivacyColumn_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-gluetablereference.html
    pub struct GlueTableReference_ {
        pub database_name: crate::value::ExpString,
        pub region: Option<crate::value::ExpString>,
        pub table_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_ConfiguredTable_GlueTableReference {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::ConfiguredTable.GlueTableReference"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_ConfiguredTable_GlueTableReference as GlueTableReference;
    impl crate::value::ToValue for GlueTableReference_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DatabaseName".to_string(),
                crate::value::ToValue::to_value(&self.database_name),
            );
            if let Some(ref value) = self.region {
                properties.insert("Region".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "TableName".to_string(),
                crate::value::ToValue::to_value(&self.table_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-snowflaketablereference.html
    pub struct SnowflakeTableReference_ {
        pub account_identifier: crate::value::ExpString,
        pub database_name: crate::value::ExpString,
        pub schema_name: crate::value::ExpString,
        pub secret_arn: crate::value::ExpString,
        pub table_name: crate::value::ExpString,
        pub table_schema: Box<SnowflakeTableSchema_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_ConfiguredTable_SnowflakeTableReference {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::ConfiguredTable.SnowflakeTableReference"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_ConfiguredTable_SnowflakeTableReference as SnowflakeTableReference;
    impl crate::value::ToValue for SnowflakeTableReference_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AccountIdentifier".to_string(),
                crate::value::ToValue::to_value(&self.account_identifier),
            );
            properties.insert(
                "DatabaseName".to_string(),
                crate::value::ToValue::to_value(&self.database_name),
            );
            properties.insert(
                "SchemaName".to_string(),
                crate::value::ToValue::to_value(&self.schema_name),
            );
            properties.insert(
                "SecretArn".to_string(),
                crate::value::ToValue::to_value(&self.secret_arn),
            );
            properties.insert(
                "TableName".to_string(),
                crate::value::ToValue::to_value(&self.table_name),
            );
            properties.insert(
                "TableSchema".to_string(),
                crate::value::ToValue::to_value(&self.table_schema),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-snowflaketableschema.html
    pub struct SnowflakeTableSchema_ {
        pub v1: Vec<SnowflakeTableSchemaV1_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_ConfiguredTable_SnowflakeTableSchema {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::ConfiguredTable.SnowflakeTableSchema"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_ConfiguredTable_SnowflakeTableSchema as SnowflakeTableSchema;
    impl crate::value::ToValue for SnowflakeTableSchema_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert("V1".to_string(), crate::value::ToValue::to_value(&self.v1));
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-snowflaketableschemav1.html
    pub struct SnowflakeTableSchemaV1_ {
        pub column_name: crate::value::ExpString,
        pub column_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_ConfiguredTable_SnowflakeTableSchemaV1 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::ConfiguredTable.SnowflakeTableSchemaV1"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_ConfiguredTable_SnowflakeTableSchemaV1 as SnowflakeTableSchemaV1;
    impl crate::value::ToValue for SnowflakeTableSchemaV1_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ColumnName".to_string(),
                crate::value::ToValue::to_value(&self.column_name),
            );
            properties.insert(
                "ColumnType".to_string(),
                crate::value::ToValue::to_value(&self.column_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-tablereference.html
    pub struct TableReference_ {
        pub athena: Option<Box<AthenaTableReference_>>,
        pub glue: Option<Box<GlueTableReference_>>,
        pub snowflake: Option<Box<SnowflakeTableReference_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_ConfiguredTable_TableReference {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::ConfiguredTable.TableReference"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_ConfiguredTable_TableReference as TableReference;
    impl crate::value::ToValue for TableReference_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.athena {
                properties.insert("Athena".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.glue {
                properties.insert("Glue".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.snowflake {
                properties.insert(
                    "Snowflake".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod configuredtableassociation {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtableassociation-configuredtableassociationanalysisrule.html
    pub struct ConfiguredTableAssociationAnalysisRule_ {
        pub policy: Box<ConfiguredTableAssociationAnalysisRulePolicy_>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_ConfiguredTableAssociation_ConfiguredTableAssociationAnalysisRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::ConfiguredTableAssociation.ConfiguredTableAssociationAnalysisRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_ConfiguredTableAssociation_ConfiguredTableAssociationAnalysisRule as ConfiguredTableAssociationAnalysisRule;
    impl crate::value::ToValue for ConfiguredTableAssociationAnalysisRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Policy".to_string(),
                crate::value::ToValue::to_value(&self.policy),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtableassociation-configuredtableassociationanalysisruleaggregation.html
    pub struct ConfiguredTableAssociationAnalysisRuleAggregation_ {
        pub allowed_additional_analyses: Option<Vec<crate::value::ExpString>>,
        pub allowed_result_receivers: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_ConfiguredTableAssociation_ConfiguredTableAssociationAnalysisRuleAggregation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::ConfiguredTableAssociation.ConfiguredTableAssociationAnalysisRuleAggregation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_ConfiguredTableAssociation_ConfiguredTableAssociationAnalysisRuleAggregation as ConfiguredTableAssociationAnalysisRuleAggregation;
    impl crate::value::ToValue for ConfiguredTableAssociationAnalysisRuleAggregation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allowed_additional_analyses {
                properties.insert(
                    "AllowedAdditionalAnalyses".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.allowed_result_receivers {
                properties.insert(
                    "AllowedResultReceivers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtableassociation-configuredtableassociationanalysisrulecustom.html
    pub struct ConfiguredTableAssociationAnalysisRuleCustom_ {
        pub allowed_additional_analyses: Option<Vec<crate::value::ExpString>>,
        pub allowed_result_receivers: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_ConfiguredTableAssociation_ConfiguredTableAssociationAnalysisRuleCustom {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::ConfiguredTableAssociation.ConfiguredTableAssociationAnalysisRuleCustom"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_ConfiguredTableAssociation_ConfiguredTableAssociationAnalysisRuleCustom as ConfiguredTableAssociationAnalysisRuleCustom;
    impl crate::value::ToValue for ConfiguredTableAssociationAnalysisRuleCustom_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allowed_additional_analyses {
                properties.insert(
                    "AllowedAdditionalAnalyses".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.allowed_result_receivers {
                properties.insert(
                    "AllowedResultReceivers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtableassociation-configuredtableassociationanalysisrulelist.html
    pub struct ConfiguredTableAssociationAnalysisRuleList_ {
        pub allowed_additional_analyses: Option<Vec<crate::value::ExpString>>,
        pub allowed_result_receivers: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_ConfiguredTableAssociation_ConfiguredTableAssociationAnalysisRuleList {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::ConfiguredTableAssociation.ConfiguredTableAssociationAnalysisRuleList"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_ConfiguredTableAssociation_ConfiguredTableAssociationAnalysisRuleList as ConfiguredTableAssociationAnalysisRuleList;
    impl crate::value::ToValue for ConfiguredTableAssociationAnalysisRuleList_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allowed_additional_analyses {
                properties.insert(
                    "AllowedAdditionalAnalyses".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.allowed_result_receivers {
                properties.insert(
                    "AllowedResultReceivers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtableassociation-configuredtableassociationanalysisrulepolicy.html
    pub struct ConfiguredTableAssociationAnalysisRulePolicy_ {
        pub v1: Box<ConfiguredTableAssociationAnalysisRulePolicyV1_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_ConfiguredTableAssociation_ConfiguredTableAssociationAnalysisRulePolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::ConfiguredTableAssociation.ConfiguredTableAssociationAnalysisRulePolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_ConfiguredTableAssociation_ConfiguredTableAssociationAnalysisRulePolicy as ConfiguredTableAssociationAnalysisRulePolicy;
    impl crate::value::ToValue for ConfiguredTableAssociationAnalysisRulePolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert("V1".to_string(), crate::value::ToValue::to_value(&self.v1));
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtableassociation-configuredtableassociationanalysisrulepolicyv1.html
    pub struct ConfiguredTableAssociationAnalysisRulePolicyV1_ {
        pub aggregation: Option<Box<ConfiguredTableAssociationAnalysisRuleAggregation_>>,
        pub custom: Option<Box<ConfiguredTableAssociationAnalysisRuleCustom_>>,
        pub list: Option<Box<ConfiguredTableAssociationAnalysisRuleList_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_ConfiguredTableAssociation_ConfiguredTableAssociationAnalysisRulePolicyV1 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::ConfiguredTableAssociation.ConfiguredTableAssociationAnalysisRulePolicyV1"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_ConfiguredTableAssociation_ConfiguredTableAssociationAnalysisRulePolicyV1 as ConfiguredTableAssociationAnalysisRulePolicyV1;
    impl crate::value::ToValue for ConfiguredTableAssociationAnalysisRulePolicyV1_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.aggregation {
                properties.insert(
                    "Aggregation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom {
                properties.insert("Custom".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.list {
                properties.insert("List".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod idmappingtable {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-idmappingtable-idmappingtableinputreferenceconfig.html
    pub struct IdMappingTableInputReferenceConfig_ {
        pub input_reference_arn: crate::value::ExpString,
        pub manage_resource_policies: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_IdMappingTable_IdMappingTableInputReferenceConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::IdMappingTable.IdMappingTableInputReferenceConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_IdMappingTable_IdMappingTableInputReferenceConfig as IdMappingTableInputReferenceConfig;
    impl crate::value::ToValue for IdMappingTableInputReferenceConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InputReferenceArn".to_string(),
                crate::value::ToValue::to_value(&self.input_reference_arn),
            );
            properties.insert(
                "ManageResourcePolicies".to_string(),
                crate::value::ToValue::to_value(&self.manage_resource_policies),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-idmappingtable-idmappingtableinputreferenceproperties.html
    pub struct IdMappingTableInputReferenceProperties_ {
        pub id_mapping_table_input_source: Vec<IdMappingTableInputSource_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_IdMappingTable_IdMappingTableInputReferenceProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::IdMappingTable.IdMappingTableInputReferenceProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_IdMappingTable_IdMappingTableInputReferenceProperties as IdMappingTableInputReferenceProperties;
    impl crate::value::ToValue for IdMappingTableInputReferenceProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IdMappingTableInputSource".to_string(),
                crate::value::ToValue::to_value(&self.id_mapping_table_input_source),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-idmappingtable-idmappingtableinputsource.html
    pub struct IdMappingTableInputSource_ {
        pub id_namespace_association_id: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_IdMappingTable_IdMappingTableInputSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::IdMappingTable.IdMappingTableInputSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_IdMappingTable_IdMappingTableInputSource as IdMappingTableInputSource;
    impl crate::value::ToValue for IdMappingTableInputSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IdNamespaceAssociationId".to_string(),
                crate::value::ToValue::to_value(&self.id_namespace_association_id),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
}
pub mod idnamespaceassociation {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-idnamespaceassociation-idmappingconfig.html
    pub struct IdMappingConfig_ {
        pub allow_use_as_dimension_column: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_IdNamespaceAssociation_IdMappingConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::IdNamespaceAssociation.IdMappingConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_IdNamespaceAssociation_IdMappingConfig as IdMappingConfig;
    impl crate::value::ToValue for IdMappingConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AllowUseAsDimensionColumn".to_string(),
                crate::value::ToValue::to_value(&self.allow_use_as_dimension_column),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-idnamespaceassociation-idnamespaceassociationinputreferenceconfig.html
    pub struct IdNamespaceAssociationInputReferenceConfig_ {
        pub input_reference_arn: crate::value::ExpString,
        pub manage_resource_policies: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_IdNamespaceAssociation_IdNamespaceAssociationInputReferenceConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::IdNamespaceAssociation.IdNamespaceAssociationInputReferenceConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_IdNamespaceAssociation_IdNamespaceAssociationInputReferenceConfig as IdNamespaceAssociationInputReferenceConfig;
    impl crate::value::ToValue for IdNamespaceAssociationInputReferenceConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InputReferenceArn".to_string(),
                crate::value::ToValue::to_value(&self.input_reference_arn),
            );
            properties.insert(
                "ManageResourcePolicies".to_string(),
                crate::value::ToValue::to_value(&self.manage_resource_policies),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-idnamespaceassociation-idnamespaceassociationinputreferenceproperties.html
    pub struct IdNamespaceAssociationInputReferenceProperties_ {
        pub id_mapping_workflows_supported: Option<Vec<serde_json::Value>>,
        pub id_namespace_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_IdNamespaceAssociation_IdNamespaceAssociationInputReferenceProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::IdNamespaceAssociation.IdNamespaceAssociationInputReferenceProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_IdNamespaceAssociation_IdNamespaceAssociationInputReferenceProperties as IdNamespaceAssociationInputReferenceProperties;
    impl crate::value::ToValue for IdNamespaceAssociationInputReferenceProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.id_mapping_workflows_supported {
                properties.insert(
                    "IdMappingWorkflowsSupported".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.id_namespace_type {
                properties.insert(
                    "IdNamespaceType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod membership {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-membership-membershipjobcomputepaymentconfig.html
    pub struct MembershipJobComputePaymentConfig_ {
        pub is_responsible: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_Membership_MembershipJobComputePaymentConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::Membership.MembershipJobComputePaymentConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_Membership_MembershipJobComputePaymentConfig as MembershipJobComputePaymentConfig;
    impl crate::value::ToValue for MembershipJobComputePaymentConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IsResponsible".to_string(),
                crate::value::ToValue::to_value(&self.is_responsible),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-membership-membershipmlpaymentconfig.html
    pub struct MembershipMLPaymentConfig_ {
        pub model_inference: Option<Box<MembershipModelInferencePaymentConfig_>>,
        pub model_training: Option<Box<MembershipModelTrainingPaymentConfig_>>,
        pub synthetic_data_generation: Option<Box<MembershipSyntheticDataGenerationPaymentConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_Membership_MembershipMLPaymentConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::Membership.MembershipMLPaymentConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_Membership_MembershipMLPaymentConfig as MembershipMLPaymentConfig;
    impl crate::value::ToValue for MembershipMLPaymentConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.model_inference {
                properties.insert(
                    "ModelInference".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.model_training {
                properties.insert(
                    "ModelTraining".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.synthetic_data_generation {
                properties.insert(
                    "SyntheticDataGeneration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-membership-membershipmodelinferencepaymentconfig.html
    pub struct MembershipModelInferencePaymentConfig_ {
        pub is_responsible: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_Membership_MembershipModelInferencePaymentConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::Membership.MembershipModelInferencePaymentConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_Membership_MembershipModelInferencePaymentConfig as MembershipModelInferencePaymentConfig;
    impl crate::value::ToValue for MembershipModelInferencePaymentConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IsResponsible".to_string(),
                crate::value::ToValue::to_value(&self.is_responsible),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-membership-membershipmodeltrainingpaymentconfig.html
    pub struct MembershipModelTrainingPaymentConfig_ {
        pub is_responsible: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_Membership_MembershipModelTrainingPaymentConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::Membership.MembershipModelTrainingPaymentConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_Membership_MembershipModelTrainingPaymentConfig as MembershipModelTrainingPaymentConfig;
    impl crate::value::ToValue for MembershipModelTrainingPaymentConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IsResponsible".to_string(),
                crate::value::ToValue::to_value(&self.is_responsible),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-membership-membershippaymentconfiguration.html
    pub struct MembershipPaymentConfiguration_ {
        pub job_compute: Option<Box<MembershipJobComputePaymentConfig_>>,
        pub machine_learning: Option<Box<MembershipMLPaymentConfig_>>,
        pub query_compute: Box<MembershipQueryComputePaymentConfig_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_Membership_MembershipPaymentConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::Membership.MembershipPaymentConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_Membership_MembershipPaymentConfiguration as MembershipPaymentConfiguration;
    impl crate::value::ToValue for MembershipPaymentConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.job_compute {
                properties.insert(
                    "JobCompute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.machine_learning {
                properties.insert(
                    "MachineLearning".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "QueryCompute".to_string(),
                crate::value::ToValue::to_value(&self.query_compute),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-membership-membershipprotectedjoboutputconfiguration.html
    pub struct MembershipProtectedJobOutputConfiguration_ {
        pub s3: Box<ProtectedJobS3OutputConfigurationInput_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_Membership_MembershipProtectedJobOutputConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::Membership.MembershipProtectedJobOutputConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_Membership_MembershipProtectedJobOutputConfiguration as MembershipProtectedJobOutputConfiguration;
    impl crate::value::ToValue for MembershipProtectedJobOutputConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert("S3".to_string(), crate::value::ToValue::to_value(&self.s3));
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-membership-membershipprotectedjobresultconfiguration.html
    pub struct MembershipProtectedJobResultConfiguration_ {
        pub output_configuration: Box<MembershipProtectedJobOutputConfiguration_>,
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_Membership_MembershipProtectedJobResultConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::Membership.MembershipProtectedJobResultConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_Membership_MembershipProtectedJobResultConfiguration as MembershipProtectedJobResultConfiguration;
    impl crate::value::ToValue for MembershipProtectedJobResultConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "OutputConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.output_configuration),
            );
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-membership-membershipprotectedqueryoutputconfiguration.html
    pub struct MembershipProtectedQueryOutputConfiguration_ {
        pub s3: Box<ProtectedQueryS3OutputConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_Membership_MembershipProtectedQueryOutputConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::Membership.MembershipProtectedQueryOutputConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_Membership_MembershipProtectedQueryOutputConfiguration as MembershipProtectedQueryOutputConfiguration;
    impl crate::value::ToValue for MembershipProtectedQueryOutputConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert("S3".to_string(), crate::value::ToValue::to_value(&self.s3));
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-membership-membershipprotectedqueryresultconfiguration.html
    pub struct MembershipProtectedQueryResultConfiguration_ {
        pub output_configuration: Box<MembershipProtectedQueryOutputConfiguration_>,
        pub role_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_Membership_MembershipProtectedQueryResultConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::Membership.MembershipProtectedQueryResultConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_Membership_MembershipProtectedQueryResultConfiguration as MembershipProtectedQueryResultConfiguration;
    impl crate::value::ToValue for MembershipProtectedQueryResultConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "OutputConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.output_configuration),
            );
            if let Some(ref value) = self.role_arn {
                properties.insert(
                    "RoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-membership-membershipquerycomputepaymentconfig.html
    pub struct MembershipQueryComputePaymentConfig_ {
        pub is_responsible: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_Membership_MembershipQueryComputePaymentConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::Membership.MembershipQueryComputePaymentConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_Membership_MembershipQueryComputePaymentConfig as MembershipQueryComputePaymentConfig;
    impl crate::value::ToValue for MembershipQueryComputePaymentConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IsResponsible".to_string(),
                crate::value::ToValue::to_value(&self.is_responsible),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-membership-membershipsyntheticdatagenerationpaymentconfig.html
    pub struct MembershipSyntheticDataGenerationPaymentConfig_ {
        pub is_responsible: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_Membership_MembershipSyntheticDataGenerationPaymentConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::Membership.MembershipSyntheticDataGenerationPaymentConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_Membership_MembershipSyntheticDataGenerationPaymentConfig as MembershipSyntheticDataGenerationPaymentConfig;
    impl crate::value::ToValue for MembershipSyntheticDataGenerationPaymentConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IsResponsible".to_string(),
                crate::value::ToValue::to_value(&self.is_responsible),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-membership-protectedjobs3outputconfigurationinput.html
    pub struct ProtectedJobS3OutputConfigurationInput_ {
        pub bucket: crate::value::ExpString,
        pub key_prefix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_Membership_ProtectedJobS3OutputConfigurationInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::Membership.ProtectedJobS3OutputConfigurationInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_Membership_ProtectedJobS3OutputConfigurationInput as ProtectedJobS3OutputConfigurationInput;
    impl crate::value::ToValue for ProtectedJobS3OutputConfigurationInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Bucket".to_string(),
                crate::value::ToValue::to_value(&self.bucket),
            );
            if let Some(ref value) = self.key_prefix {
                properties.insert(
                    "KeyPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-membership-protectedquerys3outputconfiguration.html
    pub struct ProtectedQueryS3OutputConfiguration_ {
        pub bucket: crate::value::ExpString,
        pub key_prefix: Option<crate::value::ExpString>,
        pub result_format: crate::value::ExpString,
        pub single_file_output: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_Membership_ProtectedQueryS3OutputConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::Membership.ProtectedQueryS3OutputConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_Membership_ProtectedQueryS3OutputConfiguration as ProtectedQueryS3OutputConfiguration;
    impl crate::value::ToValue for ProtectedQueryS3OutputConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Bucket".to_string(),
                crate::value::ToValue::to_value(&self.bucket),
            );
            if let Some(ref value) = self.key_prefix {
                properties.insert(
                    "KeyPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ResultFormat".to_string(),
                crate::value::ToValue::to_value(&self.result_format),
            );
            if let Some(ref value) = self.single_file_output {
                properties.insert(
                    "SingleFileOutput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod privacybudgettemplate {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-privacybudgettemplate-budgetparameter.html
    pub struct BudgetParameter_ {
        pub auto_refresh: Option<crate::value::ExpString>,
        pub budget: i32,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_PrivacyBudgetTemplate_BudgetParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::PrivacyBudgetTemplate.BudgetParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_PrivacyBudgetTemplate_BudgetParameter as BudgetParameter;
    impl crate::value::ToValue for BudgetParameter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auto_refresh {
                properties.insert(
                    "AutoRefresh".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Budget".to_string(),
                crate::value::ToValue::to_value(&self.budget),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-privacybudgettemplate-parameters.html
    pub struct Parameters_ {
        pub budget_parameters: Option<Vec<BudgetParameter_>>,
        pub epsilon: Option<i32>,
        pub resource_arn: Option<crate::value::ExpString>,
        pub users_noise_per_query: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanrooms_PrivacyBudgetTemplate_Parameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRooms::PrivacyBudgetTemplate.Parameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanrooms_PrivacyBudgetTemplate_Parameters as Parameters;
    impl crate::value::ToValue for Parameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.budget_parameters {
                properties.insert(
                    "BudgetParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.epsilon {
                properties.insert(
                    "Epsilon".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_arn {
                properties.insert(
                    "ResourceArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.users_noise_per_query {
                properties.insert(
                    "UsersNoisePerQuery".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-analysistemplate.html
pub struct AnalysisTemplate_ {
    pub analysis_parameters: Option<Vec<super::cleanrooms::analysistemplate::AnalysisParameter_>>,
    pub description: Option<crate::value::ExpString>,
    pub error_message_configuration:
        Option<super::cleanrooms::analysistemplate::ErrorMessageConfiguration_>,
    pub format: crate::value::ExpString,
    pub membership_identifier: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub schema: Option<super::cleanrooms::analysistemplate::AnalysisSchema_>,
    pub source: super::cleanrooms::analysistemplate::AnalysisSource_,
    pub source_metadata: Option<super::cleanrooms::analysistemplate::AnalysisSourceMetadata_>,
    pub synthetic_data_parameters:
        Option<super::cleanrooms::analysistemplate::SyntheticDataParameters_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cleanrooms_AnalysisTemplate {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CleanRooms::AnalysisTemplate"
        $($field $value)*)
    };
}
pub use crate::__aws_cleanrooms_AnalysisTemplate as AnalysisTemplate;
impl crate::template::ToResource for AnalysisTemplate_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CleanRooms"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AnalysisTemplate"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.analysis_parameters {
            properties.insert(
                "AnalysisParameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.error_message_configuration {
            properties.insert(
                "ErrorMessageConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Format".to_string(),
            crate::value::ToValue::to_value(&self.format),
        );
        properties.insert(
            "MembershipIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.membership_identifier),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.schema {
            properties.insert("Schema".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Source".to_string(),
            crate::value::ToValue::to_value(&self.source),
        );
        if let Some(ref value) = self.source_metadata {
            properties.insert(
                "SourceMetadata".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.synthetic_data_parameters {
            properties.insert(
                "SyntheticDataParameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-collaboration.html
pub struct Collaboration_ {
    pub allowed_result_regions: Option<Vec<crate::value::ExpString>>,
    pub analytics_engine: Option<crate::value::ExpString>,
    pub auto_approved_change_types: Option<Vec<crate::value::ExpString>>,
    pub creator_display_name: crate::value::ExpString,
    pub creator_ml_member_abilities: Option<super::cleanrooms::collaboration::MLMemberAbilities_>,
    pub creator_member_abilities: Option<Vec<crate::value::ExpString>>,
    pub creator_payment_configuration:
        Option<super::cleanrooms::collaboration::PaymentConfiguration_>,
    pub data_encryption_metadata: Option<super::cleanrooms::collaboration::DataEncryptionMetadata_>,
    pub description: crate::value::ExpString,
    pub is_metrics_enabled: Option<crate::value::ExpBool>,
    pub job_log_status: Option<crate::value::ExpString>,
    pub members: Option<Vec<super::cleanrooms::collaboration::MemberSpecification_>>,
    pub name: crate::value::ExpString,
    pub query_log_status: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cleanrooms_Collaboration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CleanRooms::Collaboration"
        $($field $value)*)
    };
}
pub use crate::__aws_cleanrooms_Collaboration as Collaboration;
impl crate::template::ToResource for Collaboration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CleanRooms"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Collaboration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.allowed_result_regions {
            properties.insert(
                "AllowedResultRegions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.analytics_engine {
            properties.insert(
                "AnalyticsEngine".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.auto_approved_change_types {
            properties.insert(
                "AutoApprovedChangeTypes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "CreatorDisplayName".to_string(),
            crate::value::ToValue::to_value(&self.creator_display_name),
        );
        if let Some(ref value) = self.creator_ml_member_abilities {
            properties.insert(
                "CreatorMLMemberAbilities".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.creator_member_abilities {
            properties.insert(
                "CreatorMemberAbilities".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.creator_payment_configuration {
            properties.insert(
                "CreatorPaymentConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.data_encryption_metadata {
            properties.insert(
                "DataEncryptionMetadata".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Description".to_string(),
            crate::value::ToValue::to_value(&self.description),
        );
        if let Some(ref value) = self.is_metrics_enabled {
            properties.insert(
                "IsMetricsEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.job_log_status {
            properties.insert(
                "JobLogStatus".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.members {
            properties.insert(
                "Members".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "QueryLogStatus".to_string(),
            crate::value::ToValue::to_value(&self.query_log_status),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-configuredtable.html
pub struct ConfiguredTable_ {
    pub allowed_columns: Vec<crate::value::ExpString>,
    pub analysis_method: crate::value::ExpString,
    pub analysis_rules: Option<Vec<super::cleanrooms::configuredtable::AnalysisRule_>>,
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub selected_analysis_methods: Option<Vec<crate::value::ExpString>>,
    pub table_reference: super::cleanrooms::configuredtable::TableReference_,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cleanrooms_ConfiguredTable {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CleanRooms::ConfiguredTable"
        $($field $value)*)
    };
}
pub use crate::__aws_cleanrooms_ConfiguredTable as ConfiguredTable;
impl crate::template::ToResource for ConfiguredTable_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CleanRooms"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ConfiguredTable"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AllowedColumns".to_string(),
            crate::value::ToValue::to_value(&self.allowed_columns),
        );
        properties.insert(
            "AnalysisMethod".to_string(),
            crate::value::ToValue::to_value(&self.analysis_method),
        );
        if let Some(ref value) = self.analysis_rules {
            properties.insert(
                "AnalysisRules".to_string(),
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
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.selected_analysis_methods {
            properties.insert(
                "SelectedAnalysisMethods".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "TableReference".to_string(),
            crate::value::ToValue::to_value(&self.table_reference),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-configuredtableassociation.html
pub struct ConfiguredTableAssociation_ {
    pub configured_table_association_analysis_rules: Option<
        Vec<super::cleanrooms::configuredtableassociation::ConfiguredTableAssociationAnalysisRule_>,
    >,
    pub configured_table_identifier: crate::value::ExpString,
    pub description: Option<crate::value::ExpString>,
    pub membership_identifier: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub role_arn: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cleanrooms_ConfiguredTableAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CleanRooms::ConfiguredTableAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_cleanrooms_ConfiguredTableAssociation as ConfiguredTableAssociation;
impl crate::template::ToResource for ConfiguredTableAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CleanRooms"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "ConfiguredTableAssociation",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.configured_table_association_analysis_rules {
            properties.insert(
                "ConfiguredTableAssociationAnalysisRules".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ConfiguredTableIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.configured_table_identifier),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "MembershipIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.membership_identifier),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-idmappingtable.html
pub struct IdMappingTable_ {
    pub description: Option<crate::value::ExpString>,
    pub input_reference_config:
        super::cleanrooms::idmappingtable::IdMappingTableInputReferenceConfig_,
    pub kms_key_arn: Option<crate::value::ExpString>,
    pub membership_identifier: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cleanrooms_IdMappingTable {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CleanRooms::IdMappingTable"
        $($field $value)*)
    };
}
pub use crate::__aws_cleanrooms_IdMappingTable as IdMappingTable;
impl crate::template::ToResource for IdMappingTable_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CleanRooms"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("IdMappingTable"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "InputReferenceConfig".to_string(),
            crate::value::ToValue::to_value(&self.input_reference_config),
        );
        if let Some(ref value) = self.kms_key_arn {
            properties.insert(
                "KmsKeyArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "MembershipIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.membership_identifier),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-idnamespaceassociation.html
pub struct IdNamespaceAssociation_ {
    pub description: Option<crate::value::ExpString>,
    pub id_mapping_config: Option<super::cleanrooms::idnamespaceassociation::IdMappingConfig_>,
    pub input_reference_config:
        super::cleanrooms::idnamespaceassociation::IdNamespaceAssociationInputReferenceConfig_,
    pub membership_identifier: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cleanrooms_IdNamespaceAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CleanRooms::IdNamespaceAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_cleanrooms_IdNamespaceAssociation as IdNamespaceAssociation;
impl crate::template::ToResource for IdNamespaceAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CleanRooms"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("IdNamespaceAssociation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.id_mapping_config {
            properties.insert(
                "IdMappingConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "InputReferenceConfig".to_string(),
            crate::value::ToValue::to_value(&self.input_reference_config),
        );
        properties.insert(
            "MembershipIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.membership_identifier),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-membership.html
pub struct Membership_ {
    pub collaboration_identifier: crate::value::ExpString,
    pub default_job_result_configuration:
        Option<super::cleanrooms::membership::MembershipProtectedJobResultConfiguration_>,
    pub default_result_configuration:
        Option<super::cleanrooms::membership::MembershipProtectedQueryResultConfiguration_>,
    pub is_metrics_enabled: Option<crate::value::ExpBool>,
    pub job_log_status: Option<crate::value::ExpString>,
    pub payment_configuration:
        Option<super::cleanrooms::membership::MembershipPaymentConfiguration_>,
    pub query_log_status: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cleanrooms_Membership {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CleanRooms::Membership"
        $($field $value)*)
    };
}
pub use crate::__aws_cleanrooms_Membership as Membership;
impl crate::template::ToResource for Membership_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CleanRooms"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Membership"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "CollaborationIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.collaboration_identifier),
        );
        if let Some(ref value) = self.default_job_result_configuration {
            properties.insert(
                "DefaultJobResultConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.default_result_configuration {
            properties.insert(
                "DefaultResultConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.is_metrics_enabled {
            properties.insert(
                "IsMetricsEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.job_log_status {
            properties.insert(
                "JobLogStatus".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.payment_configuration {
            properties.insert(
                "PaymentConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "QueryLogStatus".to_string(),
            crate::value::ToValue::to_value(&self.query_log_status),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-privacybudgettemplate.html
pub struct PrivacyBudgetTemplate_ {
    pub auto_refresh: crate::value::ExpString,
    pub membership_identifier: crate::value::ExpString,
    pub parameters: super::cleanrooms::privacybudgettemplate::Parameters_,
    pub privacy_budget_type: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cleanrooms_PrivacyBudgetTemplate {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CleanRooms::PrivacyBudgetTemplate"
        $($field $value)*)
    };
}
pub use crate::__aws_cleanrooms_PrivacyBudgetTemplate as PrivacyBudgetTemplate;
impl crate::template::ToResource for PrivacyBudgetTemplate_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CleanRooms"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PrivacyBudgetTemplate"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AutoRefresh".to_string(),
            crate::value::ToValue::to_value(&self.auto_refresh),
        );
        properties.insert(
            "MembershipIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.membership_identifier),
        );
        properties.insert(
            "Parameters".to_string(),
            crate::value::ToValue::to_value(&self.parameters),
        );
        properties.insert(
            "PrivacyBudgetType".to_string(),
            crate::value::ToValue::to_value(&self.privacy_budget_type),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
