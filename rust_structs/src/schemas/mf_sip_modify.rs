use super::common::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MfSipModifyOrder {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<MfSipModifyOrderData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<Exception>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MfSipModifyOrderData {
    pub sip_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mf_sip_modify_order_json() -> serde_json::Result<()> {
        let jsonfile = crate::utils::read_json_from_file("../mf_sip_modify.json").unwrap();
        let deserialized: MfSipModifyOrder = serde_json::from_reader(jsonfile)?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            MfSipModifyOrder {
                status: Status::Success,
                data: Some(MfSipModifyOrderData {
                    sip_id: "986124545877922".into()
                }),
                ..MfSipModifyOrder::default()
            }
        );
        // let serialized = serde_json::to_string(&deserialized).unwrap();
        // println!("{:#?}", &serialized);
        // assert_eq!(raw_data, serialized);
        Ok(())
    }

    #[test]
    fn test_mf_sip_modify_order_error() -> serde_json::Result<()> {
        let raw_data =
            r#"{"status":"error","message":"Error message","error_type":"GeneralException"}"#;
        let deserialized: MfSipModifyOrder = serde_json::from_str(raw_data)?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            MfSipModifyOrder {
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
