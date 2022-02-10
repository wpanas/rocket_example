#[macro_use]
extern crate rocket;
extern crate rocket_example;

use rocket_example::app;

#[launch]
fn rocket() -> _ {
    app()
}

