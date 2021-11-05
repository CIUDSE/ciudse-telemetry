use std::{
    time::{Instant},
    collections::HashSet
};
use actix::prelude::*;
use actix_web::web;
use actix_web_actors::ws;
use log::warn;
use crate::{actors::common::*, data::*, messages::*};

#[derive(Debug)]
pub struct RealtimeTelemetryProvider {
    last_heartbeat: Instant,
    full_key: String,
    data: web::Data<RealtimeClientConnections>,
}

impl Handler<UpdateTelemetryMessage> for RealtimeTelemetryProvider {
    type Result = ();

    fn handle(&mut self, msg: UpdateTelemetryMessage, ctx: &mut <Self as Actor>::Context) -> Self::Result {
        ctx.text(msg.json_data.to_string());
    }
}

impl Actor for RealtimeTelemetryProvider {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // Heartbeat
        heartbeat(ctx);
        
        // Register client in global state
        if let Ok(mut sockets) = self.data.sockets.lock() {
            let addr = ctx.address();
            sockets
                .entry(self.full_key.clone())
                .or_insert(HashSet::new())
                .insert(addr);
        } else {
            ctx.close(Some(ws::CloseReason {
                code: ws::CloseCode::Error,
                description: Some("Internal server error. Couldn't register client.".to_string()),
            }));
        }
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        // Unregister client from global state
        if let Ok(mut sockets) = self.data.sockets.lock() {
            let addr = ctx.address();
            sockets
                .entry(self.full_key.clone())
                .or_insert(HashSet::new())
                .remove(&addr);
        } else {
            warn!("Couldn't aquire lock to remove client from socket list! This may cause an error if sending data to stopped actor address");
            // ? What should we do here? Is Actix web smart enough to ignore messages to stopped actor?
            // ? Maybe resume and try to aquire lock later?
            // For now just stop
            // TODO: Figure out solution
        }
        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for RealtimeTelemetryProvider {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        ws_handler(self, msg, ctx, |_, _, _|{});
    }
}

impl RealtimeTelemetryProvider {
    pub fn new(full_key: String, data: web::Data<RealtimeClientConnections>) -> Self {
        RealtimeTelemetryProvider {
            last_heartbeat: Instant::now(),
            full_key,
            data,
        }
    }
}

impl Heartbeat for RealtimeTelemetryProvider {
    fn get_heartbeat(&self) -> Instant {
        self.last_heartbeat
    }
    fn refresh_heartbeat(&mut self) {
        self.last_heartbeat = Instant::now();
    }
}