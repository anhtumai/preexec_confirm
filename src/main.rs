use std::io;
use std::io::Write;

use std::env;
use std::io::stdin;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use colored::Colorize;

use serde::{Deserialize, Serialize};

const SKIP_CONFIRM_VAR_CHAR: &str = "SKIP_CONFIRM";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Rule {
    contain: String,
    description: Option<String>,
}

fn get_violated_rule<'a>(rules: &'a Vec<Rule>, command: &String) -> Option<&'a Rule> {
    rules.iter().find(|rule| command.contains(&rule.contain))
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
            println!(
                "Your command contains text {}",
                violated_rule.contain.yellow().underline()
            );
            match &(violated_rule.description) {
                Some(description) => {
                    println!("{}: {}", format!("Description").green(), description);
                }
                None => (),
            };
            verify()
        }
        None => (),
    }
    Ok(())
}
