use std::error::Error;

use reqwest::Client;
use serde::{Deserialize, Serialize};

mod types;

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
    pub fn new<P: ToString, P2: ToString>(api_key: P, address: P2) -> Self {
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
    pub async fn get_api_version(&self) -> Result<types::ApiVersion, Box<dyn Error>> {
        let url = format!("http://{}:{}/api/version", self.address, self.port);
        let res = self.client.get(&url).send().await?;
        let body = res.json::<types::ApiVersion>().await?;

        Ok(body)
    }

    pub async fn get_connection(&self) -> Result<types::PrinterConnection, Box<dyn Error>> {
        let url = format!("http://{}:{}/api/connection", self.address, self.port);
        let res = self.client.get(&url).send().await?;
        let body = res.json::<types::PrinterConnection>().await?;

        Ok(body)
    }

    /// Set the connection settings of the printer
    pub async fn set_connection(&self, connection: types::PrinterConnectionPost) -> Result<(), Box<dyn Error>> {
        let url = format!("http://{}:{}/api/connection", self.address, self.port);
        let res = self.client.post(&url).json(&connection).send().await?;
        
        if res.status().is_client_error() {
            return Err("Incorrect request body".into());
        }

        Ok(())
    }
}
