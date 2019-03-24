use std::fs;
use std::path::Path;

use cargo_toml::Manifest;
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct MetaData {
    flutter: FlutterMetadata,
}

#[derive(Deserialize)]
struct FlutterMetadata {
    version: String,
}

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("Cannot get project dir");
    let project_path = Path::new(&manifest_dir);
    let toml_path = project_path.join("Cargo.toml");
    let manifest = Manifest::<MetaData>::from_slice_with_metadata(
        &fs::read(&toml_path).expect("Cannot read Cargo.toml"),
    )
    .expect("Cannot parse Cargo.toml");
    let version = manifest
        .package
        .expect("Flutter config missing in Cargo.toml")
        .metadata
        .expect("Flutter config missing in Cargo.toml")
        .flutter
        .version;

    let libs_dir = project_path.join("libs");

    println!("Downloading flutter engine");
    if let Ok(rx) = flutter_download::download_to(&version, &libs_dir) {
        for (total, done) in rx.iter() {
            println!("Downloading flutter engine {} of {}", done, total);
        }
    }

    let libs_dir = libs_dir.join(&version);
    write_cargo_config(&project_path, &libs_dir);

    #[cfg(target_os = "linux")]
    {
        println!(
            "cargo:rustc-link-search=native={}",
            libs_dir.to_str().expect("libs_dir invalid")
        );
    }

    #[cfg(target_os = "macos")]
    {
        println!(
            "cargo:rustc-link-search=framework={}",
            libs_dir.to_str().expect("libs_dir invalid")
        );
    }

    #[cfg(target_os = "windows")]
    {
        println!(
            "cargo:rustc-link-search=native={}",
            libs_dir.to_str().expect("libs_dir invalid")
        );

        let mut res = winres::WindowsResource::new();
        res.set_icon_with_id("./assets/icon.ico", "GLFW_ICON");
        res.compile().unwrap();
    }
}

fn write_cargo_config(project_dir: &Path, libs_dir: &Path) {
    println!("Generating .cargo/config file");

    let config_dir = project_dir.join(".cargo");
    std::fs::create_dir(&config_dir).unwrap_or(());

    let s = if cfg!(target_os = "linux") {
        format!(
            r#"[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "link-args=-Wl,-rpath,{libs}"]"#,
            libs = libs_dir.to_string_lossy()
        )
    } else if cfg!(target_os = "macos") {
        format!(
            r#"[target.x86_64-apple-darwin]
rustflags = ["-C", "link-args=-Wl,-rpath,{libs},-rpath,@executable_path/../Frameworks/"]"#,
            libs = libs_dir.to_string_lossy()
        )
    } else if cfg!(target_os = "windows") {
        // windows does not use rpath, we have to copy dll to OUT_DIR
        let src = libs_dir.join("flutter_engine.dll");
        let tar = Path::new(&std::env::var("OUT_DIR").unwrap())
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("flutter_engine.dll");

        let _ = fs::copy(src, tar);
        format!(r#""#)
    } else {
        format!(r#""#)
    };

    fs::write(config_dir.join("config"), s).expect("Cannot write linker config in .cargo/config");
}
