use super::rocket;
use rocket::local::Client;
use rocket::http::ContentType;
use rocket::http::Status;

#[test]
fn test_create_pet() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let response = client.post("/pet")
        .body(json!({
            "id": 200,
            "category": {
                "id": 200,
                "name": "string"
            },
            "name": "doggie",
            "tags": [{
                "id": 200,
                "name": "string"
            }],
            "status": "available"
            }).to_string()
        )
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_update_pet() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let response = client.put("/pet")
        .body(json!({
            "id": 0,
            "category": {
                "id": 0,
                "name": "string"
            },
            "name": "doggie",
            "tags": [{
                "id": 0,
                "name": "string"
            }],
            "status": "available"
            }).to_string()
        )
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_get_pet_by_id() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let response = client.get("/pet/0")
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_delete_pet_by_id() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let response = client.delete("/pet/0")
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_edit_pet_by_id() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let response = client.post("/pet/0")
        .body("name=doggie&status=available")
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}