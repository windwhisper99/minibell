use clap::{Parser, Subcommand};

mod domain;
mod infra;
mod migration;
mod repos;
mod services;
mod usecase;
mod utils;
mod web;

#[derive(Parser)]
#[command(version)]
#[command(about = "Run MiniBell web application", long_about = None)]
struct Args {
    /// The host to listen on
    #[arg(long, default_value = "127.0.0.1")]
    host: String,

    /// The port to listen on
    #[arg(short, long, default_value = "8080")]
    port: u16,

    /// Use an external database, instead of embedded
    #[arg(long, default_value = "false")]
    external_db: bool,

    /// Run the discord bot included in the application
    #[arg(long, default_value = "false")]
    discord_bot: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run migrations
    Migration,

    /// Run the discord bot only
    Bot,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    let args = Args::parse();

    match args.command {
        Some(Commands::Migration) => migration::run().await,
        Some(Commands::Bot) => {
            println!("Running discord bot");
            Ok(())
        }
        None => web::run(args.host, args.port).await,
    }
}
