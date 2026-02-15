//! LLM Adapter Layer — OpenAI, Local. Pluggable.

mod trait_;
mod openai;

pub use trait_::LLMAdapter;
pub use openai::OpenAIAdapter;