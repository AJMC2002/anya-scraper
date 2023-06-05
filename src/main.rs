use rayon::prelude::*;
use scraper::{Html, Selector};
use std::fs;

const BASE_URL: &str = r"https://tiermaker.com";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string("source.html")?;
    let fragment = Html::parse_fragment(&source);
    let selector = Selector::parse("div.character")?;

    let divs: Vec<String> = fragment.select(&selector).map(|el| el.html()).collect();
    divs.par_iter().enumerate().for_each(|(i, div)| {
        let mut url = "";
        div.split("&quot;").for_each(|substr| {
            if substr.contains("mahjong-soul-stickers") {
                url = substr;
            }
        });

        if !url.is_empty() {
            println!("Getting URL...");
            let img_bytes = reqwest::blocking::get(BASE_URL.to_string() + url)
                .unwrap()
                .bytes()
                .unwrap();
            image::load_from_memory(&img_bytes)
                .unwrap()
                .save(format!("./imgs/{}.png", i))
                .unwrap();
            println!("Image gotten!");
        } else {
            println!("URL not found");
        }
    });

    Ok(())
}
