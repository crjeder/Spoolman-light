use std::{env, path::PathBuf};

/// All server configuration, parsed from environment variables at startup.
#[derive(Debug, Clone)]
pub struct Config {
    /// Path to the JSON data file.
    pub data_file: PathBuf,
    /// Bind host (default: 0.0.0.0).
    pub host: String,
    /// Bind port (default: 8000).
    pub port: u16,
    /// URL base path prefix (default: "").
    pub base_path: String,
    /// Enable debug mode (default: false).
    pub debug_mode: bool,
    /// Log level string passed to tracing-subscriber (default: "info").
    pub logging_level: String,
    /// CORS allowed origin.  `None` disables CORS headers entirely.
    pub cors_origin: Option<String>,
    /// Enable automatic daily backup with 5 rotating copies (default: true).
    pub automatic_backup: bool,
    /// App version string (from Cargo.toml via env at compile time).
    pub version: String,
    /// Path to the compiled WASM frontend assets directory.
    /// Defaults to `target/site` (cargo-leptos dev output).
    /// Set `LEPTOS_SITE_ROOT` in production/container deployments.
    pub site_root: PathBuf,
}

impl Config {
    pub fn from_env() -> Self {
        let data_dir = default_data_dir();
        let data_file = env_var("SPOOLMAN_DATA_FILE")
            .map(PathBuf::from)
            .unwrap_or_else(|| data_dir.join("spoolman.json"));

        let host = env_var("SPOOLMAN_HOST").unwrap_or_else(|| "0.0.0.0".to_string());
        let port = env_var("SPOOLMAN_PORT")
            .and_then(|p| p.parse().ok())
            .unwrap_or(8000u16);

        let base_path = env_var("SPOOLMAN_BASE_PATH").unwrap_or_default();

        let debug_mode = env_var("SPOOLMAN_DEBUG_MODE")
            .map(|v| v.to_uppercase() == "TRUE")
            .unwrap_or(false);

        let logging_level = env_var("SPOOLMAN_LOGGING_LEVEL")
            .unwrap_or_else(|| "info".to_string())
            .to_lowercase();

        // FALSE (exact string) disables CORS; anything else is treated as the
        // allowed origin.
        let cors_origin = env_var("SPOOLMAN_CORS_ORIGIN").and_then(|v| {
            if v.to_uppercase() == "FALSE" {
                None
            } else {
                Some(v)
            }
        });

        let automatic_backup = env_var("SPOOLMAN_AUTOMATIC_BACKUP")
            .map(|v| v.to_uppercase() != "FALSE")
            .unwrap_or(true);

        let version = env!("CARGO_PKG_VERSION").to_string();

        // cargo-leptos sets LEPTOS_SITE_ROOT in production builds.
        // Defaults to `target/site` for local dev.
        let site_root = env_var("LEPTOS_SITE_ROOT")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("target/site"));

        Self {
            data_file,
            host,
            port,
            base_path,
            debug_mode,
            logging_level,
            cors_origin,
            automatic_backup,
            version,
            site_root,
        }
    }
}

fn env_var(key: &str) -> Option<String> {
    env::var(key).ok().filter(|v| !v.is_empty())
}

/// Platform-appropriate default data directory, matching the Python implementation.
fn default_data_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        let appdata = env::var("APPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("C:/ProgramData"));
        appdata.join("Spoolman")
    }
    #[cfg(target_os = "macos")]
    {
        let home = env::var("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/tmp"));
        home.join("Library/Application Support/Spoolman")
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        let home = env::var("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/tmp"));
        // Follow XDG_DATA_HOME if set.
        env::var("XDG_DATA_HOME")
            .map(|d| PathBuf::from(d).join("spoolman"))
            .unwrap_or_else(|_| home.join(".local/share/spoolman"))
    }
}
