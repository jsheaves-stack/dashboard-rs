use crate::docker::{Docker, DockerInterface};
use log::error;
use rocket::{get, post, serde::json::Json};
use serde_json::{Map, Value};

#[get("/container/all")]
pub fn get_all_containers() -> Result<Json<Vec<Map<String, Value>>>, String> {
    match Docker::get_all_containers() {
        Ok(containers) => Ok(Json(containers)),
        Err(e) => {
            let error_message = format!("Failed to retrieve container list: {}.", e);
            error!("{}", error_message);
            Err(error_message)
        }
    }
}

#[get("/container/<container_id>")]
pub fn get_container(container_id: String) -> Result<Json<Map<String, Value>>, String> {
    match Docker::get_container(&container_id) {
        Ok(container) => Ok(Json(container)),
        Err(e) => {
            let error_message = format!("Failed to retrieve container {}: {}.", container_id, e);
            error!("{}", error_message);
            Err(error_message)
        }
    }
}

#[get("/container/<container_id>/logs")]
pub fn get_container_logs(container_id: String) -> Result<String, String> {
    match Docker::get_container_logs(&container_id) {
        Ok(container) => Ok(container),
        Err(e) => {
            let error_message = format!(
                "Failed to retrieve logs for container {}: {}.",
                container_id, e
            );
            error!("{}", error_message);
            Err(error_message)
        }
    }
}

#[get("/container/<container_id>/stats")]
pub fn get_container_stats(container_id: String) -> Result<Json<Map<String, Value>>, String> {
    match Docker::get_container_stats(&container_id) {
        Ok(container) => Ok(Json(container)),
        Err(e) => {
            let error_message = format!(
                "Failed to retrieve logs for container {}: {}.",
                container_id, e
            );
            error!("{}", error_message);
            Err(error_message)
        }
    }
}

#[get("/container/<container_id>/top")]
pub fn get_container_top(container_id: String) -> Result<Json<Map<String, Value>>, String> {
    match Docker::get_container_top(&container_id) {
        Ok(container) => Ok(Json(container)),
        Err(e) => {
            let error_message = format!(
                "Failed to retrieve logs for container {}: {}.",
                container_id, e
            );
            error!("{}", error_message);
            Err(error_message)
        }
    }
}

#[post("/container/<container_id>/stop")]
pub fn stop_container(container_id: String) -> Result<(), String> {
    match Docker::stop_container(&container_id) {
        Ok(_) => Ok(()),
        Err(e) => {
            let error_message = format!("Failed to stop container {}: {}.", container_id, e);
            error!("{}", error_message);
            Err(error_message)
        }
    }
}

#[post("/container/<container_id>/start")]
pub fn start_container(container_id: String) -> Result<(), String> {
    match Docker::start_container(&container_id) {
        Ok(_) => Ok(()),
        Err(e) => {
            let error_message = format!("Failed to start container {}: {}.", container_id, e);
            error!("{}", error_message);
            Err(error_message)
        }
    }
}
