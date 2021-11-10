use std::error::Error;
use std::net::TcpStream;
use std::io::prelude::*;
use actix::prelude::*;
use crate::messages::*;
use log::{debug, info, warn};
use crate::data::TelemetryDatum;
use serde_json::Value;
use ms_converter;
use chrono::{NaiveDateTime, NaiveDate};

#[derive(Debug)]
pub struct DBActor {
    stream: Option<TcpStream>,
}

impl Handler<PushDBMsg> for DBActor {
    type Result = ();

    fn handle(
        &mut self,
        msg: PushDBMsg,
        _ctx: &mut <Self as Actor>::Context)
    {
        match self.pushdb(msg) {
            Ok(r) => { debug!("{}", r); },
            Err(e) => { warn!("{:?}", e); }
        };
    }
}

impl Handler<QueryDBMsg> for DBActor {
    type Result = ResponseFuture<Result<Vec<TelemetryDatum>, ()>>;

    fn handle(
        &mut self,
        msg: QueryDBMsg,
        _ctx: &mut <Self as Actor>::Context) -> Self::Result
    {
        debug!("Db msg");
        Box::pin(async move {
            let re = querydb_suppress_errors(msg);
            let re = re.await;
            Ok(re)
        })
    }
}

impl Actor for DBActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        debug!("Database actor started!");
    }
}

async fn querydb_suppress_errors(msg: QueryDBMsg) -> Vec<TelemetryDatum> {
    match querydb(msg).await {
        Ok(r) => r,
        Err(e) => {
            warn!("{:?}", &e);
            vec![]
        },
    }
}

async fn querydb(msg: QueryDBMsg) -> Result<Vec<TelemetryDatum>, Box<dyn Error>> {
    let database_url = "http://127.0.0.1:9000/exec";
    let full_key = msg.full_key;
    let start = msg.start;
    let end = msg.end;
    debug!("Stuff");
    let datum_count_limit = 500;
    let sql_query = format!(
        "SELECT * FROM \"{table}\" WHERE timestamp BETWEEN CAST({left_millis}000 AS TIMESTAMP) AND CAST({right_millis}000 AS TIMESTAMP) LIMIT {datum_count_limit}",
        table = full_key,
        left_millis = start,
        right_millis = end,
        datum_count_limit = datum_count_limit,
    );
    debug!("SQL query");
    let req = actix_web::client::Client::new().get(database_url).query(
        &[
            ("query", sql_query)
        ]
    )?;
    debug!("Request URI: \"{}\"", req.get_uri());
    let mut response = req.send().await?;
    debug!("Response");
    let raw_data = response.body().await?;
    debug!("{:?}", raw_data);
    let data: Value = serde_json::from_slice(&raw_data)?;
    debug!("{:?}", data);
    let dataset = &data["dataset"];
    let mut output: Vec<TelemetryDatum> = Vec::new();
    for row in dataset.as_array().unwrap() {
        let val = row.as_array().unwrap()[0].as_f64().unwrap();
        let timestamp_str = row.as_array().unwrap()[1].as_str().unwrap();
        let n = timestamp_str.len();
        let timestamp_str = &timestamp_str[..n-1];
        let datetime = NaiveDateTime::parse_from_str(
            timestamp_str,
            "%Y-%m-%dT%H:%M:%S%.f");

        if datetime.is_err() {
            warn!("Couldn't convert timestamp {:?}", timestamp_str);
            return Err(Box::new(datetime.err().unwrap()));
        }
        let datetime = datetime.unwrap();
        let timestamp = datetime.timestamp_millis() as u64;
        output.push(TelemetryDatum{
            timestamp,
            value: val,
        });
    }
    Ok(output)
}
impl Default for DBActor {
    fn default() -> Self {
        Self::new()
    }
}
impl DBActor {
    pub fn new() -> DBActor {
        DBActor {
            stream: None,
        }
    }

    fn pushdb(&mut self, msg: PushDBMsg) -> Result<String, Box<dyn Error>> {
        let database_address = "questdb:9009";
        if self.stream.is_none() {
            self.stream = Some(TcpStream::connect(database_address)?);
        }
        let mut stream = self.stream.as_ref().unwrap();
        // Influx line protocol timestamps are in nanoseconds
        let query = format!("{table} value={value} {timestamp}000000\n\n",
            table = msg.full_key,
            value = msg.value,
            timestamp = msg.timestamp, 
        );
        stream.write_all(query.as_bytes())?;
        Ok(query)
    }
}