use crate::http::request::Request;
use crate::http::response::Response;
use crate::http::codes::{HttpCode, ContentType};
use std::net::{TcpStream, ToSocketAddrs};
use std::fs;
use std::io::{BufRead, BufReader, Read, Write};

pub trait Server {
    fn perform(&self, request: &Request) -> Response;
}

pub struct FileServer {
    pub root: String,
}

impl FileServer {
    fn get_file_data(path: &str) -> Result<String, std::io::Error> {
        let file = fs::read_to_string(path)?;
        Ok(file)
    }

    fn get_404() -> String {
        String::from(
            "<!DOCTYPE html>
        <head>

        </head>
        <body>
        <h1>404 Not found</h1>
        </body>",
        )
    }

    fn get_folder_data(path: &str, original_path: &str) -> Result<String, std::io::Error> {
        let dir = fs::read_dir(path)?;
        let mut m = String::from(
        "<!DOCTYPE html>
        <head>

        </head>
        <body>",
        );
        if original_path.contains("/") {
            let splits: Vec<&str> = original_path.split("/").collect();
            print!("{:#?}", splits);
            if splits.len() > 2 {
                m += &format!(
                    "<a href = {}>..</a></br>\r\n",
                    splits[0..splits.len() - 1].join("/")
                );
            } else {
                m += "<a href = \"/\">..</a></br>\r\n"
            }
        }
        for f in dir {
            if let Ok(file) = f {
                let name = file.file_name().into_string().unwrap();
                let metadata = file.metadata()?;
                let icon;
                let file_type = if metadata.is_dir() {
                    icon = "folder-solid.svg";
                    "directory"
                } else if metadata.is_file() {
                    icon = "file-solid.svg";
                    "file"
                } else {
                    icon = "link-solid.svg";
                    "symlink"
                };
                m += &format!(
                    "<img src = {} alt={} style = \" width:10px; \"/> <a href = {}>{}</></br>\r\n",
                    format!("file_server/{}", icon),
                    file_type,
                    format!("{}/{}", original_path, &name),
                    name
                )
            }
        }
        m += "</body>";
        return Ok(m);
    }

    fn get_data(&self, path: &str, extension: &mut String) -> Result<String, std::io::Error> {
        extension.clear();
        extension.push_str("html");
        if path == "/" {
            let p = format!("{}/index.html", self.root);
            if std::path::Path::new(&p).exists() {
                return FileServer::get_file_data(&p);
            }
        }

        let formated_path = path.strip_suffix("/").unwrap_or_else(|| path);
        let mut p = format!("{}{formated_path}", self.root);

        if formated_path.split("/").last().unwrap().contains(".") {
            extension.clear();
            extension.push_str(formated_path.split(".").last().unwrap());
            FileServer::get_file_data(&p)
        } else if std::path::Path::new(&p).exists() {
            FileServer::get_folder_data(&p, formated_path)
        } else {
            p += ".html";
            FileServer::get_file_data(&p)
        }
    }

    
}

impl Server for FileServer {

    fn perform(&self, request: &Request) -> Response {
        let status: HttpCode;
        let text_status;
        let data;
        let mut extension = String::new();

        match self.get_data(&request.path, &mut extension) {
            Ok(d) => {
                status = HttpCode::Ok;
                text_status = "OK";
                data = d;
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    status = HttpCode::NotFound;
                    text_status = "NOT FOUND";
                    data = FileServer::get_404()
                } else {
                    panic!("State not expected");
                }
            }
        };

        let len = data.len();
        Response {
            status: status,
            text_status: String::from(text_status),
            version: request.version.to_owned(),
            content: data,
            content_length: len,
            content_type: ContentType::from_extension(&extension),
            charset: "Utf-8".to_owned()
        }
    }
}

pub struct ProxyServer 
{
    pub destination: String,
    pub port: u16
}

impl Server for ProxyServer {

    fn perform(&self, request: &Request) -> Response {
        // let addr =format!("{}:80", &self.destination,);
        // match std::net::TcpStream::connect(addr) {
        //     Ok(mut stream) => {

        //         stream.write_all(request.request.as_bytes());
        //         let buf_reader = BufReader::new(&mut stream);
        //         let http_request: Vec<_> = buf_reader.lines()
        //         .map(|result| result.unwrap())
        //         .take_while(|line| !line.is_empty())
        //         .collect();
        //         Response::from_str(&http_request.concat())
        //     }
        //     Err(e) => 
        //     {
        //         let resp = "SERVER UNVAILABLE".to_owned();
                
        //         Response{status:HttpCode::ServiceUnvailable, version:request.version.to_owned(), 
        //             text_status: "SERVER UNVAILABLE".to_owned(), 
        //             content_length:resp.len(), 
        //             content:resp,
        //             content_type:ContentType::Txt,
        //         }
        //     }


        // Convert URL to socket address
        let addr = (self.destination.to_string(), self.port).to_socket_addrs().unwrap().next().unwrap();

        // Connect to the server
        let mut stream = TcpStream::connect(addr).unwrap();

        // Send HTTP request
        let request = format!(
            "GET {} HTTP/1.1\r\nHost: {}:{}\r\n\r\n{}",
            &request.path, addr.ip().to_string(),self.port, &request.request.lines().skip(2).collect::<Vec<&str>>().join("\r\n")
        );
        println!("{}",request);
        stream.write_all(request.as_bytes()).unwrap();

        // Read response
        let mut response = String::new();
        stream.read_to_string(&mut response).unwrap();

        // Print response
        println!("{}", response);
        Response::from_str(&response)

        }
        }