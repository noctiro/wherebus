use std::path::PathBuf;
use std::sync::OnceLock;

static ANDROID_DATA_DIR: OnceLock<PathBuf> = OnceLock::new();

pub fn set_android_data_dir(path: impl Into<PathBuf>) {
    let _ = ANDROID_DATA_DIR.set(path.into());
}

pub fn data_dir() -> PathBuf {
    #[cfg(target_os = "android")]
    {
        if let Some(path) = ANDROID_DATA_DIR.get() {
            return path.clone();
        }

        if let Ok(path) = std::env::var("WHEREBUS_DATA_DIR") {
            return PathBuf::from(path);
        }

        PathBuf::from("/data/data/com.noctiro.wherebus/files")
    }

    #[cfg(target_os = "ios")]
    {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join("Library/Application Support/wherebus")
    }

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("wherebus")
    }
}
