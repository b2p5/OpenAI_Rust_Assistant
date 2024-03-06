// Script of how to use the OpenAI API in Rust
// This Script shows how to use the OpenAI API in Rust to create an assistant, 
// start a conversation, and get a response from the assistant. 
// The Script uses the reqwest crate to make HTTP requests to the OpenAI API. 
// The Script also uses the dotenv crate to load the API key from a .env file.

use reqwest::Client;
use std::io::{self, Write};

mod openai_api_data;
mod functions_ai;
use crate::functions_ai::{api_key, create_assistant, create_thread, 
                          add_message_to_thread, run_assistant, 
                          wait_for_completion, print_messages
                         };
mod functions_other;

/// Main function that demonstrates how to use the OpenAI API in Rust.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // Load environment variables from the .env file
    let api_key = api_key();

    // Create a reqwest client to make HTTP requests
    let client = Client::new();
    
    // Array of processed messages
    let mut arr_message_id: Vec<String> = Vec::new();

    // Create a new assistant
    let assistant = create_assistant(client.clone(), api_key.clone()).await?;
    let assistant_id = assistant.id;
    
    // Create a new conversation thread
    let thread = create_thread(client.clone(), api_key.clone()).await?;
    let thread_id = thread.id;  

    println!("\n\nBienvenido al asistente de OpenAI en Rust");
    println!("Escribe tu pregunta a continuación del símbolo >>");
    println!("Presione Enter para iniciar la conversación. \"Exit\" para salir\n");

    loop {
        
        // Input from the user to start the conversation
        print!(">> ");

        io::stdout().flush().unwrap(); // Fuerza a vaciar el búfer de salida para mostrar el texto inmediatamente
        let mut question = String::new();
        io::stdin().read_line(&mut question).unwrap();

        // If the user types "Exit", the conversation ends
        if question.trim().to_lowercase() == "exit" {
            return Ok(());
        }

        // Send a question to the assistant
        let message = add_message_to_thread(client.clone(), api_key.clone(), 
                                                     thread_id.clone(),question).await?;
        let _message_id = message.id;

        print_messages (client.clone(), api_key.clone(), thread_id.clone(), 
        &mut arr_message_id).await?; 

        // Run the assistant
        let runs = run_assistant(client.clone(), api_key.clone(), 
                                       thread_id.clone(), assistant_id.clone()).await?;
        let run_id = runs.id;

        // Wait for the assistant to complete processing
        let result = wait_for_completion(client.clone(), api_key.clone(), 
                                                 thread_id.clone(), run_id.clone()).await?;
        if result == "completed" {
            // Process the messages and print them
            print_messages (client.clone(), api_key.clone(), thread_id.clone(), 
                            &mut arr_message_id).await?; 
            // Print emoticon
            println!("😀");
            println!("");

        } else if result == "req_action" {
            println!("Ejecución requiere acción" );
        } else {
            println!("Error desconocido");
        }
    }


}

