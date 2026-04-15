use proxy_wasm::traits::*;
use proxy_wasm::types::*;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Info);
    // WASM 모듈이 시작될 때 사용할 루트 컨텍스트를 등록한다.
    // 실제 요청이 들어오면 이 루트 컨텍스트가 요청별 HttpContext를 생성한다.
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(GatewayRoot)
    });
}}

struct GatewayRoot;

impl Context for GatewayRoot {}

impl RootContext for GatewayRoot {
    fn get_type(&self) -> Option<ContextType> {
        // 이 필터는 HTTP 요청/응답 흐름에서 동작하도록 선언한다.
        Some(ContextType::HttpContext)
    }

    fn create_http_context(&self, _context_id: u32) -> Option<Box<dyn HttpContext>> {
        // 요청 하나마다 독립적인 필터 컨텍스트를 만든다.
        // v1에서는 요청 간 상태를 공유하지 않는 가장 단순한 구조로 유지한다.
        Some(Box::new(GatewayFilter))
    }
}

struct GatewayFilter;

impl Context for GatewayFilter {}

impl HttpContext for GatewayFilter {
    fn on_http_request_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        // nginx가 요청 헤더를 읽는 시점에 호출된다.
        // 여기서 path를 보고 어떤 라우트로 들어왔는지 간단히 식별한다.
        let path = self.get_http_request_header(":path").unwrap_or_default();
        let route = if path.starts_with("/users/") {
            "users"
        } else if path.starts_with("/orders/") {
            "orders"
        } else {
            "unknown"
        };

        // upstream이 "WASM 필터가 실제로 실행됐다"는 것을 확인할 수 있도록
        // 요청 헤더에 표시용 값을 추가한다.
        self.set_http_request_header("x-gateway-wasm", Some("active"));
        self.set_http_request_header("x-gateway-route", Some(route));

        // 요청 처리를 중단하지 않고 다음 단계로 넘긴다.
        Action::Continue
    }

    fn on_http_response_headers(
        &mut self,
        _num_headers: usize,
        _end_of_stream: bool,
    ) -> Action {
        // 클라이언트도 필터 동작 여부를 바로 확인할 수 있도록
        // 응답 헤더에도 같은 표시를 남긴다.
        self.set_http_response_header("x-gateway-wasm", Some("active"));
        Action::Continue
    }
}
