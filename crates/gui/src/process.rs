use crate::{structures::SortBy, MyApp};
use engine::{methods::bazaar::ProfitInfo, Hypixel};
use std::sync::Arc;

impl MyApp {
    pub fn click_add_find_button(&mut self) {
        let progress = Arc::clone(&self.progress);
        let runtime = Arc::clone(&self.runtime);
        let original_data = Arc::clone(&self.original_data);

        runtime.spawn(async move {
            progress.write().unwrap().set(10.0, 470.0, 0.5);

            let new_data = Hypixel::new().bazaar_profit().await.unwrap();

            original_data.write().unwrap().clear();
            original_data.write().unwrap().extend(new_data);
            progress.write().unwrap().set_default();
        });
    }

    pub fn calculate(&mut self) {
        let polled_data = self.original_data.clone();

        let mut polled_data: Vec<ProfitInfo> = polled_data.read().unwrap().clone()
            .into_iter()
            .filter(|a| {
                // basic profitability filtering
                a.bazaar_buy_price != 0.0
                    && a.bazaar_sell_price != 0.0
                    && a.weekly_buy_orders != 0
                    && a.bazaar_buy_price > a.bazaar_sell_price
                // optional search fields
                && { a.weekly_buy_orders + a.weekly_sell_orders } as i64
                     > self.search_fields.order_total.parse::<i64>().unwrap_or_default()
                && a.flip_value > self.search_fields.profit_total.parse().unwrap_or_default()
                && a.item_name.contains::<&String>(&self.search_fields.name.to_ascii_uppercase().chars()
                    .map(|c| if c == '_' { ' ' } else { c })
                    .collect())
                    // special case
              //  && name_correct(&a.item_name, &self.search_fields.filter.field, self.search_fields.filter.invert)
                && self.search_fields.filter.field.check2(&a.item_name)

            })
            .collect();

        match self.search_fields.sort_by.sort_by {
            SortBy::FlipValue => polled_data.sort_by_key(|a| a.flip_value as i32),
            SortBy::WeeklyOrders => {
                polled_data.sort_by_key(|a| (a.weekly_sell_orders, a.weekly_buy_orders))
            }
            SortBy::Az => polled_data.sort_by_key(|a| a.item_name.clone()),
            SortBy::FlipPercentage => polled_data.sort_by_key(|a| a.flip_percentage as i32),
        }

        match (
            self.search_fields.sort_by.inverted,
            &self.search_fields.sort_by.sort_by,
        ) {
            // sorted
            (false, SortBy::FlipValue) => polled_data.reverse(),
            (false, SortBy::FlipPercentage) => polled_data.reverse(),

            // standard handling
            (true, _) => polled_data.reverse(),
            (false, _) => {}
        }
        self.processed_data = polled_data;
    }
}
