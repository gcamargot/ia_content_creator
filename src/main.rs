use reqwest;
use std::fs;
use std::path::Path;
use std::error::Error;
use serde_json;


#[tokio::main]
async fn main() {

    let file_path = Path::new("./src/apikey.txt");
    let apikey = fs::read_to_string(file_path)
        .expect("Should have been able to read the file!");
    let response = request_prompt(apikey.as_str()).await.unwrap();
    println!("{}", response);

}



async fn request_prompt(apikey: &str) -> Result<String, Box<dyn Error>>{
    let query =  format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key={apikey}");
    //println!("{}",query);
    let client = reqwest::Client::new();
    let res = client.post(query )
        .body(r#"{
            "contents": [{
                "parts":[{
                "text": "Write a story about a magic backpack."}]
            }]
        }"#)
        .send()
        .await?;
    if res.status().is_success(){
        let json: serde_json::Value = serde_json::from_str(res.text().await?.as_str()).expect("JSON was not well-formatted");
        if let Some(candidates) = json.get("candidates"){
            if let Some(candidate) = candidates.get(0){
                if let Some(content) = candidate.get("content"){
                    if let Some(parts) = content.get("parts"){
                        if let Some(part) = parts.get(0){
                            if let Some(text) = part.get("text"){
                                return Ok(text.as_str().unwrap_or("No text found").to_string());
                            }
                        }
                    }
                }
            }
        }



    }
    Err("Unable to retrieve text from the response".into())

}




