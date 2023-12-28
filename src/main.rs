use crate::structures::{Progress, SearchFields};
use eframe::{HardwareAcceleration, Renderer};

use crate::engine::methods::bazaar::ProfitInfo;
use std::sync::{Arc, RwLock};
use tokio::runtime::Runtime;

pub mod engine;
pub mod process;
pub mod structures;
pub mod view;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([480.0, 480.0])
            .with_min_inner_size([480.0, 300.0])
            .with_transparent(true),
        vsync: true,
        hardware_acceleration: HardwareAcceleration::Preferred,
        renderer: Renderer::Glow,
        follow_system_theme: true,
        centered: false,
        ..Default::default()
    };

    eframe::run_native(
        "Bazaar",
        options,
        Box::new(|_cc| Box::from(MyApp::from_theme(_cc.egui_ctx.style().visuals.dark_mode))),
    )?;

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

impl MyApp {
    pub fn from_theme(theme: bool) -> Self {
        Self::default().set_theme(theme)
    }
    pub fn set_theme(&mut self, is_dark: bool) -> Self {
        self.is_dark = is_dark;
        self.clone()
    }
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
