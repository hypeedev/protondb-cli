mod post_result;
mod post_body;
mod args;
mod utils;
mod display_reports;

use post_result::PostResult;
use post_body::Body;
use args::Args;
use utils::{fetch_images, fetch_summaries, get_colored_steam_deck_status, get_colored_tier, is_query_id, print_image, Summary};
use display_reports::{fetch_reports, Reports};

use reqwest::ClientBuilder;
use clap::Parser;
use colored::Colorize;
use reqwest::header::HeaderMap;
use futures::join;
use image::DynamicImage;
use crate::display_reports::display_reports;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let mut new_search = Vec::new();
    for arg in &args.query {
        if arg.contains(" ") {
            let split = arg.split_whitespace();
            new_search.extend(split.map(|s| s.to_string()));
        } else {
            new_search.push(arg.clone());
        }
    }
    let query = new_search.join(" ");

    let should_display_reports = args.count == 1 || is_query_id(&query);

    let body = Body {
        query: query.to_string(),
        facet_filters: vec![vec!["appType:Game"]],
        hits_per_page: args.count,
        attributes_to_retrieve: vec!["name", "objectID", "oslist"],
        page: 0
    };

    let client = ClientBuilder::new()
        .default_headers(
            HeaderMap::from_iter(
                vec![
                    ("x-algolia-api-key", "9ba0e69fb2974316cdaec8f5f257088f"),
                    ("x-algolia-application-id", "94HE6YATEI"),
                    ("Referer", "https://www.protondb.com")
                ].into_iter().map(|(k, v)| (k.parse().unwrap(), v.parse().unwrap()))
            )
        ).build().unwrap();
    let res = client
        .post("https://94he6yatei-dsn.algolia.net/1/indexes/steamdb/query")
        .body(serde_json::to_string(&body).unwrap())
        .send()
        .await.unwrap().json::<PostResult>().await.unwrap();

    let steam_ids: Vec<String> = res.hits.iter().map(|game| game.object_id.clone()).collect();

    let summaries: Vec<Option<Summary>>;
    let mut images: Vec<DynamicImage> = Vec::new();
    let mut reports = Reports {
        page: 0,
        per_page: 0,
        reports: Vec::new(),
        total: 0
    };

    let steam_id: u32 = steam_ids.first().unwrap().parse().unwrap();
    if args.images {
        if should_display_reports {
            (summaries, images, reports) = join!(fetch_summaries(&client, &steam_ids), fetch_images(&client, &steam_ids), fetch_reports(&client, steam_id));
        } else {
            (summaries, images) = join!(fetch_summaries(&client, &steam_ids), fetch_images(&client, &steam_ids));
        }
    } else {
        if args.count == 1 {
            (summaries, reports) = join!(fetch_summaries(&client, &steam_ids), fetch_reports(&client, steam_id));
        } else {
            summaries = fetch_summaries(&client, &steam_ids).await;
        }
    }

    const IMAGE_WIDTH: u32 = 14;
    const IMAGE_HEIGHT: u32 = 3;
    let mut lines_printed;
    macro_rules! label {
        ($label:expr, $value:expr) => {
            // move cursor to the right of the image
            if args.images { print!("\x1B[{}C", IMAGE_WIDTH + 1) }
            println!("{} {}", format!("{}:", $label).truecolor(200, 200, 200), $value);
            lines_printed += 1;
        };
        ($value:expr) => {
            // move cursor to the right of the image
            if args.images { print!("\x1B[{}C", IMAGE_WIDTH + 1) }
            println!("{}", $value);
            lines_printed += 1;
        };
    }

    for (index, game) in res.hits.into_iter().enumerate() {
        lines_printed = 0;
        if index != 0 { println!() }

        if args.images { print_image(&images[index], IMAGE_WIDTH, IMAGE_HEIGHT) }

        label!(game.name.bold());

        if let Some(summary) = &summaries[index] {
            label!("Rating", get_colored_tier(&summary.tier, &game.oslist));

            let steam_deck_status = game.oslist.iter().find(|os| os.starts_with("Steam Deck"));
            if let Some(status) = steam_deck_status {
                let status = get_colored_steam_deck_status(status);
                label!("Steam Deck", status);
            }
        } else {
            label!("Rating", get_colored_tier(&"pending".to_string(), &game.oslist));
        }

        if args.images && IMAGE_HEIGHT > lines_printed {
            print!("{}", "\n".repeat((IMAGE_HEIGHT - lines_printed) as usize));
        }
    }

    if should_display_reports {
        let termsize::Size { rows: _, cols: width } = termsize::get().unwrap();
        display_reports(reports, &args, &client, width).await;
    }
}
