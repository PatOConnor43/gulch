#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

pub mod app;
pub mod logger;

fn main() {
    logger::init();
    app::start();
}
