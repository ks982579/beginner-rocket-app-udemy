use base64::prelude::*;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
};

// Authentication (should be own module)
#[derive(Debug, PartialEq, PartialOrd)]
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
                let foo_bar: BasicAuth = BasicAuth {
                    username: "foo".to_string(),
                    password: "bar".to_string(),
                };
                if foo_bar == auth {
                    return Outcome::Success(auth);
                } else {
                    return Outcome::Error((Status::NotFound, ()));
                }
            }
        }
        Outcome::Error((Status::Unauthorized, ()))
    }
}
