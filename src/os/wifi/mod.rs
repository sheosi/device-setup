pub mod networkmanager;

#[cfg(feature="networkmanager")]
pub use networkmanager::*;

use actix_web::{body, HttpResponse};
use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to connect or perform some operation on the system bus: {0}")]
    SystemBus(#[from] dbus::Error),

    #[error("Connection to the system bus has been aborted")]
    BusConnectionAborted,
}

impl actix_web::ResponseError for Error {
    fn error_response(&self) -> HttpResponse<body::BoxBody> {
        actix_web::HttpResponse::ServiceUnavailable().body(self.to_string())
    }
}

#[async_trait]
pub trait Handler {
    async fn connect_to(&mut self, ssid: &str, password: &str) -> Result<(), Error>;
}

