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

        match res {
            Err(e) => Err(InformationRequestError::ReqwestError(e)),
            Ok(x) => {
                dbg!(&x);

                let body = x.json::<types::ApiVersion>().await;
                match body {
                    Err(e) => Err(InformationRequestError::ReqwestError(e)),
                    Ok(x) => Ok(x),
                }
            }
        }
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

        match res {
            Err(e) => Err(InformationRequestError::ReqwestError(e)),
            Ok(x) => {
                dbg!(&x);

                let body = x.json::<types::PrinterConnection>().await;
                match body {
                    Err(e) => Err(InformationRequestError::ReqwestError(e)),
                    Ok(x) => Ok(x),
                }
            }
        }
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

        match res {
            Err(e) => Err(SetConnectionError::ReqwestError(e)),
            Ok(x) if x.status() != StatusCode::OK => {
                let text = x.text().await.unwrap();
                Err(SetConnectionError::BadRequest(text))
            }
            _ => Ok(()),
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

        match res {
            Err(e) => Err(FileRequestError::ReqwestError(e)),
            Ok(x) => {
                if x.status() != StatusCode::OK {
                    let text = x.text().await.unwrap();
                    return Err(FileRequestError::ParseError(text));
                }

                let text = x.text().await.unwrap();
                let result = &mut serde_json::Deserializer::from_str(text.as_str());
                let deserialized = serde_path_to_error::deserialize(result);

                match deserialized {
                    Err(err) => Err(FileRequestError::ParseError(err.to_string())),
                    Ok(x) => Ok(x),
                }
            }
        }
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

        dbg!(&url);

        let res = self
            .client
            .get(&url)
            .header("X-Api-Key", &self.api_key)
            .send()
            .await;

        match res {
            Err(e) => Err(FileRequestError::ReqwestError(e)),
            Ok(x) => {
                if x.status() == StatusCode::NOT_FOUND {
                    let text = x.text().await.unwrap();
                    return Err(FileRequestError::NotFound(text));
                }

                let text = x.text().await.unwrap();
                let result = &mut serde_json::Deserializer::from_str(text.as_str());
                let deserialized = serde_path_to_error::deserialize(result);

                match deserialized {
                    Err(err) => Err(FileRequestError::ParseError(err.to_string())),
                    Ok(x) => Ok(x),
                }
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

        match res {
            Err(e) => Err(FileCommandError::ReqwestError(e)),
            Ok(x) => match x.status() {
                StatusCode::CREATED | StatusCode::NO_CONTENT => Ok(()),
                StatusCode::BAD_REQUEST => {
                    let text = x.text().await.unwrap();
                    Err(FileCommandError::BadRequest(text))
                }
                StatusCode::INTERNAL_SERVER_ERROR => {
                    let text = x.text().await.unwrap();
                    Err(FileCommandError::BadRequest(text))
                }
                StatusCode::CONFLICT => {
                    let text = x.text().await.unwrap();
                    Err(FileCommandError::Conflict(text))
                }
                // Shouldnt be able to get here
                _ => {
                    let text = x.text().await.unwrap();
                    panic!("wut: {}", text);
                }
            },
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

        match res {
            Err(e) => Err(FileDeletionError::ReqwestError(e)),
            Ok(x) => match x.status() {
                StatusCode::NO_CONTENT => Ok(()),
                StatusCode::NOT_FOUND => {
                    let text = x.text().await.unwrap();
                    Err(FileDeletionError::NotFound(text))
                }
                StatusCode::CONFLICT => {
                    let text = x.text().await.unwrap();
                    Err(FileDeletionError::Conflict(text))
                }
                // Shouldnt be able to get here
                _ => {
                    let text = x.text().await.unwrap();
                    panic!("wut: {}", text);
                }
            },
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

        match res {
            Err(e) => Err(JobCommandError::ReqwestError(e)),
            Ok(x) => match x.status() {
                StatusCode::NO_CONTENT => Ok(()),
                StatusCode::CONFLICT => {
                    let text = x.text().await.unwrap();
                    Err(JobCommandError::Conflict(text))
                }
                // Shouldnt be able to get here
                _ => {
                    let text = x.text().await.unwrap();
                    panic!("wut: {}", text);
                }
            },
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

        match res {
            Err(e) => Err(InformationRequestError::ReqwestError(e)),
            Ok(x) => {
                let text = x.text().await.unwrap();
                let result = &mut serde_json::Deserializer::from_str(text.as_str());
                let deserialized = serde_path_to_error::deserialize(result);

                match deserialized {
                    Err(e) => Err(InformationRequestError::ParseError(e.to_string())),
                    Ok(x) => Ok(x),
                }
            }
        }
    }

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
    pub async fn get_printer_telemetry(&self) -> Result<types::RawPrinter, PrinterCommandError> {
        let url = format!("http://{}:{}/api/printer", self.address, self.port);

        let res = self
            .client
            .get(&url)
            .header("X-Api-Key", &self.api_key)
            .send()
            .await;

        match res {
            Err(e) => Err(PrinterCommandError::ReqwestError(e)),
            Ok(x) => {
                if x.status() == StatusCode::CONFLICT {
                    let text = x.text().await.unwrap();
                    return Err(PrinterCommandError::Conflict(text));
                }

                let text = x.text().await.unwrap();
                let result = &mut serde_json::Deserializer::from_str(text.as_str());
                let deserialized = serde_path_to_error::deserialize(result);

                match deserialized {
                    Err(e) => panic!("wut: {}", e.to_string()),
                    Ok(x) => Ok(x),
                }
            }
        }
    } 
}
