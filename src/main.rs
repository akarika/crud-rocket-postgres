#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

//for use macro #[derive(Serialize, Deserialize)]
// i do import this in main , for globally utilisation
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

pub mod models;
pub mod crud;
pub mod  db;
pub mod schema;

use crud::crud_hero as api;


#[get("/<id>/<shop>")]
fn index(id:u8,shop: String) -> String {
    format!("id: {} , shoppping {}",id,shop)
}

fn main() {

    rocket::ignite()
        .manage(db::connect())
        .mount("/hero", routes![
        api::create,
        api::update,
        api::delete
        ])
        .mount("/heroes", routes![
        crud::crud_hero::read
        ])
       /* .register(
            catchers![api::not_found]
        )*/
        .launch();
}