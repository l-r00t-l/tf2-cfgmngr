use clap::{Command, Parser, Subcommand};
use crate::data_types::{CfgData, CfgMap};
use std::io;
use std::io::Write;
use std::ops::Add;
use std::path::PathBuf;

fn get_inputln(prompt: &str) -> String {
    let prompt_str = prompt.to_string();
    get_input(prompt_str.add("\n").as_str())
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {},
        Err(_no_updates_is_fine) => {},
    }
    input.trim().to_string()
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    // config_id: Option<String>,
    //
    // #[arg(short, long)]
    // list: bool,
    #[clap(subcommand)]
    command: Commands
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(alias = "i")]
    Init { path: Option<String>},
    #[command(aliases = ["l", "ls"])]
    List {},
    #[command(aliases = ["s", ""])]
    Switch { config: String },
}


fn handle_list(cfg: &CfgMap) {
    for (k, v) in cfg {
        println!("- {}", k);
    }
}


// TODO: preferably with interactive ui
// Crate dada.json at given path
fn handle_init(path: Option<String>) -> io::Result<()> {
    let mut path_actual = match path {
        None => { std::env::current_dir().expect("Failed to get cwd.")}
        Some(path) => { PathBuf::from(path) }
    };

    let name = path_actual.file_name().and_then(|os_str| os_str.to_str()).expect("Failed to cwd name.");
    // Check if there are cfg/ and custom/ dirs
    if !path_actual.join("/cfg/").exists() {
        println!("[WARNING] Dir '{}' doesn't contain the cfg/ folder", name);
    }

    if !path_actual.join("/custom/").exists() {
        println!("[WARNING] Dir '{}' doesn't contain the custom/ folder", name);
    }
    let author = get_input("Enter config author: ");
    let description = get_inputln("Enter config description: ");
    let alias = get_input("Enter config alias: ");

    let new_cfg = CfgData { description, author, alias };

    let json_cfg = serde_json::to_string(&new_cfg)?;
    std::fs::write(path_actual.join("data.json"), json_cfg)
}

pub fn run(cfgs: CfgMap) {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init { path } => { handle_init(path).expect("Failed to initialize data file."); }
        Commands::List {} => { handle_list(&cfgs) }
        Commands::Switch { config } => { todo!("switch cfg") }
    }
}
