use std::{
    error::Error,
    time::{Instant, UNIX_EPOCH},
};
use actix::prelude::*;
use actix_web::web;
use actix_web_actors::ws;
use log::{debug, warn};
use serde_json::json;
use crate::{actors::common::*, data::*, messages::*};

#[derive(Debug)]
pub struct InjestSocket {
    last_heartbeat: Instant,
    full_key: String,
    client_data: web::Data<RealtimeClientConnections>,
    db_data: web::Data<DBAddr>
}

impl InjestSocket {
    fn injest_data(&self, msg: String) -> Result<(), Box<dyn Error>> {
        let value = msg.parse::<f32>()?;

        // ? Will we ever get that far ?
        let timestamp = UNIX_EPOCH.elapsed().unwrap().as_millis() as u64;

        self.db_data.addr.do_send(PushDBMsg {
            full_key: self.full_key.clone(),
            value,
            timestamp
        });

        // For some reason using "?" doesn't work here
        if let Ok(client_sockets) = self.client_data.sockets.lock() {
            if let Some(key_client_sockets) = client_sockets.get(&self.full_key) {
                let message = UpdateTelemetryMessage::from(json!({
                    "timestamp": timestamp,
                    "value": value
                }));
                for addr in key_client_sockets {
                    debug!("Sending message [{}] to: {:?}", self.full_key, addr);
                    addr.do_send(message.clone());
                }
            }
        } else {
            return Err("Couldn't acquire lock!".into());
        }
        
        Ok(())
    }

    pub fn new(full_key: String, client_data: web::Data<RealtimeClientConnections>, db_data: web::Data<DBAddr>) -> Self {
        InjestSocket {
            last_heartbeat: Instant::now(),
            full_key,
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
            match act.injest_data(text){
                Ok(_) => {},
                Err(e) => { warn!("Error injesting data! {:?}", e); }
            };
        });
    }
}
