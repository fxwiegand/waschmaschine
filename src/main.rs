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

#[cfg(test)]
mod test {
    use super::rocket;
    use crate::checkouts::Dart;
    use crate::checkouts::Region;
    use crate::Throw;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn test_checkout_api() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/checkout/170").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let expected_checkout = Throw {
            darts: vec![
                Dart {
                    field: 20,
                    region: Region::Triple,
                },
                Dart {
                    field: 20,
                    region: Region::Triple,
                },
                Dart {
                    field: 25,
                    region: Region::Double,
                },
            ],
        };
        assert_eq!(expected_checkout, response.into_json::<Throw>().unwrap());
    }

    #[test]
    fn test_checkout_api_bogey_number() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/checkout/168").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(None, response.into_json::<Throw>());
    }
}
