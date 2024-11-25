mod policy_extraction;
mod policy_translation;

use crate::policy_extraction::parse_policy_json;
use crate::policy_translation::translate_policies;


fn main() -> Result<(), String> {
    let policies_file_path = String::from("/home/bananna/Documents/GitHub/policies_translation/output/policies_json.txt");
    let policies_body_response = parse_policy_json::parse_policies_json(&policies_file_path);
    let policies_body;
    match policies_body_response {
        Ok(p) => {
            policies_body = p
        }
        Err(e) => return Err(String::from(e))
    }
    let policy_language = String::from("ALFA");
    let models = ["Mistral", "Gemini", "ChatGPT"];
    for m in models {
        let model = String::from(m);
        println!("Model: {}", model);

        let res = translate_policies::translate_policies(&policies_body, &model, &policy_language);
        match res {
            Ok(()) => continue,
            Err(e) => return Err(e)
        }
    }
    Ok(())
}






