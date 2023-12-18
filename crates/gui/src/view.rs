use egui::{CollapsingHeader, CollapsingResponse, ComboBox, ProgressBar, Slider, Ui, Visuals};
use std::ops::RangeInclusive;

use crate::structures::{ItemPropertyF, SortBy, SortInfo};
use crate::MyApp;
use engine::methods::bazaar::ProfitInfo;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.is_dark {
            ctx.set_visuals(Visuals::dark());
        } else {
            ctx.set_visuals(Visuals::light());
        }

        egui::CentralPanel::default().show(ctx, |ui| update_fn(self, ui));
    }
}

fn update_fn(value: &mut MyApp, ui: &mut Ui) {
    value.calculate();
    ui.checkbox(&mut value.is_dark, "Dark theme");

    search_field(ui, "Profit Margin", &mut value.search_fields.profit_total);
    search_field(ui, "Orders Total", &mut value.search_fields.order_total);
    search_field(ui, "Name", &mut value.search_fields.name);

    ui.horizontal(|ui| {
        ui.label("Dp");
        ui.add(Slider::new(
            &mut value.search_fields.dp,
            RangeInclusive::new(0, 6),
        ));
    });

    container_filter(
        ui,
        &mut value.search_fields.filter.field,
        &mut value.search_fields.filter.invert,
    );

    sort_combo_box(&mut value.search_fields.sort_by, ui);

    // UI buttons
    ui.horizontal(|ui| {
        if ui.button("Poll").clicked() {
            value.click_add_find_button();
        }
    });

    ui.add(
        ProgressBar::new(value.progress.read().unwrap().percentage)
            .desired_height(value.progress.read().unwrap().height)
            .desired_width(value.progress.read().unwrap().width),
    );

    ui.separator();

    egui::ScrollArea::vertical()
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            for (index, item) in value.processed_data.iter().enumerate() {
                product_container(item, ui, value.search_fields.dp, (index % 2) != 0);
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
