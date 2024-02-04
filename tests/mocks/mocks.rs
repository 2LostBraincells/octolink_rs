pub struct MockFrame {
    pub server: mockito::ServerGuard,
    pub mock: Option<mockito::Mock>,
    pub address: String,
    pub port: u16,
    pub api_key: String,
}

/// Creates the base for a mock server, parsing the given url and returning the server, address, port and api key
///
/// Naming convention for the derivative functions is `mock_{method}_{path}`
/// where path is the path of the api that is being mocked, with slashes replaced by underscores
/// for example, the path `/api/printer` with the GET method is mocked by `mock_get_api_printer`
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

pub fn mock_get_api_version() -> MockFrame {
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

pub fn mock_get_api_connection() -> MockFrame {
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

pub fn mock_post_api_connection() -> MockFrame {
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

pub fn mock_get_api_files() -> MockFrame {
    let mut server = mock_base();

    let mock = Some(server.server
        .mock("GET", "/api/files")
        .match_header("X-Api-Key", server.api_key.as_str())
        .with_status(200)
        .with_body(
        r#"{
  "files": [
    {
      "name": "whistle_v2.gcode",
      "path": "whistle_v2.gcode",
      "type": "machinecode",
      "typePath": ["machinecode", "gcode"],
      "hash": "...",
      "size": 1468987,
      "date": 1378847754,
      "origin": "local",
      "refs": {
        "resource": "http://example.com/api/files/local/whistle_v2.gcode",
        "download": "http://example.com/downloads/files/local/whistle_v2.gcode"
      },
      "gcodeAnalysis": {
        "estimatedPrintTime": 1188,
        "filament": {
          "length": 810,
          "volume": 5.36
        }
      },
      "print": {
        "failure": 4,
        "success": 23,
        "last": {
          "date": 1387144346,
          "success": true
        }
      }
    },
    {
      "name": "whistle_.gco",
      "path": "whistle_.gco",
      "type": "machinecode",
      "typePath": ["machinecode", "gcode"],
      "origin": "sdcard",
      "refs": {
        "resource": "http://example.com/api/files/sdcard/whistle_.gco"
      }
    },
    {
      "name": "folderA",
      "path": "folderA",
      "type": "folder",
      "typePath": ["folder"],
      "children": [
        {
          "name": "whistle_v2_copy.gcode",
          "path": "whistle_v2_copy.gcode",
          "type": "machinecode",
          "typePath": ["machinecode", "gcode"],
          "hash": "...",
          "size": 1468987,
          "date": 1378847754,
          "origin": "local",
          "refs": {
            "resource": "http://example.com/api/files/local/folderA/whistle_v2_copy.gcode",
            "download": "http://example.com/downloads/files/local/folderA/whistle_v2_copy.gcode"
          },
          "gcodeAnalysis": {
            "estimatedPrintTime": 1188,
            "filament": {
              "length": 810,
              "volume": 5.36
            }
          },
          "print": {
            "failure": 4,
            "success": 23,
            "last": {
              "date": 1387144346,
              "success": true
            }
          }
        }
      ]
    }
  ],
  "free": "3.2GB"
}"#
        )
        .create());

    MockFrame {
        mock,
        ..server
    }
}

pub fn mock_get_api_files_recursive() -> MockFrame {
    let mut server = mock_base();

    let mock = Some(server.server.mock("GET", "/api/files?recursive=true")
        .match_header("X-Api-Key", server.api_key.as_str())
        .with_status(200)
        .with_body(
        r#"{
  "files": [
    {
      "name": "whistle_v2.gcode",
      "path": "whistle_v2.gcode",
      "type": "machinecode",
      "typePath": ["machinecode", "gcode"],
      "hash": "...",
      "size": 1468987,
      "date": 1378847754,
      "origin": "local",
      "refs": {
        "resource": "http://example.com/api/files/local/whistle_v2.gcode",
        "download": "http://example.com/downloads/files/local/whistle_v2.gcode"
      },
      "gcodeAnalysis": {
        "estimatedPrintTime": 1188,
        "filament": {
          "length": 810,
          "volume": 5.36
        }
      },
      "print": {
        "failure": 4,
        "success": 23,
        "last": {
          "date": 1387144346,
          "success": true
        }
      }
    },
    {
      "name": "whistle_.gco",
      "path": "whistle_.gco",
      "type": "machinecode",
      "typePath": ["machinecode", "gcode"],
      "origin": "sdcard",
      "refs": {
        "resource": "http://example.com/api/files/sdcard/whistle_.gco"
      }
    },
    {
      "name": "folderA",
      "path": "folderA",
      "type": "folder",
      "typePath": ["folder"],
      "children": [
        {
          "name": "test.gcode",
          "path": "folderA/test.gcode",
          "type": "machinecode",
          "typePath": ["machinecode", "gcode"],
          "hash": "...",
          "size": 1234,
          "date": 1378847754,
          "origin": "local",
          "refs": {
            "resource": "http://example.com/api/files/local/folderA/test.gcode",
            "download": "http://example.com/downloads/files/local/folderA/test.gcode"
          }
        },
        {
          "name": "subfolder",
          "path": "folderA/subfolder",
          "type": "folder",
          "typePath": ["folder"],
          "children": [
            {
              "name": "test.gcode",
              "path": "folderA/subfolder/test2.gcode",
              "type": "machinecode",
              "typePath": ["machinecode", "gcode"],
              "hash": "...",
              "size": 100,
              "date": 1378847754,
              "origin": "local",
              "refs": {
                "resource": "http://example.com/api/files/local/folderA/subfolder/test2.gcode",
                "download": "http://example.com/downloads/files/local/folderA/subfolder/test2.gcode"
              }
            },
          ],
          "size": 100,
          "refs": {
            "resource": "http://example.com/api/files/local/folderA/subfolder",
          }
        }
      ],
      "size": 1334,
      "refs": {
        "resource": "http://example.com/api/files/local/folderA",
      }
    }
  ],
  "free": "3.2GB"
}"#
            )
        .create());

    MockFrame {
        mock,
        ..server
    }
}
