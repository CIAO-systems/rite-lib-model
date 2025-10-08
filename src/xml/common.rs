//! Module for common XML structures

use serde::{Deserialize, Serialize};

/// A structure to store TCP/IP connection information to a database
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "connection")]
pub struct DatabaseConnection {
    #[serde(rename = "@host")]
    pub host: String,
    #[serde(rename = "@port")]
    pub port: u16,
    #[serde(rename = "@database")]
    pub database: String,
    #[serde(rename = "@user")]
    pub user: String,
    #[serde(rename = "@password")]
    pub password: String,
}
