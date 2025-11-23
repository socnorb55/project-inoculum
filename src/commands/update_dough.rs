use chrono::Utc;
use surrealdb::sql::Datetime;

pub async fn update_dough(
    dough_name: &str,
    updated_status: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let database_connection: surrealdb::Surreal<surrealdb::engine::remote::ws::Client> =
        crate::database::connection::get_database_client().await?;

    let status: crate::database::models::DoughStatus = updated_status.parse()?;

    let dough_status_object: crate::database::models::DoughStatusObject =
        crate::database::models::DoughStatusObject {
            dough_name: dough_name.to_string(),
            dough_status: status.clone(),
            timestamp: Datetime::from(Utc::now()),
        };

    let created: Vec<crate::database::models::DoughStatusObject> = database_connection
        .query("CREATE dough_statuses CONTENT $data")
        .bind(("data", dough_status_object))
        .await?
        .take(0)?;

    let updated: Vec<crate::database::models::Dough> = database_connection
    .query("UPDATE doughs SET status = $status, update_timestamp = time::now() WHERE name = $name RETURN AFTER;")
    .bind(("status", status))
    .bind(("name", dough_name.to_string()))
    .await?
    .take(0)?;

    println!("Created dough status: {:?}", created);
    println!("Updated dough status: {:?}", updated);

    Ok(())
}
