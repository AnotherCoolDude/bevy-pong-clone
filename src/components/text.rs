use std::fmt;

pub struct TextContainer;
pub struct FPSText;

pub struct ScoreText {
    pub score_left: u16,
    pub score_right: u16,
}

impl Default for ScoreText {
    fn default() -> Self {
        Self {
            score_left: 0,
            score_right: 0,
        }
    }
}

impl fmt::Display for ScoreText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {}", self.score_left, self.score_right)
    }
}