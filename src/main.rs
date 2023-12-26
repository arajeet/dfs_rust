//use std::borrow::Borrow;
use std::io::{self, stdin, Read, Write};
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

    let mut results = vec![];
    for (_, [method, path, http_version]) in re.captures_iter(&resp).map(|c| c.extract()) {
        results.push((method, path, http_version));
    }
    for data in results {
        println!("{:?}", data);
    }
}
// io::stdin().read_line(&mut resp).and_then(op)

fn handle_connection(mut stream: TcpStream) {
    let mut http_info = String::new();
    // stream.set_nonblocking(true).unwrap();
    let mut buffer = Vec::new();
    // let len = stream.peek(&mut buf).expect("peek failed");
    let ten_millis = time::Duration::from_millis(1000);
    stream.set_nodelay(true).unwrap();
    stream.set_nonblocking(true).unwrap();
    //  stream.set_ttl(10).unwrap();
    // let mut response= stream.try_clone();
    //    let respx= stream.borrow();

    //    for byte in respx.bytes(){
    //        println!("{}", byte.unwrap());
    //    }
         loop {
        match stream.read_to_end(&mut buffer) {
            Ok(_) => break,
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // wait until network socket is ready, typically implemented
                // via platform-specific APIs such as epoll or IOCP

                println!("{e}");
                sleep(ten_millis)
            }
            Err(e) => panic!("encountered IO error: {e}"),
        };
    } 
/*     stream.read_to_end(buf)
    loop {
        match stream.read_to_string(&mut http_info) {
            Ok(_) => break,
            // Err(e) => println!("couldn't get client: {e:?}"),
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // wait until network socket is ready, typically implemented
                // via platform-specific APIs such as epoll or IOCP

                println!("{e}");
                sleep(ten_millis)
            }
            Err(e) => panic!("encountered IO error: {e}"),
        };
    } */
    let s = String::from_utf8(buffer).expect("Found invalid UTF-8");
    println!("result: {}", s);
    url_parser(&s);
    println!("{http_info}");
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
    //stream.flush().unwrap();
    // stream.write(b"handleConnection").unwrap();
    //    stream.shutdown(std::net::Shutdown::Both).expect("Shutdown Failed");
    println!("{http_info}");
    //  stream.set_nodelay(true);
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
   // let tcp_listener2 = listener.try_clone().unwrap();
    let t = spawn(move || loop {
        match listener.accept() {
            //  Ok((_socket, addr)) => println!("new client: {addr:?}"),
            Ok((_socket, _)) => handle_connection(_socket),
            Err(e) => println!("couldn't get client: {e:?}"),
        }

        //  listener.
    });
 //   drop(listener);
    assert!(t.join().is_ok());
    //Ok(())
}
