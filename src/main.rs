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

        let info: Vec<&str> = std::str::from_utf8(&buf).unwrap().split("\r\n").collect();
        let mut http_response = info[0].split_whitespace();
        let _ =  http_response.next();
        let path = http_response.next();

        if bytes_read == 0 {
            return ;
        }

        if path != Some("/") {
            let response = format!("HTTP/1.1 404 Not Found\r\n\r\n");
            stream.write(response.as_bytes()).expect("Failed to write to server");
            break;
        }


        let response = b"HTTP/1.1  200 OK\r\n";

        stream.write_all(response).expect("Failed to write to server");
        break;

    }
}
