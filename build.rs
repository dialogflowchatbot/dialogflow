use core::result::Result;
use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use flate2::{Compression, write::GzEncoder};

fn walk_assets(path: impl AsRef<Path>) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut files: Vec<PathBuf> = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_type = entry.file_type()?;
                if file_type.is_dir() {
                    let mut other_files = walk_assets(entry.path())?;
                    files.append(&mut other_files);
                } else if file_type.is_file() {
                    let path = entry.path();
                    let ext = path.extension();
                    if ext.is_none() {
                        continue;
                    }
                    let ext = ext.unwrap().to_os_string().into_string().unwrap();
                    if ext.find("gz").is_some() {
                        continue;
                    }
                    files.push(entry.path());
                    // files.push(entry.file_name().to_os_string().into_string().unwrap());
                }
            }
        }
    }
    Ok(files)
}

fn gz_files(raw_asset_files: Vec<PathBuf>) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut gz_files: Vec<PathBuf> = Vec::new();

    for asset_file in raw_asset_files.iter() {
        let cache: Vec<u8> = Vec::with_capacity(65535);
        let mut e = GzEncoder::new(cache, Compression::default());
        let b = fs::read(asset_file)?;
        e.write_all(b.as_slice())?;
        let compressed_bytes = e.finish()?;
        let mut extension = asset_file
            .extension()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();
        extension.push_str(".gz");
        let gz_file = asset_file.with_extension(extension.as_str());
        fs::write(gz_file.as_path(), compressed_bytes.as_slice())?;
        gz_files.push(gz_file);
    }
    Ok(gz_files)
}

fn get_content_type(filename: String) -> String {
    if filename.rfind(".css").is_some() {
        String::from("text/css")
    } else if filename.rfind(".js").is_some() {
        String::from("text/javascript")
    } else if filename.rfind(".html").is_some() {
        String::from("text/html; charset=utf-8")
    } else if filename.rfind(".wasm").is_some() {
        String::from("application/wasm")
    } else if filename.rfind(".ico").is_some() {
        String::from("image/x-icon")
    } else {
        String::new()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo::rerun-if-changed=build.rs,resources/assets/index.html");
    // println!("cargo::rustc-link-search=/usr/local/musl/lib");
    // println!("cargo::rustc-link-search=/usr/local/musl/include");
    // println!("cargo::rustc-link-search=./lib");
    // println!("cargo::rustc-link-lib=static=sqlite_vec0");
    // println!("cargo:rustc-link-lib=static=sqlite_vec0");

    // embed all static resource asset files
    let asset_root = Path::new("src").join("resources").join("assets");
    let asset_root = format!("{}/", asset_root.display());
    let asset_root = asset_root.as_str();
    let all_static_asset_files = walk_assets(asset_root)?;
    let gz_files = gz_files(all_static_asset_files)?;
    let mut file_list: Vec<String> = Vec::new();
    let mut service_asset_file = File::create(Path::new("src").join("web").join("asset.txt"))?;
    writeln!(&mut service_asset_file, r##"["##,)?;
    for f in gz_files.iter() {
        let file_name = format!("{}", f.display())
            .replace(asset_root, "")
            .replace(".gz", "")
            .replace("\\", std::path::MAIN_SEPARATOR_STR);
        file_list.push(file_name);
        writeln!(
            &mut service_asset_file,
            r##"(include_bytes!(r#"{file_path}"#), "{mime}"),"##,
            file_path = format!("{}", f.display())
                .replace("\\", std::path::MAIN_SEPARATOR_STR)
                .replace("src", ".."),
            mime = get_content_type(format!("{}", f.display())),
        )?;
    }
    writeln!(&mut service_asset_file, r##"]"##,)?;

    let mut service_asset_file = File::create(Path::new("src").join("web").join("asset.rs"))?;
    writeln!(
        &mut service_asset_file,
        r##"use std::collections::HashMap;"##,
    )?;
    writeln!(&mut service_asset_file, r##""##,)?;
    writeln!(&mut service_asset_file, r##"use std::sync::LazyLock;"##,)?;
    writeln!(&mut service_asset_file, r##""##,)?;
    // use std::cell::LazyCell;
    writeln!(
        &mut service_asset_file,
        r##"pub(crate) static ASSETS_MAP: LazyLock<HashMap<&str, usize>> = LazyLock::new(|| {{"##,
    )?;
    writeln!(&mut service_asset_file, r##"HashMap::from(["##,)?;
    let mut i = 0u8;
    for f in file_list.iter() {
        if f.eq("index.html") {
            writeln!(
                &mut service_asset_file,
                r##"("/", {counter}),"##,
                counter = i,
            )?;
        }
        writeln!(
            &mut service_asset_file,
            r##"(r"/{name}", {counter}),"##,
            name = f.replace("\\", "/"),
            counter = i,
        )?;
        i = i + 1;
    }
    writeln!(&mut service_asset_file, r##"])}});"##,)?;

    Ok(())
}
