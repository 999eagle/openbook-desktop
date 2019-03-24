#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use std::path::PathBuf;

use flutter_engine::{FlutterEngine, FlutterEngineArgs};
use log::{debug, info};

mod logging;
mod plugins;

fn main() {
    if cfg!(debug_assertions) {
        logging::setup_logging(2, false).expect("Failed to setup logging");
    } else {
        logging::setup_logging(1, true).expect("Failed to setup logging");
    }
    info!("Starting openbook-desktop");
    debug!("Loading flutter engine");
    flutter_engine::init();

    let assets_path = PathBuf::from("openbook-app/build/flutter_assets");
    let icu_data_path = PathBuf::from("icudtl.dat");

    let args = FlutterEngineArgs {
        assets_path: assets_path.to_string_lossy().into_owned(),
        icu_data_path: icu_data_path.to_string_lossy().into_owned(),
        title: String::from("Openbook"),
        width: 800,
        height: 600,
        ..Default::default()
    };

    debug!("Creating flutter engine");
    let engine = FlutterEngine::new(args);
    info!("Registering plugins");
    engine.add_plugin(Box::new(plugins::FlutterSecureStoragePlugin::new()));
    engine.add_plugin(Box::new(plugins::PathProviderPlugin::new()));
    debug!("Running app");
    engine.run();
    info!("Shutting down");
    engine.shutdown();
}
