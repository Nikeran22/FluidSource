# FluidSource: Automated Faucet Deployment for Web3

FluidSource automates the deployment and management of cryptocurrency faucets via smart contract interaction, facilitating decentralized distribution of testnet tokens.

FluidSource provides a robust and efficient solution for managing cryptocurrency distributions, primarily focusing on testnet environments. It leverages the power of smart contracts, serverless functions, and decentralized data storage to create a fully automated and scalable faucet system. The primary goal is to eliminate the manual processes associated with token distribution, allowing developers and users to seamlessly acquire testnet tokens for experimentation and application development. By interacting with a meticulously designed smart contract, users can initiate requests for testnet tokens. Serverless functions act as the backbone of the distribution process, handling the complexities of blockchain transactions and ensuring secure and reliable token delivery. Furthermore, decentralized data storage is employed to maintain transaction history and user data, guaranteeing transparency and immutability.

The architecture of FluidSource is carefully designed to ensure scalability, security, and reliability. The smart contract acts as the central authority, defining the rules and parameters for token distribution. Serverless functions manage the interaction with the blockchain, handling transaction creation, signing, and broadcasting. Decentralized storage ensures that all data is securely stored and accessible. This combination creates a system that is both robust and efficient, capable of handling a large number of requests with minimal overhead. FluidSource streamlines the process of acquiring testnet tokens, making it easier for developers to test and deploy their decentralized applications.

FluidSource not only simplifies the distribution process but also provides a high level of security and transparency. All transactions are recorded on the blockchain, providing an immutable record of token distribution. The smart contract ensures that tokens are distributed according to predefined rules, preventing abuse and ensuring fairness. The serverless functions are designed to be secure and reliable, minimizing the risk of errors or vulnerabilities. Overall, FluidSource is a comprehensive solution for managing cryptocurrency distributions, offering a secure, scalable, and transparent platform for testnet token distribution.

## Key Features

*   **Smart Contract Interaction:** Rust-based smart contract integration for defining faucet rules, token limits, and recipient verification. The contract utilizes `ink!` framework for building secure and efficient Wasm smart contracts.
*   **Serverless Function Automation:** AWS Lambda functions (written in Rust utilizing `lambda_runtime`) automatically process faucet requests, sign transactions, and broadcast them to the blockchain network. Error handling and retry mechanisms are implemented for transaction failures.
*   **Decentralized Data Storage:** IPFS integration for storing transaction history, user request logs, and configuration data, ensuring immutability and transparency. Data is structured and indexed for efficient querying.
*   **Request Rate Limiting:** Implements rate limiting at both the smart contract and serverless function levels to prevent abuse and ensure fair distribution of tokens. Uses a sliding window algorithm for accurate rate limiting.
*   **REST API Endpoint:** Provides a REST API endpoint for interacting with the faucet. Uses `actix-web` framework for creating a fast and reliable API. Includes authentication and authorization mechanisms to prevent unauthorized access.
*   **Blockchain Network Agnostic:** Designed to be compatible with multiple blockchain networks (e.g., Ethereum, Polkadot) by configuring the appropriate network parameters and smart contract addresses.
*   **Configurable Token Denomination:** Allows for configuration of the token denomination to be distributed, supporting different cryptocurrencies and custom tokens.

## Technology Stack

*   **Rust:** The primary programming language used for smart contracts, serverless functions, and API development, offering performance, safety, and concurrency.
*   **ink!:** A smart contract programming language built on Rust, specifically designed for creating Wasm smart contracts for Substrate-based blockchains.
*   **actix-web:** A powerful, pragmatic, and extremely fast Rust web framework, used for creating the REST API endpoint.
*   **AWS Lambda:** A serverless computing service provided by Amazon Web Services, used for executing the automated faucet logic.
*   **IPFS (InterPlanetary File System):** A decentralized storage network used for storing transaction history, logs, and configuration data.
*   **Substrate/Polkadot (Optional):** A modular framework for building blockchains, potentially used for deploying the faucet smart contract and managing the blockchain network.
*   **Web3.rs:** Rust library for interacting with Ethereum-like blockchains, used for transaction signing and broadcasting.

## Installation

1.  **Install Rust:** Ensure you have Rust installed. Follow the instructions on [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

2.  **Install `cargo-contract`:** This tool is required for building and testing ink! smart contracts. Run: `cargo install cargo-contract --force`

3.  **Clone the repository:**
    `git clone https://github.com/Nikeran22/FluidSource.git`
    `cd FluidSource`

4.  **Build the smart contract:** Navigate to the `contracts` directory and run:
    `cargo contract build`
    This will generate a WASM file for the smart contract.

5.  **Build the serverless function:** Navigate to the `functions` directory and build the serverless function using:
    `cargo build --release`

6.  **Install AWS CLI:** Install and configure the AWS CLI for deploying the serverless function to AWS Lambda. Instructions can be found on the AWS website.

7.  **Deploy the smart contract:** Deploy the compiled WASM file to your target blockchain environment (e.g., local Substrate node, testnet). Note the contract address.

## Configuration

The following environment variables need to be configured for the serverless function:

*   `CONTRACT_ADDRESS`: The address of the deployed faucet smart contract.
*   `PRIVATE_KEY`: The private key of the account used to sign transactions.
*   `NETWORK_URL`: The URL of the blockchain node to connect to (e.g., RPC endpoint).
*   `IPFS_GATEWAY`: The URL of the IPFS gateway to use for data storage.
*   `RATE_LIMIT_WINDOW`: The time window for rate limiting (e.g., 60 seconds).
*   `RATE_LIMIT_REQUESTS`: The maximum number of requests allowed within the rate limit window (e.g., 5 requests).
*   `TOKEN_DECIMAL`: The decimal places the token has.
*   `TOKEN_ID`: The chain ID of the token.

These variables should be set in the AWS Lambda environment configuration. The `contract` directory contains the smart contract code and requires no runtime configuration. The `api` directory contains similar configuration needs to the function directory and should be similarly configured.

## Usage

**Smart Contract Interaction:**

The smart contract exposes functions for requesting tokens and querying faucet status. Refer to the `contracts/src/lib.rs` file for the specific function signatures and usage details.

**API Endpoint:**

Send a POST request to the API endpoint with the user's blockchain address to request tokens. For example:

`curl -X POST -H "Content-Type: application/json" -d '{"address": "0x1234567890abcdef"}' <API_ENDPOINT_URL>`

The API will return a JSON response indicating the status of the request.

**IPFS Data Retrieval:**

Transaction history and logs can be retrieved from IPFS using the hash stored in the smart contract or returned by the API. Use an IPFS gateway or client to retrieve the data.

## Contributing

We welcome contributions to FluidSource! Please follow these guidelines:

1.  Fork the repository.
2.  Create a new branch for your feature or bug fix.
3.  Write clear and concise commit messages.
4.  Submit a pull request with a detailed description of your changes.

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/Nikeran22/FluidSource/blob/main/LICENSE) file for details.

## Acknowledgements

We would like to thank the Rust community, the `ink!` developers, and the creators of the various libraries and tools used in this project.