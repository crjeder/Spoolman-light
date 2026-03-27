use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;

/// RGBA color in sRGB color space, compatible with OpenTag3D / OpenPrintTag NFC tag standards.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

// ── MaterialType ───────────────────────────────────────────────────────────────

/// Filament material type, based on the OpenPrintTag material_type_enum specification.
///
/// Serializes / deserializes as the uppercase abbreviation string (e.g. `"PLA"`).
/// Unknown strings round-trip as `Other(string)` so future spec additions and
/// hand-edited data files are preserved without error.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MaterialType {
    /// key 0 — Polylactic Acid
    Pla,
    /// key 1 — Polyethylene Terephthalate Glycol
    Petg,
    /// key 2 — Thermoplastic Polyurethane
    Tpu,
    /// key 3 — Acrylonitrile Butadiene Styrene
    Abs,
    /// key 4 — Acrylonitrile Styrene Acrylate
    Asa,
    /// key 5 — Polycarbonate
    Pc,
    /// key 6 — Polycyclohexylenedimethylene Terephthalate Glycol
    Pctg,
    /// key 7 — Polypropylene
    Pp,
    /// key 8 — Polyamide 6
    Pa6,
    /// key 9 — Polyamide 11
    Pa11,
    /// key 10 — Polyamide 12
    Pa12,
    /// key 11 — Polyamide 66
    Pa66,
    /// key 12 — Copolyester
    Cpe,
    /// key 13 — Thermoplastic Elastomer
    Tpe,
    /// key 14 — High Impact Polystyrene
    Hips,
    /// key 15 — Polyhydroxyalkanoate
    Pha,
    /// key 16 — Polyethylene Terephthalate
    Pet,
    /// key 17 — Polyetherimide
    Pei,
    /// key 18 — Polybutylene Terephthalate
    Pbt,
    /// key 19 — Polyvinyl Butyral
    Pvb,
    /// key 20 — Polyvinyl Alcohol
    Pva,
    /// key 21 — Polyetherketoneketone
    Pekk,
    /// key 22 — Polyether Ether Ketone
    Peek,
    /// key 23 — Butenediol Vinyl Alcohol Copolymer
    Bvoh,
    /// key 24 — Thermoplastic Copolyester
    Tpc,
    /// key 25 — Polyphenylene Sulfide
    Pps,
    /// key 26 — Polyphenylsulfone
    Ppsu,
    /// key 27 — Polyvinyl Chloride
    Pvc,
    /// key 28 — Polyether Block Amide
    Peba,
    /// key 29 — Polyvinylidene Fluoride
    Pvdf,
    /// key 30 — Polyphthalamide
    Ppa,
    /// key 31 — Polycaprolactone
    Pcl,
    /// key 32 — Polyethersulfone
    Pes,
    /// key 33 — Polymethyl Methacrylate
    Pmma,
    /// key 34 — Polyoxymethylene
    Pom,
    /// key 35 — Polyphenylene Ether
    Ppe,
    /// key 36 — Polystyrene
    Ps,
    /// key 37 — Polysulfone
    Psu,
    /// key 38 — Thermoplastic Polyimide
    Tpi,
    /// key 39 — Styrene-Butadiene-Styrene
    Sbs,
    /// key 40 — Olefin Block Copolymer
    Obc,
    /// key 41 — Ethylene Vinyl Acetate
    Eva,
    /// Any string not recognised by the current spec version.
    Other(String),
}

impl MaterialType {
    /// Uppercase abbreviation as used in the OpenPrintTag spec and serialized to JSON.
    pub fn abbreviation(&self) -> &str {
        match self {
            Self::Pla   => "PLA",
            Self::Petg  => "PETG",
            Self::Tpu   => "TPU",
            Self::Abs   => "ABS",
            Self::Asa   => "ASA",
            Self::Pc    => "PC",
            Self::Pctg  => "PCTG",
            Self::Pp    => "PP",
            Self::Pa6   => "PA6",
            Self::Pa11  => "PA11",
            Self::Pa12  => "PA12",
            Self::Pa66  => "PA66",
            Self::Cpe   => "CPE",
            Self::Tpe   => "TPE",
            Self::Hips  => "HIPS",
            Self::Pha   => "PHA",
            Self::Pet   => "PET",
            Self::Pei   => "PEI",
            Self::Pbt   => "PBT",
            Self::Pvb   => "PVB",
            Self::Pva   => "PVA",
            Self::Pekk  => "PEKK",
            Self::Peek  => "PEEK",
            Self::Bvoh  => "BVOH",
            Self::Tpc   => "TPC",
            Self::Pps   => "PPS",
            Self::Ppsu  => "PPSU",
            Self::Pvc   => "PVC",
            Self::Peba  => "PEBA",
            Self::Pvdf  => "PVDF",
            Self::Ppa   => "PPA",
            Self::Pcl   => "PCL",
            Self::Pes   => "PES",
            Self::Pmma  => "PMMA",
            Self::Pom   => "POM",
            Self::Ppe   => "PPE",
            Self::Ps    => "PS",
            Self::Psu   => "PSU",
            Self::Tpi   => "TPI",
            Self::Sbs   => "SBS",
            Self::Obc   => "OBC",
            Self::Eva   => "EVA",
            Self::Other(s) => s.as_str(),
        }
    }

    /// Full material name from the OpenPrintTag spec, or `None` for `Other`.
    pub fn full_name(&self) -> Option<&'static str> {
        match self {
            Self::Pla   => Some("Polylactic Acid"),
            Self::Petg  => Some("Polyethylene Terephthalate Glycol"),
            Self::Tpu   => Some("Thermoplastic Polyurethane"),
            Self::Abs   => Some("Acrylonitrile Butadiene Styrene"),
            Self::Asa   => Some("Acrylonitrile Styrene Acrylate"),
            Self::Pc    => Some("Polycarbonate"),
            Self::Pctg  => Some("Polycyclohexylenedimethylene Terephthalate Glycol"),
            Self::Pp    => Some("Polypropylene"),
            Self::Pa6   => Some("Polyamide 6"),
            Self::Pa11  => Some("Polyamide 11"),
            Self::Pa12  => Some("Polyamide 12"),
            Self::Pa66  => Some("Polyamide 66"),
            Self::Cpe   => Some("Copolyester"),
            Self::Tpe   => Some("Thermoplastic Elastomer"),
            Self::Hips  => Some("High Impact Polystyrene"),
            Self::Pha   => Some("Polyhydroxyalkanoate"),
            Self::Pet   => Some("Polyethylene Terephthalate"),
            Self::Pei   => Some("Polyetherimide"),
            Self::Pbt   => Some("Polybutylene Terephthalate"),
            Self::Pvb   => Some("Polyvinyl Butyral"),
            Self::Pva   => Some("Polyvinyl Alcohol"),
            Self::Pekk  => Some("Polyetherketoneketone"),
            Self::Peek  => Some("Polyether Ether Ketone"),
            Self::Bvoh  => Some("Butenediol Vinyl Alcohol Copolymer"),
            Self::Tpc   => Some("Thermoplastic Copolyester"),
            Self::Pps   => Some("Polyphenylene Sulfide"),
            Self::Ppsu  => Some("Polyphenylsulfone"),
            Self::Pvc   => Some("Polyvinyl Chloride"),
            Self::Peba  => Some("Polyether Block Amide"),
            Self::Pvdf  => Some("Polyvinylidene Fluoride"),
            Self::Ppa   => Some("Polyphthalamide"),
            Self::Pcl   => Some("Polycaprolactone"),
            Self::Pes   => Some("Polyethersulfone"),
            Self::Pmma  => Some("Polymethyl Methacrylate"),
            Self::Pom   => Some("Polyoxymethylene"),
            Self::Ppe   => Some("Polyphenylene Ether"),
            Self::Ps    => Some("Polystyrene"),
            Self::Psu   => Some("Polysulfone"),
            Self::Tpi   => Some("Thermoplastic Polyimide"),
            Self::Sbs   => Some("Styrene-Butadiene-Styrene"),
            Self::Obc   => Some("Olefin Block Copolymer"),
            Self::Eva   => Some("Ethylene Vinyl Acetate"),
            Self::Other(_) => None,
        }
    }

    /// Integer key from the OpenPrintTag spec (for NFC encoding), or `None` for `Other`.
    pub fn key(&self) -> Option<u8> {
        match self {
            Self::Pla   => Some(0),
            Self::Petg  => Some(1),
            Self::Tpu   => Some(2),
            Self::Abs   => Some(3),
            Self::Asa   => Some(4),
            Self::Pc    => Some(5),
            Self::Pctg  => Some(6),
            Self::Pp    => Some(7),
            Self::Pa6   => Some(8),
            Self::Pa11  => Some(9),
            Self::Pa12  => Some(10),
            Self::Pa66  => Some(11),
            Self::Cpe   => Some(12),
            Self::Tpe   => Some(13),
            Self::Hips  => Some(14),
            Self::Pha   => Some(15),
            Self::Pet   => Some(16),
            Self::Pei   => Some(17),
            Self::Pbt   => Some(18),
            Self::Pvb   => Some(19),
            Self::Pva   => Some(20),
            Self::Pekk  => Some(21),
            Self::Peek  => Some(22),
            Self::Bvoh  => Some(23),
            Self::Tpc   => Some(24),
            Self::Pps   => Some(25),
            Self::Ppsu  => Some(26),
            Self::Pvc   => Some(27),
            Self::Peba  => Some(28),
            Self::Pvdf  => Some(29),
            Self::Ppa   => Some(30),
            Self::Pcl   => Some(31),
            Self::Pes   => Some(32),
            Self::Pmma  => Some(33),
            Self::Pom   => Some(34),
            Self::Ppe   => Some(35),
            Self::Ps    => Some(36),
            Self::Psu   => Some(37),
            Self::Tpi   => Some(38),
            Self::Sbs   => Some(39),
            Self::Obc   => Some(40),
            Self::Eva   => Some(41),
            Self::Other(_) => None,
        }
    }

    /// All 42 known material types in spec order.
    pub fn all_known() -> Vec<MaterialType> {
        vec![
            MaterialType::Pla,
            MaterialType::Petg,
            MaterialType::Tpu,
            MaterialType::Abs,
            MaterialType::Asa,
            MaterialType::Pc,
            MaterialType::Pctg,
            MaterialType::Pp,
            MaterialType::Pa6,
            MaterialType::Pa11,
            MaterialType::Pa12,
            MaterialType::Pa66,
            MaterialType::Cpe,
            MaterialType::Tpe,
            MaterialType::Hips,
            MaterialType::Pha,
            MaterialType::Pet,
            MaterialType::Pei,
            MaterialType::Pbt,
            MaterialType::Pvb,
            MaterialType::Pva,
            MaterialType::Pekk,
            MaterialType::Peek,
            MaterialType::Bvoh,
            MaterialType::Tpc,
            MaterialType::Pps,
            MaterialType::Ppsu,
            MaterialType::Pvc,
            MaterialType::Peba,
            MaterialType::Pvdf,
            MaterialType::Ppa,
            MaterialType::Pcl,
            MaterialType::Pes,
            MaterialType::Pmma,
            MaterialType::Pom,
            MaterialType::Ppe,
            MaterialType::Ps,
            MaterialType::Psu,
            MaterialType::Tpi,
            MaterialType::Sbs,
            MaterialType::Obc,
            MaterialType::Eva,
        ]
    }

    /// Parse a material abbreviation string into a `MaterialType`.
    /// Unknown strings become `Other(s)`.
    pub fn from_abbreviation(s: &str) -> Self {
        match s {
            "PLA"  => Self::Pla,
            "PETG" => Self::Petg,
            "TPU"  => Self::Tpu,
            "ABS"  => Self::Abs,
            "ASA"  => Self::Asa,
            "PC"   => Self::Pc,
            "PCTG" => Self::Pctg,
            "PP"   => Self::Pp,
            "PA6"  => Self::Pa6,
            "PA11" => Self::Pa11,
            "PA12" => Self::Pa12,
            "PA66" => Self::Pa66,
            "CPE"  => Self::Cpe,
            "TPE"  => Self::Tpe,
            "HIPS" => Self::Hips,
            "PHA"  => Self::Pha,
            "PET"  => Self::Pet,
            "PEI"  => Self::Pei,
            "PBT"  => Self::Pbt,
            "PVB"  => Self::Pvb,
            "PVA"  => Self::Pva,
            "PEKK" => Self::Pekk,
            "PEEK" => Self::Peek,
            "BVOH" => Self::Bvoh,
            "TPC"  => Self::Tpc,
            "PPS"  => Self::Pps,
            "PPSU" => Self::Ppsu,
            "PVC"  => Self::Pvc,
            "PEBA" => Self::Peba,
            "PVDF" => Self::Pvdf,
            "PPA"  => Self::Ppa,
            "PCL"  => Self::Pcl,
            "PES"  => Self::Pes,
            "PMMA" => Self::Pmma,
            "POM"  => Self::Pom,
            "PPE"  => Self::Ppe,
            "PS"   => Self::Ps,
            "PSU"  => Self::Psu,
            "TPI"  => Self::Tpi,
            "SBS"  => Self::Sbs,
            "OBC"  => Self::Obc,
            "EVA"  => Self::Eva,
            other  => Self::Other(other.to_string()),
        }
    }
}

impl Serialize for MaterialType {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.abbreviation())
    }
}

impl<'de> Deserialize<'de> for MaterialType {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        Ok(Self::from_abbreviation(&s))
    }
}

// ── Filament ───────────────────────────────────────────────────────────────────

/// A filament material specification. No color fields — colors belong to Spool.
/// Fields align with OpenTag3D / OpenPrintTag NFC tag core and extended spec.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Filament {
    pub id: u32,
    pub manufacturer: Option<String>,
    /// Material type per the OpenPrintTag `material_type_enum` spec (e.g. `PLA`, `PETG`).
    pub material: Option<MaterialType>,
    pub material_modifier: Option<String>,
    /// Filament diameter in mm. Default 1.75.
    pub diameter: f32,
    /// Legacy field — present in schema v1 JSON, read during migration only, never written.
    #[serde(default, skip_serializing)]
    pub net_weight: Option<f32>,
    /// Density in g/cm³.
    pub density: f32,
    /// Nominal print temperature in °C.
    pub print_temp: Option<i32>,
    /// Nominal bed temperature in °C.
    pub bed_temp: Option<i32>,
    /// Empty spool weight in grams (informational, not used for tracking).
    pub spool_weight: Option<f32>,
    pub min_print_temp: Option<i32>,
    pub max_print_temp: Option<i32>,
    pub min_bed_temp: Option<i32>,
    pub max_bed_temp: Option<i32>,
    pub registered: DateTime<Utc>,
    pub comment: Option<String>,
}

/// A physical spool of filament. Carries color information and weight tracking.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Spool {
    pub id: u32,
    pub filament_id: u32,
    pub location_id: Option<u32>,
    /// Colors of this spool (1–4 RGBA values). Physically attached to this spool instance.
    pub colors: Vec<Rgba>,
    /// Human-readable color name (e.g. "Galaxy Black").
    pub color_name: Option<String>,
    /// Total weight at creation time (spool + filament), in grams (scale reading).
    pub initial_weight: f32,
    /// Latest scale reading (spool + filament), in grams.
    pub current_weight: f32,
    /// Net weight of filament only (no spool tare), in grams. Specific to this spool purchase.
    pub net_weight: Option<f32>,
    pub registered: DateTime<Utc>,
    pub first_used: Option<DateTime<Utc>>,
    pub last_used: Option<DateTime<Utc>>,
    pub comment: Option<String>,
    pub archived: bool,
}

/// A storage location for spools.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Location {
    pub id: u32,
    /// Non-empty display name.
    pub name: String,
}

/// DataStore schema version, stored in the JSON file meta block.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StoreMeta {
    pub schema_version: u32,
}

/// Top-level JSON storage format.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct DataStore {
    pub meta: StoreMeta,
    pub filaments: Vec<Filament>,
    pub spools: Vec<Spool>,
    pub locations: Vec<Location>,
    /// Key-value settings (e.g. "currency_symbol" → "€").
    pub settings: HashMap<String, String>,
}

impl Default for StoreMeta {
    fn default() -> Self {
        Self { schema_version: 2 }
    }
}

impl Filament {
    /// Derived display name: "{manufacturer} {material} {material_modifier}" with absent fields omitted.
    pub fn display_name(&self) -> String {
        [
            self.manufacturer.as_deref(),
            self.material.as_ref().map(|m| m.abbreviation()),
            self.material_modifier.as_deref(),
        ]
        .iter()
        .filter_map(|s| *s)
        .collect::<Vec<_>>()
        .join(" ")
    }
}
