use serde::{Serialize, Deserialize};
use reqwest::Client;
use std::env;
//mod policy_language_choice;

use crate::policy_translation::policy_language_choice::choose_policy_language;

#[derive(Serialize)]
struct ChatGPTRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize, Debug)]
struct ChatGPTResponse {
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

    let api_key = env::var("OPENAI_API_KEY").expect("OpenAI API key not found.");

    let client = Client::new();

    let mut response_contents = Vec::new();

    let mut message_incipit;
    match choose_policy_language::choose_policy_language(policy_language) {
        Ok(s) => {
            message_incipit = s;
        }
        Err(e) => return Err(Box::from(e))
    }
    let mut i = 0;
    for policy_body in policies_vec {
        i += 1;
        println!("i: {i}");
        let url = String::from("https://api.openai.com/v1/chat/completions");
        let mut messages: Vec<Message> = Vec::new();
        let msg_body = format!("{}\n\n{}", message_incipit, policy_body);
        //println!("{msg_body}\n\n");
        let msg = Message {
            role: String::from("user"),
            content: msg_body
        };

        messages.push(msg);

        let chatgpt_request = ChatGPTRequest {
            model: String::from("gpt-4o"),
            messages
        };
        let response = client
            .post(url)
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&chatgpt_request)
            .send()
            .await?
            .json::<ChatGPTResponse>()
            .await?;

        for choice in response.choices {
            //println!("{:?}", choice);
            //println!("ChatGPT response:\n{}", choice.message.content);
            response_contents.push(choice.message.content);
        }
    }
    //println!("{:?}", messages);

    Ok(response_contents)
}

