# Spec Delta

## Purpose

Gives MCP clients a simple way to confirm that the server is running and to see which server and version they are connected to.

## ADDED Requirements

### Requirement: List the server_status tool
The server SHALL offer a tool named `server_status` with a description. The tool SHALL take no input parameters.

#### Scenario: Tool is listed
- **WHEN** a client lists the available tools
- **THEN** the list contains `server_status` with a description
- **AND** its input schema has no required parameters

### Requirement: Report status, name and version
The `server_status` tool SHALL return a structured result with exactly three fields: `status` with the value `ok`, `name` with the value `finance-mcp`, and `version` with the server's released version number.

#### Scenario: Call the tool
- **WHEN** a client calls `server_status` without arguments
- **THEN** the call succeeds
- **AND** the result contains `status` equal to `ok`
- **AND** the result contains `name` equal to `finance-mcp`
- **AND** the result contains `version` equal to the server's version, for example `0.1.0`

#### Scenario: No additional data
- **WHEN** a client calls `server_status`
- **THEN** the result contains no fields other than `status`, `name` and `version`
