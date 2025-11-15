use clap::Parser;

mod database;

#[derive(Parser)]
struct Cli {
    test: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Cli = Cli::parse();

    database::connection::get_database_client().await?;

    println!("Hello, {:?}!", args.test);
    Ok(())
}
