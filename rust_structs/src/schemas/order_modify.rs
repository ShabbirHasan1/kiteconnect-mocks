use super::common::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderModify {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<OrderModifyData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<Exception>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderModifyData {
    pub order_id: String,
}

#[test]
fn test_order_modify_json() -> serde_json::Result<()> {
    let jsonfile = crate::utils::read_user_from_file("../order_modify.json").unwrap();
    let deserialized: OrderModify = serde_json::from_reader(jsonfile)?;
    // println!("{:#?}", &deserialized);
    assert_eq!(
        deserialized,
        OrderModify {
            status: Status::Success,
            data: Some(OrderModifyData {
                order_id: "151220000000000".to_owned()
            }),
            ..OrderModify::default()
        }
    );
    // let serialized = serde_json::to_string(&deserialized).unwrap();
    // println!("{:#?}", &serialized);
    // assert_eq!(raw_data, serialized);
    Ok(())
}

#[test]
fn test_order_modify_error() -> serde_json::Result<()> {
    let raw_data =
        r#"{"status":"error","message":"Error message","error_type":"GeneralException"}"#;
    let deserialized: OrderModify = serde_json::from_str(&raw_data)?;
    // println!("{:#?}", &deserialized);
    assert_eq!(
        deserialized,
        OrderModify {
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
