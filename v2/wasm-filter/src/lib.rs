mod config;
mod error_response;
mod metadata;
mod policy;

use config::Config;
use policy::Route;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Info);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(GatewayRoot::new())
    });
}}

struct GatewayRoot {
    config: Config,
}

impl GatewayRoot {
    fn new() -> Self {
        Self {
            config: Config::load(),
        }
    }
}

impl Context for GatewayRoot {}

impl RootContext for GatewayRoot {
    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

    fn create_http_context(&self, context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(GatewayFilter::new(context_id, self.config.clone())))
    }
}

struct GatewayFilter {
    context_id: u32,
    config: Config,
    route: Route,
    request_id: String,
    trace_id: String,
}

impl GatewayFilter {
    fn new(context_id: u32, config: Config) -> Self {
        Self {
            context_id,
            config,
            route: Route::Unknown,
            request_id: String::new(),
            trace_id: String::new(),
        }
    }

    fn attach_standard_headers(&self) {
        self.set_http_request_header("x-gateway-wasm", Some("active"));
        self.set_http_request_header("x-gateway-route", Some(self.route.as_str()));
        self.set_http_request_header("x-organization", Some(self.config.org_name.as_str()));
        self.set_http_request_header(
            self.config.request_id_header.as_str(),
            Some(self.request_id.as_str()),
        );
        self.set_http_request_header(
            self.config.trace_id_header.as_str(),
            Some(self.trace_id.as_str()),
        );

        if let Some(policy) = self.route.policy(&self.config) {
            self.set_http_request_header("x-policy-profile", Some(policy.policy_profile.as_str()));
        }
    }

    fn reject_unauthorized(&self) -> Action {
        let body =
            error_response::unauthorized(self.route.as_str(), &self.request_id, &self.trace_id);
        self.send_http_response(
            401,
            vec![
                ("content-type", "application/json"),
                ("x-gateway-wasm", "active"),
                ("x-gateway-route", self.route.as_str()),
                (self.config.request_id_header.as_str(), self.request_id.as_str()),
                (self.config.trace_id_header.as_str(), self.trace_id.as_str()),
            ],
            Some(body.as_bytes()),
        );
        Action::Pause
    }
}

impl Context for GatewayFilter {}

impl HttpContext for GatewayFilter {
    fn on_http_request_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        let path = self.get_http_request_header(":path").unwrap_or_default();
        self.route = Route::from_path(&path);

        self.request_id = metadata::ensure_or_generate(
            self.get_http_request_header(self.config.request_id_header.as_str()),
            "req",
            self.context_id,
        );
        self.trace_id = metadata::ensure_or_generate(
            self.get_http_request_header(self.config.trace_id_header.as_str()),
            "trace",
            self.context_id,
        );

        self.attach_standard_headers();

        let Some(policy) = self.route.policy(&self.config) else {
            return Action::Continue;
        };

        if !policy.api_key_required {
            return Action::Continue;
        }

        let provided_api_key = self
            .get_http_request_header(self.config.auth_header.as_str())
            .unwrap_or_default();

        if provided_api_key == policy.api_key {
            Action::Continue
        } else {
            self.reject_unauthorized()
        }
    }

    fn on_http_response_headers(
        &mut self,
        _num_headers: usize,
        _end_of_stream: bool,
    ) -> Action {
        self.set_http_response_header("x-gateway-wasm", Some("active"));
        self.set_http_response_header("x-gateway-route", Some(self.route.as_str()));
        self.set_http_response_header(
            self.config.request_id_header.as_str(),
            Some(self.request_id.as_str()),
        );
        self.set_http_response_header(
            self.config.trace_id_header.as_str(),
            Some(self.trace_id.as_str()),
        );
        Action::Continue
    }
}
