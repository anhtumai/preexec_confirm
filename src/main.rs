use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{stdin, stdout, Write};

use colored::Colorize;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

use confirm::{get_violated_rule, Rule};

const SKIP_CONFIRM_VAR_CHAR: &str = "SKIP_CONFIRM";

fn print_rule(rule: &Rule) {
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
        stdout().flush().unwrap();
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

fn main() -> Result<(), Box<dyn Error>> {
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

    let file = File::open(config_path)?;
    let rules: Vec<Rule> = serde_yaml::from_reader(file)?;

    let violated_rule_option = get_violated_rule(&rules, &command);
    match violated_rule_option {
        Some(violated_rule) => {
            print_rule(violated_rule);
            verify();
        }
        None => (),
    }
    Ok(())
}
