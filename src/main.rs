#[macro_use]
extern crate rocket;

mod errors;
mod fairings;
mod handlers;

#[launch]
fn rocket() -> _ {
    let routes = routes![
        handlers::index,
        handlers::json,
        handlers::big_json,
        handlers::time_now
    ];
    let catchers = catchers![errors::not_found, errors::internal_server_error];

    rocket::build()
        .attach(fairings::RequestTimer)
        .register("/", catchers)
        .mount("/", routes)
}
