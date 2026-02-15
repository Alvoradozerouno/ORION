//! LLM Adapter trait — model-agnostic

use async_trait::async_trait;

#[async_trait]
pub trait LLMAdapter: Send + Sync {
    async fn complete(&self, prompt: &str, temperature: f32) -> Result<String, String>;
}
