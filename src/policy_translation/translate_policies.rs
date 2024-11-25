use std::fs;
use crate::policy_translation::model_choice::translation_chatgpt;
use crate::policy_translation::model_choice::translation_mistral;
use crate::policy_translation::model_choice::translation_gemini;

pub fn translate_policies(policies_body: &Vec<String>, model: &String, policy_language: &String) -> Result<(), String> {
    let response_content;

    let translation = match model.as_str() {
        "ChatGPT" => translation_chatgpt::translate_policy(policies_body, policy_language),
        "Mistral" => translation_mistral::translate_policy(policies_body, policy_language),
        "Gemini" => translation_gemini::translate_policy(policies_body, policy_language),
        _ => return Err(String::from("Unrecognized model")),
    };

    match translation {
        Ok(contents) => {
            response_content = contents;
        },
        Err(_) => return Err(String::from("Error"))
    }

    let mut alfa_policies_string = String::new();
    let mut i = 0;
    for content in response_content {
        i += 1;
        //println!("{}\n", content);
        alfa_policies_string = format!("{}\n\n-------------------------------------------------------------------------------------\n\n{}", alfa_policies_string, content);
    }

    println!("Length: {}", i);

    let output_file_path = format!("/home/bananna/Documents/GitHub/policies_translation/output/policies_{}_{}.txt", policy_language, model);

    fs::write(output_file_path, alfa_policies_string).expect("Unable to write to file");

    Ok(())
}