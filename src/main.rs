#![feature(proc_macro_hygiene, decl_macro)]
extern crate serde_json;

#[macro_use] extern crate rocket;

mod models;

use models::{Questions, MultipleChoice, ShortAnswer, TrueOrFalse};

use rand::Rng;
use rocket_contrib::json::Json;
use std::fs::File;
use std::io::Read;

// Remove all unwraps at some point to not have any panics!
// https://stackoverflow.com/questions/36362020/what-is-unwrap-in-rust-and-what-is-it-used-for

// Base route (4fun)
#[get("/")]
fn index() -> &'static str {
    "Gr8 API m8!"
}

// Multiple-choice routes
#[get("/<question_id>")]
fn get_mc_question_by_id(question_id: usize) -> Json<MultipleChoice> {
    let questions: Questions = load_json();
    let current_question = questions.multiple_choice[question_id].clone();
    Json(current_question)
}

#[get("/all")]
fn get_all_mc_questions() -> Json<Vec<MultipleChoice>> {
    let questions: Questions = load_json();
    let all_multiple_choice = questions.multiple_choice.clone();
    Json(all_multiple_choice)
}

// Short-answer routes
#[get("/<question_id>")]
fn get_sa_question_by_id(question_id: usize) -> Json<ShortAnswer> {
    let questions: Questions = load_json();
    let current_question = questions.short_answer[question_id].clone();
    Json(current_question)
}

#[get("/all")]
fn get_all_sa_questions() -> Json<Vec<ShortAnswer>> {
    let questions: Questions = load_json();
    let all_short_answer = questions.short_answer.clone();
    Json(all_short_answer)
}

// True-or-false routes
#[get("/<question_id>")]
fn get_tf_question_by_id(question_id: usize) -> Json<TrueOrFalse> {
    let questions: Questions = load_json();
    let current_question = questions.true_or_false[question_id].clone();
    Json(current_question)
}

#[get("/all")]
fn get_all_tf_questions() -> Json<Vec<TrueOrFalse>> {
    let questions: Questions = load_json();
    let all_true_or_false = questions.true_or_false.clone();
    Json(all_true_or_false)
}

fn load_json() -> Questions {
    let mut file = File::open("./questions.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    return serde_json::from_str(&data).expect("JSON was wrongly formatted!");
}

fn main() {
    rocket::ignite().mount("/", routes![index])
        .mount("/api/questions/multiple-choice", routes![get_mc_question_by_id, get_all_mc_questions])
        .mount("/api/questions/short-answer", routes![get_sa_question_by_id, get_all_sa_questions])
        .mount("/api/questions/true-or-false", routes![get_tf_question_by_id, get_all_tf_questions])
        .launch();
}
