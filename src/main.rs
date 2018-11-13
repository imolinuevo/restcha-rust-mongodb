#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate validator_derive;
extern crate validator;

use rocket_contrib::json::{Json, JsonValue};
use validator::Validate;

fn main() {
    rocket::ignite()
    .register(catchers![not_found, invalid_entity])
    .mount("/", routes![hello])
    .launch();
}

#[derive(Deserialize, Validate)]
struct User {
    #[validate(length(min = "1"))]
    name: String,
    birthdate: String,
}

#[post("/hello", format = "json", data = "<user>")]
fn hello(user: Json<User>) -> JsonValue {
    match user.validate() {
        Ok(_) => (
            json!({
                "message": "",
                "data": {
                    "id": "1",
                    "name": user.name,
                    "birthdate": user.birthdate
                }
            })
        ),
        Err(_e) => (
            json!({
                "message": "Invalid input data format",
                "data": {}
            })
        )
    }
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "message": "Error 404: Not found",
        "data": {}
    })
}

#[catch(422)]
fn invalid_entity() -> JsonValue {
    json!({
        "message": "Error 422: Unprocessable Entity",
        "data": {}
    })
}