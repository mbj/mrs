pub mod loadbalancer {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-accessloggingpolicy.html
    pub struct AccessLoggingPolicy_ {
        pub emit_interval: Option<i32>,
        pub enabled: crate::value::ExpBool,
        pub s3_bucket_name: crate::value::ExpString,
        pub s3_bucket_prefix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancing_LoadBalancer_AccessLoggingPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancing::LoadBalancer.AccessLoggingPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancing_LoadBalancer_AccessLoggingPolicy as AccessLoggingPolicy;
    impl crate::value::ToValue for AccessLoggingPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.emit_interval {
                properties.insert(
                    "EmitInterval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            properties.insert(
                "S3BucketName".to_string(),
                crate::value::ToValue::to_value(&self.s3_bucket_name),
            );
            if let Some(ref value) = self.s3_bucket_prefix {
                properties.insert(
                    "S3BucketPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-AppCookieStickinessPolicy.html
    pub struct AppCookieStickinessPolicy_ {
        pub cookie_name: crate::value::ExpString,
        pub policy_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancing_LoadBalancer_AppCookieStickinessPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancing::LoadBalancer.AppCookieStickinessPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancing_LoadBalancer_AppCookieStickinessPolicy as AppCookieStickinessPolicy;
    impl crate::value::ToValue for AppCookieStickinessPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CookieName".to_string(),
                crate::value::ToValue::to_value(&self.cookie_name),
            );
            properties.insert(
                "PolicyName".to_string(),
                crate::value::ToValue::to_value(&self.policy_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-connectiondrainingpolicy.html
    pub struct ConnectionDrainingPolicy_ {
        pub enabled: crate::value::ExpBool,
        pub timeout: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancing_LoadBalancer_ConnectionDrainingPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancing::LoadBalancer.ConnectionDrainingPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancing_LoadBalancer_ConnectionDrainingPolicy as ConnectionDrainingPolicy;
    impl crate::value::ToValue for ConnectionDrainingPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            if let Some(ref value) = self.timeout {
                properties.insert(
                    "Timeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-connectionsettings.html
    pub struct ConnectionSettings_ {
        pub idle_timeout: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancing_LoadBalancer_ConnectionSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancing::LoadBalancer.ConnectionSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancing_LoadBalancer_ConnectionSettings as ConnectionSettings;
    impl crate::value::ToValue for ConnectionSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IdleTimeout".to_string(),
                crate::value::ToValue::to_value(&self.idle_timeout),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-health-check.html
    pub struct HealthCheck_ {
        pub healthy_threshold: crate::value::ExpString,
        pub interval: crate::value::ExpString,
        pub target: crate::value::ExpString,
        pub timeout: crate::value::ExpString,
        pub unhealthy_threshold: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancing_LoadBalancer_HealthCheck {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancing::LoadBalancer.HealthCheck"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancing_LoadBalancer_HealthCheck as HealthCheck;
    impl crate::value::ToValue for HealthCheck_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "HealthyThreshold".to_string(),
                crate::value::ToValue::to_value(&self.healthy_threshold),
            );
            properties.insert(
                "Interval".to_string(),
                crate::value::ToValue::to_value(&self.interval),
            );
            properties.insert(
                "Target".to_string(),
                crate::value::ToValue::to_value(&self.target),
            );
            properties.insert(
                "Timeout".to_string(),
                crate::value::ToValue::to_value(&self.timeout),
            );
            properties.insert(
                "UnhealthyThreshold".to_string(),
                crate::value::ToValue::to_value(&self.unhealthy_threshold),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-LBCookieStickinessPolicy.html
    pub struct LBCookieStickinessPolicy_ {
        pub cookie_expiration_period: Option<crate::value::ExpString>,
        pub policy_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancing_LoadBalancer_LBCookieStickinessPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancing::LoadBalancer.LBCookieStickinessPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancing_LoadBalancer_LBCookieStickinessPolicy as LBCookieStickinessPolicy;
    impl crate::value::ToValue for LBCookieStickinessPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cookie_expiration_period {
                properties.insert(
                    "CookieExpirationPeriod".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.policy_name {
                properties.insert(
                    "PolicyName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-listener.html
    pub struct Listeners_ {
        pub instance_port: crate::value::ExpString,
        pub instance_protocol: Option<crate::value::ExpString>,
        pub load_balancer_port: crate::value::ExpString,
        pub policy_names: Option<Vec<crate::value::ExpString>>,
        pub protocol: crate::value::ExpString,
        pub ssl_certificate_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancing_LoadBalancer_Listeners {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancing::LoadBalancer.Listeners"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancing_LoadBalancer_Listeners as Listeners;
    impl crate::value::ToValue for Listeners_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InstancePort".to_string(),
                crate::value::ToValue::to_value(&self.instance_port),
            );
            if let Some(ref value) = self.instance_protocol {
                properties.insert(
                    "InstanceProtocol".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "LoadBalancerPort".to_string(),
                crate::value::ToValue::to_value(&self.load_balancer_port),
            );
            if let Some(ref value) = self.policy_names {
                properties.insert(
                    "PolicyNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Protocol".to_string(),
                crate::value::ToValue::to_value(&self.protocol),
            );
            if let Some(ref value) = self.ssl_certificate_id {
                properties.insert(
                    "SSLCertificateId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-policy.html
    pub struct Policies_ {
        pub attributes: Vec<serde_json::Value>,
        pub instance_ports: Option<Vec<crate::value::ExpString>>,
        pub load_balancer_ports: Option<Vec<crate::value::ExpString>>,
        pub policy_name: crate::value::ExpString,
        pub policy_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticloadbalancing_LoadBalancer_Policies {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElasticLoadBalancing::LoadBalancer.Policies"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticloadbalancing_LoadBalancer_Policies as Policies;
    impl crate::value::ToValue for Policies_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Attributes".to_string(),
                crate::value::ToValue::to_value(&self.attributes),
            );
            if let Some(ref value) = self.instance_ports {
                properties.insert(
                    "InstancePorts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.load_balancer_ports {
                properties.insert(
                    "LoadBalancerPorts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "PolicyName".to_string(),
                crate::value::ToValue::to_value(&self.policy_name),
            );
            properties.insert(
                "PolicyType".to_string(),
                crate::value::ToValue::to_value(&self.policy_type),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb.html
pub struct LoadBalancer_ {
    pub access_logging_policy:
        Option<super::elasticloadbalancing::loadbalancer::AccessLoggingPolicy_>,
    pub app_cookie_stickiness_policy:
        Option<Vec<super::elasticloadbalancing::loadbalancer::AppCookieStickinessPolicy_>>,
    pub availability_zones: Option<Vec<crate::value::ExpString>>,
    pub connection_draining_policy:
        Option<super::elasticloadbalancing::loadbalancer::ConnectionDrainingPolicy_>,
    pub connection_settings: Option<super::elasticloadbalancing::loadbalancer::ConnectionSettings_>,
    pub cross_zone: Option<crate::value::ExpBool>,
    pub health_check: Option<super::elasticloadbalancing::loadbalancer::HealthCheck_>,
    pub instances: Option<Vec<crate::value::ExpString>>,
    pub lb_cookie_stickiness_policy:
        Option<Vec<super::elasticloadbalancing::loadbalancer::LBCookieStickinessPolicy_>>,
    pub listeners: Vec<super::elasticloadbalancing::loadbalancer::Listeners_>,
    pub load_balancer_name: Option<crate::value::ExpString>,
    pub policies: Option<Vec<super::elasticloadbalancing::loadbalancer::Policies_>>,
    pub scheme: Option<crate::value::ExpString>,
    pub security_groups: Option<Vec<crate::value::ExpString>>,
    pub subnets: Option<Vec<crate::value::ExpString>>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_elasticloadbalancing_LoadBalancer {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ElasticLoadBalancing::LoadBalancer"
        $($field $value)*)
    };
}
pub use crate::__aws_elasticloadbalancing_LoadBalancer as LoadBalancer;
impl crate::template::ToResource for LoadBalancer_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ElasticLoadBalancing"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LoadBalancer"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.access_logging_policy {
            properties.insert(
                "AccessLoggingPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.app_cookie_stickiness_policy {
            properties.insert(
                "AppCookieStickinessPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.availability_zones {
            properties.insert(
                "AvailabilityZones".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.connection_draining_policy {
            properties.insert(
                "ConnectionDrainingPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.connection_settings {
            properties.insert(
                "ConnectionSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cross_zone {
            properties.insert(
                "CrossZone".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.health_check {
            properties.insert(
                "HealthCheck".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.instances {
            properties.insert(
                "Instances".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.lb_cookie_stickiness_policy {
            properties.insert(
                "LBCookieStickinessPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Listeners".to_string(),
            crate::value::ToValue::to_value(&self.listeners),
        );
        if let Some(ref value) = self.load_balancer_name {
            properties.insert(
                "LoadBalancerName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.policies {
            properties.insert(
                "Policies".to_string(),
                crate::value::ToValue::to_value(value),
            );
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
        if let Some(ref value) = self.subnets {
            properties.insert(
                "Subnets".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
