#[macro_use]
extern crate rocket;

mod errors;
mod fairings;
mod handlers;

// fn rocket() -> rocket::Rocket {
//     rocket::ignite()
//         .attach(fairings::RequestTimer)
//         .register(catchers![errors::not_found, errors::internal_server_error])
//         .mount(
//             "/",
//             routes![
//                 handlers::index,
//                 handlers::json,
//                 handlers::big_json,
//                 handlers::time_now,
//                 handlers::big_json_stream,
//             ],
//         )
// }

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(fairings::RequestTimer)
        .register(
            "/",
            catchers![errors::not_found, errors::internal_server_error],
        )
        .mount("/", routes![handlers::index])
}
