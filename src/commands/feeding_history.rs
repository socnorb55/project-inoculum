use comfy_table::{ContentArrangement, Table, presets::UTF8_FULL};
use serde_json::Value;
use std::collections::BTreeSet;

pub async fn feeding_history(
    maximum_results: Option<i32>,
) -> Result<(), Box<dyn std::error::Error>> {
    let database_connection: surrealdb::Surreal<surrealdb::engine::remote::ws::Client> =
        crate::database::connection::get_database_client().await?;

    let records: Vec<crate::database::models::StarterFeeding> =
        match maximum_results.filter(|value| *value > 0) {
            Some(limit) => {
                let mut response = database_connection
                    .query("SELECT * FROM starter_feeding_log ORDER BY timestamp DESC LIMIT $limit")
                    .bind(("limit", limit))
                    .await?;

                response.take(0)?
            }
            None => {
                let mut response = database_connection
                    .query("SELECT * FROM starter_feeding_log ORDER BY timestamp DESC")
                    .await?;

                response.take(0)?
            }
        };

    if records.is_empty() {
        println!("No feeding events found.");
        return Ok(());
    }

    let json_records: Vec<Value> = records
        .iter()
        .map(serde_json::to_value)
        .collect::<Result<_, _>>()?;

    let mut keys: BTreeSet<String> = BTreeSet::new();
    for v in &json_records {
        if let Value::Object(map) = v {
            for k in map.keys() {
                keys.insert(k.clone());
            }
        }
    }
    let headers: Vec<String> = keys.into_iter().collect();

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(headers.clone());

    for value in json_records {
        let mut row: Vec<String> = Vec::with_capacity(headers.len());
        for key in &headers {
            let cell = match &value {
                Value::Object(map) => map.get(key).unwrap_or(&Value::Null),
                _ => &Value::Null,
            };
            let string = match cell {
                Value::Null => String::new(),
                Value::String(string) => string.clone(),
                Value::Number(number) => {
                    if let Some(f) = number.as_f64() {
                        format!("{:.2}", f)
                    } else {
                        number.to_string()
                    }
                }
                Value::Bool(boolean) => boolean.to_string(),
                other => other.to_string(),
            };
            row.push(string);
        }
        table.add_row(row);
    }

    println!("{table}");

    Ok(())
}
