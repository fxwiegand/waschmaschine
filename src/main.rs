mod checkouts;

use crate::checkouts::Throw;

#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket::State;
use std::collections::HashMap;
use std::str::FromStr;

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
    let checkouts: HashMap<u16, Throw> = include_str!("../checkouts.txt")
        .lines()
        .map(|l| l.trim().split_once(' ').unwrap())
        .map(|(score, throw)| {
            (
                u16::from_str(score).unwrap(),
                Throw::from_str(throw).unwrap(),
            )
        })
        .collect();
    rocket::build()
        .mount("/", routes![checkout])
        .manage(checkouts)
}
