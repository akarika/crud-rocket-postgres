use std::io::Cursor;
use std::sync::atomic::{AtomicUsize, Ordering};

use rocket::{Request, Data, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Method, ContentType, Status};


#[derive(Default)]
pub struct PostFairing {}

impl Fairing for PostFairing {
    fn info(&self) -> Info {
        Info {
            name: "POST Rewriter",
            kind: Kind::Response,
        }
    }


    fn on_response(&self, request: &Request, response: &mut Response) {
        println!("equest.method() {}", request.method());
        if let Status::InternalServerError = response.status() {
            return;
        }
        match request.method() {
            Method::Post => {
                response.set_status(Status::Created);
            }
            _ => {}
        };
    }
}