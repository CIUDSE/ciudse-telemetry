use crate::telemetry::types::DomainObjectIdentifier;
use crate::{actors::common::*, data::*, messages::*};
use actix::prelude::*;
use actix_web::web;
use actix_web_actors::ws;
use log::warn;
use std::{collections::HashSet, iter::FromIterator, time::Instant};

#[derive(Debug)]
pub struct RealtimeTelemetryProvider {
    last_heartbeat: Instant,
    identifier: DomainObjectIdentifier,
    data: web::Data<RealtimeClientConnections>,
}

impl Handler<UpdateTelemetryMessage> for RealtimeTelemetryProvider {
    type Result = ();

    fn handle(
        &mut self,
        msg: UpdateTelemetryMessage,
        ctx: &mut <Self as Actor>::Context,
    ) -> Self::Result {
        ctx.text(
            serde_json::Value::Array(
                msg.datums
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
            .to_string(),
        );
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
                .entry(self.identifier.clone())
                .or_insert_with(HashSet::new)
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
                .entry(self.identifier.clone())
                .or_insert_with(HashSet::new)
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
        ws_handler(self, msg, ctx, |_, _, _| {});
    }
}

impl RealtimeTelemetryProvider {
    pub fn new(
        identifier: DomainObjectIdentifier,
        data: web::Data<RealtimeClientConnections>,
    ) -> Self {
        RealtimeTelemetryProvider {
            last_heartbeat: Instant::now(),
            identifier,
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
