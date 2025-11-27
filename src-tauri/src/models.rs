use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingStatus {
    pub is_recording: bool,
    pub hts_detected: bool,
    pub hts_name: Option<String>,
    pub recording_duration: Option<u64>, // 초 단위
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeEvent {
    pub action: TradeAction,
    pub timestamp: DateTime<Utc>,
    pub screenshot_path: String,
    pub window_title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TradeAction {
    Buy,
    Sell,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HTSConfig {
    pub process_names: Vec<String>,
    pub window_titles: Vec<String>,
    pub check_interval_ms: u64,
}

impl Default for HTSConfig {
    fn default() -> Self {
        Self {
            process_names: vec![
                "kiwoom.exe".to_string(),
                "eFriend.exe".to_string(),
                "Ctrade.exe".to_string(),
                "KOAStudio.exe".to_string(),
                "hable.exe".to_string()
            ],
            window_titles: vec![
                "키움".to_string(),
                "영웅문".to_string(),
                "이베스트".to_string(),
                "키움".to_string(),
                "KB증권".to_string()
            ],
            check_interval_ms: 1000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub hts: HTSConfig,
    pub output_dir: String,
    pub fps: u32,
    pub enable_ocr: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            hts: HTSConfig::default(),
            output_dir: "./recordings".to_string(),
            fps: 30,
            enable_ocr: false,
        }
    }
}
