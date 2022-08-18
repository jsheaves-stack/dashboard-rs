use docker::Containers;
use docker::Docker;
use docker::DockerInterface;
use rocket::fairing::Fairing;
use rocket::fairing::Info;
use rocket::fairing::Kind;
use rocket::get;
use rocket::http::Header;
use rocket::http::Method;
use rocket::http::Status;
use rocket::launch;
use rocket::routes;
use rocket::serde::json::Json;
use rocket::Request;
use rocket::Response;
// use system::{ListProcesses, ProcessExplorer};
pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        let domain = "http://localhost:3000";
        let methods = "POST, GET, PATCH, OPTIONS";

        response.set_header(Header::new("Allow", methods));
        response.set_header(Header::new("Access-Control-Allow-Origin", domain));
        response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
        response.set_header(Header::new("Access-Control-Allow-Methods", methods));
        response.set_header(Header::new("Access-Control-Request-Methods", methods));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));

        if request.method() == Method::Options {
            response.set_status(Status::Ok);
        }
    }
}

#[get("/container/all")]
fn get_all_containers() -> Result<Json<Containers>, String> {
    match Docker::get_all_containers() {
        Ok(containers) => Ok(Json(containers)),
        Err(e) => {
            println!("Error retrieving container list: {}", e);
            Err(String::from("Internal server error"))
        }
    }
}

#[get("/healthcheck")]
fn healthcheck() -> () {
    ();
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/docker", routes![get_all_containers])
        .mount("/", routes![healthcheck])
        .attach(CORS)
}
