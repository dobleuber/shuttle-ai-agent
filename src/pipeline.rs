use crate::agent::Agent;
use crate::errors::ApiError;
use async_trait::async_trait;

#[async_trait]
pub trait Pipeline: Send + Sync {
    fn init_agents(&self) -> Vec<Box<dyn Agent>>;

    async fn run_pipeline(&self, initial_input: &str) -> Result<String, ApiError> {
        let agents = self.init_agents();
        let mut current_output = String::from(initial_input);

        for agent in agents {
            current_output = agent.prompt(&current_output, String::new()).await?;
        }

        Ok(current_output)
    }
}

pub struct ContentPipeline {
    agents: Vec<Box<dyn Agent>>,
}

impl ContentPipeline {
    pub fn new(agents: Vec<Box<dyn Agent>>) -> Self {
        Self { agents }
    }
}

impl Pipeline for ContentPipeline {
    fn init_agents(&self) -> Vec<Box<dyn Agent>> {
        self.agents.iter().map(|agent| agent.clone_box()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::{LinkedInAgent, TwitterAgent};

    #[tokio::test]
    async fn test_content_pipeline() {
        let agents: Vec<Box<dyn Agent>> = vec![
            Box::new(TwitterAgent::new()),
            Box::new(LinkedInAgent::new()),
        ];

        let pipeline = ContentPipeline::new(agents);
        let result = pipeline.run_pipeline("Test input").await;
        assert!(result.is_ok());
    }
}
