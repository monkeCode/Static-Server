use std::{borrow::Borrow, collections::{HashMap, HashSet}};

use crate::http::codes::{ContentType, HttpCode};

use super::request;

pub struct Response {
    pub status: HttpCode,
    pub text_status: String,
    pub version: String,
    pub content_length: usize,
    pub content_type: ContentType,
    pub content: String,
    pub charset: String
}

impl Response {
    pub fn from_str(request: &str) -> Self
    {
        let rec: Vec<&str> = request.lines().collect();
        let mut first_line = rec[0].split_whitespace();
        let mut hash: HashMap<String, String> = HashMap::new();
        let mut content = "".to_owned();
        for line in 0..rec.len()
        {
            if rec[line] == ""
            {
                content = rec[line+1..rec.len()].join("\r\n");
                break;
            }
            let splits:Vec<&str> = rec[line].split_whitespace().collect();
            if splits.len() > 1
            {
                hash.insert(splits[0].trim_end_matches(":").to_lowercase(), splits[1..splits.len()].join(" "));
            }
        }
        Response{
            version: first_line.next().unwrap().to_owned(),
            status: HttpCode::try_from(first_line.next().unwrap().parse::<u16>().unwrap()).unwrap(),
            text_status: first_line.next().unwrap().to_owned(),
            content_length: hash.get("content-length").unwrap().parse::<usize>().unwrap(),
            content_type: ContentType::from_str(hash.get("content-type").unwrap()),
            charset: "Utf-8".to_owned(),
            content: content,


        }
    }
    pub fn to_string(&self) -> String {
        // TODO: this place
        String::from(format!(
            "{} {} {}\r\nContent-Type: {}\r\n Content-Length: {}\r\n\r\n{}",
            self.version,
            self.status as u16,
            self.text_status,
            self.content_type.to_string(),
            self.content_length,
            self.content
        ))
    }
}
