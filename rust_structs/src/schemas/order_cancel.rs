use super::common::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderCancel {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<OrderCancelData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<Exception>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderCancelData {
    pub order_id: String,
}

#[test]
fn test_order_cancel_json() -> serde_json::Result<()> {
    let jsonfile = crate::utils::read_user_from_file("../order_cancel.json").unwrap();
    let deserialized: OrderCancel = serde_json::from_reader(jsonfile)?;
    // println!("{:#?}", &deserialized);
    assert_eq!(
        deserialized,
        OrderCancel {
            status: Status::Success,
            data: Some(OrderCancelData {
                order_id: "151220000000000".to_owned()
            }),
            ..OrderCancel::default()
        }
    );
    // let serialized = serde_json::to_string(&deserialized).unwrap();
    // println!("{:#?}", &serialized);
    // assert_eq!(raw_data, serialized);
    Ok(())
}

#[test]
fn test_order_cancel_error() -> serde_json::Result<()> {
    let raw_data =
        r#"{"status":"error","message":"Error message","error_type":"GeneralException"}"#;
    let deserialized: OrderCancel = serde_json::from_str(&raw_data)?;
    // println!("{:#?}", &deserialized);
    assert_eq!(
        deserialized,
        OrderCancel {
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
