use std::{error::Error, time::Duration};
use reqwest::Client;

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
    pub fn new<P: ToString, P2: ToString>(address: P2, api_key: P) -> Self {
        Self {
            address: address.to_string(),
            port: 80,
            api_key: api_key.to_string(),
            client: Client::new(),
        }
    }

    /// Set the port of the printer
    /// If this is not set, it will default to 80
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Build the printer struct
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
        connection: types::PrinterConnectionCommand,
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

    /// Will get the printer files and folders in the specified directory
    pub async fn get_file(
        &self,
        location: &str,
    ) -> Result<types::printer_files::Files, Box<dyn Error>> {
        let url = format!(
            "http://{}:{}/api/files/{}",
            self.address, self.port, location
        );
        let res = self
            .client
            .get(&url)
            .header("X-Api-Key", &self.api_key)
            .send()
            .await?;
        let body = res.json::<types::printer_files::Files>().await?;

        Ok(body)
    }

    /// Will get all printer files and folders recursively from the specified directory
    pub async fn get_file_recursive(
        &self,
        location: &str,
    ) -> Result<types::printer_files::Files, Box<dyn Error>> {
        let url = format!(
            "http://{}:{}/api/files/{}?recursive=true",
            self.address, self.port, location
        );
        let res = self
            .client
            .get(&url)
            .header("X-Api-Key", &self.api_key)
            .send()
            .await?;
        let body = res.json::<types::printer_files::Files>().await?;

        Ok(body)
    }

    /// Will get the printer files and folders in the root directory
    pub async fn get_files(&self) -> Result<types::printer_files::Files, Box<dyn Error>> {
        let url = format!("http://{}:{}/api/files", self.address, self.port);
        let res = self
            .client
            .get(&url)
            .header("X-Api-Key", &self.api_key)
            .send()
            .await?;
        let body = res.json::<types::printer_files::Files>().await?;

        Ok(body)
    }

    /// Will get all printer files and folders recursively
    pub async fn get_files_recursive(
        &self,
    ) -> Result<types::printer_files::Files, Box<dyn Error>> {
        let url = format!("http://{}:{}/api/files?recursive=true", self.address, self.port);
        let res = self
            .client
            .get(&url)
            .header("X-Api-Key", &self.api_key)
            .send()
            .await?;
        let body = res.json::<types::printer_files::Files>().await?;

        Ok(body)
    }
}
