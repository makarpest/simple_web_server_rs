use std::{ thread, fs, io::prelude::* };
use std::net::{ TcpListener, TcpStream };

use hello_web::ThreadPool;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = vec![0; 512];
    stream.read_exact(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status, filename) = if buffer.starts_with(get) {

        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")

    } else if buffer.starts_with(sleep) {

        thread::sleep(std::time::Duration::from_secs(5));

        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")

    } else {

        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")

    };

    let cont = fs::read_to_string(filename).unwrap();
    let resp = format!("{status}{cont}");

    stream.write_all(resp.as_bytes()).unwrap();
    stream.flush().unwrap();
}
