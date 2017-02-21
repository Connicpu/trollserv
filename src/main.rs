#![feature(conservative_impl_trait)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate redis;
extern crate rocket;

use redis::Commands;
use rocket::State;
use rocket::http::ContentType;
use rocket::http::ascii::UncasedAscii;
use rocket::response::content::*;
use std::borrow::Cow;
use std::env;
use std::sync::Mutex;

mod assets;

type Redis = Mutex<redis::Connection>;

#[get("/")]
fn index(redis: State<Redis>) -> impl Response {
    let () = redis.lock().unwrap().incr("count", 1).unwrap();
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

#[get("/count.txt")]
fn count(redis: State<Redis>) -> impl Response {
    let count: i64 = redis.lock().unwrap().get("count").unwrap();
    Plain(format!("{}", count))
}

#[error(404)]
fn not_found() -> impl Response {
    HTML(assets::HTTP_404)
}

fn config() -> rocket::config::Config {
    use rocket::config::*;

    let env = Environment::active().unwrap();
    let mut config = Config::build(env);
    if let Some(port) = env::var("PORT").ok().and_then(|s| s.parse().ok()) {
        config = config.port(port);
    }
    config.finalize().unwrap()
}

fn main() {
    rocket::custom(config(), false)
        .manage(make_redis())
        .mount("/",
               routes![index, troll_js, troll_css, troll_gif, troll_mp3, troll_ogg, play_png,
                       favicon, count])
        .catch(errors![not_found])
        .launch();
}

fn make_redis() -> Redis {
    let url = env::var("REDIS_URL").unwrap();
    let client = redis::Client::open(&url[..]).unwrap();
    let conn = client.get_connection().unwrap();
    Mutex::new(conn)
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
