#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate validator_derive;
extern crate validator;

use rocket::Rocket;
use rocket_contrib::json::{Json, JsonValue};
use validator::Validate;

#[cfg(test)] mod tests;

fn main() {
    rocket().launch();
}

pub fn rocket() -> Rocket {
    rocket::ignite()
    .register(catchers![
        not_found,
        invalid_entity,
        bad_request])
    .mount("/", routes![
        hello,
        create_pet,
        update_pet
    ])
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
            bad_request()
        )
    }
}

#[derive(Deserialize, Validate)]
struct Pet {
    #[validate(range(min = "1", max = "65535"))]
    id: i32,
    #[validate]
    category: Category,
    #[validate(length(min = "1"))]
    name: String,
    #[validate]
    tags: Vec<Tag>,
    #[validate(length(min = "1"))]
    status: String
}

#[derive(Deserialize, Validate)]
struct Category {
    #[validate(range(min = "1", max = "65535"))]
    id: i32,
    #[validate(length(min = "1"))]
    name: String,
}

#[derive(Deserialize, Validate)]
struct Tag {
    #[validate(range(min = "1", max = "65535"))]
    id: i32,
    #[validate(length(min = "1"))]
    name: String,
}

#[post("/pet", format = "json", data = "<pet>")]
fn create_pet(pet: Json<Pet>) -> JsonValue {
    match pet.validate() {
        Ok(_) => (
            json!({
                "message": format!("Pet {} created successfully.", pet.name)
            })
        ),
        Err(_e) => (
            bad_request()
        )
    }
}

#[put("/pet", format = "json", data = "<pet>")]
fn update_pet(pet: Json<Pet>) -> JsonValue {
    match pet.validate() {
        Ok(_) => (
            json!({
                "message": format!("Pet {} created successfully.", pet.name)
            })
        ),
        Err(_e) => (
            bad_request()
        )
    }
}

#[catch(400)]
fn bad_request() -> JsonValue {
    json!({
        "message": "Error 400: Bad request",
        "data": {}
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
        "message": "Error 422: Unprocessable entity",
        "data": {}
    })
}