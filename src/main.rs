use clap::{Parser, Subcommand};
use dotenv;

#[derive(Subcommand)]
enum Command {
    /// Starts the server
    Server {
        /// Sets the port to bind to
        #[clap(long, short = 'p')]
        port: u32,
    },
    /// Starts the client
    Client {
        /// Sets the IP address to connect to
        #[clap(long, short = 'i')]
        ip_address: String,
        /// Sets the port to connect to
        #[clap(long, short = 'p')]
        port: u32,
    },
}
/// Texas Holdem' in the terminal
#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

fn main() {
    // initialize the logger
    let _ = dotenv::dotenv();
    env_logger::init();

    // start the application
    match Cli::parse().command {
        Command::Server { port } => {
            println!("Binding the server to port {}", port);
        }
        Command::Client { ip_address, port } => {
            println!("Connecting the client to {}:{}", ip_address, port);
        }
    }
}
