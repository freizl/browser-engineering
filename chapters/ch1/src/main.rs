use openssl::ssl::{SslConnector, SslMethod};
use std::collections::HashMap;
use std::env;
use std::io::{Error, ErrorKind, Read, Write, Result};
use std::net::TcpStream;
use std::path::Path;
use std::fs;

use hw_uri;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        let err: Error = Error::from(ErrorKind::InvalidInput);
        print!("URL is required to run this program\r\n");
        return Err(err);
    }

    for x in &args {
        print!("-- show all arguments --");
        print!("{}\r\n", x);
    }

    let maybe_uri: Option<hw_uri::URI> = hw_uri::parse(&args[1]);

    match maybe_uri {
        None => invalid_uri(&args[1]),
        Some(uri) => fetch_page(uri),
    }
}

fn invalid_uri(uri: &str) -> Result<()> {
    print!("Invalid uri {}", uri);
    Ok(())
}

fn fetch_page(url: hw_uri::URI) -> Result<()> {
    let mut buffer = String::new();

    if url.is_file() {
        let file_path = Path::new(&url.path);
        if !Path::exists(file_path) {
            let err_msg = format!("can not find file: {}", url.path);
            return Err(Error::new(ErrorKind::NotFound, err_msg));
        }
        let file_content = fs::read(file_path).unwrap();
        print!("{}", String::from_utf8(file_content).unwrap());
        return Ok(());
    }

    if url.use_tls() {
        read_ssl_stream(url, &mut buffer)?;
    } else {
        read_tcp_stream(url, &mut buffer)?;
    }

    let (status, header, body) = parse_response(&buffer);
    print!("status: {}\r\n", status);
    print!("body: {}\r\n", body);
    print!("headers:\r\n");
    for (k, v) in &header {
        print!("\t{} - {}\r\n", k, v);
    }

    Ok(())
}

fn read_tcp_stream(uri_input: hw_uri::URI, buffer: &mut String) -> Result<usize> {
    let domain: String = uri_input.get_domain_port();
    let mut stream: TcpStream = TcpStream::connect(domain)?;

    write_n_read_stream(&mut stream, uri_input, buffer)
}

fn read_ssl_stream(uri_input: hw_uri::URI, buffer: &mut String) -> Result<usize> {
    let domain: String = uri_input.get_domain_port();
    let tcp_stream: TcpStream = TcpStream::connect(domain)?;
    let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();
    let mut stream = connector.connect(&uri_input.domain, tcp_stream).unwrap();

    write_n_read_stream(&mut stream, uri_input, buffer)
}

fn write_n_read_stream<T: Write + Read>(
    stream: &mut T,
    uri_input: hw_uri::URI,
    buffer: &mut String,
) -> Result<usize> {
    let request: String = format!("GET {} HTTP/1.1\r\n", uri_input.path)
        + &(format!("Host: {}\r\n", uri_input.domain))
        + &("User-Agent: hw-browser-poc\r\n")
        + &("Connection: close\r\n")
        + &("\r\n");

    stream.write(request.as_bytes())?;
    stream.flush()?;

    stream.read_to_string(buffer)
}

fn parse_response(resp: &str) -> (&str, HashMap<&str, &str>, &str) {
    // split between body and others
    let xs: Vec<&str> = resp.split("\r\n\r\n").collect();
    // split between status and headers
    let mut ys: Vec<&str> = xs[0].split("\r\n").collect();
    let mut hs = HashMap::new();

    let status = ys.remove(0);
    for hl in ys.iter() {
        let zs: Vec<&str> = hl.split(": ").collect();
        hs.insert(zs[0], zs[1]);
    }
    return (status, hs, xs[1]);
}
