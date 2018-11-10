#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate rocket_contrib;
use rocket_contrib::json::Json;
use rocket::Request;

fn main() {
    rocket::ignite()
    .register(catchers![not_found, invalid_entity])
    .mount("/", routes![hello])
    .launch();
}

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    birthdate: String,
}

#[post("/hello", format = "json", data = "<user>")]
fn hello(user: Json<User>) -> String {
    format!("username: {}, birthdate {}", user.name, user.birthdate)
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

#[catch(422)]
fn invalid_entity() -> String {
    format!("Invalid input data")
}