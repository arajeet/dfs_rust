//use std::borrow::Borrow;
use std::io::{self, prelude::*, BufReader,};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use regex::Regex;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};
#[derive(Debug)]
struct ThreadPool<Z> {
    size: usize,
    tx: Sender<Z>,
    rx : Receiver<Z>

}

impl<Z> ThreadPool<Z>  {

      pub fn new<F,T>(&self,size: usize)  -> Self
    where 
        F: FnOnce() -> T,
        T: Send + 'static, 
   {
        (self.tx, self.rx)= channel();
        let pool_size = Arc::new(size);
        for i in 1..*pool_size {
            thread::spawn(move||self.)
        }
       // return true
   }

    pub fn send<F,T>(&self,t: T)  -> bool
        where 
         F: FnOnce() -> T,
         T: Send + 'static,
    {
        //let (tx, _rx): (Sender<T>, Receiver<T>) = channel();

        let my_size_join  = thread::spawn( move || self.tx.send(t).unwrap());
        return my_size_join.join().is_ok();
    }

 
}





fn url_parser(resp: &String) {
    let re = Regex::new(r#"(^GET|POST?) (/.*) (HTTP.*)"#).unwrap();
    let mut results = vec![];
    for (_, [method, path, http_version]) in re.captures_iter(&resp).map(|c| c.extract()) {
        //for (_, [method, path]) in re.captures_iter(&resp).map(|c| c.extract()) {
        results.push((method, path, http_version));
    }
    for data in results {
        println!("{:?}", data);
    }
}

fn handle_connection(mut stream: TcpStream) {
    stream.set_nodelay(true).unwrap();
    stream.set_nonblocking(true).unwrap();
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .flatten()
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);
    let s = http_request.join("/n");
    url_parser(&s);
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
    for stream in listener.incoming() {
        thread::spawn(|| match stream {
            Ok(stream) => handle_connection(stream),
            Err(e) => println!("couldn't get client: {e:?}"),
        });

    }
    //assert!(t.join().is_ok());
}
