use serde::{Deserialize, Serialize};
use binary_enclave::{enclave, Enclave};
use serde_json::to_string_pretty;

#[derive(Default, Serialize, Deserialize)]
struct Config {
    name: String,
    kind: String,
    amount: u32,
    features: Vec<String>,
    bitfield: Option<u32>,
}

#[enclave(simple)]
static CONFIG: Enclave<Config, 128> = Enclave::new();

// usage: ./target/debug/examples/simple-serde [tpl|examples/simple.json]
fn main() {
    let mut args = std::env::args();
    args.next().unwrap();

    match args.next().as_deref() {
        None => {
            let conf = CONFIG.decode().unwrap();
            println!("{}", to_string_pretty(&conf).unwrap())
        },
        Some("tpl") => println!("{}", to_string_pretty(&Config::default()).unwrap()),
        Some(f) => {
            let buf = std::fs::read(f).unwrap();
            let inbound: Config = serde_json::from_slice(&buf).unwrap();
            CONFIG.write(&inbound).unwrap();
        },
    };
}
