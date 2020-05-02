use diesel::{QueryDsl};
use diesel::{PgConnection, RunQueryDsl};
use crate::schema::heroes::dsl::*;
use crate::schema::heroes;
use rocket_okapi::{JsonSchema};

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset, JsonSchema,Debug)]
#[table_name = "heroes"]
pub struct Hero {
    pub id: Option<i32>,
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32,
}



impl Hero {
    pub fn create(hero: Hero, connection: &PgConnection) -> Result<Hero, diesel::result::Error> {
        diesel::insert_into(heroes)
            .values(&hero)
            .get_result(connection)

    }

    pub fn read(connection: &PgConnection) -> Result<Vec<Hero> , diesel::result::Error>{
        heroes
            .limit(5)
            .load::<Hero>(connection)

    }

    pub fn update(i: i32, hero: Hero, connection: &PgConnection) -> Result<usize, diesel::result::Error> {
        diesel::update(heroes.find(i))
            .set(hero)
            .execute(connection)
    }

    pub fn delete(i: i32, connection: &PgConnection) -> Result<usize, diesel::result::Error> {
        diesel::delete(heroes.find(i)).execute(connection)
    }

    pub fn show(i: i32, connection: &PgConnection) ->  Result<Hero, diesel::result::Error> {
        heroes.find(i).first(connection)
    }

}