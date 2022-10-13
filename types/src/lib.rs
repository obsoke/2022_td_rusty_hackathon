use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Flashcard {
    id: i32,
    question: String,
    answer: String,
    category: i32,
}

impl Flashcard {
    pub fn new(question: &str, answer: &str) -> Self {
        Self {
            id: 1,
            question: question.to_owned(),
            answer: answer.to_owned(),
            category: 0,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn question(&self) -> &str {
        &self.question
    }

    pub fn answer(&self) -> &str {
        &self.answer
    }
}

#[derive(Serialize, sqlx::FromRow)]
pub struct Category {
    id: i32,
    name: String,
}

impl Category {
    pub fn new(name: &str) -> Self {
        Self {
            id: 0,
            name: name.to_owned(),
        }
    }
}
