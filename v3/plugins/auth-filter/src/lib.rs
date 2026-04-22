use proxy_wasm::traits::*;
use proxy_wasm::types::*;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Info);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(AuthRoot)
    });
}}

struct AuthRoot;

impl Context for AuthRoot {}

impl RootContext for AuthRoot {
    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

    fn create_http_context(&self, context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(AuthFilter::new(context_id)))
    }
}

struct AuthFilter {
    context_id: u32,
    route: &'static str,
}

impl AuthFilter {
    fn new(context_id: u32) -> Self {
        Self {
            context_id,
            route: "unknown",
        }
    }

    fn classify_route(path: &str) -> &'static str {
        if path.starts_with("/users/") {
            "users"
        } else if path.starts_with("/orders/") {
            "orders"
        } else {
            "unknown"
        }
    }

    fn request_id(&self) -> String {
        format!("req-{}", self.context_id)
    }

    fn trace_id(&self) -> String {
        format!("trace-{}", self.context_id)
    }
}

impl Context for AuthFilter {}

impl HttpContext for AuthFilter {
    fn on_http_request_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        let path = self.get_http_request_header(":path").unwrap_or_default();
        self.route = Self::classify_route(&path);
        let request_id = self.request_id();
        let trace_id = self.trace_id();

        self.set_http_request_header("x-request-id", Some(request_id.as_str()));
        self.set_http_request_header("x-trace-id", Some(trace_id.as_str()));
        self.set_http_request_header("x-gateway-route", Some(self.route));
        self.set_http_request_header("x-gateway-policy-profile", Some("platform-v3"));
        self.set_http_request_header("x-gateway-decision", Some("allow"));

        if self.route == "users" {
            let api_key = self.get_http_request_header("x-api-key").unwrap_or_default();
            if api_key != "users-secret" {
                self.send_http_response(
                    401,
                    vec![
                        ("content-type", "application/json"),
                        ("x-gateway-route", self.route),
                        ("x-gateway-decision", "deny"),
                    ],
                    Some(
                        br#"{"error":{"code":"unauthorized","message":"missing or invalid api key"}}"#,
                    ),
                );
                return Action::Pause;
            }
        }

        Action::Continue
    }

    fn on_http_response_headers(
        &mut self,
        _num_headers: usize,
        _end_of_stream: bool,
    ) -> Action {
        let request_id = self.request_id();
        let trace_id = self.trace_id();

        self.set_http_response_header("x-request-id", Some(request_id.as_str()));
        self.set_http_response_header("x-trace-id", Some(trace_id.as_str()));
        self.set_http_response_header("x-gateway-route", Some(self.route));
        self.set_http_response_header("x-gateway-decision", Some("allow"));
        Action::Continue
    }
}
