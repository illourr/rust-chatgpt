use reqwest::header::{HeaderValue, CONTENT_TYPE};
// use reqwest::header::AUTHORIZATION;

use serde::Deserialize;
// use std::collections::HashMap;
use std::error::Error;
use std::io;

const CHAT_ENDPOINT: &str = "https://api.openai.com/v1/chat/completions";

#[derive(Deserialize)]
struct Message {
    content: String,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Deserialize)]
struct ResponseJson {
    choices: Vec<Choice>,
}

fn main() -> Result<(), Box<dyn Error>> {
    // "start a process" to take user input

    println!("Please enter your ChatGPT API Key: ");

    // TODO: Save api_key as an environment variable.
    // Skip prompting for the api_key when its already saved.
    let mut api_key = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut api_key)?;

    println!("{}", api_key);

    // Prompt user to start a chat.
    println!("Start a chat by typing: ");
    let mut prompt = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut prompt)?;

    // This didn't work! Why NOT? (its kinda cool though)
    // #[derive(Serialize, Deserialize, Debug)]
    // enum MapValue<'a> {
    //     String(&'a str),
    //     VecOfHashMap(Vec<HashMap<&'a str, &'a str>>),
    // }

    // let mut map: HashMap<&str, MapValue> = HashMap::new();
    // map.insert("model", MapValue::String("gpt-3.5-turbo"));

    // map.insert(
    //     "messages",
    //     MapValue::VecOfHashMap(vec![HashMap::from([
    //         ("role", "user"),
    //         ("content", &prompt),
    //     ])]),
    // );
    let client = reqwest::blocking::Client::new();
    let res = client
        .post(CHAT_ENDPOINT)
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        // Passing `api_key` here causes the InvalidHeadValue to be thrown.
        .bearer_auth(api_key.trim())
        .json(&serde_json::json!({
            "model": "gpt-3.5-turbo",
            "messages": [{
                "role": "user",
                "content": &prompt
            }]
        }))
        .send()?
        .json::<ResponseJson>()?;

    println!("{}", res.choices[0].message.content);
    // Go get answer from chat gpt api

    let answer = "The first president was GW.";
    println!("{}", answer);

    Ok(())
}
