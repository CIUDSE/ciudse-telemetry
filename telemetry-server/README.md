# telemetry-server

`cargo build`

`cargo ruin`

A telemetry server for OpenMCT

Features:
  - Realtime telemetry data injest
  - Serving telemetry data directly to OpenMCT
  - Store telemetry in timeseries database
  - Fetch requests for historical telemetry data
  
By default runs on port `8081`

Injest arbitrary raw telemetry points via a websocket to `ws://localhost:8081/injest/{key}`.
`{key}` should be uniquelly identifiable key for the telemetry data. You can have multiple sockets providing data for the same telemetry value.
The input should be a raw floating point number sent as a websocket text message.

This will get packaged as a generic telemetry point for OpenMCT of the form
```
{
  timestamp: <auto generated timestamp (millis since epoch)>
  value: <parsed floating point number from websocket message>
}
```
If the text message cannot be converted to a floating point number, it will be ignored.

The packaged telemetry point gets sent to all connected OpenMCT clients.
Client connections are also made on a websocket but to `ws://localhost:8081/realtime/{key}`.
There can be any number of clients connected and listening for the same telemetry value.

If there's no clients listening for a certain key, the injest data will be ignored.
If there's no injest for a key, the server will keep the connection alive with the client and start providing telemetry points as soon as there's data being injested for the respective key.

In our usecase we use the OpenMCT domain object's `{namespace}.{key}` as the key for the telemetry data points.

Plugins in the `CIUDSE/ciudse-telemetry` repo are developed in conjuction with this server.

This server could also be used to statically serve the frontend.

An example telemetry data generator is providad as a python program. `/example_telemetry_generator.py`


When built in release mode, will serve webpage from `static/`. If in debug, will serve from `ciudse_telemetry/dist`
