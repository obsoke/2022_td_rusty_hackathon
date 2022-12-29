mod card_answer;
mod card_question;
mod new_card;

pub use card_answer::CardAnswer;
pub use card_question::CardQuestion;
pub use new_card::NewCardForCategory;

// impl NewCardForCategory {
//     pub fn new(question: impl Into<String>, answer: impl Into<String>) -> NewCardForCategory {
//         NewCardForCategory {
//             question: CardQuestion(question.into()),
//             answer: answer.into(),
//         }
//     }
// }
