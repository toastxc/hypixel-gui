use crate::engine::methods::bazaar::ProfitInfo;
use crate::structures::{ItemPropertyF, SortBy, SortInfo};
use crate::MyApp;
use eframe::egui::Context;
use eframe::{Frame, Storage};
use egui::{CollapsingHeader, CollapsingResponse, ComboBox, ProgressBar, Slider, Ui};
use material_egui::MaterialColors;
use std::ops::RangeInclusive;
use std::time::Duration;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        if self.first_run {
            if let Some(storage) = _frame.storage() {
                storage
                    .get_string("dark")
                    .map(|a| a.parse::<bool>().map(|dark| self.is_dark = dark));
                if let Some(name) = storage.get_string("search") {
                    self.search.name = name
                }
            };

            self.click_add_find_button();

        };

        MaterialColors::new("#448EDF".to_string(), self.is_dark, 1.25)
            .apply_zoom(ctx, &mut self.first_run);
        egui::CentralPanel::default().show(ctx, |ui| update_fn(self, ui));
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }
    fn auto_save_interval(&self) -> Duration {
        Duration::from_secs(5)
    }
    fn save(&mut self, _storage: &mut dyn Storage) {
        _storage.set_string("dark", self.is_dark.to_string());
        _storage.set_string("search", self.search.name.clone());
    }
}

fn update_fn(value: &mut MyApp, ui: &mut Ui) {
    value.calculate();

    ui.checkbox(&mut value.is_dark, "Dark theme");

    search_field(ui, "Name", &mut value.search.name);
    slider(ui, "Profit Margin", 250000000, &mut value.search.profit);
    slider(ui, "Dp", 6, &mut value.search.dp);
    slider(ui, "Sell Value", 280000000, &mut value.search.min_sell_val);
    slider(ui, "Buy Value", 300000000, &mut value.search.min_buy_val);

    container_filter(
        ui,
        &mut value.search.filter.field,
        &mut value.search.filter.invert,
    );

    sort_combo_box(&mut value.search.sort_by, ui);

    // UI buttons
    ui.horizontal(|ui| {
        if ui.button("Poll").clicked() {
            value.click_add_find_button();
        }
    });

    ui.add(
        ProgressBar::new(value.progress.read().unwrap().percentage)
            .desired_height(value.progress.read().unwrap().height)
            .desired_width(value.progress.read().unwrap().width)
            .animate(value.progress.read().unwrap().animated),
    );

    ui.separator();

    egui::ScrollArea::vertical()
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            for (index, item) in value.processed_data.iter().enumerate() {
                product_container(item, ui, value.search.dp, (index % 2) != 0);
            }
        });
}

fn sort_combo_box(sort: &mut SortInfo, ui: &mut Ui) {
    ui.horizontal(|ui| {
        ComboBox::from_label("Sort By")
            .selected_text(String::from(&sort.sort_by.to_string()))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut sort.sort_by, SortBy::Az, "A-Z");
                ui.selectable_value(&mut sort.sort_by, SortBy::FlipValue, "Flip Value");
                ui.selectable_value(&mut sort.sort_by, SortBy::WeeklyOrders, "Weekly Orders");
                ui.selectable_value(&mut sort.sort_by, SortBy::FlipPercentage, "Flip Percentage");
            });
        ui.checkbox(&mut sort.inverted, "Inverted");
    });
}

fn search_field(ui: &mut Ui, label: &str, field: &mut String) {
    ui.horizontal(|ui| {
        let profit_total_label = ui.label(label);
        ui.text_edit_singleline(field)
            .labelled_by(profit_total_label.id);
    });
}

fn container_filter(ui: &mut Ui, filter: &mut ItemPropertyF, invert: &mut bool) {
    ui.horizontal(|ui| {
        ComboBox::from_label("Filter")
            .selected_text(filter.to_string())
            .show_ui(ui, |ui| {
                ui.selectable_value(filter, ItemPropertyF::Other, "None");
                ui.selectable_value(filter, ItemPropertyF::Experience, "Experience");
                ui.selectable_value(filter, ItemPropertyF::EnchantedBlock, "EnchantedBlock");
                ui.selectable_value(filter, ItemPropertyF::Book, "Book");
                ui.selectable_value(filter, ItemPropertyF::Enchantment, "Enchantment");
                ui.selectable_value(filter, ItemPropertyF::Enchanted, "Enchanted");
                ui.selectable_value(filter, ItemPropertyF::Essence, "Essence");
            });
        ui.checkbox(invert, "Inverted");
    });
}

pub fn product_container(
    inner: &ProfitInfo,
    ui: &mut Ui,
    dp: u32,
    background: bool,
) -> CollapsingResponse<()> {
    CollapsingHeader::new(
        inner
            .item_name
            .to_ascii_lowercase()
            .chars()
            .map(|c| if c == '_' { ' ' } else { c })
            .collect::<String>(),
    )
    .show_background(background)
    .show(ui, |ui| {
        ui.label(inner.display_new(dp as usize));
    })
}

fn slider(ui: &mut Ui, label: &str, max: u32, value: &mut u32) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.add(Slider::new(value, RangeInclusive::new(u32::MIN, max)).logarithmic(true));
    });
}
