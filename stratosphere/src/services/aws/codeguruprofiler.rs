pub mod profilinggroup {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codeguruprofiler-profilinggroup-agentpermissions.html
    pub struct AgentPermissions_ {
        pub principals: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codeguruprofiler_ProfilingGroup_AgentPermissions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeGuruProfiler::ProfilingGroup.AgentPermissions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codeguruprofiler_ProfilingGroup_AgentPermissions as AgentPermissions;
    impl crate::value::ToValue for AgentPermissions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Principals".to_string(),
                crate::value::ToValue::to_value(&self.principals),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codeguruprofiler-profilinggroup-channel.html
    pub struct Channel_ {
        pub channel_id: Option<crate::value::ExpString>,
        pub channel_uri: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codeguruprofiler_ProfilingGroup_Channel {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeGuruProfiler::ProfilingGroup.Channel"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codeguruprofiler_ProfilingGroup_Channel as Channel;
    impl crate::value::ToValue for Channel_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.channel_id {
                properties.insert(
                    "channelId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "channelUri".to_string(),
                crate::value::ToValue::to_value(&self.channel_uri),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codeguruprofiler-profilinggroup.html
pub struct ProfilingGroup_ {
    pub agent_permissions: Option<super::codeguruprofiler::profilinggroup::AgentPermissions_>,
    pub anomaly_detection_notification_configuration:
        Option<Vec<super::codeguruprofiler::profilinggroup::Channel_>>,
    pub compute_platform: Option<crate::value::ExpString>,
    pub profiling_group_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_codeguruprofiler_ProfilingGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CodeGuruProfiler::ProfilingGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_codeguruprofiler_ProfilingGroup as ProfilingGroup;
impl crate::template::ToResource for ProfilingGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CodeGuruProfiler"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ProfilingGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.agent_permissions {
            properties.insert(
                "AgentPermissions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.anomaly_detection_notification_configuration {
            properties.insert(
                "AnomalyDetectionNotificationConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.compute_platform {
            properties.insert(
                "ComputePlatform".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ProfilingGroupName".to_string(),
            crate::value::ToValue::to_value(&self.profiling_group_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
