use super::common::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MfOrderResponse {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<MfOrderResponseData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<Exception>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MfOrderResponseData {
    pub order_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::BorrowMut;
    #[test]
    fn test_mf_order_response_json() -> std::result::Result<(), simd_json::Error> {
        let jsonfile = crate::utils::read_json_from_file("../mf_order_response.json").unwrap();
        let deserialized: MfOrderResponse = simd_json::from_reader(jsonfile)?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            MfOrderResponse {
                status: Status::Success,
                data: Some(MfOrderResponseData {
                    order_id: "3bb085d1-5038-450e-a807-6543fef6c9ae".into()
                }),
                ..MfOrderResponse::default()
            }
        );
        // let serialized = serde_json::to_string(&deserialized).unwrap();
        // println!("{:#?}", &serialized);
        // assert_eq!(raw_data, serialized);
        Ok(())
    }

    #[test]
    fn test_mf_order_response_error() -> std::result::Result<(), simd_json::Error> {
        let mut raw_data =
            r#"{"status":"error","message":"Error message","error_type":"GeneralException"}"#.to_owned();
        let deserialized: MfOrderResponse = simd_json::from_str(raw_data.borrow_mut())?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            MfOrderResponse {
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
