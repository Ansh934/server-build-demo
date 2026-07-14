use server_build::Threadpool;
use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread::{self},
    time::Duration,
};

fn main() {
    let tcp = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server is running on 127.0.0.1:7878");

    let pool = Threadpool::new(4);

    tcp.incoming().for_each(|stream| {
        let stream = stream.unwrap();
        println!("Connection from: {}", stream.peer_addr().unwrap());
        pool.execute(|| handle_connection(stream));
    });
}

fn handle_connection(mut stream: TcpStream) {
    let reader = BufReader::new(&stream);
    let request_line = reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
