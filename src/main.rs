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
    let config: Config = match Figment::new().merge(Toml::file(&tmp)).extract() {
        Ok(conf) => conf,
        Err(..) => {
            let _ = std::fs::write(
                &tmp,
                r#"# Use the following syntax to create jobs for each notes directory
# [[job]]
# name = <NAME>
# port = <PORT_NUMBER>
# dir = <ABSOLUTE_PATH_TO_NOTES_DIRECTORY>

# To add styling you use styles.css file in the config directory
# To add javascript scripts you just create a main.js file in the config directory (only this file will be sourced)"#,
            );
            println!(
                "Please add information to the configuration at: {}",
                tmp.to_string_lossy()
            );
            return;
        }
    };

    for job in config.job {
        println!("Job {} started on http://127.0.0.1:{}", job.name, job.port);
        spawn(move || {
            let server = tiny_http::Server::http(format!("127.0.0.1:{}", job.port)).unwrap();

            for request in server.incoming_requests() {
                handle(request, &job.dir);
            }
        });
    }

    let mut buffer = String::with_capacity(0);
    let stdin = std::io::stdin();
    let _ = stdin.read_line(&mut buffer);
}
