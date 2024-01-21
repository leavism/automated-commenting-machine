use std::slice::SliceIndex;

use anyhow::{Context, Result};
use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestUserMessageArgs, ChatCompletionResponseMessage,
    CreateChatCompletionRequestArgs,
};

use regex::Regex;
use reqwest::Client;
use serde::Deserialize;

use crate::app_config::config::Config;

/// Stores a single summary message candidate generated by the model
#[derive(Deserialize)]

struct CommitMessageCandidate {
    message: ChatCompletionResponseMessage,
}

/// Stores all the commit message candidates generated by the model
#[derive(Deserialize)]
struct CommitMessageCandidates {
    choices: Vec<CommitMessageCandidate>,
}
pub async fn generate_readme_summary(
    http_client: &Client,
    config: &Config,
    file_text_list: Vec<String>,
) -> Result<String> {
    let mut chat_completion_request_system_message_args_list: Vec<ChatCompletionRequestMessage> =
        Vec::new();

      
       
        
    for file_text in file_text_list {
        chat_completion_request_system_message_args_list.push(
            ChatCompletionRequestUserMessageArgs::default()
                .content(file_text)
                .build()?
                .into(),
        )
    }

    chat_completion_request_system_message_args_list.push(
        ChatCompletionRequestSystemMessageArgs::default()
            .content("turn this into a README and give me the output.")
            .build()?
            .into(),
    );
    
   

   
   
   


    let max_chars: u16 = 2408;
    let payload = CreateChatCompletionRequestArgs::default()
        .max_tokens(max_chars)
        .model("NousResearch/Nous-Hermes-2-Mixtral-8x7B-SFT")
        .messages(chat_completion_request_system_message_args_list)
        .build()
        .context("Failed to construct the request payload")?;

        let response = http_client
        .post(format!("{}/chat/completions", &config.git_api_base_url))
        .bearer_auth(&config.api_key)
        .json(&payload)
        .send()
        .await
        .context("Failed to send the request to the API provider")?
        .error_for_status()?
        .json::<CommitMessageCandidates>()
        .await
        .context("Failed to parse the response from the API provider")?;

        let commit_message = response
        .choices
        .first() // Only the first generated commit message is used
        .context("No commit messages generated")?
        .message
        .content
        .as_ref()
        .context("No commit messages generated")?;

    println!("{}", commit_message);

    Ok(commit_message.to_string())
}
