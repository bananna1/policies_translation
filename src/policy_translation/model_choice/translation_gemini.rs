use serde::{Serialize, Deserialize};
use reqwest::Client;
use std::env;

use crate::policy_translation::policy_language_choice::choose_policy_language;

#[derive(Debug, Serialize)]
struct RequestBody {
    api_key: String,
    data: String
}

#[derive(Debug, Deserialize)]
struct ResponseBody {
    status: String,
    result: Option<String>,
}

#[tokio::main]
pub async fn translate_policy(policies_vec: &Vec<String>, policy_language: &String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
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
        let api_key = env::var("GEMINI_API_KEY").expect("Gemini API key not found.");
        let url = "https://api.gemini.com/endpoint";

        let msg_body = format!("{}\n\n{}", message_incipit, policy_body);

        let request_body = RequestBody {
            api_key,
            data: msg_body,
        };
        println!("Request body: {:?}", request_body);
        let res = client
            .post(url)


            .json(&request_body)
            .send()
            .await?;

        let response_body: ResponseBody = res.json().await?;
        println!("{:?}", response_body);

        match response_body.result {
            Some(r) => response_contents.push(r),
            None => return Err(Box::from("No results received")),
        }
    }

    Ok(response_contents)
}