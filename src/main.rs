/*
 * Run git diff and pass the output to an ollama model to generate a commit message
 */

// Ignore unused warnings for now
#[allow(unused_imports)]

use {
    std::process::Command,
    http::{Request, Response},
    std::collections::HashMap,
    reqwest::{Client, Error},
    json::JsonValue,
};


fn diff() -> String {
    let output = Command::new("git")
        .arg("diff")
        .output()
        .expect("Failed to execute git diff");

    let output = String::from_utf8(output.stdout).expect("Failed to parse git diff output");

    output
}

fn get_message() -> String {
    // Make an http request to the ollama server on localhost:11434

    // Format the diff output into a json object
    let diff = diff();
    let prompt = format!("Create a commit message for the following diff:\n{}", diff);

    let mut map = HashMap::new();
    map.insert("model", "llama2");
    map.insert("prompt", &prompt);
    map.insert("stream", "false");

    let client = reqwest::Client::new();

    // Send the request
    let resp = client.post("http://127.0.0.1:11434")
        .json(&map)
        .send();

} 
