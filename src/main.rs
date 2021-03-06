use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    for stream in listener.incoming() {
        println!("{:?}", stream);
    }
}
