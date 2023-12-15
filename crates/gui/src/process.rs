use crate::{ItemProperty, MyApp, SortBy};
use engine::methods::bazaar::ProfitInfo;
use engine::Hypixel;
use std::sync::Arc;

impl MyApp {
    pub fn click_add_find_button(&self) {
        let runtime = Arc::clone(&self.runtime);
        let original_data = Arc::clone(&self.original_data);
        runtime.spawn(async move {
            let now = std::time::SystemTime::now();

            let a = Hypixel::new().bazaar_profit().await.unwrap();

            let mut write = original_data.write().unwrap();
            write.clear();
            write.extend(a);

            let done = std::time::SystemTime::now().duration_since(now);
            println!("request elapsed: {:?}", done);
        });
    }

    pub fn calculate(&mut self) {
        let polled_data = self.original_data.clone();

        let mut polled_data: Vec<ProfitInfo> = polled_data.read().unwrap().clone()
            .into_iter()
            .filter(|a| {
                // basic profitability filtering
                a.bazaar_buy_price != 0.0
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
                && name_correct(&a.item_name, &self.search_fields.filter)

            })
            .collect();

        match self.search_fields.sort_by.sort_by {
            SortBy::FlipValue => polled_data.sort_by_key(|a| a.flip_value as i32),
            SortBy::WeeklyOrders => {
                polled_data.sort_by_key(|a| (a.weekly_sell_orders, a.weekly_buy_orders))
            }
            SortBy::Az => polled_data.sort_by_key(|a| a.item_name.clone()),
        }
        if self.search_fields.sort_by.inverted {
            polled_data.reverse();
        }
        self.processed_data = polled_data;
    }
}

fn name_correct(name: &str, properties: &ItemProperty) -> bool {
    for check in properties.check() {
        if !name.contains(check.to_ascii_uppercase().as_str()) {
            return false;
        }
    }
    true
}

impl ItemProperty {
    fn check(&self) -> Vec<String> {
        match self {
            ItemProperty::Book => vec!["book"],
            ItemProperty::Enchanted => vec!["enchanted"],
            ItemProperty::EnchantedBlock => vec!["block", "enchanted"],
            ItemProperty::Experience => vec!["experience"],
            ItemProperty::Essence => vec!["essence"],
            ItemProperty::Other => Vec::new(),
        }
        .iter()
        .map(|a| a.to_string())
        .collect()
    }
}
