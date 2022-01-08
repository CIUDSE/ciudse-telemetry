use serde::{Serialize, Deserialize};

pub struct TelemetryType {
    pub key: String,
    pub name: String,
    pub description: String
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DomainObjectIdentifier {
    pub namespace: String,
    pub key: String
}

pub struct TelemetryValueHints {
    pub range: bool,
    pub domain: bool
}

pub struct TelemetryValueObject {
    pub key: String,
    pub name: String,
    pub format: String,
    pub min: f32,
    pub max: f32,
    pub hints: TelemetryValueHints
}

pub struct TelemetryObject {
    pub identifier: DomainObjectIdentifier,
    pub name: String,
    pub m_type: TelemetryType,
    pub telemetry: Vec<TelemetryValueObject>
}