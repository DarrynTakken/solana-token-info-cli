# Solana Token Info CLI
Solana Token Info CLI is a Rust-based command-line tool that retrieves token information for a given Solana address.

## Features
- Retrieve token name
- Retrieve token symbol
- Retrieve token description
- Retrieve token website
  - Retrieve all DNS entries for the token website
- Retrieve telegram
- Retrieve twitter
- Retrieve facebook
- Retrieve instagram
- Retrieve token supply

## Technologies
This project is built using the following technologies and libraries:
- Rust: The primary programming language used for developing this CLI application.
- Solana Ecosystem:
  - Solana SDK: For interacting with the Solana blockchain.
  - Solana Program: For working with Solana smart contracts.
  - Solana Client: For communication with Solana nodes.
  - SPL Token: For interacting with Solana Program Library tokens.
- Clap: For parsing command-line arguments.
- Serde and Serde JSON: For serialization and deserialization of JSON data.
- Tokio: For asynchronous runtime and operations.
- Reqwest: For making HTTP requests to fetch token information.
- Trust-DNS-Resolver: For performing DNS lookups.
- Regex: For pattern matching and text processing.
- URL: For parsing and handling URLs.
- Thiserror: For ergonomic error handling.

These technologies enable the CLI to efficiently interact with the Solana blockchain, process token data, and perform network operations to gather comprehensive token information.

## Installation
Ensure you have Rust and Cargo installed on your system. Then, clone this repository and build the project:

```bash
git clone <your-repo-url>
cd <your-repo-directory>
cargo build --release
```

## Usage

After building the project, you can run the CLI tool to retrieve token information. For example:

```bash
./solana-token-info-cli <TOKEN-ADDRESS>
```

Use --help to see all available commands and options:

```bash
./solana-token-info-cli --help
```

## Dependencies

This project relies on the following dependencies:

```bash
clap = { version = "4.0", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }
bs58 = "0.4"
solana-client = "1.9"
solana-sdk = "1.9"
solana-program = "1.9"
spl-token = "3.3.0"
borsh = "0.9" 
hex = "0.4"
reqwest = { version = "0.11", features = ["blocking", "json"] }
trust-dns-resolver = "0.20"
regex = "1.5"
url = "2.2"
thiserror = "1.0"
```

Make sure these dependencies are properly specified in your Cargo.toml file.

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for more details.
