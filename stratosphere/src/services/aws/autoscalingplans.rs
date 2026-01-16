pub mod scalingplan {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-applicationsource.html
    pub struct ApplicationSource_ {
        pub cloud_formation_stack_arn: Option<crate::value::ExpString>,
        pub tag_filters: Option<Vec<TagFilter_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscalingplans_ScalingPlan_ApplicationSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScalingPlans::ScalingPlan.ApplicationSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscalingplans_ScalingPlan_ApplicationSource as ApplicationSource;
    impl crate::value::ToValue for ApplicationSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cloud_formation_stack_arn {
                properties.insert(
                    "CloudFormationStackARN".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tag_filters {
                properties.insert(
                    "TagFilters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-customizedloadmetricspecification.html
    pub struct CustomizedLoadMetricSpecification_ {
        pub dimensions: Option<Vec<MetricDimension_>>,
        pub metric_name: crate::value::ExpString,
        pub namespace: crate::value::ExpString,
        pub statistic: crate::value::ExpString,
        pub unit: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscalingplans_ScalingPlan_CustomizedLoadMetricSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScalingPlans::ScalingPlan.CustomizedLoadMetricSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscalingplans_ScalingPlan_CustomizedLoadMetricSpecification as CustomizedLoadMetricSpecification;
    impl crate::value::ToValue for CustomizedLoadMetricSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dimensions {
                properties.insert(
                    "Dimensions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MetricName".to_string(),
                crate::value::ToValue::to_value(&self.metric_name),
            );
            properties.insert(
                "Namespace".to_string(),
                crate::value::ToValue::to_value(&self.namespace),
            );
            properties.insert(
                "Statistic".to_string(),
                crate::value::ToValue::to_value(&self.statistic),
            );
            if let Some(ref value) = self.unit {
                properties.insert("Unit".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-customizedscalingmetricspecification.html
    pub struct CustomizedScalingMetricSpecification_ {
        pub dimensions: Option<Vec<MetricDimension_>>,
        pub metric_name: crate::value::ExpString,
        pub namespace: crate::value::ExpString,
        pub statistic: crate::value::ExpString,
        pub unit: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscalingplans_ScalingPlan_CustomizedScalingMetricSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScalingPlans::ScalingPlan.CustomizedScalingMetricSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscalingplans_ScalingPlan_CustomizedScalingMetricSpecification as CustomizedScalingMetricSpecification;
    impl crate::value::ToValue for CustomizedScalingMetricSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dimensions {
                properties.insert(
                    "Dimensions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MetricName".to_string(),
                crate::value::ToValue::to_value(&self.metric_name),
            );
            properties.insert(
                "Namespace".to_string(),
                crate::value::ToValue::to_value(&self.namespace),
            );
            properties.insert(
                "Statistic".to_string(),
                crate::value::ToValue::to_value(&self.statistic),
            );
            if let Some(ref value) = self.unit {
                properties.insert("Unit".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-metricdimension.html
    pub struct MetricDimension_ {
        pub name: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscalingplans_ScalingPlan_MetricDimension {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScalingPlans::ScalingPlan.MetricDimension"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscalingplans_ScalingPlan_MetricDimension as MetricDimension;
    impl crate::value::ToValue for MetricDimension_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-predefinedloadmetricspecification.html
    pub struct PredefinedLoadMetricSpecification_ {
        pub predefined_load_metric_type: crate::value::ExpString,
        pub resource_label: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscalingplans_ScalingPlan_PredefinedLoadMetricSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScalingPlans::ScalingPlan.PredefinedLoadMetricSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscalingplans_ScalingPlan_PredefinedLoadMetricSpecification as PredefinedLoadMetricSpecification;
    impl crate::value::ToValue for PredefinedLoadMetricSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PredefinedLoadMetricType".to_string(),
                crate::value::ToValue::to_value(&self.predefined_load_metric_type),
            );
            if let Some(ref value) = self.resource_label {
                properties.insert(
                    "ResourceLabel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-predefinedscalingmetricspecification.html
    pub struct PredefinedScalingMetricSpecification_ {
        pub predefined_scaling_metric_type: crate::value::ExpString,
        pub resource_label: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscalingplans_ScalingPlan_PredefinedScalingMetricSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScalingPlans::ScalingPlan.PredefinedScalingMetricSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscalingplans_ScalingPlan_PredefinedScalingMetricSpecification as PredefinedScalingMetricSpecification;
    impl crate::value::ToValue for PredefinedScalingMetricSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PredefinedScalingMetricType".to_string(),
                crate::value::ToValue::to_value(&self.predefined_scaling_metric_type),
            );
            if let Some(ref value) = self.resource_label {
                properties.insert(
                    "ResourceLabel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-scalinginstruction.html
    pub struct ScalingInstruction_ {
        pub customized_load_metric_specification: Option<Box<CustomizedLoadMetricSpecification_>>,
        pub disable_dynamic_scaling: Option<crate::value::ExpBool>,
        pub max_capacity: i64,
        pub min_capacity: i64,
        pub predefined_load_metric_specification: Option<Box<PredefinedLoadMetricSpecification_>>,
        pub predictive_scaling_max_capacity_behavior: Option<crate::value::ExpString>,
        pub predictive_scaling_max_capacity_buffer: Option<i64>,
        pub predictive_scaling_mode: Option<crate::value::ExpString>,
        pub resource_id: crate::value::ExpString,
        pub scalable_dimension: crate::value::ExpString,
        pub scaling_policy_update_behavior: Option<crate::value::ExpString>,
        pub scheduled_action_buffer_time: Option<i64>,
        pub service_namespace: crate::value::ExpString,
        pub target_tracking_configurations: Vec<TargetTrackingConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscalingplans_ScalingPlan_ScalingInstruction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScalingPlans::ScalingPlan.ScalingInstruction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscalingplans_ScalingPlan_ScalingInstruction as ScalingInstruction;
    impl crate::value::ToValue for ScalingInstruction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.customized_load_metric_specification {
                properties.insert(
                    "CustomizedLoadMetricSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.disable_dynamic_scaling {
                properties.insert(
                    "DisableDynamicScaling".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MaxCapacity".to_string(),
                crate::value::ToValue::to_value(&self.max_capacity),
            );
            properties.insert(
                "MinCapacity".to_string(),
                crate::value::ToValue::to_value(&self.min_capacity),
            );
            if let Some(ref value) = self.predefined_load_metric_specification {
                properties.insert(
                    "PredefinedLoadMetricSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.predictive_scaling_max_capacity_behavior {
                properties.insert(
                    "PredictiveScalingMaxCapacityBehavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.predictive_scaling_max_capacity_buffer {
                properties.insert(
                    "PredictiveScalingMaxCapacityBuffer".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.predictive_scaling_mode {
                properties.insert(
                    "PredictiveScalingMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ResourceId".to_string(),
                crate::value::ToValue::to_value(&self.resource_id),
            );
            properties.insert(
                "ScalableDimension".to_string(),
                crate::value::ToValue::to_value(&self.scalable_dimension),
            );
            if let Some(ref value) = self.scaling_policy_update_behavior {
                properties.insert(
                    "ScalingPolicyUpdateBehavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scheduled_action_buffer_time {
                properties.insert(
                    "ScheduledActionBufferTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ServiceNamespace".to_string(),
                crate::value::ToValue::to_value(&self.service_namespace),
            );
            properties.insert(
                "TargetTrackingConfigurations".to_string(),
                crate::value::ToValue::to_value(&self.target_tracking_configurations),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-tagfilter.html
    pub struct TagFilter_ {
        pub key: crate::value::ExpString,
        pub values: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscalingplans_ScalingPlan_TagFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScalingPlans::ScalingPlan.TagFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscalingplans_ScalingPlan_TagFilter as TagFilter;
    impl crate::value::ToValue for TagFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            if let Some(ref value) = self.values {
                properties.insert("Values".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-targettrackingconfiguration.html
    pub struct TargetTrackingConfiguration_ {
        pub customized_scaling_metric_specification:
            Option<Box<CustomizedScalingMetricSpecification_>>,
        pub disable_scale_in: Option<crate::value::ExpBool>,
        pub estimated_instance_warmup: Option<i64>,
        pub predefined_scaling_metric_specification:
            Option<Box<PredefinedScalingMetricSpecification_>>,
        pub scale_in_cooldown: Option<i64>,
        pub scale_out_cooldown: Option<i64>,
        pub target_value: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscalingplans_ScalingPlan_TargetTrackingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScalingPlans::ScalingPlan.TargetTrackingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscalingplans_ScalingPlan_TargetTrackingConfiguration as TargetTrackingConfiguration;
    impl crate::value::ToValue for TargetTrackingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.customized_scaling_metric_specification {
                properties.insert(
                    "CustomizedScalingMetricSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.disable_scale_in {
                properties.insert(
                    "DisableScaleIn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.estimated_instance_warmup {
                properties.insert(
                    "EstimatedInstanceWarmup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.predefined_scaling_metric_specification {
                properties.insert(
                    "PredefinedScalingMetricSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scale_in_cooldown {
                properties.insert(
                    "ScaleInCooldown".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scale_out_cooldown {
                properties.insert(
                    "ScaleOutCooldown".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TargetValue".to_string(),
                crate::value::ToValue::to_value(&self.target_value),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-autoscalingplans-scalingplan.html
pub struct ScalingPlan_ {
    pub application_source: super::autoscalingplans::scalingplan::ApplicationSource_,
    pub scaling_instructions: Vec<super::autoscalingplans::scalingplan::ScalingInstruction_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_autoscalingplans_ScalingPlan {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::AutoScalingPlans::ScalingPlan"
        $($field $value)*)
    };
}
pub use crate::__aws_autoscalingplans_ScalingPlan as ScalingPlan;
impl crate::template::ToResource for ScalingPlan_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AutoScalingPlans"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ScalingPlan"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApplicationSource".to_string(),
            crate::value::ToValue::to_value(&self.application_source),
        );
        properties.insert(
            "ScalingInstructions".to_string(),
            crate::value::ToValue::to_value(&self.scaling_instructions),
        );
        properties
    }
}
