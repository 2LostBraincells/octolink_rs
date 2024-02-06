use octoprint_rs::*;
use tokio::test;

mod mocks;
use mocks::*;

use octoprint_rs::errors::{FileRequestError, InformationRequestError, SetConnectionError};

#[test]
async fn get_api_version() {
    #[allow(unused)]
    let mock = mock_get_api_version();

    let printer_builder = PrinterBuilder::new(mock.address, mock.api_key).port(mock.port);
    let mut printer = printer_builder.build();

    let api_version = printer.get_api_version().await.unwrap();

    assert_eq!(api_version.api, "0.1".to_string());
    assert_eq!(api_version.server, "1.9.3".to_string());
    assert_eq!(api_version.text, "OctoPrint 1.9.3".to_string());

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

    let _ = printer
        .set_connection(types::ConnectionCommandDescriptor::Connect {
            port: "/dev/ttyACM0".to_string(),
            baudrate: 115200,
            printer_profile: "my_printer_profile".to_string(),
            save: true,
            autoconnect: true,
        })
        .await;
    let _ = printer
        .set_connection(types::ConnectionCommandDescriptor::Disconnect)
        .await;
    let _ = printer
        .set_connection(types::ConnectionCommandDescriptor::FakeAck)
        .await;

    mock.mock = Some(mock.mock.unwrap().expect(3));
    mock.mock.unwrap().assert();
}

#[test]
async fn get_location() {
    let mut mock = mock_get_api_files();

    let mut printer = PrinterBuilder::new(mock.address, mock.api_key)
        .port(mock.port)
        .build();

    let root_files = printer
        .get_files(types::FilesFetchDescriptor {
            location: types::FilesLocation::Root,
            recursive: false,
            force: false,
        })
        .await
        .unwrap();

    dbg!(root_files);

    mock.mock.unwrap().assert();
}

#[test]
async fn get_location_recursive() {
    let mock = mock_get_api_files_q_recursive();

    let mut printer = PrinterBuilder::new(mock.address, mock.api_key)
        .port(mock.port)
        .build();

    let files_recursive = printer
        .get_files(types::FilesFetchDescriptor {
            location: types::FilesLocation::Root,
            recursive: true,
            force: false,
        })
        .await
        .unwrap();

    dbg!(files_recursive);

    mock.mock.unwrap().assert();
}

#[test]
async fn get_file() {
    let mock = mock_get_api_files_local_folder_printed();

    let printer = PrinterBuilder::new(mock.address, mock.api_key)
        .port(mock.port)
        .build();

    let file = printer
        .get_file(types::FileFetchDescriptor {
            location: types::FileLocation::Local,
            path: "/folder/printed.gcode".to_string(),
            recursive: false,
            force: false,
        })
        .await
        .unwrap();

    dbg!(file);

    mock.mock.unwrap().assert();
}
