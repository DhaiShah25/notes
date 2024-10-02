use std::fs::File;
use std::str::FromStr;
use tiny_http::{Header, Request, Response};

pub fn handle(req: Request, dir: &str) {
    match req.url() {
        "/styles" => {
            let mut tmp = dirs::config_dir().unwrap();
            tmp.push("notes/styles.css");

            let css = Header::from_str("Content-Type: text/css").unwrap();
            let cache = Header::from_str("Cache-control: max-age=86400").unwrap();

            let mut resp = Response::from_file(File::open(tmp).unwrap());
            resp.add_header(css);
            resp.add_header(cache);

            let _ = req.respond(resp);
            // TODO: Make forever cached
        }
        "/js" => {
            let js = Header::from_str("Content-Type: text/javascript").unwrap();
            let cache = Header::from_str("Cache-control: max-age=86400").unwrap();

            let mut tmp = dirs::config_dir().unwrap();
            tmp.push("notes/main.js");
            let mut resp = Response::from_file(File::open(tmp).unwrap());
            resp.add_header(js);
            resp.add_header(cache);
            let _ = req.respond(resp);
        }
        other => {
            let resp = process(other, dir);
            let _ = req.respond(resp);
        }
    }
}

fn process(info: &str, dir: &str) -> Response<std::io::Cursor<Vec<u8>>> {
    let html = Header::from_str("Content-Type: text/html").unwrap();

    let path = dir.to_owned() + info;
    let Ok(file_info) = std::fs::metadata(&path) else {
        let cache = Header::from_str("Cache-control: no-cache").unwrap();

        let mut resp = Response::from_string("<h1>File or Directory Doesn't Exist</h1>");
        resp.add_header(html);
        resp.add_header(cache);
        return resp;
    };

    if file_info.is_file() {
        let extension = std::path::Path::new(&path)
            .extension()
            .map(|e| e.to_str().unwrap())
            .unwrap_or("");
        match extension {
            "webp" => {
                use std::io::Read;
                let mut file = std::fs::File::open(path).unwrap();
                let mut buf = Vec::with_capacity(1024);

                let _ = file.read_to_end(&mut buf);

                let mut resp = Response::from_data(buf);

                let webp = Header::from_str("Content-Type: image/webp").unwrap();
                let cache = Header::from_str("Cache-control: max-age=86400").unwrap();

                resp.add_header(webp);
                resp.add_header(cache);

                return resp;
            }
            _ => {
                let contents = std::fs::read_to_string(&path).unwrap();
                let events = jotdown::Parser::new(&contents);
                let html_str = jotdown::html::render_to_string(events);
                let mut resp = Response::from_string(format!(
                    r#"<!DOCTYPE html>
                    <html>
                    <head>
                      <link rel='stylesheet' href='/styles'>
                      <script src="https://cdnjs.cloudflare.com/ajax/libs/mathjax/2.7.4/latest.js?config=AM_CHTML"></script>
                      <meta charset='UTF-8'>
                    </head>
                      <body>
                        {html_str}
                        <script src="/js"></script>
                      </body>
                    </html>"#,
                ));
                resp.add_header(html);
                return resp;
            }
        };
    } else if file_info.is_dir() {
        let mut contents = String::from(
            "<!DOCTYPE html><html><head><meta charset='UTF-8'><link rel='stylesheet' href='/styles'></head><body class='explorer'>",
        );

        let mut walk = walkdir::WalkDir::new(path)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok());
        walk.next();
        walk.for_each(|entry| {
            let path = entry.path();

            if path.is_dir() {
                contents.push_str(&format!(
                    "<a href=\"./{0}\">{0}</a>",
                    entry.file_name().to_str().unwrap()
                ));
            } else {
                let full = path.to_str().unwrap();
                let title = get_title(full);
                contents.push_str(&format!(
                    "<a href=\"{}\">{title}</a>",
                    full.strip_prefix(dir).unwrap()
                ));
            }
        });

        contents.push_str("</body></html>");

        let mut resp = Response::from_string(contents);
        resp.add_header(html);
        return resp;
    }

    let mut resp = Response::from_string("<h1>Impossible Error</h1>");
    resp.add_header(html);
    return resp;
}

fn get_title(file: &str) -> String {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = match File::open(&file) {
        Ok(file) => file,
        Err(_) => panic!("Unable to read title from {:?}", &file),
    };

    let mut buffer = BufReader::new(file);
    let mut first_line = String::new();
    let _ = buffer.read_line(&mut first_line);

    first_line
}
