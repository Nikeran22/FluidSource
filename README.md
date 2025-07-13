# FluidSource: Lightweight Data Pipeline Framework in Rust

FluidSource is a rust-based framework designed to simplify the creation and management of data pipelines. It provides a streamlined approach to defining data sources, transformations, and destinations, allowing developers to build robust and efficient data workflows with minimal boilerplate.

The core concept behind FluidSource is the modularity of pipeline components. Each data source, transformation, and destination is treated as an independent module, allowing for easy composition and reuse across different pipelines. This design promotes code maintainability and reduces the complexity of managing large data processing tasks. FluidSource leverages Rust's strong type system and memory safety features to ensure the reliability and stability of the pipelines. It handles data serialization, error propagation, and concurrency management, abstracting away many of the complexities involved in building data processing applications.

FluidSource is particularly well-suited for applications that require real-time data processing, data integration from multiple sources, and the creation of custom data transformations. Its lightweight design makes it efficient and suitable for resource-constrained environments. The framework also emphasizes extensibility, allowing developers to create custom modules for specific data sources, transformations, or destinations, making it adaptable to a wide range of data processing scenarios.

The framework is focused on maximizing performance and scalability. It employs asynchronous processing techniques and takes advantage of Rust's ability to manage concurrency safely. Developers can define their pipelines using a declarative approach, specifying the data flow and transformations without needing to worry about the underlying implementation details. FluidSource then optimizes the pipeline execution for optimal throughput and resource utilization.

Key Features:

*   **Modular Pipeline Components:** Data sources, transformations, and destinations are implemented as independent modules that can be easily composed and reused. Modules conform to a defined trait which specifies the required functionality, such as `fetch()`, `transform()`, and `sink()`.
*   **Asynchronous Processing:** FluidSource leverages Rust's asynchronous capabilities to enable concurrent data processing, maximizing throughput and minimizing latency. Tokio is utilized for the runtime environment.
*   **Data Serialization & Deserialization:** Automatic handling of data serialization and deserialization between pipeline stages, supporting common formats like JSON, CSV, and Avro using Serde. Requires feature flags to enable desired formats.
*   **Error Handling & Propagation:** Robust error handling mechanisms ensure that errors are properly propagated through the pipeline, allowing for appropriate handling and logging. Uses Rust's standard Result type.
*   **Custom Module Development:** Flexible architecture allows developers to create custom data sources, transformations, and destinations to meet specific requirements. Custom modules must implement the defined traits for compatibility.
*   **Declarative Pipeline Definition:** Pipelines are defined using a simple, declarative approach, allowing developers to focus on the data flow and transformations. The configuration is defined using Rust code, allowing maximum flexibility.
*   **Pluggable Architecture:** Allows for easy integration of external libraries and services, such as databases, message queues, and cloud storage. Requires implementing corresponding module that conforms to defined trait.

Technology Stack:

*   **Rust:** The core programming language, providing performance, safety, and concurrency features.
*   **Tokio:** An asynchronous runtime for Rust, enabling concurrent data processing.
*   **Serde:** A serialization and deserialization framework for Rust, supporting various data formats. Activated through feature flags.
*   **Log:** For logging and debugging purposes, providing insights into pipeline execution.
*   **clap:** For command-line argument parsing, enabling configuration of pipeline parameters.

Installation:

1.  Ensure you have Rust installed. You can download it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
2.  Clone the FluidSource repository:
    git clone <repository_url>
3.  Navigate to the FluidSource directory:
    cd FluidSource
4.  Build the project using Cargo:
    cargo build --release
5.  The executable will be located in the `target/release` directory.

Configuration:

FluidSource is configured using a combination of environment variables and command-line arguments. The following environment variables are supported:

*   `FLUIDSOURCE_CONFIG_PATH`: Specifies the path to the configuration file. Default is `config.toml` in the current working directory.
*   `FLUIDSOURCE_LOG_LEVEL`: Sets the logging level (e.g., `debug`, `info`, `warn`, `error`). Default is `info`.

Command-line arguments can be used to override environment variables and specify pipeline parameters. Refer to the `--help` flag for a complete list of available arguments.
cargo run --release -- --help

Usage:

FluidSource pipelines are defined using Rust code. A typical pipeline consists of a data source, one or more transformations, and a data destination.

Here's an example of a simple pipeline:

// Example code (not markdown block)
use fluidsource::prelude::*;

fn main() {
  let source = FileSource::new("input.txt");
  let transform = IdentityTransform::new();
  let destination = ConsoleDestination::new();

  let pipeline = Pipeline::new(source, transform, destination);
  pipeline.run();
}
// End example code

Refer to the API documentation for detailed information on available data sources, transformations, and destinations. You can generate the documentation using cargo doc.

Contributing:

We welcome contributions to FluidSource! Please follow these guidelines:

*   Fork the repository and create a branch for your changes.
*   Write clear and concise commit messages.
*   Include unit tests for your changes.
*   Submit a pull request with a detailed description of your changes.
*   Follow the Rust code style guidelines. Run `cargo fmt` before submitting your pull request.

License:

This project is licensed under the MIT License. See the [LICENSE](https://github.com/Nikeran22/FluidSource/blob/main/LICENSE) file for details.

Acknowledgements:

We would like to thank the Rust community for their support and contributions to the Rust ecosystem.