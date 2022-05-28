use super::common::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggerRange {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, TriggerRangeData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<Exception>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggerRangeData {
    pub instrument_token: u64,
    pub lower: f64,
    pub upper: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trigger_range_no_instruments() -> serde_json::Result<()> {
        let raw_data = r#"{"status":"success","data":{}}"#;
        let deserialized: TriggerRange = serde_json::from_str(&raw_data)?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            TriggerRange {
                status: Status::Success,
                data: Some(HashMap::new()),
                ..TriggerRange::default()
            }
        );
        // let serialized = serde_json::to_string(&deserialized).unwrap();
        // println!("{:#?}", &serialized);
        // assert_eq!(raw_data, serialized);
        Ok(())
    }

    #[test]
    fn test_trigger_range_json() -> serde_json::Result<()> {
        let jsonfile = crate::utils::read_json_from_file("../trigger_range.json").unwrap();
        let deserialized: TriggerRange = serde_json::from_reader(jsonfile)?;
        // println!("{:#?}", &deserialized);
        let mut data: HashMap<String, TriggerRangeData> = HashMap::new();
        data.extend([
            (
                "NSE:INFY".to_owned(),
                TriggerRangeData {
                    instrument_token: 0,
                    lower: 1075.599,
                    upper: 1138.2,
                },
            ),
            (
                "NSE:RELIANCE".to_owned(),
                TriggerRangeData {
                    instrument_token: 0,
                    lower: 870.57475,
                    upper: 902.15,
                },
            ),
        ]);
        assert_eq!(
            deserialized,
            TriggerRange {
                status: Status::Success,
                data: Some(data),
                ..TriggerRange::default()
            }
        );
        // let serialized = serde_json::to_string(&deserialized).unwrap();
        // println!("{:#?}", &serialized);
        // assert_eq!(raw_data, serialized);
        Ok(())
    }

    #[test]
    fn test_trigger_range_error() -> serde_json::Result<()> {
        let raw_data =
            r#"{"status":"error","message":"Error message","error_type":"GeneralException"}"#;
        let deserialized: TriggerRange = serde_json::from_str(&raw_data)?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            TriggerRange {
                status: Status::Error,
                data: None,
                message: Some("Error message".to_owned()),
                error_type: Some(Exception::GeneralException),
            }
        );
        // let serialized = serde_json::to_string(&deserialized).unwrap();
        // println!("{:#?}", &serialized);
        // assert_eq!(raw_data, serialized);
        Ok(())
    }
}
