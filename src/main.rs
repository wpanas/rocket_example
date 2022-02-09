#[macro_use]
extern crate rocket;

use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;


#[get("/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[derive(Serialize, Deserialize)]
struct GreetingsRequest<'a> {
    name: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
struct GreetingsResponse {
    greetings: String,
}

#[post("/", data = "<content>")]
fn greetings(content: Json<GreetingsRequest<'_>>) -> Json<GreetingsResponse> {
    Json(GreetingsResponse {
        greetings: format!("Hello {}", content.name)
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/hello", routes![hello])
        .mount("/greetings", routes![greetings])
}

mod tests {
    use rocket::local::blocking::Client;
    use crate::{GreetingsResponse, GreetingsRequest};

    #[test]
    fn hello_get() {
        let client = Client::tracked(super::rocket())
            .expect("valid `Rocket`");

        let response = client.get("/hello/Mike").dispatch();
        assert_eq!(response.into_string().unwrap(), "Hello, Mike!");
    }

    #[test]
    fn greetings_post() {
        let client = Client::tracked(super::rocket())
            .expect("valid `Rocket`");

        let response = client.post("/greetings")
            .json(&GreetingsRequest {
                name: "Mike"
            }).dispatch();

        let greetings_response = response
            .into_json::<GreetingsResponse>().unwrap();

        assert_eq!(greetings_response.greetings, "Hello Mike");
    }
}