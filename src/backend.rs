use std::fs::File;
use tiny_http::{Request, Response};

pub fn handle(req: Request) {
    match req.url() {
        "/styles" => {
            let mut tmp = dirs::config_dir().unwrap();
            tmp.push("notes/styles.css");
            let resp = Response::from_file(File::open(tmp).unwrap());
            let _ = req.respond(resp);
            // TODO: Make forever cached
        }
        "/js" => {
            let mut tmp = dirs::config_dir().unwrap();
            tmp.push("notes/main.js");
            let resp = Response::from_file(File::open(tmp).unwrap());
            let _ = req.respond(resp);
            // TODO: Make forever cached
        }
        other => others(req),
    }
}

fn others(req: Request) {
    // TODO: check if requested is a dir and if so return a Walkdir looked through path
    //
    // TODO: Return the jotdown formatted text
}
