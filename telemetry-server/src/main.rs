use actix::prelude::*;
use actix_cors::Cors;
use actix_files as fs;
use actix_web::{get, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use serde::Deserialize;
use telemetry_server::actors::*;
use telemetry_server::data::*;
use telemetry_server::messages::*;
use telemetry_server::telemetry::types::*;
use telemetry_server::telemetry::utils::*;
use telemetry_server::config;

#[derive(Deserialize)]
struct HistoricalTelemetryRequestQueryInfo {
    start: u64,
    end: u64,
}

#[get("/historical")]
async fn historical_index(
    query_info: web::Query<HistoricalTelemetryRequestQueryInfo>,
    web::Query(identifier): web::Query<DomainObjectIdentifier>,
    db_data: web::Data<DBAddr>,
) -> Result<HttpResponse, Error> {
    println!("DB?");
    let raw_data = db_data
        .addr
        .send(QueryDBMsg {
            identifier,
            start: query_info.start,
            end: query_info.end,
        })
        .await;
    if raw_data.is_err() {
        return Ok(HttpResponse::Ok().body("[]"));
    }
    let raw_data = raw_data.unwrap();
    if raw_data.is_err() {
        return Ok(HttpResponse::Ok().body("[]"));
    }
    let raw_data = raw_data.unwrap();
    let datums = raw_data;
    Ok(HttpResponse::Ok().body(datums_to_json(datums).to_string()))
}

#[get("/realtime")]
async fn realtime_index(
    r: HttpRequest,
    stream: web::Payload,
    web::Query(identifier): web::Query<DomainObjectIdentifier>,
    client_data: web::Data<RealtimeClientConnections>,
) -> Result<HttpResponse, Error> {
    ws::start(
        RealtimeTelemetryProvider::new(identifier, client_data),
        &r,
        stream,
    )
}

#[get("/injest")]
async fn injest_index(
    r: HttpRequest,
    stream: web::Payload,
    web::Query(identifier): web::Query<DomainObjectIdentifier>,
    client_data: web::Data<RealtimeClientConnections>,
    db_data: web::Data<DBAddr>,
) -> Result<HttpResponse, Error> {
    ws::start(
        InjestSocket::new(identifier, client_data, db_data),
        &r,
        stream,
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug,actix_server=debug,actix_web=debug");
    //std::env::set_var("RUST_LOG", "info,actix_server=info,actix_web=info");

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
    .bind(format!("0.0.0.0:{}", config::BIND_PORT))?
    .run()
    .await
}
