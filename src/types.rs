use serde::{Deserialize, Serialize};

//
// API VERSION
//

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiVersion {
    api: String,
    server: String,
    text: String,
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
    port: String,
    baudrate: u32,
    #[serde(rename = "printerProfile")]
    printer_profile: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrinterConnectionStateOptions {
    ports: Vec<String>,
    baudrates: Vec<u32>,
    #[serde(rename = "printerProfile")]
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

/// This is the struct that is used to model the JSON body of the request
///
/// `command`: The command to send to the printer. This can be one of the following:
///    * "`connect`"
///    * "`disconnect`"
///    * "`fake_ack`"
///
/// The following fields are only used if the command is "`connect`"
/// `port`: The port to connect to. Available ports can be gotten by running [`get_connection`]
/// `baudrate`: The baudrate to connect at. Available baudrates can be gotten by running [`get_connection`]
/// `printer_profile`: The printer profile to use. Available printer profiles can be gotten by running [`get_connection`]
/// `save`: Whether or not to save the connection settings as the new preference.
/// `autoconnect`: Whether or not to automatically connect to the printer on server startup.
#[derive(Serialize, Deserialize, Debug)]
pub struct PrinterConnectionPost {
    command: String,
    port: String,
    baudrate: u32,
    printer_profile: String,
    save: bool,
    autoconnect: bool,
}
