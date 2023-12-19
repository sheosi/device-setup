use std::time::Duration;

use dbus::Message;
use dbus::arg::{RefArg, Variant};
use dbus::blocking::{stdintf::org_freedesktop_dbus::PropertiesPropertiesChanged, Connection};

const NM_DEST: &str = "org.freedesktop.NetworkManager";
const TIMEOUT: Duration = Duration::from_secs(5);


#[cfg(feature="networkmanager")]
pub fn watch_network() {
    let dbus_connection = Connection::new_system().unwrap();
    let nm_proxy = dbus_connection.with_proxy(NM_DEST, "/org/freedesktop/NetworkManager", TIMEOUT);

     check_setup_status(
        &dbus_connection, 
        nm_api::Nm::new(&nm_proxy).get_active_connections()
    );

    let _id = nm_proxy.match_signal(|c: PropertiesPropertiesChanged, _: &Connection, _: &Message|{
        if let Some(cons) = c.changed_properties.get("ActiveConnections") {
            let dbus_connection = Connection::new_system().unwrap();
            check_setup_status(&dbus_connection, cons);
        }

        // We want to keep the match
        true
    }).unwrap();    

    loop {
        dbus_connection.process(Duration::from_secs(std::u16::MAX.into())).unwrap();
    }
}

mod nm_api {
    use dbus::arg::{PropMap, Variant, RefArg};
    use dbus::blocking::{Connection, stdintf::org_freedesktop_dbus::Properties, Proxy};

    pub struct Nm (PropMap);
    impl Nm {
        pub fn new(nm_proxy: &Proxy<'_, &Connection>) -> Self {
            let all_data = 
                nm_proxy
                .get_all("org.freedesktop.NetworkManager")
                .unwrap();

            Self(all_data)
        }
        
        pub fn get_active_connections(&self) -> &Variant<Box<dyn RefArg>> {
            self.0.get("ActiveConnections").unwrap()
        }

        pub fn get_conn_data(dbus: &Connection, conn_path: &str) -> Result<NmConnection, dbus::Error> {
            dbus
            .with_proxy(super::NM_DEST, conn_path, super::TIMEOUT)
                .get_all("org.freedesktop.NetworkManager.Connection.Active")
                .map(NmConnection)
        }
    }

    pub struct NmConnection(PropMap);

    impl NmConnection {
        pub fn get_device(&self, dbus: &Connection) -> Option<NmDevice> {
            self.0
            .get("Devices") // Returns a Variant(Array(Path))
            .unwrap()
            .as_iter() // Iter the variant
            .unwrap()
            .next()
            .unwrap() // Iter the Array
            .as_iter()
            .unwrap()
            .next()
            .map(|d|{ // Extract the device itself
                let as_str = d.as_str().unwrap();

                let data = dbus
                .with_proxy(super::NM_DEST, as_str, super::TIMEOUT)
                .get_all("org.freedesktop.NetworkManager.Device")
                .unwrap();

                NmDevice(data)
            })
        }
    }

    pub struct NmDevice(PropMap);

    impl NmDevice {
        pub fn get_interface(&self) -> &str {
            self.0.get("Interface").unwrap().as_str().unwrap()
        }    
    }
}

fn has_usable_conn(dbus: &Connection,data: &Variant<Box<dyn RefArg>>) -> bool {
    
    data.as_iter().unwrap().next().unwrap().as_iter().unwrap().any(|con|{
        let conn_path = con.as_str().unwrap();
        nm_api::Nm::get_conn_data(dbus, conn_path).map_or(false,|conn_data| {
            conn_data.get_device(dbus)
                .map_or(false,|d| { // A connection might not have any device
                    d.get_interface() != "lo"
                })
            })
        })

}

fn check_setup_status(dbus: &Connection, connections: &Variant<Box<dyn RefArg>>) {
    
    if !has_usable_conn(dbus, connections) {
        crate::execution::start_device_setup()
    }
    else {
        crate::execution::stop_device_stop()
    }
}