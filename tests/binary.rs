use std::{
    io::{BufRead, BufReader, Read},
    net::TcpListener,
    process::{Child, Command, Stdio},
    sync::mpsc,
    thread,
    time::Duration,
};

const BINARY: &str = env!("CARGO_BIN_EXE_finance-mcp");
const PORT_VARIABLE: &str = "FINANCE_MCP_PORT";
const STARTUP_TIMEOUT: Duration = Duration::from_secs(30);

fn free_port() -> u16 {
    TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port()
}

fn binary() -> Command {
    let mut command = Command::new(BINARY);
    command.env_remove(PORT_VARIABLE);
    command
}

fn spawn(command: &mut Command) -> Child {
    command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap()
}

fn wait_for_listening_line(child: &mut Child) -> String {
    let stderr = child.stderr.take().unwrap();
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        for line in BufReader::new(stderr).lines().map_while(Result::ok) {
            if line.contains("listening on") {
                let _ = sender.send(line);
                return;
            }
        }
    });
    receiver.recv_timeout(STARTUP_TIMEOUT).unwrap()
}

fn stop(mut child: Child) -> String {
    child.kill().unwrap();
    child.wait().unwrap();
    let mut stdout = String::new();
    child
        .stdout
        .take()
        .unwrap()
        .read_to_string(&mut stdout)
        .unwrap();
    stdout
}

#[test]
fn reads_port_from_environment_variable() {
    let port = free_port();
    let mut child = spawn(binary().env(PORT_VARIABLE, port.to_string()));

    let line = wait_for_listening_line(&mut child);
    stop(child);

    assert!(line.contains(&format!("127.0.0.1:{port}")));
}

#[test]
fn argument_overrides_environment_variable() {
    let argument_port = free_port();
    let mut child = spawn(
        binary()
            .arg("--port")
            .arg(argument_port.to_string())
            .env(PORT_VARIABLE, "1"),
    );

    let line = wait_for_listening_line(&mut child);
    stop(child);

    assert!(line.contains(&format!("127.0.0.1:{argument_port}")));
}

#[test]
fn logs_to_stderr_and_writes_nothing_to_stdout() {
    let port = free_port();
    let mut child = spawn(binary().arg("--port").arg(port.to_string()));

    wait_for_listening_line(&mut child);
    let stdout = stop(child);

    assert!(stdout.is_empty());
}

#[test]
fn fails_when_port_is_in_use() {
    let occupied = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = occupied.local_addr().unwrap().port();

    let output = binary()
        .arg("--port")
        .arg(port.to_string())
        .output()
        .unwrap();

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains(&port.to_string()));
}
