pub trait Handler {
    fn run(&self) -> impl std::future::Future<Output = ()>;
}

pub async fn run<T: Handler + std::str::FromStr>() {
    let value = std::env::var("_HANDLER").expect("_HANDLER environment variable");
    T::from_str(&value)
        .unwrap_or_else(|_| panic!("_HANDLER value error: {} not registered", &value))
        .run()
        .await
}

pub struct Client {
    pub base_url: reqwest::Url,
    pub http: reqwest::Client,
}

#[derive(Debug)]
pub struct RequestId(pub String);
#[derive(Debug)]
pub struct TraceHeader(pub String);

#[derive(Debug)]
pub struct Event<T> {
    pub body: T,
    pub request_id: RequestId,
    pub trace_header: TraceHeader,
}

impl Client {
    pub fn load() -> Client {
        let env_value = &std::env::var("AWS_LAMBDA_RUNTIME_API")
            .expect("Environment variable AWS_LAMBDA_RUNTIME_API missing");

        let base_uri: http::Uri = match env_value
            .parse::<http::Uri>()
            .expect("Cannot parse AWS_LAMBDA_RUNTIME_API as URI")
            .into_parts()
        {
            http::uri::Parts {
                authority: Some(authority),
                path_and_query: None,
                scheme: None,
                ..
            } => http::Uri::builder()
                .scheme("http")
                .authority(authority)
                .path_and_query("/")
                .build()
                .unwrap(),
            other => panic!("Unexpected AWS_LAMBDA_RUNTIME_API URI: {other:#?}"),
        };

        let base_url = reqwest::Url::parse(&base_uri.to_string()).unwrap();

        let http = reqwest::ClientBuilder::new()
            .user_agent("mrs/lambda")
            .no_proxy()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap();

        Client { base_url, http }
    }

    pub async fn run_event_loop<E: for<'de> serde::Deserialize<'de>, R, F>(&self, process: F)
    where
        F: for<'a> AsyncFn(&'a Event<E>) -> R,
        R: serde::Serialize,
    {
        loop {
            let event = self.read_next_event().await;

            self.send_response(&event.request_id, process(&event).await)
                .await;
        }
    }

    pub async fn read_next_event<T: for<'de> serde::Deserialize<'de>>(&self) -> Event<T> {
        fn fetch(map: &reqwest::header::HeaderMap, key: &str) -> String {
            map.get(key)
                .unwrap_or_else(|| panic!("Missing required header: {key}"))
                .to_str()
                .unwrap_or_else(|error| panic!("Cannot decode header: {key}, error: {error:#?}"))
                .to_string()
        }

        let url = self.absolute_url("/2018-06-01/runtime/invocation/next");

        let response = self.http.get(url).send().await.unwrap();

        if response.status() == reqwest::StatusCode::OK {
            let headers = response.headers();

            let request_id = RequestId(fetch(headers, "Lambda-Runtime-Aws-Request-Id"));
            let trace_header = TraceHeader(fetch(headers, "Lambda-Runtime-Trace-Id"));

            let body = response.json().await.unwrap();

            Event {
                body,
                request_id,
                trace_header,
            }
        } else {
            panic!("Unexpected response: {response:#?}");
        }
    }

    pub async fn send_response<T: serde::Serialize>(&self, request_id: &RequestId, response: T) {
        let url = self.absolute_url(&format!(
            "/2018-06-01/runtime/invocation/{}/response",
            request_id.0
        ));

        let response = self.http.post(url).json(&response).send().await.unwrap();

        if response.status() != reqwest::StatusCode::ACCEPTED {
            panic!("Unexpected response while sending lambda response: {response:#?}")
        }
    }

    fn absolute_url(&self, path: &str) -> reqwest::Url {
        let mut new_url = self.base_url.clone();
        new_url.set_path(path);
        new_url
    }
}
