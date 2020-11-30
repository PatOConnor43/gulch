#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

pub mod api;
pub mod app;
pub mod error;
pub mod logger;
pub mod resolver;
pub mod youtube;

#[launch]
fn rocket() -> rocket::Rocket {
    logger::init();
    app::start()
}
