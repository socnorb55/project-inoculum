use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StarterFeeding {
    pub flour_amount: f32,
    pub starter_amount: f32,
    pub water_amount: f32,
    pub water_temperature: f32,
}
