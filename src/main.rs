mod checkouts;

use crate::checkouts::{get_checkouts, Throw};

#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket::{ State};
use std::collections::HashMap;
use rocket::http::{Method};
use rocket_cors::{AllowedHeaders, AllowedOrigins};

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

    let allowed_origins = AllowedOrigins::all();

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
        .to_cors().unwrap();

    rocket::build()
        .mount("/", routes![checkout])
        .manage(checkouts)
        .attach(cors)
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
