use actix::prelude::*;
use crate::data::TelemetryDatum;

#[derive(Message, Debug, Clone)]
#[rtype("()")]
pub struct PushDBMsg {
    pub full_key: String,
    pub value: f32,
    pub timestamp: u64
}

#[derive(Message, Debug, Clone)]
#[rtype(result = "Result<Vec<TelemetryDatum>, ()>")]
pub struct QueryDBMsg {
    pub full_key: String,
    pub start: u64,
    pub end: u64
}

#[derive(Message, Debug, Clone)]
#[rtype("()")]
pub struct UpdateTelemetryMessage {
    pub json_data: serde_json::Value,
}

impl UpdateTelemetryMessage {
    pub fn from(data: serde_json::Value) -> UpdateTelemetryMessage {
        UpdateTelemetryMessage { json_data: data }
    }
}