//! Parser for OrcaSlicer / Bambu Studio filament profile JSON files.

use serde_json::Value;
use spoolman_types::models::MaterialType;

/// Filament form values extracted from a slicer profile. `None` = not provided.
#[derive(Debug, Default, PartialEq)]
pub struct ProfileData {
    pub manufacturer: Option<String>,
    pub material: Option<String>,
    pub modifier: Option<String>,
    pub diameter: Option<f32>,
    pub density: Option<f32>,
    pub print_temp: Option<i32>,
    pub bed_temp: Option<i32>,
}

/// First array element (or the plain string); `""` and `"nil"` count as absent.
fn get_str(v: &Value, key: &str) -> Option<String> {
    let x = v.get(key)?;
    let s = x.as_array().and_then(|a| a.first()).unwrap_or(x).as_str()?.trim();
    (!s.is_empty() && s != "nil").then(|| s.to_string())
}

fn get_num(v: &Value, key: &str) -> Option<f32> {
    get_str(v, key)?.parse().ok()
}

fn bed_key(bed_type: Option<&str>) -> &'static str {
    match bed_type {
        Some("Cool Plate") => "cool_plate_temp",
        Some("Engineering Plate") => "eng_plate_temp",
        Some("Textured PEI Plate") => "textured_plate_temp",
        _ => "hot_plate_temp",
    }
}

/// "Polymaker HT-PLA @User" + vendor "Polymaker" + type "PLA" -> "HT".
fn modifier_from_name(name: &str, vendor: Option<&str>, material: Option<&str>) -> Option<String> {
    let mut s = name.split(" @").next().unwrap_or(name).trim();
    if let Some(v) = vendor.filter(|v| s.len() >= v.len() && s.is_char_boundary(v.len())) {
        if s[..v.len()].eq_ignore_ascii_case(v) {
            s = s[v.len()..].trim();
        }
    }
    if let Some(m) = material.filter(|m| s.len() >= m.len() && s.is_char_boundary(s.len() - m.len())) {
        if s[s.len() - m.len()..].eq_ignore_ascii_case(m) {
            s = &s[..s.len() - m.len()];
        }
    }
    let s = s.trim_matches(|c: char| c == '-' || c.is_whitespace());
    (!s.is_empty()).then(|| s.to_string())
}

/// Parse an Orca filament profile. Errors if the file is not JSON or not a filament profile.
pub fn parse_orca(text: &str) -> Result<ProfileData, String> {
    let v: Value = serde_json::from_str(text).map_err(|e| format!("Not a valid JSON file: {e}"))?;
    if !["filament_type", "filament_vendor", "filament_density"]
        .iter()
        .any(|k| v.get(k).is_some())
    {
        return Err("Not a slicer filament profile".to_string());
    }
    let manufacturer = get_str(&v, "filament_vendor");
    let type_str = get_str(&v, "filament_type");
    let modifier = get_str(&v, "name")
        .and_then(|n| modifier_from_name(&n, manufacturer.as_deref(), type_str.as_deref()));
    let bed_type = get_str(&v, "bed_type");
    Ok(ProfileData {
        manufacturer,
        material: type_str.map(|t| MaterialType::from_abbreviation(&t).abbreviation().to_string()),
        modifier,
        diameter: get_num(&v, "filament_diameter"),
        density: get_num(&v, "filament_density"),
        print_temp: get_num(&v, "nozzle_temperature").map(|t| t.round() as i32),
        bed_temp: get_num(&v, bed_key(bed_type.as_deref())).map(|t| t.round() as i32),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const HT_PLA: &str = r#"{
        "name": "Polymaker HT-PLA @User",
        "cool_plate_temp": ["60"], "hot_plate_temp": ["65"],
        "filament_density": ["1.28"], "filament_diameter": ["1.75"],
        "filament_retraction_length": ["nil"],
        "filament_type": ["PLA"], "filament_vendor": ["Polymaker"],
        "bed_type": ["Cool Plate"], "nozzle_temperature": ["220"]
    }"#;

    #[test]
    fn polymaker_example() {
        let d = parse_orca(HT_PLA).unwrap();
        assert_eq!(d.manufacturer.as_deref(), Some("Polymaker"));
        assert_eq!(d.material.as_deref(), Some("PLA"));
        assert_eq!(d.modifier.as_deref(), Some("HT"));
        assert_eq!(d.diameter, Some(1.75));
        assert_eq!(d.density, Some(1.28));
        assert_eq!(d.print_temp, Some(220));
        assert_eq!(d.bed_temp, Some(60));
    }

    #[test]
    fn nil_and_missing_are_none() {
        let d = parse_orca(r#"{"filament_type": ["PETG"], "filament_density": ["nil"]}"#).unwrap();
        assert_eq!(d.material.as_deref(), Some("PETG"));
        assert_eq!(d.density, None);
        assert_eq!(d.bed_temp, None);
        assert_eq!(d.modifier, None);
    }

    #[test]
    fn unknown_bed_type_falls_back_to_hot_plate() {
        let d = parse_orca(r#"{"filament_type": ["PLA"], "bed_type": ["Weird"], "hot_plate_temp": ["65"]}"#).unwrap();
        assert_eq!(d.bed_temp, Some(65));
    }

    #[test]
    fn plain_name_has_no_modifier() {
        assert_eq!(modifier_from_name("Generic PLA", None, Some("PLA")).as_deref(), Some("Generic"));
        assert_eq!(modifier_from_name("Polymaker PLA @X", Some("Polymaker"), Some("PLA")), None);
    }

    #[test]
    fn rejects_bad_input() {
        assert!(parse_orca("not json").is_err());
        assert!(parse_orca(r#"{"foo": 1}"#).is_err());
    }
}
