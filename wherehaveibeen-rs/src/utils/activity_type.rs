use std::collections::HashMap;

use super::environment::get_activity_type_conversions;

pub fn sanitize_activity_type(activity_type: &str) -> String {
    let conversions_json = get_activity_type_conversions();
    let conversions: HashMap<String, String> =
        serde_json::from_str(&conversions_json).expect("Invalid JSON format in CONVERSIONS_JSON");

    match conversions.get(activity_type) {
        Some(conversion) => {
            return conversion.clone();
        }
        None => {
            if activity_type.is_empty() {
                return "other".to_string();
            }
            return activity_type.to_string();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        let result = sanitize_activity_type("StandUpPaddling");
        assert_eq!(result, "Stand Up Paddling".to_string());
    }
}
