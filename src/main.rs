mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

use api::user_api::{create_user, edit_user, get_all_users, get_user};
use repository::mongodb_repos::MongoRepo;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build().manage(db).mount(
        "/",
        routes![create_user, get_user, edit_user, get_all_users],
    )
}
