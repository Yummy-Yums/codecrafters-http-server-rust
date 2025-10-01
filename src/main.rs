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
                    handle_gzip_headers(stream);
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

fn handle_compression_headers(mut stream: TcpStream) {
    let mut buf = [0; 1024];

    loop {
        let bytes_read = stream.read(&mut buf).expect("Failed to read from client");

        if bytes_read == 0 {
            return ;
        }

        let mut info= std::str::from_utf8(&buf[..bytes_read])
            .unwrap();

        let accept_encoding = info.
            lines()
            .find(|line| line.starts_with("Accept-Encoding: "))
            .and_then(|line| line
                .split(":")
                .nth(1)
                .map(|s| s.trim().to_string())
            )
            .unwrap_or("".to_string());
        println!("body {:?}", accept_encoding);
        let content_type = "Content-Type: text/plain\r\n";
        let http_header = "HTTP/1.1 200 OK\r\n";

        let response =
            if accept_encoding.contains("gzip") {
                let encoding = "Content-Encoding: gzip\r\n";
                format!(
                    "{}{}{}\r\n",
                    http_header,
                    encoding,
                    content_type,
                )
            } else {
                format!(
                    "{}{}",
                    http_header,
                    content_type
                )

            };

        stream
            .write_all(response.as_bytes())
            .expect("Failed to write to server");

        return;

    }
}

fn handle_gzip_headers(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    let bytes_read = stream.read(&mut buf).expect("Failed to read from client");

    if bytes_read == 0 {
        return;
    }

    let request_str = std::str::from_utf8(&buf[..bytes_read]).unwrap();

    // Extract path and content
    let path = request_str
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .unwrap_or("");

    // Correct path extraction - remove "/echo/" prefix
    let content = path.strip_prefix("/echo/").unwrap_or("default");
    
    // Parse Accept-Encoding header
    let accept_encoding = request_str
        .lines()
        .find(|line| line.starts_with("Accept-Encoding:"))
        .and_then(|line| line.split(':').nth(1))
        .map(|s| s.trim())
        .unwrap_or("");

    println!("Accept-Encoding: '{}'", accept_encoding);

    let (response_headers, body) = if accept_encoding.contains("gzip") {
        // Compress the body
        let compressed_body = compress_data(content);
        let content_length = compressed_body.len();

        let headers = format!(
            "HTTP/1.1 200 OK\r\n\
             Content-Encoding: gzip\r\n\
             Content-Type: text/plain\r\n\
             Content-Length: {}\r\n\
             \r\n",
            content_length
        );

        (headers, compressed_body)  // This should be the COMPRESSED binary data
    } else {
        // No compression
        let body_bytes = content.as_bytes().to_vec();
        let content_length = body_bytes.len();

        let headers = format!(
            "HTTP/1.1 200 OK\r\n\
             Content-Type: text/plain\r\n\
             Content-Length: {}\r\n\
             \r\n",
            content_length
        );

        (headers, body_bytes)  // This should be the UNCOMPRESSED text
    };

    println!("Response body: {:?}", body);

    // Send headers + body
    stream.write_all(response_headers.as_bytes()).unwrap();
    stream.write_all(&body).unwrap();
}

use flate2::write::GzEncoder;
use flate2::Compression;

fn compress_data(data: &str) -> Vec<u8> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data.as_bytes()).unwrap();
    encoder.finish().unwrap()
}