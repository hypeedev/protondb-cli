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
        "Verified" => "Verified".green(),
        "Playable" => "Playable".yellow(),
        "Unsupported" => "Unsupported".red(),
        _ => ColoredString::from("unknown")
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

fn get_hash(n1: u32, n2: u32, timestamp: u32) -> String {
    format!("{}W{}", n2, n1 as u64 * (n2 % timestamp) as u64)
}

fn get_protondb_id(hash: &str) -> u32 {
    (hash.to_owned() + "m")
        .chars()
        .fold(0, |acc: i32, char| (acc << 5).wrapping_sub(acc) + char as i32)
        .abs() as u32
}

pub(crate) fn calculate_protondb_id(steam_id: u32, number_of_reports: u32, counts_timestamp: u32) -> u32 {
    let hash1 = get_hash(steam_id, number_of_reports, counts_timestamp);
    let hash2 = get_hash(1, steam_id, counts_timestamp);
    let hash3 = format!("p{}*vRT{}undefined", hash1, hash2);
    get_protondb_id(&hash3)
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Counts {
    pub reports: u32,
    pub timestamp: u32
}

pub(crate) fn is_query_id(query: &str) -> bool {
    query.chars().all(char::is_numeric)
}
