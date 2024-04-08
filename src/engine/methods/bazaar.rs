use crate::engine::data::bazaar::{DataResponseBazaarSummary, ProductDataSummary};
use crate::engine::data::item::Item;
use crate::engine::Hypixel;
use reywen_http::{
    self,
    engines::hyper::{Method, Result},
};
use serde::{Deserialize, Serialize};

impl Hypixel {
    // async fn full_bazaar_get(&self) -> Result<DataResponseBazaar> {
    //     self.engine.request(Method::GET, "skyblock/bazaar", None).await
    // }
    pub async fn bazaar_get(&self) -> Result<Vec<ProfitInfo>> {
        Ok(self
            .engine
            .request::<DataResponseBazaarSummary>(Method::GET, "skyblock/bazaar", None)
            .await?
            .products
            .into_values()
            .map(|a| a.into())
            .collect())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ProfitInfo {
    pub item_name: String,
    pub bazaar_buy_price: f32,
    pub bazaar_sell_price: f32,
    pub flip_value: f32,
    pub flip_percentage: f32,
    pub weekly_buy_orders: i32,
    pub weekly_sell_orders: i32,
    pub metadata: Option<Item>,
}

impl ProfitInfo {
    pub fn display(&self) -> String {
        format!("Bazaar Buy Price: {}\nBazaar Sell Price: {}\nFlip Value: {}\nFlip Percentage: {}\nWeekly Buy Orders: {}\nWeekly Sell Orders: {}",
            self.bazaar_buy_price,
            self.bazaar_sell_price,
            self.flip_value,
            self.flip_percentage,
            self.weekly_buy_orders,
            self.weekly_sell_orders
        )
    }
}
impl ProfitInfo {
    pub fn display_new(&self, dp: usize) -> String {
        format!("Bazaar Buy Price: {:.*}\nBazaar Sell Price: {:.*}\nFlip Value: {:.*}\nFlip Percentage: {:.*}\nWeekly Buy Orders: {:.*}\nWeekly Sell Orders: {:.*}",
                dp, self.bazaar_buy_price,
                dp, self.bazaar_sell_price,
                dp, self.flip_value,
                dp, self.flip_percentage,
                dp, self.weekly_buy_orders,
                dp, self.weekly_sell_orders,
        )
    }
}

impl From<ProductDataSummary> for ProfitInfo {
    fn from(value: ProductDataSummary) -> Self {
        Self {
            item_name: value.product_id,
            bazaar_buy_price: value.quick_status.buy_price,
            bazaar_sell_price: value.quick_status.sell_price,
            flip_value: value.quick_status.buy_price - value.quick_status.sell_price,
            flip_percentage: 100.0
                - (value.quick_status.sell_price / value.quick_status.buy_price) * 100.0,
            weekly_buy_orders: value.quick_status.buy_moving_weekly,
            weekly_sell_orders: value.quick_status.sell_moving_week,
            metadata: None,
        }
    }
}
