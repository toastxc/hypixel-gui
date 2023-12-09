use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]

pub struct DataResponseBazaar {
    pub success: bool,
    #[serde(rename = "lastUpdated")]
    pub last_updated: i64,
    pub products: HashMap<String, ProductData>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProductData {
    pub product_id: String,
    pub sell_summary: Vec<DataSellBuySummary>,
    pub buy_summary: Vec<DataSellBuySummary>,
    pub quick_status: DataQuickStatus,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DataSellBuySummary {
    pub amount: i32,
    #[serde(rename = "pricePerUnit")]
    pub price_per_unit: f32,
    pub orders: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DataQuickStatus {
    #[serde(rename = "productId")]
    pub product_id: String,
    #[serde(rename = "sellPrice")]
    pub sell_price: f32,
    #[serde(rename = "sellVolume")]
    pub sell_volume: i32,
    #[serde(rename = "sellMovingWeek")]
    pub sell_moving_week: i32,
    #[serde(rename = "sellOrders")]
    pub sell_orders: i32,
    #[serde(rename = "buyPrice")]
    pub buy_price: f32,
    #[serde(rename = "buyVolume")]
    pub buy_volume: i32,
    #[serde(rename = "buyMovingWeek")]
    pub buy_moving_weekly: i32,
    #[serde(rename = "buyOrders")]
    pub buy_orders: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]

pub struct DataResponseBazaarSummary {
    pub products: HashMap<String, ProductDataSummary>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProductDataSummary {
    pub product_id: String,
    pub quick_status: DataQuickStatus,
}
