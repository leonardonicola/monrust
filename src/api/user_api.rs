use crate::{models::user_model::User, repository::mongodb_repos::MongoRepo};
use mongodb::{
    bson::oid::ObjectId,
    results::{InsertOneResult},
};
use rocket::{http::Status, serde::json::Json, State};

#[post("/user", data = "<new_user>")]
pub fn create_user(
    db: &State<MongoRepo>,
    new_user: Json<User>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = User {
        id: None,
        name: new_user.name.to_owned(),
        title: new_user.title.to_owned(),
    };

    let user_detail = db.create_user(data);

    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/user/<path>")]
pub fn get_user(db: &State<MongoRepo>, path: String) -> Result<Json<User>, Status> {
    let id = path;

    if id.is_empty() {
        return Err(Status::BadRequest);
    };

    let user_detail = db.get_user(&id);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/user")]
pub fn get_all_users(db: &State<MongoRepo>) -> Result<Json<Vec<User>>, Status> {
    let users = db.get_all_users();
   match users {
        Ok(users) => {
            if users.is_empty() {
                Err(Status::NotFound)
            } else {
                Ok(Json(users))
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/user/<path>", data = "<edited_user>")]
pub fn edit_user(
    db: &State<MongoRepo>,
    path: String,
    edited_user: Json<User>,
) -> Result<Json<User>, Status> {
    let id = path;

    if id.is_empty() {
        return Err(Status::BadRequest);
    };

    let data = User {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        name: edited_user.name.to_owned(),
        title: edited_user.title.to_owned(),
    };

    let update_result = db.edit_user(&id, data);

    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_user_info = db.get_user(&id);
                return match updated_user_info {
                    Ok(user) => Ok(Json(user)),
                    Err(_) => Err(Status::InternalServerError),
                };
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}
