use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

const DEFAULT_URL_COMPLETION: &str = "https://api.openai.com/v1/chat/completions";

fn default_url_completion() -> String {
    DEFAULT_URL_COMPLETION.to_string()
}

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    openai: OpenAI,
}

#[derive(Debug, Deserialize, Serialize)]
struct OpenAI {
    model: String,
    access_key: String,
    #[serde(default = "default_url_completion")]
    url_completion: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Role {
    System,
    User,
    Assistant,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Role::System => write!(f, "system"),
            Role::User => write!(f, "user"),
            Role::Assistant => write!(f, "assistant"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct CompletionBody {
    model: String,
    messages: Vec<Message>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub usage: UsageStats,
    pub choices: Vec<CompletionChoice>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageStats {
    #[serde(rename = "prompt_tokens")]
    pub prompt_tokens: i64,
    #[serde(rename = "completion_tokens")]
    pub completion_tokens: i64,
    #[serde(rename = "total_tokens")]
    pub total_tokens: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionChoice {
    pub message: Message,
    #[serde(rename = "finish_reason")]
    pub finish_reason: String,
    pub index: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub role: String,
    pub content: String,
}

pub struct Gptcli {
    config: Config,
}

impl Gptcli {
    pub fn new(config_file: String) -> Gptcli {
        let config = Gptcli::load_config(&config_file);
        Gptcli { config }
    }

    fn load_config(config_file: &str) -> Config {
        let config = match std::fs::read_to_string(config_file) {
            Ok(c) => c,
            Err(e) => {
                println!("Error: {}", e);
                std::process::exit(1);
            }
        };

        let config: Config = match toml::from_str(&config) {
            Ok(c) => c,
            Err(e) => {
                println!("Error: {}", e);
                std::process::exit(1);
            }
        };

        config
    }

    pub fn submit_messages(
        &self,
        messages: Vec<Message>,
    ) -> Result<CompletionResponse, Box<dyn Error>> {
        let cb = CompletionBody {
            model: self.config.openai.model.to_string(),
            messages,
        };

        let client = reqwest::blocking::Client::new();
        let res = client
            .post(&self.config.openai.url_completion)
            .header(
                "Authorization",
                format!("Bearer {}", self.config.openai.access_key),
            )
            .json(&cb)
            .send()?
            .text()?;

        let res: CompletionResponse = match serde_json::from_str(&res) {
            Ok(m) => m,
            Err(e) => {
                println!("Message: {}", res);
                println!("Error: {}", e);
                return Err(Box::new(e));
            }
        };

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, default_url_completion};

    #[test]
    fn parse_config_with_custom_url() {
        let toml_str = r#"
        [openai]
        model = "gpt-3.5-turbo"
        access_key = "key"
        url_completion = "http://example.com"
        "#;
        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.openai.url_completion, "http://example.com");
    }

    #[test]
    fn parse_config_without_url_uses_default() {
        let toml_str = r#"
        [openai]
        model = "gpt-3.5-turbo"
        access_key = "key"
        "#;
        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.openai.url_completion, default_url_completion());
    }
}
