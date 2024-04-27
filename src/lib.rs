use reqwest::header;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::error::Error;

#[no_mangle]
pub fn translate(
    text: &str,
    _from: &str,
    to: &str,
    _detect: &str,
    needs: HashMap<String, String>,
) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::blocking::ClientBuilder::new().build()?;
    // api
    let api_key = match needs.get("api_key") {
        Some(key) => key.to_string(),
        None => {
            return Err("API key is missing".into());
        }
    };
    let api_string = format!("Bearer {}", api_key);
    // model
    let default_model = "Qwen/Qwen1.5-14B-Chat".to_string();
    let model_string = needs.get("model_string").unwrap_or(&default_model);
    // system prompt
    let default_system_prompt = format!("你是一位翻译家，请把下面的内容翻译成{to}:");
    let system_prompt = needs.get("system_prompt").unwrap_or(&default_system_prompt);
    // url
    let default_url = "https://api.together.xyz/v1/chat/completions".to_string();
    let url = needs.get("url").unwrap_or(&default_url);

    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", header::HeaderValue::from_str(&api_string)?);
    headers.insert(
        "accept",
        header::HeaderValue::from_static("application/json"),
    );
    headers.insert(
        "content-type",
        header::HeaderValue::from_static("application/json"),
    );

    let data = json!({
    "model": model_string,
    "messages": [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": text}
        ]
    });

    let res = client.post(url).headers(headers).json(&data).send()?;

    let status = res.status();
    match status {
        reqwest::StatusCode::OK => {
            let result_text = res.text()?;
            let result: Value = serde_json::from_str(&result_text)?;
            return Ok(result["choices"][0]["message"]["content"].to_owned());
        }
        _ => {
            return Err(format!("Error {}: {}", status, res.text()?).into());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_request() {
        let mut needs = HashMap::new();
        needs.insert("api_key".to_string(), "YOUR_API_KEY".to_string()); // Replace YOUR_API_KEY with your api key
        let result = translate("你好 世界！", "", "英语", "", needs).unwrap();
        println!("{:}", result.as_str().unwrap());
    }
}
