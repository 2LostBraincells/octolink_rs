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
  "server": "1.9.3",
  "text": "OctoPrint 1.9.3"
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
            .with_status(200) .with_body(
                r#"{
  "current": {
    "baudrate": null,
    "port": null,
    "printerProfile": "_default",
    "state": "Closed"
  },
  "options": {
    "baudratePreference": null,
    "baudrates": [
      250000,
      230400,
      115200,
      57600,
      38400,
      19200,
      9600
    ],
    "portPreference": null,
    "ports": [
      "/dev/ttyS0",
      "/dev/ttyS1",
      "/dev/ttyS2",
      "/dev/ttyS3",
      "/dev/ttyS4",
      "/dev/ttyS5",
      "/dev/ttyS6",
      "/dev/ttyS7",
      "/dev/ttyS8",
      "/dev/ttyS9",
      "/dev/ttyS10",
      "/dev/ttyS11",
      "/dev/ttyS12",
      "/dev/ttyS13",
      "/dev/ttyS14",
      "/dev/ttyS15",
      "/dev/ttyS16",
      "/dev/ttyS17",
      "/dev/ttyS18",
      "/dev/ttyS19",
      "/dev/ttyS20",
      "/dev/ttyS21",
      "/dev/ttyS22",
      "/dev/ttyS23",
      "/dev/ttyS24",
      "/dev/ttyS25",
      "/dev/ttyS26",
      "/dev/ttyS27",
      "/dev/ttyS28",
      "/dev/ttyS29",
      "/dev/ttyS30",
      "/dev/ttyS31"
    ],
    "printerProfilePreference": "_default",
    "printerProfiles": [
      {
        "id": "_default",
        "name": "Default"
      }
    ]
  }
}"#,
            )
            .create(),
    );

    MockFrame { mock, ..server }
}

pub fn mock_post_api_connection() -> MockFrame {
    let mut server = mock_base();

    let mock = Some(
        server
            .server
            .mock("POST", "/api/connection")
            .match_header("X-Api-Key", server.api_key.as_str())
            .with_status(204)
            .create(),
    );

    MockFrame { mock, ..server }
}

pub fn mock_get_api_files() -> MockFrame {
    let mut server = mock_base();

    let mock = Some(
        server
            .server
            .mock("GET", "/api/files")
            .match_header("X-Api-Key", server.api_key.as_str())
            .with_status(200)
            .with_body(
                r#"{
  "files": [
    {
      "children": [],
      "display": "folder",
      "name": "folder",
      "origin": "local",
      "path": "folder",
      "refs": {
        "resource": "http://127.0.0.1:5000/api/files/local/folder"
      },
      "size": 801365,
      "type": "folder",
      "typePath": [
        "folder"
      ]
    },
    {
      "date": 1707166449,
      "display": "pushrod.gcode",
      "gcodeAnalysis": {
        "dimensions": {
          "depth": 99.349,
          "height": 5.0,
          "width": 142.374
        },
        "estimatedPrintTime": 1368.6617568899217,
        "filament": {
          "tool0": {
            "length": 1508.521400000127,
            "volume": 3.628419182080407
          }
        },
        "printingArea": {
          "maxX": 170.0,
          "maxY": 97.349,
          "maxZ": 5.0,
          "minX": 27.626,
          "minY": -2.0,
          "minZ": 0.0
        },
        "travelArea": {
          "maxX": 179.0,
          "maxY": 178.0,
          "maxZ": 35.0,
          "minX": 0.0,
          "minY": -2.0,
          "minZ": 0.0
        },
        "travelDimensions": {
          "depth": 180.0,
          "height": 35.0,
          "width": 179.0
        }
      },
      "name": "pushrod.gcode",
      "origin": "local",
      "path": "pushrod.gcode",
      "refs": {
        "download": "http://127.0.0.1:5000/downloads/files/local/pushrod.gcode",
        "resource": "http://127.0.0.1:5000/api/files/local/pushrod.gcode"
      },
      "size": 801365,
      "type": "machinecode",
      "typePath": [
        "machinecode",
        "gcode"
      ]
    }
  ],
  "free": 423822610432,
  "total": 499031998464
}"#,
            )
            .create(),
    );

    MockFrame { mock, ..server }
}

pub fn mock_get_api_files_q_recursive() -> MockFrame {
    let mut server = mock_base();

    let mock = Some(
        server
            .server
            .mock("GET", "/api/files?recursive=true")
            .match_header("X-Api-Key", server.api_key.as_str())
            .with_status(200)
            .with_body(
                r#"{
  "files": [
    {
      "children": [
        {
          "date": 1707166498,
          "display": "pushrod.gcode",
          "gcodeAnalysis": {
            "dimensions": {
              "depth": 99.349,
              "height": 5.0,
              "width": 142.374
            },
            "estimatedPrintTime": 1368.6617568899217,
            "filament": {
              "tool0": {
                "length": 1508.521400000127,
                "volume": 3.628419182080407
              }
            },
            "printingArea": {
              "maxX": 170.0,
              "maxY": 97.349,
              "maxZ": 5.0,
              "minX": 27.626,
              "minY": -2.0,
              "minZ": 0.0
            },
            "travelArea": {
              "maxX": 179.0,
              "maxY": 178.0,
              "maxZ": 35.0,
              "minX": 0.0,
              "minY": -2.0,
              "minZ": 0.0
            },
            "travelDimensions": {
              "depth": 180.0,
              "height": 35.0,
              "width": 179.0
            }
          },
          "name": "pushrod.gcode",
          "origin": "local",
          "path": "folder/pushrod.gcode",
          "refs": {
            "download": "http://127.0.0.1:5000/downloads/files/local/folder/pushrod.gcode",
            "resource": "http://127.0.0.1:5000/api/files/local/folder/pushrod.gcode"
          },
          "size": 801365,
          "type": "machinecode",
          "typePath": [
            "machinecode",
            "gcode"
          ]
        }
      ],
      "display": "folder",
      "name": "folder",
      "origin": "local",
      "path": "folder",
      "refs": {
        "resource": "http://127.0.0.1:5000/api/files/local/folder"
      },
      "size": 801365,
      "type": "folder",
      "typePath": [
        "folder"
      ]
    },
    {
      "date": 1707166449,
      "display": "pushrod.gcode",
      "gcodeAnalysis": {
        "dimensions": {
          "depth": 99.349,
          "height": 5.0,
          "width": 142.374
        },
        "estimatedPrintTime": 1368.6617568899217,
        "filament": {
          "tool0": {
            "length": 1508.521400000127,
            "volume": 3.628419182080407
          }
        },
        "printingArea": {
          "maxX": 170.0,
          "maxY": 97.349,
          "maxZ": 5.0,
          "minX": 27.626,
          "minY": -2.0,
          "minZ": 0.0
        },
        "travelArea": {
          "maxX": 179.0,
          "maxY": 178.0,
          "maxZ": 35.0,
          "minX": 0.0,
          "minY": -2.0,
          "minZ": 0.0
        },
        "travelDimensions": {
          "depth": 180.0,
          "height": 35.0,
          "width": 179.0
        }
      },
      "name": "pushrod.gcode",
      "origin": "local",
      "path": "pushrod.gcode",
      "refs": {
        "download": "http://127.0.0.1:5000/downloads/files/local/pushrod.gcode",
        "resource": "http://127.0.0.1:5000/api/files/local/pushrod.gcode"
      },
      "size": 801365,
      "type": "machinecode",
      "typePath": [
        "machinecode",
        "gcode"
      ]
    }
  ],
  "free": 423822696448,
  "total": 499031998464
}"#,
            )
            .create(),
    );

    MockFrame { mock, ..server }
}

pub fn mock_get_api_files_local() -> MockFrame {
    let mut server = mock_base();

    let mock = Some(
        server
            .server
            .mock("GET", "/api/files/local")
            .match_header("X-Api-Key", server.api_key.as_str())
            .with_status(200)
            .with_body(
                r#"{
  "files": [
    {
      "children": [],
      "display": "folder",
      "name": "folder",
      "origin": "local",
      "path": "folder",
      "refs": {
        "resource": "http://127.0.0.1:5000/api/files/local/folder"
      },
      "size": 4614729,
      "type": "folder",
      "typePath": [
        "folder"
      ]
    },
    {
      "date": 1707166449,
      "display": "pushrod.gcode",
      "gcodeAnalysis": {
        "dimensions": {
          "depth": 99.349,
          "height": 5.0,
          "width": 142.374
        },
        "estimatedPrintTime": 1368.6617568899217,
        "filament": {
          "tool0": {
            "length": 1508.521400000127,
            "volume": 3.628419182080407
          }
        },
        "printingArea": {
          "maxX": 170.0,
          "maxY": 97.349,
          "maxZ": 5.0,
          "minX": 27.626,
          "minY": -2.0,
          "minZ": 0.0
        },
        "travelArea": {
          "maxX": 179.0,
          "maxY": 178.0,
          "maxZ": 35.0,
          "minX": 0.0,
          "minY": -2.0,
          "minZ": 0.0
        },
        "travelDimensions": {
          "depth": 180.0,
          "height": 35.0,
          "width": 179.0
        }
      },
      "name": "pushrod.gcode",
      "origin": "local",
      "path": "pushrod.gcode",
      "refs": {
        "download": "http://127.0.0.1:5000/downloads/files/local/pushrod.gcode",
        "resource": "http://127.0.0.1:5000/api/files/local/pushrod.gcode"
      },
      "size": 801365,
      "type": "machinecode",
      "typePath": [
        "machinecode",
        "gcode"
      ]
    }
  ],
  "free": 423737376768,
  "total": 499031998464
}
                   "#,
            )
            .create(),
    );

    MockFrame { mock, ..server }
}

pub fn mock_get_api_files_local_folder_printed() -> MockFrame {
    let mut server = mock_base();

    let mock = Some(
        server
            .server
            .mock("GET", "/api/files/local/folder/printed.gcode")
            .match_header("X-Api-Key", server.api_key.as_str())
            .with_status(200)
            .with_body(
                r#"
{
  "date": 1707214053,
  "display": "printed.gcode",
  "gcodeAnalysis": {
    "dimensions": {
      "depth": 233.447,
      "height": 30.8,
      "width": 222.0
    },
    "estimatedPrintTime": 4693.44709714754,
    "filament": {
      "tool0": {
        "length": 15976.600509998501,
        "volume": 38.42822763728064
      }
    },
    "printingArea": {
      "maxX": 240.0,
      "maxY": 234.447,
      "maxZ": 30.8,
      "minX": 18.0,
      "minY": 1.0,
      "minZ": 0.0
    },
    "travelArea": {
      "maxX": 240.0,
      "maxY": 265.0,
      "maxZ": 130.8,
      "minX": 0.0,
      "minY": -3.0,
      "minZ": -1.5
    },
    "travelDimensions": {
      "depth": 268.0,
      "height": 132.3,
      "width": 240.0
    }
  },
  "name": "printed.gcode",
  "origin": "local",
  "path": "folder/printed.gcode",
  "refs": {
    "download": "http://127.0.0.1:5000/downloads/files/local/folder/printed.gcode",
    "resource": "http://127.0.0.1:5000/api/files/local/folder/printed.gcode"
  },
  "size": 3813364,
  "type": "machinecode",
  "typePath": [
    "machinecode",
    "gcode"
  ]
}            "#,
            )
            .create(),
    );

    MockFrame { mock, ..server }
}
