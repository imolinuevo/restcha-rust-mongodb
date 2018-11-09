#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate rocket_contrib;
use rocket_contrib::json::Json;


fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
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