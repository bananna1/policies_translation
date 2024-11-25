use std::fs;
use crate::policy_extraction::policy::Policy;

pub fn parse_policies_json(file_path: &String) -> Result<Vec<String>, String> {
    let text = fs::read_to_string(file_path)
        .expect("Could not read file.");
    // parse text in lines
    let lines = text.split("\n");
    let mut policies: Vec<String> = Vec::new();

    // iterate all lines, except the first one which is blank
    for line in lines.skip(1) {
        //println!("{}", line);
        let policy: Policy;
        // Extract json from line
        match serde_json::from_str(line) {
            Ok(p) => {
                policy = p;
            }
            Err(e) => return Err(String::from("Could not extract json from string"))
        }

        let policy = format!("{}. {}", policy.context, policy.body);
        policies.push(policy);
    }
    Ok(policies)
}