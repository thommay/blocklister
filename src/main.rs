use serde::Deserialize;
use std::path::{PathBuf};
use std::fs::{OpenOptions};
use std::io::{Read, Write};
use anyhow::Result;

fn main() -> Result<()>{
    let c = std::env::args().last().unwrap();
    let mut cf = OpenOptions::new().read(true).open(c)?;
    let mut cfg = String::new();
    cf.read_to_string(&mut cfg)?;
    let config: Config = toml::from_str(&cfg)?;
    let mut urls = vec!();
    for url in config.blacklists {
        let body: String = reqwest::blocking::get(&url)?.text()?;
        for line in body.lines() {
            if line.is_empty() || line.starts_with('#') { continue; }
            let line = line.split('#').collect::<Vec<&str>>();
            let line = line.first().unwrap().trim();
            let line = String::from(line.trim_end_matches('\n'));
            urls.push(line);
        }
    }
    urls.sort();
    urls.dedup();
    let mut output = String::from("return{");
    for entry in urls {
        output += format!("\"{}\",", entry).as_ref();
    }
    let mut output = String::from(output.trim_end_matches(','));
    output.push('}');
    let mut o = OpenOptions::new().create(true).truncate(true).write(true).open(config.output)?;
    o.write(output.as_bytes())?;
    Ok(())
}

#[derive(Default,Debug,Clone,Deserialize)]
struct Config {
    blacklists: Vec<String>,
    output: PathBuf,
}