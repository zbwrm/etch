use dirs::config_dir;
use markdown::mdast::{Node, Toml};
use markdown::{self, Constructs, ParseOptions};
use serde::Deserialize;
use simple_logger::SimpleLogger;
use std::collections::HashMap;
use std::fs::{self};
use std::{error::Error, path::PathBuf};

use log::{debug, info};

#[derive(Deserialize, Debug)]
struct Config {
    lists: ListsConfig,
}

#[derive(Deserialize, Debug)]
struct ListsConfig {
    lists_dir: PathBuf,
    search_dir: PathBuf,
}

#[derive(Debug)]
struct Category {
    entries: Vec<ListEntry>,
}

#[derive(Deserialize, Debug)]
struct ListEntry {
    filename: String,
    name: String,
}

#[derive(Deserialize, Debug)]
struct ListFrontmatter {
    list: String,
    name: Option<String>,
    category: Option<String>,
}

#[derive(Debug)]
struct List {
    categories: HashMap<String, Category>,
}

impl List {
    fn to_string(&self, name: String) -> String {
        let mut outstr = String::from("");
        outstr.push_str(&format!("# {}\n\n", name));
        for (catname, category) in self.categories.iter() {
            outstr.push_str(&format!("## {catname}\n\n"));
            for entry in &category.entries {
                outstr.push_str(&format!("- ({})[[{}]]\n", entry.name, entry.filename));
            }
            outstr.push('\n');
        }

        outstr
    }
}

fn add_from_frontmatter(
    lists: &mut HashMap<String, List>,
    frontmatter: ListFrontmatter,
    filename: String,
) {
    let catname = match frontmatter.category {
        Some(s) => s,
        None => String::from("Misc."),
    };

    let name = match frontmatter.name {
        Some(s) => s,
        None => filename.clone(),
    };

    if let Some(list) = lists.get_mut(&frontmatter.list) {
        debug!("list named {:#?} already exists", list);
        // if the list already exists...
        if let Some(category) = list.categories.get_mut(&catname) {
            // if the category already exists, add this entry
            category.entries.push(ListEntry { filename, name });
        } else {
            debug!("inserting category {:#?} into list {:#?}", catname, list);
            // if the category doesn't exist, create it, add the entry, and add it to the list
            list.categories.insert(
                catname,
                Category {
                    entries: vec![ListEntry { filename, name }],
                },
            );
        }
    } else {
        // if the list doesn't exist, add a new list and category
        let mut newlist: List = List {
            categories: HashMap::new(),
        };
        newlist.categories.insert(
            catname,
            Category {
                entries: vec![ListEntry { filename, name }],
            },
        );
        lists.insert(frontmatter.list, newlist);
    }
}

fn get_frontmatter_toml(mdast: &Node) -> Option<&Toml> {
    match mdast {
        Node::Root(n) => match &n.children[0] {
            Node::Toml(t) => Some(t),
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

    let mut lists: HashMap<String, List> = HashMap::new();

    for entry in fs::read_dir(cfg.lists.search_dir)? {
        let entry = entry?;
        if let Some(extension) = entry.path().extension() {
            match extension.to_str().unwrap() {
                "md" => {
                    // there has to be a better way
                    let f1 = entry.file_name();
                    let f2 = f1.to_string_lossy();
                    let filename = String::from(f2.split('.').next().unwrap());
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

                    if let Some(t) = get_frontmatter_toml(&mdast) {
                        let frontmatter: ListFrontmatter = toml::from_str(&t.value)?;
                        add_from_frontmatter(&mut lists, frontmatter, filename);
                    }
                }
                _ => continue,
            }
        }
    }

    info!("lists:\n{:#?}", lists);

    for (list_name, list) in lists.iter() {
        let mut list_path = cfg.lists.lists_dir.clone();
        list_path.push(format!("{list_name}.md"));
        fs::write(list_path, list.to_string(list_name.clone()))?
    }

    Ok(())
}
