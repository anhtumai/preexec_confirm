use serde::{Deserialize, Serialize};

use regex::Regex;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Rule {
    pub contain: String,
    pub description: Option<String>,
    pub regex: Option<bool>,
}

pub fn get_violated_rule<'a>(rules: &'a Vec<Rule>, command: &String) -> Option<&'a Rule> {
    rules.iter().find(|rule| {
        if rule.regex == Some(true) {
            match Regex::new(&rule.contain) {
                Ok(re) => re.is_match(&command),
                Err(_) => false,
            }
        } else {
            command.contains(&rule.contain)
        }
    })
}
