pub async fn end_dough(name: &String, notes: &String) -> Result<(), Box<dyn std::error::Error>> {
    let database_connection: surrealdb::Surreal<surrealdb::engine::remote::ws::Client> =
        crate::database::connection::get_database_client().await?;

    let doughs: Vec<crate::database::models::Dough> = database_connection
        .query("SELECT * FROM doughs WHERE name = $name")
        .bind(("name", name.clone()))
        .await?
        .take(0)?;

    let dough = doughs
        .into_iter()
        .next()
        .ok_or("No dough found with that name")?;

    let ended: Vec<crate::database::models::Dough> = database_connection
        .query("UPDATE doughs SET notes = $notes, status = $status, update_timestamp = time::now() WHERE name = $name RETURN AFTER;")
        .bind(("status", crate::database::models::DoughStatus::Ended))
        .bind(("name", name.clone()))
        .bind(("notes", notes.clone()))
        .await?
        .take(0)?;

    println!("Ended dough: {:?}", ended);
    Ok(())
}
