use engine::methods::bazaar::ProfitInfo;
use std::sync::{Arc, RwLock};
use tokio::runtime::Runtime;
pub mod process;
pub mod view;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([480.0, 480.0]),
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
