use super::common::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderResponse {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<OrderResponseData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<Exception>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderResponseData {
    pub order_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_order_response_json() -> serde_json::Result<()> {
        let jsonfile = crate::utils::read_json_from_file("../order_response.json").unwrap();
        let deserialized: OrderResponse = serde_json::from_reader(jsonfile)?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            OrderResponse {
                status: Status::Success,
                data: Some(OrderResponseData {
                    order_id: "151220000000000".to_owned()
                }),
                ..OrderResponse::default()
            }
        );
        // let serialized = serde_json::to_string(&deserialized).unwrap();
        // println!("{:#?}", &serialized);
        // assert_eq!(raw_data, serialized);
        Ok(())
    }

    #[test]
    fn test_order_response_error() -> serde_json::Result<()> {
        let raw_data =
            r#"{"status":"error","message":"Error message","error_type":"GeneralException"}"#;
        let deserialized: OrderResponse = serde_json::from_str(raw_data)?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            OrderResponse {
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
