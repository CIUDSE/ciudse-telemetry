use actix::prelude::*;
use actix_web::{get, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use actix_files as fs;
use actix_cors::Cors;
use log::{debug, info};
use serde::Deserialize;
use telemetry_server::actors::*;
use telemetry_server::data::*;
use telemetry_server::messages::*;

#[derive(Deserialize)]
struct HistoricalTelemetryRequestQueryInfo {
    start: u64,
    end: u64,
}

#[get("/historical/{full_key}")]
async fn historical_index(
    path_info: web::Path<String>,
    query_info: web::Query<HistoricalTelemetryRequestQueryInfo>,
    db_data: web::Data<DBAddr>,
) -> Result<HttpResponse, Error> {
    let full_key = path_info.0;
    println!("DB?");
    let raw_data = db_data.addr.send(QueryDBMsg{
        full_key,
        start: query_info.start,
        end: query_info.end
    }).await;
    debug!("Got DB msg, responding: {:?}", &raw_data);
    if raw_data.is_err() { return Ok(HttpResponse::Ok().body("[]")); }
    let raw_data = raw_data.unwrap();
    if raw_data.is_err() { return Ok(HttpResponse::Ok().body("[]")); }
    let raw_data = raw_data.unwrap();
    let datums = raw_data;
    Ok(HttpResponse::Ok().json(
        datums
    ))
}

#[get("/realtime/{full_key}")]
async fn realtime_index(
    r: HttpRequest,
    stream: web::Payload,
    info: web::Path<String>,
    client_data: web::Data<RealtimeClientConnections>,
) -> Result<HttpResponse, Error> {
    debug!("{:?}", r);
    let full_key = info.0;
    debug!("Domain object requesting telemetry: {}", full_key);
    ws::start(RealtimeTelemetryProvider::new(full_key, client_data), &r, stream)
}

#[get("/injest/{full_key}")]
async fn injest_index(
    r: HttpRequest,
    stream: web::Payload,
    info: web::Path<String>,
    client_data: web::Data<RealtimeClientConnections>,
    db_data: web::Data<DBAddr>,
) -> Result<HttpResponse, Error> {
    let full_key = info.0;
    debug!("New injest socket for: {}", full_key);
    ws::start(InjestSocket::new(full_key, client_data, db_data), &r, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //std::env::set_var("RUST_LOG", "debug,actix_server=debug,actix_web=debug");
    std::env::set_var("RUST_LOG", "info,actix_server=info,actix_web=info");
    
    env_logger::init();

    let realtime_connections = web::Data::new(RealtimeClientConnections::new());

    let db_data = web::Data::new(DBAddr::from(DBActor::new().start()));

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(realtime_connections.clone())
            .app_data(db_data.clone())
            // enable logger
            .wrap(middleware::Logger::default())
            // websocket route
            .service(realtime_index)
            .service(injest_index)
            .service(historical_index)
            .service(fs::Files::new("/", "static").index_file("index.html"))
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
}
