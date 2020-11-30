#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

pub mod api;
pub mod app;
pub mod logger;
pub mod resolver;

fn main() {
    logger::init();
    app::start();
}
