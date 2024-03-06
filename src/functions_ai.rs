// Function to interact with OpenAI API
// Functions to make a Assistant in OpenAI API
// Create a new assistant, create a new thread, add a message to a thread, run an assistant,
// process messages, wait for the completion of a run, print the messages (user and assistant)

use std::env;
use dotenv::dotenv;
use serde_json::json;
use reqwest::Client;

use crate::openai_api_data::{CreateAssistantResponse, Thread, Message, Runs, MessageListResponse}; 

use crate::functions_other::capitalize_first_letter;

// Function to get the API key from the .env file
pub fn api_key() -> String {
    dotenv().ok();  
    let api_key = env::var("OPENAI_API_KEY").expect("api_key must be set");
    api_key
}

// Function to create a new assistant
pub async fn create_assistant(client:Client, api_key:String) -> Result<CreateAssistantResponse, reqwest::Error>{
    let assistant = client.post("https://api.openai.com/v1/assistants")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("OpenAI-Beta", format!("assistants=v1"))
        .header("Content-Type", "application/json")
        .json(&json!({
            "model": "gpt-3.5-turbo",
            "name": "MiAsistente",
            "instructions": "Este mi asistente personal. Respuestas breves y concisas."
        }))
        .send()
        .await?;

    let res = assistant.json::<CreateAssistantResponse>().await;
    res
}

// Function to create a new thread
pub async fn create_thread(client:Client, api_key:String) -> Result<Thread, reqwest::Error> {
    let thread = client.post("https://api.openai.com/v1/threads")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("OpenAI-Beta", format!("assistants=v1"))
        .header("Content-Type", "application/json")
        .send()
        .await?;

    let res = thread.json::<Thread>().await;
    res
}

// Function to add a message to a thread
pub async fn add_message_to_thread(client:Client, api_key:String, thread_id:String, 
                                   input: String) -> Result<Message, reqwest::Error> {
    let url = format!("https://api.openai.com/v1/threads/{}/messages", thread_id);
    let message = client.post( url.as_str())
        .header("Authorization", format!("Bearer {}", api_key))
        .header("OpenAI-Beta", format!("assistants=v1"))
        .header("Content-Type", "application/json")
        .json(&json!({
            "role": "user",
            "content": input,
        }))
        .send()
        .await?;

    let res = message.json::<Message>().await;
    res
}

// Function to run an assistant
pub async fn run_assistant(client:Client, api_key:String, thread_id:String, assistant_id:String) -> Result<Runs, reqwest::Error> {
    let url = format!("https://api.openai.com/v1/threads/{}/runs", thread_id);
    let runs = client.post( url.as_str())
        .header("Authorization", format!("Bearer {}", api_key))
        .header("OpenAI-Beta", format!("assistants=v1"))
        .header("Content-Type", "application/json")
        .json(&json!({
            "assistant_id": assistant_id,
            "instructions": "Responde como un experto varios temas."
        }))
        .send()
        .await?;

    let res = runs.json::<Runs>().await;
    res
}

// Function to process messages
pub async fn process_messages(client:Client, api_key:String, thread_id:String) -> Result<MessageListResponse, reqwest::Error> {
    let url = format!("https://api.openai.com/v1/threads/{}/messages", thread_id);
    let message_list = client.get( url.as_str())
        .header("Authorization", format!("Bearer {}", api_key))
        .header("OpenAI-Beta", format!("assistants=v1"))
        .header("Content-Type", "application/json")
        .send()
        .await?;

    let res = message_list.json::<MessageListResponse>().await;
    res
} 

// Function to wait for the completion of a run
pub async fn wait_for_completion(client:Client, api_key:String, thread_id:String, 
                                  run_id:String) -> Result<String, reqwest::Error> {
    loop {
        // estera 5  segundos
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        let url = format!("https://api.openai.com/v1/threads/{}/runs/{}", thread_id, run_id);
        let wait_completion = client.get( url.as_str())
            .header("Authorization", format!("Bearer {}", api_key))
            .header("OpenAI-Beta", format!("assistants=v1"))
            .header("Content-Type", "application/json")
            .send()
            .await?;

        let res = wait_completion.json::<Runs>().await;
        let run = res?;
        let status: String = run.status;
        
        if status == "completed" {
            return Ok("completed".to_string());

        } else if status == "requires_action"{
            // if run_status.required_action.type == "submit_tool_outputs":
            //         self.call_required_functions(run_status.required_action.submit_tool_outputs.model_dump())
            return Ok("requires_action".to_string());
        
        }

    }
    

}


// Function to print the messages
pub async fn print_messages (client:Client, api_key:String, thread_id:String,
                             arr_message_id: &mut Vec<String> ) -> Result<(), reqwest::Error>{
    
    // Process the messages and print them
    let message_list = process_messages(client.clone(), api_key.clone(), thread_id.clone()).await?;
    for message in message_list.data.iter().rev() {
        // Check if the message has already been processed
        let id_of_message  = message.id.to_string();
        if arr_message_id.contains(&id_of_message.to_string()) {
            continue;
        }
        arr_message_id.push(id_of_message.to_string());
        println!("{}: {}", capitalize_first_letter(&message.role)  , message.content[0].text.value);
    }
    //println!(" " );

    Ok(())
}