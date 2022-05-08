use std::process::Command;

use dbus::blocking::Connection;
use networkmanager::devices::{Any, Device, Wireless};
use networkmanager::NetworkManager;

fn has_connection(nm: &NetworkManager) -> bool {
    for dev in nm.get_devices().unwrap() {
        match dev {
            Device::Ethernet(x) => {
                if x.active_connection().is_ok() {
                    return true;
                }
            }
            Device::WiFi(x) => {
                if x.active_connection().is_ok() || x.active_access_point().is_ok() {
                    return true;
                }
                
            }
            _ => {}
        }
    }

    false
}


fn main() {
    let dbus_connection = Connection::new_system().unwrap();

    let nm = NetworkManager::new(&dbus_connection);

    loop {
        if !has_connection(&nm) {
            
            match Command::new("/usr/bin/systemctl").arg("start").arg("device-setup.service").output() {
                Ok(output) => {
                    if !output.status.success() {
                        eprintln!("'systemctl' returned a failure code {}: {}", output.status, std::str::from_utf8(&output.stdout).unwrap_or("ERROR: NOT UTF-8"));
                    }
                }
                Err(e) => {
                    eprintln!("Failed to execute 'systemctl': {}", e);
                }

            }
        }

        std::thread::sleep(std::time::Duration::from_secs(180));
    }
}