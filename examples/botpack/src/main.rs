mod cmd;
mod config;

use bincrypt::{bincrypt, Bincrypt};
use config::Config;

#[bincrypt(botpack)]
pub static CONFIG: Bincrypt<Config, 512> = Bincrypt::new();

fn main() {
    let cmd = cmd::parse();

    if cmd.print {
        let config = CONFIG.decode();
        let config = serde_json::to_string_pretty(&config).unwrap();
        println!("{}", config);
    }

    if cmd.template {
        let template = serde_json::to_string_pretty(&Config::default()).unwrap();
        println!("{}", template);
    }

    match cmd.write {
        Some(path) => {
            let buffer = std::fs::read(path).unwrap();
            let config: Config = serde_json::from_slice(&buffer).unwrap();

            match CONFIG.write(&config) {
                Err(err) => println!("Write failed: {}", err),
                _ => println!("Wrote configuration successfully"),
            };
        },
        _ => {},
    };
}
