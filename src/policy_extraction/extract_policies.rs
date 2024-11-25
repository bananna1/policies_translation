use std::fs;
use crate::policy_extraction::parse_input_text;

pub fn extract_policies() -> Result<(), String> {
    let input_file_path = String::from("/home/bananna/Documents/GitHub/policies_translation/data/policies.txt");
    let output_file_path = String::from("/home/bananna/Documents/GitHub/policies_translation/output/policies_json.txt");
    let output_pretty_file_path = String::from("/home/bananna/Documents/GitHub/policies_translation/output/pretty_policies_json.txt");
    let mut policies = Vec::new();
    match parse_input_text::parse_input_text(&input_file_path) {
        Ok(p) => {
            policies = p;
        }
        Err(s) => println!("{}", s)
    }
    let mut policies_string = String::new();
    let mut policies_string_pretty = String::new();
    for policy in &policies {
        let policy_json = serde_json::to_string(&policy).expect("Failed to serialize policy to json.");
        let policy_json_pretty = serde_json::to_string_pretty(&policy).expect("Failed to serialize policy to json.");
        //println!("{}\n\n", policy_json);
        policies_string = format!("{}\n{}", policies_string, policy_json);
        policies_string_pretty = format!("{}\n\n{}", policies_string_pretty, policy_json_pretty);
    }
    fs::write(output_file_path, policies_string).expect("Unable to write to file");
    fs::write(output_pretty_file_path, policies_string_pretty).expect("Unable to write to file");

    Ok(())
}