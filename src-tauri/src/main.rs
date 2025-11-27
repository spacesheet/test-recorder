#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, State, AppHandle};
use std::sync::{Arc, Mutex};
use tokio::time::{interval, Duration};

mod models;
mod process_monitor;
mod recorder;
mod window_capture;

use models::*;
use process_monitor::ProcessMonitor;
use recorder::ScreenRecorder;
use window_capture::WindowCapture;

// 전역 상태
#[derive(Clone)]
struct AppState {
    recorder: Arc<Mutex<ScreenRecorder>>,
    monitor: Arc<Mutex<ProcessMonitor>>,
    capture: Arc<Mutex<WindowCapture>>,
    config: Arc<Mutex<AppConfig>>,
    trade_history: Arc<Mutex<Vec<TradeEvent>>>,
}

// Tauri 명령어들

#[tauri::command]
async fn start_monitoring(state: State<'_, AppState>) -> Result<String, String> {
    let config = state.config.lock().unwrap().clone();
    let recorder = Arc::clone(&state.recorder);
    
    let output_dir = config.output_dir.clone();
    let result = recorder.lock().unwrap().start_recording(&output_dir)?;
    
    Ok(format!("Monitoring started. Recording to: {}", result))
}

#[tauri::command]
async fn stop_monitoring(state: State<'_, AppState>) -> Result<String, String> {
    let recorder = Arc::clone(&state.recorder);
    
    let result = {
        let rec = recorder.lock().unwrap();
        rec.stop_recording()?
    };
    
    match result {
        Some(path) => Ok(format!("Recording saved to: {}", path)),
        None => Ok("Monitoring stopped".to_string()),
    }
}

#[tauri::command]
async fn get_recording_status(state: State<'_, AppState>) -> Result<RecordingStatus, String> {
    let recorder = state.recorder.lock().unwrap();
    let monitor = state.monitor.lock().unwrap();
    
    let is_recording = recorder.is_recording();
    let hts_detected_name = monitor.is_target_running();
    let recording_duration = recorder.get_recording_duration();
    
    Ok(RecordingStatus {
        is_recording,
        hts_detected: hts_detected_name.is_some(),
        hts_name: hts_detected_name,
        recording_duration,
    })
}

#[tauri::command]
async fn capture_screenshot(state: State<'_, AppState>) -> Result<String, String> {
    let config = state.config.lock().unwrap();
    let output_dir = config.output_dir.clone();
    
    WindowCapture::capture_screen_and_save(&output_dir, "screenshot")
}

#[tauri::command]
async fn list_windows() -> Result<Vec<String>, String> {
    WindowCapture::list_all_windows()
}

#[tauri::command]
async fn get_trade_history(state: State<'_, AppState>) -> Result<Vec<TradeEvent>, String> {
    let history = state.trade_history.lock().unwrap();
    Ok(history.clone())
}

#[tauri::command]
async fn get_config(state: State<'_, AppState>) -> Result<AppConfig, String> {
    let config = state.config.lock().unwrap();
    Ok(config.clone())
}

#[tauri::command]
async fn update_config(
    state: State<'_, AppState>,
    new_config: AppConfig,
) -> Result<(), String> {
    let mut config = state.config.lock().unwrap();
    *config = new_config;
    Ok(())
}

// 백그라운드 모니터링 태스크
async fn background_monitoring_task(app_handle: AppHandle, state: Arc<AppState>) {
    let mut check_interval = interval(Duration::from_secs(1));
    let mut was_hts_running = false;
    
    loop {
        check_interval.tick().await;
        
        let monitor = state.monitor.lock().unwrap();
        let is_hts_running = monitor.is_target_running().is_some();
        drop(monitor);
        
        // HTS 상태 변경 감지
        if is_hts_running && !was_hts_running {
            println!("HTS detected! Starting recording...");
            
            let recorder = state.recorder.lock().unwrap();
            let config = state.config.lock().unwrap();
            let _ = recorder.start_recording(&config.output_dir);
            
            // 프론트엔드에 알림
            let _ = app_handle.emit_all("hts-detected", true);
            let _ = app_handle.emit_all("recording-started", ());
            
        } else if !is_hts_running && was_hts_running {
            println!("HTS closed! Stopping recording...");
            
            let recorder = state.recorder.lock().unwrap();
            let _ = recorder.stop_recording();
            
            // 프론트엔드에 알림
            let _ = app_handle.emit_all("hts-detected", false);
            let _ = app_handle.emit_all("recording-stopped", ());
        }
        
        was_hts_running = is_hts_running;
        
        // 주기적으로 상태 전송
        if is_hts_running {
            let recorder = state.recorder.lock().unwrap();
            let duration = recorder.get_recording_duration();
            let _ = app_handle.emit_all("recording-duration", duration);
        }
    }
}

fn main() {
    // 설정 로드
    let config = AppConfig::default();
    
    // 상태 초기화
    let app_state = Arc::new(AppState {
        recorder: Arc::new(Mutex::new(ScreenRecorder::new())),
        monitor: Arc::new(Mutex::new(ProcessMonitor::new(
            config.hts.process_names.clone(),
        ))),
        capture: Arc::new(Mutex::new(WindowCapture::new())),
        config: Arc::new(Mutex::new(config)),
        trade_history: Arc::new(Mutex::new(Vec::new())),
    });
    
    tauri::Builder::default()
        .manage(app_state.clone())
        .invoke_handler(tauri::generate_handler![
            start_monitoring,
            stop_monitoring,
            get_recording_status,
            capture_screenshot,
            list_windows,
            get_trade_history,
            get_config,
            update_config,
        ])
        .setup(move |app| {
            let app_handle = app.handle();
            
            // Tokio 런타임에서 백그라운드 태스크 실행
            tauri::async_runtime::spawn(async move {
                background_monitoring_task(app_handle, app_state).await;
            });
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
