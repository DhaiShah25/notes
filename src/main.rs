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
    dir: String,
}

#[derive(Deserialize, Debug)]
struct Config {
    job: Vec<Job>,
}

fn main() {
    let mut tmp = dirs::config_dir().unwrap();
    tmp.push("notes/Settings.toml");
    let config: Config = Figment::new().merge(Toml::file(tmp)).extract().unwrap();

    for job in config.job {
        log::info!("Job Started: {}", job.name);
        spawn(move || {
            let server = tiny_http::Server::http(format!("127.0.0.1:{}", job.port)).unwrap();

            for request in server.incoming_requests() {
                handle(request, &job.dir);
            }
        });
    }
    let mut buffer = String::new();
    let stdin = std::io::stdin(); // We get `Stdin` here.
    let _ = stdin.read_line(&mut buffer);
}
