use serde::{Deserialize, Serialize};

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
    pub kernel: Option<String>,
    pub os: String,
    pub ram: Option<String>,
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
    pub save_game_faults: Option<String>,
    #[serde(rename = "customizationsUsed")]
    pub customizations_used: Option<String>,
    #[serde(rename = "batteryPerformance")]
    pub battery_performance: Option<String>,
    #[serde(rename = "readability")]
    pub readability: Option<String>,
    #[serde(rename = "concludingNotes")]
    pub concluding_notes: Option<String>,
    #[serde(rename = "controlLayout")]
    pub control_layout: Option<String>,
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
            PerformanceFaults::SlightSlowdown => faults.push("Slight Performance Problems".to_string()),
            PerformanceFaults::SignificantSlowdown => faults.push("Significant Performance Problems".to_string()),
        }
        faults
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct InputFaults {
    #[serde(rename = "controllerNotDetected")]
    pub controller_not_detected: Option<bool>,
    pub bounding: Option<bool>,
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
    pub other: Option<bool>,
    pub crackling: Option<bool>,
}

impl AudioFaults {
    pub fn keys(&self) -> Vec<String> {
        let mut faults = Vec::new();
        if self.low_quality.unwrap_or(false) { faults.push("Low Quality".to_string()) }
        if self.other.unwrap_or(false) { faults.push("Other".to_string()) }
        if self.crackling.unwrap_or(false) { faults.push("Crackling".to_string()) }
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
    pub answer_to_what_game: Option<String>,
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
    pub installs: Option<String>,
    pub launcher: Option<String>,
    pub notes: Notes,
    pub opens: Option<String>,
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
    pub verdict: Option<String>,
    #[serde(rename = "verdictOob")]
    pub verdict_oob: Option<String>,
    #[serde(rename = "windowingFaults")]
    pub windowing_faults: Option<String>,
    #[serde(rename = "protonVersion")]
    pub proton_version: Option<String>,
    #[serde(rename = "customProtonVersion")]
    pub custom_proton_version: Option<String>,
    #[serde(rename = "launchOptions")]
    pub launch_options: Option<String>,
    #[serde(rename = "batteryPerformance")]
    pub battery_performance: Option<String>,
    #[serde(rename = "controlLayout")]
    pub control_layout: Option<String>,
    #[serde(rename = "controlLayoutCustomization")]
    pub control_layout_customization: Option<String>,
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
    pub owned: Option<bool>,
    pub playtime: Option<u32>,
    pub avatar: String,
    pub nickname: String,
    #[serde(rename = "playtimeLinux")]
    pub playtime_linux: Option<u32>,
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