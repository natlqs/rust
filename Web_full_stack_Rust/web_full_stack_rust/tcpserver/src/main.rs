use std::io::{Read, Write};
use std::net::TcpListener;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Listening on 127.0.0.1:8080");

    // let result = listener.accept().unwrap(); // 只能接受一个连接
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("New connection: {:?}", &stream);
        let mut buffer = [0; 1024];

        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    }
}
