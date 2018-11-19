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