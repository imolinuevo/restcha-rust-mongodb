#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use rocket::request::Form;


fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}

#[derive(FromForm)]
struct User {
    name: String,
    birthdate: String,
}

#[post("/hello", data = "<user>")]
fn hello(user: Form<User>) -> String {
    format!("username: {}, birthdate {}", user.name, user.birthdate)
}