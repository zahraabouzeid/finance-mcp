use clap::Parser;

pub const DEFAULT_PORT: u16 = 8765;

#[derive(Debug, Parser)]
#[command(
    name = "finance-mcp",
    version,
    about = "An MCP server for personal finance data"
)]
pub struct Args {
    #[arg(long, env = "FINANCE_MCP_PORT", default_value_t = DEFAULT_PORT)]
    pub port: u16,
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::{Args, DEFAULT_PORT};

    #[test]
    fn uses_default_port_without_arguments() {
        let args = Args::try_parse_from(["finance-mcp"]).unwrap();

        assert_eq!(args.port, DEFAULT_PORT);
        assert_eq!(DEFAULT_PORT, 8765);
    }

    #[test]
    fn reads_port_from_argument() {
        let args = Args::try_parse_from(["finance-mcp", "--port", "9100"]).unwrap();

        assert_eq!(args.port, 9100);
    }

    #[test]
    fn rejects_port_out_of_range() {
        let error = Args::try_parse_from(["finance-mcp", "--port", "70000"]).unwrap_err();

        assert!(error.to_string().contains("70000"));
    }

    #[test]
    fn rejects_port_that_is_not_a_number() {
        let error = Args::try_parse_from(["finance-mcp", "--port", "abc"]).unwrap_err();

        assert!(error.to_string().contains("abc"));
    }
}
