use http::{Client, Http, HttpMethod};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Container {
    Id: String,
    Names: Vec<String>,
    Image: String,
    ImageID: String,
    Command: String,
    Created: u32,
    State: String,
    Status: String,
    Ports: Vec<Map<String, Value>>,
    Labels: Map<String, Value>,
    SizeRw: Option<u32>,
    SizeRootFs: Option<u32>,
    HostConfig: Map<String, Value>,
    NetworkSettings: Map<String, Value>,
    Mounts: Vec<Map<String, Value>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Containers {
    pub containers: Vec<Container>,
}

pub trait DockerInterface {
    fn get_all_containers() -> Result<Containers, String>;
}

pub struct Docker;

impl DockerInterface for Docker {
    fn get_all_containers() -> Result<Containers, String> {
        match Http::send_request(Http::create_request(
            String::from("/var/run/docker.sock"),
            String::from("/containers/json"),
            HttpMethod::GET,
            String::new(),
        )) {
            Ok(r) => {
                let json_response: Result<Vec<Container>, serde_json::Error> =
                    serde_json::from_str(&r.body);

                match json_response {
                    Ok(containers) => Ok(Containers { containers }),
                    Err(e) => {
                        println!("Error serializing container list: {}", e);
                        Err(String::from(format!(
                            "An unhandled exception has occurrred: {}",
                            e.to_string()
                        )))
                    }
                }
            }
            Err(e) => Err(String::from(format!("Request error: {}", e.to_string()))),
        }
    }
}
