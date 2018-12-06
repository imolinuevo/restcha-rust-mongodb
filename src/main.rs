#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate validator_derive;
#[macro_use] extern crate mongodb;
extern crate validator;

use mongodb::{Bson, doc};
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use rocket::request::Form;
use rocket::Rocket;
use rocket_contrib::json::{Json, JsonValue};
use validator::Validate;

#[cfg(test)] mod tests;

static MONGO_HOST: &str = "localhost";
static MONGO_PORT: u16 = 27017;

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
        create_pet,
        update_pet,
        get_pet_by_id,
        delete_pet_by_id,
        edit_pet_by_id
    ])
}

fn get_collection(database: &str, collection: &str) -> mongodb::coll::Collection {
    let client = Client::connect(MONGO_HOST, MONGO_PORT)
        .expect("Failed to initialize client.");
    return client.db(database).collection(collection);
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
        Ok(_) => (insert_pet_in_db(pet)),
        Err(_e) => (
            bad_request()
        )
    }
}

fn insert_pet_in_db(pet: Json<Pet>) -> JsonValue {
    // TODO check by id if pet exists
    // TODO split tags

    let coll = get_collection("store", "pet");
    coll.insert_one(doc!{
        "id": pet.id,
        "category": {
            "id": pet.category.id,
            "name": &pet.category.name
        },
        "name": &pet.name,
        "tags": [{
                "id": 0,
                "name": "string"
            }],
        "status": &pet.status
        }, None).ok().expect("Failed to insert pet.");
    let response = json!({
        "message": format!("Pet {} created successfully.", pet.name)
    });
    return response;
}

#[put("/pet", format = "json", data = "<pet>")]
fn update_pet(pet: Json<Pet>) -> JsonValue {
    match pet.validate() {
        Ok(_) => (
            json!({
                "message": format!("Pet {} updated successfully.", pet.name)
            })
        ),
        Err(_e) => (
            bad_request()
        )
    }
}

#[get("/pet/<pet_id>")]
fn get_pet_by_id(pet_id: i32) -> JsonValue {
    let coll = get_collection("store", "pet");
    let cursor = coll.find(Some(doc!{"id": pet_id}), None).unwrap();
    if cursor.count() > 0 {
        let cursor = coll.find(Some(doc!{"id": pet_id}), None).unwrap();
        let mut bundle: Vec<String> = Vec::new();
        for result in cursor {
            if let Ok(item) = result {
                if let Some(&Bson::String(ref name)) = item.get("name") {
                    bundle.push(name.to_string())
                }
            }
        }
        return json!({
            "message": format!("Pet {} requested successfully.", pet_id),
            "data": bundle
        });
    } else {
        return json!({
            "message": format!("Pet {} not found.", pet_id)
        });
    }
}

#[delete("/pet/<pet_id>")]
fn delete_pet_by_id(pet_id: i32) -> JsonValue {
    let coll = get_collection("store", "pet");
    let cursor = coll.find(Some(doc!{"id": pet_id}), None).unwrap();
    if cursor.count() > 0 {
        coll.delete_one(doc!{"id": pet_id}, None).unwrap();
        return json!({
            "message": format!("Pet {} deleted successfully.", pet_id)
        });
    } else {
        return json!({
            "message": format!("Pet {} not found.", pet_id)
        });
    }
}

#[derive(FromForm, Validate)]
struct EditPetData {
    #[validate(length(min = "1"))]
    name: String,
    #[validate(length(min = "1"))]
    status: String,
}

#[post("/pet/<pet_id>", data = "<edit_pet_data>")]
fn edit_pet_by_id(pet_id: i32, edit_pet_data: Form<EditPetData>) -> JsonValue {
    match edit_pet_data.validate() {
        Ok(_) => (
            json!({
                "message": format!("Pet {} edited successfully.", pet_id)
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