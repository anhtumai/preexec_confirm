use std::io;
use std::io::Write;

use std::env;
use std::io::stdin;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use colored::Colorize;

use serde::{Deserialize, Serialize};

use regex::Regex;

const SKIP_CONFIRM_VAR_CHAR: &str = "SKIP_CONFIRM";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Rule {
    contain: String,
    description: Option<String>,
    regex: Option<bool>,
}

fn get_violated_rule<'a>(rules: &'a Vec<Rule>, command: &String) -> Option<&'a Rule> {
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

fn print_rule(&rule: &&Rule) {
    let contain_type = if rule.regex == Some(true) {
        "regex pattern"
    } else {
        "text"
    };
    println!(
        "Your command contains {} {}",
        contain_type,
        rule.contain.yellow().underline()
    );

    match &(rule.description) {
        Some(description) => {
            println!("{}: {}", format!("Description").green(), description);
        }
        None => (),
    };
}

fn verify() {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(5)
        .map(char::from)
        .collect();
    while true {
        print!(
            "Press Ctrl-C to cancel or type {} to continue: ",
            rand_string.red()
        );
        io::stdout().flush().unwrap();
        let mut user_input = String::new();
        stdin()
            .read_line(&mut user_input)
            .ok()
            .expect("Failed to read line");
        if user_input.trim().eq(&rand_string) {
            break;
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match env::var(SKIP_CONFIRM_VAR_CHAR) {
        Ok(v) => {
            if v == "true" {
                return Ok(());
            }
        }
        Err(_) => (),
    }

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Ok(());
    };

    let config_path = args.get(1).unwrap().to_owned();
    let command = args.get(2).unwrap().to_owned();

    let file = std::fs::File::open(config_path)?;
    let rules: Vec<Rule> = serde_yaml::from_reader(file)?;

    let violated_rule_option = get_violated_rule(&rules, &command);
    match violated_rule_option {
        Some(violated_rule) => {
            print_rule(&violated_rule);
            verify();
        }
        None => (),
    }
    Ok(())
}
