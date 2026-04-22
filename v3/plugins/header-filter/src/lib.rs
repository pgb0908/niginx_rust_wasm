use proxy_wasm::traits::*;
use proxy_wasm::types::*;

const FILTER_VERSION: &str = "header-filter/0.1.0";

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Info);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(HeaderRoot)
    });
}}

struct HeaderRoot;

impl Context for HeaderRoot {}

impl RootContext for HeaderRoot {
    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

    fn create_http_context(&self, _context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(HeaderFilter))
    }
}

struct HeaderFilter;

impl Context for HeaderFilter {}

impl HttpContext for HeaderFilter {
    fn on_http_request_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        let route = self
            .get_http_request_header("x-gateway-route")
            .unwrap_or_else(|| "unknown".to_string());
        let tenant = self
            .get_http_request_header("x-tenant-id")
            .unwrap_or_else(|| "default-tenant".to_string());

        let profile = if route == "users" { "protected" } else { "public" };

        self.set_http_request_header("x-tenant-id", Some(tenant.as_str()));
        self.set_http_request_header("x-gateway-policy-profile", Some(profile));
        self.set_http_request_header(
            "x-gateway-plugin-chain",
            Some("auth-filter,header-filter,observe-filter"),
        );
        self.set_http_request_header("x-gateway-filter-version", Some(FILTER_VERSION));

        Action::Continue
    }

    fn on_http_response_headers(
        &mut self,
        _num_headers: usize,
        _end_of_stream: bool,
    ) -> Action {
        let tenant = self
            .get_http_request_header("x-tenant-id")
            .unwrap_or_else(|| "default-tenant".to_string());
        let profile = self
            .get_http_request_header("x-gateway-policy-profile")
            .unwrap_or_else(|| "unknown".to_string());

        self.set_http_response_header("x-tenant-id", Some(tenant.as_str()));
        self.set_http_response_header("x-gateway-policy-profile", Some(profile.as_str()));
        self.set_http_response_header(
            "x-gateway-plugin-chain",
            Some("auth-filter,header-filter,observe-filter"),
        );
        self.set_http_response_header("x-gateway-filter-version", Some(FILTER_VERSION));
        Action::Continue
    }
}
