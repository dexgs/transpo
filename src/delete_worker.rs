use super::config;
use super::time;
use std::time::Duration;
use std::path::PathBuf;

pub fn spawn() {
    std::thread::spawn(|| {
        let storage_dir = PathBuf::from(config::STORAGE_PATH);
        loop {
            if let Ok(entries) = std::fs::read_dir(&storage_dir) {
                for dir_entry in entries {
                    if let Ok(dir) = dir_entry {
                        let dir = dir.path();
                        if time::is_expired_sync(dir.join("expiry_date")).unwrap_or(true) {
                            drop(std::fs::remove_dir_all(dir));
                        }
                    }
                }
            }
            std::thread::sleep(Duration::from_secs(3600));
        }
    });
}
