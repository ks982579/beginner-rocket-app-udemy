#[macro_use]
extern crate rocket;

use base64::prelude::*;

use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
    response::status::{self, Unauthorized},
    serde::json::{json, Value},
};

// Authentication (should be own module)
pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

// Factory Methods (common in Rust)
impl BasicAuth {
    fn from_authorization_header(header: &str) -> Option<BasicAuth> {
        let split = header.split_whitespace().collect::<Vec<_>>();
        if split.len() != 2 {
            return None;
        }
        if split[0] != "Basic" {
            return None;
        }
        Self::from_base64_encoded(split[1])
    }

    fn from_base64_encoded(base64_string: &str) -> Option<BasicAuth> {
        let decoded = BASE64_STANDARD.decode(base64_string).ok()?;
        let decoded_str = String::from_utf8(decoded).ok()?;
        let split = decoded_str.split(":").collect::<Vec<_>>();

        // If exactly username and password were received
        if split.len() != 2 {
            return None;
        }
        let (username, password) = (split[0].to_string(), split[1].to_string());
        Some(BasicAuth {
            username: username,
            password: password,
        })
    }
}

// Creating a Request Guard
#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");

        if let Some(auth_header) = auth_header {
            if let Some(auth) = Self::from_authorization_header(auth_header) {
                return Outcome::Success(auth);
            }
        }
        Outcome::Error((Status::Unauthorized, ()))
    }
}

#[get("/")]
fn _hello() -> Value {
    json!("Hello, world!")
}

#[get("/rustaceans")]
fn get_rustaceans(auth: BasicAuth) -> Value {
    let fake_data: &[Value] = &[
        json!({ "id": 1, "name": "John Doe" }),
        json!({"id": 2, "name": "Jane Doe"}),
    ];

    json!(fake_data)
}

#[get("/rustaceans/<id>")]
fn view_rustacean(id: i32) -> Value {
    json!({ "id": id, "name": "Kevin", "email": "kevin@crazy.com"})
}

#[post("/rustaceans", format = "json")]
fn create_rustacean() -> Value {
    json!({"id": 3, "name": "Lisa", "email": "lisa@crazy.com"})
}

#[put("/rustaceans/<id>", format = "json")]
fn update_rustacean(id: i32) -> Value {
    json!({ "id": id, "name": "John Doe", "email": "john@crazy.com" })
}

#[delete("/rustaceans/<id>")]
fn delete_rustacean(id: i32) -> status::NoContent {
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
