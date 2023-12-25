use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
//use core::net::socket_addr;

//fn read_stream()
fn url_parser(){


}


fn handle_connection(mut stream: TcpStream) {
    let mut http_info = String::new();
    // stream.set_nonblocking(true).unwrap();
    stream.set_nodelay(true).unwrap();
    stream.set_nonblocking(true).unwrap();
    
    match stream.read_to_string(&mut http_info) {
        Ok(_) => println!("Read"),
        Err(e) => println!("couldn't get client: {e:?}"),
    };

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
    let mut paddr = String::from("127.0.0.1:");
    paddr.push_str(&port);

    println!("Address  for binding: {paddr}");
    let listener = TcpListener::bind(paddr).unwrap();

    loop {
        match listener.accept() {
            //  Ok((_socket, addr)) => println!("new client: {addr:?}"),
            Ok((_socket, _)) => handle_connection(_socket),
            Err(e) => println!("couldn't get client: {e:?}"),
        }

        //  listener.
    }

    //Ok(())
}
