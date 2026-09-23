# Proposal

## Why

The project has no running code yet. A walking skeleton proves the full path from an MCP client to a tool and back, and gives every later change a working base to grow from.

## What Changes

- Add an MCP server that runs as a single binary and serves MCP over Streamable HTTP.
- The server listens on the loopback interface only.
- The server rejects requests with an untrusted `Host` or `Origin` header.
- The port is configurable through a command line argument or an environment variable, with a default.
- Add one tool, `server_status`, that reports that the server is running, its name and its version.
- Replace the placeholder `Hello, world!` program.

## Capabilities

### New Capabilities

- `mcp-server`: How the server is started, configured and reached, including network binding and request validation.
- `server-status`: A tool that reports the server's status, name and version.

### Modified Capabilities

None.

## Out of scope

- Final architecture, workspace and crate structure. The code stays in a single package and will be restructured in a later change.
- Authentication. It is required before the first tool that returns financial data.
- Docker, container images and binding to other interfaces than loopback.
- Plain HTTP endpoints outside MCP, such as `GET /health`.
- Any financial data, providers or domain types.
- Configuration of the host, allowed origins or other settings besides the port.

## Security impact

- The server opens a local TCP port. This is new attack surface.
- Mitigations: the server binds to `127.0.0.1` only, validates the `Host` header against loopback names to prevent DNS rebinding, and rejects requests from browser origins.
- The only tool returns public information (name, version, status). No secrets or personal data are handled.
- New dependencies increase supply chain risk. They are limited to well-known crates and checked by `cargo deny` in CI.

## Impact

- Code: `src/main.rs` is replaced, `src/lib.rs` and a small number of modules are added, and integration tests are added under `tests/`.
- Dependencies: MCP SDK, async runtime, HTTP server, CLI parsing, serialization and logging crates. Each is justified in `design.md`.
- Users: the server can be started locally and connected to with any MCP client that supports Streamable HTTP.
