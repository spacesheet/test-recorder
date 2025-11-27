use xcap::Window;
use std::path::Path;
use chrono::Local;

pub struct WindowCapture {
    target_window: Option<Window>,
}

impl WindowCapture {
    pub fn new() -> Self {
        Self {
            target_window: None,
        }
    }

    /// 윈도우 타이틀로 찾기
    pub fn find_window_by_title(&mut self, title_contains: &str) -> Result<(), String> {
        let windows = Window::all().map_err(|e| format!("Failed to get windows: {}", e))?;
        
        for window in windows {
            let window_title = window.title();
            if window_title.to_lowercase().contains(&title_contains.to_lowercase()) {
                println!("Found window: {}", window_title);
                self.target_window = Some(window);
                return Ok(());
            }
        }
        
        Err(format!("Window containing '{}' not found", title_contains))
    }

    /// 모든 윈도우 리스트 가져오기
    pub fn list_all_windows() -> Result<Vec<String>, String> {
        let windows = Window::all().map_err(|e| format!("Failed to get windows: {}", e))?;
        
        Ok(windows
            .into_iter()
            .map(|w| format!("{} ({}x{})", w.title(), w.width(), w.height()))
            .collect())
    }

    /// 현재 타겟 윈도우 캡처
    pub fn capture_window(&self) -> Result<Vec<u8>, String> {
        if let Some(window) = &self.target_window {
            let image = window
                .capture_image()
                .map_err(|e| format!("Failed to capture: {}", e))?;
            
            // xcap의 ImageBuffer를 PNG로 인코딩
            let mut buffer = Vec::new();
            let cursor = std::io::Cursor::new(&mut buffer);
            
            // DynamicImage로 변환 후 PNG 저장
            use xcap::image::DynamicImage;
            let dynamic = DynamicImage::ImageRgba8(image);
            dynamic
                .write_to(cursor, xcap::image::ImageFormat::Png)
                .map_err(|e| format!("Failed to encode PNG: {}", e))?;
            
            Ok(buffer)
        } else {
            Err("No target window set".to_string())
        }
    }

    /// 캡처해서 파일로 저장
    pub fn capture_and_save(&self, output_dir: &str, prefix: &str) -> Result<String, String> {
        let image_data = self.capture_window()?;
        
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let filename = format!("{}_{}.png", prefix, timestamp);
        let filepath = Path::new(output_dir).join(&filename);
        
        std::fs::create_dir_all(output_dir)
            .map_err(|e| format!("Failed to create directory: {}", e))?;
        
        std::fs::write(&filepath, image_data)
            .map_err(|e| format!("Failed to write file: {}", e))?;
        
        Ok(filepath.to_string_lossy().to_string())
    }

    /// 전체 화면 캡처
    pub fn capture_primary_screen() -> Result<Vec<u8>, String> {
        let screens = screenshots::Screen::all()
            .map_err(|e| format!("Failed to get screens: {}", e))?;
        
        if let Some(screen) = screens.first() {
            let image = screen
                .capture()
                .map_err(|e| format!("Failed to capture screen: {}", e))?;
            
            // PNG로 인코딩
            image
                .to_png()
                .map_err(|e| format!("Failed to encode PNG: {}", e))
        } else {
            Err("No screen found".to_string())
        }
    }

    /// 전체 화면을 파일로 저장
    pub fn capture_screen_and_save(output_dir: &str, prefix: &str) -> Result<String, String> {
        let image_data = Self::capture_primary_screen()?;
        
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let filename = format!("{}_{}.png", prefix, timestamp);
        let filepath = Path::new(output_dir).join(&filename);
        
        std::fs::create_dir_all(output_dir)
            .map_err(|e| format!("Failed to create directory: {}", e))?;
        
        std::fs::write(&filepath, image_data)
            .map_err(|e| format!("Failed to write file: {}", e))?;
        
        Ok(filepath.to_string_lossy().to_string())
    }

    /// 윈도우 정보 가져오기
    pub fn get_window_info(&self) -> Option<(String, u32, u32)> {
        self.target_window.as_ref().map(|w| {
            (w.title().to_string(), w.width(), w.height())
        })
    }
}

impl Default for WindowCapture {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_capture_creation() {
        let capture = WindowCapture::new();
        assert!(capture.target_window.is_none());
    }

    #[test]
    fn test_list_all_windows() {
        let windows = WindowCapture::list_all_windows();
        assert!(windows.is_ok());
        if let Ok(list) = windows {
            println!("Found {} windows", list.len());
        }
    }
}
