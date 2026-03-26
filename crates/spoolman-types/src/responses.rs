use crate::models::{Filament, Location, Rgba, Spool};
use serde::{Deserialize, Serialize};

/// Spool response with derived weight metrics included.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SpoolResponse {
    #[serde(flatten)]
    pub spool: Spool,
    /// used_weight = initial_weight - current_weight
    pub used_weight: f32,
    /// remaining_filament = filament.net_weight - used_weight (None if net_weight unknown)
    pub remaining_filament: Option<f32>,
    /// remaining_pct = remaining_filament / net_weight * 100 (None if net_weight unknown)
    pub remaining_pct: Option<f32>,
    /// The associated filament (embedded for convenience).
    pub filament: Filament,
}

impl SpoolResponse {
    pub fn new(spool: Spool, filament: Filament) -> Self {
        let used_weight = spool.initial_weight - spool.current_weight;
        let remaining_filament = filament.net_weight.map(|nw| nw - used_weight);
        let remaining_pct = filament
            .net_weight
            .filter(|&nw| nw > 0.0)
            .map(|nw| (nw - used_weight) / nw * 100.0);
        Self {
            spool,
            filament,
            used_weight,
            remaining_filament,
            remaining_pct,
        }
    }
}

/// Filament response (currently identical to Filament, reserved for future extension).
pub type FilamentResponse = Filament;

/// Location response with spool count.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LocationResponse {
    #[serde(flatten)]
    pub location: Location,
    pub spool_count: usize,
}

/// Paginated list wrapper. Total count is returned in the X-Total-Count header by handlers.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ListResponse<T> {
    pub items: Vec<T>,
    pub total: usize,
}

/// A filament entry returned from the SpoolmanDB proxy search.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SpoolmanDbEntry {
    pub manufacturer: Option<String>,
    pub material: Option<String>,
    pub material_modifier: Option<String>,
    pub diameter: Option<f32>,
    pub net_weight: Option<f32>,
    pub density: Option<f32>,
    pub print_temp: Option<i32>,
    pub bed_temp: Option<i32>,
    pub color_hex: Option<String>,
    pub color_name: Option<String>,
    pub colors: Option<Vec<Rgba>>,
}

/// Server info response for GET /info.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InfoResponse {
    pub version: String,
    pub data_file: String,
}
