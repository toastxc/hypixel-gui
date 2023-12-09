use crate::data::*;
use druid::widget::List;
use druid::{
    widget::Scroll,
    widget::{Button, Flex, Label, TextBox},
    Data, Env, LensExt, Widget, WidgetExt,
};
use engine::methods::bazaar::ProfitInfo;

pub fn build_ui() -> impl Widget<AppState> {
    let mut flex = Flex::column();

    flex.add_spacer(20.0);
    flex.add_child(Label::new("order total"));
    flex.add_child(textbox_order_total("100"));
    flex.add_spacer(40.0);

    flex.add_child(Label::new("price difference"));
    flex.add_child(textbox_price_difference("20"));
    flex.add_spacer(20.0);

    flex.add_child(Button::new("Poll").on_click(AppState::click_add_find_button));

    flex.add_child(Button::new("Calculate").on_click(AppState::click_calculate_button));

  //  let list = List::new(list_textbox).lens(AppState::processed_data);

    let mut scroll = Scroll::new(flex);
    scroll.set_horizontal_scroll_enabled(false);

    scroll
}


fn textbox_order_total(placeholder: impl Into<String>) -> impl Widget<AppState> {
    let new_todo_textbox = TextBox::new()
        .with_placeholder(placeholder.into())
        .expand_width()
        .lens(AppState::order_total);

    Flex::row().with_flex_child(new_todo_textbox, 1.)
}

fn textbox_price_difference(placeholder: impl Into<String>) -> impl Widget<AppState> {
    let new_todo_textbox = TextBox::new()
        .with_placeholder(placeholder.into())
        .expand_width()
        .lens(AppState::price_diff);

    Flex::row().with_flex_child(new_todo_textbox, 1.)
}
