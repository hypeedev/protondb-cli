use colored::{ColoredString, Colorize};
use futures::future::join_all;
use image::DynamicImage;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use viuer::Config;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Summary {
    #[serde(rename = "bestReportedTier")]
    pub best_reported_tier: String,
    pub confidence: String,
    pub score: f64,
    pub tier: String,
    pub total: u32,
    #[serde(rename = "trendingTier")]
    pub trending_tier: String,
}

pub(crate) async fn fetch_summaries(client: &Client, steam_ids: &Vec<String>) -> Vec<Option<Summary>> {
    let futures = steam_ids.into_iter().map(|steam_id| {
        let client = client.clone();
        async move {
            let summary = client.get(format!("https://www.protondb.com/api/v1/reports/summaries/{}.json", steam_id))
                .send().await.unwrap()
                .json::<Summary>().await.ok();
            summary
        }
    });
    join_all(futures).await
}

pub(crate) fn get_colored_tier(tier: &String, oslist: &Vec<String>) -> ColoredString {
    if oslist.contains(&"Linux".to_string()) { return "Native".truecolor(0, 255, 0) }
    match tier.as_str() {
        "borked" => "Borked".truecolor(255, 0, 0),
        "bronze" => "Bronze".truecolor(205, 127, 50),
        "silver" => "Silver".truecolor(166, 166, 166),
        "gold" => "Gold".truecolor(207, 181, 59),
        "platinum" => "Platinum".truecolor(180, 199, 220),
        "pending" => "Pending (unrated)".truecolor(68, 68, 68),
        _ => "unknown".truecolor(200, 200, 200)
    }
}

pub(crate) fn get_colored_steam_deck_status(status: &String) -> ColoredString {
    let status = status.split_whitespace().last().unwrap();
    match status {
        "Verified" => "Verified".truecolor(0, 207, 65),
        "Playable" => "Playable".truecolor(255, 201, 42),
        "Unsupported" => "Unsupported".truecolor(255, 0, 0),
        _ => "unknown".truecolor(200, 200, 200)
    }
}

pub(crate) async fn fetch_images(client: &Client, steam_ids: &Vec<String>) -> Vec<DynamicImage> {
    let futures = steam_ids.into_iter().map(|steam_id| {
        let client = client.clone();
        async move {
            let image = client.get(format!("https://steamcdn-a.akamaihd.net/steam/apps/{}/header.jpg", steam_id))
                .send().await.unwrap()
                .bytes().await.unwrap().to_vec();
            let image = image::load_from_memory(&image).unwrap();
            let image = DynamicImage::ImageRgba8(image.to_rgba8());
            image
        }
    });
    join_all(futures).await
}

pub(crate) fn print_image(image: &DynamicImage, width: u32, height: u32) {
    let conf = Config {
        absolute_offset: false,
        width: Some(width),
        height: Some(height),
        ..Default::default()
    };

    viuer::print(image, &conf).unwrap();

    // move cursor to the top of the image
    print!("\x1B[{}A", height);
}