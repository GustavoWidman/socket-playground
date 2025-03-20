use std::rc::Rc;

use clap::Parser;

mod cli;
mod client;
mod server;
mod utils;

fn main() {
    let args = Rc::new(cli::Args::parse());
    utils::Logger::init(&args);

    match args.mode {
        cli::Mode::Client => {
            let client = client::Client::new(args);

            let result = match client {
                Ok(client) => client.run(),
                Err(e) => {
                    log::error!("Error while initializing client:\n\n{}\n", e);
                    std::process::exit(1);
                }
            };

            if let Err(e) = result {
                log::error!("Error during client execution:\n\n{}\n", e);
                std::process::exit(1);
            }
        }
        cli::Mode::Server => {
            let server = server::Server::new(args);

            let result = match server {
                Ok(server) => server.run(),
                Err(e) => {
                    log::error!("Error while initializing server:\n\n{}\n", e);
                    std::process::exit(1);
                }
            };

            if let Err(e) = result {
                log::error!("Error during server execution:\n\n{}\n", e);
                std::process::exit(1);
            }
        }
    }
}
