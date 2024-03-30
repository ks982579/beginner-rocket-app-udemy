#[macro_use]
extern crate rocket;

mod auth;

use auth::BasicAuth;
use rocket::{
    response::status::{self, Unauthorized},
    serde::json::{json, Value},
};

#[get("/")]
fn _hello() -> Value {
    json!("Hello, world!")
}

#[get("/rustaceans")]
fn get_rustaceans(_auth: BasicAuth) -> Value {
    let fake_data: &[Value] = &[
        json!({ "id": 1, "name": "John Doe" }),
        json!({"id": 2, "name": "Jane Doe"}),
    ];

    json!(fake_data)
}

#[get("/rustaceans/<id>")]
fn view_rustacean(id: i32, _auth: BasicAuth) -> Value {
    json!({ "id": id, "name": "Kevin", "email": "kevin@crazy.com"})
}

#[post("/rustaceans", format = "json")]
fn create_rustacean(_auth: BasicAuth) -> Value {
    json!({"id": 3, "name": "Lisa", "email": "lisa@crazy.com"})
}

#[put("/rustaceans/<id>", format = "json")]
fn update_rustacean(id: i32, _auth: BasicAuth) -> Value {
    json!({ "id": id, "name": "John Doe", "email": "john@crazy.com" })
}

#[delete("/rustaceans/<id>")]
fn delete_rustacean(id: i32, _auth: BasicAuth) -> status::NoContent {
    status::NoContent
}

// Error Patterns
#[catch(401)]
fn unauthorized() -> Value {
    json!("Unauthorized")
}
#[catch(404)]
fn not_found() -> Value {
    json!("Not Found!")
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                get_rustaceans,
                view_rustacean,
                create_rustacean,
                update_rustacean,
                delete_rustacean,
            ],
        )
        .register("/", catchers![not_found, unauthorized])
        .launch()
        .await;
}
