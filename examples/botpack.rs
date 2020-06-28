use serde::{Deserialize, Serialize};
use binary_enclave::{enclave, Enclave};
use serde_json::to_string_pretty;

// This is just a larger `simple` example.
// An IRC botpack was the source of this so
// I thought it prudent to include its example.

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub pack: Pack,
    pub local: Option<Local>,
}

impl Default for Config {
  fn default() -> Self {
      Self {
          pack: Pack::default(),
          local: Some(Local::default()),
      }
  }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Pack {
    packname: String,
    shell_hash: String,
    salts: Vec<String>,
    dcc_prefix: String,
    owners: Vec<String>,
    hubs: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Local {
    bots: Vec<String>,
    uid: usize,
    autocron: bool,
    username: String,
    datadir: String,
    homedir: String,
    port_min: usize,
    port_max: usize,
}

#[enclave(botpack)]
pub static CONFIG: Enclave<Config, 512> = Enclave::new();

// usage: ./target/debug/examples/botpack [tpl|examples/botpack.json]
fn main() {
    let mut args = std::env::args();
    args.next().unwrap();

    match args.next().as_deref() {
        None => {
            let conf = CONFIG.decode().unwrap();
            println!("{}", to_string_pretty(&conf).unwrap());
        },
        Some("tpl") => println!("{}", to_string_pretty(&Config::default()).unwrap()),
        Some(f) => {
            let buf = std::fs::read(f).unwrap();
            let inbound: Config = serde_json::from_slice(&buf).unwrap();
            CONFIG.write(&inbound).unwrap();
        },
    };
}
