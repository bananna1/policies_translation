mod parse_input_text;
mod policy;
use std::fs;

fn main() {

    let file_path = String::from("/home/bananna/Documents/GitHub/policies_translation/src/policies.txt");
    let output_path = String::from("/home/bananna/Documents/GitHub/policies_translation/src/policies_json.txt");
    let policies = parse_input_text::parse_input_text(&file_path);
    let mut  policies_string = String::new();
    for policy in &policies {
        let policy_json = serde_json::to_string_pretty(&policy).expect("Failed to serialize policy to json.");
        println!("{}\n\n", policy_json);
        policies_string = format!("{}\n\n{}", policies_string, policy_json);
    }
    fs::write(output_path, policies_string).expect("Unable to write to file");

}
