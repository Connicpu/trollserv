extern crate iron;
extern crate byteorder;

use iron::prelude::*;
use iron::status;
use iron::headers::{self, CacheDirective};
use iron::modifiers::Header;

use troll_count::TrollCount;

mod assets;
mod troll_count;

fn main() {
    let troll_count = TrollCount::create();

    Iron::new(move |request: &mut Request| {
        let mut path = request.url.path().into_iter();

        match path.next().unwrap_or("404.html") {
            // Favicon
            "favicon.ico" => serve_static("image/ico", assets::TROLL_ICO, Some(60*60*24)),

            // The main page
            ""           => serve_home(&troll_count),
            "index.html" => serve_home(&troll_count),
            "troll.html" => serve_home(&troll_count),

            // Assets
            "troll.js"   => serve_static("text/javascript", assets::TROLL_JS, Some(180)),
            "troll.css"  => serve_static("text/css", assets::TROLL_CSS, Some(60*60*24)),
            "troll.gif"  => serve_static("image/gif", assets::TROLL_GIF, Some(60*60*24)),
            "troll.mp3"  => serve_static("audio/mp3", assets::TROLL_MP3, Some(60*60*24)),
            "troll.ogg"  => serve_static("audio/ogg", assets::TROLL_OGG, Some(60*60*24)),

            // Get the number of trolls served
            "count.txt"  => serve_count(&troll_count),

            // 404
            _ => serve_static("text/html", assets::HTTP_404, Some(60*60*24)),
        }
    }).http("localhost:1337").expect("Failed to create HTTP server");
}

fn serve_home(count: &TrollCount) -> IronResult<Response> {
    count.tick();
    // no cache, we need the hit counter
    serve_static("text/html", assets::TROLL_HTML, Some(15))
}

fn serve_count(count: &TrollCount) -> IronResult<Response> {
    serve_static("text/plain", format!("{}", count.value()).as_bytes(), None)
}

fn serve_static(mime: &str, asset: &[u8], cache: Option<u32>) -> IronResult<Response> {
    use iron::mime::Mime;
    use std::str::FromStr;
    let mime = Mime::from_str(mime).expect("invalid mime type");
    let cache = Header(headers::CacheControl(if let Some(time) = cache {
        vec![CacheDirective::MaxAge(time)] // 1 hour
    } else {
        vec![CacheDirective::NoCache, CacheDirective::NoStore]
    }));
    Ok(Response::with((mime, cache, status::Ok, asset)))
}

