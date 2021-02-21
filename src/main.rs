use dbus::arg::{prop_cast, PropMap};
use dbus::blocking::stdintf::org_freedesktop_dbus::ObjectManager;
use dbus::blocking::Connection;
use dbus::Path;
use std::collections::HashMap;
use std::time::Duration;

fn main() {
    let conn = Connection::new_system().unwrap();
    let proxy = conn.with_proxy("org.bluez", "/", Duration::from_millis(5000));

    let tree: HashMap<Path<'static>, HashMap<String, PropMap>> =
        proxy.get_managed_objects().unwrap();

    tree.into_iter()
        .filter_map(|(key, interfaces)| {
            let props: &PropMap = interfaces.get("org.bluez.Adapter1")?;

            Some(Adapter::new(key, props))
        })
        .for_each(|d| {
            println!("{:?}", d);
        });
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
    fn new(key: Path, map: &PropMap) -> Adapter {
        let map_str = |key| prop_cast::<String>(map, key).unwrap().to_owned();
        let map_u32 = |key| prop_cast::<u32>(map, key).copied().unwrap();
        let map_bool = |key| prop_cast::<bool>(map, key).copied().unwrap();

        let id = key
            .to_string()
            .strip_prefix("/org/bluez/")
            .unwrap()
            .to_owned();

        Adapter {
            id,
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
