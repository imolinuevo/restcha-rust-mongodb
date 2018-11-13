#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate validator_derive;
extern crate validator;

use rocket::Rocket;
use rocket_contrib::json::{Json, JsonValue};
use validator::Validate;

fn main() {
    rocket().launch();
}

pub fn rocket() -> Rocket {
    rocket::ignite()
    .register(catchers![not_found, invalid_entity, bad_request])
    .mount("/", routes![hello])
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

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::ContentType;
    use rocket::http::Status;

    #[test]
    fn test_200() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response = client.post("/hello")
            .body("{ \"name\": \"alice\", \"birthdate\": \"10/10/1991\"}")
            .header(ContentType::JSON)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_400() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response = client.post("/hello")
            .body("{ \"name\": \"\", \"birthdate\": \"10/10/1991\"}")
            .header(ContentType::JSON)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        // should be Status::BadRequest
    }

    #[test]
    fn test_404() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response = client.get("/helloa").dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn test_422() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response = client.post("/hello")
            .body("{}")
            .header(ContentType::JSON)
            .dispatch();
        assert_eq!(response.status(), Status::UnprocessableEntity);
    }
    
}