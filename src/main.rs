/*
 * Run git diff and pass the output to an ollama model to generate a commit message
 */

use {
    std::process::Command,
    std::str,
};

fn main() {
    let prompt = "Generate a commit message based on the provided git diff. The generated text will be used directly as the commit message, so please provide a concise and informative message that exclusively focuses on summarizing the changes. Use a format like 'Fix: [Description]', 'Feature: [Description]', or 'Refactor: [Description]' to describe the purpose of the changes. The commit message should contain no extraneous punctuation, discussion, or preamble—just a direct and clear summary of the modifications. You may use bullet points or relevant details from the diff to create the commit message.".to_string();
    let model = "orca-mini".to_string();
    let diff = get_diff();
    let commit_message = get_commit_message(prompt, model, diff);
    println!("{}", commit_message);
}

// Run git diff and return the output as a string
fn get_diff() -> String {
    let output = Command::new("git")
        .arg("diff")
        .output()
        .expect("failed to execute process");

    let diff = str::from_utf8(&output.stdout)
        .expect("failed to convert output to string");

    return diff.to_string();
}

// Run ollama with the provided prompt, model, and diff and return the output as a string
fn get_commit_message(prompt: String, model: String, diff: String) -> String {
    let output = Command::new("ollama")
        .arg("run")
        .arg(model)
        .arg(prompt)
        .arg(diff)
        .output()
        .expect("failed to execute process");

    let commit_message = str::from_utf8(&output.stdout)
        .expect("failed to convert output to string");

    return commit_message.to_string();
}
