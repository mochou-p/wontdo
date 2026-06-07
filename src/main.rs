// mochou-p/wontdo/src/main.rs

use std::io::{Read as _, Write as _};
use std::net::{TcpListener, TcpStream};
use webuild::Tag;


fn main() {
    const PORT: u16 = 10067;

    let server = TcpListener::bind(format!("127.0.0.1:{PORT}")).unwrap();

    println!("\x1b[34mopen \x1b[36mlocalhost:{PORT}\x1b[34m in your browser\x1b[0m");
    println!("\x1b[31m^C\x1b[33m to stop the server\x1b[0m");

    for client in server.incoming() {
        handle_client(client.unwrap());
    }
}

fn handle_client(mut client: TcpStream) {
    let request  = read_request(&mut client);
    let response = get(&request);

    client.write_all(response.as_bytes()).unwrap();
}

fn read_request(client: &mut TcpStream) -> String {
    const LEN: usize = 1024;

    let mut buffer = [0; LEN];
    let     count  = client.read(&mut buffer).unwrap();

    assert!(count < LEN);

    String::from_utf8_lossy(&buffer[..count]).into_owned()
}

fn get(request: &str) -> String {
    let     line  = request.lines().next().unwrap();
    let mut parts = line.split_whitespace();

    let Some(method ) = parts.next() else { return bad_request(); };
    let Some(path   ) = parts.next() else { return bad_request(); };
    let Some(version) = parts.next() else { return bad_request(); };

    if method  != "GET"      { return            not_implemented(); }
    if version != "HTTP/1.1" { return http_version_not_supported(); }
    if path    != "/"        { return                  not_found(); }

    document()
}

fn                bad_request() -> String { format!("HTTP/1.1 400 Bad Request{}",                "\r\n\r\n") }
fn            not_implemented() -> String { format!("HTTP/1.1 501 Not Implemented{}",            "\r\n\r\n") }
fn http_version_not_supported() -> String { format!("HTTP/1.1 505 HTTP Version Not Supported{}", "\r\n\r\n") }
fn                  not_found() -> String { format!("HTTP/1.1 404 Not Found{}",                  "\r\n\r\n") }

fn document() -> String {
    let mut response = String::new();

    response.push_str("HTTP/1.1 200 OK\r\n");
    response.push_str("Content-Type: application/xhtml+xml; charset=utf-8\r\n\r\n");
    response.push_str(
        &webuild::DocumentBuilder::default()
            .with_lang("en")
            .with_charset("UTF-8")
            .with_responsive_viewport(true)
            .with_title(env!("CARGO_BIN_NAME"))
            .with_body_children(&[Tag::new("h1").children(&["todo :D"])])
            .build_xhtml()
    );

    response
}

