use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusCheck {
    status: Status,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "error")]
    Error,
}

impl Default for Status {
    fn default() -> Self {
        Status::Success
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Exception {
    TokenException,
    UserException,
    OrderException,
    InputException,
    NetworkException,
    DataException,
    GeneralException,
}

#[test]
fn test_status_success_json() -> serde_json::Result<()> {
    let raw_data = r#"{"status":"success"}"#;
    let deserialized: StatusCheck = serde_json::from_str(&raw_data)?;
    assert_eq!(
        deserialized,
        StatusCheck {
            status: Status::Success
        }
    );
    Ok(())
}

#[test]
fn test_status_error_json() -> serde_json::Result<()> {
    let raw_data = r#"{"status":"error"}"#;
    let deserialized: StatusCheck = serde_json::from_str(&raw_data)?;
    assert_eq!(
        deserialized,
        StatusCheck {
            status: Status::Error
        }
    );
    Ok(())
}
