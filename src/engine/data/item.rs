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
    pub tier: Option<TierEnum>,
    pub stats: Option<Stats>,
    pub npc_sell_price: Option<f32>,
    pub id: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TierEnum {
    COMMON,
    UNCOMMON,
    RARE,
    EPIC,
    LEGENDARY,
    MYTHIC,
    SUPREME,
    SPECIAL,
    VERY_SPECIAL,
    UNOBTAINABLE,
}
impl Display for TierEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            TierEnum::COMMON => "Common",
            TierEnum::UNCOMMON => "Uncommon",
            TierEnum::RARE => "Rare",
            TierEnum::EPIC => "Epic",
            TierEnum::LEGENDARY => "Legendary",
            TierEnum::MYTHIC => "Mythic",
            TierEnum::SUPREME => "Supreme",
            TierEnum::SPECIAL => "Special",
            TierEnum::VERY_SPECIAL => "Very Special",
            TierEnum::UNOBTAINABLE => "UNOBTAINABLE",
        }
        .to_string();
        write!(f, "{}", str)
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Items {
    pub success: bool,
    #[serde(rename = "lastUpdated")]
    pub last_updated: i64,
    pub items: Vec<Item>,
}
use std::collections::HashMap;
use std::fmt::Display;

pub type ItemMap = HashMap<String, Item>;
