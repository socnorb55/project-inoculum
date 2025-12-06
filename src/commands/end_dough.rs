use chrono::Utc;
use surrealdb::sql::Datetime;

pub async fn end_dough(name: &str, notes: &str) -> Result<(), Box<dyn std::error::Error>> {
    let database_connection: surrealdb::Surreal<surrealdb::engine::remote::ws::Client> =
        crate::database::connection::get_database_client().await?;

    let dough_status_object: crate::database::models::DoughStatusObject =
        crate::database::models::DoughStatusObject {
            dough_name: name.to_string(),
            dough_status: crate::database::models::DoughStatus::Ended,
            timestamp: Datetime::from(Utc::now()),
        };

    let created: Vec<crate::database::models::DoughStatusObject> = database_connection
        .query("CREATE dough_statuses CONTENT $data")
        .bind(("data", dough_status_object))
        .await?
        .take(0)?;

    let ended: Vec<crate::database::models::Dough> = database_connection
        .query("UPDATE doughs SET notes = $notes, status = $status, update_timestamp = time::now() WHERE name = $name RETURN AFTER;")
        .bind(("status", crate::database::models::DoughStatus::Ended))
        .bind(("name", name.to_string()))
        .bind(("notes", notes.to_string()))
        .await?
        .take(0)?;

    println!("Created dough status: {:?}", created);
    println!("Ended dough status: {:?}", ended);

    Ok(())
}
