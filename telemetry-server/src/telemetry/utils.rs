use std::iter::FromIterator;
use crate::data::TelemetryDatum;

pub fn datums_to_json(datums: Vec<TelemetryDatum>) -> serde_json::Value {
    serde_json::Value::Array(
        datums
        .iter()
        .map(|datum| {
            let mut map = serde_json::Map::from_iter(
                datum.values.iter().map(|(k, v)| (k.to_owned(), serde_json::json!(v))),
            );
            map.insert(
                "server_timestamp".to_string(),
                serde_json::json!(datum.timestamp),
            );
            serde_json::Value::Object(map)
        })
        .collect(),
    )
}