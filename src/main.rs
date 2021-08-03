use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::str::FromStr;
use url::Url;

mod download;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let url = Url::parse(&args[1]).context("Failed to parse URL")?;
    match url.host_str().context("Host string is invalid")? {
        "nhentai.net" => {
            let mut paths = url.path_segments().context("can not be base")?;
            let code_str = paths.nth(1).context("bad url")?;
            let code_int: usize = FromStr::from_str(code_str).context("can't convert code str to code int")?;
            download::nhentai::start(code_int)?;
        }
        _ => {
            println!("Unsupported URL");
            std::process::exit(1);
        }
    }

    fs::create_dir_all("data").context("Failed to create folder")?;

    Ok(())
}
