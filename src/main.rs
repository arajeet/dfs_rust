//use std::borrow::Borrow;
use std::io::{self, stdin, Read, Write,prelude::*,BufReader};
use std::net::{TcpListener, TcpStream};
//use std::str::Bytes;
use std::thread::{sleep, spawn};
//use core::net::socket_addr;
use regex::Regex;
//fn read_stream()
use std::time;

//fn parser_method() {}

fn url_parser(resp: &String) {
    let re = Regex::new(r#"(^GET|POST?) (/.*) (HTTP.*)"#).unwrap();
    //let re = Regex::new(r#"(^GET|POST?) (/.*)"#).unwrap();
    let mut results = vec![];
    for (_, [method, path, http_version]) in re.captures_iter(&resp).map(|c| c.extract()) {
        //for (_, [method, path]) in re.captures_iter(&resp).map(|c| c.extract()) {
        results.push((method, path, http_version));
    }
    for data in results {
        println!("{:?}", data);
    }
}
// io::stdin().read_line(&mut resp).and_then(op)

fn handle_connection(mut stream: TcpStream) {
    let mut buffer: [u8; 256] = [0; 256];
    let ten_millis = time::Duration::from_millis(1000);
    stream.set_nodelay(true).unwrap();
    stream.set_nonblocking(true).unwrap();
/*     loop {
        match stream.read(&mut buffer) {
            Ok(_) => break,
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // wait until network socket is ready, typically implemented
                // via platform-specific APIs such as epoll or IOCP

                println!("{e}");
                sleep(ten_millis)
            }
            Err(e) => panic!("encountered IO error: {e}"),
        };
    } */
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);
    let s = http_request.join("/n");
    //let s = String::from_utf8(http_request).expect("Found invalid UTF-8");
   // println!("result: {}", s);
    url_parser(&s);
    // println!("{http_info}");
    //stream.write(buf)
    stream
        .write_all(
            b"HTTP/1.1 200 OK
Content-Type: text/html; charset=utf-8

<!DOCTYPE html>
<html lang=\"en\">
<head>
    <meta charset=\"utf-8\">
    <title>A simple webpage</title>
</head>
<body>
    <h1>ARAJEET</h1>
    <p>Hello, world!</p>
</body>
</html>",
        )
        .unwrap();
}

fn main() {
    let mut port = String::new();
    println!("Enter the port number below:");
    io::stdin()
        .read_line(&mut port)
        .expect("Failed to capture Port Number");

    let port = port.trim_end();

    //create the full URl to bind for the creation of the server
    let mut paddr = String::from("127.0.0.1:");
    paddr.push_str(&port);

    println!("Address  for binding: {paddr}");
    let listener = TcpListener::bind(paddr).unwrap();
    let t = spawn(move || loop {
        for stream in listener.incoming() {
            match stream {
                //  Ok((_socket, addr)) => println!("new client: {addr:?}"),
                Ok(stream) => handle_connection(stream),
                Err(e) => println!("couldn't get client: {e:?}"),
            }
        }

        //  listener.
    });
    //   drop(listener);
    assert!(t.join().is_ok());
    //Ok(())
}
