use chrono::Utc;
use surrealdb::sql::Datetime;

pub async fn log_feed(
    flour_amount: f32,
    starter_amount: f32,
    water_amount: f32,
    water_temp: f32,
) -> Result<(), Box<dyn std::error::Error>> {
    let starter_feeding: crate::database::models::StarterFeeding =
        crate::database::models::StarterFeeding {
            flour_amount,
            starter_amount,
            timestamp: Datetime::from(Utc::now()), // changed
            water_amount,
            water_temp,
        };

    let database_connection: surrealdb::Surreal<surrealdb::engine::remote::ws::Client> =
        crate::database::connection::get_database_client().await?;

    let created: Vec<crate::database::models::StarterFeeding> = database_connection
        .query("CREATE starter_feeding_log CONTENT $data")
        .bind(("data", starter_feeding))
        .await?
        .take(0)?;

    println!("Created record: {:?}", created);

    Ok(())
}
