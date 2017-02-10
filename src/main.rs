extern crate curl;
extern crate dns_lookup;
extern crate futures;
extern crate tokio_core;
extern crate toml;

use std::env;
use std::sync::mpsc;
use std::thread::spawn;

use config::Config;

mod probe;
mod proxy;
mod state;
mod dns;
mod config;


fn main() {

    let config = match env::args().nth(1) {
        Some(config_file) => Config::from_toml(config_file),
        None => Config::default(),
    };

    println!("{:?}", config);

    start(config);
}

fn start(config: Config) {
    let (tx, rx) = mpsc::channel();

    let server_addr = dns::lookup(config.remote_host.as_str(), config.remote_port);
    let client_addr = dns::lookup("0.0.0.0", config.local_port);
    let probe_url = format!("{}{}{}",
                            config.remote_host,
                            config.remote_port,
                            config.probe_path);
    let interval = config.probe_interval as u64;

    let h1 = spawn(move || proxy::start(rx, server_addr, client_addr));
    let h2 = spawn(move || probe::start(tx, probe_url.as_str(), interval));

    h1.join().unwrap();
    h2.join().unwrap();
}
