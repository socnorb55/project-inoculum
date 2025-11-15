use clap::{Parser, Subcommand};

mod commands;
mod database;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    LogFeed {
        #[arg(short, long)]
        flour_amount: f32,

        #[arg(short, long)]
        starter_amount: f32,

        #[arg(long)]
        water_amount: f32,

        #[arg(long)]
        water_temp: f32,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments: Cli = Cli::parse();

    match &arguments.command {
        Some(Commands::LogFeed {
            flour_amount,
            starter_amount,
            water_amount,
            water_temp,
        }) => commands::log_feed::log_feed(
            *flour_amount,
            *starter_amount,
            *water_amount,
            *water_temp,
        ).await?,
        None => {
            println!("No command provided");
        }
    }

    Ok(())
}
