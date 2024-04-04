use crate::{structures::SortBy, MyApp};

use crate::engine::methods::bazaar::ProfitInfo;
use crate::engine::Hypixel;
use std::sync::Arc;

impl MyApp {
    pub fn click_add_find_button(&mut self) {
        let progress = Arc::clone(&self.progress);
        let runtime = Arc::clone(&self.runtime);
        let original_data = Arc::clone(&self.original_data);

        runtime.spawn(async move {
            progress.write().unwrap().set(15.0, 470.0, 0.5, true);
            *original_data.write().unwrap() =
                Hypixel::new().bazaar_profit().await.unwrap_or_default();
            progress.write().unwrap().set_default();
        });
    }

    pub fn calculate(&mut self) {
        let mut polled_data: Vec<ProfitInfo> = self
            .original_data
            .clone()
            .read()
            .unwrap()
            .clone()
            .into_iter()
            .map(|mut a| {
                a.item_name = a.item_name.replace(['-', '_'], " ");
                a
            })
            .filter(|a| {
                // basic profitability filtering
                a.bazaar_buy_price >= self.search.min_buy_val as f32
                    && a.bazaar_sell_price >= self.search.min_sell_val as  f32
                    && a.weekly_buy_orders != 0
                    && a.weekly_sell_orders != 0
                    && a.flip_value != 0.0
                    && a.bazaar_buy_price > a.bazaar_sell_price
                // optional search fields
                && { a.weekly_buy_orders + a.weekly_sell_orders } as i64
                     > self.search.order_total as i64
                && a.flip_value > self.search.profit as f32
                && a.item_name.contains::<&String>(&self.search.name.to_ascii_uppercase())
                    // special case
                && self.search.filter.field.check3(&a.item_name, &self.search.filter.invert)
            })
            .collect();

        match self.search.sort_by.sort_by {
            SortBy::FlipValue => polled_data.sort_by_key(|a| a.flip_value as i32),
            SortBy::WeeklyOrders => {
                polled_data.sort_by_key(|a| (a.weekly_sell_orders, a.weekly_buy_orders))
            }
            SortBy::Az => polled_data.sort_by_key(|a| a.item_name.clone()),
            SortBy::FlipPercentage => polled_data.sort_by_key(|a| a.flip_percentage as i32),
        }

        match (self.search.sort_by.inverted, &self.search.sort_by.sort_by) {
            (true, SortBy::Az) => polled_data.reverse(),
            (false, SortBy::Az) => {}

            (true, _) => {}
            (false, _) => polled_data.reverse(),
        }
        self.processed_data = polled_data;
    }
}
