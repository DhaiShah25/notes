use figment::{
    providers::{Format, Toml},
    Figment,
};
use serde::Deserialize;
mod backend;
use backend::handle;
use std::thread::spawn;

#[derive(Deserialize, Debug)]
struct Job {
    name: String,
    port: u16,
    dir: std::path::PathBuf,
}

#[derive(Deserialize, Debug)]
struct Config {
    job: Vec<Job>,
}

fn main() {
    let config: Config = Figment::new()
        .merge(Toml::file("Settings.toml"))
        .extract()
        .unwrap();

    for job in config.job {
        dbg!(&job);
        spawn(move || {
            let server = tiny_http::Server::http(format!("127.0.0.1:{}", job.port)).unwrap();

            for request in server.incoming_requests() {
                handle(request);
            }
        });
    }
    let mut buffer = String::new();
    let stdin = std::io::stdin(); // We get `Stdin` here.
    let _ = stdin.read_line(&mut buffer);
}
