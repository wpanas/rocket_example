#[macro_use] extern crate rocket;

use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;


#[get("/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[derive(Deserialize)]
struct Request<'a> {
    name: &'a str
}

#[derive(Serialize)]
struct Response {
    greetings: String
}

#[post("/", data="<content>")]
fn greetings(content: Json<Request<'_>>) -> Json<Response> {
    Json(Response {
        greetings: format!("Hello {}", content.name)
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/hello", routes![hello])
        .mount("/greetings", routes![greetings])
}
