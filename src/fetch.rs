use std::collections::HashMap;
use tokio;
//use reqwest::blocking;
use reqwest::{ Client };

#[tokio::main]
pub async fn send_message(arr: std::vec::Vec<std::string::String> ) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let mut map = HashMap::new();
    map.insert("news", arr);

    //.headers("Content-Type", "application/json")
    let res = client
        .post("http://httpbin.org/post")
        .json(&map)
        .send()
        .await?;

    println!("{:?}", res);

    Ok(())
}