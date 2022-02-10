use rocket::local::blocking::Client;
use rocket_example::dto::{GreetingsResponse, GreetingsRequest};
use rocket_example::app;

#[test]
fn hello_get() {
    let client = Client::tracked(app())
        .expect("valid `Rocket`");

    let response = client.get("/hello/Mike").dispatch();
    assert_eq!(response.into_string().unwrap(), "Hello, Mike!");
}

#[test]
fn greetings_post() {
    let client = Client::tracked(app())
        .expect("valid `Rocket`");

    let response = client.post("/greetings")
        .json(&GreetingsRequest {
            name: String::from("Mike")
        }).dispatch();

    let greetings_response = response
        .into_json::<GreetingsResponse>().unwrap();

    assert_eq!(greetings_response.greetings, "Hello Mike");
}