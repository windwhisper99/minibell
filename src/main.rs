use clap::{Parser, Subcommand};

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
    Migrate,

    /// Run the discord bot only
    Bot,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    match args.command {
        Some(Commands::Migrate) => {
            println!("Running migrations");
            return Ok(());
        }
        Some(Commands::Bot) => {
            println!("Running discord bot");
            return Ok(());
        }
        None => web::run(args.host, args.port).await,
    }
}
