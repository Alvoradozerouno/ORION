//! LLM Adapter Layer — OpenAI, Llama, Anthropic austauschbar

pub trait LLMAdapter: Send + Sync {
    fn complete(&self, prompt: &str, temperature: f32) -> Result<String, String>;
}
