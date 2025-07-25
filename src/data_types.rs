use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub tf_path: String,          // Path to the tf directory
    pub safe_move: bool, // True -> Copy instead of moving || False -> Move the config file
    // pub configs_directory: String, // directory to save your configs
}

pub type CfgMap = HashMap<String, CfgData>;

impl Default for Settings {
    fn default() -> Self {
        Settings {
            tf_path: "E:\\SteamLibrary\\steamapps\\common\\Team Fortress 2\\tf".to_string(),
            safe_move: true,
            // configs_directory: "E:\\SteamLibrary\\steamapps\\common\\Team Fortress 2\\tf\\.cfgmgr"
            //     .to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CfgData {
    // pub name: String, // Automatically replace with folder name
    pub description: String,
    pub author: String,
    pub alias: String,
}
