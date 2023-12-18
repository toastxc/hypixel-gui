use crate::structures::{Progress, SearchFields};
use eframe::{HardwareAcceleration, Renderer, Theme};
use engine::methods::bazaar::ProfitInfo;
use std::sync::{Arc, RwLock};
use tokio::runtime::Runtime;

pub mod process;
pub mod structures;
pub mod view;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([480.0, 480.0])
            .with_min_inner_size([480.0, 480.0]),
        vsync: true,
        hardware_acceleration: HardwareAcceleration::Preferred,
        renderer: Renderer::Glow,
        follow_system_theme: true,
        default_theme: Theme::Dark,
        centered: false,
        ..Default::default()
    };
    eframe::run_native(
        "Bazaar GUI",
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
    pub progress: Arc<RwLock<Progress>>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            runtime: Arc::new(Runtime::new().unwrap()),
            search_fields: Default::default(),
            processed_data: vec![],
            original_data: Arc::new(RwLock::new(Vec::new())),
            progress: Arc::new(RwLock::new(Progress::default())),
        }
    }
}
