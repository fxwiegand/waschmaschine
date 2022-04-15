mod checkouts;

use crate::checkouts::{get_checkouts, Throw};

#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket::State;
use std::collections::HashMap;

#[get("/checkout/<score>")]
fn checkout(score: u16, state: &State<HashMap<u16, Throw>>) -> Json<Option<&Throw>> {
    if let Some(checkout) = state.get(&score) {
        Json(Some(checkout))
    } else {
        Json(None)
    }
}

#[launch]
fn rocket() -> _ {
    let checkouts = get_checkouts();
    rocket::build()
        .mount("/", routes![checkout])
        .manage(checkouts)
}
