# Socket Playground

Socket Playground is a Rust-based project that demonstrates a simple TCP server and client implementation. It uses popular libraries like [clap](https://crates.io/crates/clap) for argument parsing, [log](https://crates.io/crates/log) with [env_logger](https://crates.io/crates/env_logger) and [colog](https://crates.io/crates/colog) for logging, and [chrono](https://crates.io/crates/chrono) for timestamping logs. The actual communication between the client and server does not use any external libraries, but rather uses the standard library's TCP Listener and Stream implementation, see [`std::net::TcpListener`](https://doc.rust-lang.org/std/net/struct.TcpListener.html) and [`std::net::TcpStream`](https://doc.rust-lang.org/std/net/struct.TcpStream.html).

## Features

- **Server Mode**: Reads data from a provided file and sends it upon request.
- **Client Mode**: Sends a request string (`REQUEST_DATA`) and receives data from the server, then writes it to a file or displays it.
- **Logging**: Comprehensive logging with colored outputs and custom formatting.
- **Architecture**: Built using Rust in a fully synchronous fashion and super-duper simple blocking I/O.

## Getting Started

### Prerequisites

- Rust (edition 2024)
- Cargo

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/socket-playground.git
   ```

2. Navigate into the project directory:

   ```bash
   cd socket-playground
   ```

3. Build the project:

   ```bash
   cargo build --release
   ```

## Usage

Socket Playground operates in two modes: **client** and **server**. The mode is selectable via a command-line flag (default is **client**).

### Server Mode

Run the server and specify a file path to serve data from:

```bash
cargo run -- --mode server --addr 127.0.0.1 --port 7878 --file /path/to/your/file.txt
```

The server listens for incoming connections and sends the contents of the specified file upon receiving a valid request.

### Client Mode

Run the client to request data from the server:

```bash
cargo run -- --mode client --addr 127.0.0.1 --port 7878 --file /path/to/save/received.txt
```

The client connects to the server, sends a `REQUEST_DATA` command, and writes the server's response to the given file. If no file is provided, it simply prints the response to the standard output.

## Project Structure

- `src/main.rs`: Application entry point.
- `src/server/`: Contains server-related code including TCP handling and request processing.
- `src/client/`: Contains client implementation for requesting and receiving data.
- `src/cli/`: Command-line argument parsing and enums.
- `src/utils/`: Logging setup and miscellaneous helper functions.

## License

This project is licensed under CC0 1.0 Universal. See [LICENSE.txt](LICENSE.txt) for more details.
