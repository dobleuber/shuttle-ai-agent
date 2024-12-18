use crate::errors::ApiError;
use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
};
use async_openai::{config::OpenAIConfig, Client as OpenAIClient};
use reqwest::header::HeaderMap;
use serde_json::Value;
use std::env;
use async_trait::async_trait;

#[async_trait]
pub trait Agent: Send + Sync {
    fn name(&self) -> String;
    fn client(&self) -> OpenAIClient<OpenAIConfig>;
    fn system_prompt(&self) -> String;
    fn clone_box(&self) -> Box<dyn Agent>;

    async fn prompt(&self, input: &str, data: String) -> Result<String, ApiError> {
        let input = format!(
            "{input}
            Provided context:
            {}
            ",
            serde_json::to_string_pretty(&data)?
        );

        let res = self
            .client()
            .chat()
            .create(
                CreateChatCompletionRequestArgs::default()
                    .model("gpt-4o-mini")
                    .messages(vec![
                        //First we add the system message to define what the Agent does
                        ChatCompletionRequestMessage::System(
                            ChatCompletionRequestSystemMessageArgs::default()
                                .content(self.system_prompt())
                                .build()?,
                        ),
                        //Then we add our prompt
                        ChatCompletionRequestMessage::User(
                            ChatCompletionRequestUserMessageArgs::default()
                                .content(input)
                                .build()?,
                        ),
                    ])
                    .build()?,
            )
            .await
            .map(|res| res.choices[0].message.content.clone().unwrap())?;

        println!("Retrieved result from prompt: {}", res);

        Ok(res)
    }
}

#[derive(Clone)]
pub struct Researcher {
    http_client: reqwest::Client,
    system: Option<String>,
    openai_client: OpenAIClient<OpenAIConfig>,
}

impl Agent for Researcher {
    fn name(&self) -> String {
        String::from("Researcher")
    }

    fn client(&self) -> OpenAIClient<OpenAIConfig> {
        self.openai_client.clone()
    }

    fn system_prompt(&self) -> String {
        if let Some(message) = &self.system {
            message.clone()
        } else {
            "You are an agent.

        You will receive a question that may be quite short or does not have much context.
        Your job is to research the Internet and to return with a high-quality summary to the user, assisted by the provided context.
        The provided context will be in JSON format and contains data about the initial Google results for the website or query.

        Be concise.

        Question:
        ".to_string()
        }
    }

    fn clone_box(&self) -> Box<dyn Agent> {
        Box::new(self.clone())
    }
}

impl Researcher {
    pub fn new() -> Self {
        let openai_client = OpenAIClient::new();

        let mut headers = HeaderMap::new();
        headers.insert(
            "X-Api-Key",
            env::var("SERPER_API_KEY")
                .expect("SERPER_API_KEY must be set")
                .parse()
                .unwrap(),
        );
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let http_client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Researcher {
            http_client,
            system: None,
            openai_client,
        }
    }

    pub async fn prepare_data(&self, prompt: &str) -> Result<String, ApiError> {
        let json = serde_json::json!({
            "q": prompt,
        });

        let res = self
            .http_client
            .post("https://google.serper.dev/search")
            .json(&json)
            .send()
            .await
            .unwrap();

        let json = res.json::<Value>().await?;

        Ok(serde_json::to_string_pretty(&json)?)
    }
}

#[derive(Clone)]
pub struct Writer {
    system: Option<String>,
    client: OpenAIClient<OpenAIConfig>,
}

impl Writer {
    pub fn new() -> Self {
        let client = OpenAIClient::new();
        Writer {
            system: None,
            client,
        }
    }
}

impl Agent for Writer {
    fn name(&self) -> String {
        String::from("Writer")
    }

    fn client(&self) -> OpenAIClient<OpenAIConfig> {
        self.client.clone()
    }

    fn system_prompt(&self) -> String {
        if let Some(message) = &self.system {
            message.clone()
        } else {
            "You are an agent.

        You will receive some context from another agent about some Google results that a user has searched.
        Your job is to research the Internet and to write a high-quality article that a user has written. The article must not appear to be AI written. The article should be SEO optimised without overly compromising the
        quality of the article.

        You are free to be as creative as you wish. However, each paragraph must have the following:
        - The point you are trying to make
        - If there is a follow up action point
        - Why the follow up action point exists (or why the user needs to carry it out)

        Search query:
"
        .to_string()
        }
    }

    fn clone_box(&self) -> Box<dyn Agent> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct TwitterAgent {
    system: Option<String>,
    client: OpenAIClient<OpenAIConfig>,
}

impl TwitterAgent {
    pub fn new() -> Self {
        let client = OpenAIClient::new();
        TwitterAgent {
            system: None,
            client,
        }
    }
}

impl Agent for TwitterAgent {
    fn name(&self) -> String {
        String::from("twitter")
    }

    fn client(&self) -> OpenAIClient<OpenAIConfig> {
        self.client.clone()
    }

    fn system_prompt(&self) -> String {
        self.system.clone().unwrap_or_else(|| String::from(
            "You are a social media agent specializing in Twitter content creation.

            Your job is to craft high-quality Twitter posts that are engaging, concise, and tailored for virality. 
            The content should reflect a personal tone, avoid AI-written patterns, and adhere to Twitter's character limit.

            When crafting each post:
            - Hook the reader in the first sentence (e.g., surprising fact, bold statement, or question)
            - Deliver the main point or insight concisely
            - Include a follow-up action, like engagement prompts (e.g., 'Share your thoughts,' 'RT if you agree')
            - Make use of hashtags, emojis, or a storytelling structure to boost engagement when appropriate
            - Ensure the tone is conversational, witty, or thought-provoking, depending on the context

            The content must be SEO-aware without appearing overly optimized. Avoid stuffing keywords unnaturally."
        ))
    }

    fn clone_box(&self) -> Box<dyn Agent> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct LinkedInAgent {
    system: Option<String>,
    client: OpenAIClient<OpenAIConfig>,
}

impl LinkedInAgent {
    pub fn new() -> Self {
        let client = OpenAIClient::new();
        LinkedInAgent {
            system: None,
            client,
        }
    }
}

impl Agent for LinkedInAgent {
    fn name(&self) -> String {
        String::from("linkedin")
    }

    fn client(&self) -> OpenAIClient<OpenAIConfig> {
        self.client.clone()
    }

    fn system_prompt(&self) -> String {
        self.system.clone().unwrap_or_else(|| String::from(
            "You are a professional networking agent specializing in LinkedIn content creation.

            Your task is to write LinkedIn posts that are insightful, engaging, and suitable for a professional audience 
            while maintaining a human tone.

            When writing each LinkedIn post:
            - Start with a strong opening (e.g., a thought-provoking question, a bold statement, or an anecdote)
            - Clearly communicate the main idea or insight while providing context
            - Include actionable advice or a follow-up point to inspire engagement, reflection, or discussion
            - Explain the 'why' behind the advice, connecting it to professional or personal development
            - Maintain a professional, thoughtful tone with a touch of authenticity
            - Add a strong call-to-action: invite comments, share experiences, or encourage reposts

            Structure the content in short, scannable paragraphs. Break ideas into clear sections for readability.
            The content must align with LinkedIn's audience: professionals seeking value, insights, and connection."
        ))
    }

    fn clone_box(&self) -> Box<dyn Agent> {
        Box::new(self.clone())
    }
}
