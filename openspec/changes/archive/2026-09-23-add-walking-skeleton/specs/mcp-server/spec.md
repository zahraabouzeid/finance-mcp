# Spec Delta

## Purpose

Defines how the finance-mcp server is started, configured and reached by MCP clients, including which network interfaces it listens on and which requests it accepts.

## ADDED Requirements

### Requirement: Serve MCP over Streamable HTTP
The server SHALL serve the Model Context Protocol over the Streamable HTTP transport at the path `/mcp`.

#### Scenario: Client connects
- **WHEN** an MCP client connects to `http://127.0.0.1:<port>/mcp` and initializes
- **THEN** the initialization succeeds
- **AND** the server reports that it provides tools

### Requirement: Listen on loopback only
The server SHALL listen on the IPv4 loopback address `127.0.0.1` only. It MUST NOT listen on any other network interface.

#### Scenario: Server starts
- **WHEN** the server starts
- **THEN** it accepts connections on `127.0.0.1`
- **AND** it does not accept connections on any other interface

### Requirement: Configurable port
The server SHALL listen on port `8765` by default. The port SHALL be configurable through the command line argument `--port` and the environment variable `FINANCE_MCP_PORT`. When both are set, the command line argument MUST take precedence.

#### Scenario: Default port
- **WHEN** the server starts without `--port` and without `FINANCE_MCP_PORT`
- **THEN** it listens on port `8765`

#### Scenario: Port from environment variable
- **WHEN** the server starts with `FINANCE_MCP_PORT=9000` and without `--port`
- **THEN** it listens on port `9000`

#### Scenario: Argument overrides environment variable
- **WHEN** the server starts with `FINANCE_MCP_PORT=9000` and `--port 9100`
- **THEN** it listens on port `9100`

#### Scenario: Invalid port
- **WHEN** the server starts with a port that is not a number between 0 and 65535
- **THEN** it exits with a non-zero exit code
- **AND** it prints an error message that names the invalid value

#### Scenario: Port already in use
- **WHEN** the server starts on a port that is already in use
- **THEN** it exits with a non-zero exit code
- **AND** it prints an error message that names the port

### Requirement: Reject untrusted Host headers
The server SHALL accept only requests whose `Host` header names a loopback host. Other requests MUST be rejected with an HTTP client error status and MUST NOT reach any tool.

#### Scenario: Loopback host
- **WHEN** a request arrives with `Host: 127.0.0.1:<port>` or `Host: localhost:<port>`
- **THEN** the server processes the request

#### Scenario: Foreign host
- **WHEN** a request arrives with `Host: attacker.example`
- **THEN** the server rejects it with an HTTP client error status

### Requirement: Reject browser origins
The server SHALL reject every request that carries an `Origin` header with an HTTP client error status. Requests without an `Origin` header SHALL be processed.

#### Scenario: Request from a web page
- **WHEN** a request arrives with `Origin: https://attacker.example`
- **THEN** the server rejects it with an HTTP client error status

#### Scenario: Request from a native client
- **WHEN** a request arrives without an `Origin` header
- **THEN** the server processes the request

### Requirement: Log to standard error
The server SHALL write all log output to standard error. On startup it SHALL log the address it listens on.

#### Scenario: Startup log
- **WHEN** the server starts successfully
- **THEN** it writes a log line with the listening address to standard error
- **AND** it writes nothing to standard output

### Requirement: Graceful shutdown
The server SHALL shut down gracefully when it receives an interrupt signal (Ctrl+C).

#### Scenario: Interrupt
- **WHEN** the server receives an interrupt signal
- **THEN** it stops accepting new requests
- **AND** it exits with exit code 0
