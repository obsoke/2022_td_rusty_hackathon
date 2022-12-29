use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CardQuestion(String);

impl CardQuestion {
    pub fn parse(s: impl Into<String>) -> Result<CardQuestion, String> {
        let s = s.into();
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.len() > 256;

        if is_empty_or_whitespace || is_too_long {
            Err(format!("{s} is not a valid question."))
        } else {
            Ok(CardQuestion(s))
        }
    }
}

impl AsRef<str> for CardQuestion {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::CardQuestion;
    use claims::{assert_err, assert_ok};

    #[test]
    fn long_name_is_rejected() {
        let question = "a".repeat(257);
        assert_err!(CardQuestion::parse(question));
    }

    #[test]
    fn whitespace_only_is_rejected() {
        let question = " ";
        assert_err!(CardQuestion::parse(question));
    }

    #[test]
    fn empty_string_is_rejected() {
        let question = "";
        assert_err!(CardQuestion::parse(question));
    }

    #[test]
    fn valid_question_is_parsed_successfully() {
        let question = "What is the capital of Ontario?";
        assert_ok!(CardQuestion::parse(question));
    }
}
