use crate::data::bazaar::{DataResponseBazaarSummary, ProductDataSummary};
use crate::{data::bazaar::DataResponseBazaar, Hypixel};
use druid::{Data, Lens};
use reywen_http::{
    self,
    engines::hyper::{Method, Result},
};
use serde::{Deserialize, Serialize};

impl Hypixel {
    pub async fn bazaar_get(&self) -> Result<DataResponseBazaar> {
        self.engine.request(Method::GET, "bazaar", None).await
    }

    pub async fn baazar_aggregate(&self) -> Result<Vec<DataInti>> {
        let data = self.bazaar_get().await?;

        // item_name | flip_value | quantity
        let mut sort_list: Vec<DataInti> = Vec::new();

        for item in data.products {
            sort_list.push(DataInti {
                item_name: item.0,
                bazaar_buy_price: item.1.quick_status.buy_price,
                bazaar_sell_price: item.1.quick_status.sell_price,
                weekly_buy_orders: item.1.quick_status.buy_moving_weekly,
                weekly_sell_orders: item.1.quick_status.sell_moving_week,
            })
        }

        Ok(sort_list)
    }

    pub async fn bazaar_profit(&self) -> Result<Vec<ProfitInfo>> {
        let data: DataResponseBazaarSummary =
            self.engine.request(Method::GET, "bazaar", None).await?;

        let mut data: Vec<ProfitInfo> = data.products.into_values().map(|a| a.into()).collect();

        // sorted by flip value and quickbuy + quicksell
        data.sort_by_key(|item| {
            (item.flip_value as i32, {
                item.weekly_buy_orders + item.weekly_sell_orders
            })
        });

        Ok(data)
    }
}

pub struct DataInti {
    // name
    pub item_name: String,

    // pricing
    pub bazaar_buy_price: f32,
    pub bazaar_sell_price: f32,

    // market wants
    pub weekly_buy_orders: i32,
    pub weekly_sell_orders: i32,
}

impl From<DataInti> for ProfitInfo {
    fn from(value: DataInti) -> Self {
        ProfitInfo {
            item_name: value.item_name,
            bazaar_buy_price: value.bazaar_buy_price,
            bazaar_sell_price: value.bazaar_sell_price,
            flip_value: { value.bazaar_buy_price - value.bazaar_sell_price },
            weekly_buy_orders: value.weekly_buy_orders,
            weekly_sell_orders: value.weekly_sell_orders,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Data, Lens)]
pub struct ProfitInfo {
    pub item_name: String,

    pub bazaar_buy_price: f32,
    pub bazaar_sell_price: f32,

    pub flip_value: f32,

    pub weekly_buy_orders: i32,
    pub weekly_sell_orders: i32,
}

impl From<ProductDataSummary> for ProfitInfo {
    fn from(value: ProductDataSummary) -> Self {
        Self {
            item_name: value.product_id,
            bazaar_buy_price: value.quick_status.buy_price,
            bazaar_sell_price: value.quick_status.sell_price,
            flip_value: { value.quick_status.buy_price - value.quick_status.sell_price },
            weekly_buy_orders: value.quick_status.buy_moving_weekly,
            weekly_sell_orders: value.quick_status.sell_moving_week,
        }
    }
}
