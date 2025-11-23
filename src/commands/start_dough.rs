use chrono::Utc;
use surrealdb::sql::Datetime;

pub async fn create_dough(
    database_connection: surrealdb::Surreal<surrealdb::engine::remote::ws::Client>,
    fat: f32,
    flour: f32,
    hydration: f32,
    leaven: f32,
    name: String,
    salt: f32,
    sugar: f32,
    total_weight: f32,
    water: f32,
) -> Result<(), Box<dyn std::error::Error>> {
    let dough = crate::database::models::Dough {
        fat,
        flour,
        hydration,
        leaven,
        name,
        salt,
        start_timestamp: Datetime::from(Utc::now()),
        status: crate::database::models::DoughStatus::BulkProofing,
        sugar,
        total_weight,
        update_timestamp: Datetime::from(Utc::now()),
        water,
    };
    let created: Vec<crate::database::models::Dough> = database_connection
        .query("CREATE doughs CONTENT $data")
        .bind(("data", dough))
        .await?
        .take(0)?;

    println!("Created dough: {:?}", created);

    Ok(())
}

pub async fn start_dough(
    fat: Option<f32>,
    flour: Option<f32>,
    leaven: Option<f32>,
    name: &String,
    recipe: &Option<String>,
    salt: Option<f32>,
    scale: Option<f32>,
    sugar: Option<f32>,
    water: Option<f32>,
) -> Result<(), Box<dyn std::error::Error>> {
    let database_connection: surrealdb::Surreal<surrealdb::engine::remote::ws::Client> =
        crate::database::connection::get_database_client().await?;

    if recipe.is_some() {
        if scale.is_none() {
            println!("Scale (--scale) is a required field if using a recipe");
            return Ok(());
        } else {
            let scale_value = scale.unwrap();

            let recipe: Option<crate::database::models::Recipe> = database_connection
                .query("SELECT * FROM recipes WHERE name = $name")
                .bind(("name", recipe.as_ref().unwrap().clone()))
                .await?
                .take(0)?;

            let recipe_data = match recipe {
                Some(data) => data,
                None => {
                    println!("Recipe not found");
                    return Ok(());
                }
            };

            let fat_value = recipe_data.fat.unwrap_or(0.0) * scale_value;
            let flour_value = recipe_data.flour * scale_value;
            let leaven_value = recipe_data.leaven * scale_value;
            let salt_value = recipe_data.salt * scale_value;
            let sugar_value = recipe_data.sugar.unwrap_or(0.0) * scale_value;
            let water_value = recipe_data.water * scale_value;

            if flour_value <= 0.0 {
                println!("Error: Flour value must be greater than zero to calculate hydration.");
                return Ok(());
            }
            let hydration = (water_value / flour_value) * 100.0;
            let total_weight =
                fat_value + flour_value + leaven_value + salt_value + sugar_value + water_value;

            return create_dough(
                database_connection,
                fat_value,
                flour_value,
                hydration,
                leaven_value,
                name.to_string(),
                salt_value,
                sugar_value,
                total_weight,
                water_value,
            )
            .await;
        }
    } else {
        let numeric_params: &[(&str, Option<f32>)] = &[
            ("flour", flour),
            ("leaven", leaven),
            ("salt", salt),
            ("water", water),
        ];

        let missing_required_ingredients: Vec<&str> = numeric_params
            .iter()
            .filter(|(_, opt)| opt.is_none())
            .map(|(name, _)| *name)
            .collect();

        if !missing_required_ingredients.is_empty() {
            println!(
                "Missing required ingredients: {:?}",
                missing_required_ingredients
            );

            return Ok(());
        } else {
            let fat_value = fat.unwrap_or(0.0);
            let flour_value = flour.unwrap();
            let leaven_value = leaven.unwrap();
            let salt_value = salt.unwrap();
            let sugar_value = sugar.unwrap_or(0.0);
            let water_value = water.unwrap();

            if flour_value <= 0.0 {
                println!("Invalid value for flour: must be greater than zero.");
                return Ok(());
            }
            let hydration = (water_value / flour_value) * 100.0;
            let total_weight =
                fat_value + flour_value + leaven_value + salt_value + sugar_value + water_value;

            return create_dough(
                database_connection,
                fat_value,
                flour_value,
                hydration,
                leaven_value,
                name.to_string(),
                salt_value,
                sugar_value,
                total_weight,
                water_value,
            )
            .await;
        }
    }
}
