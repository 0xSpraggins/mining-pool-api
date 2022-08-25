# mining-pool-api
A REST API built using Rust to track crypto mining pools

## Available API Endpoints

### Miners
    - Get
        - '/miners'
        - '/miners/{id}'
    - Post 
        - '/wallets/{id}/miners'
### Wallets
    - Get
        - '/wallets'
        - '/wallets/{id}'
    - Post 
        - '/wallets'

## Instructions
1. Make sure you are running the required software versions on your machine
2. Navigate to the mining-pool-api root directory in your terminal
3. Enter the command `cargo build` to compile the project.
4. Enter the command `cargo run` to start the server.
## Requirements
- Rust (1.59.0)