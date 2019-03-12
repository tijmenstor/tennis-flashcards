#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::http::RawStr;

/// Returns

#[get("/<id>")]
fn get_tennis_flashcard(id: &RawStr) -> String {
    format!("Your sent ID: {}", id.as_str())
}

fn main() {
    rocket::ignite().mount("/", routes![get_tennis_flashcard]).launch();
}
