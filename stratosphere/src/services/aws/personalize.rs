pub mod dataset {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-dataset-datasource.html
    pub struct DataSource_ {
        pub data_location: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_personalize_Dataset_DataSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Personalize::Dataset.DataSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_personalize_Dataset_DataSource as DataSource;
    impl crate::value::ToValue for DataSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.data_location {
                properties.insert(
                    "DataLocation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-dataset-datasetimportjob.html
    pub struct DatasetImportJob_ {
        pub data_source: Option<Box<DataSource_>>,
        pub dataset_arn: Option<crate::value::ExpString>,
        pub dataset_import_job_arn: Option<crate::value::ExpString>,
        pub job_name: Option<crate::value::ExpString>,
        pub role_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_personalize_Dataset_DatasetImportJob {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Personalize::Dataset.DatasetImportJob"
            $($field $value)*)
        };
    }
    pub use crate::__aws_personalize_Dataset_DatasetImportJob as DatasetImportJob;
    impl crate::value::ToValue for DatasetImportJob_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.data_source {
                properties.insert(
                    "DataSource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dataset_arn {
                properties.insert(
                    "DatasetArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dataset_import_job_arn {
                properties.insert(
                    "DatasetImportJobArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.job_name {
                properties.insert(
                    "JobName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.role_arn {
                properties.insert(
                    "RoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod solution {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-algorithmhyperparameterranges.html
    pub struct AlgorithmHyperParameterRanges_ {
        pub categorical_hyper_parameter_ranges: Option<Vec<CategoricalHyperParameterRange_>>,
        pub continuous_hyper_parameter_ranges: Option<Vec<ContinuousHyperParameterRange_>>,
        pub integer_hyper_parameter_ranges: Option<Vec<IntegerHyperParameterRange_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_personalize_Solution_AlgorithmHyperParameterRanges {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Personalize::Solution.AlgorithmHyperParameterRanges"
            $($field $value)*)
        };
    }
    pub use crate::__aws_personalize_Solution_AlgorithmHyperParameterRanges as AlgorithmHyperParameterRanges;
    impl crate::value::ToValue for AlgorithmHyperParameterRanges_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.categorical_hyper_parameter_ranges {
                properties.insert(
                    "CategoricalHyperParameterRanges".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.continuous_hyper_parameter_ranges {
                properties.insert(
                    "ContinuousHyperParameterRanges".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.integer_hyper_parameter_ranges {
                properties.insert(
                    "IntegerHyperParameterRanges".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-automlconfig.html
    pub struct AutoMLConfig_ {
        pub metric_name: Option<crate::value::ExpString>,
        pub recipe_list: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_personalize_Solution_AutoMLConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Personalize::Solution.AutoMLConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_personalize_Solution_AutoMLConfig as AutoMLConfig;
    impl crate::value::ToValue for AutoMLConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.metric_name {
                properties.insert(
                    "MetricName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.recipe_list {
                properties.insert(
                    "RecipeList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-categoricalhyperparameterrange.html
    pub struct CategoricalHyperParameterRange_ {
        pub name: Option<crate::value::ExpString>,
        pub values: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_personalize_Solution_CategoricalHyperParameterRange {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Personalize::Solution.CategoricalHyperParameterRange"
            $($field $value)*)
        };
    }
    pub use crate::__aws_personalize_Solution_CategoricalHyperParameterRange as CategoricalHyperParameterRange;
    impl crate::value::ToValue for CategoricalHyperParameterRange_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.values {
                properties.insert("Values".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-continuoushyperparameterrange.html
    pub struct ContinuousHyperParameterRange_ {
        pub max_value: Option<f64>,
        pub min_value: Option<f64>,
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_personalize_Solution_ContinuousHyperParameterRange {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Personalize::Solution.ContinuousHyperParameterRange"
            $($field $value)*)
        };
    }
    pub use crate::__aws_personalize_Solution_ContinuousHyperParameterRange as ContinuousHyperParameterRange;
    impl crate::value::ToValue for ContinuousHyperParameterRange_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max_value {
                properties.insert(
                    "MaxValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.min_value {
                properties.insert(
                    "MinValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-hpoconfig.html
    pub struct HpoConfig_ {
        pub algorithm_hyper_parameter_ranges: Option<Box<AlgorithmHyperParameterRanges_>>,
        pub hpo_objective: Option<Box<HpoObjective_>>,
        pub hpo_resource_config: Option<Box<HpoResourceConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_personalize_Solution_HpoConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Personalize::Solution.HpoConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_personalize_Solution_HpoConfig as HpoConfig;
    impl crate::value::ToValue for HpoConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.algorithm_hyper_parameter_ranges {
                properties.insert(
                    "AlgorithmHyperParameterRanges".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hpo_objective {
                properties.insert(
                    "HpoObjective".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hpo_resource_config {
                properties.insert(
                    "HpoResourceConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-hpoobjective.html
    pub struct HpoObjective_ {
        pub metric_name: Option<crate::value::ExpString>,
        pub metric_regex: Option<crate::value::ExpString>,
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_personalize_Solution_HpoObjective {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Personalize::Solution.HpoObjective"
            $($field $value)*)
        };
    }
    pub use crate::__aws_personalize_Solution_HpoObjective as HpoObjective;
    impl crate::value::ToValue for HpoObjective_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.metric_name {
                properties.insert(
                    "MetricName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metric_regex {
                properties.insert(
                    "MetricRegex".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-hporesourceconfig.html
    pub struct HpoResourceConfig_ {
        pub max_number_of_training_jobs: Option<crate::value::ExpString>,
        pub max_parallel_training_jobs: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_personalize_Solution_HpoResourceConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Personalize::Solution.HpoResourceConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_personalize_Solution_HpoResourceConfig as HpoResourceConfig;
    impl crate::value::ToValue for HpoResourceConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max_number_of_training_jobs {
                properties.insert(
                    "MaxNumberOfTrainingJobs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_parallel_training_jobs {
                properties.insert(
                    "MaxParallelTrainingJobs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-integerhyperparameterrange.html
    pub struct IntegerHyperParameterRange_ {
        pub max_value: Option<i64>,
        pub min_value: Option<i64>,
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_personalize_Solution_IntegerHyperParameterRange {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Personalize::Solution.IntegerHyperParameterRange"
            $($field $value)*)
        };
    }
    pub use crate::__aws_personalize_Solution_IntegerHyperParameterRange as IntegerHyperParameterRange;
    impl crate::value::ToValue for IntegerHyperParameterRange_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max_value {
                properties.insert(
                    "MaxValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.min_value {
                properties.insert(
                    "MinValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-solutionconfig.html
    pub struct SolutionConfig_ {
        pub algorithm_hyper_parameters:
            Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub auto_ml_config: Option<Box<AutoMLConfig_>>,
        pub event_value_threshold: Option<crate::value::ExpString>,
        pub feature_transformation_parameters:
            Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub hpo_config: Option<Box<HpoConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_personalize_Solution_SolutionConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Personalize::Solution.SolutionConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_personalize_Solution_SolutionConfig as SolutionConfig;
    impl crate::value::ToValue for SolutionConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.algorithm_hyper_parameters {
                properties.insert(
                    "AlgorithmHyperParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.auto_ml_config {
                properties.insert(
                    "AutoMLConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.event_value_threshold {
                properties.insert(
                    "EventValueThreshold".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.feature_transformation_parameters {
                properties.insert(
                    "FeatureTransformationParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hpo_config {
                properties.insert(
                    "HpoConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-personalize-dataset.html
pub struct Dataset_ {
    pub dataset_group_arn: crate::value::ExpString,
    pub dataset_import_job: Option<super::personalize::dataset::DatasetImportJob_>,
    pub dataset_type: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub schema_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_personalize_Dataset {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Personalize::Dataset"
        $($field $value)*)
    };
}
pub use crate::__aws_personalize_Dataset as Dataset;
impl crate::template::ToResource for Dataset_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Personalize"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Dataset"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DatasetGroupArn".to_string(),
            crate::value::ToValue::to_value(&self.dataset_group_arn),
        );
        if let Some(ref value) = self.dataset_import_job {
            properties.insert(
                "DatasetImportJob".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DatasetType".to_string(),
            crate::value::ToValue::to_value(&self.dataset_type),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "SchemaArn".to_string(),
            crate::value::ToValue::to_value(&self.schema_arn),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-personalize-datasetgroup.html
pub struct DatasetGroup_ {
    pub domain: Option<crate::value::ExpString>,
    pub kms_key_arn: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub role_arn: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_personalize_DatasetGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Personalize::DatasetGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_personalize_DatasetGroup as DatasetGroup;
impl crate::template::ToResource for DatasetGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Personalize"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DatasetGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.domain {
            properties.insert("Domain".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.kms_key_arn {
            properties.insert(
                "KmsKeyArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.role_arn {
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-personalize-schema.html
pub struct Schema_ {
    pub domain: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub schema: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_personalize_Schema {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Personalize::Schema"
        $($field $value)*)
    };
}
pub use crate::__aws_personalize_Schema as Schema;
impl crate::template::ToResource for Schema_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Personalize"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Schema"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.domain {
            properties.insert("Domain".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "Schema".to_string(),
            crate::value::ToValue::to_value(&self.schema),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-personalize-solution.html
pub struct Solution_ {
    pub dataset_group_arn: crate::value::ExpString,
    pub event_type: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub perform_auto_ml: Option<crate::value::ExpBool>,
    pub perform_hpo: Option<crate::value::ExpBool>,
    pub recipe_arn: Option<crate::value::ExpString>,
    pub solution_config: Option<super::personalize::solution::SolutionConfig_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_personalize_Solution {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Personalize::Solution"
        $($field $value)*)
    };
}
pub use crate::__aws_personalize_Solution as Solution;
impl crate::template::ToResource for Solution_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Personalize"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Solution"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DatasetGroupArn".to_string(),
            crate::value::ToValue::to_value(&self.dataset_group_arn),
        );
        if let Some(ref value) = self.event_type {
            properties.insert(
                "EventType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.perform_auto_ml {
            properties.insert(
                "PerformAutoML".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.perform_hpo {
            properties.insert(
                "PerformHPO".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.recipe_arn {
            properties.insert(
                "RecipeArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.solution_config {
            properties.insert(
                "SolutionConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
