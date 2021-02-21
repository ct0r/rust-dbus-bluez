use dbus::arg::{prop_cast, PropMap};
use dbus::blocking::stdintf::org_freedesktop_dbus::ObjectManager;
use dbus::blocking::Connection;
use dbus::{Error, Path};
use std::{thread, time::Duration};

fn main() {
    let conn = Connection::new_system().unwrap();

    let adapters = get_adapters(&conn).unwrap();
    print_adapters(&adapters);

    let devices = get_devices(&conn).unwrap();
    print_devices(&devices);

    start_discovery(&conn).unwrap();
    thread::sleep(Duration::from_millis(30000));
    stop_discovery(&conn).unwrap();

    let devices = get_devices(&conn).unwrap();
    print_devices(&devices);
}

fn get_adapters(conn: &Connection) -> Result<Vec<Adapter>, Error> {
    let proxy = conn.with_proxy("org.bluez", "/", Duration::from_millis(5000));
    let tree = proxy.get_managed_objects()?;

    let adapters = tree
        .into_iter()
        .filter_map(|(key, interfaces)| {
            let props: &PropMap = interfaces.get("org.bluez.Adapter1")?;
            Some(Adapter::new(key, props))
        })
        .collect();

    Ok(adapters)
}

fn get_devices(conn: &Connection) -> Result<Vec<Device>, Error> {
    let proxy = conn.with_proxy("org.bluez", "/", Duration::from_millis(5000));
    let tree = proxy.get_managed_objects()?;

    let devices = tree
        .into_iter()
        .filter_map(|(path, interfaces)| {
            let props: &PropMap = interfaces.get("org.bluez.Device1")?;
            Some(Device::new(path, props))
        })
        .collect();

    Ok(devices)
}

fn start_discovery(conn: &Connection) -> Result<(), Error> {
    let proxy = conn.with_proxy("org.bluez", "/org/bluez/hci0", Duration::from_millis(5000));
    proxy.method_call("org.bluez.Adapter1", "StartDiscovery", ())?;
    Ok(())
}

fn stop_discovery(conn: &Connection) -> Result<(), Error> {
    let proxy = conn.with_proxy("org.bluez", "/org/bluez/hci0", Duration::from_millis(5000));
    proxy.method_call("org.bluez.Adapter1", "StopDiscovery", ())?;
    Ok(())
}

#[derive(Debug)]
struct Adapter {
    id: String,
    address: String,
    address_type: String,
    name: String,
    alias: String,
    class: u32,
    powered: bool,
    discoverable: bool,
    discoverable_timeout: u32,
    pairable: bool,
    pairable_timeout: u32,
    discovering: bool,
    modalias: String,
    uuids: Vec<String>,
}

impl Adapter {
    fn new(path: Path, map: &PropMap) -> Adapter {
        let map_str = |key| prop_cast::<String>(map, key).unwrap().to_owned();
        let map_u32 = |key| prop_cast::<u32>(map, key).copied().unwrap();
        let map_bool = |key| prop_cast::<bool>(map, key).copied().unwrap();

        Adapter {
            id: path.to_string().to_owned(),
            address: map_str("Address"),
            address_type: map_str("AddressType"),
            name: map_str("Name"),
            alias: map_str("Alias"),
            class: map_u32("Class"),
            powered: map_bool("Powered"),
            discoverable: map_bool("Discoverable"),
            discoverable_timeout: map_u32("DiscoverableTimeout"),
            pairable: map_bool("Pairable"),
            pairable_timeout: map_u32("PairableTimeout"),
            discovering: map_bool("Discovering"),
            modalias: map_str("Modalias"),
            uuids: prop_cast::<Vec<String>>(map, "UUIDs").unwrap().to_owned(),
        }
    }
}

#[derive(Debug)]
struct Device {
    address: String,
    // address_type: String,
    // name: String,
    // alias: String,
    // class: u32,
    // appearance: u16,
    // icon: String,
    paired: bool,
    trusted: bool,
    blocked: bool,
    legacy_pairing: bool,
    // rssi: i16,
    connected: bool,
    uuids: Vec<String>,
    // modalias: String,
    // adapter:
    // manufacturer_data:
    // service_data:
    // tx_power: i16,
    services_resolved: bool,
}

impl Device {
    fn new(path: Path, map: &PropMap) -> Device {
        let map_str = |key| prop_cast::<String>(map, key).unwrap().to_owned();
        let map_u32 = |key| prop_cast::<u32>(map, key).copied().unwrap();
        let map_u16 = |key| prop_cast::<u16>(map, key).copied().unwrap();
        let map_i16 = |key| prop_cast::<i16>(map, key).copied().unwrap();
        let map_bool = |key| prop_cast::<bool>(map, key).copied().unwrap();

        Device {
            address: map_str("Address"),
            // address_type: map_str("AddressType"),
            // name: map_str("Name"),
            // alias: map_str("Alias"),
            // class: map_u32("Class"),
            // appearance: map_u16("Appearance"),
            // icon: map_str("icon"),
            paired: map_bool("Paired"),
            trusted: map_bool("Trusted"),
            blocked: map_bool("Blocked"),
            legacy_pairing: map_bool("LegacyPairing"),
            // rssi: map_i16("RSSI"),
            connected: map_bool("Connected"),
            uuids: prop_cast::<Vec<String>>(map, "UUIDs").unwrap().to_owned(),
            // modalias: map_str("Modalias"),
            // adapter:
            // manufacturer_data:
            // service_data:
            // tx_power: map_i16("TxPower"),
            services_resolved: map_bool("ServicesResolved"),
        }
    }
}

fn print_devices(devices: &Vec<Device>) {
    println!("Devices:");
    devices
        .into_iter()
        .for_each(|d| println!("{} - {}", d.address, if d.connected { "on" } else { "off" }));
    println!();
}

fn print_adapters(adapters: &Vec<Adapter>) {
    println!("Adapters:");
    adapters.into_iter().for_each(|a| println!("{}", a.name));
    println!();
}
