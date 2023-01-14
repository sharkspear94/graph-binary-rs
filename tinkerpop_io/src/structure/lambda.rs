use std::fmt::Display;

use crate::conversion;

#[derive(Debug, PartialEq, Clone, Hash, Eq, PartialOrd, Ord)]
pub struct Lambda {
    pub language: String,
    pub script: String,
    pub arguments_length: i32,
}

impl Lambda {
    #[must_use]
    pub fn new(script: &str) -> Self {
        Lambda {
            language: "gremlin-groovy".to_string(),
            script: script.to_string(),
            arguments_length: 1,
        }
    }
}

impl Display for Lambda {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}),({}),args_len:{}",
            self.language, self.script, self.arguments_length
        )
    }
}

conversion!(Lambda, Lambda);
