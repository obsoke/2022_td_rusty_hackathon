use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CardAnswer(String);

impl CardAnswer {
    pub fn parse(s: impl Into<String>) -> Result<CardAnswer, String> {
        let s = s.into();
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.len() > 256;

        if is_empty_or_whitespace || is_too_long {
            Err(format!("{s} is not a valid answer."))
        } else {
            Ok(CardAnswer(s))
        }
    }
}

impl AsRef<str> for CardAnswer {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
