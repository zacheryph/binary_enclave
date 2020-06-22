use serde::{Deserialize, Serialize};

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
