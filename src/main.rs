use serde::Deserialize;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::PathBuf;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let c = std::env::args().last().unwrap();
    let mut cf = OpenOptions::new().read(true).open(c)?;
    let mut cfg = String::new();
    cf.read_to_string(&mut cfg)?;
    let config: Config = toml::from_str(&cfg)?;
    process(config.blocklists, config.blocklist_output).await?;
    process(config.permitted, config.permitted_output).await?;
    Ok(())
}

async fn process(tgt: Vec<String>, path: PathBuf) -> Result<()> {
    let mut urls = vec![];
    for url in tgt {
        let body: String = if let Ok(req) = reqwest::get(&url).await {
            req.text().await?
        } else {
            dbg!(format!("got busted URL: {}", url));
            continue;
        };
        for line in body.lines() {
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let line = line.split('#').collect::<Vec<&str>>();
            let line = line.first().unwrap().trim();
            let line = String::from(line.trim_end_matches('\n'));
            urls.push(line);
        }
    }

    let output = luaify(&mut urls);
    let mut o = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(path)?;
    o.write(output.as_bytes())?;
    Ok(())
}

fn luaify(input: &mut Vec<String>) -> String {
    input.sort();
    input.dedup();
    let mut output = String::from("return{");
    for entry in input {
        output += format!("\"{}\",", entry).as_ref();
    }
    let mut output = String::from(output.trim_end_matches(','));
    output.push('}');
    output
}
#[derive(Default, Debug, Clone, Deserialize)]
struct Config {
    blocklists: Vec<String>,
    blocklist_output: PathBuf,
    permitted: Vec<String>,
    permitted_output: PathBuf,
}
