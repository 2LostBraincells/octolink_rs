use octoprint_rs::*;
use tokio::test;

mod mocks;
use mocks::*;

#[test]
async fn get_api_version() {
    #[allow(unused)]
    let mock = mock_get_api_version();

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
    let mut mock = mock_post_api_connection();


    let mut printer = PrinterBuilder::new(mock.address, mock.api_key)
        .port(mock.port)
        .build();

    let _ = printer.set_connection(types::PrinterConnectionCommand::Connect {
        port: "/dev/ttyACM0".to_string(),
        baudrate: 115200,
        printer_profile: "my_printer_profile".to_string(),
        save: true,
        autoconnect: true,
    }).await;
    let _ = printer.set_connection(types::PrinterConnectionCommand::Disconnect).await;
    let _ = printer.set_connection(types::PrinterConnectionCommand::FakeAck).await;

    mock.mock = Some(mock.mock.unwrap().expect(3));
    mock.mock.unwrap().assert();
}

#[test]
async fn get_files() {
    let mut mock = mock_get_api_files();

    let mut printer = PrinterBuilder::new(mock.address, mock.api_key)
        .port(mock.port)
        .build();

    let root_files = printer.get_files().await.unwrap();

    dbg!(root_files);

    panic!();
}
