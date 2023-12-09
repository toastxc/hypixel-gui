use crate::{ItemProperty, MyApp};
use engine::methods::bazaar::ProfitInfo;
use engine::Hypixel;
use std::sync::{mpsc, Arc};

impl MyApp {
    pub fn click_add_find_button<'a>(&'a self) {
      //  let (sender, receiver) = mpsc::channel();

        let runtime = Arc::clone(&self.runtime);
        let original_data = Arc::clone(&self.original_data);
        runtime.spawn(async move {
            let now = std::time::SystemTime::now();

            let a = Hypixel::new().bazaar_profit().await.unwrap();

            let mut write = original_data.write().unwrap();
            write.clear();
            write.extend(a.into_iter());

            // sender.send(a).unwrap();

            let done = std::time::SystemTime::now().duration_since(now);
            println!("request elapsed: {:?}", done);
        });

        //  self.original_data = receiver.recv().unwrap();
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
                && { a.weekly_buy_orders + a.weekly_sell_orders }
                     > self.search_fields.order_total.parse().unwrap_or_default()
                && a.flip_value > self.search_fields.profit_total.parse().unwrap_or_default()
                && a.item_name.contains::<&String>(&self.search_fields.name.to_ascii_uppercase().chars()
                    .map(|c| if c == '_' { ' ' } else { c })
                    .collect())
                    // special case
                && {
                    if self.search_fields.filter.is_block() {

                        a.item_name.contains("BLOCK") && a.item_name.contains("ENCHANTED")

                    }else {
                        a.item_name.contains(&self.search_fields.filter.to_string().to_ascii_uppercase())
                    }
                }
            })
            .collect();

        // sorted
        polled_data.sort_by_key(|item| {
            (item.flip_value as i32, {
                item.weekly_buy_orders + item.weekly_sell_orders
            })
        });
        self.processed_data = polled_data;
    }
}

impl ToString for ItemProperty {
    fn to_string(&self) -> String {
        match self {
            ItemProperty::Book => "book",
            ItemProperty::Enchanted => "enchanted",
            ItemProperty::EnchantedBlock => "block",
            ItemProperty::Experience => "experience",
            ItemProperty::Essence => "essence",
            ItemProperty::Other => "",
        }
        .to_string()
    }
}
