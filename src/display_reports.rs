use reqwest::Client;
use colored::Colorize;
use futures::future::join_all;
use image::DynamicImage;
use chrono_humanize::HumanTime;
use crate::args::Args;
use crate::reports::{Report, Reports};
use crate::utils::{calculate_protondb_id, Counts, print_image, indent_by, capitalize};

pub(crate) async fn fetch_reports(client: &Client, steam_id: u32) -> Reports {
    let counts = client
        .get("https://www.protondb.com/data/counts.json")
        .send().await.unwrap()
        .json::<Counts>().await.unwrap();

    let protondb_id = calculate_protondb_id(steam_id, counts.reports, counts.timestamp);

    // let reports = client
    //     .get(format!("https://www.protondb.com/data/reports/all-devices/app/{}.json", protondb_id))
    //     .send().await.unwrap()
    //     .text().await.unwrap();
    // println!("{}", reports);

    client
        .get(format!("https://www.protondb.com/data/reports/all-devices/app/{}.json", protondb_id))
        .send().await.unwrap()
        .json::<Reports>().await.unwrap()
}

pub(crate) async fn fetch_avatars(client: &Client, avatar_urls: &Vec<String>) -> Vec<DynamicImage> {
    let futures = avatar_urls.into_iter().map(|avatar_url| {
        let client = client.clone();
        async move {
            let image = client.get(avatar_url)
                .send().await.unwrap()
                .bytes().await.unwrap().to_vec();
            let image = image::load_from_memory(&image).unwrap();
            let image = DynamicImage::ImageRgba8(image.to_rgba8());
            image
        }
    });
    join_all(futures).await
}

fn get_tinker_steps(report: &Report) -> Vec<String> {
    let mut tinker_steps = Vec::new();

    if let Some(launcher) = &report.responses.notes.launcher {
        tinker_steps.push(format!("Launcher: {}", launcher));
    }

    if let Some(variant) = &report.responses.variant {
        if variant == "experimental" {
            tinker_steps.push("Switch to experimental".to_string());
        } else if variant == "ge" {
            if let Some(custom_proton_version) = &report.responses.custom_proton_version {
                tinker_steps.push(format!("Custom Proton: {}", custom_proton_version));
            } else {
                tinker_steps.push("Custom Proton: GE".to_string());
            }
        } else if variant == "notListed" {
            if let Some(proton_version) = &report.responses.proton_version {
                tinker_steps.push(format!("Custom Proton: {}", proton_version));
            } else {
                tinker_steps.push("Custom Proton".to_string());
            }
        }
    }

    if report.responses.launch_options.is_some() {
        tinker_steps.push("Set launch options".to_string());
    }

    if let Some(tinker_override) = &report.responses.tinker_override {
        if tinker_override == "yes" {
            tinker_steps.push("Other".to_string());
        }
    }

    tinker_steps
}

macro_rules! label {
    ($label:expr, $value:expr) => {
        let label = indent_by(AVATAR_WIDTH as usize + 1, (&format!("{}:", $label).truecolor(200, 200, 200)).to_string());
        println!("{} {}", label, $value);
    };
    ($value:expr) => {
        println!("{}", indent_by(AVATAR_WIDTH as usize + 1, $value.to_string()));
    };
    () => {
        println!();
    };
}

fn print_timestamp(report: &Report) -> u8 {
    let timestamp = chrono::DateTime::from_timestamp(report.timestamp as i64, 0).unwrap();
    label!(HumanTime::from(timestamp).to_string().truecolor(120, 120, 120).italic());
    1
}

fn print_header(report: &Report) -> u8 {
    let mut report_header = format!("{} {}",
        report.contributor.steam.nickname,
        format!("({} reports)", report.contributor.report_tally).truecolor(120, 120, 120)
    );
    if let Some(playtime) = &report.contributor.steam.playtime {
        let hours = playtime / 60;
        report_header = format!("{} | {}",
            report_header,
            format!("{}{} hour{} overall",
                    if hours == 0 { "< " } else { "" },
                    hours.max(1),
                    if hours == 0 || hours == 1 { "" } else { "s" }
            ).truecolor(200, 200, 200)
        );
    }
    label!(report_header);
    1
}

fn print_verdict(report: &Report, tinker_steps: &Vec<String>, print_newline: &mut bool) -> u8 {
    let mut lines_printed = 0;
    let starts_play = report.responses.starts_play == Some("yes".to_string());

    if starts_play {
        if tinker_steps.is_empty() {
            if let Some(verdict) = &report.responses.verdict {
                if verdict == "yes" {
                    label!("Recommended".green());
                } else {
                    label!("Not Recommended".yellow());
                }
                lines_printed += 1;
            }
        } else {
            if let Some(verdict_oob) = &report.responses.verdict_oob {
                if verdict_oob == "yes" {
                    label!("Recommended".green());
                } else {
                    label!("Not recommended".yellow());
                }
                lines_printed += 1;
            }

            if let Some(verdict) = &report.responses.verdict {
                if verdict == "yes" {
                    label!("Recommended (Tinker)".green());
                } else {
                    label!("Not Recommended (Tinker)".yellow());
                }
                lines_printed += 1;
            }
        }
    } else {
        label!("Borked".red());
        lines_printed += 1;
    }

    if let Some(verdict) = &report.responses.notes.verdict {
        label!(verdict.replace("\n", " ").bold());
        lines_printed += 1;
        *print_newline = true;
    }

    lines_printed
}

fn print_tinker_steps(report: &Report, tinker_steps: &Vec<String>, print_newline: &mut bool) -> u8 {
    let starts_play = report.responses.starts_play == Some("yes".to_string());
    let mut lines_printed = 0;

    if !tinker_steps.is_empty() {
        label!();
        label!("Tinker Steps", tinker_steps.join(", "));
        lines_printed += 2;
        *print_newline = true;
    }

    if let Some(launch_options) = &report.responses.launch_options {
        if !launch_options.is_empty() {
            label!(launch_options.trim().on_truecolor(68, 68, 68));
            lines_printed += 1;
            *print_newline = true;
        }
    }

    if let Some(customizations_used) = &report.responses.notes.customizations_used {
        label!();
        label!(customizations_used);
        lines_printed += 2;
    }

    if let Some(tinker_override) = &report.responses.notes.tinker_override {
        if !tinker_override.is_empty() {
            if *print_newline { label!(); }
            label!(tinker_override);
            lines_printed += 1;
        }
    }

    let installs = report.responses.installs.clone().unwrap_or("no".to_string());
    let opens = report.responses.opens.clone().unwrap_or("no".to_string());

    if installs == "no" || opens == "no" || !starts_play {
        // print newline before "Installs", "Opens" or "Starts Play"
        label!();
    }

    if installs == "no" || opens == "no" || !starts_play {
        label!("Installs", capitalize(&installs).yellow());
        lines_printed += 1;
    }

    if installs == "yes" && opens == "no" || !starts_play {
        label!("Opens", capitalize(&opens).yellow());
        lines_printed += 1;
    }

    if opens == "yes" && !starts_play {
        label!("Starts Play", "No".yellow());
        lines_printed += 1;
    }

    lines_printed
}

fn print_faults(report: &Report) -> u8 {
    let mut lines_printed = 0;

    if let Some(did_change_control_layout) = &report.responses.did_change_control_layout {
        if did_change_control_layout == "yes" {
            if let Some(control_layout) = &report.responses.control_layout {
                label!();
                let label = match control_layout.as_str() {
                    "official" => "Switch To Official".to_string(),
                    "community" => {
                        let mut label = "Switch to Community Layout".to_string();
                        if let Some(layout) = &report.responses.notes.control_layout {
                            label = format!("{}: {}", label, layout);
                        }
                        label
                    },
                    _ => "Unknown".to_string()
                };
                label!("Control Layout", label.yellow());
                lines_printed += 2;
            }
        }
    }

    if let Some(battery_performance) = &report.responses.battery_performance {
        if battery_performance == "yes" {
            label!();
            label!("Battery Performance", "Made Changes To Improve".yellow());
            if let Some(note) = &report.responses.notes.battery_performance {
                label!(note);
            }
            lines_printed += 2;
        }
    }

    macro_rules! print_faults {
        // with follow up
        ($faults:ident, $label:expr, true) => {
            if let Some(faults) = &report.responses.$faults {
                if faults == "yes" {
                    label!();

                    let faults = report.responses.follow_up.as_ref().unwrap().$faults.as_ref().unwrap().keys();
                    label!($label, faults.join(", ").yellow());
                    lines_printed += 1;

                    if let Some(note) = report.responses.notes.$faults.as_ref() {
                        label!(note);
                        lines_printed += 1;
                    }
                }
            }
        };
        // without follow up
        ($faults:ident, $label:expr, false) => {
            if let Some(faults) = &report.responses.$faults {
                if faults == "yes" {
                    label!();

                    label!($label, "Yes".yellow());
                    lines_printed += 1;

                    if let Some(note) = report.responses.notes.$faults.as_ref() {
                        label!(note);
                        lines_printed += 1;
                    }
                }
            }
        };
    }

    print_faults!(audio_faults, "Audio", true);
    print_faults!(graphical_faults, "Graphics", true);
    print_faults!(windowing_faults, "Windowing", true);
    print_faults!(input_faults, "Input", true);
    print_faults!(save_game_faults, "Save Game", true);
    print_faults!(performance_faults, "Performance", true);
    print_faults!(stability_faults, "Stability", true);
    print_faults!(significant_bugs, "Significant Bugs", false);
    print_faults!(readability, "Difficult To Read Text", false);

    lines_printed
}

fn print_concluding_notes(report: &Report) -> u8 {
    let mut lines_printed = 0;
    if let Some(concluding_notes) = &report.responses.concluding_notes {
        if !concluding_notes.is_empty() {
            label!();
            label!(concluding_notes.replace("\n", " "));
            lines_printed += 2;
        }
    } else if let Some(concluding_notes) = &report.responses.notes.concluding_notes {
        if !concluding_notes.is_empty() {
            label!();
            label!(concluding_notes.replace("\n", " "));
            lines_printed += 2;
        }
    }
    lines_printed
}

const AVATAR_WIDTH: u8 = 7;
const AVATAR_HEIGHT: u8 = 3;

pub(crate) async fn display_reports(reports: Reports, args: &Args, client: &Client, terminal_width: u16) {
    let avatars = fetch_avatars(client, &reports.reports.iter().filter(|report| !report.contributor.steam.avatar.is_empty()).map(|report| report.contributor.steam.avatar.clone()).collect()).await;

    let max_index = reports.reports.len().min(args.reports as usize);
    let reports = reports.reports[0..max_index].to_vec();
    for (index, report) in reports.iter().enumerate() {
        let mut lines_printed = 0;
        let tinker_steps = get_tinker_steps(&report);
        let mut print_newline = false;

        println!("{}", "—".repeat(terminal_width as usize));

        if args.images && index < avatars.len() {
            print_image(&avatars[index], AVATAR_WIDTH as u32, AVATAR_HEIGHT as u32)
        }

        lines_printed += print_timestamp(&report);
        lines_printed += print_header(&report);
        lines_printed += print_verdict(&report, &tinker_steps, &mut print_newline);
        lines_printed += print_tinker_steps(&report, &tinker_steps, &mut print_newline);
        lines_printed += print_faults(&report);
        lines_printed += print_concluding_notes(&report);

        if args.images && AVATAR_HEIGHT > lines_printed {
            print!("{}", "\n".repeat((AVATAR_HEIGHT - lines_printed) as usize));
        }
    }
}
