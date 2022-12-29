use serde::{Deserialize, Serialize};

use crate::domain::{CardAnswer, CardQuestion};

#[derive(Serialize, Deserialize)]
pub struct NewCardForCategory {
    pub question: CardQuestion,
    pub answer: CardAnswer,
}
