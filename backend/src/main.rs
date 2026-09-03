#[macro_use] extern crate rocket;

use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Serialize)]
struct DataResponse {
    message: String,
}

// CORS Fairing to allow frontend access
pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS Headers",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "http://localhost:5173"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[get("/api/data")]
fn get_data() -> Json<DataResponse> {
    Json(DataResponse {
        message: "Hello from the Rocket backend!".to_string(),
    })
}

#[options("/api/data")]
fn options_data() -> rocket::http::Status {
    rocket::http::Status::NoContent
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .mount("/", routes![get_data, options_data])
}
