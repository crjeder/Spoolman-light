use crate::models::{Filament, Location, Rgba, Spool};
use serde::{Deserialize, Serialize};

/// Spool response with derived weight metrics included.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SpoolResponse {
    #[serde(flatten)]
    pub spool: Spool,
    /// used_weight = initial_weight - current_weight (None until initial_weight is set)
    pub used_weight: Option<f32>,
    /// remaining_filament = spool.net_weight - used_weight (= net_weight when initial_weight is unset; None if net_weight unknown)
    pub remaining_filament: Option<f32>,
    /// price_per_kg = spool.price / net_weight_kg; None when price or net_weight absent
    pub price_per_kg: Option<f32>,
    /// The associated filament (embedded for convenience).
    pub filament: Filament,
}

impl SpoolResponse {
    pub fn new(spool: Spool, filament: Filament) -> Self {
        // initial_weight of 0 means "not set yet" (e.g. imported data) — weight math is meaningless until it is.
        let has_initial_weight = spool.initial_weight != 0.0;
        let used_weight = has_initial_weight.then_some(spool.initial_weight - spool.current_weight);
        let remaining_filament = match used_weight {
            Some(uw) => spool.net_weight.map(|nw| nw - uw),
            None => spool.net_weight,
        };
        let price_per_kg = spool
            .price
            .zip(spool.net_weight)
            .filter(|(_, w)| *w > 0.0)
            .map(|(p, w)| p / (w / 1000.0));
        Self {
            spool,
            filament,
            used_weight,
            remaining_filament,
            price_per_kg,
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

/// Server info response for GET /info and GET /api/v1/info.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InfoResponse {
    pub version: String,
    pub debug: bool,
    pub data_directory: String,
    pub automatic_backup: bool,
}
