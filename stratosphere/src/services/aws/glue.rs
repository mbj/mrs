pub mod classifier {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-classifier-csvclassifier.html
    pub struct CsvClassifier_ {
        pub allow_single_column: Option<crate::value::ExpBool>,
        pub contains_custom_datatype: Option<Vec<crate::value::ExpString>>,
        pub contains_header: Option<crate::value::ExpString>,
        pub custom_datatype_configured: Option<crate::value::ExpBool>,
        pub delimiter: Option<crate::value::ExpString>,
        pub disable_value_trimming: Option<crate::value::ExpBool>,
        pub header: Option<Vec<crate::value::ExpString>>,
        pub name: Option<crate::value::ExpString>,
        pub quote_symbol: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Classifier_CsvClassifier {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Classifier.CsvClassifier"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Classifier_CsvClassifier as CsvClassifier;
    impl crate::value::ToValue for CsvClassifier_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allow_single_column {
                properties.insert(
                    "AllowSingleColumn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.contains_custom_datatype {
                properties.insert(
                    "ContainsCustomDatatype".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.contains_header {
                properties.insert(
                    "ContainsHeader".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_datatype_configured {
                properties.insert(
                    "CustomDatatypeConfigured".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.delimiter {
                properties.insert(
                    "Delimiter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.disable_value_trimming {
                properties.insert(
                    "DisableValueTrimming".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.header {
                properties.insert("Header".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.quote_symbol {
                properties.insert(
                    "QuoteSymbol".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-classifier-grokclassifier.html
    pub struct GrokClassifier_ {
        pub classification: crate::value::ExpString,
        pub custom_patterns: Option<crate::value::ExpString>,
        pub grok_pattern: crate::value::ExpString,
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Classifier_GrokClassifier {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Classifier.GrokClassifier"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Classifier_GrokClassifier as GrokClassifier;
    impl crate::value::ToValue for GrokClassifier_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Classification".to_string(),
                crate::value::ToValue::to_value(&self.classification),
            );
            if let Some(ref value) = self.custom_patterns {
                properties.insert(
                    "CustomPatterns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "GrokPattern".to_string(),
                crate::value::ToValue::to_value(&self.grok_pattern),
            );
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-classifier-jsonclassifier.html
    pub struct JsonClassifier_ {
        pub json_path: crate::value::ExpString,
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Classifier_JsonClassifier {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Classifier.JsonClassifier"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Classifier_JsonClassifier as JsonClassifier;
    impl crate::value::ToValue for JsonClassifier_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "JsonPath".to_string(),
                crate::value::ToValue::to_value(&self.json_path),
            );
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-classifier-xmlclassifier.html
    pub struct XMLClassifier_ {
        pub classification: crate::value::ExpString,
        pub name: Option<crate::value::ExpString>,
        pub row_tag: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Classifier_XMLClassifier {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Classifier.XMLClassifier"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Classifier_XMLClassifier as XMLClassifier;
    impl crate::value::ToValue for XMLClassifier_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Classification".to_string(),
                crate::value::ToValue::to_value(&self.classification),
            );
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "RowTag".to_string(),
                crate::value::ToValue::to_value(&self.row_tag),
            );
            properties.into()
        }
    }
}
pub mod connection {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-connection-authenticationconfigurationinput.html
    pub struct AuthenticationConfigurationInput_ {
        pub authentication_type: crate::value::ExpString,
        pub basic_authentication_credentials: Option<Box<BasicAuthenticationCredentials_>>,
        pub custom_authentication_credentials: Option<serde_json::Value>,
        pub kms_key_arn: Option<crate::value::ExpString>,
        pub o_auth2_properties: Option<Box<OAuth2PropertiesInput_>>,
        pub secret_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Connection_AuthenticationConfigurationInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Connection.AuthenticationConfigurationInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Connection_AuthenticationConfigurationInput as AuthenticationConfigurationInput;
    impl crate::value::ToValue for AuthenticationConfigurationInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AuthenticationType".to_string(),
                crate::value::ToValue::to_value(&self.authentication_type),
            );
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-connection-authorizationcodeproperties.html
    pub struct AuthorizationCodeProperties_ {
        pub authorization_code: Option<crate::value::ExpString>,
        pub redirect_uri: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Connection_AuthorizationCodeProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Connection.AuthorizationCodeProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Connection_AuthorizationCodeProperties as AuthorizationCodeProperties;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-connection-basicauthenticationcredentials.html
    pub struct BasicAuthenticationCredentials_ {
        pub password: Option<crate::value::ExpString>,
        pub username: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Connection_BasicAuthenticationCredentials {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Connection.BasicAuthenticationCredentials"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Connection_BasicAuthenticationCredentials as BasicAuthenticationCredentials;
    impl crate::value::ToValue for BasicAuthenticationCredentials_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-connection-connectioninput.html
    pub struct ConnectionInput_ {
        pub athena_properties: Option<serde_json::Value>,
        pub authentication_configuration: Option<Box<AuthenticationConfigurationInput_>>,
        pub connection_properties: Option<serde_json::Value>,
        pub connection_type: crate::value::ExpString,
        pub description: Option<crate::value::ExpString>,
        pub match_criteria: Option<Vec<crate::value::ExpString>>,
        pub name: Option<crate::value::ExpString>,
        pub physical_connection_requirements: Option<Box<PhysicalConnectionRequirements_>>,
        pub python_properties: Option<serde_json::Value>,
        pub spark_properties: Option<serde_json::Value>,
        pub validate_credentials: Option<crate::value::ExpBool>,
        pub validate_for_compute_environments: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Connection_ConnectionInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Connection.ConnectionInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Connection_ConnectionInput as ConnectionInput;
    impl crate::value::ToValue for ConnectionInput_ {
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
            properties.insert(
                "ConnectionType".to_string(),
                crate::value::ToValue::to_value(&self.connection_type),
            );
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-connection-oauth2clientapplication.html
    pub struct OAuth2ClientApplication_ {
        pub aws_managed_client_application_reference: Option<crate::value::ExpString>,
        pub user_managed_client_application_client_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Connection_OAuth2ClientApplication {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Connection.OAuth2ClientApplication"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Connection_OAuth2ClientApplication as OAuth2ClientApplication;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-connection-oauth2credentials.html
    pub struct OAuth2Credentials_ {
        pub access_token: Option<crate::value::ExpString>,
        pub jwt_token: Option<crate::value::ExpString>,
        pub refresh_token: Option<crate::value::ExpString>,
        pub user_managed_client_application_client_secret: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Connection_OAuth2Credentials {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Connection.OAuth2Credentials"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Connection_OAuth2Credentials as OAuth2Credentials;
    impl crate::value::ToValue for OAuth2Credentials_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-connection-oauth2propertiesinput.html
    pub struct OAuth2PropertiesInput_ {
        pub authorization_code_properties: Option<Box<AuthorizationCodeProperties_>>,
        pub o_auth2_client_application: Option<Box<OAuth2ClientApplication_>>,
        pub o_auth2_credentials: Option<Box<OAuth2Credentials_>>,
        pub o_auth2_grant_type: Option<crate::value::ExpString>,
        pub token_url: Option<crate::value::ExpString>,
        pub token_url_parameters_map: Option<serde_json::Value>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Connection_OAuth2PropertiesInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Connection.OAuth2PropertiesInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Connection_OAuth2PropertiesInput as OAuth2PropertiesInput;
    impl crate::value::ToValue for OAuth2PropertiesInput_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-connection-physicalconnectionrequirements.html
    pub struct PhysicalConnectionRequirements_ {
        pub availability_zone: Option<crate::value::ExpString>,
        pub security_group_id_list: Option<Vec<crate::value::ExpString>>,
        pub subnet_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Connection_PhysicalConnectionRequirements {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Connection.PhysicalConnectionRequirements"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Connection_PhysicalConnectionRequirements as PhysicalConnectionRequirements;
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
            properties.into()
        }
    }
}
pub mod crawler {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-catalogtarget.html
    pub struct CatalogTarget_ {
        pub connection_name: Option<crate::value::ExpString>,
        pub database_name: Option<crate::value::ExpString>,
        pub dlq_event_queue_arn: Option<crate::value::ExpString>,
        pub event_queue_arn: Option<crate::value::ExpString>,
        pub tables: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Crawler_CatalogTarget {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Crawler.CatalogTarget"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Crawler_CatalogTarget as CatalogTarget;
    impl crate::value::ToValue for CatalogTarget_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.connection_name {
                properties.insert(
                    "ConnectionName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.database_name {
                properties.insert(
                    "DatabaseName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dlq_event_queue_arn {
                properties.insert(
                    "DlqEventQueueArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.event_queue_arn {
                properties.insert(
                    "EventQueueArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tables {
                properties.insert("Tables".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-deltatarget.html
    pub struct DeltaTarget_ {
        pub connection_name: Option<crate::value::ExpString>,
        pub create_native_delta_table: Option<crate::value::ExpBool>,
        pub delta_tables: Option<Vec<crate::value::ExpString>>,
        pub write_manifest: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Crawler_DeltaTarget {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Crawler.DeltaTarget"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Crawler_DeltaTarget as DeltaTarget;
    impl crate::value::ToValue for DeltaTarget_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.connection_name {
                properties.insert(
                    "ConnectionName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.create_native_delta_table {
                properties.insert(
                    "CreateNativeDeltaTable".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.delta_tables {
                properties.insert(
                    "DeltaTables".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.write_manifest {
                properties.insert(
                    "WriteManifest".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-dynamodbtarget.html
    pub struct DynamoDBTarget_ {
        pub path: Option<crate::value::ExpString>,
        pub scan_all: Option<crate::value::ExpBool>,
        pub scan_rate: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Crawler_DynamoDBTarget {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Crawler.DynamoDBTarget"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Crawler_DynamoDBTarget as DynamoDBTarget;
    impl crate::value::ToValue for DynamoDBTarget_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.path {
                properties.insert("Path".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.scan_all {
                properties.insert(
                    "ScanAll".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scan_rate {
                properties.insert(
                    "ScanRate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-huditarget.html
    pub struct HudiTarget_ {
        pub connection_name: Option<crate::value::ExpString>,
        pub exclusions: Option<Vec<crate::value::ExpString>>,
        pub maximum_traversal_depth: Option<i32>,
        pub paths: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Crawler_HudiTarget {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Crawler.HudiTarget"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Crawler_HudiTarget as HudiTarget;
    impl crate::value::ToValue for HudiTarget_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.connection_name {
                properties.insert(
                    "ConnectionName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exclusions {
                properties.insert(
                    "Exclusions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.maximum_traversal_depth {
                properties.insert(
                    "MaximumTraversalDepth".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.paths {
                properties.insert("Paths".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-icebergtarget.html
    pub struct IcebergTarget_ {
        pub connection_name: Option<crate::value::ExpString>,
        pub exclusions: Option<Vec<crate::value::ExpString>>,
        pub maximum_traversal_depth: Option<i32>,
        pub paths: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Crawler_IcebergTarget {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Crawler.IcebergTarget"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Crawler_IcebergTarget as IcebergTarget;
    impl crate::value::ToValue for IcebergTarget_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.connection_name {
                properties.insert(
                    "ConnectionName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exclusions {
                properties.insert(
                    "Exclusions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.maximum_traversal_depth {
                properties.insert(
                    "MaximumTraversalDepth".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.paths {
                properties.insert("Paths".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-jdbctarget.html
    pub struct JdbcTarget_ {
        pub connection_name: Option<crate::value::ExpString>,
        pub enable_additional_metadata: Option<Vec<crate::value::ExpString>>,
        pub exclusions: Option<Vec<crate::value::ExpString>>,
        pub path: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Crawler_JdbcTarget {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Crawler.JdbcTarget"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Crawler_JdbcTarget as JdbcTarget;
    impl crate::value::ToValue for JdbcTarget_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.connection_name {
                properties.insert(
                    "ConnectionName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enable_additional_metadata {
                properties.insert(
                    "EnableAdditionalMetadata".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exclusions {
                properties.insert(
                    "Exclusions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.path {
                properties.insert("Path".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-lakeformationconfiguration.html
    pub struct LakeFormationConfiguration_ {
        pub account_id: Option<crate::value::ExpString>,
        pub use_lake_formation_credentials: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Crawler_LakeFormationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Crawler.LakeFormationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Crawler_LakeFormationConfiguration as LakeFormationConfiguration;
    impl crate::value::ToValue for LakeFormationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.account_id {
                properties.insert(
                    "AccountId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.use_lake_formation_credentials {
                properties.insert(
                    "UseLakeFormationCredentials".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-mongodbtarget.html
    pub struct MongoDBTarget_ {
        pub connection_name: Option<crate::value::ExpString>,
        pub path: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Crawler_MongoDBTarget {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Crawler.MongoDBTarget"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Crawler_MongoDBTarget as MongoDBTarget;
    impl crate::value::ToValue for MongoDBTarget_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.connection_name {
                properties.insert(
                    "ConnectionName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.path {
                properties.insert("Path".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-recrawlpolicy.html
    pub struct RecrawlPolicy_ {
        pub recrawl_behavior: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Crawler_RecrawlPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Crawler.RecrawlPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Crawler_RecrawlPolicy as RecrawlPolicy;
    impl crate::value::ToValue for RecrawlPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.recrawl_behavior {
                properties.insert(
                    "RecrawlBehavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-s3target.html
    pub struct S3Target_ {
        pub connection_name: Option<crate::value::ExpString>,
        pub dlq_event_queue_arn: Option<crate::value::ExpString>,
        pub event_queue_arn: Option<crate::value::ExpString>,
        pub exclusions: Option<Vec<crate::value::ExpString>>,
        pub path: Option<crate::value::ExpString>,
        pub sample_size: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Crawler_S3Target {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Crawler.S3Target"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Crawler_S3Target as S3Target;
    impl crate::value::ToValue for S3Target_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.connection_name {
                properties.insert(
                    "ConnectionName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dlq_event_queue_arn {
                properties.insert(
                    "DlqEventQueueArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.event_queue_arn {
                properties.insert(
                    "EventQueueArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exclusions {
                properties.insert(
                    "Exclusions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.path {
                properties.insert("Path".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.sample_size {
                properties.insert(
                    "SampleSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-schedule.html
    pub struct Schedule_ {
        pub schedule_expression: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Crawler_Schedule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Crawler.Schedule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Crawler_Schedule as Schedule;
    impl crate::value::ToValue for Schedule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.schedule_expression {
                properties.insert(
                    "ScheduleExpression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-schemachangepolicy.html
    pub struct SchemaChangePolicy_ {
        pub delete_behavior: Option<crate::value::ExpString>,
        pub update_behavior: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Crawler_SchemaChangePolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Crawler.SchemaChangePolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Crawler_SchemaChangePolicy as SchemaChangePolicy;
    impl crate::value::ToValue for SchemaChangePolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.delete_behavior {
                properties.insert(
                    "DeleteBehavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.update_behavior {
                properties.insert(
                    "UpdateBehavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-targets.html
    pub struct Targets_ {
        pub catalog_targets: Option<Vec<CatalogTarget_>>,
        pub delta_targets: Option<Vec<DeltaTarget_>>,
        pub dynamo_db_targets: Option<Vec<DynamoDBTarget_>>,
        pub hudi_targets: Option<Vec<HudiTarget_>>,
        pub iceberg_targets: Option<Vec<IcebergTarget_>>,
        pub jdbc_targets: Option<Vec<JdbcTarget_>>,
        pub mongo_db_targets: Option<Vec<MongoDBTarget_>>,
        pub s3_targets: Option<Vec<S3Target_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Crawler_Targets {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Crawler.Targets"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Crawler_Targets as Targets;
    impl crate::value::ToValue for Targets_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.catalog_targets {
                properties.insert(
                    "CatalogTargets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.delta_targets {
                properties.insert(
                    "DeltaTargets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dynamo_db_targets {
                properties.insert(
                    "DynamoDBTargets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hudi_targets {
                properties.insert(
                    "HudiTargets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.iceberg_targets {
                properties.insert(
                    "IcebergTargets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.jdbc_targets {
                properties.insert(
                    "JdbcTargets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mongo_db_targets {
                properties.insert(
                    "MongoDBTargets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_targets {
                properties.insert(
                    "S3Targets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod datacatalogencryptionsettings {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-datacatalogencryptionsettings-connectionpasswordencryption.html
    pub struct ConnectionPasswordEncryption_ {
        pub kms_key_id: Option<crate::value::ExpString>,
        pub return_connection_password_encrypted: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_DataCatalogEncryptionSettings_ConnectionPasswordEncryption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::DataCatalogEncryptionSettings.ConnectionPasswordEncryption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_DataCatalogEncryptionSettings_ConnectionPasswordEncryption as ConnectionPasswordEncryption;
    impl crate::value::ToValue for ConnectionPasswordEncryption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.return_connection_password_encrypted {
                properties.insert(
                    "ReturnConnectionPasswordEncrypted".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-datacatalogencryptionsettings-datacatalogencryptionsettings.html
    pub struct DataCatalogEncryptionSettings_ {
        pub connection_password_encryption: Option<Box<ConnectionPasswordEncryption_>>,
        pub encryption_at_rest: Option<Box<EncryptionAtRest_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_DataCatalogEncryptionSettings_DataCatalogEncryptionSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::DataCatalogEncryptionSettings.DataCatalogEncryptionSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_DataCatalogEncryptionSettings_DataCatalogEncryptionSettings as DataCatalogEncryptionSettings;
    impl crate::value::ToValue for DataCatalogEncryptionSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.connection_password_encryption {
                properties.insert(
                    "ConnectionPasswordEncryption".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.encryption_at_rest {
                properties.insert(
                    "EncryptionAtRest".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-datacatalogencryptionsettings-encryptionatrest.html
    pub struct EncryptionAtRest_ {
        pub catalog_encryption_mode: Option<crate::value::ExpString>,
        pub catalog_encryption_service_role: Option<crate::value::ExpString>,
        pub sse_aws_kms_key_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_DataCatalogEncryptionSettings_EncryptionAtRest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::DataCatalogEncryptionSettings.EncryptionAtRest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_DataCatalogEncryptionSettings_EncryptionAtRest as EncryptionAtRest;
    impl crate::value::ToValue for EncryptionAtRest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.catalog_encryption_mode {
                properties.insert(
                    "CatalogEncryptionMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.catalog_encryption_service_role {
                properties.insert(
                    "CatalogEncryptionServiceRole".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sse_aws_kms_key_id {
                properties.insert(
                    "SseAwsKmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod dataqualityruleset {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-dataqualityruleset-dataqualitytargettable.html
    pub struct DataQualityTargetTable_ {
        pub database_name: Option<crate::value::ExpString>,
        pub table_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_DataQualityRuleset_DataQualityTargetTable {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::DataQualityRuleset.DataQualityTargetTable"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_DataQualityRuleset_DataQualityTargetTable as DataQualityTargetTable;
    impl crate::value::ToValue for DataQualityTargetTable_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.database_name {
                properties.insert(
                    "DatabaseName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.table_name {
                properties.insert(
                    "TableName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod database {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-database-datalakeprincipal.html
    pub struct DataLakePrincipal_ {
        pub data_lake_principal_identifier: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Database_DataLakePrincipal {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Database.DataLakePrincipal"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Database_DataLakePrincipal as DataLakePrincipal;
    impl crate::value::ToValue for DataLakePrincipal_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.data_lake_principal_identifier {
                properties.insert(
                    "DataLakePrincipalIdentifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-database-databaseidentifier.html
    pub struct DatabaseIdentifier_ {
        pub catalog_id: Option<crate::value::ExpString>,
        pub database_name: Option<crate::value::ExpString>,
        pub region: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Database_DatabaseIdentifier {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Database.DatabaseIdentifier"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Database_DatabaseIdentifier as DatabaseIdentifier;
    impl crate::value::ToValue for DatabaseIdentifier_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.catalog_id {
                properties.insert(
                    "CatalogId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.database_name {
                properties.insert(
                    "DatabaseName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.region {
                properties.insert("Region".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-database-databaseinput.html
    pub struct DatabaseInput_ {
        pub create_table_default_permissions: Option<Vec<PrincipalPrivileges_>>,
        pub description: Option<crate::value::ExpString>,
        pub federated_database: Option<Box<FederatedDatabase_>>,
        pub location_uri: Option<crate::value::ExpString>,
        pub name: Option<crate::value::ExpString>,
        pub parameters: Option<serde_json::Value>,
        pub target_database: Option<Box<DatabaseIdentifier_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Database_DatabaseInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Database.DatabaseInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Database_DatabaseInput as DatabaseInput;
    impl crate::value::ToValue for DatabaseInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.create_table_default_permissions {
                properties.insert(
                    "CreateTableDefaultPermissions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.federated_database {
                properties.insert(
                    "FederatedDatabase".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.location_uri {
                properties.insert(
                    "LocationUri".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.parameters {
                properties.insert(
                    "Parameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_database {
                properties.insert(
                    "TargetDatabase".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-database-federateddatabase.html
    pub struct FederatedDatabase_ {
        pub connection_name: Option<crate::value::ExpString>,
        pub identifier: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Database_FederatedDatabase {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Database.FederatedDatabase"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Database_FederatedDatabase as FederatedDatabase;
    impl crate::value::ToValue for FederatedDatabase_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.connection_name {
                properties.insert(
                    "ConnectionName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.identifier {
                properties.insert(
                    "Identifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-database-principalprivileges.html
    pub struct PrincipalPrivileges_ {
        pub permissions: Option<Vec<crate::value::ExpString>>,
        pub principal: Option<Box<DataLakePrincipal_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Database_PrincipalPrivileges {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Database.PrincipalPrivileges"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Database_PrincipalPrivileges as PrincipalPrivileges;
    impl crate::value::ToValue for PrincipalPrivileges_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.permissions {
                properties.insert(
                    "Permissions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.principal {
                properties.insert(
                    "Principal".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod job {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-job-connectionslist.html
    pub struct ConnectionsList_ {
        pub connections: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Job_ConnectionsList {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Job.ConnectionsList"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Job_ConnectionsList as ConnectionsList;
    impl crate::value::ToValue for ConnectionsList_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.connections {
                properties.insert(
                    "Connections".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-job-executionproperty.html
    pub struct ExecutionProperty_ {
        pub max_concurrent_runs: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Job_ExecutionProperty {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Job.ExecutionProperty"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Job_ExecutionProperty as ExecutionProperty;
    impl crate::value::ToValue for ExecutionProperty_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max_concurrent_runs {
                properties.insert(
                    "MaxConcurrentRuns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-job-jobcommand.html
    pub struct JobCommand_ {
        pub name: Option<crate::value::ExpString>,
        pub python_version: Option<crate::value::ExpString>,
        pub runtime: Option<crate::value::ExpString>,
        pub script_location: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Job_JobCommand {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Job.JobCommand"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Job_JobCommand as JobCommand;
    impl crate::value::ToValue for JobCommand_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.python_version {
                properties.insert(
                    "PythonVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.runtime {
                properties.insert(
                    "Runtime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.script_location {
                properties.insert(
                    "ScriptLocation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-job-notificationproperty.html
    pub struct NotificationProperty_ {
        pub notify_delay_after: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Job_NotificationProperty {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Job.NotificationProperty"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Job_NotificationProperty as NotificationProperty;
    impl crate::value::ToValue for NotificationProperty_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.notify_delay_after {
                properties.insert(
                    "NotifyDelayAfter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod mltransform {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-mltransform-transformparameters-findmatchesparameters.html
    pub struct FindMatchesParameters_ {
        pub accuracy_cost_tradeoff: Option<f64>,
        pub enforce_provided_labels: Option<crate::value::ExpBool>,
        pub precision_recall_tradeoff: Option<f64>,
        pub primary_key_column_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_MLTransform_FindMatchesParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::MLTransform.FindMatchesParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_MLTransform_FindMatchesParameters as FindMatchesParameters;
    impl crate::value::ToValue for FindMatchesParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.accuracy_cost_tradeoff {
                properties.insert(
                    "AccuracyCostTradeoff".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enforce_provided_labels {
                properties.insert(
                    "EnforceProvidedLabels".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.precision_recall_tradeoff {
                properties.insert(
                    "PrecisionRecallTradeoff".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "PrimaryKeyColumnName".to_string(),
                crate::value::ToValue::to_value(&self.primary_key_column_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-mltransform-inputrecordtables-gluetables.html
    pub struct GlueTables_ {
        pub catalog_id: Option<crate::value::ExpString>,
        pub connection_name: Option<crate::value::ExpString>,
        pub database_name: crate::value::ExpString,
        pub table_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_MLTransform_GlueTables {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::MLTransform.GlueTables"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_MLTransform_GlueTables as GlueTables;
    impl crate::value::ToValue for GlueTables_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.catalog_id {
                properties.insert(
                    "CatalogId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.connection_name {
                properties.insert(
                    "ConnectionName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DatabaseName".to_string(),
                crate::value::ToValue::to_value(&self.database_name),
            );
            properties.insert(
                "TableName".to_string(),
                crate::value::ToValue::to_value(&self.table_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-mltransform-inputrecordtables.html
    pub struct InputRecordTables_ {
        pub glue_tables: Option<Vec<GlueTables_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_MLTransform_InputRecordTables {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::MLTransform.InputRecordTables"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_MLTransform_InputRecordTables as InputRecordTables;
    impl crate::value::ToValue for InputRecordTables_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.glue_tables {
                properties.insert(
                    "GlueTables".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-mltransform-transformencryption-mluserdataencryption.html
    pub struct MLUserDataEncryption_ {
        pub kms_key_id: Option<crate::value::ExpString>,
        pub ml_user_data_encryption_mode: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_MLTransform_MLUserDataEncryption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::MLTransform.MLUserDataEncryption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_MLTransform_MLUserDataEncryption as MLUserDataEncryption;
    impl crate::value::ToValue for MLUserDataEncryption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MLUserDataEncryptionMode".to_string(),
                crate::value::ToValue::to_value(&self.ml_user_data_encryption_mode),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-mltransform-transformencryption.html
    pub struct TransformEncryption_ {
        pub ml_user_data_encryption: Option<Box<MLUserDataEncryption_>>,
        pub task_run_security_configuration_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_MLTransform_TransformEncryption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::MLTransform.TransformEncryption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_MLTransform_TransformEncryption as TransformEncryption;
    impl crate::value::ToValue for TransformEncryption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ml_user_data_encryption {
                properties.insert(
                    "MLUserDataEncryption".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.task_run_security_configuration_name {
                properties.insert(
                    "TaskRunSecurityConfigurationName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-mltransform-transformparameters.html
    pub struct TransformParameters_ {
        pub find_matches_parameters: Option<Box<FindMatchesParameters_>>,
        pub transform_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_MLTransform_TransformParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::MLTransform.TransformParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_MLTransform_TransformParameters as TransformParameters;
    impl crate::value::ToValue for TransformParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.find_matches_parameters {
                properties.insert(
                    "FindMatchesParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TransformType".to_string(),
                crate::value::ToValue::to_value(&self.transform_type),
            );
            properties.into()
        }
    }
}
pub mod partition {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-column.html
    pub struct Column_ {
        pub comment: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Partition_Column {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Partition.Column"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Partition_Column as Column;
    impl crate::value::ToValue for Column_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.comment {
                properties.insert(
                    "Comment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-order.html
    pub struct Order_ {
        pub column: crate::value::ExpString,
        pub sort_order: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Partition_Order {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Partition.Order"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Partition_Order as Order;
    impl crate::value::ToValue for Order_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Column".to_string(),
                crate::value::ToValue::to_value(&self.column),
            );
            if let Some(ref value) = self.sort_order {
                properties.insert(
                    "SortOrder".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-partitioninput.html
    pub struct PartitionInput_ {
        pub parameters: Option<serde_json::Value>,
        pub storage_descriptor: Option<Box<StorageDescriptor_>>,
        pub values: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Partition_PartitionInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Partition.PartitionInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Partition_PartitionInput as PartitionInput;
    impl crate::value::ToValue for PartitionInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.parameters {
                properties.insert(
                    "Parameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.storage_descriptor {
                properties.insert(
                    "StorageDescriptor".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Values".to_string(),
                crate::value::ToValue::to_value(&self.values),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-schemaid.html
    pub struct SchemaId_ {
        pub registry_name: Option<crate::value::ExpString>,
        pub schema_arn: Option<crate::value::ExpString>,
        pub schema_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Partition_SchemaId {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Partition.SchemaId"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Partition_SchemaId as SchemaId;
    impl crate::value::ToValue for SchemaId_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.registry_name {
                properties.insert(
                    "RegistryName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.schema_arn {
                properties.insert(
                    "SchemaArn".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-schemareference.html
    pub struct SchemaReference_ {
        pub schema_id: Option<Box<SchemaId_>>,
        pub schema_version_id: Option<crate::value::ExpString>,
        pub schema_version_number: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Partition_SchemaReference {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Partition.SchemaReference"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Partition_SchemaReference as SchemaReference;
    impl crate::value::ToValue for SchemaReference_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.schema_id {
                properties.insert(
                    "SchemaId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.schema_version_id {
                properties.insert(
                    "SchemaVersionId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.schema_version_number {
                properties.insert(
                    "SchemaVersionNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-serdeinfo.html
    pub struct SerdeInfo_ {
        pub name: Option<crate::value::ExpString>,
        pub parameters: Option<serde_json::Value>,
        pub serialization_library: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Partition_SerdeInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Partition.SerdeInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Partition_SerdeInfo as SerdeInfo;
    impl crate::value::ToValue for SerdeInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.parameters {
                properties.insert(
                    "Parameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.serialization_library {
                properties.insert(
                    "SerializationLibrary".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-skewedinfo.html
    pub struct SkewedInfo_ {
        pub skewed_column_names: Option<Vec<crate::value::ExpString>>,
        pub skewed_column_value_location_maps: Option<serde_json::Value>,
        pub skewed_column_values: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Partition_SkewedInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Partition.SkewedInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Partition_SkewedInfo as SkewedInfo;
    impl crate::value::ToValue for SkewedInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.skewed_column_names {
                properties.insert(
                    "SkewedColumnNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.skewed_column_value_location_maps {
                properties.insert(
                    "SkewedColumnValueLocationMaps".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.skewed_column_values {
                properties.insert(
                    "SkewedColumnValues".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-storagedescriptor.html
    pub struct StorageDescriptor_ {
        pub bucket_columns: Option<Vec<crate::value::ExpString>>,
        pub columns: Option<Vec<Column_>>,
        pub compressed: Option<crate::value::ExpBool>,
        pub input_format: Option<crate::value::ExpString>,
        pub location: Option<crate::value::ExpString>,
        pub number_of_buckets: Option<i32>,
        pub output_format: Option<crate::value::ExpString>,
        pub parameters: Option<serde_json::Value>,
        pub schema_reference: Option<Box<SchemaReference_>>,
        pub serde_info: Option<Box<SerdeInfo_>>,
        pub skewed_info: Option<Box<SkewedInfo_>>,
        pub sort_columns: Option<Vec<Order_>>,
        pub stored_as_sub_directories: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Partition_StorageDescriptor {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Partition.StorageDescriptor"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Partition_StorageDescriptor as StorageDescriptor;
    impl crate::value::ToValue for StorageDescriptor_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bucket_columns {
                properties.insert(
                    "BucketColumns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.columns {
                properties.insert(
                    "Columns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.compressed {
                properties.insert(
                    "Compressed".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_format {
                properties.insert(
                    "InputFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.location {
                properties.insert(
                    "Location".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.number_of_buckets {
                properties.insert(
                    "NumberOfBuckets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.output_format {
                properties.insert(
                    "OutputFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parameters {
                properties.insert(
                    "Parameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.schema_reference {
                properties.insert(
                    "SchemaReference".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.serde_info {
                properties.insert(
                    "SerdeInfo".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.skewed_info {
                properties.insert(
                    "SkewedInfo".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sort_columns {
                properties.insert(
                    "SortColumns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stored_as_sub_directories {
                properties.insert(
                    "StoredAsSubDirectories".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod schema {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-schema-registry.html
    pub struct Registry_ {
        pub arn: Option<crate::value::ExpString>,
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Schema_Registry {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Schema.Registry"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Schema_Registry as Registry;
    impl crate::value::ToValue for Registry_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.arn {
                properties.insert("Arn".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-schema-schemaversion.html
    pub struct SchemaVersion_ {
        pub is_latest: Option<crate::value::ExpBool>,
        pub version_number: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Schema_SchemaVersion {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Schema.SchemaVersion"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Schema_SchemaVersion as SchemaVersion;
    impl crate::value::ToValue for SchemaVersion_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.is_latest {
                properties.insert(
                    "IsLatest".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.version_number {
                properties.insert(
                    "VersionNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod schemaversion {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-schemaversion-schema.html
    pub struct Schema_ {
        pub registry_name: Option<crate::value::ExpString>,
        pub schema_arn: Option<crate::value::ExpString>,
        pub schema_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_SchemaVersion_Schema {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::SchemaVersion.Schema"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_SchemaVersion_Schema as Schema;
    impl crate::value::ToValue for Schema_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.registry_name {
                properties.insert(
                    "RegistryName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.schema_arn {
                properties.insert(
                    "SchemaArn".to_string(),
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
}
pub mod securityconfiguration {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-securityconfiguration-cloudwatchencryption.html
    pub struct CloudWatchEncryption_ {
        pub cloud_watch_encryption_mode: Option<crate::value::ExpString>,
        pub kms_key_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_SecurityConfiguration_CloudWatchEncryption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::SecurityConfiguration.CloudWatchEncryption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_SecurityConfiguration_CloudWatchEncryption as CloudWatchEncryption;
    impl crate::value::ToValue for CloudWatchEncryption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cloud_watch_encryption_mode {
                properties.insert(
                    "CloudWatchEncryptionMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kms_key_arn {
                properties.insert(
                    "KmsKeyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-securityconfiguration-encryptionconfiguration.html
    pub struct EncryptionConfiguration_ {
        pub cloud_watch_encryption: Option<Box<CloudWatchEncryption_>>,
        pub job_bookmarks_encryption: Option<Box<JobBookmarksEncryption_>>,
        pub s3_encryptions: Option<Box<S3Encryptions_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_SecurityConfiguration_EncryptionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::SecurityConfiguration.EncryptionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_SecurityConfiguration_EncryptionConfiguration as EncryptionConfiguration;
    impl crate::value::ToValue for EncryptionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cloud_watch_encryption {
                properties.insert(
                    "CloudWatchEncryption".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.job_bookmarks_encryption {
                properties.insert(
                    "JobBookmarksEncryption".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_encryptions {
                properties.insert(
                    "S3Encryptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-securityconfiguration-jobbookmarksencryption.html
    pub struct JobBookmarksEncryption_ {
        pub job_bookmarks_encryption_mode: Option<crate::value::ExpString>,
        pub kms_key_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_SecurityConfiguration_JobBookmarksEncryption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::SecurityConfiguration.JobBookmarksEncryption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_SecurityConfiguration_JobBookmarksEncryption as JobBookmarksEncryption;
    impl crate::value::ToValue for JobBookmarksEncryption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.job_bookmarks_encryption_mode {
                properties.insert(
                    "JobBookmarksEncryptionMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kms_key_arn {
                properties.insert(
                    "KmsKeyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-securityconfiguration-s3encryption.html
    pub struct S3Encryption_ {
        pub kms_key_arn: Option<crate::value::ExpString>,
        pub s3_encryption_mode: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_SecurityConfiguration_S3Encryption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::SecurityConfiguration.S3Encryption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_SecurityConfiguration_S3Encryption as S3Encryption;
    impl crate::value::ToValue for S3Encryption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_arn {
                properties.insert(
                    "KmsKeyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_encryption_mode {
                properties.insert(
                    "S3EncryptionMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-securityconfiguration-s3encryptions.html
    pub struct S3Encryptions_ {}
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_SecurityConfiguration_S3Encryptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::SecurityConfiguration.S3Encryptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_SecurityConfiguration_S3Encryptions as S3Encryptions;
    impl crate::value::ToValue for S3Encryptions_ {
        fn to_value(&self) -> serde_json::Value {
            serde_json::Value::Object(serde_json::Map::new())
        }
    }
}
pub mod table {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-column.html
    pub struct Column_ {
        pub comment: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Table_Column {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Table.Column"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Table_Column as Column;
    impl crate::value::ToValue for Column_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.comment {
                properties.insert(
                    "Comment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-iceberginput.html
    pub struct IcebergInput_ {
        pub metadata_operation: Option<Box<MetadataOperation_>>,
        pub version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Table_IcebergInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Table.IcebergInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Table_IcebergInput as IcebergInput;
    impl crate::value::ToValue for IcebergInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.metadata_operation {
                properties.insert(
                    "MetadataOperation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.version {
                properties.insert(
                    "Version".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-metadataoperation.html
    pub struct MetadataOperation_ {}
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Table_MetadataOperation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Table.MetadataOperation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Table_MetadataOperation as MetadataOperation;
    impl crate::value::ToValue for MetadataOperation_ {
        fn to_value(&self) -> serde_json::Value {
            serde_json::Value::Object(serde_json::Map::new())
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-opentableformatinput.html
    pub struct OpenTableFormatInput_ {
        pub iceberg_input: Option<Box<IcebergInput_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Table_OpenTableFormatInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Table.OpenTableFormatInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Table_OpenTableFormatInput as OpenTableFormatInput;
    impl crate::value::ToValue for OpenTableFormatInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.iceberg_input {
                properties.insert(
                    "IcebergInput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-order.html
    pub struct Order_ {
        pub column: crate::value::ExpString,
        pub sort_order: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Table_Order {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Table.Order"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Table_Order as Order;
    impl crate::value::ToValue for Order_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Column".to_string(),
                crate::value::ToValue::to_value(&self.column),
            );
            properties.insert(
                "SortOrder".to_string(),
                crate::value::ToValue::to_value(&self.sort_order),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-schemaid.html
    pub struct SchemaId_ {
        pub registry_name: Option<crate::value::ExpString>,
        pub schema_arn: Option<crate::value::ExpString>,
        pub schema_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Table_SchemaId {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Table.SchemaId"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Table_SchemaId as SchemaId;
    impl crate::value::ToValue for SchemaId_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.registry_name {
                properties.insert(
                    "RegistryName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.schema_arn {
                properties.insert(
                    "SchemaArn".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-schemareference.html
    pub struct SchemaReference_ {
        pub schema_id: Option<Box<SchemaId_>>,
        pub schema_version_id: Option<crate::value::ExpString>,
        pub schema_version_number: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Table_SchemaReference {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Table.SchemaReference"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Table_SchemaReference as SchemaReference;
    impl crate::value::ToValue for SchemaReference_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.schema_id {
                properties.insert(
                    "SchemaId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.schema_version_id {
                properties.insert(
                    "SchemaVersionId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.schema_version_number {
                properties.insert(
                    "SchemaVersionNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-serdeinfo.html
    pub struct SerdeInfo_ {
        pub name: Option<crate::value::ExpString>,
        pub parameters: Option<serde_json::Value>,
        pub serialization_library: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Table_SerdeInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Table.SerdeInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Table_SerdeInfo as SerdeInfo;
    impl crate::value::ToValue for SerdeInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.parameters {
                properties.insert(
                    "Parameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.serialization_library {
                properties.insert(
                    "SerializationLibrary".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-skewedinfo.html
    pub struct SkewedInfo_ {
        pub skewed_column_names: Option<Vec<crate::value::ExpString>>,
        pub skewed_column_value_location_maps: Option<serde_json::Value>,
        pub skewed_column_values: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Table_SkewedInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Table.SkewedInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Table_SkewedInfo as SkewedInfo;
    impl crate::value::ToValue for SkewedInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.skewed_column_names {
                properties.insert(
                    "SkewedColumnNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.skewed_column_value_location_maps {
                properties.insert(
                    "SkewedColumnValueLocationMaps".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.skewed_column_values {
                properties.insert(
                    "SkewedColumnValues".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-storagedescriptor.html
    pub struct StorageDescriptor_ {
        pub bucket_columns: Option<Vec<crate::value::ExpString>>,
        pub columns: Option<Vec<Column_>>,
        pub compressed: Option<crate::value::ExpBool>,
        pub input_format: Option<crate::value::ExpString>,
        pub location: Option<crate::value::ExpString>,
        pub number_of_buckets: Option<i32>,
        pub output_format: Option<crate::value::ExpString>,
        pub parameters: Option<serde_json::Value>,
        pub schema_reference: Option<Box<SchemaReference_>>,
        pub serde_info: Option<Box<SerdeInfo_>>,
        pub skewed_info: Option<Box<SkewedInfo_>>,
        pub sort_columns: Option<Vec<Order_>>,
        pub stored_as_sub_directories: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Table_StorageDescriptor {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Table.StorageDescriptor"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Table_StorageDescriptor as StorageDescriptor;
    impl crate::value::ToValue for StorageDescriptor_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bucket_columns {
                properties.insert(
                    "BucketColumns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.columns {
                properties.insert(
                    "Columns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.compressed {
                properties.insert(
                    "Compressed".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_format {
                properties.insert(
                    "InputFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.location {
                properties.insert(
                    "Location".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.number_of_buckets {
                properties.insert(
                    "NumberOfBuckets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.output_format {
                properties.insert(
                    "OutputFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parameters {
                properties.insert(
                    "Parameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.schema_reference {
                properties.insert(
                    "SchemaReference".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.serde_info {
                properties.insert(
                    "SerdeInfo".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.skewed_info {
                properties.insert(
                    "SkewedInfo".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sort_columns {
                properties.insert(
                    "SortColumns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stored_as_sub_directories {
                properties.insert(
                    "StoredAsSubDirectories".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-tableidentifier.html
    pub struct TableIdentifier_ {
        pub catalog_id: Option<crate::value::ExpString>,
        pub database_name: Option<crate::value::ExpString>,
        pub name: Option<crate::value::ExpString>,
        pub region: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Table_TableIdentifier {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Table.TableIdentifier"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Table_TableIdentifier as TableIdentifier;
    impl crate::value::ToValue for TableIdentifier_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.catalog_id {
                properties.insert(
                    "CatalogId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.database_name {
                properties.insert(
                    "DatabaseName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.region {
                properties.insert("Region".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-tableinput.html
    pub struct TableInput_ {
        pub description: Option<crate::value::ExpString>,
        pub name: Option<crate::value::ExpString>,
        pub owner: Option<crate::value::ExpString>,
        pub parameters: Option<serde_json::Value>,
        pub partition_keys: Option<Vec<Column_>>,
        pub retention: Option<i32>,
        pub storage_descriptor: Option<Box<StorageDescriptor_>>,
        pub table_type: Option<crate::value::ExpString>,
        pub target_table: Option<Box<TableIdentifier_>>,
        pub view_expanded_text: Option<crate::value::ExpString>,
        pub view_original_text: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Table_TableInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Table.TableInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Table_TableInput as TableInput;
    impl crate::value::ToValue for TableInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.owner {
                properties.insert("Owner".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.parameters {
                properties.insert(
                    "Parameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.partition_keys {
                properties.insert(
                    "PartitionKeys".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.retention {
                properties.insert(
                    "Retention".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.storage_descriptor {
                properties.insert(
                    "StorageDescriptor".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.table_type {
                properties.insert(
                    "TableType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_table {
                properties.insert(
                    "TargetTable".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.view_expanded_text {
                properties.insert(
                    "ViewExpandedText".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.view_original_text {
                properties.insert(
                    "ViewOriginalText".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod tableoptimizer {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-tableoptimizer-tableoptimizerconfiguration-orphanfiledeletionconfiguration-icebergconfiguration.html
    pub struct IcebergConfiguration_ {
        pub location: Option<crate::value::ExpString>,
        pub orphan_file_retention_period_in_days: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_TableOptimizer_IcebergConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::TableOptimizer.IcebergConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_TableOptimizer_IcebergConfiguration as IcebergConfiguration;
    impl crate::value::ToValue for IcebergConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.location {
                properties.insert(
                    "Location".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.orphan_file_retention_period_in_days {
                properties.insert(
                    "OrphanFileRetentionPeriodInDays".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-tableoptimizer-icebergretentionconfiguration.html
    pub struct IcebergRetentionConfiguration_ {
        pub clean_expired_files: Option<crate::value::ExpBool>,
        pub number_of_snapshots_to_retain: Option<i32>,
        pub snapshot_retention_period_in_days: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_TableOptimizer_IcebergRetentionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::TableOptimizer.IcebergRetentionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_TableOptimizer_IcebergRetentionConfiguration as IcebergRetentionConfiguration;
    impl crate::value::ToValue for IcebergRetentionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.clean_expired_files {
                properties.insert(
                    "CleanExpiredFiles".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.number_of_snapshots_to_retain {
                properties.insert(
                    "NumberOfSnapshotsToRetain".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.snapshot_retention_period_in_days {
                properties.insert(
                    "SnapshotRetentionPeriodInDays".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-tableoptimizer-tableoptimizerconfiguration-orphanfiledeletionconfiguration.html
    pub struct OrphanFileDeletionConfiguration_ {
        pub iceberg_configuration: Option<Box<IcebergConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_TableOptimizer_OrphanFileDeletionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::TableOptimizer.OrphanFileDeletionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_TableOptimizer_OrphanFileDeletionConfiguration as OrphanFileDeletionConfiguration;
    impl crate::value::ToValue for OrphanFileDeletionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.iceberg_configuration {
                properties.insert(
                    "IcebergConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-tableoptimizer-tableoptimizerconfiguration-retentionconfiguration.html
    pub struct RetentionConfiguration_ {
        pub iceberg_configuration: Option<Box<IcebergRetentionConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_TableOptimizer_RetentionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::TableOptimizer.RetentionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_TableOptimizer_RetentionConfiguration as RetentionConfiguration;
    impl crate::value::ToValue for RetentionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.iceberg_configuration {
                properties.insert(
                    "IcebergConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-tableoptimizer-tableoptimizerconfiguration.html
    pub struct TableOptimizerConfiguration_ {
        pub enabled: crate::value::ExpBool,
        pub orphan_file_deletion_configuration: Option<Box<OrphanFileDeletionConfiguration_>>,
        pub retention_configuration: Option<Box<RetentionConfiguration_>>,
        pub role_arn: crate::value::ExpString,
        pub vpc_configuration: Option<Box<VpcConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_TableOptimizer_TableOptimizerConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::TableOptimizer.TableOptimizerConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_TableOptimizer_TableOptimizerConfiguration as TableOptimizerConfiguration;
    impl crate::value::ToValue for TableOptimizerConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            if let Some(ref value) = self.orphan_file_deletion_configuration {
                properties.insert(
                    "OrphanFileDeletionConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.retention_configuration {
                properties.insert(
                    "RetentionConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            if let Some(ref value) = self.vpc_configuration {
                properties.insert(
                    "VpcConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-tableoptimizer-tableoptimizerconfiguration-vpcconfiguration.html
    pub struct VpcConfiguration_ {
        pub glue_connection_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_TableOptimizer_VpcConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::TableOptimizer.VpcConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_TableOptimizer_VpcConfiguration as VpcConfiguration;
    impl crate::value::ToValue for VpcConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.glue_connection_name {
                properties.insert(
                    "GlueConnectionName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod trigger {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-action.html
    pub struct Action_ {
        pub arguments: Option<serde_json::Value>,
        pub crawler_name: Option<crate::value::ExpString>,
        pub job_name: Option<crate::value::ExpString>,
        pub notification_property: Option<Box<NotificationProperty_>>,
        pub security_configuration: Option<crate::value::ExpString>,
        pub timeout: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Trigger_Action {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Trigger.Action"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Trigger_Action as Action;
    impl crate::value::ToValue for Action_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.arguments {
                properties.insert(
                    "Arguments".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.crawler_name {
                properties.insert(
                    "CrawlerName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.job_name {
                properties.insert(
                    "JobName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.notification_property {
                properties.insert(
                    "NotificationProperty".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.security_configuration {
                properties.insert(
                    "SecurityConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timeout {
                properties.insert(
                    "Timeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-condition.html
    pub struct Condition_ {
        pub crawl_state: Option<crate::value::ExpString>,
        pub crawler_name: Option<crate::value::ExpString>,
        pub job_name: Option<crate::value::ExpString>,
        pub logical_operator: Option<crate::value::ExpString>,
        pub state: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Trigger_Condition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Trigger.Condition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Trigger_Condition as Condition;
    impl crate::value::ToValue for Condition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.crawl_state {
                properties.insert(
                    "CrawlState".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.crawler_name {
                properties.insert(
                    "CrawlerName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.job_name {
                properties.insert(
                    "JobName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.logical_operator {
                properties.insert(
                    "LogicalOperator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.state {
                properties.insert("State".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-eventbatchingcondition.html
    pub struct EventBatchingCondition_ {
        pub batch_size: i32,
        pub batch_window: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Trigger_EventBatchingCondition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Trigger.EventBatchingCondition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Trigger_EventBatchingCondition as EventBatchingCondition;
    impl crate::value::ToValue for EventBatchingCondition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BatchSize".to_string(),
                crate::value::ToValue::to_value(&self.batch_size),
            );
            if let Some(ref value) = self.batch_window {
                properties.insert(
                    "BatchWindow".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-notificationproperty.html
    pub struct NotificationProperty_ {
        pub notify_delay_after: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Trigger_NotificationProperty {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Trigger.NotificationProperty"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Trigger_NotificationProperty as NotificationProperty;
    impl crate::value::ToValue for NotificationProperty_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.notify_delay_after {
                properties.insert(
                    "NotifyDelayAfter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-predicate.html
    pub struct Predicate_ {
        pub conditions: Option<Vec<Condition_>>,
        pub logical: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_Trigger_Predicate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::Trigger.Predicate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_Trigger_Predicate as Predicate;
    impl crate::value::ToValue for Predicate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.conditions {
                properties.insert(
                    "Conditions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.logical {
                properties.insert(
                    "Logical".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod usageprofile {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-usageprofile-configurationobject.html
    pub struct ConfigurationObject_ {
        pub allowed_values: Option<Vec<crate::value::ExpString>>,
        pub default_value: Option<crate::value::ExpString>,
        pub max_value: Option<crate::value::ExpString>,
        pub min_value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_UsageProfile_ConfigurationObject {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::UsageProfile.ConfigurationObject"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_UsageProfile_ConfigurationObject as ConfigurationObject;
    impl crate::value::ToValue for ConfigurationObject_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allowed_values {
                properties.insert(
                    "AllowedValues".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.default_value {
                properties.insert(
                    "DefaultValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
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
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-usageprofile-profileconfiguration.html
    pub struct ProfileConfiguration_ {
        pub job_configuration: Option<std::collections::BTreeMap<String, ConfigurationObject_>>,
        pub session_configuration: Option<std::collections::BTreeMap<String, ConfigurationObject_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_glue_UsageProfile_ProfileConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Glue::UsageProfile.ProfileConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_glue_UsageProfile_ProfileConfiguration as ProfileConfiguration;
    impl crate::value::ToValue for ProfileConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.job_configuration {
                properties.insert(
                    "JobConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.session_configuration {
                properties.insert(
                    "SessionConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-classifier.html
pub struct Classifier_ {
    pub csv_classifier: Option<super::glue::classifier::CsvClassifier_>,
    pub grok_classifier: Option<super::glue::classifier::GrokClassifier_>,
    pub json_classifier: Option<super::glue::classifier::JsonClassifier_>,
    pub xml_classifier: Option<super::glue::classifier::XMLClassifier_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_glue_Classifier {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Glue::Classifier"
        $($field $value)*)
    };
}
pub use crate::__aws_glue_Classifier as Classifier;
impl crate::template::ToResource for Classifier_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Glue"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Classifier"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.csv_classifier {
            properties.insert(
                "CsvClassifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.grok_classifier {
            properties.insert(
                "GrokClassifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.json_classifier {
            properties.insert(
                "JsonClassifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.xml_classifier {
            properties.insert(
                "XMLClassifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-connection.html
pub struct Connection_ {
    pub catalog_id: crate::value::ExpString,
    pub connection_input: super::glue::connection::ConnectionInput_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_glue_Connection {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Glue::Connection"
        $($field $value)*)
    };
}
pub use crate::__aws_glue_Connection as Connection;
impl crate::template::ToResource for Connection_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Glue"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Connection"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "CatalogId".to_string(),
            crate::value::ToValue::to_value(&self.catalog_id),
        );
        properties.insert(
            "ConnectionInput".to_string(),
            crate::value::ToValue::to_value(&self.connection_input),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-crawler.html
pub struct Crawler_ {
    pub classifiers: Option<Vec<crate::value::ExpString>>,
    pub configuration: Option<crate::value::ExpString>,
    pub crawler_security_configuration: Option<crate::value::ExpString>,
    pub database_name: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub lake_formation_configuration: Option<super::glue::crawler::LakeFormationConfiguration_>,
    pub name: Option<crate::value::ExpString>,
    pub recrawl_policy: Option<super::glue::crawler::RecrawlPolicy_>,
    pub role: crate::value::ExpString,
    pub schedule: Option<super::glue::crawler::Schedule_>,
    pub schema_change_policy: Option<super::glue::crawler::SchemaChangePolicy_>,
    pub table_prefix: Option<crate::value::ExpString>,
    pub tags: Option<serde_json::Value>,
    pub targets: super::glue::crawler::Targets_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_glue_Crawler {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Glue::Crawler" $($field
        $value)*)
    };
}
pub use crate::__aws_glue_Crawler as Crawler;
impl crate::template::ToResource for Crawler_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Glue"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Crawler"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.classifiers {
            properties.insert(
                "Classifiers".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.configuration {
            properties.insert(
                "Configuration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.crawler_security_configuration {
            properties.insert(
                "CrawlerSecurityConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.database_name {
            properties.insert(
                "DatabaseName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.lake_formation_configuration {
            properties.insert(
                "LakeFormationConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.recrawl_policy {
            properties.insert(
                "RecrawlPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Role".to_string(),
            crate::value::ToValue::to_value(&self.role),
        );
        if let Some(ref value) = self.schedule {
            properties.insert(
                "Schedule".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.schema_change_policy {
            properties.insert(
                "SchemaChangePolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.table_prefix {
            properties.insert(
                "TablePrefix".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Targets".to_string(),
            crate::value::ToValue::to_value(&self.targets),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-customentitytype.html
pub struct CustomEntityType_ {
    pub context_words: Option<Vec<crate::value::ExpString>>,
    pub name: Option<crate::value::ExpString>,
    pub regex_string: Option<crate::value::ExpString>,
    pub tags: Option<serde_json::Value>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_glue_CustomEntityType {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Glue::CustomEntityType"
        $($field $value)*)
    };
}
pub use crate::__aws_glue_CustomEntityType as CustomEntityType;
impl crate::template::ToResource for CustomEntityType_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Glue"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("CustomEntityType"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.context_words {
            properties.insert(
                "ContextWords".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.regex_string {
            properties.insert(
                "RegexString".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-datacatalogencryptionsettings.html
pub struct DataCatalogEncryptionSettings_ {
    pub catalog_id: crate::value::ExpString,
    pub data_catalog_encryption_settings:
        super::glue::datacatalogencryptionsettings::DataCatalogEncryptionSettings_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_glue_DataCatalogEncryptionSettings {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Glue::DataCatalogEncryptionSettings"
        $($field $value)*)
    };
}
pub use crate::__aws_glue_DataCatalogEncryptionSettings as DataCatalogEncryptionSettings;
impl crate::template::ToResource for DataCatalogEncryptionSettings_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Glue"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "DataCatalogEncryptionSettings",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "CatalogId".to_string(),
            crate::value::ToValue::to_value(&self.catalog_id),
        );
        properties.insert(
            "DataCatalogEncryptionSettings".to_string(),
            crate::value::ToValue::to_value(&self.data_catalog_encryption_settings),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-dataqualityruleset.html
pub struct DataQualityRuleset_ {
    pub client_token: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub ruleset: Option<crate::value::ExpString>,
    pub tags: Option<serde_json::Value>,
    pub target_table: Option<super::glue::dataqualityruleset::DataQualityTargetTable_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_glue_DataQualityRuleset {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Glue::DataQualityRuleset"
        $($field $value)*)
    };
}
pub use crate::__aws_glue_DataQualityRuleset as DataQualityRuleset;
impl crate::template::ToResource for DataQualityRuleset_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Glue"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DataQualityRuleset"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.client_token {
            properties.insert(
                "ClientToken".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.ruleset {
            properties.insert(
                "Ruleset".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.target_table {
            properties.insert(
                "TargetTable".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-database.html
pub struct Database_ {
    pub catalog_id: crate::value::ExpString,
    pub database_input: super::glue::database::DatabaseInput_,
    pub database_name: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_glue_Database {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Glue::Database" $($field
        $value)*)
    };
}
pub use crate::__aws_glue_Database as Database;
impl crate::template::ToResource for Database_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Glue"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Database"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "CatalogId".to_string(),
            crate::value::ToValue::to_value(&self.catalog_id),
        );
        properties.insert(
            "DatabaseInput".to_string(),
            crate::value::ToValue::to_value(&self.database_input),
        );
        if let Some(ref value) = self.database_name {
            properties.insert(
                "DatabaseName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-devendpoint.html
pub struct DevEndpoint_ {
    pub arguments: Option<serde_json::Value>,
    pub endpoint_name: Option<crate::value::ExpString>,
    pub extra_jars_s3_path: Option<crate::value::ExpString>,
    pub extra_python_libs_s3_path: Option<crate::value::ExpString>,
    pub glue_version: Option<crate::value::ExpString>,
    pub number_of_nodes: Option<i32>,
    pub number_of_workers: Option<i32>,
    pub public_key: Option<crate::value::ExpString>,
    pub public_keys: Option<Vec<crate::value::ExpString>>,
    pub role_arn: crate::value::ExpString,
    pub security_configuration: Option<crate::value::ExpString>,
    pub security_group_ids: Option<Vec<crate::value::ExpString>>,
    pub subnet_id: Option<crate::value::ExpString>,
    pub tags: Option<serde_json::Value>,
    pub worker_type: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_glue_DevEndpoint {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Glue::DevEndpoint"
        $($field $value)*)
    };
}
pub use crate::__aws_glue_DevEndpoint as DevEndpoint;
impl crate::template::ToResource for DevEndpoint_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Glue"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DevEndpoint"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.arguments {
            properties.insert(
                "Arguments".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.endpoint_name {
            properties.insert(
                "EndpointName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.extra_jars_s3_path {
            properties.insert(
                "ExtraJarsS3Path".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.extra_python_libs_s3_path {
            properties.insert(
                "ExtraPythonLibsS3Path".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.glue_version {
            properties.insert(
                "GlueVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.number_of_nodes {
            properties.insert(
                "NumberOfNodes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.number_of_workers {
            properties.insert(
                "NumberOfWorkers".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.public_key {
            properties.insert(
                "PublicKey".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.public_keys {
            properties.insert(
                "PublicKeys".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        if let Some(ref value) = self.security_configuration {
            properties.insert(
                "SecurityConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.security_group_ids {
            properties.insert(
                "SecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.subnet_id {
            properties.insert(
                "SubnetId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.worker_type {
            properties.insert(
                "WorkerType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-job.html
pub struct Job_ {
    pub allocated_capacity: Option<f64>,
    pub command: super::glue::job::JobCommand_,
    pub connections: Option<super::glue::job::ConnectionsList_>,
    pub default_arguments: Option<serde_json::Value>,
    pub description: Option<crate::value::ExpString>,
    pub execution_class: Option<crate::value::ExpString>,
    pub execution_property: Option<super::glue::job::ExecutionProperty_>,
    pub glue_version: Option<crate::value::ExpString>,
    pub job_mode: Option<crate::value::ExpString>,
    pub job_run_queuing_enabled: Option<crate::value::ExpBool>,
    pub log_uri: Option<crate::value::ExpString>,
    pub maintenance_window: Option<crate::value::ExpString>,
    pub max_capacity: Option<f64>,
    pub max_retries: Option<f64>,
    pub name: Option<crate::value::ExpString>,
    pub non_overridable_arguments: Option<serde_json::Value>,
    pub notification_property: Option<super::glue::job::NotificationProperty_>,
    pub number_of_workers: Option<i32>,
    pub role: crate::value::ExpString,
    pub security_configuration: Option<crate::value::ExpString>,
    pub tags: Option<serde_json::Value>,
    pub timeout: Option<i32>,
    pub worker_type: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_glue_Job {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Glue::Job" $($field
        $value)*)
    };
}
pub use crate::__aws_glue_Job as Job;
impl crate::template::ToResource for Job_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Glue"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Job"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.allocated_capacity {
            properties.insert(
                "AllocatedCapacity".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Command".to_string(),
            crate::value::ToValue::to_value(&self.command),
        );
        if let Some(ref value) = self.connections {
            properties.insert(
                "Connections".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.default_arguments {
            properties.insert(
                "DefaultArguments".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.execution_class {
            properties.insert(
                "ExecutionClass".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.execution_property {
            properties.insert(
                "ExecutionProperty".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.glue_version {
            properties.insert(
                "GlueVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.job_mode {
            properties.insert(
                "JobMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.job_run_queuing_enabled {
            properties.insert(
                "JobRunQueuingEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.log_uri {
            properties.insert("LogUri".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.maintenance_window {
            properties.insert(
                "MaintenanceWindow".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.max_capacity {
            properties.insert(
                "MaxCapacity".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.max_retries {
            properties.insert(
                "MaxRetries".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.non_overridable_arguments {
            properties.insert(
                "NonOverridableArguments".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.notification_property {
            properties.insert(
                "NotificationProperty".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.number_of_workers {
            properties.insert(
                "NumberOfWorkers".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Role".to_string(),
            crate::value::ToValue::to_value(&self.role),
        );
        if let Some(ref value) = self.security_configuration {
            properties.insert(
                "SecurityConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.timeout {
            properties.insert(
                "Timeout".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.worker_type {
            properties.insert(
                "WorkerType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-mltransform.html
pub struct MLTransform_ {
    pub description: Option<crate::value::ExpString>,
    pub glue_version: Option<crate::value::ExpString>,
    pub input_record_tables: super::glue::mltransform::InputRecordTables_,
    pub max_capacity: Option<f64>,
    pub max_retries: Option<i32>,
    pub name: Option<crate::value::ExpString>,
    pub number_of_workers: Option<i32>,
    pub role: crate::value::ExpString,
    pub tags: Option<serde_json::Value>,
    pub timeout: Option<i32>,
    pub transform_encryption: Option<super::glue::mltransform::TransformEncryption_>,
    pub transform_parameters: super::glue::mltransform::TransformParameters_,
    pub worker_type: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_glue_MLTransform {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Glue::MLTransform"
        $($field $value)*)
    };
}
pub use crate::__aws_glue_MLTransform as MLTransform;
impl crate::template::ToResource for MLTransform_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Glue"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("MLTransform"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.glue_version {
            properties.insert(
                "GlueVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "InputRecordTables".to_string(),
            crate::value::ToValue::to_value(&self.input_record_tables),
        );
        if let Some(ref value) = self.max_capacity {
            properties.insert(
                "MaxCapacity".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.max_retries {
            properties.insert(
                "MaxRetries".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.number_of_workers {
            properties.insert(
                "NumberOfWorkers".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Role".to_string(),
            crate::value::ToValue::to_value(&self.role),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.timeout {
            properties.insert(
                "Timeout".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.transform_encryption {
            properties.insert(
                "TransformEncryption".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "TransformParameters".to_string(),
            crate::value::ToValue::to_value(&self.transform_parameters),
        );
        if let Some(ref value) = self.worker_type {
            properties.insert(
                "WorkerType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-partition.html
pub struct Partition_ {
    pub catalog_id: crate::value::ExpString,
    pub database_name: crate::value::ExpString,
    pub partition_input: super::glue::partition::PartitionInput_,
    pub table_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_glue_Partition {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Glue::Partition" $($field
        $value)*)
    };
}
pub use crate::__aws_glue_Partition as Partition;
impl crate::template::ToResource for Partition_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Glue"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Partition"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "CatalogId".to_string(),
            crate::value::ToValue::to_value(&self.catalog_id),
        );
        properties.insert(
            "DatabaseName".to_string(),
            crate::value::ToValue::to_value(&self.database_name),
        );
        properties.insert(
            "PartitionInput".to_string(),
            crate::value::ToValue::to_value(&self.partition_input),
        );
        properties.insert(
            "TableName".to_string(),
            crate::value::ToValue::to_value(&self.table_name),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-registry.html
pub struct Registry_ {
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_glue_Registry {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Glue::Registry" $($field
        $value)*)
    };
}
pub use crate::__aws_glue_Registry as Registry;
impl crate::template::ToResource for Registry_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Glue"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Registry"),
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
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-schema.html
pub struct Schema_ {
    pub checkpoint_version: Option<super::glue::schema::SchemaVersion_>,
    pub compatibility: crate::value::ExpString,
    pub data_format: crate::value::ExpString,
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub registry: Option<super::glue::schema::Registry_>,
    pub schema_definition: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_glue_Schema {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Glue::Schema" $($field
        $value)*)
    };
}
pub use crate::__aws_glue_Schema as Schema;
impl crate::template::ToResource for Schema_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Glue"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Schema"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.checkpoint_version {
            properties.insert(
                "CheckpointVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Compatibility".to_string(),
            crate::value::ToValue::to_value(&self.compatibility),
        );
        properties.insert(
            "DataFormat".to_string(),
            crate::value::ToValue::to_value(&self.data_format),
        );
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
        if let Some(ref value) = self.registry {
            properties.insert(
                "Registry".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.schema_definition {
            properties.insert(
                "SchemaDefinition".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-schemaversion.html
pub struct SchemaVersion_ {
    pub schema: super::glue::schemaversion::Schema_,
    pub schema_definition: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_glue_SchemaVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Glue::SchemaVersion"
        $($field $value)*)
    };
}
pub use crate::__aws_glue_SchemaVersion as SchemaVersion;
impl crate::template::ToResource for SchemaVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Glue"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SchemaVersion"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Schema".to_string(),
            crate::value::ToValue::to_value(&self.schema),
        );
        properties.insert(
            "SchemaDefinition".to_string(),
            crate::value::ToValue::to_value(&self.schema_definition),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-schemaversionmetadata.html
pub struct SchemaVersionMetadata_ {
    pub key: crate::value::ExpString,
    pub schema_version_id: crate::value::ExpString,
    pub value: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_glue_SchemaVersionMetadata {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Glue::SchemaVersionMetadata"
        $($field $value)*)
    };
}
pub use crate::__aws_glue_SchemaVersionMetadata as SchemaVersionMetadata;
impl crate::template::ToResource for SchemaVersionMetadata_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Glue"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SchemaVersionMetadata"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Key".to_string(),
            crate::value::ToValue::to_value(&self.key),
        );
        properties.insert(
            "SchemaVersionId".to_string(),
            crate::value::ToValue::to_value(&self.schema_version_id),
        );
        properties.insert(
            "Value".to_string(),
            crate::value::ToValue::to_value(&self.value),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-securityconfiguration.html
pub struct SecurityConfiguration_ {
    pub encryption_configuration: super::glue::securityconfiguration::EncryptionConfiguration_,
    pub name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_glue_SecurityConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Glue::SecurityConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_glue_SecurityConfiguration as SecurityConfiguration;
impl crate::template::ToResource for SecurityConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Glue"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SecurityConfiguration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "EncryptionConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.encryption_configuration),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-table.html
pub struct Table_ {
    pub catalog_id: crate::value::ExpString,
    pub database_name: crate::value::ExpString,
    pub open_table_format_input: Option<super::glue::table::OpenTableFormatInput_>,
    pub table_input: super::glue::table::TableInput_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_glue_Table {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Glue::Table" $($field
        $value)*)
    };
}
pub use crate::__aws_glue_Table as Table;
impl crate::template::ToResource for Table_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Glue"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Table"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "CatalogId".to_string(),
            crate::value::ToValue::to_value(&self.catalog_id),
        );
        properties.insert(
            "DatabaseName".to_string(),
            crate::value::ToValue::to_value(&self.database_name),
        );
        if let Some(ref value) = self.open_table_format_input {
            properties.insert(
                "OpenTableFormatInput".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "TableInput".to_string(),
            crate::value::ToValue::to_value(&self.table_input),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-tableoptimizer.html
pub struct TableOptimizer_ {
    pub catalog_id: crate::value::ExpString,
    pub database_name: crate::value::ExpString,
    pub table_name: crate::value::ExpString,
    pub table_optimizer_configuration: super::glue::tableoptimizer::TableOptimizerConfiguration_,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_glue_TableOptimizer {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Glue::TableOptimizer"
        $($field $value)*)
    };
}
pub use crate::__aws_glue_TableOptimizer as TableOptimizer;
impl crate::template::ToResource for TableOptimizer_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Glue"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TableOptimizer"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "CatalogId".to_string(),
            crate::value::ToValue::to_value(&self.catalog_id),
        );
        properties.insert(
            "DatabaseName".to_string(),
            crate::value::ToValue::to_value(&self.database_name),
        );
        properties.insert(
            "TableName".to_string(),
            crate::value::ToValue::to_value(&self.table_name),
        );
        properties.insert(
            "TableOptimizerConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.table_optimizer_configuration),
        );
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-trigger.html
pub struct Trigger_ {
    pub actions: Vec<super::glue::trigger::Action_>,
    pub description: Option<crate::value::ExpString>,
    pub event_batching_condition: Option<super::glue::trigger::EventBatchingCondition_>,
    pub name: Option<crate::value::ExpString>,
    pub predicate: Option<super::glue::trigger::Predicate_>,
    pub schedule: Option<crate::value::ExpString>,
    pub start_on_creation: Option<crate::value::ExpBool>,
    pub tags: Option<serde_json::Value>,
    pub r#type: crate::value::ExpString,
    pub workflow_name: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_glue_Trigger {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Glue::Trigger" $($field
        $value)*)
    };
}
pub use crate::__aws_glue_Trigger as Trigger;
impl crate::template::ToResource for Trigger_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Glue"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Trigger"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Actions".to_string(),
            crate::value::ToValue::to_value(&self.actions),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.event_batching_condition {
            properties.insert(
                "EventBatchingCondition".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.predicate {
            properties.insert(
                "Predicate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.schedule {
            properties.insert(
                "Schedule".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.start_on_creation {
            properties.insert(
                "StartOnCreation".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        if let Some(ref value) = self.workflow_name {
            properties.insert(
                "WorkflowName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-usageprofile.html
pub struct UsageProfile_ {
    pub configuration: Option<super::glue::usageprofile::ProfileConfiguration_>,
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_glue_UsageProfile {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Glue::UsageProfile"
        $($field $value)*)
    };
}
pub use crate::__aws_glue_UsageProfile as UsageProfile;
impl crate::template::ToResource for UsageProfile_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Glue"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("UsageProfile"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.configuration {
            properties.insert(
                "Configuration".to_string(),
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
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-workflow.html
pub struct Workflow_ {
    pub default_run_properties: Option<serde_json::Value>,
    pub description: Option<crate::value::ExpString>,
    pub max_concurrent_runs: Option<i32>,
    pub name: Option<crate::value::ExpString>,
    pub tags: Option<serde_json::Value>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_glue_Workflow {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Glue::Workflow" $($field
        $value)*)
    };
}
pub use crate::__aws_glue_Workflow as Workflow;
impl crate::template::ToResource for Workflow_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Glue"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Workflow"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.default_run_properties {
            properties.insert(
                "DefaultRunProperties".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.max_concurrent_runs {
            properties.insert(
                "MaxConcurrentRuns".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
