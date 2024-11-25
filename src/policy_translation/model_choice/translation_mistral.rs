use serde::{Serialize, Deserialize};
use reqwest::Client;
use std::env;

use crate::policy_translation::policy_language_choice::choose_policy_language;

#[derive(Serialize)]
struct MistralRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize, Debug)]
struct MistralResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: MessageResponse,
}

#[derive(Deserialize, Debug)]
struct MessageResponse {
    content: String,
}

#[tokio::main]
pub async fn translate_policy(policies_vec: &Vec<String>, policy_language: &String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut response_contents = Vec::new();
    let mut i = 0;

    for policy_body in policies_vec {
        i += 1;
        println!("i: {i}");
        let api_key = env::var("MISTRAL_API_KEY").expect("Mistral API key not found.");
        let url = "https://api.mistral.ai/v1/chat/completions";
        let model = String::from("mistral-large-latest");

        let mut messages: Vec<Message> = Vec::new();

        let mut message_incipit;
        match choose_policy_language::choose_policy_language(policy_language) {
            Ok(s) => {
                message_incipit = s;
            }
            Err(e) => return Err(Box::from(e))
        }

        let msg_body = format!("{}\n\n{}", message_incipit, policy_body);

        let msg = Message {
            role: String::from("user"),
            content: msg_body
        };

        messages.push(msg);

        let mistral_request = MistralRequest {
            model,
            messages
        };

        let response = client
            .post(url)
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&mistral_request)
            .send()
            .await?
            .json::<MistralResponse>()
            .await?;

        for choice in response.choices {

            response_contents.push(choice.message.content);
        }
    }
    Ok(response_contents)
}