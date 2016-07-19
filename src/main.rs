extern crate iron;
extern crate byteorder;

use iron::prelude::*;
use iron::status;

use troll_count::TrollCount;

mod assets;
mod troll_count;

fn main() {
    let troll_count = TrollCount::create();

    Iron::new(move |request: &mut Request| {
        let mut path = request.url.path().into_iter();

        match path.next().unwrap_or("404.html") {
            // The main page
            ""           => serve_home(&troll_count),
            "index.html" => serve_home(&troll_count),
            "troll.html" => serve_home(&troll_count),

            // Assets
            "troll.js"   => serve_static("text/javascript", assets::TROLL_JS),
            "troll.gif"  => serve_static("image/gif", assets::TROLL_GIF),
            "troll.mp3"  => serve_static("audio/mp3", assets::TROLL_MP3),
            "troll.ogg"  => serve_static("audio/ogg", assets::TROLL_OGG),

            // Get the number of trolls served
            "count.txt"  => serve_count(&troll_count),

            // 404
            _ => serve_static("text/html", assets::HTTP_404),
        }
    }).http("localhost:1337").expect("Failed to create HTTP server");
}

fn serve_home(count: &TrollCount) -> IronResult<Response> {
    count.tick();
    serve_static("text/html", assets::TROLL_HTML)
}

fn serve_count(count: &TrollCount) -> IronResult<Response> {
    serve_static("text/plain", format!("{}", count.value()).as_bytes())
}

fn serve_static(mime: &str, asset: &[u8]) -> IronResult<Response> {
    use iron::mime::Mime;
    use std::str::FromStr;
    Ok(Response::with((Mime::from_str(mime).expect("invalid mime type"), status::Ok, asset)))
}

