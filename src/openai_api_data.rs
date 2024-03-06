// Structs to handle the data returned by the OpenAI API

use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Serialize, Deserialize)]
pub struct CreateAssistantResponse {
    pub id: String,
    object: String,
    created_at: i64,
    name: String,
    description: Option<String>,
    model: String,
    instructions: String,
    tools: Vec<serde_json::Value>,
    file_ids: Vec<String>,
    metadata: serde_json::Value,
}


#[derive(Serialize, Deserialize)]
pub struct Thread {
    pub id: String,
    object: String,
    created_at: i64,
    metadata: serde_json::Value,
}


#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    role: String,
    object: String,
    created_at: i64,
    thread_id: String,

}


#[derive(Serialize, Deserialize)]
pub struct Runs {
    pub id: String,
    pub status: String,
    object: String,
    created_at: i64,
    model: String,
    assistant_id: String,
    thread_id: String,
   
}


#[derive(Serialize, Deserialize, Debug)]
pub struct MessageListResponse {
    object: String,
    pub data: Vec<Messages>,  
    first_id: String,
    last_id: String,
    has_more: bool,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Messages {
    pub id: String,
    object: String,
    created_at: i64,
    thread_id: String,
    pub role: String,
    pub content: Vec<Content>,
    file_ids: Vec<String>,
    assistant_id: Option<String>,  
    run_id: Option<String>,  
    metadata: HashMap<String, serde_json::Value>,  
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    #[serde(rename = "type")]
    content_type: String,  
    pub text: Text,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Text {
    pub value: String,
    annotations: Vec<serde_json::Value>,  
}



#[derive(Serialize, Deserialize, Debug)]
struct ErrorResponse {
    error: ErrorDetail,
}
#[derive(Serialize, Deserialize, Debug)]
struct ErrorDetail {
    message: String,
    #[serde(rename = "type")]
    error_type: String,
    param: Option<String>,
    code: String,
}

