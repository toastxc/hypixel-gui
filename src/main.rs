use crate::structures::{Progress, SearchFields};
use eframe::{HardwareAcceleration, Renderer};

use std::sync::{Arc, RwLock};
use tokio::runtime::Runtime;
use crate::engine::methods::bazaar::ProfitInfo;


pub mod engine;
pub mod process;
pub mod structures;
pub mod view;

fn main() -> Result<(), eframe::Error> {


    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([480.0, 480.0])
            .with_min_inner_size([480.0, 300.0]),
        vsync: true,
        hardware_acceleration: HardwareAcceleration::Preferred,
        renderer: Renderer::Glow,
        follow_system_theme: true,
        centered: false,
        ..Default::default()
    };
    eframe::run_native("Bazaar", options, Box::new(|_cc| Box::<MyApp>::default()))?;

    Ok(())
}

#[derive(Debug, Clone)]
struct MyApp {
    pub search: SearchFields,
    pub runtime: Arc<Runtime>,
    pub original_data: Arc<RwLock<Vec<ProfitInfo>>>,
    pub processed_data: Vec<ProfitInfo>,
    pub progress: Arc<RwLock<Progress>>,
    pub is_dark: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            runtime: Arc::new(Runtime::new().unwrap()),
            search: Default::default(),
            processed_data: vec![],
            original_data: Arc::new(RwLock::new(Vec::new())),
            progress: Arc::new(RwLock::new(Progress::default())),
            is_dark: true,
        }
    }
}
