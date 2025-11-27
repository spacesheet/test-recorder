use sysinfo::System;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time;

pub struct ProcessMonitor {
    system: Arc<Mutex<System>>,
    target_processes: Vec<String>,
}

impl ProcessMonitor {
    pub fn new(target_processes: Vec<String>) -> Self {
        Self {
            system: Arc::new(Mutex::new(System::new_all())),
            target_processes,
        }
    }

    pub fn is_target_running(&self) -> Option<String> {
        let mut system = self.system.lock().unwrap();
        system.refresh_processes();
        
        for (_, process) in system.processes() {
            let process_name = process.name().to_lowercase();
            
            for target in &self.target_processes {
                if process_name.contains(&target.to_lowercase()) {
                    return Some(process.name().to_string());
                }
            }
        }
        
        None
    }

    pub fn find_process_pid(&self, process_name: &str) -> Option<u32> {
        let mut system = self.system.lock().unwrap();
        system.refresh_processes();
        
        for (pid, process) in system.processes() {
            if process.name().to_lowercase().contains(&process_name.to_lowercase()) {
                return Some(pid.as_u32());
            }
        }
        
        None
    }

    pub async fn start_monitoring<F>(
        &self,
        interval_ms: u64,
        mut callback: F,
    ) -> Result<(), String>
    where
        F: FnMut(Option<String>) + Send + 'static,
    {
        let mut was_running: Option<String> = None;
        
        loop {
            let is_running = self.is_target_running();
            
            if is_running != was_running {
                callback(is_running.clone());
                was_running = is_running;
            }
            
            time::sleep(Duration::from_millis(interval_ms)).await;
        }
    }
}
