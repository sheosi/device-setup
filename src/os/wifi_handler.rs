use std::collections::HashMap;
use std::time::Duration;

use thiserror::Error;
use dbus::{blocking::Connection, arg::{Variant, self, PropMap, RefArg}};

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
    
    pub fn prepare_connection_object<'a>(ssid: &'a str, password: &'a str) -> HashMap<&'static str, arg::PropMap> {
        fn variant_str(str: &str) -> Variant<Box<dyn RefArg>> {
            Variant(Box::new(Variant(str.to_string())))
        }

        fn variant_str_bytes(str: &str) -> Variant<Box<dyn RefArg>> {
            Variant(Box::new(Variant(str.to_string().into_bytes())))
        }


        let mut con = PropMap::new();        
        con.insert("type".into(), variant_str("802-11-wireless"));
        con.insert("uuid".into(), variant_str(""));
        con.insert("id".into(), variant_str("My-WPA_PSK"));

        let mut wifi = PropMap::new();
        wifi.insert("ssid".into(), variant_str_bytes(ssid));
        wifi.insert("mode".into(), variant_str("infrastructure"));

        let mut sec = PropMap::new();
        sec.insert("key-mgmt".into(), variant_str("wpa-psk"));
        sec.insert("auth-alg".into(), variant_str("open"));
        sec.insert("psk".into(), variant_str(password));

        let mut ipv4 = PropMap::new();
        ipv4.insert("method".into(), variant_str("auto"));

        let mut ipv6 = PropMap::new();
        ipv6.insert("method".into(), variant_str("ignore"));

        let mut res = HashMap::new();
        res.insert("connection", con);
        res.insert("802-11-wireless", wifi);
        res.insert("802-11-wireless-security", sec);
        res.insert("ipv4", ipv4);
        res.insert("ipv6", ipv6);

        res
    }
    
        /*let dbus_connection = Connection::new_system()?;
        let nm = NetworkManager::new(&dbus_connection);*/
    
}

impl WifiHandler for NetworkManagerWifi {
    fn connect_to(&mut self, ssid: &str, password: &str) -> Result<(), WifiError> {
        let dbus = Connection::new_system().unwrap();
        let proxy = dbus.with_proxy("org.freedesktop.NetworkManager", "/org/freedesktop/NetworkManager/Settings", Duration::from_millis(5000));
        
        let connection = Self::prepare_connection_object(ssid, password);
        let path_ = proxy.method_call("org.freedesktop.NetworkManager.Settings", "AddConnection", (connection, ))
        .map(|r: (dbus::Path<'static>, )| r.0).unwrap();

        Ok(())
    }
}

pub fn get_wifi_handler() -> Box<dyn WifiHandler> {
    Box::new(NetworkManagerWifi::new())
}