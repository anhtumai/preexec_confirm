use confirm::{get_violated_rule, Rule};

#[test]
fn test_rules_with_no_regex() {
    let rules = vec![
        Rule {
            contain: String::from("prod-environment"),
            description: None,
            regex: None,
        },
        Rule {
            contain: String::from("beta-environment"),
            description: None,
            regex: Some(false),
        },
    ];

    let dev_command = String::from("deploy to dev-environment...");
    assert_eq!(get_violated_rule(&rules, &dev_command), None);

    let prod_command = String::from("deploy to prod-environment...");
    assert_eq!(get_violated_rule(&rules, &prod_command), rules.get(0));

    let beta_command = String::from("deploy to beta-environment...");
    assert_eq!(get_violated_rule(&rules, &beta_command), rules.get(1));
}

#[test]
fn test_rules_with_regex() {
    let rules = vec![Rule {
        contain: String::from("deploy.*prod"),
        description: None,
        regex: Some(true),
    }];

    let violated_command = String::from("this command deploy to prod....");
    assert_eq!(get_violated_rule(&rules, &violated_command), rules.get(0));

    let unviolated_command = String::from("AWS_PROFILE=prod sls deploy");
    assert_eq!(get_violated_rule(&rules, &unviolated_command), None);
}

#[test]
fn rules_with_invalid_regex_are_ignored() {
    let rules = vec![Rule {
        contain: String::from("*"),
        description: None,
        regex: Some(true),
    }];

    let command = String::from("any_command");
    assert_eq!(get_violated_rule(&rules, &command), None);
}
