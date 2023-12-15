use eframe::{HardwareAcceleration, Renderer, Theme};
use engine::methods::bazaar::ProfitInfo;
use std::sync::{Arc, RwLock};
use tokio::runtime::Runtime;

pub mod process;
pub mod view;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([480.0, 480.0]),
        vsync: true,
        hardware_acceleration: HardwareAcceleration::Preferred,
        renderer: Renderer::Glow,
        follow_system_theme: true,
        default_theme: Theme::Dark,
        centered: false,
        ..Default::default()
    };
    eframe::run_native(
        "Money Machine",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

#[derive(Debug, Clone)]
struct MyApp {
    pub search_fields: SearchFields,
    pub runtime: Arc<Runtime>,
    pub original_data: Arc<RwLock<Vec<ProfitInfo>>>,
    pub processed_data: Vec<ProfitInfo>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            runtime: Arc::new(Runtime::new().unwrap()),
            search_fields: Default::default(),
            processed_data: vec![],
            original_data: Arc::new(RwLock::new(Vec::new())),
        }
    }
}
#[derive(Debug, Clone, Default)]
pub struct SearchFields {
    profit_total: String,
    order_total: String,
    name: String,
    filter: ItemProperty,
    sort_by: SortInfo,
}
#[derive(Debug, Clone, Default)]
pub struct SortInfo {
    pub sort_by: SortBy,
    pub inverted: bool,
}
#[derive(Debug, PartialEq, Clone, Default)]
pub enum SortBy {
    #[default]
    FlipValue,
    WeeklyOrders,
    Az,
}

impl ToString for SortBy {
    fn to_string(&self) -> String {
        match self {
            SortBy::FlipValue => "Flip Value",
            SortBy::WeeklyOrders => "Weekly Orders",
            SortBy::Az => "A-Z",
        }
        .to_string()
    }
}
#[derive(Debug, Clone, Default, PartialEq)]
pub enum ItemProperty {
    Book,
    Enchanted,
    EnchantedBlock,
    Experience,
    Essence,
    #[default]
    Other,
}

impl ItemProperty {
    pub fn is_block(&self) -> bool {
        self == &Self::EnchantedBlock
    }
}
