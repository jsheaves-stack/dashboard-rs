use std::{
    io::{Read, Write},
    os::unix::net::UnixStream,
};

use log::{trace};

#[allow(dead_code)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

pub struct HttpRequest {
    pub method: HttpMethod,
    pub body: String,
    pub path: String,
    pub route: String,
    pub parameters: String,
}

pub struct Http;

pub trait Client {
    fn create_request(
        path: String,
        route: String,
        method: HttpMethod,
        body: String,
        parameters: String,
    ) -> HttpRequest;
    fn send_request(request: HttpRequest) -> Result<String, String>;
}

impl Client for Http {
    fn create_request(
        path: String,
        route: String,
        method: HttpMethod,
        body: String,
        parameters: String,
    ) -> HttpRequest {
        HttpRequest {
            method,
            body,
            path,
            route,
            parameters,
        }
    }

    fn send_request(request: HttpRequest) -> Result<String, String> {
        let request_str: String = format!(
            "{} {}{} HTTP/1.0\r\n\r\n",
            match request.method {
                HttpMethod::GET => "GET",
                HttpMethod::POST => "POST",
                HttpMethod::PUT => "PUT",
                HttpMethod::DELETE => "DELETE",
            },
            request.route,
            request.parameters
        );

        trace!("{}", request_str);

        let mut stream = match UnixStream::connect(request.path) {
            Ok(r) => r,
            Err(e) => return Err(e.to_string()),
        };

        stream.write_all(request_str.as_bytes()).unwrap();

        let mut response = String::new();
        stream.read_to_string(&mut response).unwrap();

        trace!("{}", response);

        let response_header = match response.as_str().lines().next() {
            Some(header) => String::from(header),
            None => return Err(String::from("No header")),
        };

        trace!("{}", response_header);

        let response_code = response_header
            .as_str()
            .get(9..12)
            .unwrap()
            .parse::<u16>()
            .unwrap();

        match response_code {
            200 | 201 | 202 | 204 | 301 | 304 => {
                Ok(response.as_str().lines().last().unwrap().to_string())
            }
            400..=451 | 500..=511 => Err(format!("Response code {}", response_code.to_string())),
            _ => Err(String::from("Unknown error has ocurred")),
        }
    }
}
