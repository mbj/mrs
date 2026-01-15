pub mod connector {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connector-connectorprovisioningconfig.html
    pub struct ConnectorProvisioningConfig_ {
        pub lambda: Option<Box<LambdaConnectorProvisioningConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Connector_ConnectorProvisioningConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Connector.ConnectorProvisioningConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Connector_ConnectorProvisioningConfig as ConnectorProvisioningConfig;
    impl crate::value::ToValue for ConnectorProvisioningConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.lambda {
                properties.insert("Lambda".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connector-lambdaconnectorprovisioningconfig.html
    pub struct LambdaConnectorProvisioningConfig_ {
        pub lambda_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Connector_LambdaConnectorProvisioningConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Connector.LambdaConnectorProvisioningConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Connector_LambdaConnectorProvisioningConfig as LambdaConnectorProvisioningConfig;
    impl crate::value::ToValue for LambdaConnectorProvisioningConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LambdaArn".to_string(),
                crate::value::ToValue::to_value(&self.lambda_arn),
            );
            properties.into()
        }
    }
}
pub mod connectorprofile {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-amplitudeconnectorprofilecredentials.html
    pub struct AmplitudeConnectorProfileCredentials_ {
        pub api_key: crate::value::ExpString,
        pub secret_key: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_AmplitudeConnectorProfileCredentials {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.AmplitudeConnectorProfileCredentials"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_AmplitudeConnectorProfileCredentials as AmplitudeConnectorProfileCredentials;
    impl crate::value::ToValue for AmplitudeConnectorProfileCredentials_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ApiKey".to_string(),
                crate::value::ToValue::to_value(&self.api_key),
            );
            properties.insert(
                "SecretKey".to_string(),
                crate::value::ToValue::to_value(&self.secret_key),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-apikeycredentials.html
    pub struct ApiKeyCredentials_ {
        pub api_key: crate::value::ExpString,
        pub api_secret_key: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_ApiKeyCredentials {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.ApiKeyCredentials"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_ApiKeyCredentials as ApiKeyCredentials;
    impl crate::value::ToValue for ApiKeyCredentials_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ApiKey".to_string(),
                crate::value::ToValue::to_value(&self.api_key),
            );
            if let Some(ref value) = self.api_secret_key {
                properties.insert(
                    "ApiSecretKey".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-basicauthcredentials.html
    pub struct BasicAuthCredentials_ {
        pub password: crate::value::ExpString,
        pub username: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_BasicAuthCredentials {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.BasicAuthCredentials"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_BasicAuthCredentials as BasicAuthCredentials;
    impl crate::value::ToValue for BasicAuthCredentials_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Password".to_string(),
                crate::value::ToValue::to_value(&self.password),
            );
            properties.insert(
                "Username".to_string(),
                crate::value::ToValue::to_value(&self.username),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectoroauthrequest.html
    pub struct ConnectorOAuthRequest_ {
        pub auth_code: Option<crate::value::ExpString>,
        pub redirect_uri: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_ConnectorOAuthRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.ConnectorOAuthRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_ConnectorOAuthRequest as ConnectorOAuthRequest;
    impl crate::value::ToValue for ConnectorOAuthRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auth_code {
                properties.insert(
                    "AuthCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.redirect_uri {
                properties.insert(
                    "RedirectUri".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofileconfig.html
    pub struct ConnectorProfileConfig_ {
        pub connector_profile_credentials: Option<Box<ConnectorProfileCredentials_>>,
        pub connector_profile_properties: Option<Box<ConnectorProfileProperties_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_ConnectorProfileConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.ConnectorProfileConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_ConnectorProfileConfig as ConnectorProfileConfig;
    impl crate::value::ToValue for ConnectorProfileConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.connector_profile_credentials {
                properties.insert(
                    "ConnectorProfileCredentials".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.connector_profile_properties {
                properties.insert(
                    "ConnectorProfileProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofilecredentials.html
    pub struct ConnectorProfileCredentials_ {
        pub amplitude: Option<Box<AmplitudeConnectorProfileCredentials_>>,
        pub custom_connector: Option<Box<CustomConnectorProfileCredentials_>>,
        pub datadog: Option<Box<DatadogConnectorProfileCredentials_>>,
        pub dynatrace: Option<Box<DynatraceConnectorProfileCredentials_>>,
        pub google_analytics: Option<Box<GoogleAnalyticsConnectorProfileCredentials_>>,
        pub infor_nexus: Option<Box<InforNexusConnectorProfileCredentials_>>,
        pub marketo: Option<Box<MarketoConnectorProfileCredentials_>>,
        pub pardot: Option<Box<PardotConnectorProfileCredentials_>>,
        pub redshift: Option<Box<RedshiftConnectorProfileCredentials_>>,
        pub sapo_data: Option<Box<SAPODataConnectorProfileCredentials_>>,
        pub salesforce: Option<Box<SalesforceConnectorProfileCredentials_>>,
        pub service_now: Option<Box<ServiceNowConnectorProfileCredentials_>>,
        pub singular: Option<Box<SingularConnectorProfileCredentials_>>,
        pub slack: Option<Box<SlackConnectorProfileCredentials_>>,
        pub snowflake: Option<Box<SnowflakeConnectorProfileCredentials_>>,
        pub trendmicro: Option<Box<TrendmicroConnectorProfileCredentials_>>,
        pub veeva: Option<Box<VeevaConnectorProfileCredentials_>>,
        pub zendesk: Option<Box<ZendeskConnectorProfileCredentials_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_ConnectorProfileCredentials {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.ConnectorProfileCredentials"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_ConnectorProfileCredentials as ConnectorProfileCredentials;
    impl crate::value::ToValue for ConnectorProfileCredentials_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.amplitude {
                properties.insert(
                    "Amplitude".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_connector {
                properties.insert(
                    "CustomConnector".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.datadog {
                properties.insert(
                    "Datadog".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dynatrace {
                properties.insert(
                    "Dynatrace".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.google_analytics {
                properties.insert(
                    "GoogleAnalytics".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.infor_nexus {
                properties.insert(
                    "InforNexus".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.marketo {
                properties.insert(
                    "Marketo".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pardot {
                properties.insert("Pardot".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.redshift {
                properties.insert(
                    "Redshift".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sapo_data {
                properties.insert(
                    "SAPOData".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.salesforce {
                properties.insert(
                    "Salesforce".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.service_now {
                properties.insert(
                    "ServiceNow".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.singular {
                properties.insert(
                    "Singular".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.slack {
                properties.insert("Slack".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.snowflake {
                properties.insert(
                    "Snowflake".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.trendmicro {
                properties.insert(
                    "Trendmicro".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.veeva {
                properties.insert("Veeva".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.zendesk {
                properties.insert(
                    "Zendesk".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofileproperties.html
    pub struct ConnectorProfileProperties_ {
        pub custom_connector: Option<Box<CustomConnectorProfileProperties_>>,
        pub datadog: Option<Box<DatadogConnectorProfileProperties_>>,
        pub dynatrace: Option<Box<DynatraceConnectorProfileProperties_>>,
        pub infor_nexus: Option<Box<InforNexusConnectorProfileProperties_>>,
        pub marketo: Option<Box<MarketoConnectorProfileProperties_>>,
        pub pardot: Option<Box<PardotConnectorProfileProperties_>>,
        pub redshift: Option<Box<RedshiftConnectorProfileProperties_>>,
        pub sapo_data: Option<Box<SAPODataConnectorProfileProperties_>>,
        pub salesforce: Option<Box<SalesforceConnectorProfileProperties_>>,
        pub service_now: Option<Box<ServiceNowConnectorProfileProperties_>>,
        pub slack: Option<Box<SlackConnectorProfileProperties_>>,
        pub snowflake: Option<Box<SnowflakeConnectorProfileProperties_>>,
        pub veeva: Option<Box<VeevaConnectorProfileProperties_>>,
        pub zendesk: Option<Box<ZendeskConnectorProfileProperties_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_ConnectorProfileProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.ConnectorProfileProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_ConnectorProfileProperties as ConnectorProfileProperties;
    impl crate::value::ToValue for ConnectorProfileProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_connector {
                properties.insert(
                    "CustomConnector".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.datadog {
                properties.insert(
                    "Datadog".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dynatrace {
                properties.insert(
                    "Dynatrace".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.infor_nexus {
                properties.insert(
                    "InforNexus".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.marketo {
                properties.insert(
                    "Marketo".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pardot {
                properties.insert("Pardot".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.redshift {
                properties.insert(
                    "Redshift".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sapo_data {
                properties.insert(
                    "SAPOData".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.salesforce {
                properties.insert(
                    "Salesforce".to_string(),
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
            if let Some(ref value) = self.snowflake {
                properties.insert(
                    "Snowflake".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.veeva {
                properties.insert("Veeva".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.zendesk {
                properties.insert(
                    "Zendesk".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-customauthcredentials.html
    pub struct CustomAuthCredentials_ {
        pub credentials_map: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub custom_authentication_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_CustomAuthCredentials {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.CustomAuthCredentials"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_CustomAuthCredentials as CustomAuthCredentials;
    impl crate::value::ToValue for CustomAuthCredentials_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.credentials_map {
                properties.insert(
                    "CredentialsMap".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "CustomAuthenticationType".to_string(),
                crate::value::ToValue::to_value(&self.custom_authentication_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-customconnectorprofilecredentials.html
    pub struct CustomConnectorProfileCredentials_ {
        pub api_key: Option<Box<ApiKeyCredentials_>>,
        pub authentication_type: crate::value::ExpString,
        pub basic: Option<Box<BasicAuthCredentials_>>,
        pub custom: Option<Box<CustomAuthCredentials_>>,
        pub oauth2: Option<Box<OAuth2Credentials_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_CustomConnectorProfileCredentials {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.CustomConnectorProfileCredentials"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_CustomConnectorProfileCredentials as CustomConnectorProfileCredentials;
    impl crate::value::ToValue for CustomConnectorProfileCredentials_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.api_key {
                properties.insert("ApiKey".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "AuthenticationType".to_string(),
                crate::value::ToValue::to_value(&self.authentication_type),
            );
            if let Some(ref value) = self.basic {
                properties.insert("Basic".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.custom {
                properties.insert("Custom".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.oauth2 {
                properties.insert("Oauth2".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-customconnectorprofileproperties.html
    pub struct CustomConnectorProfileProperties_ {
        pub o_auth2_properties: Option<Box<OAuth2Properties_>>,
        pub profile_properties: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_CustomConnectorProfileProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.CustomConnectorProfileProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_CustomConnectorProfileProperties as CustomConnectorProfileProperties;
    impl crate::value::ToValue for CustomConnectorProfileProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.o_auth2_properties {
                properties.insert(
                    "OAuth2Properties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.profile_properties {
                properties.insert(
                    "ProfileProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-datadogconnectorprofilecredentials.html
    pub struct DatadogConnectorProfileCredentials_ {
        pub api_key: crate::value::ExpString,
        pub application_key: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_DatadogConnectorProfileCredentials {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.DatadogConnectorProfileCredentials"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_DatadogConnectorProfileCredentials as DatadogConnectorProfileCredentials;
    impl crate::value::ToValue for DatadogConnectorProfileCredentials_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ApiKey".to_string(),
                crate::value::ToValue::to_value(&self.api_key),
            );
            properties.insert(
                "ApplicationKey".to_string(),
                crate::value::ToValue::to_value(&self.application_key),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-datadogconnectorprofileproperties.html
    pub struct DatadogConnectorProfileProperties_ {
        pub instance_url: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_DatadogConnectorProfileProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.DatadogConnectorProfileProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_DatadogConnectorProfileProperties as DatadogConnectorProfileProperties;
    impl crate::value::ToValue for DatadogConnectorProfileProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InstanceUrl".to_string(),
                crate::value::ToValue::to_value(&self.instance_url),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-dynatraceconnectorprofilecredentials.html
    pub struct DynatraceConnectorProfileCredentials_ {
        pub api_token: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_DynatraceConnectorProfileCredentials {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.DynatraceConnectorProfileCredentials"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_DynatraceConnectorProfileCredentials as DynatraceConnectorProfileCredentials;
    impl crate::value::ToValue for DynatraceConnectorProfileCredentials_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ApiToken".to_string(),
                crate::value::ToValue::to_value(&self.api_token),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-dynatraceconnectorprofileproperties.html
    pub struct DynatraceConnectorProfileProperties_ {
        pub instance_url: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_DynatraceConnectorProfileProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.DynatraceConnectorProfileProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_DynatraceConnectorProfileProperties as DynatraceConnectorProfileProperties;
    impl crate::value::ToValue for DynatraceConnectorProfileProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InstanceUrl".to_string(),
                crate::value::ToValue::to_value(&self.instance_url),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-googleanalyticsconnectorprofilecredentials.html
    pub struct GoogleAnalyticsConnectorProfileCredentials_ {
        pub access_token: Option<crate::value::ExpString>,
        pub client_id: crate::value::ExpString,
        pub client_secret: crate::value::ExpString,
        pub connector_o_auth_request: Option<Box<ConnectorOAuthRequest_>>,
        pub refresh_token: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_GoogleAnalyticsConnectorProfileCredentials {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.GoogleAnalyticsConnectorProfileCredentials"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_GoogleAnalyticsConnectorProfileCredentials as GoogleAnalyticsConnectorProfileCredentials;
    impl crate::value::ToValue for GoogleAnalyticsConnectorProfileCredentials_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.access_token {
                properties.insert(
                    "AccessToken".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ClientId".to_string(),
                crate::value::ToValue::to_value(&self.client_id),
            );
            properties.insert(
                "ClientSecret".to_string(),
                crate::value::ToValue::to_value(&self.client_secret),
            );
            if let Some(ref value) = self.connector_o_auth_request {
                properties.insert(
                    "ConnectorOAuthRequest".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.refresh_token {
                properties.insert(
                    "RefreshToken".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-infornexusconnectorprofilecredentials.html
    pub struct InforNexusConnectorProfileCredentials_ {
        pub access_key_id: crate::value::ExpString,
        pub datakey: crate::value::ExpString,
        pub secret_access_key: crate::value::ExpString,
        pub user_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_InforNexusConnectorProfileCredentials {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.InforNexusConnectorProfileCredentials"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_InforNexusConnectorProfileCredentials as InforNexusConnectorProfileCredentials;
    impl crate::value::ToValue for InforNexusConnectorProfileCredentials_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AccessKeyId".to_string(),
                crate::value::ToValue::to_value(&self.access_key_id),
            );
            properties.insert(
                "Datakey".to_string(),
                crate::value::ToValue::to_value(&self.datakey),
            );
            properties.insert(
                "SecretAccessKey".to_string(),
                crate::value::ToValue::to_value(&self.secret_access_key),
            );
            properties.insert(
                "UserId".to_string(),
                crate::value::ToValue::to_value(&self.user_id),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-infornexusconnectorprofileproperties.html
    pub struct InforNexusConnectorProfileProperties_ {
        pub instance_url: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_InforNexusConnectorProfileProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.InforNexusConnectorProfileProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_InforNexusConnectorProfileProperties as InforNexusConnectorProfileProperties;
    impl crate::value::ToValue for InforNexusConnectorProfileProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InstanceUrl".to_string(),
                crate::value::ToValue::to_value(&self.instance_url),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-marketoconnectorprofilecredentials.html
    pub struct MarketoConnectorProfileCredentials_ {
        pub access_token: Option<crate::value::ExpString>,
        pub client_id: crate::value::ExpString,
        pub client_secret: crate::value::ExpString,
        pub connector_o_auth_request: Option<Box<ConnectorOAuthRequest_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_MarketoConnectorProfileCredentials {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.MarketoConnectorProfileCredentials"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_MarketoConnectorProfileCredentials as MarketoConnectorProfileCredentials;
    impl crate::value::ToValue for MarketoConnectorProfileCredentials_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.access_token {
                properties.insert(
                    "AccessToken".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ClientId".to_string(),
                crate::value::ToValue::to_value(&self.client_id),
            );
            properties.insert(
                "ClientSecret".to_string(),
                crate::value::ToValue::to_value(&self.client_secret),
            );
            if let Some(ref value) = self.connector_o_auth_request {
                properties.insert(
                    "ConnectorOAuthRequest".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-marketoconnectorprofileproperties.html
    pub struct MarketoConnectorProfileProperties_ {
        pub instance_url: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_MarketoConnectorProfileProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.MarketoConnectorProfileProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_MarketoConnectorProfileProperties as MarketoConnectorProfileProperties;
    impl crate::value::ToValue for MarketoConnectorProfileProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InstanceUrl".to_string(),
                crate::value::ToValue::to_value(&self.instance_url),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-oauth2credentials.html
    pub struct OAuth2Credentials_ {
        pub access_token: Option<crate::value::ExpString>,
        pub client_id: Option<crate::value::ExpString>,
        pub client_secret: Option<crate::value::ExpString>,
        pub o_auth_request: Option<Box<ConnectorOAuthRequest_>>,
        pub refresh_token: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_OAuth2Credentials {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.OAuth2Credentials"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_OAuth2Credentials as OAuth2Credentials;
    impl crate::value::ToValue for OAuth2Credentials_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.access_token {
                properties.insert(
                    "AccessToken".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.client_id {
                properties.insert(
                    "ClientId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.client_secret {
                properties.insert(
                    "ClientSecret".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.o_auth_request {
                properties.insert(
                    "OAuthRequest".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.refresh_token {
                properties.insert(
                    "RefreshToken".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-oauth2properties.html
    pub struct OAuth2Properties_ {
        pub o_auth2_grant_type: Option<crate::value::ExpString>,
        pub token_url: Option<crate::value::ExpString>,
        pub token_url_custom_properties:
            Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_OAuth2Properties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.OAuth2Properties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_OAuth2Properties as OAuth2Properties;
    impl crate::value::ToValue for OAuth2Properties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.o_auth2_grant_type {
                properties.insert(
                    "OAuth2GrantType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.token_url {
                properties.insert(
                    "TokenUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.token_url_custom_properties {
                properties.insert(
                    "TokenUrlCustomProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-oauthcredentials.html
    pub struct OAuthCredentials_ {
        pub access_token: Option<crate::value::ExpString>,
        pub client_id: Option<crate::value::ExpString>,
        pub client_secret: Option<crate::value::ExpString>,
        pub connector_o_auth_request: Option<Box<ConnectorOAuthRequest_>>,
        pub refresh_token: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_OAuthCredentials {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.OAuthCredentials"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_OAuthCredentials as OAuthCredentials;
    impl crate::value::ToValue for OAuthCredentials_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.access_token {
                properties.insert(
                    "AccessToken".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.client_id {
                properties.insert(
                    "ClientId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.client_secret {
                properties.insert(
                    "ClientSecret".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.connector_o_auth_request {
                properties.insert(
                    "ConnectorOAuthRequest".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.refresh_token {
                properties.insert(
                    "RefreshToken".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-oauthproperties.html
    pub struct OAuthProperties_ {
        pub auth_code_url: Option<crate::value::ExpString>,
        pub o_auth_scopes: Option<Vec<crate::value::ExpString>>,
        pub token_url: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_OAuthProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.OAuthProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_OAuthProperties as OAuthProperties;
    impl crate::value::ToValue for OAuthProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auth_code_url {
                properties.insert(
                    "AuthCodeUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.o_auth_scopes {
                properties.insert(
                    "OAuthScopes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.token_url {
                properties.insert(
                    "TokenUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-pardotconnectorprofilecredentials.html
    pub struct PardotConnectorProfileCredentials_ {
        pub access_token: Option<crate::value::ExpString>,
        pub client_credentials_arn: Option<crate::value::ExpString>,
        pub connector_o_auth_request: Option<Box<ConnectorOAuthRequest_>>,
        pub refresh_token: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_PardotConnectorProfileCredentials {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.PardotConnectorProfileCredentials"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_PardotConnectorProfileCredentials as PardotConnectorProfileCredentials;
    impl crate::value::ToValue for PardotConnectorProfileCredentials_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.access_token {
                properties.insert(
                    "AccessToken".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.client_credentials_arn {
                properties.insert(
                    "ClientCredentialsArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.connector_o_auth_request {
                properties.insert(
                    "ConnectorOAuthRequest".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.refresh_token {
                properties.insert(
                    "RefreshToken".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-pardotconnectorprofileproperties.html
    pub struct PardotConnectorProfileProperties_ {
        pub business_unit_id: crate::value::ExpString,
        pub instance_url: Option<crate::value::ExpString>,
        pub is_sandbox_environment: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_PardotConnectorProfileProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.PardotConnectorProfileProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_PardotConnectorProfileProperties as PardotConnectorProfileProperties;
    impl crate::value::ToValue for PardotConnectorProfileProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BusinessUnitId".to_string(),
                crate::value::ToValue::to_value(&self.business_unit_id),
            );
            if let Some(ref value) = self.instance_url {
                properties.insert(
                    "InstanceUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.is_sandbox_environment {
                properties.insert(
                    "IsSandboxEnvironment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-redshiftconnectorprofilecredentials.html
    pub struct RedshiftConnectorProfileCredentials_ {
        pub password: Option<crate::value::ExpString>,
        pub username: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_RedshiftConnectorProfileCredentials {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.RedshiftConnectorProfileCredentials"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_RedshiftConnectorProfileCredentials as RedshiftConnectorProfileCredentials;
    impl crate::value::ToValue for RedshiftConnectorProfileCredentials_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.password {
                properties.insert(
                    "Password".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.username {
                properties.insert(
                    "Username".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-redshiftconnectorprofileproperties.html
    pub struct RedshiftConnectorProfileProperties_ {
        pub bucket_name: crate::value::ExpString,
        pub bucket_prefix: Option<crate::value::ExpString>,
        pub cluster_identifier: Option<crate::value::ExpString>,
        pub data_api_role_arn: Option<crate::value::ExpString>,
        pub database_name: Option<crate::value::ExpString>,
        pub database_url: Option<crate::value::ExpString>,
        pub is_redshift_serverless: Option<crate::value::ExpBool>,
        pub role_arn: crate::value::ExpString,
        pub workgroup_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_RedshiftConnectorProfileProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.RedshiftConnectorProfileProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_RedshiftConnectorProfileProperties as RedshiftConnectorProfileProperties;
    impl crate::value::ToValue for RedshiftConnectorProfileProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BucketName".to_string(),
                crate::value::ToValue::to_value(&self.bucket_name),
            );
            if let Some(ref value) = self.bucket_prefix {
                properties.insert(
                    "BucketPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cluster_identifier {
                properties.insert(
                    "ClusterIdentifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.data_api_role_arn {
                properties.insert(
                    "DataApiRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.database_name {
                properties.insert(
                    "DatabaseName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.database_url {
                properties.insert(
                    "DatabaseUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.is_redshift_serverless {
                properties.insert(
                    "IsRedshiftServerless".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            if let Some(ref value) = self.workgroup_name {
                properties.insert(
                    "WorkgroupName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-sapodataconnectorprofilecredentials.html
    pub struct SAPODataConnectorProfileCredentials_ {
        pub basic_auth_credentials: Option<Box<BasicAuthCredentials_>>,
        pub o_auth_credentials: Option<Box<OAuthCredentials_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_SAPODataConnectorProfileCredentials {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.SAPODataConnectorProfileCredentials"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_SAPODataConnectorProfileCredentials as SAPODataConnectorProfileCredentials;
    impl crate::value::ToValue for SAPODataConnectorProfileCredentials_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.basic_auth_credentials {
                properties.insert(
                    "BasicAuthCredentials".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.o_auth_credentials {
                properties.insert(
                    "OAuthCredentials".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-sapodataconnectorprofileproperties.html
    pub struct SAPODataConnectorProfileProperties_ {
        pub application_host_url: Option<crate::value::ExpString>,
        pub application_service_path: Option<crate::value::ExpString>,
        pub client_number: Option<crate::value::ExpString>,
        pub disable_sso: Option<crate::value::ExpBool>,
        pub logon_language: Option<crate::value::ExpString>,
        pub o_auth_properties: Option<Box<OAuthProperties_>>,
        pub port_number: Option<i64>,
        pub private_link_service_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_SAPODataConnectorProfileProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.SAPODataConnectorProfileProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_SAPODataConnectorProfileProperties as SAPODataConnectorProfileProperties;
    impl crate::value::ToValue for SAPODataConnectorProfileProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.application_host_url {
                properties.insert(
                    "ApplicationHostUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.application_service_path {
                properties.insert(
                    "ApplicationServicePath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.client_number {
                properties.insert(
                    "ClientNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.disable_sso {
                properties.insert(
                    "DisableSSO".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.logon_language {
                properties.insert(
                    "LogonLanguage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.o_auth_properties {
                properties.insert(
                    "OAuthProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.port_number {
                properties.insert(
                    "PortNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.private_link_service_name {
                properties.insert(
                    "PrivateLinkServiceName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-salesforceconnectorprofilecredentials.html
    pub struct SalesforceConnectorProfileCredentials_ {
        pub access_token: Option<crate::value::ExpString>,
        pub client_credentials_arn: Option<crate::value::ExpString>,
        pub connector_o_auth_request: Option<Box<ConnectorOAuthRequest_>>,
        pub jwt_token: Option<crate::value::ExpString>,
        pub o_auth2_grant_type: Option<crate::value::ExpString>,
        pub refresh_token: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_SalesforceConnectorProfileCredentials {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.SalesforceConnectorProfileCredentials"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_SalesforceConnectorProfileCredentials as SalesforceConnectorProfileCredentials;
    impl crate::value::ToValue for SalesforceConnectorProfileCredentials_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.access_token {
                properties.insert(
                    "AccessToken".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.client_credentials_arn {
                properties.insert(
                    "ClientCredentialsArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.connector_o_auth_request {
                properties.insert(
                    "ConnectorOAuthRequest".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.jwt_token {
                properties.insert(
                    "JwtToken".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.o_auth2_grant_type {
                properties.insert(
                    "OAuth2GrantType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.refresh_token {
                properties.insert(
                    "RefreshToken".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-salesforceconnectorprofileproperties.html
    pub struct SalesforceConnectorProfileProperties_ {
        pub instance_url: Option<crate::value::ExpString>,
        pub is_sandbox_environment: Option<crate::value::ExpBool>,
        pub use_private_link_for_metadata_and_authorization: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_SalesforceConnectorProfileProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.SalesforceConnectorProfileProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_SalesforceConnectorProfileProperties as SalesforceConnectorProfileProperties;
    impl crate::value::ToValue for SalesforceConnectorProfileProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.instance_url {
                properties.insert(
                    "InstanceUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.is_sandbox_environment {
                properties.insert(
                    "isSandboxEnvironment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.use_private_link_for_metadata_and_authorization {
                properties.insert(
                    "usePrivateLinkForMetadataAndAuthorization".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-servicenowconnectorprofilecredentials.html
    pub struct ServiceNowConnectorProfileCredentials_ {
        pub o_auth2_credentials: Option<Box<OAuth2Credentials_>>,
        pub password: Option<crate::value::ExpString>,
        pub username: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_ServiceNowConnectorProfileCredentials {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.ServiceNowConnectorProfileCredentials"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_ServiceNowConnectorProfileCredentials as ServiceNowConnectorProfileCredentials;
    impl crate::value::ToValue for ServiceNowConnectorProfileCredentials_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.o_auth2_credentials {
                properties.insert(
                    "OAuth2Credentials".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.password {
                properties.insert(
                    "Password".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.username {
                properties.insert(
                    "Username".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-servicenowconnectorprofileproperties.html
    pub struct ServiceNowConnectorProfileProperties_ {
        pub instance_url: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_ServiceNowConnectorProfileProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.ServiceNowConnectorProfileProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_ServiceNowConnectorProfileProperties as ServiceNowConnectorProfileProperties;
    impl crate::value::ToValue for ServiceNowConnectorProfileProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InstanceUrl".to_string(),
                crate::value::ToValue::to_value(&self.instance_url),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-singularconnectorprofilecredentials.html
    pub struct SingularConnectorProfileCredentials_ {
        pub api_key: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_SingularConnectorProfileCredentials {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.SingularConnectorProfileCredentials"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_SingularConnectorProfileCredentials as SingularConnectorProfileCredentials;
    impl crate::value::ToValue for SingularConnectorProfileCredentials_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ApiKey".to_string(),
                crate::value::ToValue::to_value(&self.api_key),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-slackconnectorprofilecredentials.html
    pub struct SlackConnectorProfileCredentials_ {
        pub access_token: Option<crate::value::ExpString>,
        pub client_id: crate::value::ExpString,
        pub client_secret: crate::value::ExpString,
        pub connector_o_auth_request: Option<Box<ConnectorOAuthRequest_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_SlackConnectorProfileCredentials {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.SlackConnectorProfileCredentials"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_SlackConnectorProfileCredentials as SlackConnectorProfileCredentials;
    impl crate::value::ToValue for SlackConnectorProfileCredentials_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.access_token {
                properties.insert(
                    "AccessToken".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ClientId".to_string(),
                crate::value::ToValue::to_value(&self.client_id),
            );
            properties.insert(
                "ClientSecret".to_string(),
                crate::value::ToValue::to_value(&self.client_secret),
            );
            if let Some(ref value) = self.connector_o_auth_request {
                properties.insert(
                    "ConnectorOAuthRequest".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-slackconnectorprofileproperties.html
    pub struct SlackConnectorProfileProperties_ {
        pub instance_url: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_SlackConnectorProfileProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.SlackConnectorProfileProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_SlackConnectorProfileProperties as SlackConnectorProfileProperties;
    impl crate::value::ToValue for SlackConnectorProfileProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InstanceUrl".to_string(),
                crate::value::ToValue::to_value(&self.instance_url),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-snowflakeconnectorprofilecredentials.html
    pub struct SnowflakeConnectorProfileCredentials_ {
        pub password: crate::value::ExpString,
        pub username: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_SnowflakeConnectorProfileCredentials {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.SnowflakeConnectorProfileCredentials"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_SnowflakeConnectorProfileCredentials as SnowflakeConnectorProfileCredentials;
    impl crate::value::ToValue for SnowflakeConnectorProfileCredentials_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Password".to_string(),
                crate::value::ToValue::to_value(&self.password),
            );
            properties.insert(
                "Username".to_string(),
                crate::value::ToValue::to_value(&self.username),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-snowflakeconnectorprofileproperties.html
    pub struct SnowflakeConnectorProfileProperties_ {
        pub account_name: Option<crate::value::ExpString>,
        pub bucket_name: crate::value::ExpString,
        pub bucket_prefix: Option<crate::value::ExpString>,
        pub private_link_service_name: Option<crate::value::ExpString>,
        pub region: Option<crate::value::ExpString>,
        pub stage: crate::value::ExpString,
        pub warehouse: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_SnowflakeConnectorProfileProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.SnowflakeConnectorProfileProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_SnowflakeConnectorProfileProperties as SnowflakeConnectorProfileProperties;
    impl crate::value::ToValue for SnowflakeConnectorProfileProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.account_name {
                properties.insert(
                    "AccountName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "BucketName".to_string(),
                crate::value::ToValue::to_value(&self.bucket_name),
            );
            if let Some(ref value) = self.bucket_prefix {
                properties.insert(
                    "BucketPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.private_link_service_name {
                properties.insert(
                    "PrivateLinkServiceName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.region {
                properties.insert("Region".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Stage".to_string(),
                crate::value::ToValue::to_value(&self.stage),
            );
            properties.insert(
                "Warehouse".to_string(),
                crate::value::ToValue::to_value(&self.warehouse),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-trendmicroconnectorprofilecredentials.html
    pub struct TrendmicroConnectorProfileCredentials_ {
        pub api_secret_key: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_TrendmicroConnectorProfileCredentials {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.TrendmicroConnectorProfileCredentials"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_TrendmicroConnectorProfileCredentials as TrendmicroConnectorProfileCredentials;
    impl crate::value::ToValue for TrendmicroConnectorProfileCredentials_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ApiSecretKey".to_string(),
                crate::value::ToValue::to_value(&self.api_secret_key),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-veevaconnectorprofilecredentials.html
    pub struct VeevaConnectorProfileCredentials_ {
        pub password: crate::value::ExpString,
        pub username: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_VeevaConnectorProfileCredentials {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.VeevaConnectorProfileCredentials"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_VeevaConnectorProfileCredentials as VeevaConnectorProfileCredentials;
    impl crate::value::ToValue for VeevaConnectorProfileCredentials_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Password".to_string(),
                crate::value::ToValue::to_value(&self.password),
            );
            properties.insert(
                "Username".to_string(),
                crate::value::ToValue::to_value(&self.username),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-veevaconnectorprofileproperties.html
    pub struct VeevaConnectorProfileProperties_ {
        pub instance_url: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_VeevaConnectorProfileProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.VeevaConnectorProfileProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_VeevaConnectorProfileProperties as VeevaConnectorProfileProperties;
    impl crate::value::ToValue for VeevaConnectorProfileProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InstanceUrl".to_string(),
                crate::value::ToValue::to_value(&self.instance_url),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-zendeskconnectorprofilecredentials.html
    pub struct ZendeskConnectorProfileCredentials_ {
        pub access_token: Option<crate::value::ExpString>,
        pub client_id: crate::value::ExpString,
        pub client_secret: crate::value::ExpString,
        pub connector_o_auth_request: Option<Box<ConnectorOAuthRequest_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_ZendeskConnectorProfileCredentials {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.ZendeskConnectorProfileCredentials"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_ZendeskConnectorProfileCredentials as ZendeskConnectorProfileCredentials;
    impl crate::value::ToValue for ZendeskConnectorProfileCredentials_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.access_token {
                properties.insert(
                    "AccessToken".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ClientId".to_string(),
                crate::value::ToValue::to_value(&self.client_id),
            );
            properties.insert(
                "ClientSecret".to_string(),
                crate::value::ToValue::to_value(&self.client_secret),
            );
            if let Some(ref value) = self.connector_o_auth_request {
                properties.insert(
                    "ConnectorOAuthRequest".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-zendeskconnectorprofileproperties.html
    pub struct ZendeskConnectorProfileProperties_ {
        pub instance_url: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_ConnectorProfile_ZendeskConnectorProfileProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::ConnectorProfile.ZendeskConnectorProfileProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_ConnectorProfile_ZendeskConnectorProfileProperties as ZendeskConnectorProfileProperties;
    impl crate::value::ToValue for ZendeskConnectorProfileProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InstanceUrl".to_string(),
                crate::value::ToValue::to_value(&self.instance_url),
            );
            properties.into()
        }
    }
}
pub mod flow {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-aggregationconfig.html
    pub struct AggregationConfig_ {
        pub aggregation_type: Option<crate::value::ExpString>,
        pub target_file_size: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_AggregationConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.AggregationConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_AggregationConfig as AggregationConfig;
    impl crate::value::ToValue for AggregationConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.aggregation_type {
                properties.insert(
                    "AggregationType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_file_size {
                properties.insert(
                    "TargetFileSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-amplitudesourceproperties.html
    pub struct AmplitudeSourceProperties_ {
        pub object: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_AmplitudeSourceProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.AmplitudeSourceProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_AmplitudeSourceProperties as AmplitudeSourceProperties;
    impl crate::value::ToValue for AmplitudeSourceProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Object".to_string(),
                crate::value::ToValue::to_value(&self.object),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-connectoroperator.html
    pub struct ConnectorOperator_ {
        pub amplitude: Option<crate::value::ExpString>,
        pub custom_connector: Option<crate::value::ExpString>,
        pub datadog: Option<crate::value::ExpString>,
        pub dynatrace: Option<crate::value::ExpString>,
        pub google_analytics: Option<crate::value::ExpString>,
        pub infor_nexus: Option<crate::value::ExpString>,
        pub marketo: Option<crate::value::ExpString>,
        pub pardot: Option<crate::value::ExpString>,
        pub s3: Option<crate::value::ExpString>,
        pub sapo_data: Option<crate::value::ExpString>,
        pub salesforce: Option<crate::value::ExpString>,
        pub service_now: Option<crate::value::ExpString>,
        pub singular: Option<crate::value::ExpString>,
        pub slack: Option<crate::value::ExpString>,
        pub trendmicro: Option<crate::value::ExpString>,
        pub veeva: Option<crate::value::ExpString>,
        pub zendesk: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_ConnectorOperator {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.ConnectorOperator"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_ConnectorOperator as ConnectorOperator;
    impl crate::value::ToValue for ConnectorOperator_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.amplitude {
                properties.insert(
                    "Amplitude".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_connector {
                properties.insert(
                    "CustomConnector".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.datadog {
                properties.insert(
                    "Datadog".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dynatrace {
                properties.insert(
                    "Dynatrace".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.google_analytics {
                properties.insert(
                    "GoogleAnalytics".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.infor_nexus {
                properties.insert(
                    "InforNexus".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.marketo {
                properties.insert(
                    "Marketo".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pardot {
                properties.insert("Pardot".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.s3 {
                properties.insert("S3".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.sapo_data {
                properties.insert(
                    "SAPOData".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.salesforce {
                properties.insert(
                    "Salesforce".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.service_now {
                properties.insert(
                    "ServiceNow".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.singular {
                properties.insert(
                    "Singular".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.slack {
                properties.insert("Slack".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.trendmicro {
                properties.insert(
                    "Trendmicro".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.veeva {
                properties.insert("Veeva".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.zendesk {
                properties.insert(
                    "Zendesk".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-customconnectordestinationproperties.html
    pub struct CustomConnectorDestinationProperties_ {
        pub custom_properties: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub entity_name: crate::value::ExpString,
        pub error_handling_config: Option<Box<ErrorHandlingConfig_>>,
        pub id_field_names: Option<Vec<crate::value::ExpString>>,
        pub write_operation_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_CustomConnectorDestinationProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.CustomConnectorDestinationProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_CustomConnectorDestinationProperties as CustomConnectorDestinationProperties;
    impl crate::value::ToValue for CustomConnectorDestinationProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_properties {
                properties.insert(
                    "CustomProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "EntityName".to_string(),
                crate::value::ToValue::to_value(&self.entity_name),
            );
            if let Some(ref value) = self.error_handling_config {
                properties.insert(
                    "ErrorHandlingConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.id_field_names {
                properties.insert(
                    "IdFieldNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.write_operation_type {
                properties.insert(
                    "WriteOperationType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-customconnectorsourceproperties.html
    pub struct CustomConnectorSourceProperties_ {
        pub custom_properties: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub data_transfer_api: Option<Box<DataTransferApi_>>,
        pub entity_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_CustomConnectorSourceProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.CustomConnectorSourceProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_CustomConnectorSourceProperties as CustomConnectorSourceProperties;
    impl crate::value::ToValue for CustomConnectorSourceProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_properties {
                properties.insert(
                    "CustomProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.data_transfer_api {
                properties.insert(
                    "DataTransferApi".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "EntityName".to_string(),
                crate::value::ToValue::to_value(&self.entity_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-datatransferapi.html
    pub struct DataTransferApi_ {
        pub name: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_DataTransferApi {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.DataTransferApi"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_DataTransferApi as DataTransferApi;
    impl crate::value::ToValue for DataTransferApi_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-datadogsourceproperties.html
    pub struct DatadogSourceProperties_ {
        pub object: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_DatadogSourceProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.DatadogSourceProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_DatadogSourceProperties as DatadogSourceProperties;
    impl crate::value::ToValue for DatadogSourceProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Object".to_string(),
                crate::value::ToValue::to_value(&self.object),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-destinationconnectorproperties.html
    pub struct DestinationConnectorProperties_ {
        pub custom_connector: Option<Box<CustomConnectorDestinationProperties_>>,
        pub event_bridge: Option<Box<EventBridgeDestinationProperties_>>,
        pub lookout_metrics: Option<Box<LookoutMetricsDestinationProperties_>>,
        pub marketo: Option<Box<MarketoDestinationProperties_>>,
        pub redshift: Option<Box<RedshiftDestinationProperties_>>,
        pub s3: Option<Box<S3DestinationProperties_>>,
        pub sapo_data: Option<Box<SAPODataDestinationProperties_>>,
        pub salesforce: Option<Box<SalesforceDestinationProperties_>>,
        pub snowflake: Option<Box<SnowflakeDestinationProperties_>>,
        pub upsolver: Option<Box<UpsolverDestinationProperties_>>,
        pub zendesk: Option<Box<ZendeskDestinationProperties_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_DestinationConnectorProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.DestinationConnectorProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_DestinationConnectorProperties as DestinationConnectorProperties;
    impl crate::value::ToValue for DestinationConnectorProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_connector {
                properties.insert(
                    "CustomConnector".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.event_bridge {
                properties.insert(
                    "EventBridge".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lookout_metrics {
                properties.insert(
                    "LookoutMetrics".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.marketo {
                properties.insert(
                    "Marketo".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.redshift {
                properties.insert(
                    "Redshift".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3 {
                properties.insert("S3".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.sapo_data {
                properties.insert(
                    "SAPOData".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.salesforce {
                properties.insert(
                    "Salesforce".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.snowflake {
                properties.insert(
                    "Snowflake".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.upsolver {
                properties.insert(
                    "Upsolver".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.zendesk {
                properties.insert(
                    "Zendesk".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-destinationflowconfig.html
    pub struct DestinationFlowConfig_ {
        pub api_version: Option<crate::value::ExpString>,
        pub connector_profile_name: Option<crate::value::ExpString>,
        pub connector_type: crate::value::ExpString,
        pub destination_connector_properties: Box<DestinationConnectorProperties_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_DestinationFlowConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.DestinationFlowConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_DestinationFlowConfig as DestinationFlowConfig;
    impl crate::value::ToValue for DestinationFlowConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.api_version {
                properties.insert(
                    "ApiVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.connector_profile_name {
                properties.insert(
                    "ConnectorProfileName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ConnectorType".to_string(),
                crate::value::ToValue::to_value(&self.connector_type),
            );
            properties.insert(
                "DestinationConnectorProperties".to_string(),
                crate::value::ToValue::to_value(&self.destination_connector_properties),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-dynatracesourceproperties.html
    pub struct DynatraceSourceProperties_ {
        pub object: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_DynatraceSourceProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.DynatraceSourceProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_DynatraceSourceProperties as DynatraceSourceProperties;
    impl crate::value::ToValue for DynatraceSourceProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Object".to_string(),
                crate::value::ToValue::to_value(&self.object),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-errorhandlingconfig.html
    pub struct ErrorHandlingConfig_ {
        pub bucket_name: Option<crate::value::ExpString>,
        pub bucket_prefix: Option<crate::value::ExpString>,
        pub fail_on_first_error: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_ErrorHandlingConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.ErrorHandlingConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_ErrorHandlingConfig as ErrorHandlingConfig;
    impl crate::value::ToValue for ErrorHandlingConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bucket_name {
                properties.insert(
                    "BucketName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bucket_prefix {
                properties.insert(
                    "BucketPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.fail_on_first_error {
                properties.insert(
                    "FailOnFirstError".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-eventbridgedestinationproperties.html
    pub struct EventBridgeDestinationProperties_ {
        pub error_handling_config: Option<Box<ErrorHandlingConfig_>>,
        pub object: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_EventBridgeDestinationProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.EventBridgeDestinationProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_EventBridgeDestinationProperties as EventBridgeDestinationProperties;
    impl crate::value::ToValue for EventBridgeDestinationProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.error_handling_config {
                properties.insert(
                    "ErrorHandlingConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Object".to_string(),
                crate::value::ToValue::to_value(&self.object),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-gluedatacatalog.html
    pub struct GlueDataCatalog_ {
        pub database_name: crate::value::ExpString,
        pub role_arn: crate::value::ExpString,
        pub table_prefix: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_GlueDataCatalog {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.GlueDataCatalog"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_GlueDataCatalog as GlueDataCatalog;
    impl crate::value::ToValue for GlueDataCatalog_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DatabaseName".to_string(),
                crate::value::ToValue::to_value(&self.database_name),
            );
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.insert(
                "TablePrefix".to_string(),
                crate::value::ToValue::to_value(&self.table_prefix),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-googleanalyticssourceproperties.html
    pub struct GoogleAnalyticsSourceProperties_ {
        pub object: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_GoogleAnalyticsSourceProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.GoogleAnalyticsSourceProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_GoogleAnalyticsSourceProperties as GoogleAnalyticsSourceProperties;
    impl crate::value::ToValue for GoogleAnalyticsSourceProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Object".to_string(),
                crate::value::ToValue::to_value(&self.object),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-incrementalpullconfig.html
    pub struct IncrementalPullConfig_ {
        pub datetime_type_field_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_IncrementalPullConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.IncrementalPullConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_IncrementalPullConfig as IncrementalPullConfig;
    impl crate::value::ToValue for IncrementalPullConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.datetime_type_field_name {
                properties.insert(
                    "DatetimeTypeFieldName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-infornexussourceproperties.html
    pub struct InforNexusSourceProperties_ {
        pub object: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_InforNexusSourceProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.InforNexusSourceProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_InforNexusSourceProperties as InforNexusSourceProperties;
    impl crate::value::ToValue for InforNexusSourceProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Object".to_string(),
                crate::value::ToValue::to_value(&self.object),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-lookoutmetricsdestinationproperties.html
    pub struct LookoutMetricsDestinationProperties_ {
        pub object: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_LookoutMetricsDestinationProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.LookoutMetricsDestinationProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_LookoutMetricsDestinationProperties as LookoutMetricsDestinationProperties;
    impl crate::value::ToValue for LookoutMetricsDestinationProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.object {
                properties.insert("Object".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-marketodestinationproperties.html
    pub struct MarketoDestinationProperties_ {
        pub error_handling_config: Option<Box<ErrorHandlingConfig_>>,
        pub object: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_MarketoDestinationProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.MarketoDestinationProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_MarketoDestinationProperties as MarketoDestinationProperties;
    impl crate::value::ToValue for MarketoDestinationProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.error_handling_config {
                properties.insert(
                    "ErrorHandlingConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Object".to_string(),
                crate::value::ToValue::to_value(&self.object),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-marketosourceproperties.html
    pub struct MarketoSourceProperties_ {
        pub object: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_MarketoSourceProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.MarketoSourceProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_MarketoSourceProperties as MarketoSourceProperties;
    impl crate::value::ToValue for MarketoSourceProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Object".to_string(),
                crate::value::ToValue::to_value(&self.object),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-metadatacatalogconfig.html
    pub struct MetadataCatalogConfig_ {
        pub glue_data_catalog: Option<Box<GlueDataCatalog_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_MetadataCatalogConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.MetadataCatalogConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_MetadataCatalogConfig as MetadataCatalogConfig;
    impl crate::value::ToValue for MetadataCatalogConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.glue_data_catalog {
                properties.insert(
                    "GlueDataCatalog".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-pardotsourceproperties.html
    pub struct PardotSourceProperties_ {
        pub object: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_PardotSourceProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.PardotSourceProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_PardotSourceProperties as PardotSourceProperties;
    impl crate::value::ToValue for PardotSourceProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Object".to_string(),
                crate::value::ToValue::to_value(&self.object),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-prefixconfig.html
    pub struct PrefixConfig_ {
        pub path_prefix_hierarchy: Option<Vec<crate::value::ExpString>>,
        pub prefix_format: Option<crate::value::ExpString>,
        pub prefix_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_PrefixConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.PrefixConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_PrefixConfig as PrefixConfig;
    impl crate::value::ToValue for PrefixConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.path_prefix_hierarchy {
                properties.insert(
                    "PathPrefixHierarchy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.prefix_format {
                properties.insert(
                    "PrefixFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.prefix_type {
                properties.insert(
                    "PrefixType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-redshiftdestinationproperties.html
    pub struct RedshiftDestinationProperties_ {
        pub bucket_prefix: Option<crate::value::ExpString>,
        pub error_handling_config: Option<Box<ErrorHandlingConfig_>>,
        pub intermediate_bucket_name: crate::value::ExpString,
        pub object: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_RedshiftDestinationProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.RedshiftDestinationProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_RedshiftDestinationProperties as RedshiftDestinationProperties;
    impl crate::value::ToValue for RedshiftDestinationProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bucket_prefix {
                properties.insert(
                    "BucketPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.error_handling_config {
                properties.insert(
                    "ErrorHandlingConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "IntermediateBucketName".to_string(),
                crate::value::ToValue::to_value(&self.intermediate_bucket_name),
            );
            properties.insert(
                "Object".to_string(),
                crate::value::ToValue::to_value(&self.object),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-s3destinationproperties.html
    pub struct S3DestinationProperties_ {
        pub bucket_name: crate::value::ExpString,
        pub bucket_prefix: Option<crate::value::ExpString>,
        pub s3_output_format_config: Option<Box<S3OutputFormatConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_S3DestinationProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.S3DestinationProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_S3DestinationProperties as S3DestinationProperties;
    impl crate::value::ToValue for S3DestinationProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BucketName".to_string(),
                crate::value::ToValue::to_value(&self.bucket_name),
            );
            if let Some(ref value) = self.bucket_prefix {
                properties.insert(
                    "BucketPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_output_format_config {
                properties.insert(
                    "S3OutputFormatConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-s3inputformatconfig.html
    pub struct S3InputFormatConfig_ {
        pub s3_input_file_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_S3InputFormatConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.S3InputFormatConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_S3InputFormatConfig as S3InputFormatConfig;
    impl crate::value::ToValue for S3InputFormatConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.s3_input_file_type {
                properties.insert(
                    "S3InputFileType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-s3outputformatconfig.html
    pub struct S3OutputFormatConfig_ {
        pub aggregation_config: Option<Box<AggregationConfig_>>,
        pub file_type: Option<crate::value::ExpString>,
        pub prefix_config: Option<Box<PrefixConfig_>>,
        pub preserve_source_data_typing: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_S3OutputFormatConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.S3OutputFormatConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_S3OutputFormatConfig as S3OutputFormatConfig;
    impl crate::value::ToValue for S3OutputFormatConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.aggregation_config {
                properties.insert(
                    "AggregationConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.file_type {
                properties.insert(
                    "FileType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.prefix_config {
                properties.insert(
                    "PrefixConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.preserve_source_data_typing {
                properties.insert(
                    "PreserveSourceDataTyping".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-s3sourceproperties.html
    pub struct S3SourceProperties_ {
        pub bucket_name: crate::value::ExpString,
        pub bucket_prefix: crate::value::ExpString,
        pub s3_input_format_config: Option<Box<S3InputFormatConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_S3SourceProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.S3SourceProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_S3SourceProperties as S3SourceProperties;
    impl crate::value::ToValue for S3SourceProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BucketName".to_string(),
                crate::value::ToValue::to_value(&self.bucket_name),
            );
            properties.insert(
                "BucketPrefix".to_string(),
                crate::value::ToValue::to_value(&self.bucket_prefix),
            );
            if let Some(ref value) = self.s3_input_format_config {
                properties.insert(
                    "S3InputFormatConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sapodatadestinationproperties.html
    pub struct SAPODataDestinationProperties_ {
        pub error_handling_config: Option<Box<ErrorHandlingConfig_>>,
        pub id_field_names: Option<Vec<crate::value::ExpString>>,
        pub object_path: crate::value::ExpString,
        pub success_response_handling_config: Option<Box<SuccessResponseHandlingConfig_>>,
        pub write_operation_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_SAPODataDestinationProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.SAPODataDestinationProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_SAPODataDestinationProperties as SAPODataDestinationProperties;
    impl crate::value::ToValue for SAPODataDestinationProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.error_handling_config {
                properties.insert(
                    "ErrorHandlingConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.id_field_names {
                properties.insert(
                    "IdFieldNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ObjectPath".to_string(),
                crate::value::ToValue::to_value(&self.object_path),
            );
            if let Some(ref value) = self.success_response_handling_config {
                properties.insert(
                    "SuccessResponseHandlingConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.write_operation_type {
                properties.insert(
                    "WriteOperationType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sapodatapaginationconfig.html
    pub struct SAPODataPaginationConfig_ {
        pub max_page_size: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_SAPODataPaginationConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.SAPODataPaginationConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_SAPODataPaginationConfig as SAPODataPaginationConfig;
    impl crate::value::ToValue for SAPODataPaginationConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "maxPageSize".to_string(),
                crate::value::ToValue::to_value(&self.max_page_size),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sapodataparallelismconfig.html
    pub struct SAPODataParallelismConfig_ {
        pub max_parallelism: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_SAPODataParallelismConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.SAPODataParallelismConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_SAPODataParallelismConfig as SAPODataParallelismConfig;
    impl crate::value::ToValue for SAPODataParallelismConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "maxParallelism".to_string(),
                crate::value::ToValue::to_value(&self.max_parallelism),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sapodatasourceproperties.html
    pub struct SAPODataSourceProperties_ {
        pub object_path: crate::value::ExpString,
        pub pagination_config: Option<Box<SAPODataPaginationConfig_>>,
        pub parallelism_config: Option<Box<SAPODataParallelismConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_SAPODataSourceProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.SAPODataSourceProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_SAPODataSourceProperties as SAPODataSourceProperties;
    impl crate::value::ToValue for SAPODataSourceProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ObjectPath".to_string(),
                crate::value::ToValue::to_value(&self.object_path),
            );
            if let Some(ref value) = self.pagination_config {
                properties.insert(
                    "paginationConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parallelism_config {
                properties.insert(
                    "parallelismConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-salesforcedestinationproperties.html
    pub struct SalesforceDestinationProperties_ {
        pub data_transfer_api: Option<crate::value::ExpString>,
        pub error_handling_config: Option<Box<ErrorHandlingConfig_>>,
        pub id_field_names: Option<Vec<crate::value::ExpString>>,
        pub object: crate::value::ExpString,
        pub write_operation_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_SalesforceDestinationProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.SalesforceDestinationProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_SalesforceDestinationProperties as SalesforceDestinationProperties;
    impl crate::value::ToValue for SalesforceDestinationProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.data_transfer_api {
                properties.insert(
                    "DataTransferApi".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.error_handling_config {
                properties.insert(
                    "ErrorHandlingConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.id_field_names {
                properties.insert(
                    "IdFieldNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Object".to_string(),
                crate::value::ToValue::to_value(&self.object),
            );
            if let Some(ref value) = self.write_operation_type {
                properties.insert(
                    "WriteOperationType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-salesforcesourceproperties.html
    pub struct SalesforceSourceProperties_ {
        pub data_transfer_api: Option<crate::value::ExpString>,
        pub enable_dynamic_field_update: Option<crate::value::ExpBool>,
        pub include_deleted_records: Option<crate::value::ExpBool>,
        pub object: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_SalesforceSourceProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.SalesforceSourceProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_SalesforceSourceProperties as SalesforceSourceProperties;
    impl crate::value::ToValue for SalesforceSourceProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.data_transfer_api {
                properties.insert(
                    "DataTransferApi".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enable_dynamic_field_update {
                properties.insert(
                    "EnableDynamicFieldUpdate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_deleted_records {
                properties.insert(
                    "IncludeDeletedRecords".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Object".to_string(),
                crate::value::ToValue::to_value(&self.object),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-scheduledtriggerproperties.html
    pub struct ScheduledTriggerProperties_ {
        pub data_pull_mode: Option<crate::value::ExpString>,
        pub first_execution_from: Option<f64>,
        pub flow_error_deactivation_threshold: Option<i64>,
        pub schedule_end_time: Option<f64>,
        pub schedule_expression: crate::value::ExpString,
        pub schedule_offset: Option<f64>,
        pub schedule_start_time: Option<f64>,
        pub time_zone: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_ScheduledTriggerProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.ScheduledTriggerProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_ScheduledTriggerProperties as ScheduledTriggerProperties;
    impl crate::value::ToValue for ScheduledTriggerProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.data_pull_mode {
                properties.insert(
                    "DataPullMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.first_execution_from {
                properties.insert(
                    "FirstExecutionFrom".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.flow_error_deactivation_threshold {
                properties.insert(
                    "FlowErrorDeactivationThreshold".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.schedule_end_time {
                properties.insert(
                    "ScheduleEndTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ScheduleExpression".to_string(),
                crate::value::ToValue::to_value(&self.schedule_expression),
            );
            if let Some(ref value) = self.schedule_offset {
                properties.insert(
                    "ScheduleOffset".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.schedule_start_time {
                properties.insert(
                    "ScheduleStartTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.time_zone {
                properties.insert(
                    "TimeZone".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-servicenowsourceproperties.html
    pub struct ServiceNowSourceProperties_ {
        pub object: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_ServiceNowSourceProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.ServiceNowSourceProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_ServiceNowSourceProperties as ServiceNowSourceProperties;
    impl crate::value::ToValue for ServiceNowSourceProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Object".to_string(),
                crate::value::ToValue::to_value(&self.object),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-singularsourceproperties.html
    pub struct SingularSourceProperties_ {
        pub object: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_SingularSourceProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.SingularSourceProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_SingularSourceProperties as SingularSourceProperties;
    impl crate::value::ToValue for SingularSourceProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Object".to_string(),
                crate::value::ToValue::to_value(&self.object),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-slacksourceproperties.html
    pub struct SlackSourceProperties_ {
        pub object: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_SlackSourceProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.SlackSourceProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_SlackSourceProperties as SlackSourceProperties;
    impl crate::value::ToValue for SlackSourceProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Object".to_string(),
                crate::value::ToValue::to_value(&self.object),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-snowflakedestinationproperties.html
    pub struct SnowflakeDestinationProperties_ {
        pub bucket_prefix: Option<crate::value::ExpString>,
        pub error_handling_config: Option<Box<ErrorHandlingConfig_>>,
        pub intermediate_bucket_name: crate::value::ExpString,
        pub object: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_SnowflakeDestinationProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.SnowflakeDestinationProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_SnowflakeDestinationProperties as SnowflakeDestinationProperties;
    impl crate::value::ToValue for SnowflakeDestinationProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bucket_prefix {
                properties.insert(
                    "BucketPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.error_handling_config {
                properties.insert(
                    "ErrorHandlingConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "IntermediateBucketName".to_string(),
                crate::value::ToValue::to_value(&self.intermediate_bucket_name),
            );
            properties.insert(
                "Object".to_string(),
                crate::value::ToValue::to_value(&self.object),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sourceconnectorproperties.html
    pub struct SourceConnectorProperties_ {
        pub amplitude: Option<Box<AmplitudeSourceProperties_>>,
        pub custom_connector: Option<Box<CustomConnectorSourceProperties_>>,
        pub datadog: Option<Box<DatadogSourceProperties_>>,
        pub dynatrace: Option<Box<DynatraceSourceProperties_>>,
        pub google_analytics: Option<Box<GoogleAnalyticsSourceProperties_>>,
        pub infor_nexus: Option<Box<InforNexusSourceProperties_>>,
        pub marketo: Option<Box<MarketoSourceProperties_>>,
        pub pardot: Option<Box<PardotSourceProperties_>>,
        pub s3: Option<Box<S3SourceProperties_>>,
        pub sapo_data: Option<Box<SAPODataSourceProperties_>>,
        pub salesforce: Option<Box<SalesforceSourceProperties_>>,
        pub service_now: Option<Box<ServiceNowSourceProperties_>>,
        pub singular: Option<Box<SingularSourceProperties_>>,
        pub slack: Option<Box<SlackSourceProperties_>>,
        pub trendmicro: Option<Box<TrendmicroSourceProperties_>>,
        pub veeva: Option<Box<VeevaSourceProperties_>>,
        pub zendesk: Option<Box<ZendeskSourceProperties_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_SourceConnectorProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.SourceConnectorProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_SourceConnectorProperties as SourceConnectorProperties;
    impl crate::value::ToValue for SourceConnectorProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.amplitude {
                properties.insert(
                    "Amplitude".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_connector {
                properties.insert(
                    "CustomConnector".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.datadog {
                properties.insert(
                    "Datadog".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dynatrace {
                properties.insert(
                    "Dynatrace".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.google_analytics {
                properties.insert(
                    "GoogleAnalytics".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.infor_nexus {
                properties.insert(
                    "InforNexus".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.marketo {
                properties.insert(
                    "Marketo".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pardot {
                properties.insert("Pardot".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.s3 {
                properties.insert("S3".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.sapo_data {
                properties.insert(
                    "SAPOData".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.salesforce {
                properties.insert(
                    "Salesforce".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.service_now {
                properties.insert(
                    "ServiceNow".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.singular {
                properties.insert(
                    "Singular".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.slack {
                properties.insert("Slack".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.trendmicro {
                properties.insert(
                    "Trendmicro".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.veeva {
                properties.insert("Veeva".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.zendesk {
                properties.insert(
                    "Zendesk".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sourceflowconfig.html
    pub struct SourceFlowConfig_ {
        pub api_version: Option<crate::value::ExpString>,
        pub connector_profile_name: Option<crate::value::ExpString>,
        pub connector_type: crate::value::ExpString,
        pub incremental_pull_config: Option<Box<IncrementalPullConfig_>>,
        pub source_connector_properties: Box<SourceConnectorProperties_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_SourceFlowConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.SourceFlowConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_SourceFlowConfig as SourceFlowConfig;
    impl crate::value::ToValue for SourceFlowConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.api_version {
                properties.insert(
                    "ApiVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.connector_profile_name {
                properties.insert(
                    "ConnectorProfileName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ConnectorType".to_string(),
                crate::value::ToValue::to_value(&self.connector_type),
            );
            if let Some(ref value) = self.incremental_pull_config {
                properties.insert(
                    "IncrementalPullConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SourceConnectorProperties".to_string(),
                crate::value::ToValue::to_value(&self.source_connector_properties),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-successresponsehandlingconfig.html
    pub struct SuccessResponseHandlingConfig_ {
        pub bucket_name: Option<crate::value::ExpString>,
        pub bucket_prefix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_SuccessResponseHandlingConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.SuccessResponseHandlingConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_SuccessResponseHandlingConfig as SuccessResponseHandlingConfig;
    impl crate::value::ToValue for SuccessResponseHandlingConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bucket_name {
                properties.insert(
                    "BucketName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bucket_prefix {
                properties.insert(
                    "BucketPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-task.html
    pub struct Task_ {
        pub connector_operator: Option<Box<ConnectorOperator_>>,
        pub destination_field: Option<crate::value::ExpString>,
        pub source_fields: Vec<crate::value::ExpString>,
        pub task_properties: Option<Vec<TaskPropertiesObject_>>,
        pub task_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_Task {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.Task"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_Task as Task;
    impl crate::value::ToValue for Task_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.connector_operator {
                properties.insert(
                    "ConnectorOperator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.destination_field {
                properties.insert(
                    "DestinationField".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SourceFields".to_string(),
                crate::value::ToValue::to_value(&self.source_fields),
            );
            if let Some(ref value) = self.task_properties {
                properties.insert(
                    "TaskProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TaskType".to_string(),
                crate::value::ToValue::to_value(&self.task_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-taskpropertiesobject.html
    pub struct TaskPropertiesObject_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_TaskPropertiesObject {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.TaskPropertiesObject"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_TaskPropertiesObject as TaskPropertiesObject;
    impl crate::value::ToValue for TaskPropertiesObject_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-trendmicrosourceproperties.html
    pub struct TrendmicroSourceProperties_ {
        pub object: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_TrendmicroSourceProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.TrendmicroSourceProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_TrendmicroSourceProperties as TrendmicroSourceProperties;
    impl crate::value::ToValue for TrendmicroSourceProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Object".to_string(),
                crate::value::ToValue::to_value(&self.object),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-triggerconfig.html
    pub struct TriggerConfig_ {
        pub trigger_properties: Option<Box<ScheduledTriggerProperties_>>,
        pub trigger_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_TriggerConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.TriggerConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_TriggerConfig as TriggerConfig;
    impl crate::value::ToValue for TriggerConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.trigger_properties {
                properties.insert(
                    "TriggerProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TriggerType".to_string(),
                crate::value::ToValue::to_value(&self.trigger_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-upsolverdestinationproperties.html
    pub struct UpsolverDestinationProperties_ {
        pub bucket_name: crate::value::ExpString,
        pub bucket_prefix: Option<crate::value::ExpString>,
        pub s3_output_format_config: Box<UpsolverS3OutputFormatConfig_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_UpsolverDestinationProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.UpsolverDestinationProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_UpsolverDestinationProperties as UpsolverDestinationProperties;
    impl crate::value::ToValue for UpsolverDestinationProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BucketName".to_string(),
                crate::value::ToValue::to_value(&self.bucket_name),
            );
            if let Some(ref value) = self.bucket_prefix {
                properties.insert(
                    "BucketPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "S3OutputFormatConfig".to_string(),
                crate::value::ToValue::to_value(&self.s3_output_format_config),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-upsolvers3outputformatconfig.html
    pub struct UpsolverS3OutputFormatConfig_ {
        pub aggregation_config: Option<Box<AggregationConfig_>>,
        pub file_type: Option<crate::value::ExpString>,
        pub prefix_config: Box<PrefixConfig_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_UpsolverS3OutputFormatConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.UpsolverS3OutputFormatConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_UpsolverS3OutputFormatConfig as UpsolverS3OutputFormatConfig;
    impl crate::value::ToValue for UpsolverS3OutputFormatConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.aggregation_config {
                properties.insert(
                    "AggregationConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.file_type {
                properties.insert(
                    "FileType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "PrefixConfig".to_string(),
                crate::value::ToValue::to_value(&self.prefix_config),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-veevasourceproperties.html
    pub struct VeevaSourceProperties_ {
        pub document_type: Option<crate::value::ExpString>,
        pub include_all_versions: Option<crate::value::ExpBool>,
        pub include_renditions: Option<crate::value::ExpBool>,
        pub include_source_files: Option<crate::value::ExpBool>,
        pub object: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_VeevaSourceProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.VeevaSourceProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_VeevaSourceProperties as VeevaSourceProperties;
    impl crate::value::ToValue for VeevaSourceProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.document_type {
                properties.insert(
                    "DocumentType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_all_versions {
                properties.insert(
                    "IncludeAllVersions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_renditions {
                properties.insert(
                    "IncludeRenditions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_source_files {
                properties.insert(
                    "IncludeSourceFiles".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Object".to_string(),
                crate::value::ToValue::to_value(&self.object),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-zendeskdestinationproperties.html
    pub struct ZendeskDestinationProperties_ {
        pub error_handling_config: Option<Box<ErrorHandlingConfig_>>,
        pub id_field_names: Option<Vec<crate::value::ExpString>>,
        pub object: crate::value::ExpString,
        pub write_operation_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_ZendeskDestinationProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.ZendeskDestinationProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_ZendeskDestinationProperties as ZendeskDestinationProperties;
    impl crate::value::ToValue for ZendeskDestinationProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.error_handling_config {
                properties.insert(
                    "ErrorHandlingConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.id_field_names {
                properties.insert(
                    "IdFieldNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Object".to_string(),
                crate::value::ToValue::to_value(&self.object),
            );
            if let Some(ref value) = self.write_operation_type {
                properties.insert(
                    "WriteOperationType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-zendesksourceproperties.html
    pub struct ZendeskSourceProperties_ {
        pub object: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appflow_Flow_ZendeskSourceProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppFlow::Flow.ZendeskSourceProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appflow_Flow_ZendeskSourceProperties as ZendeskSourceProperties;
    impl crate::value::ToValue for ZendeskSourceProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Object".to_string(),
                crate::value::ToValue::to_value(&self.object),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appflow-connector.html
pub struct Connector_ {
    pub connector_label: Option<crate::value::ExpString>,
    pub connector_provisioning_config: super::appflow::connector::ConnectorProvisioningConfig_,
    pub connector_provisioning_type: crate::value::ExpString,
    pub description: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appflow_Connector {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::AppFlow::Connector"
        $($field $value)*)
    };
}
pub use crate::__aws_appflow_Connector as Connector;
impl crate::template::ToResource for Connector_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppFlow"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Connector"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.connector_label {
            properties.insert(
                "ConnectorLabel".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ConnectorProvisioningConfig".to_string(),
            crate::value::ToValue::to_value(&self.connector_provisioning_config),
        );
        properties.insert(
            "ConnectorProvisioningType".to_string(),
            crate::value::ToValue::to_value(&self.connector_provisioning_type),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appflow-connectorprofile.html
pub struct ConnectorProfile_ {
    pub connection_mode: crate::value::ExpString,
    pub connector_label: Option<crate::value::ExpString>,
    pub connector_profile_config: Option<super::appflow::connectorprofile::ConnectorProfileConfig_>,
    pub connector_profile_name: crate::value::ExpString,
    pub connector_type: crate::value::ExpString,
    pub kms_arn: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appflow_ConnectorProfile {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::AppFlow::ConnectorProfile"
        $($field $value)*)
    };
}
pub use crate::__aws_appflow_ConnectorProfile as ConnectorProfile;
impl crate::template::ToResource for ConnectorProfile_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppFlow"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ConnectorProfile"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ConnectionMode".to_string(),
            crate::value::ToValue::to_value(&self.connection_mode),
        );
        if let Some(ref value) = self.connector_label {
            properties.insert(
                "ConnectorLabel".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.connector_profile_config {
            properties.insert(
                "ConnectorProfileConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ConnectorProfileName".to_string(),
            crate::value::ToValue::to_value(&self.connector_profile_name),
        );
        properties.insert(
            "ConnectorType".to_string(),
            crate::value::ToValue::to_value(&self.connector_type),
        );
        if let Some(ref value) = self.kms_arn {
            properties.insert("KMSArn".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appflow-flow.html
pub struct Flow_ {
    pub description: Option<crate::value::ExpString>,
    pub destination_flow_config_list: Vec<super::appflow::flow::DestinationFlowConfig_>,
    pub flow_name: crate::value::ExpString,
    pub flow_status: Option<crate::value::ExpString>,
    pub kms_arn: Option<crate::value::ExpString>,
    pub metadata_catalog_config: Option<super::appflow::flow::MetadataCatalogConfig_>,
    pub source_flow_config: super::appflow::flow::SourceFlowConfig_,
    pub tags: Option<Vec<crate::Tag_>>,
    pub tasks: Vec<super::appflow::flow::Task_>,
    pub trigger_config: super::appflow::flow::TriggerConfig_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appflow_Flow {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::AppFlow::Flow" $($field
        $value)*)
    };
}
pub use crate::__aws_appflow_Flow as Flow;
impl crate::template::ToResource for Flow_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppFlow"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Flow"),
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
            "DestinationFlowConfigList".to_string(),
            crate::value::ToValue::to_value(&self.destination_flow_config_list),
        );
        properties.insert(
            "FlowName".to_string(),
            crate::value::ToValue::to_value(&self.flow_name),
        );
        if let Some(ref value) = self.flow_status {
            properties.insert(
                "FlowStatus".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_arn {
            properties.insert("KMSArn".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.metadata_catalog_config {
            properties.insert(
                "MetadataCatalogConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SourceFlowConfig".to_string(),
            crate::value::ToValue::to_value(&self.source_flow_config),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Tasks".to_string(),
            crate::value::ToValue::to_value(&self.tasks),
        );
        properties.insert(
            "TriggerConfig".to_string(),
            crate::value::ToValue::to_value(&self.trigger_config),
        );
        properties
    }
}
