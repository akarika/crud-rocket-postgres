#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

//for use macro #[derive(Serialize, Deserialize)]
// i do import this in main , for globally utilisation
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;

use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig, UrlObject};
use rocket_okapi::{routes_with_openapi};
pub mod models;
pub mod crud;
pub mod  db;
pub mod schema;
pub mod errors;
pub mod fairings;

use crud::crud_hero as api;
use rocket::fairing::Fairing;

fn main() {

    rocket::ignite()
        .manage(db::connect())
        .mount("/hero", routes_with_openapi![
        api::create,
        api::update,
        api::delete,
        api::show
        ])
        .mount("/heroes", routes_with_openapi![
        api::read
        ])
        .mount("/swagger", make_swagger_ui(
            &SwaggerUIConfig {
                url: Some("/hero/openapi.json".to_owned()),
                urls: Some(vec![
                    UrlObject::new("Hero", "/hero/openapi.json"),
                    UrlObject::new("Heroes", "/heroes/openapi.json")
                ]),
            }
        ))
       // .register(catchers![api::not_found])
        .attach(fairings::post_request::PostFairing::default())
        .launch();
}