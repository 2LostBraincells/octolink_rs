use octoprint_rs::*;
use tokio::test;

pub struct MockFrame {
    server: mockito::ServerGuard,
    mock: Option<mockito::Mock>,
    address: String,
    port: u16,
    api_key: String,
}

/// Creates the base for a mock server, parsing the given url and returning the server, address, port and api key
///
/// Naming convention for the derivative functions is `mock_{path}`
/// where path is the path of the api that is being mocked, with slashes replaced by underscores
/// for example, the path `/api/printer` is mocked by `mock_api_printer`
fn mock_base() -> MockFrame {
    let server = mockito::Server::new();

    let url = server.url();
    let url = url.strip_prefix("http://").unwrap();
    let url = url.split_at(url.find(':').unwrap());
    let (address, port) = (
        url.0.to_string(),
        url.1
            .to_string()
            .strip_prefix(':')
            .unwrap()
            .parse::<u16>()
            .unwrap(),
    );

    let api_key = "1234567890".to_string();

    dbg!(&address, &port);

    MockFrame {
        server,
        address,
        port,
        api_key,
        mock: None,
    }
}

fn mock_api_version() -> MockFrame {
    let mut server = mock_base();

    let mock = Some(
        server
            .server
            .mock("GET", "/api/version")
            .match_header("X-Api-Key", server.api_key.as_str())
            .with_status(200)
            .with_body(
                r#"{
  "api": "0.1",
  "server": "1.3.10",
  "text": "OctoPrint 1.3.10"
}"#,
            )
            .create(),
    );

    MockFrame { mock, ..server }
}

fn mock_get_api_connection() -> MockFrame {
    let mut server = mock_base();

    let mock = Some(
        server
            .server
            .mock("GET", "/api/connection")
            .match_header("X-Api-Key", server.api_key.as_str())
            .with_status(200)
            .with_body(
                r#"{
  "current": {
    "state": "Operational",
    "port": "/dev/ttyACM0",
    "baudrate": 250000,
    "printerProfile": "_default"
  },
  "options": {
    "ports": ["/dev/ttyACM0", "VIRTUAL"],
    "baudrates": [250000, 230400, 115200, 57600, 38400, 19200, 9600],
    "printerProfiles": [{"name": "Default", "id": "_default"}],
    "portPreference": "/dev/ttyACM0",
    "baudratePreference": 250000,
    "printerProfilePreference": "_default",
    "autoconnect": true
  }
}"#,
            )
            .create(),
    );

    MockFrame { mock, ..server }
}

fn mock_post_api_connection() -> MockFrame {
    let mut server = mock_base();

    let mock = Some(server.server
        .mock("POST", "/api/connection")
        .match_header("X-Api-Key", server.api_key.as_str())
        .with_status(204)
        .create());

    MockFrame {
        mock,
        ..server
    }
}

#[test]
async fn get_api_version() {
    #[allow(unused)]
    let mock = mock_api_version();

    let printer_builder = PrinterBuilder::new(mock.address, mock.api_key).port(mock.port);
    let mut printer = printer_builder.build();

    let api_version = printer.get_api_version().await.unwrap();

    assert_eq!(api_version.api, "0.1".to_string());
    assert_eq!(api_version.server, "1.3.10".to_string());
    assert_eq!(api_version.text, "OctoPrint 1.3.10".to_string());

    mock.mock.unwrap().assert()
}

#[test]
async fn get_connection() {
    let mock = mock_get_api_connection();

    let mut printer = PrinterBuilder::new(mock.address, mock.api_key)
        .port(mock.port)
        .build();

    let printer_connection = printer.get_connection().await.unwrap();

    mock.mock.unwrap().assert();
}

#[test]
async fn set_connection() {
    let mock = mock_post_api_connection();

    let mut printer = PrinterBuilder::new(mock.address, mock.api_key)
        .port(mock.port)
        .build();

    let _ = printer.set_connection(types::PrinterConnectionCommand::Connect {
        port: "/dev/ttyACM0".to_string(),
        baudrate: 115200,
        printer_profile: "my_printer_profile".to_string(),
        save: true,
        autoconnect: true,
    });
    let _ = printer.set_connection(types::PrinterConnectionCommand::Disconnect);
    let _ = printer.set_connection(types::PrinterConnectionCommand::FakeAck);

    mock.mock.unwrap().assert();
}
