pub mod crypto;

use crypto::CryptoService;
use dotenv::dotenv;
use serde::Deserialize;
use eyre::WrapErr;
use color_eyre::Result;
use tracing::{info, instrument};
use tracing_subscriber::EnvFilter;
use std::time::Duration;
use std::sync::Arc;
use r2d2::Pool;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use std::env;
use std;
use failure::Error;
use r2d2::ManageConnection;



#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: i32,
    pub database_url: String,
    pub secret_key: String
}

impl Config {
    #[instrument]
    pub fn from_env() -> Result<Config> {
        dotenv().ok();
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();
        info!("Loading configuration");
        
        let mut c = config::Config::new();
        c.merge(config::Environment::default())?;
        c.try_into()
            .context("config from env")
    }

}