#[derive(Clone, Debug)]
pub struct RoutePolicy {
    pub api_key_required: bool,
    pub api_key: String,
    pub policy_profile: String,
}

#[derive(Clone, Debug)]
pub struct Config {
    pub org_name: String,
    pub request_id_header: String,
    pub trace_id_header: String,
    pub auth_header: String,
    pub users: RoutePolicy,
    pub orders: RoutePolicy,
}

impl Config {
    pub fn load() -> Self {
        let raw = include_str!("../policy.conf");
        let mut config = Self::default();

        for line in raw.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let Some((key, value)) = line.split_once('=') else {
                continue;
            };

            let key = key.trim();
            let value = value.trim();

            match key {
                "org_name" => config.org_name = value.to_string(),
                "request_id_header" => config.request_id_header = value.to_string(),
                "trace_id_header" => config.trace_id_header = value.to_string(),
                "auth_header" => config.auth_header = value.to_string(),
                "users.api_key_required" => config.users.api_key_required = parse_bool(value),
                "users.api_key" => config.users.api_key = value.to_string(),
                "orders.api_key_required" => config.orders.api_key_required = parse_bool(value),
                "orders.api_key" => config.orders.api_key = value.to_string(),
                _ => {}
            }
        }

        config
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            org_name: "acme-gateway".to_string(),
            request_id_header: "x-request-id".to_string(),
            trace_id_header: "x-trace-id".to_string(),
            auth_header: "x-api-key".to_string(),
            users: RoutePolicy {
                api_key_required: true,
                api_key: "users-secret".to_string(),
                policy_profile: "protected".to_string(),
            },
            orders: RoutePolicy {
                api_key_required: false,
                api_key: "orders-demo".to_string(),
                policy_profile: "public".to_string(),
            },
        }
    }
}

fn parse_bool(value: &str) -> bool {
    matches!(value, "1" | "true" | "yes" | "on")
}
