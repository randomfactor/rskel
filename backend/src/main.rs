#[macro_use]
extern crate rocket;

mod db;

use rocket::State;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::serde::json::Json;
use rocket::{Request, Response};
use serde::Serialize;
use serde_json::Value;

use db::{DbPool, KVStore};

type LocalDbPool = DbPool<surrealdb::engine::local::Db>;

#[derive(Serialize)]
struct DataResponse {
    message: String,
    visits: i64,
}

#[derive(Serialize)]
struct VisitResponse {
    total: i64,
}

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
        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            "http://localhost:5173",
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[get("/")]
async fn root(store: &State<LocalDbPool>) -> (rocket::http::ContentType, String) {
    let visits = store
        .increment("counter:global_home_visits", 1)
        .await
        .unwrap_or(0);

    (
        rocket::http::ContentType::HTML,
        format!(
            "<!DOCTYPE html><html><head><meta charset=\"utf-8\"><title>RSKEL</title></head><body><h1>RSKEL</h1><p>Visits: {visits}</p></body></html>"
        ),
    )
}

#[get("/api/data")]
async fn get_data(store: &State<LocalDbPool>) -> Json<DataResponse> {
    let visits = store
        .increment("counter:global_home_visits", 1)
        .await
        .unwrap_or(0);

    Json(DataResponse {
        message: "Hello from the Rocket backend!".to_string(),
        visits,
    })
}

#[get("/api/visits")]
async fn get_visits(store: &State<LocalDbPool>) -> Json<VisitResponse> {
    let total = match store.get("counter:global_home_visits").await {
        Ok(Some(Value::Number(number))) => number.as_i64().unwrap_or(0),
        Ok(Some(Value::String(value))) => value.parse::<i64>().unwrap_or(0),
        Ok(_) => 0,
        Err(_) => 0,
    };

    Json(VisitResponse { total })
}

#[options("/api/data")]
fn options_data() -> rocket::http::Status {
    rocket::http::Status::NoContent
}

#[launch]
async fn rocket() -> _ {
    let store: LocalDbPool = DbPool::from_env()
        .await
        .expect("failed to initialize SurrealDB connection pool");

    rocket::build()
        .manage(store)
        .attach(CORS)
        .mount("/", routes![root, get_data, get_visits, options_data])
}
