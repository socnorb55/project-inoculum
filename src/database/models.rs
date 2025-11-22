use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

#[derive(Debug, Serialize, Deserialize)]
pub struct StarterFeeding {
    pub flour_amount: f32,
    pub id: String,
    pub starter_amount: f32,
    pub timestamp: Datetime,
    pub water_amount: f32,
    pub water_temp: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dough {
    pub fat: Option<f32>,
    pub flour: f32,
    pub hydration: f32,
    pub id: String,
    pub leaven: f32,
    pub name: String,
    pub salt: f32,
    pub status: DoughStatus,
    pub sugar: Option<f32>,
    pub timestamp: Datetime,
    pub total_weight: f32,
    pub water: f32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum DoughStatus {
    BulkProofing,
    Cooking,
    Created,
    SecondaryProofing,
    Shaping,
}
