pub mod listener {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-action.html>
    pub struct Action_ {
        pub authenticate_cognito_config: Option<Box<AuthenticateCognitoConfig_>>,
        pub authenticate_oidc_config: Option<Box<AuthenticateOidcConfig_>>,
        pub fixed_response_config: Option<Box<FixedResponseConfig_>>,
        pub forward_config: Option<Box<ForwardConfig_>>,
        pub jwt_validation_config: Option<Box<JwtValidationConfig_>>,
        pub order: Option<i32>,
        pub redirect_config: Option<Box<RedirectConfig_>>,
        pub target_group_arn: Option<crate::value::ExpString>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_Listener_Action {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::Listener.Action"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_Listener_Action as Action;
    impl crate::value::ToValue for Action_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.authenticate_cognito_config {
                properties.insert(
                    "AuthenticateCognitoConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.authenticate_oidc_config {
                properties.insert(
                    "AuthenticateOidcConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.fixed_response_config {
                properties.insert(
                    "FixedResponseConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.forward_config {
                properties.insert(
                    "ForwardConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.jwt_validation_config {
                properties.insert(
                    "JwtValidationConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.order {
                properties.insert("Order".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.redirect_config {
                properties.insert(
                    "RedirectConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_group_arn {
                properties.insert(
                    "TargetGroupArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-authenticatecognitoconfig.html>
    pub struct AuthenticateCognitoConfig_ {
        pub authentication_request_extra_params:
            Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub on_unauthenticated_request: Option<crate::value::ExpString>,
        pub scope: Option<crate::value::ExpString>,
        pub session_cookie_name: Option<crate::value::ExpString>,
        pub session_timeout: Option<crate::value::ExpString>,
        pub user_pool_arn: crate::value::ExpString,
        pub user_pool_client_id: crate::value::ExpString,
        pub user_pool_domain: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_Listener_AuthenticateCognitoConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::Listener.AuthenticateCognitoConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_Listener_AuthenticateCognitoConfig as AuthenticateCognitoConfig;
    impl crate::value::ToValue for AuthenticateCognitoConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.authentication_request_extra_params {
                properties.insert(
                    "AuthenticationRequestExtraParams".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.on_unauthenticated_request {
                properties.insert(
                    "OnUnauthenticatedRequest".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scope {
                properties.insert("Scope".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.session_cookie_name {
                properties.insert(
                    "SessionCookieName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.session_timeout {
                properties.insert(
                    "SessionTimeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "UserPoolArn".to_string(),
                crate::value::ToValue::to_value(&self.user_pool_arn),
            );
            properties.insert(
                "UserPoolClientId".to_string(),
                crate::value::ToValue::to_value(&self.user_pool_client_id),
            );
            properties.insert(
                "UserPoolDomain".to_string(),
                crate::value::ToValue::to_value(&self.user_pool_domain),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-authenticateoidcconfig.html>
    pub struct AuthenticateOidcConfig_ {
        pub authentication_request_extra_params:
            Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub authorization_endpoint: crate::value::ExpString,
        pub client_id: crate::value::ExpString,
        pub client_secret: Option<crate::value::ExpString>,
        pub issuer: crate::value::ExpString,
        pub on_unauthenticated_request: Option<crate::value::ExpString>,
        pub scope: Option<crate::value::ExpString>,
        pub session_cookie_name: Option<crate::value::ExpString>,
        pub session_timeout: Option<crate::value::ExpString>,
        pub token_endpoint: crate::value::ExpString,
        pub use_existing_client_secret: Option<crate::value::ExpBool>,
        pub user_info_endpoint: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_Listener_AuthenticateOidcConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::Listener.AuthenticateOidcConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_Listener_AuthenticateOidcConfig as AuthenticateOidcConfig;
    impl crate::value::ToValue for AuthenticateOidcConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.authentication_request_extra_params {
                properties.insert(
                    "AuthenticationRequestExtraParams".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "AuthorizationEndpoint".to_string(),
                crate::value::ToValue::to_value(&self.authorization_endpoint),
            );
            properties.insert(
                "ClientId".to_string(),
                crate::value::ToValue::to_value(&self.client_id),
            );
            if let Some(ref value) = self.client_secret {
                properties.insert(
                    "ClientSecret".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Issuer".to_string(),
                crate::value::ToValue::to_value(&self.issuer),
            );
            if let Some(ref value) = self.on_unauthenticated_request {
                properties.insert(
                    "OnUnauthenticatedRequest".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scope {
                properties.insert("Scope".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.session_cookie_name {
                properties.insert(
                    "SessionCookieName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.session_timeout {
                properties.insert(
                    "SessionTimeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TokenEndpoint".to_string(),
                crate::value::ToValue::to_value(&self.token_endpoint),
            );
            if let Some(ref value) = self.use_existing_client_secret {
                properties.insert(
                    "UseExistingClientSecret".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "UserInfoEndpoint".to_string(),
                crate::value::ToValue::to_value(&self.user_info_endpoint),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-certificate.html>
    pub struct Certificate_ {
        pub certificate_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_Listener_Certificate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::Listener.Certificate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_Listener_Certificate as Certificate;
    impl crate::value::ToValue for Certificate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.certificate_arn {
                properties.insert(
                    "CertificateArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-fixedresponseconfig.html>
    pub struct FixedResponseConfig_ {
        pub content_type: Option<crate::value::ExpString>,
        pub message_body: Option<crate::value::ExpString>,
        pub status_code: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_Listener_FixedResponseConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::Listener.FixedResponseConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_Listener_FixedResponseConfig as FixedResponseConfig;
    impl crate::value::ToValue for FixedResponseConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.content_type {
                properties.insert(
                    "ContentType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.message_body {
                properties.insert(
                    "MessageBody".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "StatusCode".to_string(),
                crate::value::ToValue::to_value(&self.status_code),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-forwardconfig.html>
    pub struct ForwardConfig_ {
        pub target_group_stickiness_config: Option<Box<TargetGroupStickinessConfig_>>,
        pub target_groups: Option<Vec<TargetGroupTuple_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_Listener_ForwardConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::Listener.ForwardConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_Listener_ForwardConfig as ForwardConfig;
    impl crate::value::ToValue for ForwardConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.target_group_stickiness_config {
                properties.insert(
                    "TargetGroupStickinessConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_groups {
                properties.insert(
                    "TargetGroups".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-jwtvalidationactionadditionalclaim.html>
    pub struct JwtValidationActionAdditionalClaim_ {
        pub format: crate::value::ExpString,
        pub name: crate::value::ExpString,
        pub values: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_Listener_JwtValidationActionAdditionalClaim {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::Listener.JwtValidationActionAdditionalClaim"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_Listener_JwtValidationActionAdditionalClaim as JwtValidationActionAdditionalClaim;
    impl crate::value::ToValue for JwtValidationActionAdditionalClaim_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Format".to_string(),
                crate::value::ToValue::to_value(&self.format),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "Values".to_string(),
                crate::value::ToValue::to_value(&self.values),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-jwtvalidationconfig.html>
    pub struct JwtValidationConfig_ {
        pub additional_claims: Option<Vec<JwtValidationActionAdditionalClaim_>>,
        pub issuer: crate::value::ExpString,
        pub jwks_endpoint: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_Listener_JwtValidationConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::Listener.JwtValidationConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_Listener_JwtValidationConfig as JwtValidationConfig;
    impl crate::value::ToValue for JwtValidationConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.additional_claims {
                properties.insert(
                    "AdditionalClaims".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Issuer".to_string(),
                crate::value::ToValue::to_value(&self.issuer),
            );
            properties.insert(
                "JwksEndpoint".to_string(),
                crate::value::ToValue::to_value(&self.jwks_endpoint),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-listenerattribute.html>
    pub struct ListenerAttribute_ {
        pub key: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_Listener_ListenerAttribute {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::Listener.ListenerAttribute"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_Listener_ListenerAttribute as ListenerAttribute;
    impl crate::value::ToValue for ListenerAttribute_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.key {
                properties.insert("Key".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-mutualauthentication.html>
    pub struct MutualAuthentication_ {
        pub advertise_trust_store_ca_names: Option<crate::value::ExpString>,
        pub ignore_client_certificate_expiry: Option<crate::value::ExpBool>,
        pub mode: Option<crate::value::ExpString>,
        pub trust_store_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_Listener_MutualAuthentication {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::Listener.MutualAuthentication"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_Listener_MutualAuthentication as MutualAuthentication;
    impl crate::value::ToValue for MutualAuthentication_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.advertise_trust_store_ca_names {
                properties.insert(
                    "AdvertiseTrustStoreCaNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ignore_client_certificate_expiry {
                properties.insert(
                    "IgnoreClientCertificateExpiry".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mode {
                properties.insert("Mode".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.trust_store_arn {
                properties.insert(
                    "TrustStoreArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-redirectconfig.html>
    pub struct RedirectConfig_ {
        pub host: Option<crate::value::ExpString>,
        pub path: Option<crate::value::ExpString>,
        pub port: Option<crate::value::ExpString>,
        pub protocol: Option<crate::value::ExpString>,
        pub query: Option<crate::value::ExpString>,
        pub status_code: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_Listener_RedirectConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::Listener.RedirectConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_Listener_RedirectConfig as RedirectConfig;
    impl crate::value::ToValue for RedirectConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.host {
                properties.insert("Host".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.path {
                properties.insert("Path".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.port {
                properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.protocol {
                properties.insert(
                    "Protocol".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.query {
                properties.insert("Query".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "StatusCode".to_string(),
                crate::value::ToValue::to_value(&self.status_code),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-targetgroupstickinessconfig.html>
    pub struct TargetGroupStickinessConfig_ {
        pub duration_seconds: Option<i32>,
        pub enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_Listener_TargetGroupStickinessConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::Listener.TargetGroupStickinessConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_Listener_TargetGroupStickinessConfig as TargetGroupStickinessConfig;
    impl crate::value::ToValue for TargetGroupStickinessConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.duration_seconds {
                properties.insert(
                    "DurationSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-targetgrouptuple.html>
    pub struct TargetGroupTuple_ {
        pub target_group_arn: Option<crate::value::ExpString>,
        pub weight: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_Listener_TargetGroupTuple {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::Listener.TargetGroupTuple"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_Listener_TargetGroupTuple as TargetGroupTuple;
    impl crate::value::ToValue for TargetGroupTuple_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.target_group_arn {
                properties.insert(
                    "TargetGroupArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.weight {
                properties.insert("Weight".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod listenercertificate {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-certificates.html>
    pub struct Certificate_ {
        pub certificate_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_ListenerCertificate_Certificate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::ListenerCertificate.Certificate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_ListenerCertificate_Certificate as Certificate;
    impl crate::value::ToValue for Certificate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.certificate_arn {
                properties.insert(
                    "CertificateArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod listenerrule {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-action.html>
    pub struct Action_ {
        pub authenticate_cognito_config: Option<Box<AuthenticateCognitoConfig_>>,
        pub authenticate_oidc_config: Option<Box<AuthenticateOidcConfig_>>,
        pub fixed_response_config: Option<Box<FixedResponseConfig_>>,
        pub forward_config: Option<Box<ForwardConfig_>>,
        pub jwt_validation_config: Option<Box<JwtValidationConfig_>>,
        pub order: Option<i32>,
        pub redirect_config: Option<Box<RedirectConfig_>>,
        pub target_group_arn: Option<crate::value::ExpString>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_ListenerRule_Action {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::ListenerRule.Action"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_ListenerRule_Action as Action;
    impl crate::value::ToValue for Action_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.authenticate_cognito_config {
                properties.insert(
                    "AuthenticateCognitoConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.authenticate_oidc_config {
                properties.insert(
                    "AuthenticateOidcConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.fixed_response_config {
                properties.insert(
                    "FixedResponseConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.forward_config {
                properties.insert(
                    "ForwardConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.jwt_validation_config {
                properties.insert(
                    "JwtValidationConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.order {
                properties.insert("Order".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.redirect_config {
                properties.insert(
                    "RedirectConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_group_arn {
                properties.insert(
                    "TargetGroupArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-authenticatecognitoconfig.html>
    pub struct AuthenticateCognitoConfig_ {
        pub authentication_request_extra_params:
            Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub on_unauthenticated_request: Option<crate::value::ExpString>,
        pub scope: Option<crate::value::ExpString>,
        pub session_cookie_name: Option<crate::value::ExpString>,
        pub session_timeout: Option<i32>,
        pub user_pool_arn: crate::value::ExpString,
        pub user_pool_client_id: crate::value::ExpString,
        pub user_pool_domain: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_ListenerRule_AuthenticateCognitoConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::ListenerRule.AuthenticateCognitoConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_ListenerRule_AuthenticateCognitoConfig as AuthenticateCognitoConfig;
    impl crate::value::ToValue for AuthenticateCognitoConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.authentication_request_extra_params {
                properties.insert(
                    "AuthenticationRequestExtraParams".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.on_unauthenticated_request {
                properties.insert(
                    "OnUnauthenticatedRequest".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scope {
                properties.insert("Scope".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.session_cookie_name {
                properties.insert(
                    "SessionCookieName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.session_timeout {
                properties.insert(
                    "SessionTimeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "UserPoolArn".to_string(),
                crate::value::ToValue::to_value(&self.user_pool_arn),
            );
            properties.insert(
                "UserPoolClientId".to_string(),
                crate::value::ToValue::to_value(&self.user_pool_client_id),
            );
            properties.insert(
                "UserPoolDomain".to_string(),
                crate::value::ToValue::to_value(&self.user_pool_domain),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-authenticateoidcconfig.html>
    pub struct AuthenticateOidcConfig_ {
        pub authentication_request_extra_params:
            Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub authorization_endpoint: crate::value::ExpString,
        pub client_id: crate::value::ExpString,
        pub client_secret: Option<crate::value::ExpString>,
        pub issuer: crate::value::ExpString,
        pub on_unauthenticated_request: Option<crate::value::ExpString>,
        pub scope: Option<crate::value::ExpString>,
        pub session_cookie_name: Option<crate::value::ExpString>,
        pub session_timeout: Option<i32>,
        pub token_endpoint: crate::value::ExpString,
        pub use_existing_client_secret: Option<crate::value::ExpBool>,
        pub user_info_endpoint: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_ListenerRule_AuthenticateOidcConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::ListenerRule.AuthenticateOidcConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_ListenerRule_AuthenticateOidcConfig as AuthenticateOidcConfig;
    impl crate::value::ToValue for AuthenticateOidcConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.authentication_request_extra_params {
                properties.insert(
                    "AuthenticationRequestExtraParams".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "AuthorizationEndpoint".to_string(),
                crate::value::ToValue::to_value(&self.authorization_endpoint),
            );
            properties.insert(
                "ClientId".to_string(),
                crate::value::ToValue::to_value(&self.client_id),
            );
            if let Some(ref value) = self.client_secret {
                properties.insert(
                    "ClientSecret".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Issuer".to_string(),
                crate::value::ToValue::to_value(&self.issuer),
            );
            if let Some(ref value) = self.on_unauthenticated_request {
                properties.insert(
                    "OnUnauthenticatedRequest".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scope {
                properties.insert("Scope".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.session_cookie_name {
                properties.insert(
                    "SessionCookieName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.session_timeout {
                properties.insert(
                    "SessionTimeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TokenEndpoint".to_string(),
                crate::value::ToValue::to_value(&self.token_endpoint),
            );
            if let Some(ref value) = self.use_existing_client_secret {
                properties.insert(
                    "UseExistingClientSecret".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "UserInfoEndpoint".to_string(),
                crate::value::ToValue::to_value(&self.user_info_endpoint),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-fixedresponseconfig.html>
    pub struct FixedResponseConfig_ {
        pub content_type: Option<crate::value::ExpString>,
        pub message_body: Option<crate::value::ExpString>,
        pub status_code: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_ListenerRule_FixedResponseConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::ListenerRule.FixedResponseConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_ListenerRule_FixedResponseConfig as FixedResponseConfig;
    impl crate::value::ToValue for FixedResponseConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.content_type {
                properties.insert(
                    "ContentType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.message_body {
                properties.insert(
                    "MessageBody".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "StatusCode".to_string(),
                crate::value::ToValue::to_value(&self.status_code),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-forwardconfig.html>
    pub struct ForwardConfig_ {
        pub target_group_stickiness_config: Option<Box<TargetGroupStickinessConfig_>>,
        pub target_groups: Option<Vec<TargetGroupTuple_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_ListenerRule_ForwardConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::ListenerRule.ForwardConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_ListenerRule_ForwardConfig as ForwardConfig;
    impl crate::value::ToValue for ForwardConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.target_group_stickiness_config {
                properties.insert(
                    "TargetGroupStickinessConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_groups {
                properties.insert(
                    "TargetGroups".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-hostheaderconfig.html>
    pub struct HostHeaderConfig_ {
        pub regex_values: Option<Vec<crate::value::ExpString>>,
        pub values: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_ListenerRule_HostHeaderConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::ListenerRule.HostHeaderConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_ListenerRule_HostHeaderConfig as HostHeaderConfig;
    impl crate::value::ToValue for HostHeaderConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.regex_values {
                properties.insert(
                    "RegexValues".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.values {
                properties.insert("Values".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-httpheaderconfig.html>
    pub struct HttpHeaderConfig_ {
        pub http_header_name: Option<crate::value::ExpString>,
        pub regex_values: Option<Vec<crate::value::ExpString>>,
        pub values: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_ListenerRule_HttpHeaderConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::ListenerRule.HttpHeaderConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_ListenerRule_HttpHeaderConfig as HttpHeaderConfig;
    impl crate::value::ToValue for HttpHeaderConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.http_header_name {
                properties.insert(
                    "HttpHeaderName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.regex_values {
                properties.insert(
                    "RegexValues".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.values {
                properties.insert("Values".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-httprequestmethodconfig.html>
    pub struct HttpRequestMethodConfig_ {
        pub values: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_ListenerRule_HttpRequestMethodConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::ListenerRule.HttpRequestMethodConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_ListenerRule_HttpRequestMethodConfig as HttpRequestMethodConfig;
    impl crate::value::ToValue for HttpRequestMethodConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.values {
                properties.insert("Values".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-jwtvalidationactionadditionalclaim.html>
    pub struct JwtValidationActionAdditionalClaim_ {
        pub format: crate::value::ExpString,
        pub name: crate::value::ExpString,
        pub values: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_ListenerRule_JwtValidationActionAdditionalClaim {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::ListenerRule.JwtValidationActionAdditionalClaim"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_ListenerRule_JwtValidationActionAdditionalClaim as JwtValidationActionAdditionalClaim;
    impl crate::value::ToValue for JwtValidationActionAdditionalClaim_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Format".to_string(),
                crate::value::ToValue::to_value(&self.format),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "Values".to_string(),
                crate::value::ToValue::to_value(&self.values),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-jwtvalidationconfig.html>
    pub struct JwtValidationConfig_ {
        pub additional_claims: Option<Vec<JwtValidationActionAdditionalClaim_>>,
        pub issuer: crate::value::ExpString,
        pub jwks_endpoint: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_ListenerRule_JwtValidationConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::ListenerRule.JwtValidationConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_ListenerRule_JwtValidationConfig as JwtValidationConfig;
    impl crate::value::ToValue for JwtValidationConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.additional_claims {
                properties.insert(
                    "AdditionalClaims".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Issuer".to_string(),
                crate::value::ToValue::to_value(&self.issuer),
            );
            properties.insert(
                "JwksEndpoint".to_string(),
                crate::value::ToValue::to_value(&self.jwks_endpoint),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-pathpatternconfig.html>
    pub struct PathPatternConfig_ {
        pub regex_values: Option<Vec<crate::value::ExpString>>,
        pub values: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_ListenerRule_PathPatternConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::ListenerRule.PathPatternConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_ListenerRule_PathPatternConfig as PathPatternConfig;
    impl crate::value::ToValue for PathPatternConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.regex_values {
                properties.insert(
                    "RegexValues".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.values {
                properties.insert("Values".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-querystringconfig.html>
    pub struct QueryStringConfig_ {
        pub values: Option<Vec<QueryStringKeyValue_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_ListenerRule_QueryStringConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::ListenerRule.QueryStringConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_ListenerRule_QueryStringConfig as QueryStringConfig;
    impl crate::value::ToValue for QueryStringConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.values {
                properties.insert("Values".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-querystringkeyvalue.html>
    pub struct QueryStringKeyValue_ {
        pub key: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_ListenerRule_QueryStringKeyValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::ListenerRule.QueryStringKeyValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_ListenerRule_QueryStringKeyValue as QueryStringKeyValue;
    impl crate::value::ToValue for QueryStringKeyValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.key {
                properties.insert("Key".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-redirectconfig.html>
    pub struct RedirectConfig_ {
        pub host: Option<crate::value::ExpString>,
        pub path: Option<crate::value::ExpString>,
        pub port: Option<crate::value::ExpString>,
        pub protocol: Option<crate::value::ExpString>,
        pub query: Option<crate::value::ExpString>,
        pub status_code: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_ListenerRule_RedirectConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::ListenerRule.RedirectConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_ListenerRule_RedirectConfig as RedirectConfig;
    impl crate::value::ToValue for RedirectConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.host {
                properties.insert("Host".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.path {
                properties.insert("Path".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.port {
                properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.protocol {
                properties.insert(
                    "Protocol".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.query {
                properties.insert("Query".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "StatusCode".to_string(),
                crate::value::ToValue::to_value(&self.status_code),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-rewriteconfig.html>
    pub struct RewriteConfig_ {
        pub regex: crate::value::ExpString,
        pub replace: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_ListenerRule_RewriteConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::ListenerRule.RewriteConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_ListenerRule_RewriteConfig as RewriteConfig;
    impl crate::value::ToValue for RewriteConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Regex".to_string(),
                crate::value::ToValue::to_value(&self.regex),
            );
            properties.insert(
                "Replace".to_string(),
                crate::value::ToValue::to_value(&self.replace),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-rewriteconfigobject.html>
    pub struct RewriteConfigObject_ {
        pub rewrites: Vec<RewriteConfig_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_ListenerRule_RewriteConfigObject {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::ListenerRule.RewriteConfigObject"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_ListenerRule_RewriteConfigObject as RewriteConfigObject;
    impl crate::value::ToValue for RewriteConfigObject_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Rewrites".to_string(),
                crate::value::ToValue::to_value(&self.rewrites),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-rulecondition.html>
    pub struct RuleCondition_ {
        pub field: Option<crate::value::ExpString>,
        pub host_header_config: Option<Box<HostHeaderConfig_>>,
        pub http_header_config: Option<Box<HttpHeaderConfig_>>,
        pub http_request_method_config: Option<Box<HttpRequestMethodConfig_>>,
        pub path_pattern_config: Option<Box<PathPatternConfig_>>,
        pub query_string_config: Option<Box<QueryStringConfig_>>,
        pub regex_values: Option<Vec<crate::value::ExpString>>,
        pub source_ip_config: Option<Box<SourceIpConfig_>>,
        pub values: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_ListenerRule_RuleCondition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::ListenerRule.RuleCondition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_ListenerRule_RuleCondition as RuleCondition;
    impl crate::value::ToValue for RuleCondition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.field {
                properties.insert("Field".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.host_header_config {
                properties.insert(
                    "HostHeaderConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.http_header_config {
                properties.insert(
                    "HttpHeaderConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.http_request_method_config {
                properties.insert(
                    "HttpRequestMethodConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.path_pattern_config {
                properties.insert(
                    "PathPatternConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.query_string_config {
                properties.insert(
                    "QueryStringConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.regex_values {
                properties.insert(
                    "RegexValues".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_ip_config {
                properties.insert(
                    "SourceIpConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.values {
                properties.insert("Values".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-sourceipconfig.html>
    pub struct SourceIpConfig_ {
        pub values: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_ListenerRule_SourceIpConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::ListenerRule.SourceIpConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_ListenerRule_SourceIpConfig as SourceIpConfig;
    impl crate::value::ToValue for SourceIpConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.values {
                properties.insert("Values".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-targetgroupstickinessconfig.html>
    pub struct TargetGroupStickinessConfig_ {
        pub duration_seconds: Option<i32>,
        pub enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_ListenerRule_TargetGroupStickinessConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::ListenerRule.TargetGroupStickinessConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_ListenerRule_TargetGroupStickinessConfig as TargetGroupStickinessConfig;
    impl crate::value::ToValue for TargetGroupStickinessConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.duration_seconds {
                properties.insert(
                    "DurationSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-targetgrouptuple.html>
    pub struct TargetGroupTuple_ {
        pub target_group_arn: Option<crate::value::ExpString>,
        pub weight: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_ListenerRule_TargetGroupTuple {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::ListenerRule.TargetGroupTuple"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_ListenerRule_TargetGroupTuple as TargetGroupTuple;
    impl crate::value::ToValue for TargetGroupTuple_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.target_group_arn {
                properties.insert(
                    "TargetGroupArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.weight {
                properties.insert("Weight".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-transform.html>
    pub struct Transform_ {
        pub host_header_rewrite_config: Option<Box<RewriteConfigObject_>>,
        pub r#type: crate::value::ExpString,
        pub url_rewrite_config: Option<Box<RewriteConfigObject_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_ListenerRule_Transform {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::ListenerRule.Transform"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_ListenerRule_Transform as Transform;
    impl crate::value::ToValue for Transform_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.host_header_rewrite_config {
                properties.insert(
                    "HostHeaderRewriteConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            if let Some(ref value) = self.url_rewrite_config {
                properties.insert(
                    "UrlRewriteConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod loadbalancer {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-loadbalancer-loadbalancerattribute.html>
    pub struct LoadBalancerAttribute_ {
        pub key: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_LoadBalancer_LoadBalancerAttribute {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::LoadBalancer.LoadBalancerAttribute"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_LoadBalancer_LoadBalancerAttribute as LoadBalancerAttribute;
    impl crate::value::ToValue for LoadBalancerAttribute_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.key {
                properties.insert("Key".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-loadbalancer-minimumloadbalancercapacity.html>
    pub struct MinimumLoadBalancerCapacity_ {
        pub capacity_units: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_LoadBalancer_MinimumLoadBalancerCapacity {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::LoadBalancer.MinimumLoadBalancerCapacity"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_LoadBalancer_MinimumLoadBalancerCapacity as MinimumLoadBalancerCapacity;
    impl crate::value::ToValue for MinimumLoadBalancerCapacity_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CapacityUnits".to_string(),
                crate::value::ToValue::to_value(&self.capacity_units),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-loadbalancer-subnetmapping.html>
    pub struct SubnetMapping_ {
        pub allocation_id: Option<crate::value::ExpString>,
        pub i_pv6_address: Option<crate::value::ExpString>,
        pub private_i_pv4_address: Option<crate::value::ExpString>,
        pub source_nat_ipv6_prefix: Option<crate::value::ExpString>,
        pub subnet_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_LoadBalancer_SubnetMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::LoadBalancer.SubnetMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_LoadBalancer_SubnetMapping as SubnetMapping;
    impl crate::value::ToValue for SubnetMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allocation_id {
                properties.insert(
                    "AllocationId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.i_pv6_address {
                properties.insert(
                    "IPv6Address".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.private_i_pv4_address {
                properties.insert(
                    "PrivateIPv4Address".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_nat_ipv6_prefix {
                properties.insert(
                    "SourceNatIpv6Prefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SubnetId".to_string(),
                crate::value::ToValue::to_value(&self.subnet_id),
            );
            properties.into()
        }
    }
}
pub mod targetgroup {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-targetgroup-matcher.html>
    pub struct Matcher_ {
        pub grpc_code: Option<crate::value::ExpString>,
        pub http_code: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_TargetGroup_Matcher {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::TargetGroup.Matcher"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_TargetGroup_Matcher as Matcher;
    impl crate::value::ToValue for Matcher_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.grpc_code {
                properties.insert(
                    "GrpcCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.http_code {
                properties.insert(
                    "HttpCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-targetgroup-targetdescription.html>
    pub struct TargetDescription_ {
        pub availability_zone: Option<crate::value::ExpString>,
        pub id: crate::value::ExpString,
        pub port: Option<i32>,
        pub quic_server_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_TargetGroup_TargetDescription {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::TargetGroup.TargetDescription"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_TargetGroup_TargetDescription as TargetDescription;
    impl crate::value::ToValue for TargetDescription_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.availability_zone {
                properties.insert(
                    "AvailabilityZone".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            if let Some(ref value) = self.port {
                properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.quic_server_id {
                properties.insert(
                    "QuicServerId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-targetgroup-targetgroupattribute.html>
    pub struct TargetGroupAttribute_ {
        pub key: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_TargetGroup_TargetGroupAttribute {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::TargetGroup.TargetGroupAttribute"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_TargetGroup_TargetGroupAttribute as TargetGroupAttribute;
    impl crate::value::ToValue for TargetGroupAttribute_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.key {
                properties.insert("Key".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod truststorerevocation {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-truststorerevocation-revocationcontent.html>
    pub struct RevocationContent_ {
        pub revocation_type: Option<crate::value::ExpString>,
        pub s3_bucket: Option<crate::value::ExpString>,
        pub s3_key: Option<crate::value::ExpString>,
        pub s3_object_version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_TrustStoreRevocation_RevocationContent {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::TrustStoreRevocation.RevocationContent"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_TrustStoreRevocation_RevocationContent as RevocationContent;
    impl crate::value::ToValue for RevocationContent_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.revocation_type {
                properties.insert(
                    "RevocationType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_bucket {
                properties.insert(
                    "S3Bucket".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_key {
                properties.insert("S3Key".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.s3_object_version {
                properties.insert(
                    "S3ObjectVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-truststorerevocation-truststorerevocation.html>
    pub struct TrustStoreRevocation_ {
        pub number_of_revoked_entries: Option<i64>,
        pub revocation_id: Option<crate::value::ExpString>,
        pub revocation_type: Option<crate::value::ExpString>,
        pub trust_store_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancingv2_TrustStoreRevocation_TrustStoreRevocation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancingV2::TrustStoreRevocation.TrustStoreRevocation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancingv2_TrustStoreRevocation_TrustStoreRevocation as TrustStoreRevocation;
    impl crate::value::ToValue for TrustStoreRevocation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.number_of_revoked_entries {
                properties.insert(
                    "NumberOfRevokedEntries".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.revocation_id {
                properties.insert(
                    "RevocationId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.revocation_type {
                properties.insert(
                    "RevocationType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.trust_store_arn {
                properties.insert(
                    "TrustStoreArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-listener.html>
pub struct Listener_ {
    pub alpn_policy: Option<Vec<crate::value::ExpString>>,
    pub certificates: Option<Vec<super::elasticloadbalancingv2::listener::Certificate_>>,
    pub default_actions: Vec<super::elasticloadbalancingv2::listener::Action_>,
    pub listener_attributes:
        Option<Vec<super::elasticloadbalancingv2::listener::ListenerAttribute_>>,
    pub load_balancer_arn: crate::value::ExpString,
    pub mutual_authentication:
        Option<super::elasticloadbalancingv2::listener::MutualAuthentication_>,
    pub port: Option<i32>,
    pub protocol: Option<crate::value::ExpString>,
    pub ssl_policy: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_elasticloadbalancingv2_Listener {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ElasticLoadBalancingV2::Listener"
        $($field $value)*)
    };
}
pub use crate::__aws_elasticloadbalancingv2_Listener as Listener;
impl crate::template::ToResource for Listener_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ElasticLoadBalancingV2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Listener"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.alpn_policy {
            properties.insert(
                "AlpnPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.certificates {
            properties.insert(
                "Certificates".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DefaultActions".to_string(),
            crate::value::ToValue::to_value(&self.default_actions),
        );
        if let Some(ref value) = self.listener_attributes {
            properties.insert(
                "ListenerAttributes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "LoadBalancerArn".to_string(),
            crate::value::ToValue::to_value(&self.load_balancer_arn),
        );
        if let Some(ref value) = self.mutual_authentication {
            properties.insert(
                "MutualAuthentication".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.port {
            properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.protocol {
            properties.insert(
                "Protocol".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ssl_policy {
            properties.insert(
                "SslPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-listenercertificate.html>
pub struct ListenerCertificate_ {
    pub certificates: Vec<super::elasticloadbalancingv2::listenercertificate::Certificate_>,
    pub listener_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_elasticloadbalancingv2_ListenerCertificate {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ElasticLoadBalancingV2::ListenerCertificate"
        $($field $value)*)
    };
}
pub use crate::__aws_elasticloadbalancingv2_ListenerCertificate as ListenerCertificate;
impl crate::template::ToResource for ListenerCertificate_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ElasticLoadBalancingV2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ListenerCertificate"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Certificates".to_string(),
            crate::value::ToValue::to_value(&self.certificates),
        );
        properties.insert(
            "ListenerArn".to_string(),
            crate::value::ToValue::to_value(&self.listener_arn),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-listenerrule.html>
pub struct ListenerRule_ {
    pub actions: Vec<super::elasticloadbalancingv2::listenerrule::Action_>,
    pub conditions: Vec<super::elasticloadbalancingv2::listenerrule::RuleCondition_>,
    pub listener_arn: Option<crate::value::ExpString>,
    pub priority: i32,
    pub transforms: Option<Vec<super::elasticloadbalancingv2::listenerrule::Transform_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_elasticloadbalancingv2_ListenerRule {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ElasticLoadBalancingV2::ListenerRule"
        $($field $value)*)
    };
}
pub use crate::__aws_elasticloadbalancingv2_ListenerRule as ListenerRule;
impl crate::template::ToResource for ListenerRule_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ElasticLoadBalancingV2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ListenerRule"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Actions".to_string(),
            crate::value::ToValue::to_value(&self.actions),
        );
        properties.insert(
            "Conditions".to_string(),
            crate::value::ToValue::to_value(&self.conditions),
        );
        if let Some(ref value) = self.listener_arn {
            properties.insert(
                "ListenerArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Priority".to_string(),
            crate::value::ToValue::to_value(&self.priority),
        );
        if let Some(ref value) = self.transforms {
            properties.insert(
                "Transforms".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-loadbalancer.html>
pub struct LoadBalancer_ {
    pub enable_capacity_reservation_provision_stabilize: Option<crate::value::ExpBool>,
    pub enable_prefix_for_ipv6_source_nat: Option<crate::value::ExpString>,
    pub enforce_security_group_inbound_rules_on_private_link_traffic:
        Option<crate::value::ExpString>,
    pub ip_address_type: Option<crate::value::ExpString>,
    pub ipv4_ipam_pool_id: Option<crate::value::ExpString>,
    pub load_balancer_attributes:
        Option<Vec<super::elasticloadbalancingv2::loadbalancer::LoadBalancerAttribute_>>,
    pub minimum_load_balancer_capacity:
        Option<super::elasticloadbalancingv2::loadbalancer::MinimumLoadBalancerCapacity_>,
    pub name: Option<crate::value::ExpString>,
    pub scheme: Option<crate::value::ExpString>,
    pub security_groups: Option<Vec<crate::value::ExpString>>,
    pub subnet_mappings: Option<Vec<super::elasticloadbalancingv2::loadbalancer::SubnetMapping_>>,
    pub subnets: Option<Vec<crate::value::ExpString>>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub r#type: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_elasticloadbalancingv2_LoadBalancer {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ElasticLoadBalancingV2::LoadBalancer"
        $($field $value)*)
    };
}
pub use crate::__aws_elasticloadbalancingv2_LoadBalancer as LoadBalancer;
impl crate::template::ToResource for LoadBalancer_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ElasticLoadBalancingV2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LoadBalancer"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.enable_capacity_reservation_provision_stabilize {
            properties.insert(
                "EnableCapacityReservationProvisionStabilize".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enable_prefix_for_ipv6_source_nat {
            properties.insert(
                "EnablePrefixForIpv6SourceNat".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enforce_security_group_inbound_rules_on_private_link_traffic {
            properties.insert(
                "EnforceSecurityGroupInboundRulesOnPrivateLinkTraffic".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ip_address_type {
            properties.insert(
                "IpAddressType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ipv4_ipam_pool_id {
            properties.insert(
                "Ipv4IpamPoolId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.load_balancer_attributes {
            properties.insert(
                "LoadBalancerAttributes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.minimum_load_balancer_capacity {
            properties.insert(
                "MinimumLoadBalancerCapacity".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.scheme {
            properties.insert("Scheme".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.security_groups {
            properties.insert(
                "SecurityGroups".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.subnet_mappings {
            properties.insert(
                "SubnetMappings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.subnets {
            properties.insert(
                "Subnets".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.r#type {
            properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-targetgroup.html>
pub struct TargetGroup_ {
    pub health_check_enabled: Option<crate::value::ExpBool>,
    pub health_check_interval_seconds: Option<i32>,
    pub health_check_path: Option<crate::value::ExpString>,
    pub health_check_port: Option<crate::value::ExpString>,
    pub health_check_protocol: Option<crate::value::ExpString>,
    pub health_check_timeout_seconds: Option<i32>,
    pub healthy_threshold_count: Option<i32>,
    pub ip_address_type: Option<crate::value::ExpString>,
    pub matcher: Option<super::elasticloadbalancingv2::targetgroup::Matcher_>,
    pub name: Option<crate::value::ExpString>,
    pub port: Option<i32>,
    pub protocol: Option<crate::value::ExpString>,
    pub protocol_version: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub target_control_port: Option<i32>,
    pub target_group_attributes:
        Option<Vec<super::elasticloadbalancingv2::targetgroup::TargetGroupAttribute_>>,
    pub target_type: Option<crate::value::ExpString>,
    pub targets: Option<Vec<super::elasticloadbalancingv2::targetgroup::TargetDescription_>>,
    pub unhealthy_threshold_count: Option<i32>,
    pub vpc_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_elasticloadbalancingv2_TargetGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ElasticLoadBalancingV2::TargetGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_elasticloadbalancingv2_TargetGroup as TargetGroup;
impl crate::template::ToResource for TargetGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ElasticLoadBalancingV2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TargetGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.health_check_enabled {
            properties.insert(
                "HealthCheckEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.health_check_interval_seconds {
            properties.insert(
                "HealthCheckIntervalSeconds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.health_check_path {
            properties.insert(
                "HealthCheckPath".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.health_check_port {
            properties.insert(
                "HealthCheckPort".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.health_check_protocol {
            properties.insert(
                "HealthCheckProtocol".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.health_check_timeout_seconds {
            properties.insert(
                "HealthCheckTimeoutSeconds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.healthy_threshold_count {
            properties.insert(
                "HealthyThresholdCount".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ip_address_type {
            properties.insert(
                "IpAddressType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.matcher {
            properties.insert(
                "Matcher".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.port {
            properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.protocol {
            properties.insert(
                "Protocol".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.protocol_version {
            properties.insert(
                "ProtocolVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.target_control_port {
            properties.insert(
                "TargetControlPort".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.target_group_attributes {
            properties.insert(
                "TargetGroupAttributes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.target_type {
            properties.insert(
                "TargetType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.targets {
            properties.insert(
                "Targets".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.unhealthy_threshold_count {
            properties.insert(
                "UnhealthyThresholdCount".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.vpc_id {
            properties.insert("VpcId".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-truststore.html>
pub struct TrustStore_ {
    pub ca_certificates_bundle_s3_bucket: Option<crate::value::ExpString>,
    pub ca_certificates_bundle_s3_key: Option<crate::value::ExpString>,
    pub ca_certificates_bundle_s3_object_version: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_elasticloadbalancingv2_TrustStore {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ElasticLoadBalancingV2::TrustStore"
        $($field $value)*)
    };
}
pub use crate::__aws_elasticloadbalancingv2_TrustStore as TrustStore;
impl crate::template::ToResource for TrustStore_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ElasticLoadBalancingV2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TrustStore"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.ca_certificates_bundle_s3_bucket {
            properties.insert(
                "CaCertificatesBundleS3Bucket".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ca_certificates_bundle_s3_key {
            properties.insert(
                "CaCertificatesBundleS3Key".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ca_certificates_bundle_s3_object_version {
            properties.insert(
                "CaCertificatesBundleS3ObjectVersion".to_string(),
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
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-truststorerevocation.html>
pub struct TrustStoreRevocation_ {
    pub revocation_contents:
        Option<Vec<super::elasticloadbalancingv2::truststorerevocation::RevocationContent_>>,
    pub trust_store_arn: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_elasticloadbalancingv2_TrustStoreRevocation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ElasticLoadBalancingV2::TrustStoreRevocation"
        $($field $value)*)
    };
}
pub use crate::__aws_elasticloadbalancingv2_TrustStoreRevocation as TrustStoreRevocation;
impl crate::template::ToResource for TrustStoreRevocation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ElasticLoadBalancingV2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TrustStoreRevocation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.revocation_contents {
            properties.insert(
                "RevocationContents".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.trust_store_arn {
            properties.insert(
                "TrustStoreArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
