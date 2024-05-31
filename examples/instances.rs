/// Example of listing instances.
/// Requires a local LXD running.
use nlxd::client::{Client, ClientConfig};

fn main() {
    let client = Client::new(ClientConfig::default_autodetect_local().unwrap()).unwrap();
    let instance_names: Vec<String> = client.instances().unwrap();
    println!("{:#?}", &instance_names);
    for name in instance_names {
        println!("{:#?}", client.get_instance(&name).unwrap());
    }
}
