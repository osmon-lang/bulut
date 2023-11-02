use std::io::Write;
use crate::package::Package;

pub struct Prompt {
    package: Package
}

#[derive(Debug)]
pub enum PromptError {
    /// Invalid prompt entered
    InvalidPrompt
}

#[derive(Debug, PartialEq)]
pub enum PromptResult {
    /// Text result
    Text(String),

    /// 32bit number result
    Number(i32),
}

impl Prompt {
    pub fn new() -> Self {
        Self {
            package: Package::new(
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string()
            )
        }
    }

    pub fn ask(&self, question: &str, default: PromptResult) -> Result<PromptResult, PromptError> {
        print!("{}: ", question);
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        match input {
            "" => Ok(default),
            _ => {
                match default {
                    PromptResult::Text(_) => Ok(PromptResult::Text(input.to_string())),
                    PromptResult::Number(_) => {
                        match input.parse::<i32>() {
                            Ok(number) => Ok(PromptResult::Number(number)),
                            Err(_) => Err(PromptError::InvalidPrompt)
                        }
                    }
                }
            }
        }
    }

    pub fn ask_package() -> Package {
        let prompt = Prompt::new();

        let name = prompt.ask("Name", PromptResult::Text("".to_string())).unwrap();
        let version = prompt.ask("Version", PromptResult::Text("".to_string())).unwrap();
        let description = prompt.ask("Description", PromptResult::Text("".to_string())).unwrap();
        let std = prompt.ask("Std", PromptResult::Text("".to_string())).unwrap();

        Package::new(
            match name {
                PromptResult::Text(name) => name,
                _ => panic!("Invalid prompt result")
            },
            match version {
                PromptResult::Text(version) => version,
                _ => panic!("Invalid prompt result")
            },
            match description {
                PromptResult::Text(description) => description,
                _ => panic!("Invalid prompt result")
            },
            match std {
                PromptResult::Text(std) => std,
                _ => panic!("Invalid prompt result")
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt() {
        let prompt = Prompt::new();

        let name = prompt.ask("Name", PromptResult::Text("".to_string())).unwrap();
        let version = prompt.ask("Version", PromptResult::Text("".to_string())).unwrap();
        let description = prompt.ask("Description", PromptResult::Text("".to_string())).unwrap();
        let std = prompt.ask("Std", PromptResult::Text("".to_string())).unwrap();

        assert_eq!(name, PromptResult::Text("a".to_string()));
        assert_eq!(version, PromptResult::Text("a".to_string()));
        assert_eq!(description, PromptResult::Text("a".to_string()));
        assert_eq!(std, PromptResult::Text("a".to_string()));
    }
}