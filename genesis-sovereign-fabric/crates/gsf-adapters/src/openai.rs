//! OpenAI Adapter — API key from ENV

use super::LLMAdapter;
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Deserialize)]
struct Message {
    content: String,
}

pub struct OpenAIAdapter {
    api_key: String,
    model: String,
    base_url: String,
}

impl OpenAIAdapter {
    pub fn from_env() -> Option<Self> {
        let api_key = std::env::var("OPENAI_API_KEY").ok()?;
        let model = std::env::var("GSF_OPENAI_MODEL").unwrap_or_else(|_| "gpt-4o-mini".to_string());
        let base_url = std::env::var("OPENAI_BASE_URL").unwrap_or_else(|_| "https://api.openai.com/v1".to_string());
        Some(Self {
            api_key,
            model,
            base_url,
        })
    }
}

#[async_trait]
impl LLMAdapter for OpenAIAdapter {
    async fn complete(&self, prompt: &str, temperature: f32) -> Result<String, String> {
        let client = reqwest::Client::new();
        let url = format!("{}/chat/completions", self.base_url);
        let body = serde_json::json!({
            "model": self.model,
            "messages": [{"role": "user", "content": prompt}],
            "temperature": temperature
        });
        let res = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        let text = res.text().await.map_err(|e| e.to_string())?;
        let parsed: OpenAIResponse = serde_json::from_str(&text).map_err(|e| e.to_string())?;
        let content = parsed
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_default();
        Ok(content)
    }
}
