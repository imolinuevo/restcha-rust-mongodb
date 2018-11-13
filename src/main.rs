#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
use rocket_contrib::json::{Json, JsonValue};

fn main() {
    rocket::ignite()
    .register(catchers![not_found, invalid_entity])
    .mount("/", routes![hello])
    .launch();
}

#[derive(Deserialize)]
struct User {
    name: String,
    birthdate: String,
}

#[post("/hello", format = "json", data = "<user>")]
fn hello(user: Json<User>) -> JsonValue {
    json!({
        "message": "",
        "data": {
            "id": "1",
            "name": user.name,
            "birthdate": user.birthdate
        }
    })
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