use std::sync::{Arc, Mutex};
use std::path::{Path, PathBuf};
use chrono::Local;
use screenshots::Screen;

pub struct ScreenRecorder {
    is_recording: Arc<Mutex<bool>>,
    output_dir: Arc<Mutex<Option<PathBuf>>>,
    start_time: Arc<Mutex<Option<std::time::Instant>>>,
    frame_count: Arc<Mutex<usize>>,
}

impl ScreenRecorder {
    pub fn new() -> Self {
        Self {
            is_recording: Arc::new(Mutex::new(false)),
            output_dir: Arc::new(Mutex::new(None)),
            start_time: Arc::new(Mutex::new(None)),
            frame_count: Arc::new(Mutex::new(0)),
        }
    }

    /// 녹화 시작 (연속 스크린샷 방식)
    pub fn start_recording(&self, output_dir: &str) -> Result<String, String> {
        let mut is_recording = self.is_recording.lock().unwrap();
        
        if *is_recording {
            return Err("Already recording".to_string());
        }

        // 출력 디렉토리 생성
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let session_dir = Path::new(output_dir).join(format!("recording_{}", timestamp));
        std::fs::create_dir_all(&session_dir)
            .map_err(|e| format!("Failed to create output directory: {}", e))?;

        let session_dir_str = session_dir.to_string_lossy().to_string();

        // 상태 업데이트
        *is_recording = true;
        *self.output_dir.lock().unwrap() = Some(session_dir.clone());
        *self.start_time.lock().unwrap() = Some(std::time::Instant::now());
        *self.frame_count.lock().unwrap() = 0;

        println!("Recording started: {}", session_dir_str);

        // 백그라운드 스레드에서 연속 캡처
        let is_recording_clone = Arc::clone(&self.is_recording);
        let output_dir_clone = Arc::clone(&self.output_dir);
        let frame_count_clone = Arc::clone(&self.frame_count);

        tokio::spawn(async move {
            // 화면 가져오기
            let screens = match Screen::all() {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Failed to get screens: {}", e);
                    return;
                }
            };

            let screen = match screens.first() {
                Some(s) => s,
                None => {
                    eprintln!("No screen found");
                    return;
                }
            };

            println!("Capturing screen: {}x{}", screen.display_info.width, screen.display_info.height);

            // 1초에 1프레임 캡처 (부하 줄이기)
            let frame_interval = tokio::time::Duration::from_secs(1);
            let mut interval = tokio::time::interval(frame_interval);

            while *is_recording_clone.lock().unwrap() {
                interval.tick().await;

                // 스크린샷 캡처
                let image = match screen.capture() {
                    Ok(img) => img,
                    Err(e) => {
                        eprintln!("Failed to capture: {}", e);
                        continue;
                    }
                };

                // 프레임 번호
                let mut frame_count = frame_count_clone.lock().unwrap();
                *frame_count += 1;
                let frame_num = *frame_count;
                drop(frame_count);

                // 파일 저장
                if let Some(output_dir) = output_dir_clone.lock().unwrap().as_ref() {
                    let filename = format!("frame_{:06}.png", frame_num);
                    let filepath = output_dir.join(filename);

                    let buffer = match image.to_png() {
                        Ok(buf) => buf,
                        Err(e) => {
                            eprintln!("Failed to encode PNG: {}", e);
                            continue;
                        }
                    };

                    if let Err(e) = std::fs::write(&filepath, buffer) {
                        eprintln!("Failed to write frame: {}", e);
                    }
                }
            }
            
            println!("Recording thread stopped");
        });

        Ok(session_dir_str)
    }

    /// 녹화 중지
    pub fn stop_recording(&self) -> Result<Option<String>, String> {
        let mut is_recording = self.is_recording.lock().unwrap();
        
        if !*is_recording {
            return Err("Not recording".to_string());
        }

        *is_recording = false;
        
        let output_dir = self.output_dir.lock().unwrap().as_ref().map(|p| p.to_string_lossy().to_string());
        let frame_count = *self.frame_count.lock().unwrap();
        
        *self.output_dir.lock().unwrap() = None;
        *self.start_time.lock().unwrap() = None;
        *self.frame_count.lock().unwrap() = 0;

        println!("Recording stopped. Total frames: {}", frame_count);
        
        Ok(output_dir)
    }

    /// 녹화 중인지 확인
    pub fn is_recording(&self) -> bool {
        *self.is_recording.lock().unwrap()
    }

    /// 녹화 시간 가져오기 (초 단위)
    pub fn get_recording_duration(&self) -> Option<u64> {
        let start_time = self.start_time.lock().unwrap();
        start_time.map(|st| st.elapsed().as_secs())
    }

    /// 현재 출력 디렉토리 경로
    pub fn get_output_path(&self) -> Option<String> {
        self.output_dir.lock().unwrap().as_ref().map(|p| p.to_string_lossy().to_string())
    }

    /// 현재까지 캡처한 프레임 수
    pub fn get_frame_count(&self) -> usize {
        *self.frame_count.lock().unwrap()
    }
}

impl Default for ScreenRecorder {
    fn default() -> Self {
        Self::new()
    }
}
