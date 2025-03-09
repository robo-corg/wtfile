use eyre::Result;
use reqwest::RequestBuilder;
use serde_json::Value;
use std::borrow::Cow;
use std::collections::HashMap;
use tracing::debug;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Message<'a> {
    pub role: Cow<'a, str>,
    pub content: Cow<'a, str>,
}

impl<'a> Message<'a> {
    pub fn user(content: &'a str) -> Self {
        Self {
            role: "user".into(),
            content: content.into(),
        }
    }

    pub fn system(content: &'a str) -> Self {
        Self {
            role: "system".into(),
            content: content.into(),
        }
    }
}

#[derive(Debug, serde::Serialize)]
struct CompletionsRequest<'a> {
    messages: &'a [Message<'a>],
    model: &'a str,
    max_tokens: u32,
    presence_penalty: f32,
    temperature: f32,
    top_p: f32,
    #[serde(default)]
    response_format: Option<ResponseFormat>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseFormatType {
    JsonObject,
}

#[derive(Debug, serde::Serialize)]
pub struct ResponseFormat {
    #[serde(rename = "type")]
    pub ty: ResponseFormatType,
    pub schema: Option<serde_json::Value>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Choice {
    pub message: Message<'static>,
    #[allow(dead_code)]
    pub finish_reason: String,
    #[allow(dead_code)]
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CompletionsResponse {
    pub choices: Vec<Choice>,

    #[allow(dead_code)]
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

pub struct TextGenClient {
    client: reqwest::Client,
    base_url: Cow<'static, str>,
}

pub struct CompletionsRequestBuilder<'a> {
    builder: RequestBuilder,
    messages: Option<&'a [Message<'a>]>,
    model: Cow<'a, str>,
    max_tokens: u32,
    presence_penalty: f32,
    temperature: f32,
    top_p: f32,
    response_format: Option<ResponseFormat>,
}

impl<'a> CompletionsRequestBuilder<'a> {
    fn new(builder: RequestBuilder) -> Self {
        Self {
            builder,
            messages: None,
            model: "hermes-2-pro-mistral-7b".into(),
            max_tokens: 512,
            presence_penalty: 0.0,
            temperature: 1.0,
            top_p: 0.9,
            response_format: None,
        }
    }

    pub fn messages(mut self, messages: &'a [Message]) -> Self {
        self.messages = Some(messages);
        self
    }

    pub fn model(mut self, model: &'a str) -> Self {
        self.model = model.into();
        self
    }

    #[allow(dead_code)]
    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = max_tokens;
        self
    }

    #[allow(dead_code)]
    pub fn presence_penalty(mut self, presence_penalty: f32) -> Self {
        self.presence_penalty = presence_penalty;
        self
    }

    #[allow(dead_code)]
    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature;
        self
    }

    #[allow(dead_code)]
    pub fn top_p(mut self, top_p: f32) -> Self {
        self.top_p = top_p;
        self
    }

    #[allow(dead_code)]
    pub fn json_mode(mut self) -> Self {
        self.response_format = Some(ResponseFormat {
            ty: ResponseFormatType::JsonObject,
            schema: None,
        });
        self
    }

    #[allow(dead_code)]
    pub fn json_schema<S>(mut self, schema: S) -> Self
    where
        S: serde::Serialize,
    {
        let schema_value = serde_json::to_value(schema).unwrap();

        self.response_format = Some(ResponseFormat {
            ty: ResponseFormatType::JsonObject,
            schema: Some(schema_value),
        });
        self
    }

    pub async fn send(self) -> Result<CompletionsResponse> {
        let messages = if let Some(messages) = self.messages {
            if messages.is_empty() {
                return Err(eyre::eyre!("No messages to send"));
            }
            messages
        } else {
            return Err(eyre::eyre!("No messages to send"));
        };

        let req = CompletionsRequest {
            messages,
            model: self.model.as_ref(),
            max_tokens: self.max_tokens,
            presence_penalty: self.presence_penalty,
            temperature: self.temperature,
            top_p: self.top_p,
            response_format: self.response_format,
        };

        //println!("Request:\n{}", serde_json::to_string_pretty(&req).unwrap());

        let resp = self.builder.json(&req).send().await?;

        if let Err(e) = resp.error_for_status_ref() {
            let body = resp.text().await?;
            return Err(eyre::eyre!(
                "Error making chat completion request: {}, body: {}",
                e,
                body
            ));
        }

        let resp_data: CompletionsResponse = resp.json().await?;

        debug!(?resp_data, "chat_completions_response");

        Ok(resp_data)
    }
}

impl TextGenClient {
    pub fn new(base_url: &str, token: &str) -> Self {
        let client = reqwest::Client::builder()
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                let bearer = format!("Bearer {}", token);
                headers.insert("Authorization", bearer.parse().unwrap());
                headers
            })
            .build()
            .unwrap();

        let base_url = base_url.to_owned().into();
        Self { client, base_url }
    }

    pub fn chat_completions(&self) -> CompletionsRequestBuilder {
        let builder = self.client.post(self.base_url.as_ref());
        CompletionsRequestBuilder::new(builder)
    }
}
