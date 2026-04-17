use proxy_wasm::traits::*;
use proxy_wasm::types::*;

// `proxy_wasm::main!` 매크로는 일반 Rust 프로그램의 `main()` 비슷한 진입점 역할을 만든다.
// 이 파일은 독립 실행 파일이 아니라 Wasm 모듈로 빌드되므로,
// 프록시(proxy-wasm 호스트)가 이 초기화 코드를 호출해서 필터를 등록한다.
proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Info);
    // WASM 모듈이 시작될 때 사용할 루트 컨텍스트를 등록한다.
    // 실제 요청이 들어오면 이 루트 컨텍스트가 요청별 HttpContext를 생성한다.
    //
    // `Box<dyn RootContext>`:
    // - `Box`는 힙에 값을 저장하는 스마트 포인터다.
    // - `dyn RootContext`는 "RootContext 트레이트를 구현한 어떤 타입이든 가능"하다는 뜻이다.
    //   즉, 구체 타입 `GatewayRoot`를 trait object 형태로 넘긴다.
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(GatewayRoot)
    });
}}

// 루트 컨텍스트는 "필터 전체의 공통 진입점"에 가깝다.
// 요청 하나하나를 직접 처리하기보다, 요청별 컨텍스트를 만들어 주는 역할을 맡는다.
struct GatewayRoot;

// `Context`는 proxy-wasm 컨텍스트들의 공통 기반 트레이트다.
// 여기서는 추가로 구현할 공통 동작이 없어서 빈 구현만 둔다.
impl Context for GatewayRoot {}

impl RootContext for GatewayRoot {
    fn get_type(&self) -> Option<ContextType> {
        // 이 필터는 HTTP 요청/응답 흐름에서 동작하도록 선언한다.
        // `Some(...)`을 반환하는 이유는 이 함수의 반환 타입이 `Option<ContextType>`이기 때문이다.
        Some(ContextType::HttpContext)
    }

    fn create_http_context(&self, _context_id: u32) -> Option<Box<dyn HttpContext>> {
        // 요청 하나마다 독립적인 필터 컨텍스트를 만든다.
        // v1에서는 요청 간 상태를 공유하지 않는 가장 단순한 구조로 유지한다.
        //
        // `_context_id` 앞의 `_`는 "지금은 안 쓰는 인자"라는 의미다.
        // Rust는 사용하지 않는 변수에 대해 경고를 내는데, `_`를 붙이면 의도적으로 무시한다고 알릴 수 있다.
        Some(Box::new(GatewayFilter))
    }
}

// `GatewayFilter`는 실제 HTTP 요청/응답을 다루는 요청별 컨텍스트다.
// 현재는 별도 필드가 없는 빈 구조체지만, 나중에 요청 중간 상태를 저장하도록 확장할 수 있다.
struct GatewayFilter;

impl Context for GatewayFilter {}

impl HttpContext for GatewayFilter {
    fn on_http_request_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        // nginx가 요청 헤더를 읽는 시점에 호출된다.
        // 여기서 path를 보고 어떤 라우트로 들어왔는지 간단히 식별한다.
        //
        // `&mut self`는 이 컨텍스트를 수정할 수 있는 가변 참조라는 뜻이다.
        // 헤더 추가 같은 변경 작업을 하려면 가변 참조가 필요하다.
        let path = self.get_http_request_header(":path").unwrap_or_default();
        // `:path`는 일반 HTTP 헤더가 아니라 HTTP/2/HTTP/3 계열에서 쓰는 pseudo-header다.
        // 예: `/users/10`, `/orders/99`
        //
        // `unwrap_or_default()`는 값이 없을 때 기본값을 쓰겠다는 뜻이다.
        // 여기서는 헤더가 없으면 빈 문자열 `""`로 처리한다.

        let route = if path.starts_with("/users/") {
            "users"
        } else if path.starts_with("/orders/") {
            "orders"
        } else {
            "unknown"
        };
        // 위 조건식은 path 문자열의 접두어(prefix)를 보고 단순 분류한다.
        // 예:
        // - `/users/10`   -> `users`
        // - `/orders/99`  -> `orders`
        // - `/health`     -> `unknown`

        // upstream이 "WASM 필터가 실제로 실행됐다"는 것을 확인할 수 있도록
        // 요청 헤더에 표시용 값을 추가한다.
        //
        // `Some("active")`처럼 `Option<&str>` 형태를 넘기는 이유는,
        // proxy-wasm API가 "헤더를 특정 값으로 설정하거나(Some), 제거하거나(None)"를 같은 함수로 처리하기 때문이다.
        self.set_http_request_header("x-gateway-wasm", Some("active"));
        self.set_http_request_header("x-gateway-route", Some(route));

        // 요청 처리를 중단하지 않고 다음 단계로 넘긴다.
        // 만약 여기서 요청을 막고 싶다면 다른 `Action`을 반환하는 방식으로 확장할 수 있다.
        Action::Continue
    }

    fn on_http_response_headers(
        &mut self,
        _num_headers: usize,
        _end_of_stream: bool,
    ) -> Action {
        // 클라이언트도 필터 동작 여부를 바로 확인할 수 있도록
        // 응답 헤더에도 같은 표시를 남긴다.
        // 즉, 요청을 받은 upstream뿐 아니라 최종 호출자도 "Wasm 필터가 응답 경로에서 실행됐다"는 것을 볼 수 있다.
        self.set_http_response_header("x-gateway-wasm", Some("active"));
        Action::Continue
    }
}
