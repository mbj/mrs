pub mod application {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-refactorspaces-application-apigatewayproxyinput.html
    pub struct ApiGatewayProxyInput_ {
        pub endpoint_type: Option<crate::value::ExpString>,
        pub stage_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_refactorspaces_Application_ApiGatewayProxyInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RefactorSpaces::Application.ApiGatewayProxyInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_refactorspaces_Application_ApiGatewayProxyInput as ApiGatewayProxyInput;
    impl crate::value::ToValue for ApiGatewayProxyInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.endpoint_type {
                properties.insert(
                    "EndpointType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stage_name {
                properties.insert(
                    "StageName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod route {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-refactorspaces-route-defaultrouteinput.html
    pub struct DefaultRouteInput_ {
        pub activation_state: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_refactorspaces_Route_DefaultRouteInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RefactorSpaces::Route.DefaultRouteInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_refactorspaces_Route_DefaultRouteInput as DefaultRouteInput;
    impl crate::value::ToValue for DefaultRouteInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ActivationState".to_string(),
                crate::value::ToValue::to_value(&self.activation_state),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-refactorspaces-route-uripathrouteinput.html
    pub struct UriPathRouteInput_ {
        pub activation_state: crate::value::ExpString,
        pub append_source_path: Option<crate::value::ExpBool>,
        pub include_child_paths: Option<crate::value::ExpBool>,
        pub methods: Option<Vec<crate::value::ExpString>>,
        pub source_path: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_refactorspaces_Route_UriPathRouteInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RefactorSpaces::Route.UriPathRouteInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_refactorspaces_Route_UriPathRouteInput as UriPathRouteInput;
    impl crate::value::ToValue for UriPathRouteInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ActivationState".to_string(),
                crate::value::ToValue::to_value(&self.activation_state),
            );
            if let Some(ref value) = self.append_source_path {
                properties.insert(
                    "AppendSourcePath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_child_paths {
                properties.insert(
                    "IncludeChildPaths".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.methods {
                properties.insert(
                    "Methods".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_path {
                properties.insert(
                    "SourcePath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod service {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-refactorspaces-service-lambdaendpointinput.html
    pub struct LambdaEndpointInput_ {
        pub arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_refactorspaces_Service_LambdaEndpointInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RefactorSpaces::Service.LambdaEndpointInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_refactorspaces_Service_LambdaEndpointInput as LambdaEndpointInput;
    impl crate::value::ToValue for LambdaEndpointInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Arn".to_string(),
                crate::value::ToValue::to_value(&self.arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-refactorspaces-service-urlendpointinput.html
    pub struct UrlEndpointInput_ {
        pub health_url: Option<crate::value::ExpString>,
        pub url: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_refactorspaces_Service_UrlEndpointInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RefactorSpaces::Service.UrlEndpointInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_refactorspaces_Service_UrlEndpointInput as UrlEndpointInput;
    impl crate::value::ToValue for UrlEndpointInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.health_url {
                properties.insert(
                    "HealthUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Url".to_string(),
                crate::value::ToValue::to_value(&self.url),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-refactorspaces-application.html
pub struct Application_ {
    pub api_gateway_proxy: Option<super::refactorspaces::application::ApiGatewayProxyInput_>,
    pub environment_identifier: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub proxy_type: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_refactorspaces_Application {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::RefactorSpaces::Application"
        $($field $value)*)
    };
}
pub use crate::__aws_refactorspaces_Application as Application;
impl crate::template::ToResource for Application_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RefactorSpaces"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Application"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.api_gateway_proxy {
            properties.insert(
                "ApiGatewayProxy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "EnvironmentIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.environment_identifier),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "ProxyType".to_string(),
            crate::value::ToValue::to_value(&self.proxy_type),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "VpcId".to_string(),
            crate::value::ToValue::to_value(&self.vpc_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-refactorspaces-environment.html
pub struct Environment_ {
    pub description: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub network_fabric_type: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_refactorspaces_Environment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::RefactorSpaces::Environment"
        $($field $value)*)
    };
}
pub use crate::__aws_refactorspaces_Environment as Environment;
impl crate::template::ToResource for Environment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RefactorSpaces"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Environment"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.network_fabric_type {
            properties.insert(
                "NetworkFabricType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-refactorspaces-route.html
pub struct Route_ {
    pub application_identifier: crate::value::ExpString,
    pub default_route: Option<super::refactorspaces::route::DefaultRouteInput_>,
    pub environment_identifier: crate::value::ExpString,
    pub route_type: crate::value::ExpString,
    pub service_identifier: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub uri_path_route: Option<super::refactorspaces::route::UriPathRouteInput_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_refactorspaces_Route {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::RefactorSpaces::Route"
        $($field $value)*)
    };
}
pub use crate::__aws_refactorspaces_Route as Route;
impl crate::template::ToResource for Route_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RefactorSpaces"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Route"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApplicationIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.application_identifier),
        );
        if let Some(ref value) = self.default_route {
            properties.insert(
                "DefaultRoute".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "EnvironmentIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.environment_identifier),
        );
        properties.insert(
            "RouteType".to_string(),
            crate::value::ToValue::to_value(&self.route_type),
        );
        properties.insert(
            "ServiceIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.service_identifier),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.uri_path_route {
            properties.insert(
                "UriPathRoute".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-refactorspaces-service.html
pub struct Service_ {
    pub application_identifier: crate::value::ExpString,
    pub description: Option<crate::value::ExpString>,
    pub endpoint_type: crate::value::ExpString,
    pub environment_identifier: crate::value::ExpString,
    pub lambda_endpoint: Option<super::refactorspaces::service::LambdaEndpointInput_>,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub url_endpoint: Option<super::refactorspaces::service::UrlEndpointInput_>,
    pub vpc_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_refactorspaces_Service {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::RefactorSpaces::Service"
        $($field $value)*)
    };
}
pub use crate::__aws_refactorspaces_Service as Service;
impl crate::template::ToResource for Service_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RefactorSpaces"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Service"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApplicationIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.application_identifier),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "EndpointType".to_string(),
            crate::value::ToValue::to_value(&self.endpoint_type),
        );
        properties.insert(
            "EnvironmentIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.environment_identifier),
        );
        if let Some(ref value) = self.lambda_endpoint {
            properties.insert(
                "LambdaEndpoint".to_string(),
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
        if let Some(ref value) = self.url_endpoint {
            properties.insert(
                "UrlEndpoint".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.vpc_id {
            properties.insert("VpcId".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
