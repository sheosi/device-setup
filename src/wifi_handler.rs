use thiserror::Error;

#[derive(Debug, Error)]
pub enum WifiError {

}

pub trait WifiHandler {
    fn connect_to(&mut self, ssid: &str, password: &str) -> Result<(), WifiError>;
}

pub struct NetworkManagerWifi {

}

impl NetworkManagerWifi {
    pub fn new() -> Self {
        Self{}
    }   
}

impl WifiHandler for NetworkManagerWifi {
    fn connect_to(&mut self, ssid: &str, password: &str) -> Result<(), WifiError> {
        Ok(())
    }
}

pub fn get_wifi_handler() -> Box<dyn WifiHandler> {
    Box::new(NetworkManagerWifi::new())
}