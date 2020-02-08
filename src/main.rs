#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate serde_json;

use std::fs::File;
use std::io::Read;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

// Remove all unwraps at some point to not have any panics!
// https://stackoverflow.com/questions/36362020/what-is-unwrap-in-rust-and-what-is-it-used-for
#[get("/question/<question_id>")]
fn get_question_by_id(question_id: usize) -> String {
    let questions_json = load_json();
    let question = &questions_json[question_id]["question"];
    let question_type = &questions_json[question_id]["question_type"];
    format!("Your question type is: {} and the question is: {}", question_type, question)
}

fn load_json() -> serde_json::Value {
    let mut file = File::open("./questions.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    return serde_json::from_str(&data).expect("JSON was wrongly formatted!");
}

fn main() {
    rocket::ignite().mount("/", routes![index, get_question_by_id]).launch();
}
