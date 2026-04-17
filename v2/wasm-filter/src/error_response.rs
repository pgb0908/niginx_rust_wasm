pub fn unauthorized(route: &str, request_id: &str, trace_id: &str) -> String {
    format!(
        "{{\"error\":{{\"code\":\"unauthorized\",\"message\":\"missing or invalid API key\",\"route\":\"{}\",\"request_id\":\"{}\",\"trace_id\":\"{}\"}}}}",
        route, request_id, trace_id
    )
}
