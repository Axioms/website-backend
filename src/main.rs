#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

#[macro_use]
mod api;
mod utils;
mod db;
use rocket::State;
use std::sync::{Arc, Mutex};
use std::thread;

#[get("/")]
fn index(settings: State<utils::SettingsMutex>) -> &'static str {
    let settings = settings.lock().unwrap();
    if settings.settings.is_enabled {
        "true"
    }
    else {
        "false"
    }
}

fn init_settings(settings: utils::SettingsMutex) {
    thread::spawn(move || {
        utils::settings::detect_change(settings);
    });
}

fn main() {
    let settings = Arc::new(Mutex::new(utils::settings::SettingsWrapper::defualt()));
    init_settings(settings.clone());

    rocket::ignite().mount("/", routes![index])
        .manage(db::init_pool())
        .manage(settings)
        .launch();
} 