use reqwest::Client;
use serde::{Deserialize, Serialize};
use colored::Colorize;
use futures::future::join_all;
use image::DynamicImage;
use chrono_humanize::HumanTime;
use crate::args::Args;
use crate::utils::{calculate_protondb_id, Counts, print_image};

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct ChromeOS {
    pub board: String,
    pub channel: String,
    #[serde(rename = "chromeVersion")]
    pub chrome_version: String,
    pub platform: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Specs {
    pub cpu: String,
    pub gpu: String,
    #[serde(rename = "gpuDriver")]
    pub gpu_driver: String,
    pub kernel: String,
    pub os: String,
    pub ram: String,
    #[serde(rename = "steamRuntimeVersion")]
    pub steam_runtime_version: Option<String>,
    #[serde(rename = "xWindowManager")]
    pub x_window_manager: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Inferred {
    pub steam: Specs,
    #[serde(rename = "chromeOs")]
    pub chrome_os: Option<ChromeOS>,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Device {
    #[serde(rename = "hardwareType")]
    pub hardware_type: String,
    pub inferred: Inferred,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Notes {
    #[serde(rename = "audioFaults")]
    pub audio_faults: Option<String>,
    #[serde(rename = "graphicalFaults")]
    pub graphical_faults: Option<String>,
    #[serde(rename = "inputFaults")]
    pub input_faults: Option<String>,
    #[serde(rename = "performanceFaults")]
    pub performance_faults: Option<String>,
    pub verdict: Option<String>,
    #[serde(rename = "stabilityFaults")]
    pub stability_faults: Option<String>,
    #[serde(rename = "significantBugs")]
    pub significant_bugs: Option<String>,
    #[serde(rename = "tinkerOverride")]
    pub tinker_override: Option<String>,
    pub launcher: Option<String>,
    #[serde(rename = "windowingFaults")]
    pub windowing_faults: Option<String>,
    #[serde(rename = "saveGameFaults")]
    pub save_game_faults: Option<String>
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct SaveGameFaults {
    #[serde(rename = "errorLoading")]
    pub error_loading: Option<bool>,
    #[serde(rename = "saveNotLoading")]
    pub other: Option<bool>
}

impl SaveGameFaults {
    pub fn keys(&self) -> Vec<String> {
        let mut faults = Vec::new();
        if self.error_loading.unwrap_or(false) { faults.push("Loading".to_string()) }
        if self.other.unwrap_or(false) { faults.push("Other".to_string()) }
        faults
    }
}

#[derive(Serialize, Deserialize, Clone, strum_macros::EnumString, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) enum StabilityFaults {
    Occasionally,
    NotListed,
    FrequentCrashes
}

impl StabilityFaults {
    pub fn keys(&self) -> Vec<String> {
        let mut faults = Vec::new();
        match self {
            StabilityFaults::Occasionally => faults.push("Occasionally".to_string()),
            StabilityFaults::NotListed => faults.push("Not Listed".to_string()),
            StabilityFaults::FrequentCrashes => faults.push("Frequent Crashes".to_string()),
        }
        faults
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct WindowingFaults {
    #[serde(rename = "fullNotFull")]
    pub full_not_full: Option<bool>,
    pub other: Option<bool>,
    pub switching: Option<bool>,
}

impl WindowingFaults {
    pub fn keys(&self) -> Vec<String> {
        let mut faults = Vec::new();
        if self.full_not_full.unwrap_or(false) { faults.push("Size".to_string()) }
        if self.other.unwrap_or(false) { faults.push("Other".to_string()) }
        if self.switching.unwrap_or(false) { faults.push("Switching".to_string()) }
        faults
    }
}

#[derive(Serialize, Deserialize, Clone, strum_macros::EnumString)]
#[serde(rename_all = "camelCase")]
pub(crate) enum PerformanceFaults {
    SlightSlowdown,
    SignificantSlowdown
}

impl PerformanceFaults {
    pub fn keys(&self) -> Vec<String> {
        let mut faults = Vec::new();
        match self {
            PerformanceFaults::SlightSlowdown => faults.push("Slight Slowdown".to_string()),
            PerformanceFaults::SignificantSlowdown => faults.push("Significant Performance Problems".to_string()),
        }
        faults
    }
}

fn deserialize_input_faults_other<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
    where D: serde::Deserializer<'de> {
    let s: serde_json::Value = Deserialize::deserialize(deserializer)?;
    return if s["0"] == "on" {
        Ok(Some(false))
    } else if s == "other" {
        Ok(Some(true))
    } else {
        Ok(None)
    }
}

fn none() -> Option<bool> {
    None
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct InputFaults {
    #[serde(rename = "controllerNotDetected")]
    pub controller_not_detected: Option<bool>,
    pub bounding: Option<bool>,
    #[serde(deserialize_with = "deserialize_input_faults_other", default = "none")]
    pub other: Option<bool>,
    pub lag: Option<bool>,
}

impl InputFaults {
    pub fn keys(&self) -> Vec<String> {
        let mut faults = Vec::new();
        if self.controller_not_detected.unwrap_or(false) { faults.push("Controller Not Detected".to_string()) }
        if self.bounding.unwrap_or(false) { faults.push("Bounding".to_string()) }
        if self.other.unwrap_or(false) { faults.push("Other".to_string()) }
        if self.lag.unwrap_or(false) { faults.push("Lag".to_string()) }
        faults
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct GraphicalFaults {
    #[serde(rename = "minorArtifacts")]
    pub minor_artifacts: Option<bool>,
    pub other: Option<bool>,
    #[serde(rename = "heavyArtifacts")]
    pub heavy_artifacts: Option<bool>,
}

impl GraphicalFaults {
    pub fn keys(&self) -> Vec<String> {
        let mut faults = Vec::new();
        if self.minor_artifacts.unwrap_or(false) { faults.push("Minor Artifacts".to_string()) }
        if self.other.unwrap_or(false) { faults.push("Other".to_string()) }
        if self.heavy_artifacts.unwrap_or(false) { faults.push("Heavy Artifacts".to_string()) }
        faults
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct AudioFaults {
    #[serde(rename = "lowQuality")]
    pub low_quality: Option<bool>,
    pub other: Option<bool>
}

impl AudioFaults {
    pub fn keys(&self) -> Vec<String> {
        let mut faults = Vec::new();
        if self.low_quality.unwrap_or(false) { faults.push("Low Quality".to_string()) }
        if self.other.unwrap_or(false) { faults.push("Other".to_string()) }
        faults
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct FollowUp {
    #[serde(rename = "audioFaults")]
    pub audio_faults: Option<AudioFaults>,
    #[serde(rename = "graphicalFaults")]
    pub graphical_faults: Option<GraphicalFaults>,
    #[serde(rename = "inputFaults")]
    pub input_faults: Option<InputFaults>,
    #[serde(rename = "performanceFaults")]
    pub performance_faults: Option<PerformanceFaults>,
    #[serde(rename = "windowingFaults")]
    pub windowing_faults: Option<WindowingFaults>,
    #[serde(rename = "stabilityFaults")]
    pub stability_faults: Option<StabilityFaults>,
    #[serde(rename = "saveGameFaults")]
    pub save_game_faults: Option<SaveGameFaults>,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct CustomizationsUsed {
    #[serde(rename = "configChange")]
    pub config_change: Option<bool>,
    pub winetricks: Option<bool>,
    pub lutris: Option<bool>,
    #[serde(rename = "mediaFoundation")]
    pub media_foundation: Option<bool>,
    pub protontricks: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Responses {
    #[serde(rename = "answerToWhatGame")]
    pub answer_to_what_game: String,
    #[serde(rename = "audioFaults")]
    pub audio_faults: Option<String>,
    #[serde(rename = "concludingNotes")]
    pub concluding_notes: Option<String>,
    #[serde(rename = "customizationsUsed")]
    pub customizations_used: Option<CustomizationsUsed>,
    #[serde(rename = "followUp")]
    pub follow_up: Option<FollowUp>,
    #[serde(rename = "graphicalFaults")]
    pub graphical_faults: Option<String>,
    #[serde(rename = "inputFaults")]
    pub input_faults: Option<String>,
    pub installs: String,
    pub launcher: Option<String>,
    pub notes: Notes,
    pub opens: String,
    #[serde(rename = "performanceFaults")]
    pub performance_faults: Option<String>,
    #[serde(rename = "saveGameFaults")]
    pub save_game_faults: Option<String>,
    #[serde(rename = "significantBugs")]
    pub significant_bugs: Option<String>,
    #[serde(rename = "stabilityFaults")]
    pub stability_faults: Option<String>,
    #[serde(rename = "startsPlay")]
    pub starts_play: Option<String>,
    #[serde(rename = "tinkerOverride")]
    pub tinker_override: Option<String>,
    #[serde(rename = "triedOob")]
    pub tried_oob: Option<String>,
    pub variant: Option<String>,
    pub verdict: String,
    #[serde(rename = "verdictOob")]
    pub verdict_oob: Option<String>,
    #[serde(rename = "windowingFaults")]
    pub windowing_faults: Option<String>,
    #[serde(rename = "protonVersion")]
    pub proton_version: String,
    #[serde(rename = "customProtonVersion")]
    pub custom_proton_version: Option<String>,
    #[serde(rename = "launchOptions")]
    pub launch_options: Option<String>,
    #[serde(rename = "batteryPerformance")]
    pub battery_performance: Option<String>,
    #[serde(rename = "didChangeControlLayout")]
    pub did_change_control_layout: Option<String>,
    pub readability: Option<String>,
    #[serde(rename = "secondaryLauncher")]
    pub secondary_launcher: Option<String>,
    #[serde(rename = "appSelectionMethod")]
    pub app_selection_method: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Steam {
    pub owned: bool,
    pub playtime: u16,
    pub avatar: String,
    pub nickname: String,
    #[serde(rename = "playtimeLinux")]
    pub playtime_linux: Option<u16>,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Contributor {
    pub id: String,
    #[serde(rename = "reportTally")]
    pub report_tally: u16,
    pub steam: Steam,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Report {
    pub contributor: Contributor,
    pub id: String,
    pub responses: Responses,
    pub timestamp: u32,
    pub device: Device,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Reports {
    pub page: u8,
    #[serde(rename = "perPage")]
    pub per_page: u8,
    pub reports: Vec<Report>,
    pub total: u16,
}

pub(crate) async fn fetch_reports(client: &Client, steam_id: u32) -> Reports {
    let counts = client
        .get("https://www.protondb.com/data/counts.json")
        .send().await.unwrap()
        .json::<Counts>().await.unwrap();

    let protondb_id = calculate_protondb_id(steam_id, counts.reports, counts.timestamp);
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
            tinker_steps.push("Custom Proton: GE".to_string());
        } else if variant == "notListed" {
            tinker_steps.push("Custom Proton".to_string());
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

pub(crate) async fn display_reports(reports: Reports, args: &Args, client: &Client, terminal_width: u16) {
    let avatars = fetch_avatars(client, &reports.reports.iter().map(|report| report.contributor.steam.avatar.clone()).collect()).await;

    const IMAGE_WIDTH: u32 = 7;
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
        () => {
            println!();
            lines_printed += 1;
        };
    }

    let max_index = reports.reports.len().min(args.reports as usize);
    let reports = reports.reports[0..max_index].to_vec();
    for (index, report) in reports.iter().enumerate() {
        lines_printed = 0;

        println!("{}", "—".repeat(terminal_width as usize));

        if args.images { print_image(&avatars[index], IMAGE_WIDTH, IMAGE_HEIGHT) }

        let timestamp = chrono::DateTime::from_timestamp(report.timestamp as i64, 0).unwrap();
        label!(HumanTime::from(timestamp).to_string().truecolor(120, 120, 120).italic());

        let hours = report.contributor.steam.playtime / 60;
        label!(format!("{} {} | {}",
            report.contributor.steam.nickname,
            format!("({} reports)", report.contributor.report_tally).truecolor(120, 120, 120),
            format!("{}{} hour{} overall",
                if hours == 0 { "< " } else { "" },
                hours.max(1),
                if hours == 0 || hours == 1 { "" } else { "s" }
            ).truecolor(200, 200, 200)
        ));

        let tinker_steps = get_tinker_steps(&report);

        if report.responses.opens == "yes" {
            if tinker_steps.is_empty() {
                if report.responses.verdict == "yes" {
                    label!("Recommended".green());
                } else {
                    label!("Not Recommended".yellow());
                }
            } else {
                if let Some(verdict_oob) = &report.responses.verdict_oob {
                    if verdict_oob == "yes" {
                        label!("Recommended".green());
                    } else {
                        label!("Not recommended".yellow());
                    }
                }

                if report.responses.verdict == "yes" {
                    label!("Recommended (Tinker)".green());
                } else {
                    label!("Not Recommended (Tinker)".yellow());
                }
            }
        } else {
            label!("Borked".red());
        }

        let mut print_newline = false;

        if let Some(verdict) = &report.responses.notes.verdict {
            label!(verdict.replace("\n", " ").bold());
            print_newline = true;
        }

        if report.responses.opens == "no" {
            label!("Installs", if report.responses.installs == "yes" { "Yes" } else { "No" }.yellow());
            label!("Opens", if report.responses.opens == "yes" { "Yes" } else { "No" }.yellow());
        }

        if !tinker_steps.is_empty() {
            label!("Tinker Steps", tinker_steps.join(", "));
            print_newline = true;
        }

        if let Some(launch_options) = &report.responses.launch_options {
            label!(launch_options.on_truecolor(68, 68, 68));
            print_newline = true;
        }

        if let Some(tinker_override) = &report.responses.notes.tinker_override {
            if !tinker_override.is_empty() {
                if print_newline { label!(); }
                label!(tinker_override);
            }
        }

        macro_rules! print_faults_follow_up {
            ($faults:ident, $label:expr) => {
                if let Some(faults) = &report.responses.$faults {
                    if faults == "yes" {
                        label!();
                        let faults = report.responses.follow_up.as_ref().unwrap().$faults.as_ref().unwrap().keys();
                        label!($label, faults.join(", ").yellow());
                        if let Some(note) = report.responses.notes.$faults.as_ref() {
                            label!(note);
                        }
                    }
                }
            };
        }

        macro_rules! print_faults {
            ($faults:ident, $label:expr) => {
                if let Some(faults) = &report.responses.$faults {
                    if faults == "yes" {
                        label!();
                        label!($label, "Yes".yellow());
                        label!(report.responses.notes.$faults.as_ref().unwrap());
                    }
                }
            };
        }

        print_faults_follow_up!(audio_faults, "Audio");
        print_faults_follow_up!(graphical_faults, "Graphics");
        print_faults_follow_up!(windowing_faults, "Windowing");
        print_faults_follow_up!(input_faults, "Input");
        print_faults_follow_up!(save_game_faults, "Save Game");
        print_faults_follow_up!(performance_faults, "Performance");
        print_faults_follow_up!(stability_faults, "Stability");
        print_faults!(significant_bugs, "Significant Bugs");

        if let Some(concluding_notes) = &report.responses.concluding_notes {
            if !concluding_notes.is_empty() {
                label!();
                label!(concluding_notes.replace("\n", " "));
            }
        }

        if args.images && IMAGE_HEIGHT > lines_printed {
            print!("{}", "\n".repeat((IMAGE_HEIGHT - lines_printed) as usize));
        }
    }
}