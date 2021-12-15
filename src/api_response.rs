// The purpose of this is to be able to respond with the Json<T> Guard provided by Rocket.rs
// But also respond with arbitrary JSON in the case of an error or something else.

use rocket::serde::json;
use rocket::serde::json::Json;
use rocket::http::{ContentType, Status};
use rocket::{Request, Response, response};

use crate::api_response::JsonObject::{ArbitraryJson, GuardJson};
use crate::Serialize;

pub enum JsonObject<T: Serialize> {
    ArbitraryJson(json::Value),
    GuardJson(Json<T>),
}

impl<'r, 'o: 'r, T: Serialize> response::Responder<'r, 'o> for JsonObject<T> {
    fn respond_to(self, req: &'r Request) -> response::Result<'o> {
        let build_from = match self {
            ArbitraryJson(a) => {
                a.respond_to(&req).unwrap()
            }
            GuardJson(a) => {
                a.respond_to(&req).unwrap()
            }
        };

        Response::build_from(build_from)
            .header(ContentType::JSON)
            .ok()
    }
}

pub struct ApiResponse<T: Serialize> {
    pub json: JsonObject<T>,
    pub status: Status,
}

impl<'r, 'o: 'r, T: Serialize> response::Responder<'r, 'o> for ApiResponse<T> {
    fn respond_to(self, req: &'r Request) -> response::Result<'o> {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}