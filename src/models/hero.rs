use diesel::{QueryDsl, ExpressionMethods};
use diesel::{PgConnection, RunQueryDsl};
use crate::schema::heroes::dsl::*;
use crate::schema::heroes;
use rocket_okapi::{JsonSchema};
use rocket::response::Responder;
use rocket::{response, Request, Response};

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset, JsonSchema,Debug)]
#[table_name = "heroes"]
pub struct Hero {
    pub id: Option<i32>,
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32,
}



/*
impl<'r> Responder<'r> for Hero {

    fn respond_to(self, _req: &Request) -> response::Result<'r> {
        Response::build()
            .raw_header("X-Person-Name", self.name)
            .raw_header("X-Person-Age", self.age.to_string())
            .header(ContentType::new("application", "application/json"))
            .ok()
    }
}



*/



impl Hero {
    pub fn create(hero: Hero, connection: &PgConnection) -> Hero {
        diesel::insert_into(heroes)
            .values(&hero)
            .get_result(connection)
            .expect("Error creating new hero")
    }

    pub fn read(connection: &PgConnection) -> Result<Vec<Hero> , diesel::result::Error>{
        heroes
            .limit(5)
            .load::<Hero>(connection)

    }

    pub fn update(i: i32, hero: Hero, connection: &PgConnection) -> bool {
        diesel::update(heroes.find(i))
            .set(hero)
            .execute(connection).is_ok()
    }

    pub fn delete(i: i32, connection: &PgConnection) -> bool {
        diesel::delete(heroes.find(i)).execute(connection).is_ok()
    }

    pub fn show(i: i32, connection: &PgConnection) ->  Result<Hero, diesel::result::Error> {
        heroes.find(i).first(connection)
    }

}