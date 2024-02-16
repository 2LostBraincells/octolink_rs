use octoprint_rs::types::{printer_files::Entry, ToolState, TemperatureHistoryEntry};

#[test]
fn parse_file() {
    let json = r#"
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
"#;

    let result = &mut serde_json::Deserializer::from_str(json);
    let deserialized = serde_path_to_error::deserialize(result);

    let _: Entry = deserialized.unwrap();
}

#[test]
fn parse_filament() {
    let json = r#"{
        "tool0": {
            "length": 1508.521400000127,
            "volume": 3.628419182080407
        }
    }"#;

    let result = &mut serde_json::Deserializer::from_str(json);
    let deserialized = serde_path_to_error::deserialize(result);

    let _: octoprint_rs::types::printer_files::GCodeAnalysisTools = deserialized.unwrap();
}

#[test]
fn parse_refs() {
    let json = r#"{
      "download": "http://127.0.0.1:5000/downloads/files/local/folder/pushrod.gcode",
      "resource": "http://127.0.0.1:5000/api/files/local/folder/pushrod.gcode"
    }"#;

    let result = &mut serde_json::Deserializer::from_str(json);
    let deserialized = serde_path_to_error::deserialize(result);

    let _: octoprint_rs::types::printer_files::Refs = deserialized.unwrap();
}

#[test]
fn parse_type_path() {
    let json = r#"[
      "machinecode",
      "gcode"
    ]"#;

    let result = &mut serde_json::Deserializer::from_str(json);
    let deserialized = serde_path_to_error::deserialize(result);

    let _: Vec<String> = deserialized.unwrap();
}

#[test]
fn parse_file_without_analysis() {
    let json = r#"{
      "date": 1707166498,
      "display": "pushrod.gcode",
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
    }"#;

    let result = &mut serde_json::Deserializer::from_str(json);
    let deserialized = serde_path_to_error::deserialize(result);

    let file: Entry = deserialized.unwrap();

    if let Entry::File { date, display, name, origin, path, refs, size, type_path, .. } = file {
        assert_eq!(date, Some(1707166498));
        assert_eq!(display, "pushrod.gcode");
        assert_eq!(name, "pushrod.gcode");
        assert_eq!(origin, "local");
        assert_eq!(path, "folder/pushrod.gcode");
        assert_eq!(refs.as_ref().unwrap().download, Some("http://127.0.0.1:5000/downloads/files/local/folder/pushrod.gcode".to_string()));
        assert_eq!(refs.unwrap().resource, "http://127.0.0.1:5000/api/files/local/folder/pushrod.gcode");
        assert_eq!(size, Some(801365));
        assert_eq!(type_path, vec!["machinecode".to_string(), "gcode".to_string()]);
    }
}

#[test]
fn parse_gcode_analysis() {
    let json = r#"{
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
    }"#;

    let result = &mut serde_json::Deserializer::from_str(json);
    let deserialized = serde_path_to_error::deserialize(result);

    let _: octoprint_rs::types::printer_files::GCodeAnalysis = deserialized.unwrap();
}

#[test]
fn parse_gcode_analysis_dimensions() {
    let json = r#"{
      "depth": 99.349,
      "height": 5.0,
      "width": 142.374
    }"#;

    let result = &mut serde_json::Deserializer::from_str(json);
    let deserialized = serde_path_to_error::deserialize(result);

    let dimension: octoprint_rs::types::Dimension = deserialized.unwrap();

    assert_eq!(dimension.depth, 99.349);
    assert_eq!(dimension.height, 5.0);
    assert_eq!(dimension.width, 142.374);
}

#[test]
fn parse_gcode_analysis_printing_area() {
    let json = r#"{
      "maxX": 170.0,
      "maxY": 97.349,
      "maxZ": 5.0,
      "minX": 27.626,
      "minY": -2.0,
      "minZ": 0.0
    }"#;

    let result = &mut serde_json::Deserializer::from_str(json);
    let deserialized = serde_path_to_error::deserialize(result);

    let area: octoprint_rs::types::printer_files::GCodeAnalysisArea = deserialized.unwrap();

    assert_eq!(area.max_x, 170.0);
    assert_eq!(area.max_y, 97.349);
    assert_eq!(area.max_z, 5.0);
    assert_eq!(area.min_x, 27.626);
    assert_eq!(area.min_y, -2.0);
    assert_eq!(area.min_z, 0.0);
}

#[test]
fn parse_gcode_analysis_travel_area() {
    let json = r#"{
      "maxX": 179.0,
      "maxY": 178.0,
      "maxZ": 35.0,
      "minX": 0.0,
      "minY": -2.0,
      "minZ": 0.0
    }"#;

    let result = &mut serde_json::Deserializer::from_str(json);
    let deserialized = serde_path_to_error::deserialize(result);

    let area: octoprint_rs::types::printer_files::GCodeAnalysisArea = deserialized.unwrap();

    assert_eq!(area.max_x, 179.0);
    assert_eq!(area.max_y, 178.0);
    assert_eq!(area.max_z, 35.0);
    assert_eq!(area.min_x, 0.0);
    assert_eq!(area.min_y, -2.0);
    assert_eq!(area.min_z, 0.0);
}

#[test]
fn parse_gcode_analysis_travel_dimensions() {
    let json = r#"{
      "depth": 180.0,
      "height": 35.0,
      "width": 179.0
    }"#;

    let result = &mut serde_json::Deserializer::from_str(json);
    let deserialized = serde_path_to_error::deserialize(result);

    let dimensions: octoprint_rs::types::Dimension = deserialized.unwrap();

    assert_eq!(dimensions.depth, 180.0);
    assert_eq!(dimensions.height, 35.0);
    assert_eq!(dimensions.width, 179.0);
}

#[test]
fn parse_gcode_analysis_estimated_print_time() {
    let json = r#"1368.6617568899217"#;

    let result = &mut serde_json::Deserializer::from_str(json);
    let deserialized = serde_path_to_error::deserialize(result);

    let estimated_time: Option<f32> = deserialized.unwrap();

    assert_eq!(estimated_time, Some(1368.6617568899217));
}

#[test]
fn parse_printer_temperatire() {
    let json = r#"{
    "tool0": {
      "actual": 214.8821,
      "target": 220.0,
      "offset": 0
    },
    "tool1": {
      "actual": 25.3,
      "target": null,
      "offset": 0
    },
    "bed": {
      "actual": 50.221,
      "target": 70.0,
      "offset": 5
    },
    "history": [
      {
        "time": 1395651928,
        "tool0": {
          "actual": 214.8821,
          "target": 220.0
        },
        "tool1": {
          "actual": 25.3,
          "target": null
        },
        "bed": {
          "actual": 50.221,
          "target": 70.0
        }
      },
      {
        "time": 1395651926,
        "tool0": {
          "actual": 212.32,
          "target": 220.0
        },
        "tool1": {
          "actual": 25.1,
          "target": null
        },
        "bed": {
          "actual": 49.1123,
          "target": 70.0
        }
      }
    ]
  }"#;

    let result = &mut serde_json::Deserializer::from_str(json);
    let deserialized = serde_path_to_error::deserialize(result);

    let temperatures: ToolState = deserialized.unwrap(); 

    assert_eq!(temperatures.tools.get("tool0").unwrap().actual, 214.8821);
    assert_eq!(temperatures.tools.get("tool1").unwrap().actual, 25.3);
    assert_eq!(temperatures.tools.get("bed").unwrap().actual, 50.221);
}

#[test]
fn parse_printer_temperature_history() {
    let json = r#"[
      {
        "time": 1395651928,
        "tool0": {
          "actual": 214.8821,
          "target": 220.0
        },
        "tool1": {
          "actual": 25.3,
          "target": null
        },
        "bed": {
          "actual": 50.221,
          "target": 70.0
        }
      },
      {
        "time": 1395651926,
        "tool0": {
          "actual": 212.32,
          "target": 220.0
        },
        "tool1": {
          "actual": 25.1,
          "target": null
        },
        "bed": {
          "actual": 49.1123,
          "target": 70.0
        }
      }
    ]"#;

    let result = &mut serde_json::Deserializer::from_str(json);
    let deserialized = serde_path_to_error::deserialize(result);

    let history: Vec<TemperatureHistoryEntry> = deserialized.unwrap();

    assert_eq!(history.len(), 2);
    assert_eq!(history[0].time, 1395651928);
    assert_eq!(history[1].time, 1395651926);
}
