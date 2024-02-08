use std::collections::HashMap;
use octoprint_rs::types::printer_files::Entry;

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
  ],
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

    let _: Entry = deserialized.unwrap();
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

    let _: octoprint_rs::types::Dimension = deserialized.unwrap();
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

    let _: octoprint_rs::types::printer_files::GCodeAnalysisArea = deserialized.unwrap();
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

    let _: octoprint_rs::types::printer_files::GCodeAnalysisArea = deserialized.unwrap();
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

    let _: octoprint_rs::types::Dimension = deserialized.unwrap();
}

#[test]
fn parse_gcode_analysis_estimated_print_time() {
    let json = r#"1368.6617568899217"#;

    let result = &mut serde_json::Deserializer::from_str(json);
    let deserialized = serde_path_to_error::deserialize(result);

    let _: f64 = deserialized.unwrap();
}
