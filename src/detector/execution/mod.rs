#[cfg(feature="systemd")]
mod systemd;

#[cfg(feature="systemd")]
use systemd::{start_device_setup_impl, stop_device_setup_impl};

use std::sync::{OnceLock, Mutex};

#[derive(Copy, Clone, Debug, PartialEq)]
enum ServiceState {Started, Stopped}

static LAST_STATE: OnceLock<Mutex<ServiceState>> = OnceLock::new();

pub fn start_device_setup() {
    // If LAST_STATE is unset we'll just assume it's stopped here
    if *LAST_STATE.get_or_init(||Mutex::new(ServiceState::Stopped)).lock().unwrap() == ServiceState::Stopped {
        println!("Start service!");
        start_device_setup_impl();
        *LAST_STATE.get().unwrap().lock().unwrap() = ServiceState::Started;
    }
}

pub fn stop_device_stop() {
    // If LAST_STATE is unset we'll just assume it's started here
    if *LAST_STATE.get_or_init(||Mutex::new(ServiceState::Started)).lock().unwrap() == ServiceState::Started {
        println!("Stop service!");
        stop_device_setup_impl();
        *LAST_STATE.get().unwrap().lock().unwrap() = ServiceState::Stopped;
    }
}

