use serde::{Deserialize, Serialize};

//
// API VERSION
//

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiVersion {
    pub api: String,
    pub server: String,
    pub text: String,
}

//
// PRINTER CONNECTION GET
//

/// This is the struct 
#[derive(Serialize, Deserialize, Debug)]
pub struct PrinterConnection {
    current: PrinterConnectionStateCurrent,
    options: PrinterConnectionStateOptions,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrinterConnectionStateCurrent {
    state: String,
    port: Option<String>,
    baudrate: Option<u32>,
    #[serde(rename = "printerProfile")]
    printer_profile: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrinterConnectionStateOptions {
    ports: Vec<String>,
    baudrates: Vec<u32>,
    #[serde(rename = "printerProfiles")]
    printer_profiles: Vec<PrinterProfile>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrinterProfile {
    name: String,
    id: String,
}

//
// PRINTER CONNECTION SET 
//

/// This enum is used to model the connection commands that can be sent to the printer.
pub enum ConnectionCommandDescriptor {
    /// Run [`get_connection`](#method.get_connection) to get the available values 
    /// for `port`, `baud_rate` and `printer_profile`
    ///
    /// `port`: The port to connect to.
    /// `baudrate`: The baudrate to connect at.
    /// `printer_profile`: The printer profile to use.
    /// `save`: Whether or not to save the connection settings as the new preference.
    /// `autoconnect`: Whether or not to automatically connect to the printer on server startup.
    Connect {
        port: String,
        baudrate: u32,
        printer_profile: String,
        save: bool,
        autoconnect: bool,
    },
    Disconnect,
    FakeAck,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrinterConnectionPost {
    command: String,
    port: Option<String>,
    baudrate: Option<u32>,
    #[serde(rename = "printerProfile")]
    printer_profile: Option<String>,
    save: Option<bool>,
    autoconnect: Option<bool>,
}

impl ConnectionCommandDescriptor {
    /// Will convert the enum into a struct that can be deserialized to json and sent to the printer.
    pub fn to_post(self) -> PrinterConnectionPost {
        match self {
            ConnectionCommandDescriptor::Connect {
                port,
                baudrate,
                printer_profile,
                save,
                autoconnect,
            } => PrinterConnectionPost {
                command: "connect".to_string(),
                port: Some(port),
                baudrate: Some(baudrate),
                printer_profile: Some(printer_profile),
                save: Some(save),
                autoconnect: Some(autoconnect),
            },
            ConnectionCommandDescriptor::Disconnect => PrinterConnectionPost {
                command: "disconnect".to_string(),
                port: None,
                baudrate: None,
                printer_profile: None,
                save: None,
                autoconnect: None,
            },
            ConnectionCommandDescriptor::FakeAck => PrinterConnectionPost {
                command: "fake_ack".to_string(),
                port: None,
                baudrate: None,
                printer_profile: None,
                save: None,
                autoconnect: None,
            },
        }
    }
}

//
// GET PRINTER FILES
//

pub enum FilesLocation {
    Root,
    Local,
    Sdcard,
}

pub enum FileLocation {
    Local,
    Sdcard,
}

pub struct FilesFetchDescriptor {
    pub location: FilesLocation,
    pub recursive: bool,
    pub force: bool,
}

pub struct FileFetchDescriptor {
    pub location: FileLocation,
    pub path: String,
    pub recursive: bool,
    pub force: bool,
}

pub mod printer_files {
    use std::collections::HashMap;
    use serde::{Deserialize, Serialize};

    /// This is the struct that is returned when getting the files from the printer.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Files {
        #[serde(default)]
        files: Vec<Entry>,
        free: u64,
        total: u64,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(tag = "type")]
    pub enum Entry {
        #[serde(alias = "machinecode")]
        #[serde(alias = "model")]
        File {
            display: String,
            name: String,
            path: String,
            #[serde(default)]
            #[serde(rename = "typePath")]
            type_path: Vec<String>,
            origin: String,
            date: Option<u64>,
            hash: Option<String>,
            size: Option<u64>,
            refs: Option<Refs>,
            #[serde(rename = "gcodeAnalysis")]
            gcode_analysis: Option<GcodeAnalysis>,
            print: Option<PrintHistory>,
            statistics: Option<Statistics>,
        },
        #[serde(alias = "folder")]
        Folder {
            #[serde(default)]
            children: Vec<Entry>,
            display: String,
            name: String,
            origin: String,
            path: String,
            refs: Option<Refs>,
            #[serde(default)]
            #[serde(rename = "typePath")]
            type_path: Vec<String>,
            size: Option<u64>,
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Refs {
        resource: String,
        download: Option<String>,
        model: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct GcodeAnalysis {
        #[serde(rename = "estimatedPrintTime")]
        estimated_print_time: Option<f32>,
        filament: Option<GCodeAnalysisTools>,
        dimensions: Option<GCodeAnalysisDimensions>,
        printing_area: Option<GCodeAnalysisArea>,
        travel_area: Option<GCodeAnalysisArea>,
        travel_dimensions: Option<GCodeAnalysisDimensions>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct GCodeAnalysisDimensions {
        #[serde(rename = "width")]
        x: f32,
        #[serde(rename = "height")]
        y: f32,
        #[serde(rename = "depth")]
        z: f32,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct GCodeAnalysisArea {
        #[serde(rename = "minX")]
        min_x: f32,
        #[serde(rename = "minY")]
        min_y: f32,
        #[serde(rename = "minZ")]
        min_z: f32,
        #[serde(rename = "maxX")]
        max_x: f32,
        #[serde(rename = "maxY")]
        max_y: f32,
        #[serde(rename = "maxZ")]
        max_z: f32,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct GCodeAnalysisTool {
        length: f32,
        volume: f32,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct GCodeAnalysisTools {
        tool0: GCodeAnalysisTool,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PrintHistory {
        success: u32,
        failure: u32,
        last: PrintHistoryLast,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PrintHistoryLast {
        date: u64,
        #[serde(rename = "printTime")]
        print_time: Option<f32>,
        success: bool,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Statistics {
        #[serde(rename = "averagePrintTime")]
        average_print_time: HashMap<String, f32>,
        #[serde(rename = "lastPrintTime")]
        last_print_time: HashMap<String, f32>,
    }
}
