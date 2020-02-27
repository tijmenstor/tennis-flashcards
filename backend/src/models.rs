use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Questions {
    pub multiple_choice: Vec<MultipleChoice>,
    pub short_answer: Vec<ShortAnswer>,
    pub true_or_false: Vec<TrueOrFalse>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MultipleChoice {
    question: String,
    choices: Vec<String>,
    answer: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ShortAnswer {
    question: String,
    answer: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TrueOrFalse {
    question: String,
    answer: bool
}