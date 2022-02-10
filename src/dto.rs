use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GreetingsRequest {
    pub name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GreetingsResponse {
    pub greetings: String
}