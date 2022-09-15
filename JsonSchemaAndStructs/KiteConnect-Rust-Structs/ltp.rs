// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

extern crate serde_derive;

#[derive(Serialize, Deserialize)]
pub struct Ltp {
    #[serde(rename = "$ref")]
    ltp_ref: String,

    #[serde(rename = "$schema")]
    schema: String,

    #[serde(rename = "definitions")]
    definitions: Definitions,
}

#[derive(Serialize, Deserialize)]
pub struct Definitions {
    #[serde(rename = "Data")]
    data: Data,

    #[serde(rename = "Ltp")]
    ltp: LtpClass,

    #[serde(rename = "NseInfy")]
    nse_infy: NseInfyClass,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "additionalProperties")]
    additional_properties: bool,

    #[serde(rename = "properties")]
    properties: DataProperties,

    #[serde(rename = "required")]
    required: Vec<String>,

    #[serde(rename = "title")]
    title: String,

    #[serde(rename = "type")]
    data_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct DataProperties {
    #[serde(rename = "NSE:INFY")]
    nse_infy: NseInfy,
}

#[derive(Serialize, Deserialize)]
pub struct NseInfy {
    #[serde(rename = "$ref")]
    nse_infy_ref: String,
}

#[derive(Serialize, Deserialize)]
pub struct LtpClass {
    #[serde(rename = "additionalProperties")]
    additional_properties: bool,

    #[serde(rename = "properties")]
    properties: LtpProperties,

    #[serde(rename = "required")]
    required: Vec<String>,

    #[serde(rename = "title")]
    title: String,

    #[serde(rename = "type")]
    ltp_class_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct LtpProperties {
    #[serde(rename = "data")]
    data: NseInfy,

    #[serde(rename = "status")]
    status: Status,
}

#[derive(Serialize, Deserialize)]
pub struct Status {
    #[serde(rename = "type")]
    status_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct NseInfyClass {
    #[serde(rename = "additionalProperties")]
    additional_properties: bool,

    #[serde(rename = "properties")]
    properties: NseInfyProperties,

    #[serde(rename = "required")]
    required: Vec<String>,

    #[serde(rename = "title")]
    title: String,

    #[serde(rename = "type")]
    nse_infy_class_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct NseInfyProperties {
    #[serde(rename = "instrument_token")]
    instrument_token: Status,

    #[serde(rename = "last_price")]
    last_price: Status,
}
