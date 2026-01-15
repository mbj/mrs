pub mod gatewayroute {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-gatewayroutehostnamematch.html
    pub struct GatewayRouteHostnameMatch_ {
        pub exact: Option<crate::value::ExpString>,
        pub suffix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_GatewayRoute_GatewayRouteHostnameMatch {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::GatewayRoute.GatewayRouteHostnameMatch"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_GatewayRoute_GatewayRouteHostnameMatch as GatewayRouteHostnameMatch;
    impl crate::value::ToValue for GatewayRouteHostnameMatch_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.exact {
                properties.insert("Exact".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.suffix {
                properties.insert("Suffix".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-gatewayroutehostnamerewrite.html
    pub struct GatewayRouteHostnameRewrite_ {
        pub default_target_hostname: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_GatewayRoute_GatewayRouteHostnameRewrite {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::GatewayRoute.GatewayRouteHostnameRewrite"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_GatewayRoute_GatewayRouteHostnameRewrite as GatewayRouteHostnameRewrite;
    impl crate::value::ToValue for GatewayRouteHostnameRewrite_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.default_target_hostname {
                properties.insert(
                    "DefaultTargetHostname".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-gatewayroutemetadatamatch.html
    pub struct GatewayRouteMetadataMatch_ {
        pub exact: Option<crate::value::ExpString>,
        pub prefix: Option<crate::value::ExpString>,
        pub range: Option<Box<GatewayRouteRangeMatch_>>,
        pub regex: Option<crate::value::ExpString>,
        pub suffix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_GatewayRoute_GatewayRouteMetadataMatch {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::GatewayRoute.GatewayRouteMetadataMatch"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_GatewayRoute_GatewayRouteMetadataMatch as GatewayRouteMetadataMatch;
    impl crate::value::ToValue for GatewayRouteMetadataMatch_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.exact {
                properties.insert("Exact".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.prefix {
                properties.insert("Prefix".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.range {
                properties.insert("Range".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.regex {
                properties.insert("Regex".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.suffix {
                properties.insert("Suffix".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-gatewayrouterangematch.html
    pub struct GatewayRouteRangeMatch_ {
        pub end: i64,
        pub start: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_GatewayRoute_GatewayRouteRangeMatch {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::GatewayRoute.GatewayRouteRangeMatch"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_GatewayRoute_GatewayRouteRangeMatch as GatewayRouteRangeMatch;
    impl crate::value::ToValue for GatewayRouteRangeMatch_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "End".to_string(),
                crate::value::ToValue::to_value(&self.end),
            );
            properties.insert(
                "Start".to_string(),
                crate::value::ToValue::to_value(&self.start),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-gatewayroutespec.html
    pub struct GatewayRouteSpec_ {
        pub grpc_route: Option<Box<GrpcGatewayRoute_>>,
        pub http2_route: Option<Box<HttpGatewayRoute_>>,
        pub http_route: Option<Box<HttpGatewayRoute_>>,
        pub priority: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_GatewayRoute_GatewayRouteSpec {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::GatewayRoute.GatewayRouteSpec"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_GatewayRoute_GatewayRouteSpec as GatewayRouteSpec;
    impl crate::value::ToValue for GatewayRouteSpec_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.grpc_route {
                properties.insert(
                    "GrpcRoute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.http2_route {
                properties.insert(
                    "Http2Route".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.http_route {
                properties.insert(
                    "HttpRoute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.priority {
                properties.insert(
                    "Priority".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-gatewayroutetarget.html
    pub struct GatewayRouteTarget_ {
        pub port: Option<i64>,
        pub virtual_service: Box<GatewayRouteVirtualService_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_GatewayRoute_GatewayRouteTarget {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::GatewayRoute.GatewayRouteTarget"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_GatewayRoute_GatewayRouteTarget as GatewayRouteTarget;
    impl crate::value::ToValue for GatewayRouteTarget_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.port {
                properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "VirtualService".to_string(),
                crate::value::ToValue::to_value(&self.virtual_service),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-gatewayroutevirtualservice.html
    pub struct GatewayRouteVirtualService_ {
        pub virtual_service_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_GatewayRoute_GatewayRouteVirtualService {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::GatewayRoute.GatewayRouteVirtualService"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_GatewayRoute_GatewayRouteVirtualService as GatewayRouteVirtualService;
    impl crate::value::ToValue for GatewayRouteVirtualService_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "VirtualServiceName".to_string(),
                crate::value::ToValue::to_value(&self.virtual_service_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-grpcgatewayroute.html
    pub struct GrpcGatewayRoute_ {
        pub action: Box<GrpcGatewayRouteAction_>,
        pub r#match: Box<GrpcGatewayRouteMatch_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_GatewayRoute_GrpcGatewayRoute {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::GatewayRoute.GrpcGatewayRoute"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_GatewayRoute_GrpcGatewayRoute as GrpcGatewayRoute;
    impl crate::value::ToValue for GrpcGatewayRoute_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Action".to_string(),
                crate::value::ToValue::to_value(&self.action),
            );
            properties.insert(
                "Match".to_string(),
                crate::value::ToValue::to_value(&self.r#match),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-grpcgatewayrouteaction.html
    pub struct GrpcGatewayRouteAction_ {
        pub rewrite: Option<Box<GrpcGatewayRouteRewrite_>>,
        pub target: Box<GatewayRouteTarget_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_GatewayRoute_GrpcGatewayRouteAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::GatewayRoute.GrpcGatewayRouteAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_GatewayRoute_GrpcGatewayRouteAction as GrpcGatewayRouteAction;
    impl crate::value::ToValue for GrpcGatewayRouteAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.rewrite {
                properties.insert(
                    "Rewrite".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Target".to_string(),
                crate::value::ToValue::to_value(&self.target),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-grpcgatewayroutematch.html
    pub struct GrpcGatewayRouteMatch_ {
        pub hostname: Option<Box<GatewayRouteHostnameMatch_>>,
        pub metadata: Option<Vec<GrpcGatewayRouteMetadata_>>,
        pub port: Option<i64>,
        pub service_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_GatewayRoute_GrpcGatewayRouteMatch {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::GatewayRoute.GrpcGatewayRouteMatch"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_GatewayRoute_GrpcGatewayRouteMatch as GrpcGatewayRouteMatch;
    impl crate::value::ToValue for GrpcGatewayRouteMatch_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.hostname {
                properties.insert(
                    "Hostname".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metadata {
                properties.insert(
                    "Metadata".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.port {
                properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.service_name {
                properties.insert(
                    "ServiceName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-grpcgatewayroutemetadata.html
    pub struct GrpcGatewayRouteMetadata_ {
        pub invert: Option<crate::value::ExpBool>,
        pub r#match: Option<Box<GatewayRouteMetadataMatch_>>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_GatewayRoute_GrpcGatewayRouteMetadata {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::GatewayRoute.GrpcGatewayRouteMetadata"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_GatewayRoute_GrpcGatewayRouteMetadata as GrpcGatewayRouteMetadata;
    impl crate::value::ToValue for GrpcGatewayRouteMetadata_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.invert {
                properties.insert("Invert".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.r#match {
                properties.insert("Match".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-grpcgatewayrouterewrite.html
    pub struct GrpcGatewayRouteRewrite_ {
        pub hostname: Option<Box<GatewayRouteHostnameRewrite_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_GatewayRoute_GrpcGatewayRouteRewrite {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::GatewayRoute.GrpcGatewayRouteRewrite"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_GatewayRoute_GrpcGatewayRouteRewrite as GrpcGatewayRouteRewrite;
    impl crate::value::ToValue for GrpcGatewayRouteRewrite_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.hostname {
                properties.insert(
                    "Hostname".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayroute.html
    pub struct HttpGatewayRoute_ {
        pub action: Box<HttpGatewayRouteAction_>,
        pub r#match: Box<HttpGatewayRouteMatch_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_GatewayRoute_HttpGatewayRoute {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::GatewayRoute.HttpGatewayRoute"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_GatewayRoute_HttpGatewayRoute as HttpGatewayRoute;
    impl crate::value::ToValue for HttpGatewayRoute_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Action".to_string(),
                crate::value::ToValue::to_value(&self.action),
            );
            properties.insert(
                "Match".to_string(),
                crate::value::ToValue::to_value(&self.r#match),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayrouteaction.html
    pub struct HttpGatewayRouteAction_ {
        pub rewrite: Option<Box<HttpGatewayRouteRewrite_>>,
        pub target: Box<GatewayRouteTarget_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_GatewayRoute_HttpGatewayRouteAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::GatewayRoute.HttpGatewayRouteAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_GatewayRoute_HttpGatewayRouteAction as HttpGatewayRouteAction;
    impl crate::value::ToValue for HttpGatewayRouteAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.rewrite {
                properties.insert(
                    "Rewrite".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Target".to_string(),
                crate::value::ToValue::to_value(&self.target),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayrouteheader.html
    pub struct HttpGatewayRouteHeader_ {
        pub invert: Option<crate::value::ExpBool>,
        pub r#match: Option<Box<HttpGatewayRouteHeaderMatch_>>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_GatewayRoute_HttpGatewayRouteHeader {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::GatewayRoute.HttpGatewayRouteHeader"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_GatewayRoute_HttpGatewayRouteHeader as HttpGatewayRouteHeader;
    impl crate::value::ToValue for HttpGatewayRouteHeader_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.invert {
                properties.insert("Invert".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.r#match {
                properties.insert("Match".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayrouteheadermatch.html
    pub struct HttpGatewayRouteHeaderMatch_ {
        pub exact: Option<crate::value::ExpString>,
        pub prefix: Option<crate::value::ExpString>,
        pub range: Option<Box<GatewayRouteRangeMatch_>>,
        pub regex: Option<crate::value::ExpString>,
        pub suffix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_GatewayRoute_HttpGatewayRouteHeaderMatch {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::GatewayRoute.HttpGatewayRouteHeaderMatch"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_GatewayRoute_HttpGatewayRouteHeaderMatch as HttpGatewayRouteHeaderMatch;
    impl crate::value::ToValue for HttpGatewayRouteHeaderMatch_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.exact {
                properties.insert("Exact".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.prefix {
                properties.insert("Prefix".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.range {
                properties.insert("Range".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.regex {
                properties.insert("Regex".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.suffix {
                properties.insert("Suffix".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayroutematch.html
    pub struct HttpGatewayRouteMatch_ {
        pub headers: Option<Vec<HttpGatewayRouteHeader_>>,
        pub hostname: Option<Box<GatewayRouteHostnameMatch_>>,
        pub method: Option<crate::value::ExpString>,
        pub path: Option<Box<HttpPathMatch_>>,
        pub port: Option<i64>,
        pub prefix: Option<crate::value::ExpString>,
        pub query_parameters: Option<Vec<QueryParameter_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_GatewayRoute_HttpGatewayRouteMatch {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::GatewayRoute.HttpGatewayRouteMatch"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_GatewayRoute_HttpGatewayRouteMatch as HttpGatewayRouteMatch;
    impl crate::value::ToValue for HttpGatewayRouteMatch_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.headers {
                properties.insert(
                    "Headers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hostname {
                properties.insert(
                    "Hostname".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.method {
                properties.insert("Method".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.path {
                properties.insert("Path".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.port {
                properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.prefix {
                properties.insert("Prefix".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.query_parameters {
                properties.insert(
                    "QueryParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayroutepathrewrite.html
    pub struct HttpGatewayRoutePathRewrite_ {
        pub exact: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_GatewayRoute_HttpGatewayRoutePathRewrite {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::GatewayRoute.HttpGatewayRoutePathRewrite"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_GatewayRoute_HttpGatewayRoutePathRewrite as HttpGatewayRoutePathRewrite;
    impl crate::value::ToValue for HttpGatewayRoutePathRewrite_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.exact {
                properties.insert("Exact".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayrouteprefixrewrite.html
    pub struct HttpGatewayRoutePrefixRewrite_ {
        pub default_prefix: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_GatewayRoute_HttpGatewayRoutePrefixRewrite {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::GatewayRoute.HttpGatewayRoutePrefixRewrite"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_GatewayRoute_HttpGatewayRoutePrefixRewrite as HttpGatewayRoutePrefixRewrite;
    impl crate::value::ToValue for HttpGatewayRoutePrefixRewrite_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.default_prefix {
                properties.insert(
                    "DefaultPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpgatewayrouterewrite.html
    pub struct HttpGatewayRouteRewrite_ {
        pub hostname: Option<Box<GatewayRouteHostnameRewrite_>>,
        pub path: Option<Box<HttpGatewayRoutePathRewrite_>>,
        pub prefix: Option<Box<HttpGatewayRoutePrefixRewrite_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_GatewayRoute_HttpGatewayRouteRewrite {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::GatewayRoute.HttpGatewayRouteRewrite"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_GatewayRoute_HttpGatewayRouteRewrite as HttpGatewayRouteRewrite;
    impl crate::value::ToValue for HttpGatewayRouteRewrite_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.hostname {
                properties.insert(
                    "Hostname".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.path {
                properties.insert("Path".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.prefix {
                properties.insert("Prefix".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httppathmatch.html
    pub struct HttpPathMatch_ {
        pub exact: Option<crate::value::ExpString>,
        pub regex: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_GatewayRoute_HttpPathMatch {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::GatewayRoute.HttpPathMatch"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_GatewayRoute_HttpPathMatch as HttpPathMatch;
    impl crate::value::ToValue for HttpPathMatch_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.exact {
                properties.insert("Exact".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.regex {
                properties.insert("Regex".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-httpqueryparametermatch.html
    pub struct HttpQueryParameterMatch_ {
        pub exact: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_GatewayRoute_HttpQueryParameterMatch {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::GatewayRoute.HttpQueryParameterMatch"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_GatewayRoute_HttpQueryParameterMatch as HttpQueryParameterMatch;
    impl crate::value::ToValue for HttpQueryParameterMatch_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.exact {
                properties.insert("Exact".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-gatewayroute-queryparameter.html
    pub struct QueryParameter_ {
        pub r#match: Option<Box<HttpQueryParameterMatch_>>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_GatewayRoute_QueryParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::GatewayRoute.QueryParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_GatewayRoute_QueryParameter as QueryParameter;
    impl crate::value::ToValue for QueryParameter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.r#match {
                properties.insert("Match".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
}
pub mod mesh {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-mesh-egressfilter.html
    pub struct EgressFilter_ {
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_Mesh_EgressFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::Mesh.EgressFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_Mesh_EgressFilter as EgressFilter;
    impl crate::value::ToValue for EgressFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-mesh-meshservicediscovery.html
    pub struct MeshServiceDiscovery_ {
        pub ip_preference: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_Mesh_MeshServiceDiscovery {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::Mesh.MeshServiceDiscovery"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_Mesh_MeshServiceDiscovery as MeshServiceDiscovery;
    impl crate::value::ToValue for MeshServiceDiscovery_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ip_preference {
                properties.insert(
                    "IpPreference".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-mesh-meshspec.html
    pub struct MeshSpec_ {
        pub egress_filter: Option<Box<EgressFilter_>>,
        pub service_discovery: Option<Box<MeshServiceDiscovery_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_Mesh_MeshSpec {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::Mesh.MeshSpec"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_Mesh_MeshSpec as MeshSpec;
    impl crate::value::ToValue for MeshSpec_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.egress_filter {
                properties.insert(
                    "EgressFilter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.service_discovery {
                properties.insert(
                    "ServiceDiscovery".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod route {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-duration.html
    pub struct Duration_ {
        pub unit: crate::value::ExpString,
        pub value: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_Route_Duration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::Route.Duration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_Route_Duration as Duration;
    impl crate::value::ToValue for Duration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Unit".to_string(),
                crate::value::ToValue::to_value(&self.unit),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-grpcretrypolicy.html
    pub struct GrpcRetryPolicy_ {
        pub grpc_retry_events: Option<Vec<crate::value::ExpString>>,
        pub http_retry_events: Option<Vec<crate::value::ExpString>>,
        pub max_retries: i64,
        pub per_retry_timeout: Box<Duration_>,
        pub tcp_retry_events: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_Route_GrpcRetryPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::Route.GrpcRetryPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_Route_GrpcRetryPolicy as GrpcRetryPolicy;
    impl crate::value::ToValue for GrpcRetryPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.grpc_retry_events {
                properties.insert(
                    "GrpcRetryEvents".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.http_retry_events {
                properties.insert(
                    "HttpRetryEvents".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MaxRetries".to_string(),
                crate::value::ToValue::to_value(&self.max_retries),
            );
            properties.insert(
                "PerRetryTimeout".to_string(),
                crate::value::ToValue::to_value(&self.per_retry_timeout),
            );
            if let Some(ref value) = self.tcp_retry_events {
                properties.insert(
                    "TcpRetryEvents".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-grpcroute.html
    pub struct GrpcRoute_ {
        pub action: Box<GrpcRouteAction_>,
        pub r#match: Box<GrpcRouteMatch_>,
        pub retry_policy: Option<Box<GrpcRetryPolicy_>>,
        pub timeout: Option<Box<GrpcTimeout_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_Route_GrpcRoute {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::Route.GrpcRoute"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_Route_GrpcRoute as GrpcRoute;
    impl crate::value::ToValue for GrpcRoute_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Action".to_string(),
                crate::value::ToValue::to_value(&self.action),
            );
            properties.insert(
                "Match".to_string(),
                crate::value::ToValue::to_value(&self.r#match),
            );
            if let Some(ref value) = self.retry_policy {
                properties.insert(
                    "RetryPolicy".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-grpcrouteaction.html
    pub struct GrpcRouteAction_ {
        pub weighted_targets: Vec<WeightedTarget_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_Route_GrpcRouteAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::Route.GrpcRouteAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_Route_GrpcRouteAction as GrpcRouteAction;
    impl crate::value::ToValue for GrpcRouteAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "WeightedTargets".to_string(),
                crate::value::ToValue::to_value(&self.weighted_targets),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-grpcroutematch.html
    pub struct GrpcRouteMatch_ {
        pub metadata: Option<Vec<GrpcRouteMetadata_>>,
        pub method_name: Option<crate::value::ExpString>,
        pub port: Option<i64>,
        pub service_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_Route_GrpcRouteMatch {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::Route.GrpcRouteMatch"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_Route_GrpcRouteMatch as GrpcRouteMatch;
    impl crate::value::ToValue for GrpcRouteMatch_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.metadata {
                properties.insert(
                    "Metadata".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.method_name {
                properties.insert(
                    "MethodName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.port {
                properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.service_name {
                properties.insert(
                    "ServiceName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-grpcroutemetadata.html
    pub struct GrpcRouteMetadata_ {
        pub invert: Option<crate::value::ExpBool>,
        pub r#match: Option<Box<GrpcRouteMetadataMatchMethod_>>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_Route_GrpcRouteMetadata {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::Route.GrpcRouteMetadata"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_Route_GrpcRouteMetadata as GrpcRouteMetadata;
    impl crate::value::ToValue for GrpcRouteMetadata_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.invert {
                properties.insert("Invert".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.r#match {
                properties.insert("Match".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-grpcroutemetadatamatchmethod.html
    pub struct GrpcRouteMetadataMatchMethod_ {
        pub exact: Option<crate::value::ExpString>,
        pub prefix: Option<crate::value::ExpString>,
        pub range: Option<Box<MatchRange_>>,
        pub regex: Option<crate::value::ExpString>,
        pub suffix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_Route_GrpcRouteMetadataMatchMethod {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::Route.GrpcRouteMetadataMatchMethod"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_Route_GrpcRouteMetadataMatchMethod as GrpcRouteMetadataMatchMethod;
    impl crate::value::ToValue for GrpcRouteMetadataMatchMethod_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.exact {
                properties.insert("Exact".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.prefix {
                properties.insert("Prefix".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.range {
                properties.insert("Range".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.regex {
                properties.insert("Regex".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.suffix {
                properties.insert("Suffix".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-grpctimeout.html
    pub struct GrpcTimeout_ {
        pub idle: Option<Box<Duration_>>,
        pub per_request: Option<Box<Duration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_Route_GrpcTimeout {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::Route.GrpcTimeout"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_Route_GrpcTimeout as GrpcTimeout;
    impl crate::value::ToValue for GrpcTimeout_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.idle {
                properties.insert("Idle".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.per_request {
                properties.insert(
                    "PerRequest".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-headermatchmethod.html
    pub struct HeaderMatchMethod_ {
        pub exact: Option<crate::value::ExpString>,
        pub prefix: Option<crate::value::ExpString>,
        pub range: Option<Box<MatchRange_>>,
        pub regex: Option<crate::value::ExpString>,
        pub suffix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_Route_HeaderMatchMethod {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::Route.HeaderMatchMethod"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_Route_HeaderMatchMethod as HeaderMatchMethod;
    impl crate::value::ToValue for HeaderMatchMethod_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.exact {
                properties.insert("Exact".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.prefix {
                properties.insert("Prefix".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.range {
                properties.insert("Range".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.regex {
                properties.insert("Regex".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.suffix {
                properties.insert("Suffix".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-httppathmatch.html
    pub struct HttpPathMatch_ {
        pub exact: Option<crate::value::ExpString>,
        pub regex: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_Route_HttpPathMatch {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::Route.HttpPathMatch"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_Route_HttpPathMatch as HttpPathMatch;
    impl crate::value::ToValue for HttpPathMatch_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.exact {
                properties.insert("Exact".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.regex {
                properties.insert("Regex".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-httpqueryparametermatch.html
    pub struct HttpQueryParameterMatch_ {
        pub exact: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_Route_HttpQueryParameterMatch {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::Route.HttpQueryParameterMatch"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_Route_HttpQueryParameterMatch as HttpQueryParameterMatch;
    impl crate::value::ToValue for HttpQueryParameterMatch_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.exact {
                properties.insert("Exact".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-httpretrypolicy.html
    pub struct HttpRetryPolicy_ {
        pub http_retry_events: Option<Vec<crate::value::ExpString>>,
        pub max_retries: i64,
        pub per_retry_timeout: Box<Duration_>,
        pub tcp_retry_events: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_Route_HttpRetryPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::Route.HttpRetryPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_Route_HttpRetryPolicy as HttpRetryPolicy;
    impl crate::value::ToValue for HttpRetryPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.http_retry_events {
                properties.insert(
                    "HttpRetryEvents".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MaxRetries".to_string(),
                crate::value::ToValue::to_value(&self.max_retries),
            );
            properties.insert(
                "PerRetryTimeout".to_string(),
                crate::value::ToValue::to_value(&self.per_retry_timeout),
            );
            if let Some(ref value) = self.tcp_retry_events {
                properties.insert(
                    "TcpRetryEvents".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-httproute.html
    pub struct HttpRoute_ {
        pub action: Box<HttpRouteAction_>,
        pub r#match: Box<HttpRouteMatch_>,
        pub retry_policy: Option<Box<HttpRetryPolicy_>>,
        pub timeout: Option<Box<HttpTimeout_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_Route_HttpRoute {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::Route.HttpRoute"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_Route_HttpRoute as HttpRoute;
    impl crate::value::ToValue for HttpRoute_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Action".to_string(),
                crate::value::ToValue::to_value(&self.action),
            );
            properties.insert(
                "Match".to_string(),
                crate::value::ToValue::to_value(&self.r#match),
            );
            if let Some(ref value) = self.retry_policy {
                properties.insert(
                    "RetryPolicy".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-httprouteaction.html
    pub struct HttpRouteAction_ {
        pub weighted_targets: Vec<WeightedTarget_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_Route_HttpRouteAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::Route.HttpRouteAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_Route_HttpRouteAction as HttpRouteAction;
    impl crate::value::ToValue for HttpRouteAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "WeightedTargets".to_string(),
                crate::value::ToValue::to_value(&self.weighted_targets),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-httprouteheader.html
    pub struct HttpRouteHeader_ {
        pub invert: Option<crate::value::ExpBool>,
        pub r#match: Option<Box<HeaderMatchMethod_>>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_Route_HttpRouteHeader {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::Route.HttpRouteHeader"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_Route_HttpRouteHeader as HttpRouteHeader;
    impl crate::value::ToValue for HttpRouteHeader_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.invert {
                properties.insert("Invert".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.r#match {
                properties.insert("Match".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-httproutematch.html
    pub struct HttpRouteMatch_ {
        pub headers: Option<Vec<HttpRouteHeader_>>,
        pub method: Option<crate::value::ExpString>,
        pub path: Option<Box<HttpPathMatch_>>,
        pub port: Option<i64>,
        pub prefix: Option<crate::value::ExpString>,
        pub query_parameters: Option<Vec<QueryParameter_>>,
        pub scheme: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_Route_HttpRouteMatch {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::Route.HttpRouteMatch"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_Route_HttpRouteMatch as HttpRouteMatch;
    impl crate::value::ToValue for HttpRouteMatch_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.headers {
                properties.insert(
                    "Headers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.method {
                properties.insert("Method".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.path {
                properties.insert("Path".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.port {
                properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.prefix {
                properties.insert("Prefix".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.query_parameters {
                properties.insert(
                    "QueryParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scheme {
                properties.insert("Scheme".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-httptimeout.html
    pub struct HttpTimeout_ {
        pub idle: Option<Box<Duration_>>,
        pub per_request: Option<Box<Duration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_Route_HttpTimeout {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::Route.HttpTimeout"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_Route_HttpTimeout as HttpTimeout;
    impl crate::value::ToValue for HttpTimeout_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.idle {
                properties.insert("Idle".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.per_request {
                properties.insert(
                    "PerRequest".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-matchrange.html
    pub struct MatchRange_ {
        pub end: i64,
        pub start: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_Route_MatchRange {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::Route.MatchRange"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_Route_MatchRange as MatchRange;
    impl crate::value::ToValue for MatchRange_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "End".to_string(),
                crate::value::ToValue::to_value(&self.end),
            );
            properties.insert(
                "Start".to_string(),
                crate::value::ToValue::to_value(&self.start),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-queryparameter.html
    pub struct QueryParameter_ {
        pub r#match: Option<Box<HttpQueryParameterMatch_>>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_Route_QueryParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::Route.QueryParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_Route_QueryParameter as QueryParameter;
    impl crate::value::ToValue for QueryParameter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.r#match {
                properties.insert("Match".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-routespec.html
    pub struct RouteSpec_ {
        pub grpc_route: Option<Box<GrpcRoute_>>,
        pub http2_route: Option<Box<HttpRoute_>>,
        pub http_route: Option<Box<HttpRoute_>>,
        pub priority: Option<i64>,
        pub tcp_route: Option<Box<TcpRoute_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_Route_RouteSpec {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::Route.RouteSpec"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_Route_RouteSpec as RouteSpec;
    impl crate::value::ToValue for RouteSpec_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.grpc_route {
                properties.insert(
                    "GrpcRoute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.http2_route {
                properties.insert(
                    "Http2Route".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.http_route {
                properties.insert(
                    "HttpRoute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.priority {
                properties.insert(
                    "Priority".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tcp_route {
                properties.insert(
                    "TcpRoute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-tcproute.html
    pub struct TcpRoute_ {
        pub action: Box<TcpRouteAction_>,
        pub r#match: Option<Box<TcpRouteMatch_>>,
        pub timeout: Option<Box<TcpTimeout_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_Route_TcpRoute {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::Route.TcpRoute"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_Route_TcpRoute as TcpRoute;
    impl crate::value::ToValue for TcpRoute_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Action".to_string(),
                crate::value::ToValue::to_value(&self.action),
            );
            if let Some(ref value) = self.r#match {
                properties.insert("Match".to_string(), crate::value::ToValue::to_value(value));
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-tcprouteaction.html
    pub struct TcpRouteAction_ {
        pub weighted_targets: Vec<WeightedTarget_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_Route_TcpRouteAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::Route.TcpRouteAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_Route_TcpRouteAction as TcpRouteAction;
    impl crate::value::ToValue for TcpRouteAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "WeightedTargets".to_string(),
                crate::value::ToValue::to_value(&self.weighted_targets),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-tcproutematch.html
    pub struct TcpRouteMatch_ {
        pub port: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_Route_TcpRouteMatch {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::Route.TcpRouteMatch"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_Route_TcpRouteMatch as TcpRouteMatch;
    impl crate::value::ToValue for TcpRouteMatch_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.port {
                properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-tcptimeout.html
    pub struct TcpTimeout_ {
        pub idle: Option<Box<Duration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_Route_TcpTimeout {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::Route.TcpTimeout"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_Route_TcpTimeout as TcpTimeout;
    impl crate::value::ToValue for TcpTimeout_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.idle {
                properties.insert("Idle".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-route-weightedtarget.html
    pub struct WeightedTarget_ {
        pub port: Option<i64>,
        pub virtual_node: crate::value::ExpString,
        pub weight: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_Route_WeightedTarget {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::Route.WeightedTarget"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_Route_WeightedTarget as WeightedTarget;
    impl crate::value::ToValue for WeightedTarget_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.port {
                properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "VirtualNode".to_string(),
                crate::value::ToValue::to_value(&self.virtual_node),
            );
            properties.insert(
                "Weight".to_string(),
                crate::value::ToValue::to_value(&self.weight),
            );
            properties.into()
        }
    }
}
pub mod virtualgateway {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-jsonformatref.html
    pub struct JsonFormatRef_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualGateway_JsonFormatRef {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualGateway.JsonFormatRef"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualGateway_JsonFormatRef as JsonFormatRef;
    impl crate::value::ToValue for JsonFormatRef_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-loggingformat.html
    pub struct LoggingFormat_ {
        pub json: Option<Vec<JsonFormatRef_>>,
        pub text: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualGateway_LoggingFormat {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualGateway.LoggingFormat"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualGateway_LoggingFormat as LoggingFormat;
    impl crate::value::ToValue for LoggingFormat_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.json {
                properties.insert("Json".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.text {
                properties.insert("Text".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-subjectalternativenamematchers.html
    pub struct SubjectAlternativeNameMatchers_ {
        pub exact: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualGateway_SubjectAlternativeNameMatchers {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualGateway.SubjectAlternativeNameMatchers"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualGateway_SubjectAlternativeNameMatchers as SubjectAlternativeNameMatchers;
    impl crate::value::ToValue for SubjectAlternativeNameMatchers_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.exact {
                properties.insert("Exact".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-subjectalternativenames.html
    pub struct SubjectAlternativeNames_ {
        pub r#match: Box<SubjectAlternativeNameMatchers_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualGateway_SubjectAlternativeNames {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualGateway.SubjectAlternativeNames"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualGateway_SubjectAlternativeNames as SubjectAlternativeNames;
    impl crate::value::ToValue for SubjectAlternativeNames_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Match".to_string(),
                crate::value::ToValue::to_value(&self.r#match),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayaccesslog.html
    pub struct VirtualGatewayAccessLog_ {
        pub file: Option<Box<VirtualGatewayFileAccessLog_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualGateway_VirtualGatewayAccessLog {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualGateway.VirtualGatewayAccessLog"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualGateway_VirtualGatewayAccessLog as VirtualGatewayAccessLog;
    impl crate::value::ToValue for VirtualGatewayAccessLog_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.file {
                properties.insert("File".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaybackenddefaults.html
    pub struct VirtualGatewayBackendDefaults_ {
        pub client_policy: Option<Box<VirtualGatewayClientPolicy_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualGateway_VirtualGatewayBackendDefaults {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualGateway.VirtualGatewayBackendDefaults"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualGateway_VirtualGatewayBackendDefaults as VirtualGatewayBackendDefaults;
    impl crate::value::ToValue for VirtualGatewayBackendDefaults_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.client_policy {
                properties.insert(
                    "ClientPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayclientpolicy.html
    pub struct VirtualGatewayClientPolicy_ {
        pub tls: Option<Box<VirtualGatewayClientPolicyTls_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualGateway_VirtualGatewayClientPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualGateway.VirtualGatewayClientPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualGateway_VirtualGatewayClientPolicy as VirtualGatewayClientPolicy;
    impl crate::value::ToValue for VirtualGatewayClientPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.tls {
                properties.insert("TLS".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayclientpolicytls.html
    pub struct VirtualGatewayClientPolicyTls_ {
        pub certificate: Option<Box<VirtualGatewayClientTlsCertificate_>>,
        pub enforce: Option<crate::value::ExpBool>,
        pub ports: Option<Vec<i64>>,
        pub validation: Box<VirtualGatewayTlsValidationContext_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualGateway_VirtualGatewayClientPolicyTls {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualGateway.VirtualGatewayClientPolicyTls"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualGateway_VirtualGatewayClientPolicyTls as VirtualGatewayClientPolicyTls;
    impl crate::value::ToValue for VirtualGatewayClientPolicyTls_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.certificate {
                properties.insert(
                    "Certificate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enforce {
                properties.insert(
                    "Enforce".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ports {
                properties.insert("Ports".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Validation".to_string(),
                crate::value::ToValue::to_value(&self.validation),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayclienttlscertificate.html
    pub struct VirtualGatewayClientTlsCertificate_ {
        pub file: Option<Box<VirtualGatewayListenerTlsFileCertificate_>>,
        pub sds: Option<Box<VirtualGatewayListenerTlsSdsCertificate_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualGateway_VirtualGatewayClientTlsCertificate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualGateway.VirtualGatewayClientTlsCertificate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualGateway_VirtualGatewayClientTlsCertificate as VirtualGatewayClientTlsCertificate;
    impl crate::value::ToValue for VirtualGatewayClientTlsCertificate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.file {
                properties.insert("File".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.sds {
                properties.insert("SDS".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayconnectionpool.html
    pub struct VirtualGatewayConnectionPool_ {
        pub grpc: Option<Box<VirtualGatewayGrpcConnectionPool_>>,
        pub http: Option<Box<VirtualGatewayHttpConnectionPool_>>,
        pub http2: Option<Box<VirtualGatewayHttp2ConnectionPool_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualGateway_VirtualGatewayConnectionPool {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualGateway.VirtualGatewayConnectionPool"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualGateway_VirtualGatewayConnectionPool as VirtualGatewayConnectionPool;
    impl crate::value::ToValue for VirtualGatewayConnectionPool_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.grpc {
                properties.insert("GRPC".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.http {
                properties.insert("HTTP".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.http2 {
                properties.insert("HTTP2".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayfileaccesslog.html
    pub struct VirtualGatewayFileAccessLog_ {
        pub format: Option<Box<LoggingFormat_>>,
        pub path: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualGateway_VirtualGatewayFileAccessLog {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualGateway.VirtualGatewayFileAccessLog"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualGateway_VirtualGatewayFileAccessLog as VirtualGatewayFileAccessLog;
    impl crate::value::ToValue for VirtualGatewayFileAccessLog_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.format {
                properties.insert("Format".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Path".to_string(),
                crate::value::ToValue::to_value(&self.path),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaygrpcconnectionpool.html
    pub struct VirtualGatewayGrpcConnectionPool_ {
        pub max_requests: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualGateway_VirtualGatewayGrpcConnectionPool {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualGateway.VirtualGatewayGrpcConnectionPool"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualGateway_VirtualGatewayGrpcConnectionPool as VirtualGatewayGrpcConnectionPool;
    impl crate::value::ToValue for VirtualGatewayGrpcConnectionPool_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MaxRequests".to_string(),
                crate::value::ToValue::to_value(&self.max_requests),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayhealthcheckpolicy.html
    pub struct VirtualGatewayHealthCheckPolicy_ {
        pub healthy_threshold: i64,
        pub interval_millis: i64,
        pub path: Option<crate::value::ExpString>,
        pub port: Option<i64>,
        pub protocol: crate::value::ExpString,
        pub timeout_millis: i64,
        pub unhealthy_threshold: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualGateway_VirtualGatewayHealthCheckPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualGateway.VirtualGatewayHealthCheckPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualGateway_VirtualGatewayHealthCheckPolicy as VirtualGatewayHealthCheckPolicy;
    impl crate::value::ToValue for VirtualGatewayHealthCheckPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "HealthyThreshold".to_string(),
                crate::value::ToValue::to_value(&self.healthy_threshold),
            );
            properties.insert(
                "IntervalMillis".to_string(),
                crate::value::ToValue::to_value(&self.interval_millis),
            );
            if let Some(ref value) = self.path {
                properties.insert("Path".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.port {
                properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Protocol".to_string(),
                crate::value::ToValue::to_value(&self.protocol),
            );
            properties.insert(
                "TimeoutMillis".to_string(),
                crate::value::ToValue::to_value(&self.timeout_millis),
            );
            properties.insert(
                "UnhealthyThreshold".to_string(),
                crate::value::ToValue::to_value(&self.unhealthy_threshold),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayhttp2connectionpool.html
    pub struct VirtualGatewayHttp2ConnectionPool_ {
        pub max_requests: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualGateway_VirtualGatewayHttp2ConnectionPool {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualGateway.VirtualGatewayHttp2ConnectionPool"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualGateway_VirtualGatewayHttp2ConnectionPool as VirtualGatewayHttp2ConnectionPool;
    impl crate::value::ToValue for VirtualGatewayHttp2ConnectionPool_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MaxRequests".to_string(),
                crate::value::ToValue::to_value(&self.max_requests),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayhttpconnectionpool.html
    pub struct VirtualGatewayHttpConnectionPool_ {
        pub max_connections: i64,
        pub max_pending_requests: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualGateway_VirtualGatewayHttpConnectionPool {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualGateway.VirtualGatewayHttpConnectionPool"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualGateway_VirtualGatewayHttpConnectionPool as VirtualGatewayHttpConnectionPool;
    impl crate::value::ToValue for VirtualGatewayHttpConnectionPool_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MaxConnections".to_string(),
                crate::value::ToValue::to_value(&self.max_connections),
            );
            if let Some(ref value) = self.max_pending_requests {
                properties.insert(
                    "MaxPendingRequests".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaylistener.html
    pub struct VirtualGatewayListener_ {
        pub connection_pool: Option<Box<VirtualGatewayConnectionPool_>>,
        pub health_check: Option<Box<VirtualGatewayHealthCheckPolicy_>>,
        pub port_mapping: Box<VirtualGatewayPortMapping_>,
        pub tls: Option<Box<VirtualGatewayListenerTls_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualGateway_VirtualGatewayListener {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualGateway.VirtualGatewayListener"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualGateway_VirtualGatewayListener as VirtualGatewayListener;
    impl crate::value::ToValue for VirtualGatewayListener_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.connection_pool {
                properties.insert(
                    "ConnectionPool".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.health_check {
                properties.insert(
                    "HealthCheck".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "PortMapping".to_string(),
                crate::value::ToValue::to_value(&self.port_mapping),
            );
            if let Some(ref value) = self.tls {
                properties.insert("TLS".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaylistenertls.html
    pub struct VirtualGatewayListenerTls_ {
        pub certificate: Box<VirtualGatewayListenerTlsCertificate_>,
        pub mode: crate::value::ExpString,
        pub validation: Option<Box<VirtualGatewayListenerTlsValidationContext_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualGateway_VirtualGatewayListenerTls {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualGateway.VirtualGatewayListenerTls"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualGateway_VirtualGatewayListenerTls as VirtualGatewayListenerTls;
    impl crate::value::ToValue for VirtualGatewayListenerTls_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Certificate".to_string(),
                crate::value::ToValue::to_value(&self.certificate),
            );
            properties.insert(
                "Mode".to_string(),
                crate::value::ToValue::to_value(&self.mode),
            );
            if let Some(ref value) = self.validation {
                properties.insert(
                    "Validation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaylistenertlsacmcertificate.html
    pub struct VirtualGatewayListenerTlsAcmCertificate_ {
        pub certificate_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualGateway_VirtualGatewayListenerTlsAcmCertificate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualGateway.VirtualGatewayListenerTlsAcmCertificate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualGateway_VirtualGatewayListenerTlsAcmCertificate as VirtualGatewayListenerTlsAcmCertificate;
    impl crate::value::ToValue for VirtualGatewayListenerTlsAcmCertificate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CertificateArn".to_string(),
                crate::value::ToValue::to_value(&self.certificate_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaylistenertlscertificate.html
    pub struct VirtualGatewayListenerTlsCertificate_ {
        pub acm: Option<Box<VirtualGatewayListenerTlsAcmCertificate_>>,
        pub file: Option<Box<VirtualGatewayListenerTlsFileCertificate_>>,
        pub sds: Option<Box<VirtualGatewayListenerTlsSdsCertificate_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualGateway_VirtualGatewayListenerTlsCertificate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualGateway.VirtualGatewayListenerTlsCertificate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualGateway_VirtualGatewayListenerTlsCertificate as VirtualGatewayListenerTlsCertificate;
    impl crate::value::ToValue for VirtualGatewayListenerTlsCertificate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.acm {
                properties.insert("ACM".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.file {
                properties.insert("File".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.sds {
                properties.insert("SDS".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaylistenertlsfilecertificate.html
    pub struct VirtualGatewayListenerTlsFileCertificate_ {
        pub certificate_chain: crate::value::ExpString,
        pub private_key: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualGateway_VirtualGatewayListenerTlsFileCertificate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualGateway.VirtualGatewayListenerTlsFileCertificate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualGateway_VirtualGatewayListenerTlsFileCertificate as VirtualGatewayListenerTlsFileCertificate;
    impl crate::value::ToValue for VirtualGatewayListenerTlsFileCertificate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CertificateChain".to_string(),
                crate::value::ToValue::to_value(&self.certificate_chain),
            );
            properties.insert(
                "PrivateKey".to_string(),
                crate::value::ToValue::to_value(&self.private_key),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaylistenertlssdscertificate.html
    pub struct VirtualGatewayListenerTlsSdsCertificate_ {
        pub secret_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualGateway_VirtualGatewayListenerTlsSdsCertificate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualGateway.VirtualGatewayListenerTlsSdsCertificate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualGateway_VirtualGatewayListenerTlsSdsCertificate as VirtualGatewayListenerTlsSdsCertificate;
    impl crate::value::ToValue for VirtualGatewayListenerTlsSdsCertificate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SecretName".to_string(),
                crate::value::ToValue::to_value(&self.secret_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaylistenertlsvalidationcontext.html
    pub struct VirtualGatewayListenerTlsValidationContext_ {
        pub subject_alternative_names: Option<Box<SubjectAlternativeNames_>>,
        pub trust: Box<VirtualGatewayListenerTlsValidationContextTrust_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualGateway_VirtualGatewayListenerTlsValidationContext {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualGateway.VirtualGatewayListenerTlsValidationContext"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualGateway_VirtualGatewayListenerTlsValidationContext as VirtualGatewayListenerTlsValidationContext;
    impl crate::value::ToValue for VirtualGatewayListenerTlsValidationContext_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.subject_alternative_names {
                properties.insert(
                    "SubjectAlternativeNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Trust".to_string(),
                crate::value::ToValue::to_value(&self.trust),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaylistenertlsvalidationcontexttrust.html
    pub struct VirtualGatewayListenerTlsValidationContextTrust_ {
        pub file: Option<Box<VirtualGatewayTlsValidationContextFileTrust_>>,
        pub sds: Option<Box<VirtualGatewayTlsValidationContextSdsTrust_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualGateway_VirtualGatewayListenerTlsValidationContextTrust {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualGateway.VirtualGatewayListenerTlsValidationContextTrust"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualGateway_VirtualGatewayListenerTlsValidationContextTrust as VirtualGatewayListenerTlsValidationContextTrust;
    impl crate::value::ToValue for VirtualGatewayListenerTlsValidationContextTrust_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.file {
                properties.insert("File".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.sds {
                properties.insert("SDS".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaylogging.html
    pub struct VirtualGatewayLogging_ {
        pub access_log: Option<Box<VirtualGatewayAccessLog_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualGateway_VirtualGatewayLogging {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualGateway.VirtualGatewayLogging"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualGateway_VirtualGatewayLogging as VirtualGatewayLogging;
    impl crate::value::ToValue for VirtualGatewayLogging_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.access_log {
                properties.insert(
                    "AccessLog".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayportmapping.html
    pub struct VirtualGatewayPortMapping_ {
        pub port: i64,
        pub protocol: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualGateway_VirtualGatewayPortMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualGateway.VirtualGatewayPortMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualGateway_VirtualGatewayPortMapping as VirtualGatewayPortMapping;
    impl crate::value::ToValue for VirtualGatewayPortMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Port".to_string(),
                crate::value::ToValue::to_value(&self.port),
            );
            properties.insert(
                "Protocol".to_string(),
                crate::value::ToValue::to_value(&self.protocol),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewayspec.html
    pub struct VirtualGatewaySpec_ {
        pub backend_defaults: Option<Box<VirtualGatewayBackendDefaults_>>,
        pub listeners: Vec<VirtualGatewayListener_>,
        pub logging: Option<Box<VirtualGatewayLogging_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualGateway_VirtualGatewaySpec {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualGateway.VirtualGatewaySpec"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualGateway_VirtualGatewaySpec as VirtualGatewaySpec;
    impl crate::value::ToValue for VirtualGatewaySpec_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.backend_defaults {
                properties.insert(
                    "BackendDefaults".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Listeners".to_string(),
                crate::value::ToValue::to_value(&self.listeners),
            );
            if let Some(ref value) = self.logging {
                properties.insert(
                    "Logging".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaytlsvalidationcontext.html
    pub struct VirtualGatewayTlsValidationContext_ {
        pub subject_alternative_names: Option<Box<SubjectAlternativeNames_>>,
        pub trust: Box<VirtualGatewayTlsValidationContextTrust_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualGateway_VirtualGatewayTlsValidationContext {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualGateway.VirtualGatewayTlsValidationContext"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualGateway_VirtualGatewayTlsValidationContext as VirtualGatewayTlsValidationContext;
    impl crate::value::ToValue for VirtualGatewayTlsValidationContext_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.subject_alternative_names {
                properties.insert(
                    "SubjectAlternativeNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Trust".to_string(),
                crate::value::ToValue::to_value(&self.trust),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaytlsvalidationcontextacmtrust.html
    pub struct VirtualGatewayTlsValidationContextAcmTrust_ {
        pub certificate_authority_arns: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualGateway_VirtualGatewayTlsValidationContextAcmTrust {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualGateway.VirtualGatewayTlsValidationContextAcmTrust"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualGateway_VirtualGatewayTlsValidationContextAcmTrust as VirtualGatewayTlsValidationContextAcmTrust;
    impl crate::value::ToValue for VirtualGatewayTlsValidationContextAcmTrust_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CertificateAuthorityArns".to_string(),
                crate::value::ToValue::to_value(&self.certificate_authority_arns),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaytlsvalidationcontextfiletrust.html
    pub struct VirtualGatewayTlsValidationContextFileTrust_ {
        pub certificate_chain: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualGateway_VirtualGatewayTlsValidationContextFileTrust {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualGateway.VirtualGatewayTlsValidationContextFileTrust"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualGateway_VirtualGatewayTlsValidationContextFileTrust as VirtualGatewayTlsValidationContextFileTrust;
    impl crate::value::ToValue for VirtualGatewayTlsValidationContextFileTrust_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CertificateChain".to_string(),
                crate::value::ToValue::to_value(&self.certificate_chain),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaytlsvalidationcontextsdstrust.html
    pub struct VirtualGatewayTlsValidationContextSdsTrust_ {
        pub secret_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualGateway_VirtualGatewayTlsValidationContextSdsTrust {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualGateway.VirtualGatewayTlsValidationContextSdsTrust"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualGateway_VirtualGatewayTlsValidationContextSdsTrust as VirtualGatewayTlsValidationContextSdsTrust;
    impl crate::value::ToValue for VirtualGatewayTlsValidationContextSdsTrust_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SecretName".to_string(),
                crate::value::ToValue::to_value(&self.secret_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualgateway-virtualgatewaytlsvalidationcontexttrust.html
    pub struct VirtualGatewayTlsValidationContextTrust_ {
        pub acm: Option<Box<VirtualGatewayTlsValidationContextAcmTrust_>>,
        pub file: Option<Box<VirtualGatewayTlsValidationContextFileTrust_>>,
        pub sds: Option<Box<VirtualGatewayTlsValidationContextSdsTrust_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualGateway_VirtualGatewayTlsValidationContextTrust {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualGateway.VirtualGatewayTlsValidationContextTrust"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualGateway_VirtualGatewayTlsValidationContextTrust as VirtualGatewayTlsValidationContextTrust;
    impl crate::value::ToValue for VirtualGatewayTlsValidationContextTrust_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.acm {
                properties.insert("ACM".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.file {
                properties.insert("File".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.sds {
                properties.insert("SDS".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod virtualnode {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-accesslog.html
    pub struct AccessLog_ {
        pub file: Option<Box<FileAccessLog_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_AccessLog {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.AccessLog"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_AccessLog as AccessLog;
    impl crate::value::ToValue for AccessLog_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.file {
                properties.insert("File".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-awscloudmapinstanceattribute.html
    pub struct AwsCloudMapInstanceAttribute_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_AwsCloudMapInstanceAttribute {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.AwsCloudMapInstanceAttribute"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_AwsCloudMapInstanceAttribute as AwsCloudMapInstanceAttribute;
    impl crate::value::ToValue for AwsCloudMapInstanceAttribute_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-awscloudmapservicediscovery.html
    pub struct AwsCloudMapServiceDiscovery_ {
        pub attributes: Option<Vec<AwsCloudMapInstanceAttribute_>>,
        pub ip_preference: Option<crate::value::ExpString>,
        pub namespace_name: crate::value::ExpString,
        pub service_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_AwsCloudMapServiceDiscovery {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.AwsCloudMapServiceDiscovery"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_AwsCloudMapServiceDiscovery as AwsCloudMapServiceDiscovery;
    impl crate::value::ToValue for AwsCloudMapServiceDiscovery_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attributes {
                properties.insert(
                    "Attributes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ip_preference {
                properties.insert(
                    "IpPreference".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "NamespaceName".to_string(),
                crate::value::ToValue::to_value(&self.namespace_name),
            );
            properties.insert(
                "ServiceName".to_string(),
                crate::value::ToValue::to_value(&self.service_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-backend.html
    pub struct Backend_ {
        pub virtual_service: Option<Box<VirtualServiceBackend_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_Backend {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.Backend"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_Backend as Backend;
    impl crate::value::ToValue for Backend_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.virtual_service {
                properties.insert(
                    "VirtualService".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-backenddefaults.html
    pub struct BackendDefaults_ {
        pub client_policy: Option<Box<ClientPolicy_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_BackendDefaults {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.BackendDefaults"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_BackendDefaults as BackendDefaults;
    impl crate::value::ToValue for BackendDefaults_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.client_policy {
                properties.insert(
                    "ClientPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-clientpolicy.html
    pub struct ClientPolicy_ {
        pub tls: Option<Box<ClientPolicyTls_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_ClientPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.ClientPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_ClientPolicy as ClientPolicy;
    impl crate::value::ToValue for ClientPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.tls {
                properties.insert("TLS".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-clientpolicytls.html
    pub struct ClientPolicyTls_ {
        pub certificate: Option<Box<ClientTlsCertificate_>>,
        pub enforce: Option<crate::value::ExpBool>,
        pub ports: Option<Vec<i64>>,
        pub validation: Box<TlsValidationContext_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_ClientPolicyTls {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.ClientPolicyTls"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_ClientPolicyTls as ClientPolicyTls;
    impl crate::value::ToValue for ClientPolicyTls_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.certificate {
                properties.insert(
                    "Certificate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enforce {
                properties.insert(
                    "Enforce".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ports {
                properties.insert("Ports".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Validation".to_string(),
                crate::value::ToValue::to_value(&self.validation),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-clienttlscertificate.html
    pub struct ClientTlsCertificate_ {
        pub file: Option<Box<ListenerTlsFileCertificate_>>,
        pub sds: Option<Box<ListenerTlsSdsCertificate_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_ClientTlsCertificate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.ClientTlsCertificate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_ClientTlsCertificate as ClientTlsCertificate;
    impl crate::value::ToValue for ClientTlsCertificate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.file {
                properties.insert("File".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.sds {
                properties.insert("SDS".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-dnsservicediscovery.html
    pub struct DnsServiceDiscovery_ {
        pub hostname: crate::value::ExpString,
        pub ip_preference: Option<crate::value::ExpString>,
        pub response_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_DnsServiceDiscovery {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.DnsServiceDiscovery"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_DnsServiceDiscovery as DnsServiceDiscovery;
    impl crate::value::ToValue for DnsServiceDiscovery_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Hostname".to_string(),
                crate::value::ToValue::to_value(&self.hostname),
            );
            if let Some(ref value) = self.ip_preference {
                properties.insert(
                    "IpPreference".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.response_type {
                properties.insert(
                    "ResponseType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-duration.html
    pub struct Duration_ {
        pub unit: crate::value::ExpString,
        pub value: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_Duration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.Duration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_Duration as Duration;
    impl crate::value::ToValue for Duration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Unit".to_string(),
                crate::value::ToValue::to_value(&self.unit),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-fileaccesslog.html
    pub struct FileAccessLog_ {
        pub format: Option<Box<LoggingFormat_>>,
        pub path: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_FileAccessLog {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.FileAccessLog"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_FileAccessLog as FileAccessLog;
    impl crate::value::ToValue for FileAccessLog_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.format {
                properties.insert("Format".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Path".to_string(),
                crate::value::ToValue::to_value(&self.path),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-grpctimeout.html
    pub struct GrpcTimeout_ {
        pub idle: Option<Box<Duration_>>,
        pub per_request: Option<Box<Duration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_GrpcTimeout {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.GrpcTimeout"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_GrpcTimeout as GrpcTimeout;
    impl crate::value::ToValue for GrpcTimeout_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.idle {
                properties.insert("Idle".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.per_request {
                properties.insert(
                    "PerRequest".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-healthcheck.html
    pub struct HealthCheck_ {
        pub healthy_threshold: i64,
        pub interval_millis: i64,
        pub path: Option<crate::value::ExpString>,
        pub port: Option<i64>,
        pub protocol: crate::value::ExpString,
        pub timeout_millis: i64,
        pub unhealthy_threshold: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_HealthCheck {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.HealthCheck"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_HealthCheck as HealthCheck;
    impl crate::value::ToValue for HealthCheck_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "HealthyThreshold".to_string(),
                crate::value::ToValue::to_value(&self.healthy_threshold),
            );
            properties.insert(
                "IntervalMillis".to_string(),
                crate::value::ToValue::to_value(&self.interval_millis),
            );
            if let Some(ref value) = self.path {
                properties.insert("Path".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.port {
                properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Protocol".to_string(),
                crate::value::ToValue::to_value(&self.protocol),
            );
            properties.insert(
                "TimeoutMillis".to_string(),
                crate::value::ToValue::to_value(&self.timeout_millis),
            );
            properties.insert(
                "UnhealthyThreshold".to_string(),
                crate::value::ToValue::to_value(&self.unhealthy_threshold),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-httptimeout.html
    pub struct HttpTimeout_ {
        pub idle: Option<Box<Duration_>>,
        pub per_request: Option<Box<Duration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_HttpTimeout {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.HttpTimeout"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_HttpTimeout as HttpTimeout;
    impl crate::value::ToValue for HttpTimeout_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.idle {
                properties.insert("Idle".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.per_request {
                properties.insert(
                    "PerRequest".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-jsonformatref.html
    pub struct JsonFormatRef_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_JsonFormatRef {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.JsonFormatRef"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_JsonFormatRef as JsonFormatRef;
    impl crate::value::ToValue for JsonFormatRef_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listener.html
    pub struct Listener_ {
        pub connection_pool: Option<Box<VirtualNodeConnectionPool_>>,
        pub health_check: Option<Box<HealthCheck_>>,
        pub outlier_detection: Option<Box<OutlierDetection_>>,
        pub port_mapping: Box<PortMapping_>,
        pub tls: Option<Box<ListenerTls_>>,
        pub timeout: Option<Box<ListenerTimeout_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_Listener {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.Listener"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_Listener as Listener;
    impl crate::value::ToValue for Listener_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.connection_pool {
                properties.insert(
                    "ConnectionPool".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.health_check {
                properties.insert(
                    "HealthCheck".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.outlier_detection {
                properties.insert(
                    "OutlierDetection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "PortMapping".to_string(),
                crate::value::ToValue::to_value(&self.port_mapping),
            );
            if let Some(ref value) = self.tls {
                properties.insert("TLS".to_string(), crate::value::ToValue::to_value(value));
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listenertimeout.html
    pub struct ListenerTimeout_ {
        pub grpc: Option<Box<GrpcTimeout_>>,
        pub http: Option<Box<HttpTimeout_>>,
        pub http2: Option<Box<HttpTimeout_>>,
        pub tcp: Option<Box<TcpTimeout_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_ListenerTimeout {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.ListenerTimeout"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_ListenerTimeout as ListenerTimeout;
    impl crate::value::ToValue for ListenerTimeout_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.grpc {
                properties.insert("GRPC".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.http {
                properties.insert("HTTP".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.http2 {
                properties.insert("HTTP2".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.tcp {
                properties.insert("TCP".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listenertls.html
    pub struct ListenerTls_ {
        pub certificate: Box<ListenerTlsCertificate_>,
        pub mode: crate::value::ExpString,
        pub validation: Option<Box<ListenerTlsValidationContext_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_ListenerTls {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.ListenerTls"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_ListenerTls as ListenerTls;
    impl crate::value::ToValue for ListenerTls_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Certificate".to_string(),
                crate::value::ToValue::to_value(&self.certificate),
            );
            properties.insert(
                "Mode".to_string(),
                crate::value::ToValue::to_value(&self.mode),
            );
            if let Some(ref value) = self.validation {
                properties.insert(
                    "Validation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listenertlsacmcertificate.html
    pub struct ListenerTlsAcmCertificate_ {
        pub certificate_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_ListenerTlsAcmCertificate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.ListenerTlsAcmCertificate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_ListenerTlsAcmCertificate as ListenerTlsAcmCertificate;
    impl crate::value::ToValue for ListenerTlsAcmCertificate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CertificateArn".to_string(),
                crate::value::ToValue::to_value(&self.certificate_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listenertlscertificate.html
    pub struct ListenerTlsCertificate_ {
        pub acm: Option<Box<ListenerTlsAcmCertificate_>>,
        pub file: Option<Box<ListenerTlsFileCertificate_>>,
        pub sds: Option<Box<ListenerTlsSdsCertificate_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_ListenerTlsCertificate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.ListenerTlsCertificate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_ListenerTlsCertificate as ListenerTlsCertificate;
    impl crate::value::ToValue for ListenerTlsCertificate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.acm {
                properties.insert("ACM".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.file {
                properties.insert("File".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.sds {
                properties.insert("SDS".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listenertlsfilecertificate.html
    pub struct ListenerTlsFileCertificate_ {
        pub certificate_chain: crate::value::ExpString,
        pub private_key: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_ListenerTlsFileCertificate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.ListenerTlsFileCertificate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_ListenerTlsFileCertificate as ListenerTlsFileCertificate;
    impl crate::value::ToValue for ListenerTlsFileCertificate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CertificateChain".to_string(),
                crate::value::ToValue::to_value(&self.certificate_chain),
            );
            properties.insert(
                "PrivateKey".to_string(),
                crate::value::ToValue::to_value(&self.private_key),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listenertlssdscertificate.html
    pub struct ListenerTlsSdsCertificate_ {
        pub secret_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_ListenerTlsSdsCertificate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.ListenerTlsSdsCertificate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_ListenerTlsSdsCertificate as ListenerTlsSdsCertificate;
    impl crate::value::ToValue for ListenerTlsSdsCertificate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SecretName".to_string(),
                crate::value::ToValue::to_value(&self.secret_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listenertlsvalidationcontext.html
    pub struct ListenerTlsValidationContext_ {
        pub subject_alternative_names: Option<Box<SubjectAlternativeNames_>>,
        pub trust: Box<ListenerTlsValidationContextTrust_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_ListenerTlsValidationContext {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.ListenerTlsValidationContext"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_ListenerTlsValidationContext as ListenerTlsValidationContext;
    impl crate::value::ToValue for ListenerTlsValidationContext_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.subject_alternative_names {
                properties.insert(
                    "SubjectAlternativeNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Trust".to_string(),
                crate::value::ToValue::to_value(&self.trust),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-listenertlsvalidationcontexttrust.html
    pub struct ListenerTlsValidationContextTrust_ {
        pub file: Option<Box<TlsValidationContextFileTrust_>>,
        pub sds: Option<Box<TlsValidationContextSdsTrust_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_ListenerTlsValidationContextTrust {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.ListenerTlsValidationContextTrust"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_ListenerTlsValidationContextTrust as ListenerTlsValidationContextTrust;
    impl crate::value::ToValue for ListenerTlsValidationContextTrust_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.file {
                properties.insert("File".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.sds {
                properties.insert("SDS".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-logging.html
    pub struct Logging_ {
        pub access_log: Option<Box<AccessLog_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_Logging {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.Logging"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_Logging as Logging;
    impl crate::value::ToValue for Logging_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.access_log {
                properties.insert(
                    "AccessLog".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-loggingformat.html
    pub struct LoggingFormat_ {
        pub json: Option<Vec<JsonFormatRef_>>,
        pub text: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_LoggingFormat {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.LoggingFormat"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_LoggingFormat as LoggingFormat;
    impl crate::value::ToValue for LoggingFormat_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.json {
                properties.insert("Json".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.text {
                properties.insert("Text".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-outlierdetection.html
    pub struct OutlierDetection_ {
        pub base_ejection_duration: Box<Duration_>,
        pub interval: Box<Duration_>,
        pub max_ejection_percent: i64,
        pub max_server_errors: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_OutlierDetection {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.OutlierDetection"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_OutlierDetection as OutlierDetection;
    impl crate::value::ToValue for OutlierDetection_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BaseEjectionDuration".to_string(),
                crate::value::ToValue::to_value(&self.base_ejection_duration),
            );
            properties.insert(
                "Interval".to_string(),
                crate::value::ToValue::to_value(&self.interval),
            );
            properties.insert(
                "MaxEjectionPercent".to_string(),
                crate::value::ToValue::to_value(&self.max_ejection_percent),
            );
            properties.insert(
                "MaxServerErrors".to_string(),
                crate::value::ToValue::to_value(&self.max_server_errors),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-portmapping.html
    pub struct PortMapping_ {
        pub port: i64,
        pub protocol: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_PortMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.PortMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_PortMapping as PortMapping;
    impl crate::value::ToValue for PortMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Port".to_string(),
                crate::value::ToValue::to_value(&self.port),
            );
            properties.insert(
                "Protocol".to_string(),
                crate::value::ToValue::to_value(&self.protocol),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-servicediscovery.html
    pub struct ServiceDiscovery_ {
        pub aws_cloud_map: Option<Box<AwsCloudMapServiceDiscovery_>>,
        pub dns: Option<Box<DnsServiceDiscovery_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_ServiceDiscovery {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.ServiceDiscovery"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_ServiceDiscovery as ServiceDiscovery;
    impl crate::value::ToValue for ServiceDiscovery_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.aws_cloud_map {
                properties.insert(
                    "AWSCloudMap".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dns {
                properties.insert("DNS".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-subjectalternativenamematchers.html
    pub struct SubjectAlternativeNameMatchers_ {
        pub exact: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_SubjectAlternativeNameMatchers {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.SubjectAlternativeNameMatchers"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_SubjectAlternativeNameMatchers as SubjectAlternativeNameMatchers;
    impl crate::value::ToValue for SubjectAlternativeNameMatchers_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.exact {
                properties.insert("Exact".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-subjectalternativenames.html
    pub struct SubjectAlternativeNames_ {
        pub r#match: Box<SubjectAlternativeNameMatchers_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_SubjectAlternativeNames {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.SubjectAlternativeNames"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_SubjectAlternativeNames as SubjectAlternativeNames;
    impl crate::value::ToValue for SubjectAlternativeNames_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Match".to_string(),
                crate::value::ToValue::to_value(&self.r#match),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-tcptimeout.html
    pub struct TcpTimeout_ {
        pub idle: Option<Box<Duration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_TcpTimeout {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.TcpTimeout"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_TcpTimeout as TcpTimeout;
    impl crate::value::ToValue for TcpTimeout_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.idle {
                properties.insert("Idle".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-tlsvalidationcontext.html
    pub struct TlsValidationContext_ {
        pub subject_alternative_names: Option<Box<SubjectAlternativeNames_>>,
        pub trust: Box<TlsValidationContextTrust_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_TlsValidationContext {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.TlsValidationContext"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_TlsValidationContext as TlsValidationContext;
    impl crate::value::ToValue for TlsValidationContext_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.subject_alternative_names {
                properties.insert(
                    "SubjectAlternativeNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Trust".to_string(),
                crate::value::ToValue::to_value(&self.trust),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-tlsvalidationcontextacmtrust.html
    pub struct TlsValidationContextAcmTrust_ {
        pub certificate_authority_arns: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_TlsValidationContextAcmTrust {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.TlsValidationContextAcmTrust"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_TlsValidationContextAcmTrust as TlsValidationContextAcmTrust;
    impl crate::value::ToValue for TlsValidationContextAcmTrust_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CertificateAuthorityArns".to_string(),
                crate::value::ToValue::to_value(&self.certificate_authority_arns),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-tlsvalidationcontextfiletrust.html
    pub struct TlsValidationContextFileTrust_ {
        pub certificate_chain: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_TlsValidationContextFileTrust {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.TlsValidationContextFileTrust"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_TlsValidationContextFileTrust as TlsValidationContextFileTrust;
    impl crate::value::ToValue for TlsValidationContextFileTrust_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CertificateChain".to_string(),
                crate::value::ToValue::to_value(&self.certificate_chain),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-tlsvalidationcontextsdstrust.html
    pub struct TlsValidationContextSdsTrust_ {
        pub secret_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_TlsValidationContextSdsTrust {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.TlsValidationContextSdsTrust"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_TlsValidationContextSdsTrust as TlsValidationContextSdsTrust;
    impl crate::value::ToValue for TlsValidationContextSdsTrust_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SecretName".to_string(),
                crate::value::ToValue::to_value(&self.secret_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-tlsvalidationcontexttrust.html
    pub struct TlsValidationContextTrust_ {
        pub acm: Option<Box<TlsValidationContextAcmTrust_>>,
        pub file: Option<Box<TlsValidationContextFileTrust_>>,
        pub sds: Option<Box<TlsValidationContextSdsTrust_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_TlsValidationContextTrust {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.TlsValidationContextTrust"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_TlsValidationContextTrust as TlsValidationContextTrust;
    impl crate::value::ToValue for TlsValidationContextTrust_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.acm {
                properties.insert("ACM".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.file {
                properties.insert("File".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.sds {
                properties.insert("SDS".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-virtualnodeconnectionpool.html
    pub struct VirtualNodeConnectionPool_ {
        pub grpc: Option<Box<VirtualNodeGrpcConnectionPool_>>,
        pub http: Option<Box<VirtualNodeHttpConnectionPool_>>,
        pub http2: Option<Box<VirtualNodeHttp2ConnectionPool_>>,
        pub tcp: Option<Box<VirtualNodeTcpConnectionPool_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_VirtualNodeConnectionPool {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.VirtualNodeConnectionPool"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_VirtualNodeConnectionPool as VirtualNodeConnectionPool;
    impl crate::value::ToValue for VirtualNodeConnectionPool_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.grpc {
                properties.insert("GRPC".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.http {
                properties.insert("HTTP".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.http2 {
                properties.insert("HTTP2".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.tcp {
                properties.insert("TCP".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-virtualnodegrpcconnectionpool.html
    pub struct VirtualNodeGrpcConnectionPool_ {
        pub max_requests: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_VirtualNodeGrpcConnectionPool {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.VirtualNodeGrpcConnectionPool"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_VirtualNodeGrpcConnectionPool as VirtualNodeGrpcConnectionPool;
    impl crate::value::ToValue for VirtualNodeGrpcConnectionPool_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MaxRequests".to_string(),
                crate::value::ToValue::to_value(&self.max_requests),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-virtualnodehttp2connectionpool.html
    pub struct VirtualNodeHttp2ConnectionPool_ {
        pub max_requests: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_VirtualNodeHttp2ConnectionPool {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.VirtualNodeHttp2ConnectionPool"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_VirtualNodeHttp2ConnectionPool as VirtualNodeHttp2ConnectionPool;
    impl crate::value::ToValue for VirtualNodeHttp2ConnectionPool_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MaxRequests".to_string(),
                crate::value::ToValue::to_value(&self.max_requests),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-virtualnodehttpconnectionpool.html
    pub struct VirtualNodeHttpConnectionPool_ {
        pub max_connections: i64,
        pub max_pending_requests: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_VirtualNodeHttpConnectionPool {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.VirtualNodeHttpConnectionPool"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_VirtualNodeHttpConnectionPool as VirtualNodeHttpConnectionPool;
    impl crate::value::ToValue for VirtualNodeHttpConnectionPool_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MaxConnections".to_string(),
                crate::value::ToValue::to_value(&self.max_connections),
            );
            if let Some(ref value) = self.max_pending_requests {
                properties.insert(
                    "MaxPendingRequests".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-virtualnodespec.html
    pub struct VirtualNodeSpec_ {
        pub backend_defaults: Option<Box<BackendDefaults_>>,
        pub backends: Option<Vec<Backend_>>,
        pub listeners: Option<Vec<Listener_>>,
        pub logging: Option<Box<Logging_>>,
        pub service_discovery: Option<Box<ServiceDiscovery_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_VirtualNodeSpec {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.VirtualNodeSpec"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_VirtualNodeSpec as VirtualNodeSpec;
    impl crate::value::ToValue for VirtualNodeSpec_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.backend_defaults {
                properties.insert(
                    "BackendDefaults".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.backends {
                properties.insert(
                    "Backends".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.listeners {
                properties.insert(
                    "Listeners".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.logging {
                properties.insert(
                    "Logging".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.service_discovery {
                properties.insert(
                    "ServiceDiscovery".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-virtualnodetcpconnectionpool.html
    pub struct VirtualNodeTcpConnectionPool_ {
        pub max_connections: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_VirtualNodeTcpConnectionPool {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.VirtualNodeTcpConnectionPool"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_VirtualNodeTcpConnectionPool as VirtualNodeTcpConnectionPool;
    impl crate::value::ToValue for VirtualNodeTcpConnectionPool_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MaxConnections".to_string(),
                crate::value::ToValue::to_value(&self.max_connections),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualnode-virtualservicebackend.html
    pub struct VirtualServiceBackend_ {
        pub client_policy: Option<Box<ClientPolicy_>>,
        pub virtual_service_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualNode_VirtualServiceBackend {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualNode.VirtualServiceBackend"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualNode_VirtualServiceBackend as VirtualServiceBackend;
    impl crate::value::ToValue for VirtualServiceBackend_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.client_policy {
                properties.insert(
                    "ClientPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "VirtualServiceName".to_string(),
                crate::value::ToValue::to_value(&self.virtual_service_name),
            );
            properties.into()
        }
    }
}
pub mod virtualrouter {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualrouter-portmapping.html
    pub struct PortMapping_ {
        pub port: i64,
        pub protocol: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualRouter_PortMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualRouter.PortMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualRouter_PortMapping as PortMapping;
    impl crate::value::ToValue for PortMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Port".to_string(),
                crate::value::ToValue::to_value(&self.port),
            );
            properties.insert(
                "Protocol".to_string(),
                crate::value::ToValue::to_value(&self.protocol),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualrouter-virtualrouterlistener.html
    pub struct VirtualRouterListener_ {
        pub port_mapping: Box<PortMapping_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualRouter_VirtualRouterListener {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualRouter.VirtualRouterListener"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualRouter_VirtualRouterListener as VirtualRouterListener;
    impl crate::value::ToValue for VirtualRouterListener_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PortMapping".to_string(),
                crate::value::ToValue::to_value(&self.port_mapping),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualrouter-virtualrouterspec.html
    pub struct VirtualRouterSpec_ {
        pub listeners: Vec<VirtualRouterListener_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualRouter_VirtualRouterSpec {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualRouter.VirtualRouterSpec"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualRouter_VirtualRouterSpec as VirtualRouterSpec;
    impl crate::value::ToValue for VirtualRouterSpec_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Listeners".to_string(),
                crate::value::ToValue::to_value(&self.listeners),
            );
            properties.into()
        }
    }
}
pub mod virtualservice {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualservice-virtualnodeserviceprovider.html
    pub struct VirtualNodeServiceProvider_ {
        pub virtual_node_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualService_VirtualNodeServiceProvider {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualService.VirtualNodeServiceProvider"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualService_VirtualNodeServiceProvider as VirtualNodeServiceProvider;
    impl crate::value::ToValue for VirtualNodeServiceProvider_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "VirtualNodeName".to_string(),
                crate::value::ToValue::to_value(&self.virtual_node_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualservice-virtualrouterserviceprovider.html
    pub struct VirtualRouterServiceProvider_ {
        pub virtual_router_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualService_VirtualRouterServiceProvider {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualService.VirtualRouterServiceProvider"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualService_VirtualRouterServiceProvider as VirtualRouterServiceProvider;
    impl crate::value::ToValue for VirtualRouterServiceProvider_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "VirtualRouterName".to_string(),
                crate::value::ToValue::to_value(&self.virtual_router_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualservice-virtualserviceprovider.html
    pub struct VirtualServiceProvider_ {
        pub virtual_node: Option<Box<VirtualNodeServiceProvider_>>,
        pub virtual_router: Option<Box<VirtualRouterServiceProvider_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualService_VirtualServiceProvider {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualService.VirtualServiceProvider"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualService_VirtualServiceProvider as VirtualServiceProvider;
    impl crate::value::ToValue for VirtualServiceProvider_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.virtual_node {
                properties.insert(
                    "VirtualNode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.virtual_router {
                properties.insert(
                    "VirtualRouter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appmesh-virtualservice-virtualservicespec.html
    pub struct VirtualServiceSpec_ {
        pub provider: Option<Box<VirtualServiceProvider_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appmesh_VirtualService_VirtualServiceSpec {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AppMesh::VirtualService.VirtualServiceSpec"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appmesh_VirtualService_VirtualServiceSpec as VirtualServiceSpec;
    impl crate::value::ToValue for VirtualServiceSpec_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.provider {
                properties.insert(
                    "Provider".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-gatewayroute.html
pub struct GatewayRoute_ {
    pub gateway_route_name: Option<crate::value::ExpString>,
    pub mesh_name: crate::value::ExpString,
    pub mesh_owner: Option<crate::value::ExpString>,
    pub spec: super::appmesh::gatewayroute::GatewayRouteSpec_,
    pub tags: Option<Vec<crate::Tag_>>,
    pub virtual_gateway_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appmesh_GatewayRoute {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::AppMesh::GatewayRoute"
        $($field $value)*)
    };
}
pub use crate::__aws_appmesh_GatewayRoute as GatewayRoute;
impl crate::template::ToResource for GatewayRoute_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppMesh"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("GatewayRoute"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.gateway_route_name {
            properties.insert(
                "GatewayRouteName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "MeshName".to_string(),
            crate::value::ToValue::to_value(&self.mesh_name),
        );
        if let Some(ref value) = self.mesh_owner {
            properties.insert(
                "MeshOwner".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Spec".to_string(),
            crate::value::ToValue::to_value(&self.spec),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "VirtualGatewayName".to_string(),
            crate::value::ToValue::to_value(&self.virtual_gateway_name),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-mesh.html
pub struct Mesh_ {
    pub mesh_name: Option<crate::value::ExpString>,
    pub spec: Option<super::appmesh::mesh::MeshSpec_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appmesh_Mesh {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::AppMesh::Mesh" $($field
        $value)*)
    };
}
pub use crate::__aws_appmesh_Mesh as Mesh;
impl crate::template::ToResource for Mesh_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppMesh"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Mesh"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.mesh_name {
            properties.insert(
                "MeshName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.spec {
            properties.insert("Spec".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-route.html
pub struct Route_ {
    pub mesh_name: crate::value::ExpString,
    pub mesh_owner: Option<crate::value::ExpString>,
    pub route_name: Option<crate::value::ExpString>,
    pub spec: super::appmesh::route::RouteSpec_,
    pub tags: Option<Vec<crate::Tag_>>,
    pub virtual_router_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appmesh_Route {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::AppMesh::Route" $($field
        $value)*)
    };
}
pub use crate::__aws_appmesh_Route as Route;
impl crate::template::ToResource for Route_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppMesh"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Route"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "MeshName".to_string(),
            crate::value::ToValue::to_value(&self.mesh_name),
        );
        if let Some(ref value) = self.mesh_owner {
            properties.insert(
                "MeshOwner".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.route_name {
            properties.insert(
                "RouteName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Spec".to_string(),
            crate::value::ToValue::to_value(&self.spec),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "VirtualRouterName".to_string(),
            crate::value::ToValue::to_value(&self.virtual_router_name),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-virtualgateway.html
pub struct VirtualGateway_ {
    pub mesh_name: crate::value::ExpString,
    pub mesh_owner: Option<crate::value::ExpString>,
    pub spec: super::appmesh::virtualgateway::VirtualGatewaySpec_,
    pub tags: Option<Vec<crate::Tag_>>,
    pub virtual_gateway_name: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appmesh_VirtualGateway {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::AppMesh::VirtualGateway"
        $($field $value)*)
    };
}
pub use crate::__aws_appmesh_VirtualGateway as VirtualGateway;
impl crate::template::ToResource for VirtualGateway_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppMesh"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("VirtualGateway"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "MeshName".to_string(),
            crate::value::ToValue::to_value(&self.mesh_name),
        );
        if let Some(ref value) = self.mesh_owner {
            properties.insert(
                "MeshOwner".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Spec".to_string(),
            crate::value::ToValue::to_value(&self.spec),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.virtual_gateway_name {
            properties.insert(
                "VirtualGatewayName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-virtualnode.html
pub struct VirtualNode_ {
    pub mesh_name: crate::value::ExpString,
    pub mesh_owner: Option<crate::value::ExpString>,
    pub spec: super::appmesh::virtualnode::VirtualNodeSpec_,
    pub tags: Option<Vec<crate::Tag_>>,
    pub virtual_node_name: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appmesh_VirtualNode {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::AppMesh::VirtualNode"
        $($field $value)*)
    };
}
pub use crate::__aws_appmesh_VirtualNode as VirtualNode;
impl crate::template::ToResource for VirtualNode_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppMesh"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("VirtualNode"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "MeshName".to_string(),
            crate::value::ToValue::to_value(&self.mesh_name),
        );
        if let Some(ref value) = self.mesh_owner {
            properties.insert(
                "MeshOwner".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Spec".to_string(),
            crate::value::ToValue::to_value(&self.spec),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.virtual_node_name {
            properties.insert(
                "VirtualNodeName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-virtualrouter.html
pub struct VirtualRouter_ {
    pub mesh_name: crate::value::ExpString,
    pub mesh_owner: Option<crate::value::ExpString>,
    pub spec: super::appmesh::virtualrouter::VirtualRouterSpec_,
    pub tags: Option<Vec<crate::Tag_>>,
    pub virtual_router_name: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appmesh_VirtualRouter {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::AppMesh::VirtualRouter"
        $($field $value)*)
    };
}
pub use crate::__aws_appmesh_VirtualRouter as VirtualRouter;
impl crate::template::ToResource for VirtualRouter_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppMesh"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("VirtualRouter"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "MeshName".to_string(),
            crate::value::ToValue::to_value(&self.mesh_name),
        );
        if let Some(ref value) = self.mesh_owner {
            properties.insert(
                "MeshOwner".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Spec".to_string(),
            crate::value::ToValue::to_value(&self.spec),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.virtual_router_name {
            properties.insert(
                "VirtualRouterName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appmesh-virtualservice.html
pub struct VirtualService_ {
    pub mesh_name: crate::value::ExpString,
    pub mesh_owner: Option<crate::value::ExpString>,
    pub spec: super::appmesh::virtualservice::VirtualServiceSpec_,
    pub tags: Option<Vec<crate::Tag_>>,
    pub virtual_service_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appmesh_VirtualService {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::AppMesh::VirtualService"
        $($field $value)*)
    };
}
pub use crate::__aws_appmesh_VirtualService as VirtualService;
impl crate::template::ToResource for VirtualService_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppMesh"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("VirtualService"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "MeshName".to_string(),
            crate::value::ToValue::to_value(&self.mesh_name),
        );
        if let Some(ref value) = self.mesh_owner {
            properties.insert(
                "MeshOwner".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Spec".to_string(),
            crate::value::ToValue::to_value(&self.spec),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "VirtualServiceName".to_string(),
            crate::value::ToValue::to_value(&self.virtual_service_name),
        );
        properties
    }
}
