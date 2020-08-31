#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod handlers;
mod fairings;
mod errors;

fn main() {
    rocket::ignite()
        .attach(fairings::RequestTimer)
        .mount("/", routes![
            handlers::index, 
            handlers::json, 
            handlers::big_json, 
            handlers::time_now,
        ])
        .register(
            catchers![
                errors::not_found,
                errors::internal_server_error],
        )
        .launch();
}
