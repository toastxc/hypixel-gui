use crate::{ItemProperty, MyApp};
use egui::CollapsingHeader;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.calculate();

            // search fields
            ui.horizontal(|ui| {
                let profit_total_label = ui.label("Profit Margin");
                ui.text_edit_singleline(&mut self.search_fields.profit_total)
                    .labelled_by(profit_total_label.id);
            });

            ui.horizontal(|ui| {
                let order_total_label = ui.label("Orders Total");
                ui.text_edit_singleline(&mut self.search_fields.order_total)
                    .labelled_by(order_total_label.id)
            });

            ui.horizontal(|ui| {
                let name_label = ui.label("Name");
                ui.text_edit_singleline(&mut self.search_fields.name)
                    .labelled_by(name_label.id)
            });

            ui.horizontal(|ui| {
                ui.radio_value(&mut self.search_fields.filter, ItemProperty::Other, "Other");
                ui.radio_value(&mut self.search_fields.filter, ItemProperty::Book, "Book");

                ui.radio_value(
                    &mut self.search_fields.filter,
                    ItemProperty::Enchanted,
                    "Enchanted",
                );

                ui.radio_value(
                    &mut self.search_fields.filter,
                    ItemProperty::EnchantedBlock,
                    "Enchanted Block",
                );
            });

            // UI buttons
            ui.horizontal(|ui| {
                if ui.button("Poll").clicked() {
                    self.click_add_find_button();
                }
            });

            egui::ScrollArea::vertical().show(ui, |ui| {
                for item in &self.processed_data {
                    let text: String = item
                        .item_name
                        .to_ascii_lowercase()
                        .chars()
                        .map(|c| if c == '_' { ' ' } else { c })
                        .collect();

                    CollapsingHeader::new(text).show(ui, |ui| {
                        // bazaar buy price
                        ui.horizontal(|ui| {
                            ui.label("Bazaar Buy Price: ");
                            ui.label(item.bazaar_buy_price.to_string());
                        });

                        // bazaar sell price
                        ui.horizontal(|ui| {
                            ui.label("Bazaar Sell Price: ");
                            ui.label(item.bazaar_sell_price.to_string());
                        });

                        // flip value
                        ui.horizontal(|ui| {
                            ui.label("Flip Value: ");
                            ui.label(item.flip_value.to_string());
                        });

                        // weekly buy orders
                        ui.horizontal(|ui| {
                            ui.label("Weekly Buy Orders: ");
                            ui.label(item.weekly_buy_orders.to_string());
                        });
                        // weekly sell orders
                        ui.horizontal(|ui| {
                            ui.label("Weekly Sell Orders: ");
                            ui.label(item.weekly_sell_orders.to_string());
                        });
                    });
                }
            });
        });
    }
}
