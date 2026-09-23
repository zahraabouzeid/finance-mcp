use rmcp::schemars::{self, JsonSchema};
use serde::Serialize;

pub const SERVER_NAME: &str = env!("CARGO_PKG_NAME");
pub const SERVER_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Health {
    Ok,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, JsonSchema)]
pub struct ServerStatus {
    pub status: Health,
    pub name: &'static str,
    pub version: &'static str,
}

impl ServerStatus {
    pub fn current() -> Self {
        Self {
            status: Health::Ok,
            name: SERVER_NAME,
            version: SERVER_VERSION,
        }
    }
}

#[cfg(test)]
mod tests {
    use rmcp::serde_json::{Value, json, to_value};

    use super::ServerStatus;

    fn current_as_json() -> Value {
        to_value(ServerStatus::current()).unwrap()
    }

    #[test]
    fn reports_ok_name_and_version() {
        assert_eq!(
            current_as_json(),
            json!({
                "status": "ok",
                "name": "finance-mcp",
                "version": env!("CARGO_PKG_VERSION"),
            })
        );
    }

    #[test]
    fn contains_no_other_fields() {
        let json = current_as_json();
        let fields = json.as_object().unwrap();

        assert_eq!(fields.len(), 3);
    }
}
