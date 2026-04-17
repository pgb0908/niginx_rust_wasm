use crate::config::{Config, RoutePolicy};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Route {
    Users,
    Orders,
    Unknown,
}

impl Route {
    pub fn from_path(path: &str) -> Self {
        if path.starts_with("/users/") {
            Self::Users
        } else if path.starts_with("/orders/") {
            Self::Orders
        } else {
            Self::Unknown
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Users => "users",
            Self::Orders => "orders",
            Self::Unknown => "unknown",
        }
    }

    pub fn policy<'a>(&self, config: &'a Config) -> Option<&'a RoutePolicy> {
        match self {
            Self::Users => Some(&config.users),
            Self::Orders => Some(&config.orders),
            Self::Unknown => None,
        }
    }
}
