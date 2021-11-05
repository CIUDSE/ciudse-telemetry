use std::time::{Duration, Instant};
use actix::{Actor, ActorContext, AsyncContext};
use actix_web_actors::ws::{self, WebsocketContext};
use log::warn;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);
pub trait Heartbeat {
    fn get_heartbeat(&self) -> Instant;
    fn refresh_heartbeat(&mut self);
}

pub fn heartbeat<A: Actor<Context = WebsocketContext<A>> + Heartbeat>(ctx: &mut WebsocketContext<A>) {
    ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
        if act.get_heartbeat().elapsed() > CLIENT_TIMEOUT {
            // heartbeat timed out
            warn!("Websocket Client heartbeat failed, disconnecting!");
            // stop actor
            ctx.stop();
            // don't try to send a ping
            return;
        }
        ctx.ping(b"");
    });
}

pub fn ws_handler<A,F>(
    act: &mut A,
    msg: Result<ws::Message, ws::ProtocolError>,
    ctx: &mut WebsocketContext<A>,
    text_handler: F)
where
    A: Actor<Context = WebsocketContext<A>> + Heartbeat,
    F: Fn(String, &mut WebsocketContext<A>, &mut A)
{
    match msg {
        Ok(ws::Message::Ping(msg)) => {
            act.refresh_heartbeat();
            ctx.pong(&msg);
        }
        Ok(ws::Message::Pong(_)) => {
            act.refresh_heartbeat();
        }
        Ok(ws::Message::Text(text)) => {
            text_handler(text, ctx, act);
        }
        Ok(ws::Message::Binary(_bin)) => {}
        Ok(ws::Message::Close(reason)) => {
            ctx.close(reason);
            ctx.stop();
        }
        _ => ctx.stop(),
    }
}