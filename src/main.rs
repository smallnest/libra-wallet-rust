#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod routes;

use rocket::config::{Config, Environment, LoggingLevel};
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

fn main() {
    let config = Config::build(Environment::Staging)
        .environment(Environment::Production)
        .address("0.0.0.0")
        .port(8080)
        .log_level(LoggingLevel::Critical)
        .finalize()
        .unwrap();

    rocket::custom(config)
        .attach(Template::fairing())
        .mount("/css", StaticFiles::from("./assets/css"))
        .mount("/js", StaticFiles::from("./assets/js"))
        .mount("/images", StaticFiles::from("./assets/images"))
        .mount(
            "/",
            routes![
                routes::index,
                routes::balance,
                routes::events,
                routes::transfer,
                routes::transfer_libra,
                routes::mint,
                routes::mint_libra
            ],
        )
        .launch();
}
