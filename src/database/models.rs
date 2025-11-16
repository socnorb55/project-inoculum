use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

#[derive(Debug, Serialize, Deserialize)]
pub struct StarterFeeding {
    pub flour_amount: f32,
    pub starter_amount: f32,
    pub timestamp: Datetime,
    pub water_amount: f32,
    pub water_temp: f32,
}
