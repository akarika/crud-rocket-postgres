#![feature(plugin)]


use crate::models::hero::Hero;
use rocket_contrib::json::{Json, JsonValue};



#[post("/", data = "<hero>")]
pub fn create(hero: Json<Hero>) -> Json<Hero> {
    hero
}

#[get("/")]
pub fn read() -> JsonValue {
    json!({
         "id": 83,
        "values": [1, 2, 3, 4]
     })
}

#[put("/<id>", data = "<hero>")]
pub fn update(id: i32, hero: Json<Hero>) -> Json<Hero> {
    hero
}

#[delete("/<id>")]
pub fn delete(id: i32) -> JsonValue {
let a = format!("id delete {}",id);
    json!({
    "status": "ok",
    "delete": a
    })
}

#[catch(404)]
pub fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}
