# OmniRust

OmniRust is a modular and scalable framework that integrates GraphQL APIs, WebSocket communication, caching mechanisms, and asynchronous logging.

## Setup

1. Clone the repository:
   ```sh
   git clone https://github.com/your-repo/omnirust.git
   cd omnirust
   ```

2. Install dependencies:
   ```sh
   cargo build
   ```

3. Run the application:
   ```sh
   cargo run
   ```

## Usage

### GraphQL API

Example query:
```graphql
query {
  systemStatus
}
```

### WebSocket Communication

Example message:
```json
{
  "type": "sendMessage",
  "content": "Hello, OmniRust!"
}
```

## Dependencies

- async-graphql = "4.0"
- tokio-tungstenite = "0.16"
- serde = { version = "1.0", features = ["derive"] }
- serde_json = "1.0"
- cached = "0.8"
- tracing = "0.1"
- tracing-subscriber = "0.3"
- thiserror = "1.0"
- anyhow = "1.0"
