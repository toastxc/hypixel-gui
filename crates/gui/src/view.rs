use crate::{ItemProperty, MyApp, SortBy};
use egui::{CollapsingResponse, ComboBox, Ui};

use engine::methods::bazaar::ProfitInfo;

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
                ui.radio_value(
                    &mut self.search_fields.filter,
                    ItemProperty::Experience,
                    "Experience",
                );

                ui.radio_value(
                    &mut self.search_fields.filter,
                    ItemProperty::Essence,
                    "Essence",
                )
            });

            // combo

            ui.horizontal(|ui| {
                ComboBox::from_label("Sort By")
                    .selected_text(String::from(
                        &self.search_fields.sort_by.sort_by.to_string(),
                    ))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.search_fields.sort_by.sort_by,
                            SortBy::Az,
                            "A-Z",
                        );
                        ui.selectable_value(
                            &mut self.search_fields.sort_by.sort_by,
                            SortBy::FlipValue,
                            "Flip Value",
                        );
                        ui.selectable_value(
                            &mut self.search_fields.sort_by.sort_by,
                            SortBy::WeeklyOrders,
                            "Weekly Orders",
                        );
                    });

                ui.checkbox(&mut self.search_fields.sort_by.inverted, "Inverted");
            });

            // UI buttons
            ui.horizontal(|ui| {
                if ui.button("Poll").clicked() {
                    self.click_add_find_button();
                }
            });

            ui.separator();

            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    for item in &self.processed_data {
                        container(item, ui);
                    }
                })
        });
    }
}

pub fn container(inner: &ProfitInfo, ui: &mut Ui) -> CollapsingResponse<()> {
    ui.collapsing(
        inner
            .item_name
            .to_ascii_lowercase()
            .chars()
            .map(|c| if c == '_' { ' ' } else { c })
            .collect::<String>(),
        |ui| {
            ui.label(inner.display());
        },
    )
}
