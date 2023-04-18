use dotenv::dotenv;
use std::env;

use crate::models::user_model::User;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::InsertOneResult,
    sync::{Client, Collection},
};

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("DB_URI") {
            Ok(v) => String::from(v),
            Err(_) => format!("Error loading env variable"),
        };

        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("monrust");
        let col: Collection<User> = db.collection("User");
        Self { col }
    }

    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            title: new_user.title,
        };

        let user = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating user");
        Ok(user)
    }

    pub fn get_user(&self, user_id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(user_id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting user");
        Ok(user_detail.unwrap())
    }
}
