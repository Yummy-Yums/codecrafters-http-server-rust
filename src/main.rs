use std::io::{Read, Write};
#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;

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
                    handle_read_request_body(stream);
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
            return;;
        }

        let response = b"HTTP/1.1  200 OK\r\n";

        stream.write_all(response).expect("Failed to write to server");
        return;

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

                return;
            }
        } else {
            let response = format!("HTTP/1.1 404 Not Found\r\n\r\n");
            stream.write(response.as_bytes()).expect("Failed to write to server");
            return;
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

            return;

        } else {
            let response = format!("HTTP/1.1 404 Not Found\r\n\r\n");
            stream.write(response.as_bytes()).expect("Failed to write to server");
            return;
        }

    }
}

fn handle_returns_a_file(mut stream: TcpStream) {
    let mut buf = [0; 1024];

    loop {
        let bytes_read = stream.read(&mut buf).expect("Failed to read from client");

        if bytes_read == 0 {
            return ;
        }

        let mut info: Vec<&str> = std::str::from_utf8(&buf[..bytes_read])
            .unwrap()
            .split("\r\n")
            .collect();

        info.retain(|s| !s.is_empty());
        println!("info {:?}", info);

        let mut http_response = info[0].split_whitespace();
        let _ =  http_response.next();
        let path = http_response.next().unwrap();


        if path.starts_with("/files"){

            let filename_from_path = path.split_inclusive("/files").into_iter().last();

            if filename_from_path == Some("/") {
                let http_header = "HTTP/1.1 404 Not Found\r\n\r\n";
                let response = format!("{}\r\nempty filename, please specify filename ", http_header);
                stream
                    .write_all(response.as_bytes())
                    .expect("Failed to write to server");
                return;
            }

            let file_path = PathBuf::from(filename_from_path.unwrap());
            let temp_path = std::env::temp_dir();
            let temp_file = temp_path.join(file_path.file_name().unwrap());
            println!("temp file {:?}", file_path);

            let res = if temp_file.exists() {
                let contents =  std::fs::read_to_string(temp_file).unwrap_or("File not found".to_string());
                contents
            } else {
                let http_header = "HTTP/1.1 404 Not Found\r\n\r\n";
                let filename  = filename_from_path.unwrap().strip_prefix('/').unwrap();
                let response = format!("{}\r\nFilename {} not found", http_header, filename);
                stream
                    .write_all(response.as_bytes())
                    .expect("Failed to write to server");
                return;
            };

            let http_header = "HTTP/1.1  200 OK\r\n";
            let content_length_header = format!("Content-Length: {}\r\n", bytes_read);

            let response = format!(
                "{}{}Content-Type: application/octet-stream\r\n\r\n{}",
                http_header,
                content_length_header,
                res
            );

            stream
                .write_all(response.as_bytes())
                .expect("Failed to write to server");

           return;

        } else {
            let response = format!("HTTP/1.1 404 Not Found\r\n\r\n");
            stream.write(response.as_bytes()).expect("Failed to write to server");
            return;
        }

    }
}

fn handle_read_request_body(mut stream: TcpStream) {

    let mut buf = [0; 1024];

    loop {
        let bytes_read = stream.read(&mut buf).expect("Failed to read from client");

        if bytes_read == 0 {
            return ;
        }

        let mut info: Vec<&str> = std::str::from_utf8(&buf[..bytes_read])
            .unwrap()
            .split("\r\n")
            .collect();

        info.retain(|s| !s.is_empty());

        let mut http_response = info[0].split_whitespace();
        let _ =  http_response.next();
        let path = http_response.next().unwrap();

        let body = info.last().unwrap();

        if path.starts_with("/files"){

            let filename_from_path = path
                .split_inclusive("/files")
                .into_iter()
                .last();

            if filename_from_path == Some("/") {
                let http_header = "HTTP/1.1 404 Not Found\r\n\r\n";
                let response = format!("{}\r\nempty filename, please specify filename", http_header);
                stream
                    .write_all(response.as_bytes())
                    .expect("Failed to write to server");
                return;
            }

            let file_path = PathBuf::from(filename_from_path.unwrap());
            let temp_path = std::env::temp_dir();
            let temp_file = temp_path.join(file_path.file_name().unwrap());

            std::fs::write(temp_file, body).expect("Failed to write contents");

            let response = "HTTP/1.1 201 Created\r\n\r\n";

            stream
                .write_all(response.as_bytes())
                .expect("Failed to write to server");

            return;

        } else {
            let response = format!("HTTP/1.1 404 Not Found\r\n\r\n");
            stream.write(response.as_bytes()).expect("Failed to write to server");
            return;
        }
    }
}

fn