pub mod connection {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-connection-amazonqpropertiesinput.html>
    pub struct AmazonQPropertiesInput_ {
        pub auth_mode: Option<crate::value::ExpString>,
        pub is_enabled: Option<crate::value::ExpBool>,
        pub profile_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_Connection_AmazonQPropertiesInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::Connection.AmazonQPropertiesInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_Connection_AmazonQPropertiesInput as AmazonQPropertiesInput;
    impl crate::value::ToValue for AmazonQPropertiesInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auth_mode {
                properties.insert(
                    "AuthMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.is_enabled {
                properties.insert(
                    "IsEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.profile_arn {
                properties.insert(
                    "ProfileArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-connection-athenapropertiesinput.html>
    pub struct AthenaPropertiesInput_ {
        pub workgroup_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_Connection_AthenaPropertiesInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::Connection.AthenaPropertiesInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_Connection_AthenaPropertiesInput as AthenaPropertiesInput;
    impl crate::value::ToValue for AthenaPropertiesInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "WorkgroupName".to_string(),
                crate::value::ToValue::to_value(&self.workgroup_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-connection-authenticationconfigurationinput.html>
    pub struct AuthenticationConfigurationInput_ {
        pub authentication_type: Option<crate::value::ExpString>,
        pub basic_authentication_credentials: Option<Box<BasicAuthenticationCredentials_>>,
        pub custom_authentication_credentials:
            Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub kms_key_arn: Option<crate::value::ExpString>,
        pub o_auth2_properties: Option<Box<OAuth2Properties_>>,
        pub secret_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_Connection_AuthenticationConfigurationInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::Connection.AuthenticationConfigurationInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_Connection_AuthenticationConfigurationInput as AuthenticationConfigurationInput;
    impl crate::value::ToValue for AuthenticationConfigurationInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.authentication_type {
                properties.insert(
                    "AuthenticationType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.basic_authentication_credentials {
                properties.insert(
                    "BasicAuthenticationCredentials".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_authentication_credentials {
                properties.insert(
                    "CustomAuthenticationCredentials".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kms_key_arn {
                properties.insert(
                    "KmsKeyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.o_auth2_properties {
                properties.insert(
                    "OAuth2Properties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secret_arn {
                properties.insert(
                    "SecretArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-connection-authorizationcodeproperties.html>
    pub struct AuthorizationCodeProperties_ {
        pub authorization_code: Option<crate::value::ExpString>,
        pub redirect_uri: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_Connection_AuthorizationCodeProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::Connection.AuthorizationCodeProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_Connection_AuthorizationCodeProperties as AuthorizationCodeProperties;
    impl crate::value::ToValue for AuthorizationCodeProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.authorization_code {
                properties.insert(
                    "AuthorizationCode".to_string(),
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-connection-awslocation.html>
    pub struct AwsLocation_ {
        pub access_role: Option<crate::value::ExpString>,
        pub aws_account_id: Option<crate::value::ExpString>,
        pub aws_region: Option<crate::value::ExpString>,
        pub iam_connection_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_Connection_AwsLocation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::Connection.AwsLocation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_Connection_AwsLocation as AwsLocation;
    impl crate::value::ToValue for AwsLocation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.access_role {
                properties.insert(
                    "AccessRole".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.aws_account_id {
                properties.insert(
                    "AwsAccountId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.aws_region {
                properties.insert(
                    "AwsRegion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.iam_connection_id {
                properties.insert(
                    "IamConnectionId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-connection-basicauthenticationcredentials.html>
    pub struct BasicAuthenticationCredentials_ {
        pub password: Option<crate::value::ExpString>,
        pub user_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_Connection_BasicAuthenticationCredentials {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::Connection.BasicAuthenticationCredentials"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_Connection_BasicAuthenticationCredentials as BasicAuthenticationCredentials;
    impl crate::value::ToValue for BasicAuthenticationCredentials_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.password {
                properties.insert(
                    "Password".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_name {
                properties.insert(
                    "UserName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-connection-connectionpropertiesinput.html>
    pub struct ConnectionPropertiesInput_ {
        pub amazon_q_properties: Option<Box<AmazonQPropertiesInput_>>,
        pub athena_properties: Option<Box<AthenaPropertiesInput_>>,
        pub glue_properties: Option<Box<GluePropertiesInput_>>,
        pub hyper_pod_properties: Option<Box<HyperPodPropertiesInput_>>,
        pub iam_properties: Option<Box<IamPropertiesInput_>>,
        pub mlflow_properties: Option<Box<MlflowPropertiesInput_>>,
        pub redshift_properties: Option<Box<RedshiftPropertiesInput_>>,
        pub s3_properties: Option<Box<S3PropertiesInput_>>,
        pub spark_emr_properties: Option<Box<SparkEmrPropertiesInput_>>,
        pub spark_glue_properties: Option<Box<SparkGluePropertiesInput_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_Connection_ConnectionPropertiesInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::Connection.ConnectionPropertiesInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_Connection_ConnectionPropertiesInput as ConnectionPropertiesInput;
    impl crate::value::ToValue for ConnectionPropertiesInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.amazon_q_properties {
                properties.insert(
                    "AmazonQProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.athena_properties {
                properties.insert(
                    "AthenaProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.glue_properties {
                properties.insert(
                    "GlueProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hyper_pod_properties {
                properties.insert(
                    "HyperPodProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.iam_properties {
                properties.insert(
                    "IamProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mlflow_properties {
                properties.insert(
                    "MlflowProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.redshift_properties {
                properties.insert(
                    "RedshiftProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_properties {
                properties.insert(
                    "S3Properties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.spark_emr_properties {
                properties.insert(
                    "SparkEmrProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.spark_glue_properties {
                properties.insert(
                    "SparkGlueProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-connection-glueconnectioninput.html>
    pub struct GlueConnectionInput_ {
        pub athena_properties: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub authentication_configuration: Option<Box<AuthenticationConfigurationInput_>>,
        pub connection_properties:
            Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub connection_type: Option<crate::value::ExpString>,
        pub description: Option<crate::value::ExpString>,
        pub match_criteria: Option<crate::value::ExpString>,
        pub name: Option<crate::value::ExpString>,
        pub physical_connection_requirements: Option<Box<PhysicalConnectionRequirements_>>,
        pub python_properties: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub spark_properties: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub validate_credentials: Option<crate::value::ExpBool>,
        pub validate_for_compute_environments: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_Connection_GlueConnectionInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::Connection.GlueConnectionInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_Connection_GlueConnectionInput as GlueConnectionInput;
    impl crate::value::ToValue for GlueConnectionInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.athena_properties {
                properties.insert(
                    "AthenaProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.authentication_configuration {
                properties.insert(
                    "AuthenticationConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.connection_properties {
                properties.insert(
                    "ConnectionProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.connection_type {
                properties.insert(
                    "ConnectionType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.match_criteria {
                properties.insert(
                    "MatchCriteria".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.physical_connection_requirements {
                properties.insert(
                    "PhysicalConnectionRequirements".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.python_properties {
                properties.insert(
                    "PythonProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.spark_properties {
                properties.insert(
                    "SparkProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.validate_credentials {
                properties.insert(
                    "ValidateCredentials".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.validate_for_compute_environments {
                properties.insert(
                    "ValidateForComputeEnvironments".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-connection-glueoauth2credentials.html>
    pub struct GlueOAuth2Credentials_ {
        pub access_token: Option<crate::value::ExpString>,
        pub jwt_token: Option<crate::value::ExpString>,
        pub refresh_token: Option<crate::value::ExpString>,
        pub user_managed_client_application_client_secret: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_Connection_GlueOAuth2Credentials {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::Connection.GlueOAuth2Credentials"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_Connection_GlueOAuth2Credentials as GlueOAuth2Credentials;
    impl crate::value::ToValue for GlueOAuth2Credentials_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.access_token {
                properties.insert(
                    "AccessToken".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.jwt_token {
                properties.insert(
                    "JwtToken".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.refresh_token {
                properties.insert(
                    "RefreshToken".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_managed_client_application_client_secret {
                properties.insert(
                    "UserManagedClientApplicationClientSecret".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-connection-gluepropertiesinput.html>
    pub struct GluePropertiesInput_ {
        pub glue_connection_input: Option<Box<GlueConnectionInput_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_Connection_GluePropertiesInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::Connection.GluePropertiesInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_Connection_GluePropertiesInput as GluePropertiesInput;
    impl crate::value::ToValue for GluePropertiesInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.glue_connection_input {
                properties.insert(
                    "GlueConnectionInput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-connection-hyperpodpropertiesinput.html>
    pub struct HyperPodPropertiesInput_ {
        pub cluster_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_Connection_HyperPodPropertiesInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::Connection.HyperPodPropertiesInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_Connection_HyperPodPropertiesInput as HyperPodPropertiesInput;
    impl crate::value::ToValue for HyperPodPropertiesInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ClusterName".to_string(),
                crate::value::ToValue::to_value(&self.cluster_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-connection-iampropertiesinput.html>
    pub struct IamPropertiesInput_ {
        pub glue_lineage_sync_enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_Connection_IamPropertiesInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::Connection.IamPropertiesInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_Connection_IamPropertiesInput as IamPropertiesInput;
    impl crate::value::ToValue for IamPropertiesInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.glue_lineage_sync_enabled {
                properties.insert(
                    "GlueLineageSyncEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-connection-lineagesyncschedule.html>
    pub struct LineageSyncSchedule_ {
        pub schedule: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_Connection_LineageSyncSchedule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::Connection.LineageSyncSchedule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_Connection_LineageSyncSchedule as LineageSyncSchedule;
    impl crate::value::ToValue for LineageSyncSchedule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.schedule {
                properties.insert(
                    "Schedule".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-connection-mlflowpropertiesinput.html>
    pub struct MlflowPropertiesInput_ {
        pub tracking_server_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_Connection_MlflowPropertiesInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::Connection.MlflowPropertiesInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_Connection_MlflowPropertiesInput as MlflowPropertiesInput;
    impl crate::value::ToValue for MlflowPropertiesInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.tracking_server_arn {
                properties.insert(
                    "TrackingServerArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-connection-oauth2clientapplication.html>
    pub struct OAuth2ClientApplication_ {
        pub aws_managed_client_application_reference: Option<crate::value::ExpString>,
        pub user_managed_client_application_client_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_Connection_OAuth2ClientApplication {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::Connection.OAuth2ClientApplication"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_Connection_OAuth2ClientApplication as OAuth2ClientApplication;
    impl crate::value::ToValue for OAuth2ClientApplication_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.aws_managed_client_application_reference {
                properties.insert(
                    "AWSManagedClientApplicationReference".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_managed_client_application_client_id {
                properties.insert(
                    "UserManagedClientApplicationClientId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-connection-oauth2properties.html>
    pub struct OAuth2Properties_ {
        pub authorization_code_properties: Option<Box<AuthorizationCodeProperties_>>,
        pub o_auth2_client_application: Option<Box<OAuth2ClientApplication_>>,
        pub o_auth2_credentials: Option<Box<GlueOAuth2Credentials_>>,
        pub o_auth2_grant_type: Option<crate::value::ExpString>,
        pub token_url: Option<crate::value::ExpString>,
        pub token_url_parameters_map:
            Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_Connection_OAuth2Properties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::Connection.OAuth2Properties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_Connection_OAuth2Properties as OAuth2Properties;
    impl crate::value::ToValue for OAuth2Properties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.authorization_code_properties {
                properties.insert(
                    "AuthorizationCodeProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.o_auth2_client_application {
                properties.insert(
                    "OAuth2ClientApplication".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.o_auth2_credentials {
                properties.insert(
                    "OAuth2Credentials".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
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
            if let Some(ref value) = self.token_url_parameters_map {
                properties.insert(
                    "TokenUrlParametersMap".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-connection-physicalconnectionrequirements.html>
    pub struct PhysicalConnectionRequirements_ {
        pub availability_zone: Option<crate::value::ExpString>,
        pub security_group_id_list: Option<Vec<crate::value::ExpString>>,
        pub subnet_id: Option<crate::value::ExpString>,
        pub subnet_id_list: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_Connection_PhysicalConnectionRequirements {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::Connection.PhysicalConnectionRequirements"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_Connection_PhysicalConnectionRequirements as PhysicalConnectionRequirements;
    impl crate::value::ToValue for PhysicalConnectionRequirements_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.availability_zone {
                properties.insert(
                    "AvailabilityZone".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.security_group_id_list {
                properties.insert(
                    "SecurityGroupIdList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.subnet_id {
                properties.insert(
                    "SubnetId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.subnet_id_list {
                properties.insert(
                    "SubnetIdList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-connection-redshiftcredentials.html>
    pub struct RedshiftCredentials_ {
        pub secret_arn: Option<crate::value::ExpString>,
        pub username_password: Option<Box<UsernamePassword_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_Connection_RedshiftCredentials {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::Connection.RedshiftCredentials"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_Connection_RedshiftCredentials as RedshiftCredentials;
    impl crate::value::ToValue for RedshiftCredentials_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.secret_arn {
                properties.insert(
                    "SecretArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.username_password {
                properties.insert(
                    "UsernamePassword".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-connection-redshiftlineagesyncconfigurationinput.html>
    pub struct RedshiftLineageSyncConfigurationInput_ {
        pub enabled: Option<crate::value::ExpBool>,
        pub schedule: Option<Box<LineageSyncSchedule_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_Connection_RedshiftLineageSyncConfigurationInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::Connection.RedshiftLineageSyncConfigurationInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_Connection_RedshiftLineageSyncConfigurationInput as RedshiftLineageSyncConfigurationInput;
    impl crate::value::ToValue for RedshiftLineageSyncConfigurationInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.schedule {
                properties.insert(
                    "Schedule".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-connection-redshiftpropertiesinput.html>
    pub struct RedshiftPropertiesInput_ {
        pub credentials: Option<Box<RedshiftCredentials_>>,
        pub database_name: Option<crate::value::ExpString>,
        pub host: Option<crate::value::ExpString>,
        pub lineage_sync: Option<Box<RedshiftLineageSyncConfigurationInput_>>,
        pub port: Option<f64>,
        pub storage: Option<Box<RedshiftStorageProperties_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_Connection_RedshiftPropertiesInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::Connection.RedshiftPropertiesInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_Connection_RedshiftPropertiesInput as RedshiftPropertiesInput;
    impl crate::value::ToValue for RedshiftPropertiesInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.credentials {
                properties.insert(
                    "Credentials".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.database_name {
                properties.insert(
                    "DatabaseName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.host {
                properties.insert("Host".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.lineage_sync {
                properties.insert(
                    "LineageSync".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.port {
                properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.storage {
                properties.insert(
                    "Storage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-connection-redshiftstorageproperties.html>
    pub struct RedshiftStorageProperties_ {
        pub cluster_name: Option<crate::value::ExpString>,
        pub workgroup_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_Connection_RedshiftStorageProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::Connection.RedshiftStorageProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_Connection_RedshiftStorageProperties as RedshiftStorageProperties;
    impl crate::value::ToValue for RedshiftStorageProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cluster_name {
                properties.insert(
                    "ClusterName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.workgroup_name {
                properties.insert(
                    "WorkgroupName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-connection-s3propertiesinput.html>
    pub struct S3PropertiesInput_ {
        pub s3_access_grant_location_id: Option<crate::value::ExpString>,
        pub s3_uri: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_Connection_S3PropertiesInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::Connection.S3PropertiesInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_Connection_S3PropertiesInput as S3PropertiesInput;
    impl crate::value::ToValue for S3PropertiesInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.s3_access_grant_location_id {
                properties.insert(
                    "S3AccessGrantLocationId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "S3Uri".to_string(),
                crate::value::ToValue::to_value(&self.s3_uri),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-connection-sparkemrpropertiesinput.html>
    pub struct SparkEmrPropertiesInput_ {
        pub compute_arn: Option<crate::value::ExpString>,
        pub instance_profile_arn: Option<crate::value::ExpString>,
        pub java_virtual_env: Option<crate::value::ExpString>,
        pub log_uri: Option<crate::value::ExpString>,
        pub managed_endpoint_arn: Option<crate::value::ExpString>,
        pub python_virtual_env: Option<crate::value::ExpString>,
        pub runtime_role: Option<crate::value::ExpString>,
        pub trusted_certificates_s3_uri: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_Connection_SparkEmrPropertiesInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::Connection.SparkEmrPropertiesInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_Connection_SparkEmrPropertiesInput as SparkEmrPropertiesInput;
    impl crate::value::ToValue for SparkEmrPropertiesInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.compute_arn {
                properties.insert(
                    "ComputeArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.instance_profile_arn {
                properties.insert(
                    "InstanceProfileArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.java_virtual_env {
                properties.insert(
                    "JavaVirtualEnv".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.log_uri {
                properties.insert("LogUri".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.managed_endpoint_arn {
                properties.insert(
                    "ManagedEndpointArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.python_virtual_env {
                properties.insert(
                    "PythonVirtualEnv".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.runtime_role {
                properties.insert(
                    "RuntimeRole".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.trusted_certificates_s3_uri {
                properties.insert(
                    "TrustedCertificatesS3Uri".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-connection-sparkglueargs.html>
    pub struct SparkGlueArgs_ {
        pub connection: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_Connection_SparkGlueArgs {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::Connection.SparkGlueArgs"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_Connection_SparkGlueArgs as SparkGlueArgs;
    impl crate::value::ToValue for SparkGlueArgs_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.connection {
                properties.insert(
                    "Connection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-connection-sparkgluepropertiesinput.html>
    pub struct SparkGluePropertiesInput_ {
        pub additional_args: Option<Box<SparkGlueArgs_>>,
        pub glue_connection_name: Option<crate::value::ExpString>,
        pub glue_version: Option<crate::value::ExpString>,
        pub idle_timeout: Option<f64>,
        pub java_virtual_env: Option<crate::value::ExpString>,
        pub number_of_workers: Option<f64>,
        pub python_virtual_env: Option<crate::value::ExpString>,
        pub worker_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_Connection_SparkGluePropertiesInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::Connection.SparkGluePropertiesInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_Connection_SparkGluePropertiesInput as SparkGluePropertiesInput;
    impl crate::value::ToValue for SparkGluePropertiesInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.additional_args {
                properties.insert(
                    "AdditionalArgs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.glue_connection_name {
                properties.insert(
                    "GlueConnectionName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.glue_version {
                properties.insert(
                    "GlueVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.idle_timeout {
                properties.insert(
                    "IdleTimeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.java_virtual_env {
                properties.insert(
                    "JavaVirtualEnv".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.number_of_workers {
                properties.insert(
                    "NumberOfWorkers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.python_virtual_env {
                properties.insert(
                    "PythonVirtualEnv".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.worker_type {
                properties.insert(
                    "WorkerType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-connection-usernamepassword.html>
    pub struct UsernamePassword_ {
        pub password: crate::value::ExpString,
        pub username: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_Connection_UsernamePassword {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::Connection.UsernamePassword"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_Connection_UsernamePassword as UsernamePassword;
    impl crate::value::ToValue for UsernamePassword_ {
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
}
pub mod datasource {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-datasourceconfigurationinput.html>
    pub struct DataSourceConfigurationInput_ {
        pub glue_run_configuration: Option<Box<GlueRunConfigurationInput_>>,
        pub redshift_run_configuration: Option<Box<RedshiftRunConfigurationInput_>>,
        pub sage_maker_run_configuration: Option<Box<SageMakerRunConfigurationInput_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_DataSource_DataSourceConfigurationInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::DataSource.DataSourceConfigurationInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_DataSource_DataSourceConfigurationInput as DataSourceConfigurationInput;
    impl crate::value::ToValue for DataSourceConfigurationInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.glue_run_configuration {
                properties.insert(
                    "GlueRunConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.redshift_run_configuration {
                properties.insert(
                    "RedshiftRunConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sage_maker_run_configuration {
                properties.insert(
                    "SageMakerRunConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-filterexpression.html>
    pub struct FilterExpression_ {
        pub expression: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_DataSource_FilterExpression {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::DataSource.FilterExpression"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_DataSource_FilterExpression as FilterExpression;
    impl crate::value::ToValue for FilterExpression_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Expression".to_string(),
                crate::value::ToValue::to_value(&self.expression),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-forminput.html>
    pub struct FormInput_ {
        pub content: Option<crate::value::ExpString>,
        pub form_name: crate::value::ExpString,
        pub type_identifier: Option<crate::value::ExpString>,
        pub type_revision: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_DataSource_FormInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::DataSource.FormInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_DataSource_FormInput as FormInput;
    impl crate::value::ToValue for FormInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.content {
                properties.insert(
                    "Content".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "FormName".to_string(),
                crate::value::ToValue::to_value(&self.form_name),
            );
            if let Some(ref value) = self.type_identifier {
                properties.insert(
                    "TypeIdentifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.type_revision {
                properties.insert(
                    "TypeRevision".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-gluerunconfigurationinput.html>
    pub struct GlueRunConfigurationInput_ {
        pub auto_import_data_quality_result: Option<crate::value::ExpBool>,
        pub catalog_name: Option<crate::value::ExpString>,
        pub data_access_role: Option<crate::value::ExpString>,
        pub relational_filter_configurations: Vec<RelationalFilterConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_DataSource_GlueRunConfigurationInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::DataSource.GlueRunConfigurationInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_DataSource_GlueRunConfigurationInput as GlueRunConfigurationInput;
    impl crate::value::ToValue for GlueRunConfigurationInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auto_import_data_quality_result {
                properties.insert(
                    "AutoImportDataQualityResult".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.catalog_name {
                properties.insert(
                    "CatalogName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.data_access_role {
                properties.insert(
                    "DataAccessRole".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RelationalFilterConfigurations".to_string(),
                crate::value::ToValue::to_value(&self.relational_filter_configurations),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-recommendationconfiguration.html>
    pub struct RecommendationConfiguration_ {
        pub enable_business_name_generation: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_DataSource_RecommendationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::DataSource.RecommendationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_DataSource_RecommendationConfiguration as RecommendationConfiguration;
    impl crate::value::ToValue for RecommendationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enable_business_name_generation {
                properties.insert(
                    "EnableBusinessNameGeneration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-redshiftclusterstorage.html>
    pub struct RedshiftClusterStorage_ {
        pub cluster_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_DataSource_RedshiftClusterStorage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::DataSource.RedshiftClusterStorage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_DataSource_RedshiftClusterStorage as RedshiftClusterStorage;
    impl crate::value::ToValue for RedshiftClusterStorage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ClusterName".to_string(),
                crate::value::ToValue::to_value(&self.cluster_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-redshiftcredentialconfiguration.html>
    pub struct RedshiftCredentialConfiguration_ {
        pub secret_manager_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_DataSource_RedshiftCredentialConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::DataSource.RedshiftCredentialConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_DataSource_RedshiftCredentialConfiguration as RedshiftCredentialConfiguration;
    impl crate::value::ToValue for RedshiftCredentialConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SecretManagerArn".to_string(),
                crate::value::ToValue::to_value(&self.secret_manager_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-redshiftrunconfigurationinput.html>
    pub struct RedshiftRunConfigurationInput_ {
        pub data_access_role: Option<crate::value::ExpString>,
        pub redshift_credential_configuration: Option<Box<RedshiftCredentialConfiguration_>>,
        pub redshift_storage: Option<Box<RedshiftStorage_>>,
        pub relational_filter_configurations: Vec<RelationalFilterConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_DataSource_RedshiftRunConfigurationInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::DataSource.RedshiftRunConfigurationInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_DataSource_RedshiftRunConfigurationInput as RedshiftRunConfigurationInput;
    impl crate::value::ToValue for RedshiftRunConfigurationInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.data_access_role {
                properties.insert(
                    "DataAccessRole".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.redshift_credential_configuration {
                properties.insert(
                    "RedshiftCredentialConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.redshift_storage {
                properties.insert(
                    "RedshiftStorage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RelationalFilterConfigurations".to_string(),
                crate::value::ToValue::to_value(&self.relational_filter_configurations),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-redshiftserverlessstorage.html>
    pub struct RedshiftServerlessStorage_ {
        pub workgroup_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_DataSource_RedshiftServerlessStorage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::DataSource.RedshiftServerlessStorage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_DataSource_RedshiftServerlessStorage as RedshiftServerlessStorage;
    impl crate::value::ToValue for RedshiftServerlessStorage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "WorkgroupName".to_string(),
                crate::value::ToValue::to_value(&self.workgroup_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-redshiftstorage.html>
    pub struct RedshiftStorage_ {
        pub redshift_cluster_source: Option<Box<RedshiftClusterStorage_>>,
        pub redshift_serverless_source: Option<Box<RedshiftServerlessStorage_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_DataSource_RedshiftStorage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::DataSource.RedshiftStorage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_DataSource_RedshiftStorage as RedshiftStorage;
    impl crate::value::ToValue for RedshiftStorage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.redshift_cluster_source {
                properties.insert(
                    "RedshiftClusterSource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.redshift_serverless_source {
                properties.insert(
                    "RedshiftServerlessSource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-relationalfilterconfiguration.html>
    pub struct RelationalFilterConfiguration_ {
        pub database_name: crate::value::ExpString,
        pub filter_expressions: Option<Vec<FilterExpression_>>,
        pub schema_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_DataSource_RelationalFilterConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::DataSource.RelationalFilterConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_DataSource_RelationalFilterConfiguration as RelationalFilterConfiguration;
    impl crate::value::ToValue for RelationalFilterConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DatabaseName".to_string(),
                crate::value::ToValue::to_value(&self.database_name),
            );
            if let Some(ref value) = self.filter_expressions {
                properties.insert(
                    "FilterExpressions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.schema_name {
                properties.insert(
                    "SchemaName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-sagemakerrunconfigurationinput.html>
    pub struct SageMakerRunConfigurationInput_ {
        pub tracking_assets: serde_json::Value,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_DataSource_SageMakerRunConfigurationInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::DataSource.SageMakerRunConfigurationInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_DataSource_SageMakerRunConfigurationInput as SageMakerRunConfigurationInput;
    impl crate::value::ToValue for SageMakerRunConfigurationInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TrackingAssets".to_string(),
                crate::value::ToValue::to_value(&self.tracking_assets),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-datasource-scheduleconfiguration.html>
    pub struct ScheduleConfiguration_ {
        pub schedule: Option<crate::value::ExpString>,
        pub timezone: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_DataSource_ScheduleConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::DataSource.ScheduleConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_DataSource_ScheduleConfiguration as ScheduleConfiguration;
    impl crate::value::ToValue for ScheduleConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.schedule {
                properties.insert(
                    "Schedule".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timezone {
                properties.insert(
                    "Timezone".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod domain {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-domain-singlesignon.html>
    pub struct SingleSignOn_ {
        pub idc_instance_arn: Option<crate::value::ExpString>,
        pub r#type: Option<crate::value::ExpString>,
        pub user_assignment: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_Domain_SingleSignOn {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::Domain.SingleSignOn"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_Domain_SingleSignOn as SingleSignOn;
    impl crate::value::ToValue for SingleSignOn_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.idc_instance_arn {
                properties.insert(
                    "IdcInstanceArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.user_assignment {
                properties.insert(
                    "UserAssignment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod environment {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-environment-environmentparameter.html>
    pub struct EnvironmentParameter_ {
        pub name: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_Environment_EnvironmentParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::Environment.EnvironmentParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_Environment_EnvironmentParameter as EnvironmentParameter;
    impl crate::value::ToValue for EnvironmentParameter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod environmentactions {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-environmentactions-awsconsolelinkparameters.html>
    pub struct AwsConsoleLinkParameters_ {
        pub uri: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_EnvironmentActions_AwsConsoleLinkParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::EnvironmentActions.AwsConsoleLinkParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_EnvironmentActions_AwsConsoleLinkParameters as AwsConsoleLinkParameters;
    impl crate::value::ToValue for AwsConsoleLinkParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.uri {
                properties.insert("Uri".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod environmentblueprintconfiguration {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-environmentblueprintconfiguration-lakeformationconfiguration.html>
    pub struct LakeFormationConfiguration_ {
        pub location_registration_exclude_s3_locations: Option<Vec<crate::value::ExpString>>,
        pub location_registration_role: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_EnvironmentBlueprintConfiguration_LakeFormationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::EnvironmentBlueprintConfiguration.LakeFormationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_EnvironmentBlueprintConfiguration_LakeFormationConfiguration as LakeFormationConfiguration;
    impl crate::value::ToValue for LakeFormationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.location_registration_exclude_s3_locations {
                properties.insert(
                    "LocationRegistrationExcludeS3Locations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.location_registration_role {
                properties.insert(
                    "LocationRegistrationRole".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-environmentblueprintconfiguration-provisioningconfiguration.html>
    pub struct ProvisioningConfiguration_ {
        pub lake_formation_configuration: Box<LakeFormationConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_EnvironmentBlueprintConfiguration_ProvisioningConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::EnvironmentBlueprintConfiguration.ProvisioningConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_EnvironmentBlueprintConfiguration_ProvisioningConfiguration as ProvisioningConfiguration;
    impl crate::value::ToValue for ProvisioningConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LakeFormationConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.lake_formation_configuration),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-environmentblueprintconfiguration-regionalparameter.html>
    pub struct RegionalParameter_ {
        pub parameters: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub region: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_EnvironmentBlueprintConfiguration_RegionalParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::EnvironmentBlueprintConfiguration.RegionalParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_EnvironmentBlueprintConfiguration_RegionalParameter as RegionalParameter;
    impl crate::value::ToValue for RegionalParameter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.parameters {
                properties.insert(
                    "Parameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.region {
                properties.insert("Region".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod environmentprofile {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-environmentprofile-environmentparameter.html>
    pub struct EnvironmentParameter_ {
        pub name: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_EnvironmentProfile_EnvironmentParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::EnvironmentProfile.EnvironmentParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_EnvironmentProfile_EnvironmentParameter as EnvironmentParameter;
    impl crate::value::ToValue for EnvironmentParameter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod formtype {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-formtype-model.html>
    pub struct Model_ {
        pub smithy: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_FormType_Model {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::FormType.Model"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_FormType_Model as Model;
    impl crate::value::ToValue for Model_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.smithy {
                properties.insert("Smithy".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod owner {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-owner-ownergroupproperties.html>
    pub struct OwnerGroupProperties_ {
        pub group_identifier: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_Owner_OwnerGroupProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::Owner.OwnerGroupProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_Owner_OwnerGroupProperties as OwnerGroupProperties;
    impl crate::value::ToValue for OwnerGroupProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.group_identifier {
                properties.insert(
                    "GroupIdentifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-owner-ownerproperties.html>
    pub struct OwnerProperties_ {
        pub group: Option<Box<OwnerGroupProperties_>>,
        pub user: Option<Box<OwnerUserProperties_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_Owner_OwnerProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::Owner.OwnerProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_Owner_OwnerProperties as OwnerProperties;
    impl crate::value::ToValue for OwnerProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.group {
                properties.insert("Group".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.user {
                properties.insert("User".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-owner-owneruserproperties.html>
    pub struct OwnerUserProperties_ {
        pub user_identifier: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_Owner_OwnerUserProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::Owner.OwnerUserProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_Owner_OwnerUserProperties as OwnerUserProperties;
    impl crate::value::ToValue for OwnerUserProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.user_identifier {
                properties.insert(
                    "UserIdentifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod policygrant {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-policygrant-addtoprojectmemberpoolpolicygrantdetail.html>
    pub struct AddToProjectMemberPoolPolicyGrantDetail_ {
        pub include_child_domain_units: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_PolicyGrant_AddToProjectMemberPoolPolicyGrantDetail {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::PolicyGrant.AddToProjectMemberPoolPolicyGrantDetail"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_PolicyGrant_AddToProjectMemberPoolPolicyGrantDetail as AddToProjectMemberPoolPolicyGrantDetail;
    impl crate::value::ToValue for AddToProjectMemberPoolPolicyGrantDetail_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.include_child_domain_units {
                properties.insert(
                    "IncludeChildDomainUnits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-policygrant-createassettypepolicygrantdetail.html>
    pub struct CreateAssetTypePolicyGrantDetail_ {
        pub include_child_domain_units: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_PolicyGrant_CreateAssetTypePolicyGrantDetail {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::PolicyGrant.CreateAssetTypePolicyGrantDetail"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_PolicyGrant_CreateAssetTypePolicyGrantDetail as CreateAssetTypePolicyGrantDetail;
    impl crate::value::ToValue for CreateAssetTypePolicyGrantDetail_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.include_child_domain_units {
                properties.insert(
                    "IncludeChildDomainUnits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-policygrant-createdomainunitpolicygrantdetail.html>
    pub struct CreateDomainUnitPolicyGrantDetail_ {
        pub include_child_domain_units: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_PolicyGrant_CreateDomainUnitPolicyGrantDetail {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::PolicyGrant.CreateDomainUnitPolicyGrantDetail"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_PolicyGrant_CreateDomainUnitPolicyGrantDetail as CreateDomainUnitPolicyGrantDetail;
    impl crate::value::ToValue for CreateDomainUnitPolicyGrantDetail_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.include_child_domain_units {
                properties.insert(
                    "IncludeChildDomainUnits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-policygrant-createenvironmentprofilepolicygrantdetail.html>
    pub struct CreateEnvironmentProfilePolicyGrantDetail_ {
        pub domain_unit_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_PolicyGrant_CreateEnvironmentProfilePolicyGrantDetail {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::PolicyGrant.CreateEnvironmentProfilePolicyGrantDetail"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_PolicyGrant_CreateEnvironmentProfilePolicyGrantDetail as CreateEnvironmentProfilePolicyGrantDetail;
    impl crate::value::ToValue for CreateEnvironmentProfilePolicyGrantDetail_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.domain_unit_id {
                properties.insert(
                    "DomainUnitId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-policygrant-createformtypepolicygrantdetail.html>
    pub struct CreateFormTypePolicyGrantDetail_ {
        pub include_child_domain_units: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_PolicyGrant_CreateFormTypePolicyGrantDetail {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::PolicyGrant.CreateFormTypePolicyGrantDetail"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_PolicyGrant_CreateFormTypePolicyGrantDetail as CreateFormTypePolicyGrantDetail;
    impl crate::value::ToValue for CreateFormTypePolicyGrantDetail_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.include_child_domain_units {
                properties.insert(
                    "IncludeChildDomainUnits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-policygrant-createglossarypolicygrantdetail.html>
    pub struct CreateGlossaryPolicyGrantDetail_ {
        pub include_child_domain_units: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_PolicyGrant_CreateGlossaryPolicyGrantDetail {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::PolicyGrant.CreateGlossaryPolicyGrantDetail"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_PolicyGrant_CreateGlossaryPolicyGrantDetail as CreateGlossaryPolicyGrantDetail;
    impl crate::value::ToValue for CreateGlossaryPolicyGrantDetail_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.include_child_domain_units {
                properties.insert(
                    "IncludeChildDomainUnits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-policygrant-createprojectfromprojectprofilepolicygrantdetail.html>
    pub struct CreateProjectFromProjectProfilePolicyGrantDetail_ {
        pub include_child_domain_units: Option<crate::value::ExpBool>,
        pub project_profiles: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_PolicyGrant_CreateProjectFromProjectProfilePolicyGrantDetail {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::PolicyGrant.CreateProjectFromProjectProfilePolicyGrantDetail"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_PolicyGrant_CreateProjectFromProjectProfilePolicyGrantDetail as CreateProjectFromProjectProfilePolicyGrantDetail;
    impl crate::value::ToValue for CreateProjectFromProjectProfilePolicyGrantDetail_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.include_child_domain_units {
                properties.insert(
                    "IncludeChildDomainUnits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.project_profiles {
                properties.insert(
                    "ProjectProfiles".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-policygrant-createprojectpolicygrantdetail.html>
    pub struct CreateProjectPolicyGrantDetail_ {
        pub include_child_domain_units: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_PolicyGrant_CreateProjectPolicyGrantDetail {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::PolicyGrant.CreateProjectPolicyGrantDetail"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_PolicyGrant_CreateProjectPolicyGrantDetail as CreateProjectPolicyGrantDetail;
    impl crate::value::ToValue for CreateProjectPolicyGrantDetail_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.include_child_domain_units {
                properties.insert(
                    "IncludeChildDomainUnits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-policygrant-domainunitfilterforproject.html>
    pub struct DomainUnitFilterForProject_ {
        pub domain_unit: crate::value::ExpString,
        pub include_child_domain_units: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_PolicyGrant_DomainUnitFilterForProject {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::PolicyGrant.DomainUnitFilterForProject"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_PolicyGrant_DomainUnitFilterForProject as DomainUnitFilterForProject;
    impl crate::value::ToValue for DomainUnitFilterForProject_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DomainUnit".to_string(),
                crate::value::ToValue::to_value(&self.domain_unit),
            );
            if let Some(ref value) = self.include_child_domain_units {
                properties.insert(
                    "IncludeChildDomainUnits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-policygrant-domainunitgrantfilter.html>
    pub struct DomainUnitGrantFilter_ {
        pub all_domain_units_grant_filter: serde_json::Value,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_PolicyGrant_DomainUnitGrantFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::PolicyGrant.DomainUnitGrantFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_PolicyGrant_DomainUnitGrantFilter as DomainUnitGrantFilter;
    impl crate::value::ToValue for DomainUnitGrantFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AllDomainUnitsGrantFilter".to_string(),
                crate::value::ToValue::to_value(&self.all_domain_units_grant_filter),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-policygrant-domainunitpolicygrantprincipal.html>
    pub struct DomainUnitPolicyGrantPrincipal_ {
        pub domain_unit_designation: Option<crate::value::ExpString>,
        pub domain_unit_grant_filter: Option<Box<DomainUnitGrantFilter_>>,
        pub domain_unit_identifier: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_PolicyGrant_DomainUnitPolicyGrantPrincipal {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::PolicyGrant.DomainUnitPolicyGrantPrincipal"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_PolicyGrant_DomainUnitPolicyGrantPrincipal as DomainUnitPolicyGrantPrincipal;
    impl crate::value::ToValue for DomainUnitPolicyGrantPrincipal_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.domain_unit_designation {
                properties.insert(
                    "DomainUnitDesignation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.domain_unit_grant_filter {
                properties.insert(
                    "DomainUnitGrantFilter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.domain_unit_identifier {
                properties.insert(
                    "DomainUnitIdentifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-policygrant-grouppolicygrantprincipal.html>
    pub struct GroupPolicyGrantPrincipal_ {
        pub group_identifier: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_PolicyGrant_GroupPolicyGrantPrincipal {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::PolicyGrant.GroupPolicyGrantPrincipal"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_PolicyGrant_GroupPolicyGrantPrincipal as GroupPolicyGrantPrincipal;
    impl crate::value::ToValue for GroupPolicyGrantPrincipal_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "GroupIdentifier".to_string(),
                crate::value::ToValue::to_value(&self.group_identifier),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-policygrant-overridedomainunitownerspolicygrantdetail.html>
    pub struct OverrideDomainUnitOwnersPolicyGrantDetail_ {
        pub include_child_domain_units: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_PolicyGrant_OverrideDomainUnitOwnersPolicyGrantDetail {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::PolicyGrant.OverrideDomainUnitOwnersPolicyGrantDetail"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_PolicyGrant_OverrideDomainUnitOwnersPolicyGrantDetail as OverrideDomainUnitOwnersPolicyGrantDetail;
    impl crate::value::ToValue for OverrideDomainUnitOwnersPolicyGrantDetail_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.include_child_domain_units {
                properties.insert(
                    "IncludeChildDomainUnits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-policygrant-overrideprojectownerspolicygrantdetail.html>
    pub struct OverrideProjectOwnersPolicyGrantDetail_ {
        pub include_child_domain_units: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_PolicyGrant_OverrideProjectOwnersPolicyGrantDetail {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::PolicyGrant.OverrideProjectOwnersPolicyGrantDetail"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_PolicyGrant_OverrideProjectOwnersPolicyGrantDetail as OverrideProjectOwnersPolicyGrantDetail;
    impl crate::value::ToValue for OverrideProjectOwnersPolicyGrantDetail_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.include_child_domain_units {
                properties.insert(
                    "IncludeChildDomainUnits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-policygrant-policygrantdetail.html>
    pub struct PolicyGrantDetail_ {
        pub add_to_project_member_pool: Option<Box<AddToProjectMemberPoolPolicyGrantDetail_>>,
        pub create_asset_type: Option<Box<CreateAssetTypePolicyGrantDetail_>>,
        pub create_domain_unit: Option<Box<CreateDomainUnitPolicyGrantDetail_>>,
        pub create_environment: Option<serde_json::Value>,
        pub create_environment_from_blueprint: Option<serde_json::Value>,
        pub create_environment_profile: Option<Box<CreateEnvironmentProfilePolicyGrantDetail_>>,
        pub create_form_type: Option<Box<CreateFormTypePolicyGrantDetail_>>,
        pub create_glossary: Option<Box<CreateGlossaryPolicyGrantDetail_>>,
        pub create_project: Option<Box<CreateProjectPolicyGrantDetail_>>,
        pub create_project_from_project_profile:
            Option<Box<CreateProjectFromProjectProfilePolicyGrantDetail_>>,
        pub delegate_create_environment_profile: Option<serde_json::Value>,
        pub override_domain_unit_owners: Option<Box<OverrideDomainUnitOwnersPolicyGrantDetail_>>,
        pub override_project_owners: Option<Box<OverrideProjectOwnersPolicyGrantDetail_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_PolicyGrant_PolicyGrantDetail {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::PolicyGrant.PolicyGrantDetail"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_PolicyGrant_PolicyGrantDetail as PolicyGrantDetail;
    impl crate::value::ToValue for PolicyGrantDetail_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.add_to_project_member_pool {
                properties.insert(
                    "AddToProjectMemberPool".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.create_asset_type {
                properties.insert(
                    "CreateAssetType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.create_domain_unit {
                properties.insert(
                    "CreateDomainUnit".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.create_environment {
                properties.insert(
                    "CreateEnvironment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.create_environment_from_blueprint {
                properties.insert(
                    "CreateEnvironmentFromBlueprint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.create_environment_profile {
                properties.insert(
                    "CreateEnvironmentProfile".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.create_form_type {
                properties.insert(
                    "CreateFormType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.create_glossary {
                properties.insert(
                    "CreateGlossary".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.create_project {
                properties.insert(
                    "CreateProject".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.create_project_from_project_profile {
                properties.insert(
                    "CreateProjectFromProjectProfile".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.delegate_create_environment_profile {
                properties.insert(
                    "DelegateCreateEnvironmentProfile".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.override_domain_unit_owners {
                properties.insert(
                    "OverrideDomainUnitOwners".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.override_project_owners {
                properties.insert(
                    "OverrideProjectOwners".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-policygrant-policygrantprincipal.html>
    pub struct PolicyGrantPrincipal_ {
        pub domain_unit: Option<Box<DomainUnitPolicyGrantPrincipal_>>,
        pub group: Option<Box<GroupPolicyGrantPrincipal_>>,
        pub project: Option<Box<ProjectPolicyGrantPrincipal_>>,
        pub user: Option<Box<UserPolicyGrantPrincipal_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_PolicyGrant_PolicyGrantPrincipal {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::PolicyGrant.PolicyGrantPrincipal"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_PolicyGrant_PolicyGrantPrincipal as PolicyGrantPrincipal;
    impl crate::value::ToValue for PolicyGrantPrincipal_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.domain_unit {
                properties.insert(
                    "DomainUnit".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.group {
                properties.insert("Group".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.project {
                properties.insert(
                    "Project".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user {
                properties.insert("User".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-policygrant-projectgrantfilter.html>
    pub struct ProjectGrantFilter_ {
        pub domain_unit_filter: Box<DomainUnitFilterForProject_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_PolicyGrant_ProjectGrantFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::PolicyGrant.ProjectGrantFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_PolicyGrant_ProjectGrantFilter as ProjectGrantFilter;
    impl crate::value::ToValue for ProjectGrantFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DomainUnitFilter".to_string(),
                crate::value::ToValue::to_value(&self.domain_unit_filter),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-policygrant-projectpolicygrantprincipal.html>
    pub struct ProjectPolicyGrantPrincipal_ {
        pub project_designation: Option<crate::value::ExpString>,
        pub project_grant_filter: Option<Box<ProjectGrantFilter_>>,
        pub project_identifier: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_PolicyGrant_ProjectPolicyGrantPrincipal {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::PolicyGrant.ProjectPolicyGrantPrincipal"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_PolicyGrant_ProjectPolicyGrantPrincipal as ProjectPolicyGrantPrincipal;
    impl crate::value::ToValue for ProjectPolicyGrantPrincipal_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.project_designation {
                properties.insert(
                    "ProjectDesignation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.project_grant_filter {
                properties.insert(
                    "ProjectGrantFilter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.project_identifier {
                properties.insert(
                    "ProjectIdentifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-policygrant-userpolicygrantprincipal.html>
    pub struct UserPolicyGrantPrincipal_ {
        pub all_users_grant_filter: Option<serde_json::Value>,
        pub user_identifier: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_PolicyGrant_UserPolicyGrantPrincipal {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::PolicyGrant.UserPolicyGrantPrincipal"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_PolicyGrant_UserPolicyGrantPrincipal as UserPolicyGrantPrincipal;
    impl crate::value::ToValue for UserPolicyGrantPrincipal_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.all_users_grant_filter {
                properties.insert(
                    "AllUsersGrantFilter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_identifier {
                properties.insert(
                    "UserIdentifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod project {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-project-environmentconfigurationuserparameter.html>
    pub struct EnvironmentConfigurationUserParameter_ {
        pub environment_configuration_name: Option<crate::value::ExpString>,
        pub environment_id: Option<crate::value::ExpString>,
        pub environment_parameters: Option<Vec<EnvironmentParameter_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_Project_EnvironmentConfigurationUserParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::Project.EnvironmentConfigurationUserParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_Project_EnvironmentConfigurationUserParameter as EnvironmentConfigurationUserParameter;
    impl crate::value::ToValue for EnvironmentConfigurationUserParameter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.environment_configuration_name {
                properties.insert(
                    "EnvironmentConfigurationName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.environment_id {
                properties.insert(
                    "EnvironmentId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.environment_parameters {
                properties.insert(
                    "EnvironmentParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-project-environmentparameter.html>
    pub struct EnvironmentParameter_ {
        pub name: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_Project_EnvironmentParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::Project.EnvironmentParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_Project_EnvironmentParameter as EnvironmentParameter;
    impl crate::value::ToValue for EnvironmentParameter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod projectmembership {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-projectmembership-member.html>
    pub struct Member_ {
        pub group_identifier: Option<crate::value::ExpString>,
        pub user_identifier: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_ProjectMembership_Member {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::ProjectMembership.Member"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_ProjectMembership_Member as Member;
    impl crate::value::ToValue for Member_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.group_identifier {
                properties.insert(
                    "GroupIdentifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_identifier {
                properties.insert(
                    "UserIdentifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod projectprofile {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-projectprofile-awsaccount.html>
    pub struct AwsAccount_ {
        pub aws_account_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_ProjectProfile_AwsAccount {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::ProjectProfile.AwsAccount"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_ProjectProfile_AwsAccount as AwsAccount;
    impl crate::value::ToValue for AwsAccount_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AwsAccountId".to_string(),
                crate::value::ToValue::to_value(&self.aws_account_id),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-projectprofile-environmentconfiguration.html>
    pub struct EnvironmentConfiguration_ {
        pub aws_account: Option<Box<AwsAccount_>>,
        pub aws_region: Box<Region_>,
        pub configuration_parameters: Option<Box<EnvironmentConfigurationParametersDetails_>>,
        pub deployment_mode: Option<crate::value::ExpString>,
        pub deployment_order: Option<f64>,
        pub description: Option<crate::value::ExpString>,
        pub environment_blueprint_id: crate::value::ExpString,
        pub environment_configuration_id: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_ProjectProfile_EnvironmentConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::ProjectProfile.EnvironmentConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_ProjectProfile_EnvironmentConfiguration as EnvironmentConfiguration;
    impl crate::value::ToValue for EnvironmentConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.aws_account {
                properties.insert(
                    "AwsAccount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "AwsRegion".to_string(),
                crate::value::ToValue::to_value(&self.aws_region),
            );
            if let Some(ref value) = self.configuration_parameters {
                properties.insert(
                    "ConfigurationParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.deployment_mode {
                properties.insert(
                    "DeploymentMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.deployment_order {
                properties.insert(
                    "DeploymentOrder".to_string(),
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
                "EnvironmentBlueprintId".to_string(),
                crate::value::ToValue::to_value(&self.environment_blueprint_id),
            );
            if let Some(ref value) = self.environment_configuration_id {
                properties.insert(
                    "EnvironmentConfigurationId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-projectprofile-environmentconfigurationparameter.html>
    pub struct EnvironmentConfigurationParameter_ {
        pub is_editable: Option<crate::value::ExpBool>,
        pub name: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_ProjectProfile_EnvironmentConfigurationParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::ProjectProfile.EnvironmentConfigurationParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_ProjectProfile_EnvironmentConfigurationParameter as EnvironmentConfigurationParameter;
    impl crate::value::ToValue for EnvironmentConfigurationParameter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.is_editable {
                properties.insert(
                    "IsEditable".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-projectprofile-environmentconfigurationparametersdetails.html>
    pub struct EnvironmentConfigurationParametersDetails_ {
        pub parameter_overrides: Option<Vec<EnvironmentConfigurationParameter_>>,
        pub resolved_parameters: Option<Vec<EnvironmentConfigurationParameter_>>,
        pub ssm_path: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_ProjectProfile_EnvironmentConfigurationParametersDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::ProjectProfile.EnvironmentConfigurationParametersDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_ProjectProfile_EnvironmentConfigurationParametersDetails as EnvironmentConfigurationParametersDetails;
    impl crate::value::ToValue for EnvironmentConfigurationParametersDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.parameter_overrides {
                properties.insert(
                    "ParameterOverrides".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resolved_parameters {
                properties.insert(
                    "ResolvedParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ssm_path {
                properties.insert(
                    "SsmPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-projectprofile-region.html>
    pub struct Region_ {
        pub region_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_ProjectProfile_Region {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::ProjectProfile.Region"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_ProjectProfile_Region as Region;
    impl crate::value::ToValue for Region_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RegionName".to_string(),
                crate::value::ToValue::to_value(&self.region_name),
            );
            properties.into()
        }
    }
}
pub mod subscriptiontarget {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-subscriptiontarget-subscriptiontargetform.html>
    pub struct SubscriptionTargetForm_ {
        pub content: crate::value::ExpString,
        pub form_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_SubscriptionTarget_SubscriptionTargetForm {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::SubscriptionTarget.SubscriptionTargetForm"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_SubscriptionTarget_SubscriptionTargetForm as SubscriptionTargetForm;
    impl crate::value::ToValue for SubscriptionTargetForm_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Content".to_string(),
                crate::value::ToValue::to_value(&self.content),
            );
            properties.insert(
                "FormName".to_string(),
                crate::value::ToValue::to_value(&self.form_name),
            );
            properties.into()
        }
    }
}
pub mod userprofile {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-userprofile-iamuserprofiledetails.html>
    pub struct IamUserProfileDetails_ {
        pub arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_UserProfile_IamUserProfileDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::UserProfile.IamUserProfileDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_UserProfile_IamUserProfileDetails as IamUserProfileDetails;
    impl crate::value::ToValue for IamUserProfileDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.arn {
                properties.insert("Arn".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-userprofile-ssouserprofiledetails.html>
    pub struct SsoUserProfileDetails_ {
        pub first_name: Option<crate::value::ExpString>,
        pub last_name: Option<crate::value::ExpString>,
        pub username: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_UserProfile_SsoUserProfileDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::UserProfile.SsoUserProfileDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_UserProfile_SsoUserProfileDetails as SsoUserProfileDetails;
    impl crate::value::ToValue for SsoUserProfileDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.first_name {
                properties.insert(
                    "FirstName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.last_name {
                properties.insert(
                    "LastName".to_string(),
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datazone-userprofile-userprofiledetails.html>
    pub struct UserProfileDetails_ {
        pub iam: Option<Box<IamUserProfileDetails_>>,
        pub sso: Option<Box<SsoUserProfileDetails_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datazone_UserProfile_UserProfileDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataZone::UserProfile.UserProfileDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datazone_UserProfile_UserProfileDetails as UserProfileDetails;
    impl crate::value::ToValue for UserProfileDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.iam {
                properties.insert("Iam".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.sso {
                properties.insert("Sso".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-connection.html>
pub struct Connection_ {
    pub aws_location: Option<super::datazone::connection::AwsLocation_>,
    pub description: Option<crate::value::ExpString>,
    pub domain_identifier: crate::value::ExpString,
    pub enable_trusted_identity_propagation: Option<crate::value::ExpBool>,
    pub environment_identifier: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub project_identifier: Option<crate::value::ExpString>,
    pub props: Option<super::datazone::connection::ConnectionPropertiesInput_>,
    pub scope: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_datazone_Connection {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DataZone::Connection"
        $($field $value)*)
    };
}
pub use crate::__aws_datazone_Connection as Connection;
impl crate::template::ToResource for Connection_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DataZone"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Connection"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.aws_location {
            properties.insert(
                "AwsLocation".to_string(),
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
            "DomainIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.domain_identifier),
        );
        if let Some(ref value) = self.enable_trusted_identity_propagation {
            properties.insert(
                "EnableTrustedIdentityPropagation".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.environment_identifier {
            properties.insert(
                "EnvironmentIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.project_identifier {
            properties.insert(
                "ProjectIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.props {
            properties.insert("Props".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.scope {
            properties.insert("Scope".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-datasource.html>
pub struct DataSource_ {
    pub asset_forms_input: Option<Vec<super::datazone::datasource::FormInput_>>,
    pub configuration: Option<super::datazone::datasource::DataSourceConfigurationInput_>,
    pub connection_identifier: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub domain_identifier: crate::value::ExpString,
    pub enable_setting: Option<crate::value::ExpString>,
    pub environment_identifier: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub project_identifier: crate::value::ExpString,
    pub publish_on_import: Option<crate::value::ExpBool>,
    pub recommendation: Option<super::datazone::datasource::RecommendationConfiguration_>,
    pub schedule: Option<super::datazone::datasource::ScheduleConfiguration_>,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_datazone_DataSource {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DataZone::DataSource"
        $($field $value)*)
    };
}
pub use crate::__aws_datazone_DataSource as DataSource;
impl crate::template::ToResource for DataSource_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DataZone"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DataSource"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.asset_forms_input {
            properties.insert(
                "AssetFormsInput".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.configuration {
            properties.insert(
                "Configuration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.connection_identifier {
            properties.insert(
                "ConnectionIdentifier".to_string(),
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
            "DomainIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.domain_identifier),
        );
        if let Some(ref value) = self.enable_setting {
            properties.insert(
                "EnableSetting".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.environment_identifier {
            properties.insert(
                "EnvironmentIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "ProjectIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.project_identifier),
        );
        if let Some(ref value) = self.publish_on_import {
            properties.insert(
                "PublishOnImport".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.recommendation {
            properties.insert(
                "Recommendation".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.schedule {
            properties.insert(
                "Schedule".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-domain.html>
pub struct Domain_ {
    pub description: Option<crate::value::ExpString>,
    pub domain_execution_role: crate::value::ExpString,
    pub domain_version: Option<crate::value::ExpString>,
    pub kms_key_identifier: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub service_role: Option<crate::value::ExpString>,
    pub single_sign_on: Option<super::datazone::domain::SingleSignOn_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_datazone_Domain {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DataZone::Domain"
        $($field $value)*)
    };
}
pub use crate::__aws_datazone_Domain as Domain;
impl crate::template::ToResource for Domain_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DataZone"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Domain"),
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
            "DomainExecutionRole".to_string(),
            crate::value::ToValue::to_value(&self.domain_execution_role),
        );
        if let Some(ref value) = self.domain_version {
            properties.insert(
                "DomainVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_identifier {
            properties.insert(
                "KmsKeyIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.service_role {
            properties.insert(
                "ServiceRole".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.single_sign_on {
            properties.insert(
                "SingleSignOn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-domainunit.html>
pub struct DomainUnit_ {
    pub description: Option<crate::value::ExpString>,
    pub domain_identifier: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub parent_domain_unit_identifier: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_datazone_DomainUnit {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DataZone::DomainUnit"
        $($field $value)*)
    };
}
pub use crate::__aws_datazone_DomainUnit as DomainUnit;
impl crate::template::ToResource for DomainUnit_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DataZone"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DomainUnit"),
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
            "DomainIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.domain_identifier),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "ParentDomainUnitIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.parent_domain_unit_identifier),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-environment.html>
pub struct Environment_ {
    pub deployment_order: Option<i32>,
    pub description: Option<crate::value::ExpString>,
    pub domain_identifier: crate::value::ExpString,
    pub environment_account_identifier: Option<crate::value::ExpString>,
    pub environment_account_region: Option<crate::value::ExpString>,
    pub environment_blueprint_identifier: Option<crate::value::ExpString>,
    pub environment_configuration_id: Option<crate::value::ExpString>,
    pub environment_profile_identifier: Option<crate::value::ExpString>,
    pub environment_role_arn: Option<crate::value::ExpString>,
    pub glossary_terms: Option<Vec<crate::value::ExpString>>,
    pub name: crate::value::ExpString,
    pub project_identifier: crate::value::ExpString,
    pub user_parameters: Option<Vec<super::datazone::environment::EnvironmentParameter_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_datazone_Environment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DataZone::Environment"
        $($field $value)*)
    };
}
pub use crate::__aws_datazone_Environment as Environment;
impl crate::template::ToResource for Environment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DataZone"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Environment"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.deployment_order {
            properties.insert(
                "DeploymentOrder".to_string(),
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
            "DomainIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.domain_identifier),
        );
        if let Some(ref value) = self.environment_account_identifier {
            properties.insert(
                "EnvironmentAccountIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.environment_account_region {
            properties.insert(
                "EnvironmentAccountRegion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.environment_blueprint_identifier {
            properties.insert(
                "EnvironmentBlueprintIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.environment_configuration_id {
            properties.insert(
                "EnvironmentConfigurationId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.environment_profile_identifier {
            properties.insert(
                "EnvironmentProfileIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.environment_role_arn {
            properties.insert(
                "EnvironmentRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.glossary_terms {
            properties.insert(
                "GlossaryTerms".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "ProjectIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.project_identifier),
        );
        if let Some(ref value) = self.user_parameters {
            properties.insert(
                "UserParameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-environmentactions.html>
pub struct EnvironmentActions_ {
    pub description: Option<crate::value::ExpString>,
    pub domain_identifier: Option<crate::value::ExpString>,
    pub environment_identifier: Option<crate::value::ExpString>,
    pub identifier: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub parameters: Option<super::datazone::environmentactions::AwsConsoleLinkParameters_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_datazone_EnvironmentActions {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DataZone::EnvironmentActions"
        $($field $value)*)
    };
}
pub use crate::__aws_datazone_EnvironmentActions as EnvironmentActions;
impl crate::template::ToResource for EnvironmentActions_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DataZone"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("EnvironmentActions"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.domain_identifier {
            properties.insert(
                "DomainIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.environment_identifier {
            properties.insert(
                "EnvironmentIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.identifier {
            properties.insert(
                "Identifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.parameters {
            properties.insert(
                "Parameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-environmentblueprintconfiguration.html>
pub struct EnvironmentBlueprintConfiguration_ {
    pub domain_identifier: crate::value::ExpString,
    pub enabled_regions: Vec<crate::value::ExpString>,
    pub environment_blueprint_identifier: crate::value::ExpString,
    pub environment_role_permission_boundary: Option<crate::value::ExpString>,
    pub global_parameters: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub manage_access_role_arn: Option<crate::value::ExpString>,
    pub provisioning_configurations:
        Option<Vec<super::datazone::environmentblueprintconfiguration::ProvisioningConfiguration_>>,
    pub provisioning_role_arn: Option<crate::value::ExpString>,
    pub regional_parameters:
        Option<Vec<super::datazone::environmentblueprintconfiguration::RegionalParameter_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_datazone_EnvironmentBlueprintConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DataZone::EnvironmentBlueprintConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_datazone_EnvironmentBlueprintConfiguration as EnvironmentBlueprintConfiguration;
impl crate::template::ToResource for EnvironmentBlueprintConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DataZone"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "EnvironmentBlueprintConfiguration",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DomainIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.domain_identifier),
        );
        properties.insert(
            "EnabledRegions".to_string(),
            crate::value::ToValue::to_value(&self.enabled_regions),
        );
        properties.insert(
            "EnvironmentBlueprintIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.environment_blueprint_identifier),
        );
        if let Some(ref value) = self.environment_role_permission_boundary {
            properties.insert(
                "EnvironmentRolePermissionBoundary".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.global_parameters {
            properties.insert(
                "GlobalParameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.manage_access_role_arn {
            properties.insert(
                "ManageAccessRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.provisioning_configurations {
            properties.insert(
                "ProvisioningConfigurations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.provisioning_role_arn {
            properties.insert(
                "ProvisioningRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.regional_parameters {
            properties.insert(
                "RegionalParameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-environmentprofile.html>
pub struct EnvironmentProfile_ {
    pub aws_account_id: crate::value::ExpString,
    pub aws_account_region: crate::value::ExpString,
    pub description: Option<crate::value::ExpString>,
    pub domain_identifier: crate::value::ExpString,
    pub environment_blueprint_identifier: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub project_identifier: crate::value::ExpString,
    pub user_parameters: Option<Vec<super::datazone::environmentprofile::EnvironmentParameter_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_datazone_EnvironmentProfile {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DataZone::EnvironmentProfile"
        $($field $value)*)
    };
}
pub use crate::__aws_datazone_EnvironmentProfile as EnvironmentProfile;
impl crate::template::ToResource for EnvironmentProfile_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DataZone"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("EnvironmentProfile"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AwsAccountId".to_string(),
            crate::value::ToValue::to_value(&self.aws_account_id),
        );
        properties.insert(
            "AwsAccountRegion".to_string(),
            crate::value::ToValue::to_value(&self.aws_account_region),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DomainIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.domain_identifier),
        );
        properties.insert(
            "EnvironmentBlueprintIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.environment_blueprint_identifier),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "ProjectIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.project_identifier),
        );
        if let Some(ref value) = self.user_parameters {
            properties.insert(
                "UserParameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-formtype.html>
pub struct FormType_ {
    pub description: Option<crate::value::ExpString>,
    pub domain_identifier: crate::value::ExpString,
    pub model: super::datazone::formtype::Model_,
    pub name: crate::value::ExpString,
    pub owning_project_identifier: crate::value::ExpString,
    pub status: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_datazone_FormType {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DataZone::FormType"
        $($field $value)*)
    };
}
pub use crate::__aws_datazone_FormType as FormType;
impl crate::template::ToResource for FormType_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DataZone"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("FormType"),
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
            "DomainIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.domain_identifier),
        );
        properties.insert(
            "Model".to_string(),
            crate::value::ToValue::to_value(&self.model),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "OwningProjectIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.owning_project_identifier),
        );
        if let Some(ref value) = self.status {
            properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-groupprofile.html>
pub struct GroupProfile_ {
    pub domain_identifier: crate::value::ExpString,
    pub group_identifier: crate::value::ExpString,
    pub status: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_datazone_GroupProfile {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DataZone::GroupProfile"
        $($field $value)*)
    };
}
pub use crate::__aws_datazone_GroupProfile as GroupProfile;
impl crate::template::ToResource for GroupProfile_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DataZone"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("GroupProfile"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DomainIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.domain_identifier),
        );
        properties.insert(
            "GroupIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.group_identifier),
        );
        if let Some(ref value) = self.status {
            properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-owner.html>
pub struct Owner_ {
    pub domain_identifier: crate::value::ExpString,
    pub entity_identifier: crate::value::ExpString,
    pub entity_type: crate::value::ExpString,
    pub owner: super::datazone::owner::OwnerProperties_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_datazone_Owner {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DataZone::Owner" $($field
        $value)*)
    };
}
pub use crate::__aws_datazone_Owner as Owner;
impl crate::template::ToResource for Owner_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DataZone"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Owner"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DomainIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.domain_identifier),
        );
        properties.insert(
            "EntityIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.entity_identifier),
        );
        properties.insert(
            "EntityType".to_string(),
            crate::value::ToValue::to_value(&self.entity_type),
        );
        properties.insert(
            "Owner".to_string(),
            crate::value::ToValue::to_value(&self.owner),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-policygrant.html>
pub struct PolicyGrant_ {
    pub detail: Option<super::datazone::policygrant::PolicyGrantDetail_>,
    pub domain_identifier: crate::value::ExpString,
    pub entity_identifier: crate::value::ExpString,
    pub entity_type: crate::value::ExpString,
    pub policy_type: crate::value::ExpString,
    pub principal: Option<super::datazone::policygrant::PolicyGrantPrincipal_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_datazone_PolicyGrant {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DataZone::PolicyGrant"
        $($field $value)*)
    };
}
pub use crate::__aws_datazone_PolicyGrant as PolicyGrant;
impl crate::template::ToResource for PolicyGrant_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DataZone"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PolicyGrant"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.detail {
            properties.insert("Detail".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "DomainIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.domain_identifier),
        );
        properties.insert(
            "EntityIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.entity_identifier),
        );
        properties.insert(
            "EntityType".to_string(),
            crate::value::ToValue::to_value(&self.entity_type),
        );
        properties.insert(
            "PolicyType".to_string(),
            crate::value::ToValue::to_value(&self.policy_type),
        );
        if let Some(ref value) = self.principal {
            properties.insert(
                "Principal".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-project.html>
pub struct Project_ {
    pub description: Option<crate::value::ExpString>,
    pub domain_identifier: crate::value::ExpString,
    pub domain_unit_id: Option<crate::value::ExpString>,
    pub glossary_terms: Option<Vec<crate::value::ExpString>>,
    pub name: crate::value::ExpString,
    pub project_profile_id: Option<crate::value::ExpString>,
    pub project_profile_version: Option<crate::value::ExpString>,
    pub user_parameters:
        Option<Vec<super::datazone::project::EnvironmentConfigurationUserParameter_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_datazone_Project {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DataZone::Project"
        $($field $value)*)
    };
}
pub use crate::__aws_datazone_Project as Project;
impl crate::template::ToResource for Project_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DataZone"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Project"),
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
            "DomainIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.domain_identifier),
        );
        if let Some(ref value) = self.domain_unit_id {
            properties.insert(
                "DomainUnitId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.glossary_terms {
            properties.insert(
                "GlossaryTerms".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.project_profile_id {
            properties.insert(
                "ProjectProfileId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.project_profile_version {
            properties.insert(
                "ProjectProfileVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.user_parameters {
            properties.insert(
                "UserParameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-projectmembership.html>
pub struct ProjectMembership_ {
    pub designation: crate::value::ExpString,
    pub domain_identifier: crate::value::ExpString,
    pub member: super::datazone::projectmembership::Member_,
    pub project_identifier: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_datazone_ProjectMembership {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DataZone::ProjectMembership"
        $($field $value)*)
    };
}
pub use crate::__aws_datazone_ProjectMembership as ProjectMembership;
impl crate::template::ToResource for ProjectMembership_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DataZone"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ProjectMembership"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Designation".to_string(),
            crate::value::ToValue::to_value(&self.designation),
        );
        properties.insert(
            "DomainIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.domain_identifier),
        );
        properties.insert(
            "Member".to_string(),
            crate::value::ToValue::to_value(&self.member),
        );
        properties.insert(
            "ProjectIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.project_identifier),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-projectprofile.html>
pub struct ProjectProfile_ {
    pub description: Option<crate::value::ExpString>,
    pub domain_identifier: Option<crate::value::ExpString>,
    pub domain_unit_identifier: Option<crate::value::ExpString>,
    pub environment_configurations:
        Option<Vec<super::datazone::projectprofile::EnvironmentConfiguration_>>,
    pub name: crate::value::ExpString,
    pub status: Option<crate::value::ExpString>,
    pub use_default_configurations: Option<crate::value::ExpBool>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_datazone_ProjectProfile {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DataZone::ProjectProfile"
        $($field $value)*)
    };
}
pub use crate::__aws_datazone_ProjectProfile as ProjectProfile;
impl crate::template::ToResource for ProjectProfile_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DataZone"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ProjectProfile"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.domain_identifier {
            properties.insert(
                "DomainIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.domain_unit_identifier {
            properties.insert(
                "DomainUnitIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.environment_configurations {
            properties.insert(
                "EnvironmentConfigurations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.status {
            properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.use_default_configurations {
            properties.insert(
                "UseDefaultConfigurations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-subscriptiontarget.html>
pub struct SubscriptionTarget_ {
    pub applicable_asset_types: Vec<crate::value::ExpString>,
    pub authorized_principals: Vec<crate::value::ExpString>,
    pub domain_identifier: crate::value::ExpString,
    pub environment_identifier: crate::value::ExpString,
    pub manage_access_role: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub provider: Option<crate::value::ExpString>,
    pub subscription_target_config:
        Vec<super::datazone::subscriptiontarget::SubscriptionTargetForm_>,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_datazone_SubscriptionTarget {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DataZone::SubscriptionTarget"
        $($field $value)*)
    };
}
pub use crate::__aws_datazone_SubscriptionTarget as SubscriptionTarget;
impl crate::template::ToResource for SubscriptionTarget_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DataZone"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SubscriptionTarget"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApplicableAssetTypes".to_string(),
            crate::value::ToValue::to_value(&self.applicable_asset_types),
        );
        properties.insert(
            "AuthorizedPrincipals".to_string(),
            crate::value::ToValue::to_value(&self.authorized_principals),
        );
        properties.insert(
            "DomainIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.domain_identifier),
        );
        properties.insert(
            "EnvironmentIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.environment_identifier),
        );
        if let Some(ref value) = self.manage_access_role {
            properties.insert(
                "ManageAccessRole".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.provider {
            properties.insert(
                "Provider".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SubscriptionTargetConfig".to_string(),
            crate::value::ToValue::to_value(&self.subscription_target_config),
        );
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datazone-userprofile.html>
pub struct UserProfile_ {
    pub domain_identifier: crate::value::ExpString,
    pub status: Option<crate::value::ExpString>,
    pub user_identifier: crate::value::ExpString,
    pub user_type: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_datazone_UserProfile {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DataZone::UserProfile"
        $($field $value)*)
    };
}
pub use crate::__aws_datazone_UserProfile as UserProfile;
impl crate::template::ToResource for UserProfile_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DataZone"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("UserProfile"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DomainIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.domain_identifier),
        );
        if let Some(ref value) = self.status {
            properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "UserIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.user_identifier),
        );
        if let Some(ref value) = self.user_type {
            properties.insert(
                "UserType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
