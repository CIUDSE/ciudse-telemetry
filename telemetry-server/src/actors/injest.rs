use crate::telemetry::types::DomainObjectIdentifier;
use crate::{actors::common::*, data::*, messages::*};
use actix::prelude::*;
use actix_web::web;
use actix_web_actors::ws;
use log::{warn, info};
use std::collections::HashMap;
use std::{
    error::Error,
    time::{Instant, UNIX_EPOCH},
};

#[derive(Debug)]
pub struct InjestSocket {
    last_heartbeat: Instant,
    identifier: DomainObjectIdentifier,
    client_data: web::Data<RealtimeClientConnections>,
    db_data: web::Data<DBAddr>,
}

impl InjestSocket {
    fn injest_data(&self, msg: String) -> Result<(), Box<dyn Error>> {
        let datums: Vec<TelemetryDatum> = msg
            .lines()
            .map(|line| {
                line.split(',')
                    .filter(|&x| !x.trim().is_empty())
                    .filter_map(|x| x.split_once('='))
                    .filter_map(|(k, v)| match v.parse::<f64>() {
                        Ok(val) => Some((String::from(k), val)),
                        Err(_e) => None,
                    })
                    .collect::<HashMap<String, f64>>()
            })
            .filter(|values| !values.is_empty())
            .map(|values| TelemetryDatum {
                timestamp: UNIX_EPOCH.elapsed().unwrap().as_millis() as u64,
                values,
            })
            .collect();

        self.db_data.addr.do_send(PushDBMsg {
            identifier: self.identifier.clone(),
            datums: datums.clone(),
        });

        // For some reason using "?" doesn't work here
        if let Ok(client_sockets) = self.client_data.sockets.lock() {
            if let Some(key_client_sockets) = client_sockets.get(&self.identifier) {
                let message = UpdateTelemetryMessage::from(datums);
                for addr in key_client_sockets {
                    addr.do_send(message.clone());
                }
            }
        } else {
            return Err("Couldn't acquire lock!".into());
        }

        Ok(())
    }

    pub fn new(
        identifier: DomainObjectIdentifier,
        client_data: web::Data<RealtimeClientConnections>,
        db_data: web::Data<DBAddr>,
    ) -> Self {
        InjestSocket {
            last_heartbeat: Instant::now(),
            identifier,
            client_data,
            db_data,
        }
    }
}

impl Heartbeat for InjestSocket {
    fn get_heartbeat(&self) -> Instant {
        self.last_heartbeat
    }
    fn refresh_heartbeat(&mut self) {
        self.last_heartbeat = Instant::now();
    }
}

impl Actor for InjestSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        heartbeat(ctx);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for InjestSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        ws_handler(self, msg, ctx, |text, _ctx, act| {
            match act.injest_data(text) {
                Ok(_) => {}
                Err(e) => {
                    warn!("Error injesting data! {:?}", e);
                }
            };
        });
    }
}
