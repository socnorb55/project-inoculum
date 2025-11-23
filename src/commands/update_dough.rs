use chrono::Utc;
use surrealdb::sql::Datetime;

pub async fn update_dough(
    dough_name: &String,
    updated_status: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    let database_connection: surrealdb::Surreal<surrealdb::engine::remote::ws::Client> =
        crate::database::connection::get_database_client().await?;

    let dough_status_object: crate::database::models::DoughStatusObject =
        crate::database::models::DoughStatusObject {
            dough_name: dough_name.clone(),
            dough_status: updated_status.parse()?,
            timestamp: Datetime::from(Utc::now()),
        };

    let created: Vec<crate::database::models::DoughStatusObject> = database_connection
        .query("CREATE dough_statuses CONTENT $data")
        .bind(("data", dough_status_object))
        .await?
        .take(0)?;

    let status: crate::database::models::DoughStatus = updated_status.parse()?;

    let updated: Vec<crate::database::models::Dough> = database_connection
    .query("UPDATE doughs SET status = $status, update_timestamp = time::now() WHERE name = $name RETURN AFTER;")
    .bind(("status", status))
    .bind(("name", dough_name.clone()))
    .await?
    .take(0)?;

    println!("Created dough status: {:?}", created);
    println!("Updated dough status: {:?}", updated);

    Ok(())
}
