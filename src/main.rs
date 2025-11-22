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
    FeedingHistory {
        #[arg(short, long)]
        maximum_results: Option<i32>,
    },
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
    StartDough {
        #[arg(long)]
        fat: Option<f32>,

        #[arg(long)]
        flour: Option<f32>,

        #[arg(long)]
        leaven: Option<f32>,

        #[arg(long)]
        name: String,

        #[arg(long)]
        recipe: Option<String>,

        #[arg(long)]
        salt: Option<f32>,

        #[arg(long)]
        scale: Option<f32>,

        #[arg(long)]
        sugar: Option<f32>,

        #[arg(long)]
        water: Option<f32>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments: Cli = Cli::parse();

    match &arguments.command {
        Some(Commands::FeedingHistory { maximum_results }) => {
            commands::feeding_history::feeding_history(*maximum_results).await?
        }
        Some(Commands::LogFeed {
            flour_amount,
            starter_amount,
            water_amount,
            water_temp,
        }) => {
            commands::log_feed::log_feed(*flour_amount, *starter_amount, *water_amount, *water_temp)
                .await?
        }
        Some(Commands::StartDough {
            fat,
            flour,
            leaven,
            name,
            recipe,
            salt,
            scale,
            sugar,
            water,
        }) => {
            commands::start_dough::start_dough(
                *fat, *flour, *leaven, name.clone(), recipe.clone(), *salt, *scale, *sugar, *water,
            )
            .await?
        }
        None => {
            println!("No command provided");
        }
    }

    Ok(())
}
