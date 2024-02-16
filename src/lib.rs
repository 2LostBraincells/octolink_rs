use errors::*;
use reqwest::{Client, StatusCode};
use types::*;

pub mod errors;
pub mod types;

pub struct Printer {
    address: String,
    port: u16,
    api_key: String,
    client: Client,
}

pub struct PrinterBuilder {
    address: String,
    port: u16,
    api_key: String,
    client: Client,
}

impl PrinterBuilder {
    /// Creates a new `PrinterBuilder` from an `address` and an `api_key`.
    /// `port` can be set with the [`port()`](#method.port) method and efaults to `80`.
    pub fn new<P: ToString, P2: ToString>(address: P2, api_key: P) -> Self {
        Self {
            address: address.to_string(),
            port: 80,
            api_key: api_key.to_string(),
            client: Client::new(),
        }
    }

    /// Set the `port` of the printer
    /// If this is not set, it will default to `80`
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Build the `Printer` struct
    pub fn build(self) -> Printer {
        Printer {
            address: self.address,
            port: self.port,
            api_key: self.api_key,
            client: self.client,
        }
    }
}

impl Printer {
    //
    //  INFO: General printer information
    //

    /// Returns a struct representing the current api version.
    ///
    /// # Errors
    ///
    /// If there is an error, it will return a `InformationRequestError`
    /// * `ReqwestError` - If the request fails
    /// * `ParseError` - If the response can not be parsed
    pub async fn get_api_version(&self) -> Result<types::ApiVersion, InformationRequestError> {
        let url = format!("http://{}:{}/api/version", self.address, self.port);

        dbg!(&url);

        let res = self
            .client
            .get(&url)
            .header("X-Api-Key", &self.api_key)
            .send()
            .await;

        res.map_err(InformationRequestError::ReqwestError)?
            .json::<types::ApiVersion>()
            .await
            .map_err(|e| InformationRequestError::ParseError(e.to_string()))
    }

    //
    //  INFO: Connection settings
    //

    /// Returns a struct containing the current connection settings of the printer
    /// as well as the available printer settings
    ///
    /// # Errors
    ///
    /// If there is an error, it will return a `InformationRequestError`
    /// * `ReqwestError` - If the request fails
    /// * `ParseError` - If the response can not be parsed
    pub async fn get_connection(
        &self,
    ) -> Result<types::PrinterConnection, InformationRequestError> {
        let url = format!("http://{}:{}/api/connection", self.address, self.port);
        let res = self
            .client
            .get(&url)
            .header("X-Api-Key", &self.api_key)
            .send()
            .await;

        res.map_err(InformationRequestError::ReqwestError)?
            .json::<types::PrinterConnection>()
            .await
            .map_err(|e| InformationRequestError::ParseError(e.to_string()))
    }

    /// Set the connection settings of the printer
    ///
    /// # Arguments
    ///
    /// Takes in a [`ConnectionCommandDescriptor`](types::ConnectionCommandDescriptor) and sends it to the printer in the form of json
    ///
    /// # Errors
    ///
    /// If the request fails, it will return a `SetConnectionError`
    /// * `ReqwestError` - If the request fails
    /// * `ParseError` - If the response can not be parsed
    pub async fn set_connection(
        &self,
        connection: types::ConnectionCommandDescriptor,
    ) -> Result<(), SetConnectionError> {
        let url = format!("http://{}:{}/api/connection", self.address, self.port);
        let res = self
            .client
            .post(&url)
            .header("X-Api-Key", &self.api_key)
            .json(&connection.to_post())
            .send()
            .await;

        let body = res.map_err(SetConnectionError::ReqwestError)?;
        if body.status().is_success() {
            Ok(())
        } else {
            let text = body
                .text()
                .await
                .map_err(SetConnectionError::ReqwestError)?;
            Err(SetConnectionError::BadRequest(text))
        }
    }

    //
    //  INFO: File operations
    //

    /// Will get the printer files and folders from the specified folder
    ///
    /// # Arguments
    ///
    /// * `files_descriptor` - A struct containing the location of the files and if the request should be recursive
    ///     * `location` - The location of the files, either `Root`, `Local` or `Sdcard`
    ///     * `force` - If the request should force the printer to refresh the cache
    ///     * `recursive` - If the request should be recursive
    ///
    /// # Errors
    pub async fn get_files(
        &self,
        files_descriptor: types::FilesFetchDescriptor,
    ) -> Result<types::printer_files::Files, FileRequestError> {
        let location = match files_descriptor.location {
            types::FilesLocation::Root => "",
            types::FilesLocation::Local => "/local",
            types::FilesLocation::Sdcard => "/sdcard",
        };

        let query_params = if files_descriptor.force {
            if files_descriptor.recursive {
                "?force=true&recursive=true"
            } else {
                "?force=true"
            }
        } else if files_descriptor.recursive {
            "?recursive=true"
        } else {
            ""
        };

        let url = format!(
            "http://{}:{}/api/files{}{}",
            self.address, self.port, location, query_params
        );

        dbg!(&url);

        let res = self
            .client
            .get(&url)
            .header("X-Api-Key", &self.api_key)
            .send()
            .await;

        let body = res.map_err(FileRequestError::ReqwestError)?;
        let status = body.status();

        if status.is_server_error() {
            return Err(FileRequestError::ServerError);
        }

        let text = body
            .text()
            .await
            .map_err(FileRequestError::ReqwestError)?;

        if status.is_success() {
            let result = &mut serde_json::Deserializer::from_str(text.as_str());
            let deserialized = serde_path_to_error::deserialize(result)
                .map_err(|e| FileRequestError::ParseError(e.to_string()))?;
            return Ok(deserialized);
        }
        Err(FileRequestError::NotFound(text))
    }

    //  TODO: Make file uploads work

    /// Gets a single file or folder from the printer
    ///
    /// # Arguments
    ///
    /// `file_descriptor` - A struct describing the file to get and how to get it
    ///    - `location` - The location of the file, either `Local` or `Sdcard`
    ///    - `path` - The location of the file
    ///    - `force` - If the request should force the printer to refresh the cache
    ///    - `recursive` - If the request should be recursive
    ///
    /// # Errors
    ///
    /// * `ReqwestError` - If the request fails
    /// * `ParseError` - If the response can not be parsed
    /// * `NoFile` - If the file does not exist
    pub async fn get_file(
        &self,
        file_descriptor: types::FileFetchDescriptor,
    ) -> Result<types::printer_files::Entry, FileRequestError> {
        let location = match file_descriptor.path.location {
            types::FileLocation::Local => "local/",
            types::FileLocation::Sdcard => "sdcard/",
        };

        let path = file_descriptor
            .path
            .path
            .strip_prefix('/')
            .unwrap_or(&file_descriptor.path.path);

        let query_params = if file_descriptor.force {
            if file_descriptor.recursive {
                "?force=true&recursive=true"
            } else {
                "?force=true"
            }
        } else if file_descriptor.recursive {
            "?recursive=true"
        } else {
            ""
        };

        let url = format!(
            "http://{}:{}/api/files/{}{}{}",
            self.address, self.port, location, path, query_params,
        );

        let res = self
            .client
            .get(&url)
            .header("X-Api-Key", &self.api_key)
            .send()
            .await;

        let body = res.map_err(FileRequestError::ReqwestError)?;
        let status = body.status();

        let text = body
            .text()
            .await
            .map_err(FileRequestError::ReqwestError)?;
        match status {
            StatusCode::NOT_FOUND => Err(FileRequestError::NotFound(text)),
            StatusCode::INTERNAL_SERVER_ERROR => Err(FileRequestError::ServerError),
            _ => {
                let result = &mut serde_json::Deserializer::from_str(text.as_str());
                let deserialized = serde_path_to_error::deserialize(result)
                    .map_err(|e| FileRequestError::ParseError(e.to_string()))?;
                Ok(deserialized)
            }
        }
    }

    /// Will issue a file command to the printer.
    /// This can either be `Select`, `Deselect`, `Move` or `Copy`.
    ///
    /// If you want to `select` a file, you need to specify weather you want to print it or not.
    /// `deselect` does something but I'm not sure what.
    ///
    /// If you want to copy or move a file, you need to provide a `destination` path.
    /// The `destination` path should be the full path will not include `"/local"` or `"/sdcard"` at the
    /// start.
    ///
    /// # Errors
    ///
    /// If something goes wrong, it will return a `FileCommandError`
    /// * `ReqwestError` - If the request fails
    /// * `Conflict` - If the server responds with a `409` status code
    /// * `BadRequest` - If the server responds with a `400` or `500` status code.
    ///    Make sure to check your destination path.
    ///
    /// # Example
    /// ```
    /// # use octoprint_rs::types::FileCommandDescriptor;
    /// # use octoprint_rs::types::FileCommand;
    /// # use octoprint_rs::types::FileLocation;
    /// # use octoprint_rs::types::PathDescriptor;
    /// # use octoprint_rs::PrinterBuilder;
    /// let printer = PrinterBuilder::new("localhost", "API_KEY")
    ///     .port(5000)
    ///     .build();   
    ///
    /// //Note that this will return a future that can be awaited.
    /// printer.issue_file_command(FileCommandDescriptor {
    ///     command: FileCommand::Copy {
    ///         destination: "/folder".to_string()
    ///     },
    ///     path: PathDescriptor {
    ///         location: FileLocation::Local,
    ///         path: "/folder/file.gcode".to_string()
    ///     },
    /// });
    pub async fn issue_file_command(
        &self,
        command: types::FileCommandDescriptor,
    ) -> Result<(), FileCommandError> {
        // format url
        let location = match command.path.location {
            types::FileLocation::Local => "local/",
            types::FileLocation::Sdcard => "sdcard/",
        };

        let path = command
            .path
            .path
            .strip_prefix('/')
            .unwrap_or(&command.path.path);

        let url = format!(
            "http://{}:{}/api/files/{}{}",
            self.address, self.port, location, path
        );

        let res = self
            .client
            .post(&url)
            .header("X-Api-Key", &self.api_key)
            .json(&command.to_post())
            .send()
            .await;

        let body = res.map_err(FileCommandError::ReqwestError)?;
        let status = body.status();

        // Handle success case
        if status.is_success() {
            return Ok(());
        }

        // Handle error case
        let text = body
            .text()
            .await
            .map_err(FileCommandError::ReqwestError)?;
        match status {
            StatusCode::INTERNAL_SERVER_ERROR | StatusCode::BAD_REQUEST => {
                Err(FileCommandError::BadRequest(text))
            }
            StatusCode::CONFLICT => Err(FileCommandError::Conflict(text)),
            _ => unreachable!(),
        }
    }

    /// Deletes a file from the printer
    pub async fn delete_file(&self, path: types::PathDescriptor) -> Result<(), FileDeletionError> {
        let location = match path.location {
            types::FileLocation::Local => "local/",
            types::FileLocation::Sdcard => "sdcard/",
        };

        let path = path.path.strip_prefix('/').unwrap_or(&path.path);

        let url = format!(
            "http://{}:{}/api/files/{}{}",
            self.address, self.port, location, path
        );

        let res = self
            .client
            .delete(&url)
            .header("X-Api-Key", &self.api_key)
            .send()
            .await;

        let body = res.map_err(FileDeletionError::ReqwestError)?;
        let status = body.status();

        // Handle success case
        if status.is_success() {
            return Ok(());
        }

        // Handle error case
        let text = body
            .text()
            .await
            .map_err(FileDeletionError::ReqwestError)?;
        match status {
            StatusCode::NOT_FOUND => Err(FileDeletionError::NotFound(text)),
            StatusCode::CONFLICT => Err(FileDeletionError::Conflict(text)),
            StatusCode::INTERNAL_SERVER_ERROR => Err(FileDeletionError::ServerError),
            _ => unreachable!(),
        }
    }

    //
    //  INFO: Job operations
    //

    /// Will issue a job command to the printer.
    ///
    /// # Arguments
    ///
    /// `command` - A [`JobCommand`] representing the command to issue
    ///
    /// # Errors
    ///
    /// If something goes wrong, it will return a [`JobCommandError`]
    /// * `ReqwestError` - If the request fails
    /// * `Conflict` - If the server responds with a `409` status code
    /// This can happen if the printer is already printing and you try to start a new print
    /// or delete the file its currently printing.
    pub async fn issue_job_command(&self, command: JobCommand) -> Result<(), JobCommandError> {
        let url = "/api/job";

        let res = self
            .client
            .post(url)
            .header("X-Api-Key", &self.api_key)
            .json(&command.to_raw_command())
            .send()
            .await;

        let body = res.map_err(JobCommandError::ReqwestError)?;
        match body.status() {
            StatusCode::NO_CONTENT => Ok(()),
            StatusCode::INTERNAL_SERVER_ERROR => Err(JobCommandError::ServerError),
            StatusCode::CONFLICT => {
                let text = body
                    .text()
                    .await
                    .map_err(JobCommandError::ReqwestError)?;
                Err(JobCommandError::Conflict(text))
            }
            _ => unreachable!(),
        }
    }

    /// Gets the current job information from the printer
    ///
    /// # Returns
    ///
    /// If there is no error, this function returns a [`JobInformation`](types::JobInformation) struct representing the current job information.
    ///
    /// # Errors
    ///
    /// If there is an error, it will return a `InformationRequestError`
    /// * `ReqwestError` - If the request fails
    /// * `ParseError` - If the response can not be parsed.
    /// This can happen if the wrapper is outdated and they changed something in the api. (cry about it)
    ///
    /// # Example
    ///
    /// ```
    /// # use octoprint_rs::PrinterBuilder;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let printer = PrinterBuilder::new("localhost", "API_KEY")
    ///     .port(42069)
    ///     .build();
    ///
    /// let job = printer
    ///     .get_job()
    ///     .await;
    /// # }
    pub async fn get_job(&self) -> Result<types::JobInformation, InformationRequestError> {
        let url = "/api/job";

        let res = self
            .client
            .get(url)
            .header("X-Api-Key", &self.api_key)
            .send()
            .await;

        let body = res.map_err(InformationRequestError::ReqwestError)?;
        if body.status().is_server_error() {
            return Err(InformationRequestError::ServerError);
        }

        let text = body
            .text()
            .await
            .map_err(InformationRequestError::ReqwestError)?;
        let result = &mut serde_json::Deserializer::from_str(text.as_str());
        let deserialized = serde_path_to_error::deserialize(result)
            .map_err(|e| InformationRequestError::ParseError(e.to_string()))?;
        Ok(deserialized)
    }

    //
    //  NOTE: PRINTER COMMANDS
    //

    /// Gets the current printer telemetry.
    ///
    /// # Returns
    ///
    /// This function returns a [`RawPrinter`](types::RawPrinter) struct representing the current printer telemetry.
    ///
    /// # Errors
    ///
    /// If there is an error this function will return a [`PrinterCommandError`](errors::PrinterCommandError) enum.
    /// * `ReqwestError` - If the request fails
    /// * `Conflict` - If the server responds with a `409` status code. This can happen if the
    /// printer is not connected.
    ///
    /// # Example
    ///
    /// ```
    /// # use octoprint_rs::PrinterBuilder;
    /// # #[tokio::main]
    /// # async fn main() {
    /// use static_assertions::assert_type_eq_all;
    ///
    /// let printer = PrinterBuilder::new("localhost", "API_KEY")
    ///     .port(42069)
    ///     .build();
    ///
    /// let telemetry = printer
    ///     .get_printer_telemetry()
    ///     .await;
    /// # }
    pub async fn get_printer_telemetry(&self) -> Result<types::RawPrinter, DeviceStateError> {
        let url = format!("http://{}:{}/api/printer", self.address, self.port);

        let res = self
            .client
            .get(&url)
            .header("X-Api-Key", &self.api_key)
            .send()
            .await;

        let body = res.map_err(DeviceStateError::ReqwestError)?;
        let status = body.status();
        let text = body
            .text()
            .await
            .map_err(DeviceStateError::ReqwestError)?;
        match status {
            StatusCode::INTERNAL_SERVER_ERROR => Err(DeviceStateError::ServerError),
            StatusCode::CONFLICT => Err(DeviceStateError::Conflict(text)),
            _ => {
                let result = &mut serde_json::Deserializer::from_str(text.as_str());
                let deserialized = serde_path_to_error::deserialize(result)
                    .map_err(|e| DeviceStateError::ParseError(e.to_string()))?;
                Ok(deserialized)
            }
        }
    }

    //
    //  NOTE: PRINTHEAD COMMANDS
    //

    /// Moves the printhead to the specified location
    ///
    /// # Arguments
    ///
    /// `command` - A [`PrintheadMoveDescriptor`](types::PrintheadMoveDescriptor) representing the location to move the printhead to.
    /// This can be one of two things:
    /// * `relative` - A struct representing the relative location to move the printhead to.
    /// * `home` - A struct representing which axes will home.
    ///
    /// # Errors
    ///
    /// If there is an error, it will return a [`PrintheadMoveError`](errors::PrintheadMoveError) enum.
    /// * `ReqwestError` - If the request fails
    /// * `BadRequest` - If the server responds with a `400` status code. Can happen if you give
    /// it implossible values.
    /// * `Conflict` - If the server responds with a `409` status code. This means the printer is
    /// currently printing.
    ///
    /// # Example
    ///
    /// ```
    /// # use octoprint_rs::PrinterBuilder;
    ///
    pub async fn move_printhead(
        &self,
        command: types::PrintheadMoveDescriptor,
    ) -> Result<(), ToolCommandError> {
        let url = format!(
            "http://{}:{}/api/printer/printhead",
            self.address, self.port
        );

        let res = self
            .client
            .post(&url)
            .header("X-Api-Key", &self.api_key)
            .json(&command.to_post())
            .send()
            .await
            .map_err(ToolCommandError::ReqwestError)?;

        match res.status() {
            StatusCode::NO_CONTENT => Ok(()),
            StatusCode::INTERNAL_SERVER_ERROR => Err(ToolCommandError::ServerError),
            status @ StatusCode::BAD_REQUEST | status @ StatusCode::CONFLICT => {
                let text = res
                    .text()
                    .await
                    .map_err(ToolCommandError::ReqwestError)?;
                Err(match status {
                    StatusCode::BAD_REQUEST => ToolCommandError::BadRequest(text),
                    StatusCode::CONFLICT => ToolCommandError::Conflict(text),
                    _ => unreachable!(),
                })
            }
            _ => unreachable!(),
        }
    }

    /// Changes the feedrate of the printhead.
    ///
    /// # Arguments
    ///
    /// `factor` - A `f32` representing the factor to change the feedrate by. This will always be
    /// relative to 1.0 or 100%.
    /// This can be between `0.5` and `2.0`.
    ///
    /// # Errors
    ///
    /// If there is an error, it will return a [`PrintheadCommandError`](errors::PrintheadCommandError) enum.
    /// * `ReqwestError` - If the request fails
    /// * `BadRequest` - If the server responds with a `400` StatusCode. This means you didnt give
    /// it the a valid factor.
    /// * `Conflict` - If the server responds with a `409` StatusCode. This means the printer is
    /// currently printing
    pub async fn change_printhead_feedrate(&self, factor: f32) -> Result<(), ToolCommandError> {
        if !(0.5..=2.0).contains(&factor) {
            return Err(ToolCommandError::BadRequest(
                "Feedrate factor must be between 0.5 and 2.0".to_string(),
            ));
        }

        let url = format!(
            "http://{}:{}/api/printer/printhead",
            self.address, self.port
        );

        let res = self
            .client
            .post(&url)
            .header("X-Api-Key", &self.api_key)
            .json(&types::PrintheadCommand::from_feedrate(factor))
            .send()
            .await
            .map_err(ToolCommandError::ReqwestError)?;

        match res.status() {
            StatusCode::NO_CONTENT => Ok(()),
            StatusCode::INTERNAL_SERVER_ERROR => Err(ToolCommandError::ServerError),
            status @ StatusCode::BAD_REQUEST | status @ StatusCode::CONFLICT => {
                let text = res
                    .text()
                    .await
                    .map_err(ToolCommandError::ReqwestError)?;
                Err(match status {
                    StatusCode::BAD_REQUEST => ToolCommandError::BadRequest(text),
                    StatusCode::CONFLICT => ToolCommandError::Conflict(text),
                    _ => unreachable!(),
                })
            }
            _ => unreachable!(),
        }
    }

    //
    //  NOTE: TOOL COMMANDS
    //

    /// Used to change the temperature of a specified tool.
    /// This tools is usually called tool0 but you can get the tools by running
    /// [`get_tool_state()`](#method.get_tool_state).
    ///
    /// # Arguments
    ///
    /// This function takes in a [`ToolTempDescriptor`](types::ToolTempDescriptor) which has two
    /// variants, `Target` and `Offset`,
    ///
    /// Both of these variants have a `tool` field which is the target tool. This can be gotten by 
    /// running [`get_tool_state()`](#method.get_tool_state).
    ///
    /// `Offset` has two fields:
    /// * `tool` - The target tool.
    /// * `offset` - The amount to offset the temperature by
    ///
    /// `Target` has two fields:
    /// * `tool` - The target tool.
    /// * `target` - The target temperature.
    ///
    /// # Errors 
    /// 
    /// If there is an error, it will return a [`ToolCommandError`](errors::ToolCommandError) enum.
    /// * `ReqwestError` - If the request fails.
    /// * `BadRequest` - If the server responds with a `400` StatusCode. This usually means you
    /// didnt give it a valid tool. Run [`get_tool_state()`](#method.get_tool_state) to get the
    /// valid tools.
    /// * `Conflict` - If the server responds with a `409` StatusCode. This usually means there was a
    /// conflict and the printer is currently printing.
    pub async fn tool_temperature(
        &self,
        command: ToolTempDescriptor,
    ) -> Result<(), ToolCommandError> {
        let url = format!("http://{}:{}/api/printer/tool", &self.address, &self.port);

        let res = self
            .client
            .post(&url)
            .header("X-Api-Key", &self.api_key)
            .json(&command.to_json())
            .send()
            .await
            .map_err(ToolCommandError::ReqwestError)?;

        let status = res.status();

        if status.is_success() {
            return Ok(());
        }
        if status.is_server_error() {
            return Err(ToolCommandError::ServerError);
        }

        let text = res
            .text()
            .await
            .map_err(ToolCommandError::ReqwestError)?;

        match status {
            StatusCode::BAD_REQUEST => Err(ToolCommandError::BadRequest(text)),
            StatusCode::CONFLICT => Err(ToolCommandError::Conflict(text)),
            _ => unreachable!(),
        }
    }

    /// Gets the current state of all the tools on the printer.
    ///
    /// # Arguments
    ///
    /// `history` - An optional `u32` representing the amount of history to get. If this is `None`,
    /// it will not return any history.
    ///
    /// # Errors 
    ///
    /// If there is an error, it will return a [`DeviceStateError`](errors::DeviceStateError) enum.
    /// * `ReqwestError` - If the request fails.
    /// * `ParseError` - If the response can not be parsed. This usually means the wrapper is 
    /// outdated and they changed something in the api. (cry about it)
    /// * `Conflict` - If the server responds with a `409` StatusCode. This usually means the
    /// printer is not operational or not connected.
    pub async fn get_tool_state(
        &self,
        history: Option<u32>,
    ) -> Result<ToolState, DeviceStateError> {
        let query: String;
        if let Some(x) = history {
            query = format!("true&limit={}", x);
        } else {
            query = "false".to_string();
        }

        let url = format!(
            "http://{}:{}/api/printer?history={}",
            &self.address, &self.port, query
        );

        let res = self
            .client
            .get(&url)
            .header("X-Api-Key", &self.api_key)
            .send()
            .await
            .map_err(DeviceStateError::ReqwestError)?;

        let status = res.status();
        let text = res
            .text()
            .await
            .map_err(DeviceStateError::ReqwestError)?;

        match status {
            StatusCode::CONFLICT => Err(DeviceStateError::Conflict(text)),
            status if status.is_success() => {
                let result = &mut serde_json::Deserializer::from_str(text.as_str());
                let deserialized = serde_path_to_error::deserialize(result)
                    .map_err(|e| DeviceStateError::ParseError(e.to_string()))?;
                Ok(deserialized)
            }
            _ => unreachable!(),
        }
    }

    /// Will select a tool on the printer. The selected tool can then be used to extrude or
    /// retract filament, for example.
    ///
    /// # Arguments
    ///
    /// `tool` - A `String` representing the tool to select. Available tools can be gotten by 
    /// running [`get_tool_state()`](#method.get_tool_state).
    ///
    /// # Errors 
    ///
    /// If there is an error, it will return a [`ToolCommandError`](errors::ToolCommandError) enum.
    /// * `ReqwestError` - If the request fails.
    /// * `BadRequest` - If the server responds with a `400` StatusCode. This usually means you
    /// specified an invalid tool. Run [`get_tool_state()`](#method.get_tool_state) to get the valid
    /// tools.
    /// * `Conflict` - If the server responds with a `409` StatusCode. This usually means the printer
    /// is either currently printing or not operational.
    pub async fn select_tool(&self, tool: String) -> Result<(), ToolCommandError> {
        let url = format!("http://{}:{}/api/printer/tool", &self.address, &self.port);

        let request = ToolCommand::Select {
            command: "select".to_string(),
            tool,
        };

        let res = self
            .client
            .post(&url)
            .header("X-Api-Key", &self.api_key)
            .json(&request)
            .send()
            .await
            .map_err(ToolCommandError::ReqwestError)?;

        let status = res.status();

        if status.is_success() {
            return Ok(());
        }
        if status.is_server_error() {
            return Err(ToolCommandError::ServerError);
        }

        let text = res
            .text()
            .await
            .map_err(ToolCommandError::ReqwestError)?;

        match status {
            StatusCode::BAD_REQUEST => Err(ToolCommandError::BadRequest(text)),
            StatusCode::CONFLICT => Err(ToolCommandError::Conflict(text)),
            _ => unreachable!(),
        }
    }

    /// Extrudes using the selected tool.
    /// Use [`select_tool()`](#method.select_tool) to select a tool.
    ///
    /// # Arguments 
    ///
    /// `amount` - A `f32` representing the amount to extrude. If this is a negative number, it will
    /// instead retract the filament.
    ///
    /// # Errors 
    ///
    /// If there is an error, it will return a [`ToolCommandError`](errors::ToolCommandError) enum.
    /// * `ReqwestError` - If the request fails.
    /// * `BadRequest` - If the server responds with a `400` StatusCode. This usually means you
    /// specified an invalid tool. Run [`get_tool_state()`](#method.get_tool_state) to get the valid
    /// tools.
    /// * `Conflict` - If the server responds with a `409` StatusCode. This usually means the printer
    /// is either currently printing or not operational.
    pub async fn extrude(&self, amount: f32) -> Result<(), ToolCommandError> {
        let url = format!("http://{}:{}/api/printer/tool", &self.address, &self.port);

        let request = ToolCommand::Extrude {
            command: "select".to_string(),
            amount,
        };

        let res = self
            .client
            .post(&url)
            .header("X-Api-Key", &self.api_key)
            .json(&request)
            .send()
            .await
            .map_err(ToolCommandError::ReqwestError)?;

        let status = res.status();

        if status.is_success() {
            return Ok(());
        }
        if status.is_server_error() {
            return Err(ToolCommandError::ServerError);
        }

        let text = res
            .text()
            .await
            .map_err(ToolCommandError::ReqwestError)?;

        match status {
            StatusCode::BAD_REQUEST => Err(ToolCommandError::BadRequest(text)),
            StatusCode::CONFLICT => Err(ToolCommandError::Conflict(text)),
            _ => unreachable!(),
        }
    }

    /// retracts filament from the selected tool.
    /// use [`select_tool()`](#method.select_tool) to select a tool.
    ///
    /// Internally this calls the [`extrude()`](#method.extrude) method with `-amount` as the argument
    pub async fn retract(&self, amount: f32) -> Result<(), ToolCommandError> {
        self.extrude(-amount).await
    }

    /// Changes the flowrate of the selected tool.
    /// use [`select_tool()`](#method.select_tool) to select a tool.
    ///
    /// # Arguments
    ///
    /// `factor` - A `f32` representing the factor to change the flowrate by. The factor will always 
    /// be relative to `1.0` so the flowrate will be set to the factor. The supported range is `0.75`..`1.25`.
    ///
    /// # Errors 
    ///
    /// If there is an error, it will return a [`ToolCommandError`](errors::ToolCommandError) enum.
    /// * `ReqwestError` - If the request fails.
    /// * `BadRequest` - If the server responds with a `400` StatusCode. This usually means you
    /// havent specified a valid tool or requested a factor outside of the supported range. Run 
    /// [`get_tool_state()`](#method.get_tool_state) to get the valid tools.
    /// * `Conflict` - If the server responds with a `409` StatusCode. This usually means the printer
    /// is either currently printing or not operational.
    pub async fn change_tool_flowrate(&self, factor: f32) -> Result<(), ToolCommandError> {
        let url = format!("http://{}:{}/api/printer/tool", &self.address, &self.port);

        let request = ToolCommand::Flowrate {
            command: "select".to_string(),
            factor,
        };

        let res = self
            .client
            .post(&url)
            .header("X-Api-Key", &self.api_key)
            .json(&request)
            .send()
            .await
            .map_err(ToolCommandError::ReqwestError)?;

        let status = res.status();

        if status.is_success() {
            return Ok(());
        }
        if status.is_server_error() {
            return Err(ToolCommandError::ServerError);
        }

        let text = res
            .text()
            .await
            .map_err(ToolCommandError::ReqwestError)?;

        match status {
            StatusCode::BAD_REQUEST => Err(ToolCommandError::BadRequest(text)),
            StatusCode::CONFLICT => Err(ToolCommandError::Conflict(text)),
            _ => unreachable!(),
        }
    }

    //
    //  NOTE: BED COMMANDS
    //

    pub async fn change_bed_temp(
        &self,
        command: BedTempDescriptor,
    ) -> Result<(), ToolCommandError> {
        let url = format!("http://{}:{}/api/printer/bed", &self.address, &self.port);

        let res = self
            .client
            .post(&url)
            .header("X-Api-Key", &self.api_key)
            .json(&command.to_json())
            .send()
            .await
            .map_err(ToolCommandError::ReqwestError)?;

        let status = res.status();

        if status.is_success() {
            return Ok(());
        }
        if status.is_server_error() {
            return Err(ToolCommandError::ServerError);
        }

        let text = res.text().await.map_err(ToolCommandError::ReqwestError)?;

        match status {
            StatusCode::BAD_REQUEST => Err(ToolCommandError::BadRequest(text)),
            StatusCode::CONFLICT => Err(ToolCommandError::Conflict(text)),
            _ => unreachable!(),
        }
    }

    pub async fn get_bed_state(&self, history: Option<u32>) -> Result<BedState, DeviceStateError> {
        let query: String;
        if let Some(x) = history {
            query = format!("true&limit={}", x);
        } else {
            query = "false".to_string();
        }

        let url = format!(
            "http://{}:{}/api/printer?history={}",
            &self.address, &self.port, query
        );

        let res = self
            .client
            .get(&url)
            .header("X-Api-Key", &self.api_key)
            .send()
            .await
            .map_err(DeviceStateError::ReqwestError)?;

        let status = res.status();
        let text = res.text().await.map_err(DeviceStateError::ReqwestError)?;

        match status {
            StatusCode::CONFLICT => Err(DeviceStateError::Conflict(text)),
            status if status.is_success() => {
                let result = &mut serde_json::Deserializer::from_str(text.as_str());
                let deserialized = serde_path_to_error::deserialize(result)
                    .map_err(|e| DeviceStateError::ParseError(e.to_string()))?;
                Ok(deserialized)
            }
            _ => unreachable!(),
        }
    }
}
