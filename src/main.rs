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
                std::thread::spawn(|| {
                    handle_get_user_agent_request(stream);
                });
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

fn handle_get_request(mut stream: TcpStream) {
    let mut buf = [0; 1024];

    loop {
        let bytes_read = stream.read(&mut buf).expect("Failed to read from client");

        let mut info: Vec<&str> = std::str::from_utf8(&buf[..bytes_read])
            .unwrap()
            .split("\r\n")
            .collect();

        info.retain(|s| !s.is_empty());

        let mut http_response = info[0].split_whitespace();
        let _ =  http_response.next();
        let path = http_response.next().unwrap();

        if path.contains("/echo"){
            let mut res = path.split_inclusive("/echo/");

            let echo_path = res.next();
            let content = res.next().unwrap();

            if bytes_read == 0 {
                return ;
            }

            if echo_path == Some("/echo/") {

                let http_header = "HTTP/1.1  200 OK\r\n";
                let content_length_header = format!("Content-Length: {}\r\n", bytes_read);

                let response = format!(
                    "{}{}Content-Type: text/plain\r\n\r\n{}",
                    http_header,
                    content_length_header,
                    content
                );

                stream
                    .write_all(response.as_bytes())
                    .expect("Failed to write to server");

                break;
            }
        } else {
            let response = format!("HTTP/1.1 404 Not Found\r\n\r\n");
            stream.write(response.as_bytes()).expect("Failed to write to server");
            break;
        }

    }
}

fn handle_get_user_agent_request(mut stream: TcpStream) {
    let mut buf = [0; 1024];

    loop {
        let bytes_read = stream.read(&mut buf).expect("Failed to read from client");

        let mut info: Vec<&str> = std::str::from_utf8(&buf[..bytes_read])
            .unwrap()
            .split("\r\n")
            .collect();

        info.retain(|s| !s.is_empty());
        println!("info {:?}", info);

        let mut http_response = info[0].split_whitespace();
        let _ =  http_response.next();
        let path = http_response.next().unwrap();

        if path.contains("/user-agent"){

            let content = info.last().cloned().unwrap().split_whitespace().last().unwrap();

            if bytes_read == 0 {
                return ;
            }

            let http_header = "HTTP/1.1  200 OK\r\n";
            let content_length_header = format!("Content-Length: {}\r\n", bytes_read);

            let response = format!(
                "{}{}Content-Type: text/plain\r\n\r\n{}",
                http_header,
                content_length_header,
                content
            );

            stream
                .write_all(response.as_bytes())
                .expect("Failed to write to server");

            break;

        } else {
            let response = format!("HTTP/1.1 404 Not Found\r\n\r\n");
            stream.write(response.as_bytes()).expect("Failed to write to server");
            break;
        }

    }
}
