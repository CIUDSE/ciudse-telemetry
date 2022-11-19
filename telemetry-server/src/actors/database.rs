use crate::data::TelemetryDatum;
use crate::{messages::*, telemetry::types::DomainObjectIdentifier};
use actix::prelude::*;
use chrono::NaiveDateTime;
use itertools::Itertools;
use log::{warn, info};
use reqwest::Url;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::io::prelude::*;
use std::net::TcpStream;
use crate::config;

#[derive(Debug)]
pub struct DBActor {
    stream: Option<TcpStream>,
}

impl Handler<PushDBMsg> for DBActor {
    type Result = ();

    fn handle(&mut self, msg: PushDBMsg, _ctx: &mut <Self as Actor>::Context) {
        match self.pushdb(&msg) {
            Ok(_) => { }
            Err(e) => {
                warn!("DB Push Err:\n{:?}\n{:?}", msg, e);
            }
        };
    }
}

impl Handler<QueryDBMsg> for DBActor {
    type Result = ResponseFuture<Result<Vec<TelemetryDatum>, ()>>;

    fn handle(&mut self, msg: QueryDBMsg, _ctx: &mut <Self as Actor>::Context) -> Self::Result {
        Box::pin(async move {
            let re = querydb_suppress_errors(msg);
            let re = re.await;
            Ok(re)
        })
    }
}

impl Actor for DBActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) { }
}

async fn querydb_suppress_errors(msg: QueryDBMsg) -> Vec<TelemetryDatum> {
    match querydb(msg).await {
        Ok(r) => r,
        Err(e) => {
            warn!("{:?}", &e);
            vec![]
        }
    }
}

fn table_name(identifier: &DomainObjectIdentifier) -> String {
    format!("{}#{}", identifier.namespace, identifier.key)
}

fn format_db_query(msg: QueryDBMsg) -> String {
    format!(
        "SELECT * FROM \"{table}\" WHERE timestamp BETWEEN CAST({left_millis}000 AS TIMESTAMP) AND CAST({right_millis}000 AS TIMESTAMP) LIMIT {datum_count_limit}",
        table = table_name(&msg.identifier),
        left_millis = msg.start,
        right_millis = msg.end,
        datum_count_limit = 500,
    )
}

#[derive(PartialEq)]
enum DbColumnType {
    Double,
    Timestamp,
}

impl DbColumnType {
    fn from_str(str: &str) -> Option<DbColumnType> {
        match str {
            "DOUBLE" => Some(DbColumnType::Double),
            "TIMESTAMP" => Some(DbColumnType::Timestamp),
            _ => None,
        }
    }
}

struct DbColumn {
    col_name: String,
    col_type: DbColumnType,
}

impl DbColumn {
    fn from_value(value: &Value) -> Option<Self> {
        let value = value.as_object()?;
        let col_name = value.get("name")?.as_str()?;
        let col_name = String::from(col_name);
        let col_type_str = value.get("type")?.as_str()?;
        let col_type = DbColumnType::from_str(col_type_str)?;
        Some(DbColumn { col_name, col_type })
    }
}

fn parse_db_row(val: &Value, timestamp_col: usize, columns: &[DbColumn]) -> Option<TelemetryDatum> {
    let row = val.as_array()?;
    let timestamp_str = row.get(timestamp_col)?.as_str()?;
    let values: HashMap<String, f64> = row
        .iter()
        .zip(columns.iter())
        .filter(|(_val, col)| col.col_type == DbColumnType::Double)
        .map(|(val, col)| (col.col_name.clone(), val.as_f64().unwrap()))
        .collect();
    let timestamp = NaiveDateTime::parse_from_str(timestamp_str, "%Y-%m-%dT%H:%M:%S%.fZ")
        .ok()?
        .timestamp_millis() as u64;
    Some(TelemetryDatum { timestamp, values })
}

async fn querydb(msg: QueryDBMsg) -> Result<Vec<TelemetryDatum>, Box<dyn Error>> {
    let sql_query = format_db_query(msg);

    let url = Url::parse_with_params(&format!("https://{}:{}/exec", config::QDB_HOST, config::QDB_REST_EXEC_PORT), &[("query", sql_query)])?;
    
    let resp = reqwest::get(url).await?;

    let bytes = resp.bytes().await?;

    let data: Value = serde_json::from_slice(&bytes)?;

    let columns: Vec<DbColumn> = data["columns"]
        .as_array()
        .unwrap()
        .iter()
        .map(|val| DbColumn::from_value(val).unwrap())
        .collect();

    let dataset = data["dataset"].as_array().unwrap();
    let timestamp_col = columns
        .iter()
        .find_position(|&col| col.col_type == DbColumnType::Timestamp)
        .unwrap()
        .0;

    let datums = dataset
        .iter()
        .map(|row| parse_db_row(row, timestamp_col, &columns).unwrap())
        .collect();

    Ok(datums)
}
impl Default for DBActor {
    fn default() -> Self {
        Self::new()
    }
}
impl DBActor {
    pub fn new() -> DBActor {
        DBActor { stream: None }
    }

    fn pushdb(&mut self, msg: &PushDBMsg) -> Result<(), Box<dyn Error>> {
        let database_address = format!("{}:{}", config::QDB_HOST, config::QDB_LINE_INJEST_PORT);
        info!("logging to db addr `{}`", database_address);
        if self.stream.is_none() {
            self.stream = Some(TcpStream::connect(database_address)?);
        }
        let mut stream = self.stream.as_ref().unwrap();
        info!("stream connected");
        // Influx line protocol timestamps are in nanoseconds
        let table = table_name(&msg.identifier);
        let mut query: String = msg
            .datums
            .iter()
            .map(|datum| {
                let values: String = datum
                    .values
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .intersperse(String::from(","))
                    .collect();
                let nanos_str = format!("{}000000", datum.timestamp);
                format!("{} {} {}\n", table, values, nanos_str)
            })
            .collect();
        query.push('\n');
        let bytes = query.as_bytes();
        stream.write_all(bytes)?;
        Ok(())
    }
}
