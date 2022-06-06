use super::common::*;
use serde::{Deserialize, Serialize};


#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Logout {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<Exception>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::BorrowMut;
    #[test]
    fn test_logout_json() -> std::result::Result<(), simd_json::Error> {
        let mut raw_data = r#"{"status":"success","data":true}"#.to_owned();
        let deserialized: Logout = simd_json::from_str(raw_data.borrow_mut())?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            Logout {
                status: Status::Success,
                data: Some(true),
                ..Logout::default()
            }
        );
        // let serialized = serde_json::to_string(&deserialized).unwrap();
        // println!("{:#?}", &serialized);
        // assert_eq!(raw_data, serialized);
        Ok(())
    }

    #[test]
    fn test_logout_error() -> std::result::Result<(), simd_json::Error> {
        let mut raw_data =
            r#"{"status":"error","message":"Error message","error_type":"GeneralException"}"#.to_owned();
        let deserialized: Logout = simd_json::from_str(raw_data.borrow_mut())?;
        // println!("{:#?}", &deserialized);
        assert_eq!(
            deserialized,
            Logout {
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
