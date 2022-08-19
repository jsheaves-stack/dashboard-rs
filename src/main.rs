use rocket::fairing::Fairing;
use rocket::fairing::Info;
use rocket::fairing::Kind;
use rocket::get;
use rocket::http::Header;
use rocket::http::Method;
use rocket::http::Status;
use rocket::launch;
use rocket::response::Response;
use rocket::routes;
use rocket::Request;

use routes::docker::{
    get_all_containers, get_container, get_container_logs, get_container_stats, get_container_top,
    start_container, stop_container,
};

mod docker;
mod http;
mod routes;
mod system;

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

#[get("/healthcheck")]
fn healthcheck() -> () {
    ();
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/docker",
            routes![
                get_all_containers,
                get_container,
                stop_container,
                start_container,
                get_container_logs,
                get_container_stats,
                get_container_top
            ],
        )
        .mount("/", routes![healthcheck])
        .attach(CORS)
}
