use std::io::{Read, Write};
#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                handle_stream(stream);

            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

}

fn handle_stream(mut stream: TcpStream) {

    let mut buf = [0; 1024];

    loop {
        let bytes_read = stream.read(&mut buf).expect("Failed to read from client");
        println!("read {} bytes", bytes_read);
        println!("{}", std::str::from_utf8(&buf).unwrap());
        if bytes_read == 0 {
            return ;
        }

        let response = b"HTTP/1.1 200 OK\r\n\r\n";

        stream.write_all(response).expect("Failed to write to server");

    }
}
