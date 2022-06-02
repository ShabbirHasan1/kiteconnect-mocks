use super::common::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HoldingsAuth {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HoldingsAuthData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<Exception>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HoldingsAuthData {
    pub request_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_holdings_auth_json() -> serde_json::Result<()> {
        let jsonfile = crate::utils::read_json_from_file("../holdings_auth.json").unwrap();
        let deserialized: HoldingsAuth = serde_json::from_reader(jsonfile)?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            HoldingsAuth {
                status: Status::Success,
                data: Some(HoldingsAuthData {
                    request_id: "na8QgCeQm05UHG6NL9sAGRzdfSF64UdB".to_owned(),
                },),
                ..HoldingsAuth::default()
            }
        );
        // let serialized = serde_json::to_string(&deserialized).unwrap();
        // println!("{:#?}", &serialized);
        // assert_eq!(raw_data, serialized);
        Ok(())
    }

    #[test]
    fn test_holdings_auth_error() -> serde_json::Result<()> {
        let raw_data =
            r#"{"status":"error","message":"Error message","error_type":"GeneralException"}"#;
        let deserialized: HoldingsAuth = serde_json::from_str(raw_data)?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            HoldingsAuth {
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
