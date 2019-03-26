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

enum Target {
    Linux,
    MacOS,
    Windows,
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

    let target = std::env::var("TARGET").expect("Cannot determine target");
    if target.contains("-windows-gnu") {
        mingw_check_47048();
    }

    let target = if target.contains("linux") {
        Target::Linux
    } else if target.contains("apple") {
        Target::MacOS
    } else if target.contains("windows") {
        Target::Windows
    } else {
        panic!("Unknown target {}", target)
    };

    println!("Downloading flutter engine");
    if let Ok(rx) = flutter_download::download_to(&version, &libs_dir) {
        for (total, done) in rx.iter() {
            println!("Downloading flutter engine {} of {}", done, total);
        }
    }

    let libs_dir = libs_dir.join(&version);

    match target {
        Target::Linux => {
            let src = libs_dir.join("libflutter_engine.so");
            let tar = Path::new(&std::env::var("OUT_DIR").unwrap())
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .join("libflutter_engine.so");
            fs::copy(src, tar).expect("Cannot copy libflutter_engine.so");
            println!(
                "cargo:rustc-link-search=native={}",
                libs_dir.to_str().expect("libs_dir invalid")
            );
        }
        Target::MacOS => println!(
            "cargo:rustc-link-search=framework={}",
            libs_dir.to_str().expect("libs_dir invalid")
        ),
        Target::Windows => {
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
            fs::copy(src, tar).expect("Cannot copy flutter_engine.dll");
            println!(
                "cargo:rustc-link-search=native={}",
                libs_dir.to_str().expect("libs_dir invalid")
            );

            //            let mut res = winres::WindowsResource::new();
            //            res.set_icon_with_id("./assets/icon.ico", "GLFW_ICON");
            //            res.compile().unwrap();
        }
    };
}

fn mingw_check_47048() {
    // workaround for issue #47048 in github.com/rust-lang/rust
    let out_dir = std::env::var_os("OUT_DIR").expect("Cannot get output dir");
    let out_dir = Path::new(&out_dir);
    let try_dir = out_dir.join("try_47048");
    fs::create_dir_all(&try_dir).expect("Cannot create output dir");
    fs::write(try_dir.join("workaround.c"), WORKAROUND_C).expect("Cannot write workaround.c");
    cc::Build::new()
        .file(try_dir.join("workaround.c"))
        .compile("workaround_47048");
    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.to_str().expect("out_dir invalid")
    );
    println!("cargo:rustc-link-lib=static=workaround_47048");
}

const WORKAROUND_C: &'static str = r#"/* workaround.c */
#define _CRTBLD
#include <stdio.h>

FILE *__cdecl __acrt_iob_func(unsigned index)
{
    return &(__iob_func()[index]);
}

typedef FILE *__cdecl (*_f__acrt_iob_func)(unsigned index);
_f__acrt_iob_func __MINGW_IMP_SYMBOL(__acrt_iob_func) = __acrt_iob_func;
"#;
