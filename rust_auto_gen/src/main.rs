use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    out: String,
    inputs: Vec<String>,
    name: String
}

pub fn find_files(dir: &Path, ext_type: &str) -> Vec<PathBuf> {
    let mut result = Vec::new();
    if !dir.exists() || !dir.is_dir() {
        return result;
    }

    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return result,
    };

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                result.extend(find_files(&path, ext_type));
            } else if let Some(ext) = path.extension() {
                if ext == ext_type {
                    result.push(path);
                }
            }
        }
    }

    result
}


fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("Usage: rust_auto_gen <config-file>");
    }
    let path = args.get(1).unwrap();
    let config: Config = serde_yaml::from_str(&fs::read_to_string(path).unwrap()).unwrap();
    let files = config.inputs.iter().fold(Vec::new(), |mut files: Vec<PathBuf>, input| {
        let file_vec = find_files(Path::new(input), "proto");
        files.extend(file_vec);
        files
    });
    if !fs::exists(&config.out).unwrap() {
        fs::create_dir(&config.out).unwrap();
    }
    let src = format!("{}/src", &config.out);
    fs::create_dir(&src).unwrap();
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir(&src)
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(&files, &config.inputs).unwrap();
    let rs_files = find_files(Path::new(&src), "rs");
    let lib_file = rs_files.iter().map(|path| {
        let name =path.file_name().unwrap().to_str().unwrap().split('.').collect::<Vec<&str>>()[0];
        format!("pub mod {name};")
    }).collect::<Vec<String>>().join("\n");
    fs::write(format!("{}/lib.rs", &src), &lib_file).unwrap();
    let cargo_file = format!("{}/Cargo.toml", &config.out);
    let cargo_content = format!("[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nprost = \"0.13.1\"\ntonic = \"0.13.1\"\nserde = {{ version = \"1.0.219\", features = [\"derive\"] }}", config.name);
    fs::write(cargo_file, cargo_content).unwrap();
}
