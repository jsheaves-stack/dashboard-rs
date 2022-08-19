// use http::{Client, Http, HttpMethod};
use serde_json::{Error, Map, Value};

use crate::http::{Client, Http, HttpMethod};

pub trait DockerInterface {
    fn get_all_containers() -> Result<Vec<Map<String, Value>>, String>;
    fn get_container(container_id: &String) -> Result<Map<String, Value>, String>;
    fn stop_container(container_id: &String) -> Result<(), String>;
    fn start_container(container_id: &String) -> Result<(), String>;
    fn get_container_logs(container_id: &String) -> Result<String, String>;
    fn get_container_stats(container_id: &String) -> Result<Map<String, Value>, String>;
    fn get_container_top(container_id: &String) -> Result<Map<String, Value>, String>;
}

pub struct Docker;

impl DockerInterface for Docker {
    fn get_all_containers() -> Result<Vec<Map<String, Value>>, String> {
        match Http::send_request(Http::create_request(
            String::from("/var/run/docker.sock"),
            String::from("/v1.41/containers/json"),
            HttpMethod::GET,
            String::new(),
            String::new(),
        )) {
            Ok(response) => {
                let json_response: Result<Vec<Map<String, Value>>, Error> =
                    serde_json::from_str(&response);

                match json_response {
                    Ok(containers) => Ok(containers),
                    Err(e) => Err(e.to_string()),
                }
            }
            Err(e) => Err(String::from(e)),
        }
    }

    fn get_container(container_id: &String) -> Result<Map<String, Value>, String> {
        match Http::send_request(Http::create_request(
            String::from("/var/run/docker.sock"),
            String::from(format!("/v1.41/containers/{}/json", container_id)),
            HttpMethod::GET,
            String::new(),
            String::new(),
        )) {
            Ok(response) => {
                let json_response: Result<Map<String, Value>, Error> =
                    serde_json::from_str(&response);

                match json_response {
                    Ok(container) => Ok(container),
                    Err(e) => Err(e.to_string()),
                }
            }
            Err(e) => Err(String::from(e)),
        }
    }

    fn get_container_logs(container_id: &String) -> Result<String, String> {
        match Http::send_request(Http::create_request(
            String::from("/var/run/docker.sock"),
            String::from(format!("/v1.41/containers/{}/logs", container_id)),
            HttpMethod::GET,
            String::new(),
            String::from("?follow=false"),
        )) {
            Ok(logs) => Ok(logs),
            Err(e) => Err(String::from(e)),
        }
    }

    fn stop_container(container_id: &String) -> Result<(), String> {
        match Http::send_request(Http::create_request(
            String::from("/var/run/docker.sock"),
            String::from(format!("/v1.41/containers/{}/stop", container_id)),
            HttpMethod::POST,
            String::new(),
            String::new(),
        )) {
            Ok(_) => Ok(()),
            Err(e) => Err(String::from(e)),
        }
    }

    fn start_container(container_id: &String) -> Result<(), String> {
        match Http::send_request(Http::create_request(
            String::from("/var/run/docker.sock"),
            String::from(format!("/v1.41/containers/{}/start", container_id)),
            HttpMethod::POST,
            String::new(),
            String::new(),
        )) {
            Ok(_) => Ok(()),
            Err(e) => Err(String::from(e)),
        }
    }

    fn get_container_stats(container_id: &String) -> Result<Map<String, Value>, String> {
        match Http::send_request(Http::create_request(
            String::from("/var/run/docker.sock"),
            String::from(format!("/v1.41/containers/{}/stats", container_id)),
            HttpMethod::GET,
            String::new(),
            String::from("?stream=false"),
        )) {
            Ok(logs) => {
                let json_response: Result<Map<String, Value>, Error> = serde_json::from_str(&logs);

                match json_response {
                    Ok(logs) => Ok(logs),
                    Err(e) => Err(e.to_string()),
                }
            }
            Err(e) => Err(String::from(e)),
        }
    }

    fn get_container_top(container_id: &String) -> Result<Map<String, Value>, String> {
        match Http::send_request(Http::create_request(
            String::from("/var/run/docker.sock"),
            String::from(format!("/v1.41/containers/{}/top", container_id)),
            HttpMethod::GET,
            String::new(),
            String::new(),
        )) {
            Ok(logs) => {
                let json_response: Result<Map<String, Value>, Error> = serde_json::from_str(&logs);

                match json_response {
                    Ok(logs) => Ok(logs),
                    Err(e) => Err(e.to_string()),
                }
            }
            Err(e) => Err(String::from(e)),
        }
    }
}
