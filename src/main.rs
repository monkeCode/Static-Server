mod server;
mod http;
mod thread_pool;

use server::*;
use http::request::Request;
use std::io::{BufRead, Write};
use std::sync::Arc;
use std::{
    io::BufReader,
    net::{TcpListener, TcpStream},
};
use thread_pool::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let f_server: FileServer = server::FileServer {
        root: String::from("html"),
    };
    let proxy = ProxyServer{destination: "left-sock.ru".to_owned(), port:7878};

    let handler = Arc::new(ConnectionHandler { server: proxy });
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let handler_clone = Arc::clone(&handler);
        let f = move || {
            handler_clone.handle_connection(stream);
        };
        pool.execute(f);
    }
}

fn get_request_path(full_path:&str) {

}

struct ConnectionHandler {
    server: ProxyServer,
}
impl ConnectionHandler {
    fn handle_connection(&self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader.lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        println!("Request: {http_request:#?}");
        if http_request.is_empty() {
            return;
        }
        let request = Request::new(&http_request);

        let response = self.server.perform(&request);

        stream.write_all(response.to_string().as_bytes()).unwrap();
    }
}
