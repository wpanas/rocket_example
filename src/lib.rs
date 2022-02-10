pub mod dto;

use rocket::{Build, Rocket};
use dto::{GreetingsRequest, GreetingsResponse};

#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;

#[get("/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[post("/", data = "<content>")]
fn greetings(content: Json<GreetingsRequest>) -> Json<GreetingsResponse> {
    Json(GreetingsResponse {
        greetings: format!("Hello {}", content.name)
    })
}

pub fn app() -> Rocket<Build> {
    rocket::build()
        .mount("/hello", routes![hello])
        .mount("/greetings", routes![greetings])
}
