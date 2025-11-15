use clap::Parser;

use crate::database::models::StarterFeeding;

mod database;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    flour_amount: f32,

    #[arg(short, long)]
    starter_amount: f32,

    #[arg(long)]
    water_amount: f32,

    #[arg(long)]
    water_temperature: f32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Cli = Cli::parse();

    let starter_feeding = StarterFeeding {
        flour_amount: args.flour_amount,
        starter_amount: args.starter_amount,
        water_amount: args.water_amount,
        water_temperature: args.water_temperature,
    };

    let database_connection: surrealdb::Surreal<surrealdb::engine::remote::ws::Client> =
        database::connection::get_database_client().await?;

    let created: Vec<StarterFeeding> = database_connection
        .query("CREATE starter_feeding_log CONTENT $data")
        .bind(("data", starter_feeding))
        .await?
        .take(0)?;

    let created: StarterFeeding = created.into_iter().next().unwrap();

    println!("Created record: {:?}", created);
    Ok(())
}
