use reqwest::{Client, StatusCode};
use std::{error::Error, time::Duration};

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
    /// Returns a struct representing the current api version.
    pub async fn get_api_version(&self) -> Result<types::ApiVersion, Box<dyn Error>> {
        let url = format!("http://{}:{}/api/version", self.address, self.port);

        dbg!(&url);

        let res = self
            .client
            .get(&url)
            .header("X-Api-Key", &self.api_key)
            .timeout(Duration::from_secs(2))
            .send()
            .await?;

        dbg!(&res);

        let body = res.json::<types::ApiVersion>().await?;

        Ok(body)
    }

    /// Returns a struct containing the current connection settings of the printer
    /// as well as the available printer settings
    pub async fn get_connection(&self) -> Result<types::PrinterConnection, Box<dyn Error>> {
        let url = format!("http://{}:{}/api/connection", self.address, self.port);
        let res = self
            .client
            .get(&url)
            .header("X-Api-Key", &self.api_key)
            .send()
            .await?;
        let body = res.json::<types::PrinterConnection>().await?;

        Ok(body)
    }

    /// Set the connection settings of the printer
    pub async fn set_connection(
        &self,
        connection: types::ConnectionCommandDescriptor,
    ) -> Result<(), Box<dyn Error>> {
        let url = format!("http://{}:{}/api/connection", self.address, self.port);
        let res = self
            .client
            .post(&url)
            .header("X-Api-Key", &self.api_key)
            .json(&connection.to_post())
            .send()
            .await?;

        if res.status().is_client_error() {
            return Err("Incorrect request body".into());
        }

        Ok(())
    }

    /// Will get the printer files and folders from the specified folder
    ///
    /// # Arguments
    ///
    /// * `files_descriptor` - A struct containing the location of the files and if the request should be recursive
    ///     * `location` - The location of the files, either `Root`, `Local` or `Sdcard`
    ///     * `force` - If the request should force the printer to refresh the cache
    ///     * `recursive` - If the request should be recursive
    pub async fn get_files(
        &self,
        files_descriptor: types::FilesFetchDescriptor,
    ) -> Result<types::printer_files::Files, Box<dyn Error>> {
        let path = match files_descriptor.location {
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
            self.address, self.port, path, query_params
        );

        dbg!(&url);

        let res = self
            .client
            .get(&url)
            .header("X-Api-Key", &self.api_key)
            .send()
            .await?;

        let text = res.text().await?;
        let result = &mut serde_json::Deserializer::from_str(text.as_str());
        let deserialized = serde_path_to_error::deserialize(result);

        // TODO Error Handling
        match deserialized {
            Err(err) => {
                let path = err.path().to_string();
                panic!("Error at: {path}\n{err}");
            }
            Ok(x) => Ok(x),
        }
    }

    //
    // TODO Make file uploads work
    //

    /// Gets a single file or folder from the printer
    ///
    /// # Arguments
    ///
    /// * `file_descriptor` - A struct containing the location of the file and if the request should be recursive
    ///    * `location` - The location of the file, either `Local` or `Sdcard`
    ///    * `path` - The location of the file
    ///    * `force` - If the request should force the printer to refresh the cache
    ///    * `recursive` - If the request should be recursive
    pub async fn get_file(
        &self,
        file_descriptor: types::FileFetchDescriptor,
    ) -> Result<types::printer_files::Entry, Box<dyn Error>> {
        let location = match file_descriptor.location {
            types::FileLocation::Local => "local/",
            types::FileLocation::Sdcard => "sdcard/",
        };

        let path = file_descriptor
            .path
            .strip_prefix('/')
            .unwrap_or(&file_descriptor.path);

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
            .await?;

        match res.status() {
            StatusCode::NOT_FOUND => {
                panic!("File couldnt be found");
            }
            _ => {}
        }

        let text = res.text().await?;
        let result = &mut serde_json::Deserializer::from_str(text.as_str());
        let deseialized = serde_path_to_error::deserialize(result);

        // TODO Error Handling
        match deseialized {
            Err(err) => {
                let path = err.path().to_string();
                panic!("Error at: {path}\n{err}");
            }
            Ok(x) => Ok(x),
        }
    }

    
}
