use reqwest::Client;
use serde_json::json;
use std::env;

const GEMINI_API_URL: &str =
    "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent";

pub async fn suggest_command_llm(full_mistyped: &str, commands: &[String]) -> Option<String> {
    let api_key = match env::var("GEMINI_API_KEY") {
        Ok(key) => key,
        Err(_) => return None,
    };

    let url = format!("{}?key={}", GEMINI_API_URL, api_key);
    let prompt = format!(
        "The user typed the incorrect terminal command line: '{}'. \
        Fix any typos in the command or its arguments. \
        The main command must exist in this list of available commands: {}. \
        Provide ONLY the raw corrected command line without quotes, markdown blocks, or explanation.",
        full_mistyped,
        commands.join(", ")
    );

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(2))
        .build()
        .unwrap_or_else(|_| Client::new());

    let payload = json!({
        "contents": [{
            "parts": [{"text": prompt}]
        }]
    });

    match client.post(&url).json(&payload).send().await {
        Ok(response) if response.status().is_success() => {
            if let Ok(json_response) = response.json::<serde_json::Value>().await {
                if let Some(candidates) = json_response.get("candidates") {
                    if let Some(first_candidate) = candidates.as_array().and_then(|c| c.first()) {
                        if let Some(content) = first_candidate.get("content") {
                            if let Some(parts) = content.get("parts") {
                                if let Some(first_part) = parts.as_array().and_then(|p| p.first()) {
                                    if let Some(text) = first_part.get("text") {
                                        let mut suggestion =
                                            text.as_str().unwrap_or("").trim().to_string();

                                        // Clean markdown code blocks or quotes
                                        suggestion = suggestion.trim_matches('`').to_string();
                                        suggestion = suggestion.trim_matches('\'').to_string();
                                        suggestion = suggestion.trim_matches('"').to_string();
                                        if suggestion.starts_with("bash\n") {
                                            suggestion = suggestion["bash\n".len()..].to_string();
                                        }
                                        suggestion = suggestion.trim().to_string();

                                        // Ensure the main extracted command is valid
                                        let main_cmd =
                                            suggestion.split_whitespace().next().unwrap_or("");
                                        if commands.contains(&main_cmd.to_string()) {
                                            return Some(suggestion);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }

    None
}
