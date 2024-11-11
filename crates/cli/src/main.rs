use clap::{Parser, Subcommand};

mod duty;

#[derive(Parser)]
#[command(version)]
#[command(about = "Run MiniBell CLI", long_about = None)]
struct Args {
    /// AWS Secret Manager Key
    #[arg(short, long)]
    secret_manager_key: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run duty related commands
    Duty {
        /// Manifest file
        file: String,
    },
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    let args = Args::parse();

    let secret_manager_key = match args.secret_manager_key {
        Some(key) => key,
        None => std::env::var("SECRET_KEY").expect("SECRET_MANAGER_KEY must be set"),
    };

    match args.command {
        Some(Commands::Duty { file }) => {
            duty::upload_duty(&file, &secret_manager_key).await;
            Ok(())
        }
        None => Ok(()),
    }
}
