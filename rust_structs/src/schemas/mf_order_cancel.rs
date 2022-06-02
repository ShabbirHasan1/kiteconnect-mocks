use super::common::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MfOrderCancel {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<MfOrderCancelData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<Exception>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MfOrderCancelData {
    pub order_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mf_order_cancel_json() -> serde_json::Result<()> {
        let jsonfile = crate::utils::read_json_from_file("../mf_order_cancel.json").unwrap();
        let deserialized: MfOrderCancel = serde_json::from_reader(jsonfile)?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            MfOrderCancel {
                status: Status::Success,
                data: Some(MfOrderCancelData {
                    order_id: "3bb085d1-5038-450e-a807-6543fef6c9ae".into()
                }),
                ..MfOrderCancel::default()
            }
        );
        // let serialized = serde_json::to_string(&deserialized).unwrap();
        // println!("{:#?}", &serialized);
        // assert_eq!(raw_data, serialized);
        Ok(())
    }

    #[test]
    fn test_mf_order_cancel_error() -> serde_json::Result<()> {
        let raw_data =
            r#"{"status":"error","message":"Error message","error_type":"GeneralException"}"#;
        let deserialized: MfOrderCancel = serde_json::from_str(raw_data)?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            MfOrderCancel {
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
