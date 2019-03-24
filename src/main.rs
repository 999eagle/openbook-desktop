use std::{env, path::PathBuf};

use flutter_engine::{FlutterEngine, FlutterEngineArgs};

fn main() {
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

    let engine = FlutterEngine::new(args);
    engine.run();
    engine.shutdown();
}
