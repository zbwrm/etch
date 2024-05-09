use dirs::config_dir;
use markdown::mdast::{Node, Yaml};
use markdown::{self, Constructs, ParseOptions};
use serde::Deserialize;
use serde_yaml;
use simple_logger::SimpleLogger;
use std::fs::{self};
use std::io::Read;
use std::{error::Error, path::PathBuf};

use log::{debug, info, warn};

#[derive(Deserialize, Debug)]
struct Config {
    lists: ListsConfig,
}

#[derive(Deserialize, Debug)]
struct ListsConfig {
    lists_dir: PathBuf,
    search_dir: PathBuf,
}

fn get_frontmatter(mdast: &Node) -> Option<&Yaml> {
    match mdast {
        Node::Root(n) => match &n.children[0] {
            Node::Yaml(y) => Some(y),
            _ => None,
        },
        _ => None,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().init()?;
    let mut cfg_path: PathBuf = config_dir().expect("no config folder found");
    cfg_path.push("etch/config.toml");
    debug!("attempting to read config from {:#?}...", cfg_path);
    let cfg_string = fs::read_to_string(cfg_path.clone())?;
    info!("successfully read config from {:#?}", cfg_path);
    let cfg: Config = toml::from_str(&cfg_string)?;
    debug!(
        "found dirs\n - search_dir: {:#?}\n - lists_dir:  {:#?}",
        cfg.lists.search_dir, cfg.lists.lists_dir
    );

    info!(
        "looking at base-level markdown files in {:#?}",
        cfg.lists.search_dir
    );
    for entry in fs::read_dir(cfg.lists.search_dir)? {
        let entry = entry?;
        if let Some(extension) = entry.path().extension() {
            match extension.to_str().unwrap() {
                "md" => {
                    // there has to be a better way
                    let f1 = entry.file_name();
                    let f2 = f1.to_string_lossy();
                    let filename = f2.split('.').next().unwrap();
                    debug!("found markdown file named {}", filename);
                    let mdast = markdown::to_mdast(
                        &fs::read_to_string(entry.path()).unwrap(),
                        &ParseOptions {
                            constructs: Constructs {
                                frontmatter: true,
                                ..Constructs::default()
                            },
                            ..ParseOptions::default()
                        },
                    )
                    .unwrap();
                    let frontmatter = match get_frontmatter(&mdast) {
                        Some(y) => y.value.clone(),
                        None => continue,
                    };

                    info!("found & parsed frontmatter:\n{:#?}", frontmatter);
                }

                _ => continue,
            }
        }
    }
    Ok(())
}
