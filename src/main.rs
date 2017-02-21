#![feature(conservative_impl_trait)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate redis;
extern crate rocket;

use rocket::http::ContentType;
use rocket::http::ascii::UncasedAscii;
use rocket::response::content::*;
use std::borrow::Cow;

mod assets;

#[get("/")]
fn index() -> impl Response {
    HTML(assets::TROLL_HTML)
}

#[get("/troll.js")]
fn troll_js() -> impl Response {
    JavaScript(assets::TROLL_JS)
}

#[get("/troll.css")]
fn troll_css() -> impl Response {
    CSS(assets::TROLL_CSS)
}

#[get("/troll.gif")]
fn troll_gif() -> impl Response {
    Content(GIF, Bytes(assets::TROLL_GIF))
}

#[get("/troll.mp3")]
fn troll_mp3() -> impl Response {
    Content(MP3, Bytes(assets::TROLL_MP3))
}

#[get("/troll.ogg")]
fn troll_ogg() -> impl Response {
    Content(OGG, Bytes(assets::TROLL_OGG))
}

#[get("/play.png")]
fn play_png() -> impl Response {
    Content(PNG, Bytes(assets::PLAY_PNG))
}

#[get("/favicon.ico")]
fn favicon() -> impl Response {
    Content(ICO, Bytes(assets::TROLL_ICO))
}

#[error(404)]
fn not_found() -> impl Response {
    HTML(assets::HTTP_404)
}

fn main() {
    rocket::ignite()
        .mount("/",
               routes![index, troll_js, troll_css, troll_gif, troll_mp3, troll_ogg, play_png,
                       favicon])
        .catch(errors![not_found])
        .launch();
}

const GIF: ContentType = ContentType {
    ttype: UncasedAscii { string: Cow::Borrowed("image") },
    subtype: UncasedAscii { string: Cow::Borrowed("gif") },
    params: None,
};
const PNG: ContentType = ContentType {
    ttype: UncasedAscii { string: Cow::Borrowed("image") },
    subtype: UncasedAscii { string: Cow::Borrowed("png") },
    params: None,
};
const ICO: ContentType = ContentType {
    ttype: UncasedAscii { string: Cow::Borrowed("image") },
    subtype: UncasedAscii { string: Cow::Borrowed("ico") },
    params: None,
};
const MP3: ContentType = ContentType {
    ttype: UncasedAscii { string: Cow::Borrowed("audio") },
    subtype: UncasedAscii { string: Cow::Borrowed("mp3") },
    params: None,
};
const OGG: ContentType = ContentType {
    ttype: UncasedAscii { string: Cow::Borrowed("audio") },
    subtype: UncasedAscii { string: Cow::Borrowed("ogg") },
    params: None,
};

trait Response: rocket::response::Responder<'static> {}
impl<T> Response for T where T: rocket::response::Responder<'static> {}

struct Bytes(&'static [u8]);
impl rocket::response::Responder<'static> for Bytes {
    fn respond(self) -> Result<rocket::response::Response<'static>, rocket::http::Status> {
        use std::io::Cursor;
        rocket::response::Response::build()
            .header(ContentType::Plain)
            .sized_body(Cursor::new(self.0))
            .ok()
    }
}

/*extern crate iron;
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
            "play.png"   => serve_static("image/png", assets::PLAY_PNG, Some(60*60*24)),

            // Get the number of trolls served
            "count.txt"  => serve_count(&troll_count),

            // 404
            _ => serve_static("text/html", assets::HTTP_404, Some(60*60*24)),
        }
    }).http("0.0.0.0:1337").expect("Failed to create HTTP server");
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
}*/
