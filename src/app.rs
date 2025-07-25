use crate::{cli, data_types::{Settings, CfgData, CfgMap}, settings};
use std::io;
use std::fs;
use std::path::Path;

fn discover_cfgs(settings: Settings) -> io::Result<CfgMap> {
    let mut cfgs = CfgMap::new();
    let tf_path_str = settings.tf_path;
    let tf_path = Path::new(&tf_path_str);
    if !tf_path.is_dir() {
        return Err(io::Error::new(io::ErrorKind::NotADirectory, "tf path is not a directory."))
    }
    let cfg_path = tf_path.join(".cfgs/");

    if cfg_path.is_dir() {
        for e in fs::read_dir(cfg_path)? {
            let e = e?;
            let cfg_path = e.path();
            let cfg_name = cfg_path.iter().last().unwrap().display();
            let cfg_raw = fs::read_to_string(cfg_path.join("data.json"));
            if !cfg_raw.is_ok() {
                Err(io::Error::new(io::ErrorKind::NotFound, format!("Config data (data.json) does not exist in {cfg_name} directory.")))?;
            }
            let data: CfgData = serde_json::from_str(cfg_raw.unwrap().as_str())?;
            cfgs.insert(cfg_name.to_string(), data);
        }
    } else {
        return Err(io::Error::new(io::ErrorKind::NotADirectory, "tf/.cfgs/ is not a directory."));
    }

    Ok(cfgs)
}

pub fn run() {
    let settings = settings::load().unwrap();
    match discover_cfgs(settings) {
        Ok(cfgs) => {
            cli::run(cfgs);
        },
        Err(e) => {
            eprintln!("Failed to discover cfgs: {}", e);
        }
    }
}
