use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::net::TcpStream;
use std::collections::HashMap;
use std::env;

// type ResponseHeader = HashMap<&str, &str>;
// type ResponseBody = &str;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    for x in &args {
        print!("{}\r\n", x);
    }
    if args.len() <= 1 {
        let err: Error = Error::from(ErrorKind::InvalidInput);
        return Err(err);
    }

    let url = &args[0];
    // TODO: parse URL

    let mut stream: TcpStream = TcpStream::connect("example.org:80")?;

    stream.write(b"GET /index.html HTTP/1.0\r\n")?;
    stream.write(b"Host: example.org\r\n\r\n")?;
    stream.flush()?;

    let mut buffer = String::new();
    stream.read_to_string(&mut buffer)?;
    // print!("{}", buffer);

    let (status, header, body) = parse_response(&buffer);
    print!("status: {}", status);
    print!("body: {}", body);
    for (k, v) in &header {
        print!("{} - {}\r\n", k, v);
    }

    Ok(())
}

fn parse_response(resp: &str) -> (&str, HashMap<&str, &str>, &str) {
    let xs: Vec<&str> = resp.split("\r\n\r\n").collect();
    let mut ys: Vec<&str> = xs[0].split("\r\n").collect();
    let mut hs = HashMap::new();

    let status = ys.remove(0);
    for hl in ys.iter() {
        print!("{}", hl);
        let zs: Vec<&str> = hl.split(": ").collect();
        hs.insert(zs[0], zs[1]);
    }
    return (status, hs, xs[1]);
}
