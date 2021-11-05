use actix::prelude::*;
use std::{collections::{HashSet, HashMap}, sync::Mutex};
use crate::actors::RealtimeTelemetryProvider;
use crate::actors::DBActor;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct DBAddr {
    pub addr: Addr<DBActor>
}

impl DBAddr {
    pub fn from(addr: Addr<DBActor>) -> DBAddr{
        DBAddr { addr }
    }
}

#[derive(Debug)]
pub struct RealtimeClientConnections {
    pub sockets: Mutex<HashMap<String, HashSet<Addr<RealtimeTelemetryProvider>>>>,
}

impl RealtimeClientConnections {
    pub fn new() -> Self {
        Self {
            sockets: Mutex::new(HashMap::new()),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TelemetryDatum {
    pub timestamp: u64,
    pub value: f64
}
