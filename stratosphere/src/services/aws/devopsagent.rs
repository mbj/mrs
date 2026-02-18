pub mod association {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-devopsagent-association-awsconfiguration.html>
    pub struct AWSConfiguration_ {
        pub account_id: crate::value::ExpString,
        pub account_type: crate::value::ExpString,
        pub assumable_role_arn: crate::value::ExpString,
        pub resources: Option<Vec<AWSResource_>>,
        pub tags: Option<Vec<KeyValuePair_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_devopsagent_Association_AWSConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DevOpsAgent::Association.AWSConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_devopsagent_Association_AWSConfiguration as AWSConfiguration;
    impl crate::value::ToValue for AWSConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AccountId".to_string(),
                crate::value::ToValue::to_value(&self.account_id),
            );
            properties.insert(
                "AccountType".to_string(),
                crate::value::ToValue::to_value(&self.account_type),
            );
            properties.insert(
                "AssumableRoleArn".to_string(),
                crate::value::ToValue::to_value(&self.assumable_role_arn),
            );
            if let Some(ref value) = self.resources {
                properties.insert(
                    "Resources".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tags {
                properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-devopsagent-association-awsresource.html>
    pub struct AWSResource_ {
        pub resource_arn: crate::value::ExpString,
        pub resource_metadata: Option<serde_json::Value>,
        pub resource_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_devopsagent_Association_AWSResource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DevOpsAgent::Association.AWSResource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_devopsagent_Association_AWSResource as AWSResource;
    impl crate::value::ToValue for AWSResource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ResourceArn".to_string(),
                crate::value::ToValue::to_value(&self.resource_arn),
            );
            if let Some(ref value) = self.resource_metadata {
                properties.insert(
                    "ResourceMetadata".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_type {
                properties.insert(
                    "ResourceType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-devopsagent-association-dynatraceconfiguration.html>
    pub struct DynatraceConfiguration_ {
        pub enable_webhook_updates: Option<crate::value::ExpBool>,
        pub env_id: crate::value::ExpString,
        pub resources: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_devopsagent_Association_DynatraceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DevOpsAgent::Association.DynatraceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_devopsagent_Association_DynatraceConfiguration as DynatraceConfiguration;
    impl crate::value::ToValue for DynatraceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enable_webhook_updates {
                properties.insert(
                    "EnableWebhookUpdates".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "EnvId".to_string(),
                crate::value::ToValue::to_value(&self.env_id),
            );
            if let Some(ref value) = self.resources {
                properties.insert(
                    "Resources".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-devopsagent-association-eventchannelconfiguration.html>
    pub struct EventChannelConfiguration_ {
        pub enable_webhook_updates: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_devopsagent_Association_EventChannelConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DevOpsAgent::Association.EventChannelConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_devopsagent_Association_EventChannelConfiguration as EventChannelConfiguration;
    impl crate::value::ToValue for EventChannelConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enable_webhook_updates {
                properties.insert(
                    "EnableWebhookUpdates".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-devopsagent-association-githubconfiguration.html>
    pub struct GitHubConfiguration_ {
        pub owner: crate::value::ExpString,
        pub owner_type: crate::value::ExpString,
        pub repo_id: crate::value::ExpString,
        pub repo_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_devopsagent_Association_GitHubConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DevOpsAgent::Association.GitHubConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_devopsagent_Association_GitHubConfiguration as GitHubConfiguration;
    impl crate::value::ToValue for GitHubConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Owner".to_string(),
                crate::value::ToValue::to_value(&self.owner),
            );
            properties.insert(
                "OwnerType".to_string(),
                crate::value::ToValue::to_value(&self.owner_type),
            );
            properties.insert(
                "RepoId".to_string(),
                crate::value::ToValue::to_value(&self.repo_id),
            );
            properties.insert(
                "RepoName".to_string(),
                crate::value::ToValue::to_value(&self.repo_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-devopsagent-association-gitlabconfiguration.html>
    pub struct GitLabConfiguration_ {
        pub enable_webhook_updates: Option<crate::value::ExpBool>,
        pub instance_identifier: Option<crate::value::ExpString>,
        pub project_id: crate::value::ExpString,
        pub project_path: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_devopsagent_Association_GitLabConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DevOpsAgent::Association.GitLabConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_devopsagent_Association_GitLabConfiguration as GitLabConfiguration;
    impl crate::value::ToValue for GitLabConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enable_webhook_updates {
                properties.insert(
                    "EnableWebhookUpdates".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.instance_identifier {
                properties.insert(
                    "InstanceIdentifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ProjectId".to_string(),
                crate::value::ToValue::to_value(&self.project_id),
            );
            properties.insert(
                "ProjectPath".to_string(),
                crate::value::ToValue::to_value(&self.project_path),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-devopsagent-association-keyvaluepair.html>
    pub struct KeyValuePair_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_devopsagent_Association_KeyValuePair {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DevOpsAgent::Association.KeyValuePair"
            $($field $value)*)
        };
    }
    pub use crate::__aws_devopsagent_Association_KeyValuePair as KeyValuePair;
    impl crate::value::ToValue for KeyValuePair_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-devopsagent-association-mcpserverconfiguration.html>
    pub struct MCPServerConfiguration_ {
        pub description: Option<crate::value::ExpString>,
        pub enable_webhook_updates: Option<crate::value::ExpBool>,
        pub endpoint: crate::value::ExpString,
        pub name: crate::value::ExpString,
        pub tools: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_devopsagent_Association_MCPServerConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DevOpsAgent::Association.MCPServerConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_devopsagent_Association_MCPServerConfiguration as MCPServerConfiguration;
    impl crate::value::ToValue for MCPServerConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enable_webhook_updates {
                properties.insert(
                    "EnableWebhookUpdates".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Endpoint".to_string(),
                crate::value::ToValue::to_value(&self.endpoint),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "Tools".to_string(),
                crate::value::ToValue::to_value(&self.tools),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-devopsagent-association-mcpserverdatadogconfiguration.html>
    pub struct MCPServerDatadogConfiguration_ {
        pub description: Option<crate::value::ExpString>,
        pub enable_webhook_updates: Option<crate::value::ExpBool>,
        pub endpoint: crate::value::ExpString,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_devopsagent_Association_MCPServerDatadogConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DevOpsAgent::Association.MCPServerDatadogConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_devopsagent_Association_MCPServerDatadogConfiguration as MCPServerDatadogConfiguration;
    impl crate::value::ToValue for MCPServerDatadogConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enable_webhook_updates {
                properties.insert(
                    "EnableWebhookUpdates".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Endpoint".to_string(),
                crate::value::ToValue::to_value(&self.endpoint),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-devopsagent-association-mcpservernewrelicconfiguration.html>
    pub struct MCPServerNewRelicConfiguration_ {
        pub account_id: crate::value::ExpString,
        pub endpoint: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_devopsagent_Association_MCPServerNewRelicConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DevOpsAgent::Association.MCPServerNewRelicConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_devopsagent_Association_MCPServerNewRelicConfiguration as MCPServerNewRelicConfiguration;
    impl crate::value::ToValue for MCPServerNewRelicConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AccountId".to_string(),
                crate::value::ToValue::to_value(&self.account_id),
            );
            properties.insert(
                "Endpoint".to_string(),
                crate::value::ToValue::to_value(&self.endpoint),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-devopsagent-association-mcpserversplunkconfiguration.html>
    pub struct MCPServerSplunkConfiguration_ {
        pub description: Option<crate::value::ExpString>,
        pub enable_webhook_updates: Option<crate::value::ExpBool>,
        pub endpoint: crate::value::ExpString,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_devopsagent_Association_MCPServerSplunkConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DevOpsAgent::Association.MCPServerSplunkConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_devopsagent_Association_MCPServerSplunkConfiguration as MCPServerSplunkConfiguration;
    impl crate::value::ToValue for MCPServerSplunkConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enable_webhook_updates {
                properties.insert(
                    "EnableWebhookUpdates".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Endpoint".to_string(),
                crate::value::ToValue::to_value(&self.endpoint),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-devopsagent-association-serviceconfiguration.html>
    pub struct ServiceConfiguration_ {
        pub aws: Option<Box<AWSConfiguration_>>,
        pub dynatrace: Option<Box<DynatraceConfiguration_>>,
        pub event_channel: Option<Box<EventChannelConfiguration_>>,
        pub git_hub: Option<Box<GitHubConfiguration_>>,
        pub git_lab: Option<Box<GitLabConfiguration_>>,
        pub mcp_server: Option<Box<MCPServerConfiguration_>>,
        pub mcp_server_datadog: Option<Box<MCPServerDatadogConfiguration_>>,
        pub mcp_server_new_relic: Option<Box<MCPServerNewRelicConfiguration_>>,
        pub mcp_server_splunk: Option<Box<MCPServerSplunkConfiguration_>>,
        pub service_now: Option<Box<ServiceNowConfiguration_>>,
        pub slack: Option<Box<SlackConfiguration_>>,
        pub source_aws: Option<Box<SourceAwsConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_devopsagent_Association_ServiceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DevOpsAgent::Association.ServiceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_devopsagent_Association_ServiceConfiguration as ServiceConfiguration;
    impl crate::value::ToValue for ServiceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.aws {
                properties.insert("Aws".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.dynatrace {
                properties.insert(
                    "Dynatrace".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.event_channel {
                properties.insert(
                    "EventChannel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.git_hub {
                properties.insert("GitHub".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.git_lab {
                properties.insert("GitLab".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.mcp_server {
                properties.insert(
                    "MCPServer".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mcp_server_datadog {
                properties.insert(
                    "MCPServerDatadog".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mcp_server_new_relic {
                properties.insert(
                    "MCPServerNewRelic".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mcp_server_splunk {
                properties.insert(
                    "MCPServerSplunk".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.service_now {
                properties.insert(
                    "ServiceNow".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.slack {
                properties.insert("Slack".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.source_aws {
                properties.insert(
                    "SourceAws".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-devopsagent-association-servicenowconfiguration.html>
    pub struct ServiceNowConfiguration_ {
        pub enable_webhook_updates: Option<crate::value::ExpBool>,
        pub instance_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_devopsagent_Association_ServiceNowConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DevOpsAgent::Association.ServiceNowConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_devopsagent_Association_ServiceNowConfiguration as ServiceNowConfiguration;
    impl crate::value::ToValue for ServiceNowConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enable_webhook_updates {
                properties.insert(
                    "EnableWebhookUpdates".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.instance_id {
                properties.insert(
                    "InstanceId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-devopsagent-association-slackchannel.html>
    pub struct SlackChannel_ {
        pub channel_id: crate::value::ExpString,
        pub channel_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_devopsagent_Association_SlackChannel {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DevOpsAgent::Association.SlackChannel"
            $($field $value)*)
        };
    }
    pub use crate::__aws_devopsagent_Association_SlackChannel as SlackChannel;
    impl crate::value::ToValue for SlackChannel_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ChannelId".to_string(),
                crate::value::ToValue::to_value(&self.channel_id),
            );
            if let Some(ref value) = self.channel_name {
                properties.insert(
                    "ChannelName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-devopsagent-association-slackconfiguration.html>
    pub struct SlackConfiguration_ {
        pub transmission_target: Box<SlackTransmissionTarget_>,
        pub workspace_id: crate::value::ExpString,
        pub workspace_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_devopsagent_Association_SlackConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DevOpsAgent::Association.SlackConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_devopsagent_Association_SlackConfiguration as SlackConfiguration;
    impl crate::value::ToValue for SlackConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TransmissionTarget".to_string(),
                crate::value::ToValue::to_value(&self.transmission_target),
            );
            properties.insert(
                "WorkspaceId".to_string(),
                crate::value::ToValue::to_value(&self.workspace_id),
            );
            properties.insert(
                "WorkspaceName".to_string(),
                crate::value::ToValue::to_value(&self.workspace_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-devopsagent-association-slacktransmissiontarget.html>
    pub struct SlackTransmissionTarget_ {
        pub incident_response_target: Box<SlackChannel_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_devopsagent_Association_SlackTransmissionTarget {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DevOpsAgent::Association.SlackTransmissionTarget"
            $($field $value)*)
        };
    }
    pub use crate::__aws_devopsagent_Association_SlackTransmissionTarget as SlackTransmissionTarget;
    impl crate::value::ToValue for SlackTransmissionTarget_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IncidentResponseTarget".to_string(),
                crate::value::ToValue::to_value(&self.incident_response_target),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-devopsagent-association-sourceawsconfiguration.html>
    pub struct SourceAwsConfiguration_ {
        pub account_id: crate::value::ExpString,
        pub account_type: crate::value::ExpString,
        pub assumable_role_arn: crate::value::ExpString,
        pub resources: Option<Vec<AWSResource_>>,
        pub tags: Option<Vec<KeyValuePair_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_devopsagent_Association_SourceAwsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DevOpsAgent::Association.SourceAwsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_devopsagent_Association_SourceAwsConfiguration as SourceAwsConfiguration;
    impl crate::value::ToValue for SourceAwsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AccountId".to_string(),
                crate::value::ToValue::to_value(&self.account_id),
            );
            properties.insert(
                "AccountType".to_string(),
                crate::value::ToValue::to_value(&self.account_type),
            );
            properties.insert(
                "AssumableRoleArn".to_string(),
                crate::value::ToValue::to_value(&self.assumable_role_arn),
            );
            if let Some(ref value) = self.resources {
                properties.insert(
                    "Resources".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tags {
                properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-devopsagent-agentspace.html>
pub struct AgentSpace_ {
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_devopsagent_AgentSpace {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DevOpsAgent::AgentSpace"
        $($field $value)*)
    };
}
pub use crate::__aws_devopsagent_AgentSpace as AgentSpace;
impl crate::template::ToResource for AgentSpace_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DevOpsAgent"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AgentSpace"),
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
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-devopsagent-association.html>
pub struct Association_ {
    pub agent_space_id: crate::value::ExpString,
    pub configuration: super::devopsagent::association::ServiceConfiguration_,
    pub linked_association_ids: Option<Vec<crate::value::ExpString>>,
    pub service_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_devopsagent_Association {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DevOpsAgent::Association"
        $($field $value)*)
    };
}
pub use crate::__aws_devopsagent_Association as Association;
impl crate::template::ToResource for Association_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DevOpsAgent"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Association"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AgentSpaceId".to_string(),
            crate::value::ToValue::to_value(&self.agent_space_id),
        );
        properties.insert(
            "Configuration".to_string(),
            crate::value::ToValue::to_value(&self.configuration),
        );
        if let Some(ref value) = self.linked_association_ids {
            properties.insert(
                "LinkedAssociationIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ServiceId".to_string(),
            crate::value::ToValue::to_value(&self.service_id),
        );
        properties
    }
}
