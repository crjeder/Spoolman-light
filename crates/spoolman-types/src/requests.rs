use crate::models::{MaterialType, Rgba};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ── Filament ───────────────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateFilament {
    pub manufacturer: Option<String>,
    pub material: Option<MaterialType>,
    pub material_modifier: Option<String>,
    #[serde(default = "default_diameter")]
    pub diameter: f32,
    pub net_weight: Option<f32>,
    pub density: f32,
    pub print_temp: Option<i32>,
    pub bed_temp: Option<i32>,
    pub spool_weight: Option<f32>,
    pub min_print_temp: Option<i32>,
    pub max_print_temp: Option<i32>,
    pub min_bed_temp: Option<i32>,
    pub max_bed_temp: Option<i32>,
    pub comment: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct UpdateFilament {
    pub manufacturer: Option<String>,
    pub material: Option<MaterialType>,
    pub material_modifier: Option<String>,
    pub diameter: Option<f32>,
    pub net_weight: Option<f32>,
    pub density: Option<f32>,
    pub print_temp: Option<i32>,
    pub bed_temp: Option<i32>,
    pub spool_weight: Option<f32>,
    pub min_print_temp: Option<i32>,
    pub max_print_temp: Option<i32>,
    pub min_bed_temp: Option<i32>,
    pub max_bed_temp: Option<i32>,
    pub comment: Option<String>,
}

// ── Spool ──────────────────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateSpool {
    pub filament_id: u32,
    pub colors: Vec<Rgba>,
    pub color_name: Option<String>,
    pub location_id: Option<u32>,
    pub initial_weight: f32,
    pub first_used: Option<DateTime<Utc>>,
    pub last_used: Option<DateTime<Utc>>,
    pub comment: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct UpdateSpool {
    pub colors: Option<Vec<Rgba>>,
    pub color_name: Option<String>,
    pub location_id: Option<u32>,
    pub current_weight: Option<f32>,
    pub first_used: Option<DateTime<Utc>>,
    pub last_used: Option<DateTime<Utc>>,
    pub comment: Option<String>,
    pub archived: Option<bool>,
}

// ── Location ───────────────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateLocation {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateLocation {
    pub name: String,
}

// ── Settings ───────────────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PutSetting {
    pub value: String,
}

// ── Helpers ────────────────────────────────────────────────────────────────────

fn default_diameter() -> f32 {
    1.75
}
