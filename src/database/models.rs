use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Dough {
    pub fat: f32,
    pub flour: f32,
    pub hydration: f32,
    pub leaven: f32,
    pub name: String,
    pub salt: f32,
    pub start_timestamp: Datetime,
    pub status: DoughStatus,
    pub sugar: f32,
    pub total_weight: f32,
    pub update_timestamp: Datetime,
    pub water: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DoughStatus {
    BulkProofing,
    Cooking,
    Created,
    SecondaryProofing,
    Shaping,
}

impl std::str::FromStr for DoughStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "bulkproofing" => Ok(DoughStatus::BulkProofing),
            "cooking" => Ok(DoughStatus::Cooking),
            "created" => Ok(DoughStatus::Created),
            "secondaryproofing" => Ok(DoughStatus::SecondaryProofing),
            "shaping" => Ok(DoughStatus::Shaping),
            _ => Err(format!("Invalid dough status: {}", s)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DoughStatusObject {
    pub dough_name: String,
    pub dough_status: DoughStatus,
    pub timestamp: Datetime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    pub fat: Option<f32>,
    pub flour: f32,
    pub leaven: f32,
    pub name: String,
    pub salt: f32,
    pub servings: i32,
    pub sugar: Option<f32>,
    pub timestamp: Datetime,
    pub update_timestamp: Datetime,
    pub water: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StarterFeeding {
    pub flour_amount: f32,
    pub starter_amount: f32,
    pub timestamp: Datetime,
    pub water_amount: f32,
    pub water_temp: f32,
}