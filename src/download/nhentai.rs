use super::models::NhentaiDoujin;
use anyhow::{Context, Result};
use indicatif::ProgressBar;
use reqwest::blocking::Client;
use serde_json;
use std::fs;
use std::io;

pub fn start(code: usize) -> Result<()> {
    println!("Using nhentai.net module to download {}", code);

    let client = Client::new();
    let resp = client
        .get(format!("https://nhentai.net/api/gallery/{}", code))
        .send()
        .context("Can't send request")?
        .text()
        .context("Can't get response text")?;
    let doujin: NhentaiDoujin = serde_json::from_str(&resp).context("Can't parse JSON")?;
    println!("{} with {} pages", doujin.title.pretty, doujin.num_pages);

    let folder_name = doujin.title.pretty.replace(" ", "_");
    fs::create_dir_all(format!("data/{}", folder_name)).context("Failed to create folder")?;

    let pb = ProgressBar::new(doujin.num_pages);

    for (i, image) in doujin.images.pages.iter().enumerate() {
        let img_url = match image.t.as_str() {
            "j" => format!(
                "https://i.nhentai.net/galleries/{}/{}.jpg",
                doujin.media_id,
                i + 1
            ),
            "p" => format!(
                "https://i.nhentai.net/galleries/{}/{}.png",
                doujin.media_id,
                i + 1
            ),
            _ => panic!("Unrecognized image type"),
        };
        let mut img_resp = client.get(img_url).send().context("Can't send request")?;
        let file_path = match image.t.as_str() {
            "j" => format!("data/{}/{}.jpg", folder_name, i + 1),
            "p" => format!("data/{}/{}.png", folder_name, i + 1),
            _ => panic!("Unrecognized image type"),
        };
        let mut file = fs::File::create(file_path)?;
        io::copy(&mut img_resp, &mut file)?;

        pb.inc(1);
    }

    pb.finish_with_message("Download completed");

    Ok(())
}
