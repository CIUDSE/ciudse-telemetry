use crate::actors::DBActor;
use crate::actors::RealtimeTelemetryProvider;
use crate::telemetry::types::DomainObjectIdentifier;
use actix::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
};

#[derive(Debug)]
pub struct DBAddr {
    pub addr: Addr<DBActor>,
}

impl DBAddr {
    pub fn from(addr: Addr<DBActor>) -> DBAddr {
        DBAddr { addr }
    }
}

#[derive(Debug)]
pub struct RealtimeClientConnections {
    pub sockets: Mutex<HashMap<DomainObjectIdentifier, HashSet<Addr<RealtimeTelemetryProvider>>>>,
}

impl RealtimeClientConnections {
    pub fn new() -> Self {
        Self {
            sockets: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for RealtimeClientConnections {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TelemetryDatum {
    pub timestamp: u64,
    pub values: HashMap<String, f64>,
}
