use proxy_wasm::traits::*;
use proxy_wasm::types::*;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Info);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(ObserveRoot)
    });
}}

struct ObserveRoot;

impl Context for ObserveRoot {}

impl RootContext for ObserveRoot {
    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

    fn create_http_context(&self, context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(ObserveFilter { context_id }))
    }
}

struct ObserveFilter {
    context_id: u32,
}

impl Context for ObserveFilter {}

impl HttpContext for ObserveFilter {
    fn on_http_request_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        let span = format!("obs-span-{}", self.context_id);
        self.set_http_request_header("x-observe-span", Some(span.as_str()));
        Action::Continue
    }

    fn on_http_response_headers(
        &mut self,
        _num_headers: usize,
        _end_of_stream: bool,
    ) -> Action {
        let span = format!("obs-span-{}", self.context_id);
        self.set_http_response_header("x-observe-span", Some(span.as_str()));
        Action::Continue
    }
}
