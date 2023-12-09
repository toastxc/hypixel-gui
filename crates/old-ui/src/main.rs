use druid::{AppLauncher, WindowDesc};
use std::sync::Arc;
use tokio::runtime::Runtime;

mod data;
use data::AppState;

mod view;
use view::build_ui;

pub fn main() {
    let main_window = WindowDesc::new(build_ui())
        .title("Hypixel")
        .window_size((400.0, 400.0));

    let initial_state = AppState {
        order_total: String::new(),
        price_diff: String::new(),
        original_data: vec![],
        processed_data: vec![],
        runtime: Arc::new(Runtime::new().unwrap()),
    };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}
