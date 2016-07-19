extern crate mioco;
extern crate httparse;

use std::net::SocketAddr;
use std::str::FromStr;
use std::io::{self, Write, Read};
use mioco::tcp::TcpListener;

mod assets;

const MAX_REQUEST_SIZE: usize = 1024;

fn main() {
    let addr = listend_addr();
    let listener = TcpListener::bind(&addr).expect("Failed to open listener");
    println!("Starting trollserv http server on {:?}", listener.local_addr().unwrap());

    mioco::start(move || {
        // Spawn a listener for each mioco threadpool thread
        for _ in 0..mioco::thread_num() {
            let listener = listener.try_clone().expect("Failed to make a copy of the listener");
            mioco::spawn(move || {
                // Loop forever, accepting connections
                loop {
                    let conn = listener.accept().expect("A failure occurred while accepting a client");
                    mioco::spawn(move || handle_client(conn));
                }
            });
        }
    }).expect("A failure occurred inside mioco");
}

fn handle_client(mut conn: mioco::tcp::TcpStream) -> io::Result<()> {
    let mut buf_i = 0;
    let mut buf = [0u8; MAX_REQUEST_SIZE];

    let mut headers = [httparse::EMPTY_HEADER; 16];
    loop {
        // On each iteration, try to read some data from the client
        let len = try!(conn.read(&mut buf[buf_i..]));
        if len == 0 {
            return Ok(());
        }

        buf_i += len;

        // Check if we're done parsing headers
        let mut req = httparse::Request::new(&mut headers);
        let res = req.parse(&buf[0..buf_i]).expect("Failed to parse headers");
        
        if res.is_complete() {
            match req.path {
                Some(path) => match path {
                    // Variants of the html page
                    "/"           => try!(serve_static(&mut conn, &assets::TROLL_HTML)),
                    "/index.html" => try!(serve_static(&mut conn, &assets::TROLL_HTML)),
                    "/troll.html" => try!(serve_static(&mut conn, &assets::TROLL_HTML)),

                    // Assets
                    "/troll.js"   => try!(serve_static(&mut conn, &assets::TROLL_JS)),
                    "/troll.gif"  => try!(serve_static(&mut conn, &assets::TROLL_GIF)),
                    "/troll.mp3"  => try!(serve_static(&mut conn, &assets::TROLL_MP3)),
                    "/troll.ogg"  => try!(serve_static(&mut conn, &assets::TROLL_OGG)),

                    // TODO: API for troll count

                    // Oh noes, 404
                    _ => try!(serve_static(&mut conn, &assets::HTTP_404)),
                },
                None => try!(serve_static(&mut conn, &assets::HTTP_404)),
            }
        }
    }
}

fn serve_static(conn: &mut mioco::tcp::TcpStream, asset: &[u8]) -> io::Result<()> {
    try!(write!(conn, "HTTP/1.1 200 OK\r\n"));
    try!(write!(conn, "Content-Length: {}", asset.len()));
    try!(conn.write_all(asset));
    Ok(())
}

const DEFAULT_LISTEN_ADDR : &'static str = "0.0.0.0:1337";
fn listend_addr() -> SocketAddr {
    FromStr::from_str(DEFAULT_LISTEN_ADDR).expect("Failed to parse DEFAULT_LISTEN_ADDR")
}
