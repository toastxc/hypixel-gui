use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Stats {
    #[serde(rename = "DEFENSE")]
    pub defense: Option<i64>,
    #[serde(rename = "HEALTH")]
    pub health: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub material: String,
    pub color: Option<String>,
    pub name: String,
    pub category: Option<String>,
    pub tier: Option<String>,
    pub stats: Option<Stats>,
    pub npc_sell_price: Option<f32>,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Items {
    pub success: bool,
    #[serde(rename = "lastUpdated")]
    pub last_updated: i64,
    pub items: Vec<Item>,
}
use std::collections::HashMap;
pub type ItemMap = HashMap<String, Item>;



