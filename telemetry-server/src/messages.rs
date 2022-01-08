use actix::prelude::*;
use crate::data::TelemetryDatum;
use crate::telemetry::types::DomainObjectIdentifier;

#[derive(Message, Debug, Clone)]
#[rtype("()")]
pub struct PushDBMsg {
    pub identifier: DomainObjectIdentifier,
    pub datums: Vec<TelemetryDatum>
}

#[derive(Message, Debug, Clone)]
#[rtype(result = "Result<Vec<TelemetryDatum>, ()>")]
pub struct QueryDBMsg {
    pub identifier: DomainObjectIdentifier,
    pub start: u64,
    pub end: u64
}

#[derive(Message, Debug, Clone)]
#[rtype("()")]
pub struct UpdateTelemetryMessage {
    pub datums: Vec<TelemetryDatum>,
}

impl UpdateTelemetryMessage {
    pub fn from(datums: Vec<TelemetryDatum>) -> UpdateTelemetryMessage {
        UpdateTelemetryMessage { datums }
    }
}