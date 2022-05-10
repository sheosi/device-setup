use std::process::Command;
use std::time::Duration;

use dbus::Message;
use dbus::blocking::{stdintf::org_freedesktop_dbus::PropertiesPropertiesChanged, Connection};

fn start_device_setup_systemd() {
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

fn watch_network_nm() {
    let dbus_connection = Connection::new_system().unwrap();
    let p = dbus_connection.with_proxy("org.freedesktop.NetworkManager", "/org/freedesktop/NetworkManager", Duration::from_secs(5));
    let _id = p.match_signal(|c: PropertiesPropertiesChanged, _: &Connection, _: &Message|{
        if let Some(cons) = c.changed_properties.get("ActiveConnections") {
            
            // Access the connections as iter and try to get the first one
            // this way we know whether there's one
            if cons.0.as_iter().unwrap().next().is_none() {
                start_device_setup_systemd();
            }
        }

        // We want to keep the match
        true
    }).unwrap();    

    loop {
        dbus_connection.process(Duration::from_secs(std::u16::MAX.into())).unwrap();
    }
}


fn main() {
    watch_network_nm();
}