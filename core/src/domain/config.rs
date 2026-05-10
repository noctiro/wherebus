#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum LocationMode {
    Auto,
    Manual,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AppConfig {
    pub service_id: String,
    pub location_mode: LocationMode,
    pub manual_location: Option<(f64, f64)>,
    #[serde(skip)]
    pub auto_location: Option<(f64, f64)>,
    pub theme_index: i32,
    #[serde(default)]
    pub onboarding_done: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            service_id: String::new(),
            location_mode: LocationMode::Auto,
            manual_location: None,
            auto_location: None,
            theme_index: 0,
            onboarding_done: false,
        }
    }
}

impl AppConfig {
    pub fn effective_location(&self) -> Option<(f64, f64)> {
        match self.location_mode {
            LocationMode::Auto => self.auto_location,
            LocationMode::Manual => self.manual_location,
        }
    }
}
