use std::{
    io::{Read, Write},
    os::unix::net::UnixStream,
};
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
}

pub struct HttpResponse {
    pub status: u8,
    pub body: String,
}

pub struct Http;

pub trait Client {
    fn create_request(path: String, route: String, method: HttpMethod, body: String)
        -> HttpRequest;
    fn send_request(request: HttpRequest) -> Result<HttpResponse, String>;
}

impl Client for Http {
    fn create_request(
        path: String,
        route: String,
        method: HttpMethod,
        body: String,
    ) -> HttpRequest {
        HttpRequest {
            method,
            body,
            path,
            route,
        }
    }

    fn send_request(request: HttpRequest) -> Result<HttpResponse, String> {
        let request_str: String = format!(
            "{} {} HTTP/1.0\r\n\r\n",
            match request.method {
                HttpMethod::GET => "GET",
                HttpMethod::POST => "POST",
                HttpMethod::PUT => "PUT",
                HttpMethod::DELETE => "DELETE",
            },
            request.route
        );

        let mut stream = match UnixStream::connect(request.path) {
            Ok(r) => r,
            Err(e) => return Err(e.to_string()),
        };

        stream.write_all(request_str.as_bytes()).unwrap();

        let mut response = String::new();
        stream.read_to_string(&mut response).unwrap();

        match response.as_str().lines().last() {
            Some(body) => {
                return Ok(HttpResponse {
                    status: 200,
                    body: String::from(body),
                });
            }
            None => Err(String::from("Nothing found")),
        }
    }
}
