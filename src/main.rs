use std::{env::args, process::Command};
use crates_io_api::{SyncClient, Error};
use regex::Regex;

#[derive(Debug, Clone)]
struct CargoSourcePath {
    crate_name: String,
    version: String,
    file_path: String,
    line_number: Option<String>,
    full: String
}

impl CargoSourcePath {
    fn new(path: String) -> Option<Self> {
        let name_ver = path.split_once("/")?.0;
        println!("{}", name_ver);
        let ver = name_ver.rsplit_once('-')?.1;
        let name = name_ver.rsplit_once('-')?.0;
        if(path.contains(":")) {
            let file_path = path.split_once("/")?.1.split_once(":")?.0;
            let line = path.split_once("/")?.1.split_once(":")?.1;

            Some(CargoSourcePath {
                crate_name: name.to_string(),
                version: ver.to_string(),
                file_path: file_path.to_string(),
                line_number: Some(line.to_string()),
                full : path
            })
        } else {

            let file_path = path.split_once("/")?.1;
            Some(CargoSourcePath {
                crate_name: name.to_string(),
                version: ver.to_string(),
                file_path: file_path.to_string(),
                line_number: None,
                full : path
            })
        }
    }
}


fn remove_cargo_registry_prefix(path: &str) -> String {
    let marker = ".cargo/registry/src/index.crates.io-";
    
    if let Some(start_idx) = path.find(marker) {
        let after_marker = &path[start_idx + marker.len()..];
        
        // Skip the hash and the following slash
        if let Some(slash_idx) = after_marker.find('/') {
            return after_marker[slash_idx + 1..].trim_end().to_string();
        }
    }

    println!("{}", path);
    
    path.trim_end().to_string()
}


fn main() {

    let re = Regex::new(
        r"(?x)
        \.cargo/registry/src/
        index\.crates\.io-[a-f0-9]+/
        ([a-zA-Z0-9_-]+)-(\d+\.\d+\.\d+[a-zA-Z0-9._-]*)  # crate name and version
        /(.+?)                                            # file path
        (?::(\d+))?                                       # optional line number
        (?:$|\s|:\d+)                                    # end marker
        "
    ).unwrap();

    let path = args().nth(1).unwrap();

    let output = Command::new("strings")
        .arg(path)
        .output()
        .expect("Failed to execute strings");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let matches = re.find_iter(&stdout).
            map(|cap| cap.as_str()).
            //map(|pack| remove_cargo_registry_prefix(pack)).collect::<Vec<_>>();
            map(|pack| CargoSourcePath::new(remove_cargo_registry_prefix(pack))).collect::<Vec<_>>();

        println!("Strings found:\n{:?}", matches);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error: {}", stderr);
    }
}
