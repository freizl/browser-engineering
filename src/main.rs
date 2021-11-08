use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream: TcpStream = TcpStream::connect("example.org:80")?;

    stream.write(b"GET /index.html HTTP/1.0\r\n")?;
    stream.write(b"Host: example.org\r\n\r\n")?;
    stream.flush()?;

    let mut buffer = String::new();
    stream.read_to_string(&mut buffer)?;
    print!("{}", buffer);

    Ok(())
}
