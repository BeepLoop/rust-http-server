use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("localhost:5000");

    match listener {
        Err(err) => println!("{}", err),
        Ok(data) => {
            for stream in data.incoming() {
                let stream = stream.unwrap();

                handle_connection(stream)
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // request_line looks like "GET / HTTP/1.1"
    // take values separated by whitespace to have something like:
    // ["GET", "/", "HTTP/1.1"]
    let request_data: Vec<&str> = request_line.split_whitespace().collect();

    let (status_line, filename) = if request_data[1] == "/" {
        ("HTTP/1.1 200 OK", "./public/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "./public/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
