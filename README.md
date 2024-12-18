# Shuttle AI Agent Pipeline

A Rust-based AI agent pipeline system that chains multiple specialized agents together to process and transform content. The system currently includes agents for research, content writing, Twitter posts, and LinkedIn posts.

## Features

- **Modular Agent System**: Easily extensible system with multiple specialized agents
- **Pipeline Architecture**: Chain multiple agents together to process content sequentially
- **Built-in Agents**:
  - `Researcher`: Gathers and processes information from external sources
  - `Writer`: Transforms raw content into well-structured text
  - `TwitterAgent`: Creates engaging Twitter-optimized content
  - `LinkedInAgent`: Generates professional LinkedIn posts

## Prerequisites

- Rust (latest stable version)
- [Shuttle](https://www.shuttle.rs/) account and CLI
- OpenAI API key
- Serper API key (for research functionality)

## Environment Variables

The following environment variables are required:

```bash
OPENAI_API_KEY=your_openai_api_key
SERPER_API_KEY=your_serper_api_key
```

## Installation

1. Clone the repository:
```bash
git clone https://github.com/dobleuber/shuttle-ai-agent.git
cd shuttle-ai-agent
```

2. Install dependencies:
```bash
cargo build
```

## Usage

### Running the Server

```bash
shuttle run
```

### Making Requests

Send a POST request to `/prompt` with your query:

```bash
curl -X POST http://localhost:8000/prompt \
  -H "Content-Type: application/json" \
  -d '{"q": "Your prompt here"}'
```

### Pipeline Flow

1. The Researcher agent gathers information about the topic
2. (Optional) The Writer agent structures the content
3. The TwitterAgent creates a tweet-friendly version
4. The LinkedInAgent generates a professional LinkedIn post

## Project Structure

```
src/
├── main.rs         # Application entry point and server setup
├── agent.rs        # Agent trait and implementations
├── pipeline.rs     # Pipeline system for chaining agents
├── state.rs        # Application state management
└── errors.rs       # Error handling
```

## Extending the System

To add a new agent:

1. Create a new struct implementing the `Agent` trait
2. Implement required methods: `name()`, `client()`, `system_prompt()`, and `clone_box()`
3. Add the agent to your pipeline configuration

Example:

```rust
#[derive(Clone)]
pub struct MyNewAgent {
    system: Option<String>,
    client: OpenAIClient<OpenAIConfig>,
}

impl Agent for MyNewAgent {
    // Implement required methods
}
```

## License

MIT License

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
