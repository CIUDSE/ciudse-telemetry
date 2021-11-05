import asyncio
import aiohttp
import websockets
import requests
from time import time, sleep
from math import sin, pi

telemetry_mappings = {
    "n.heading": "kerbal.heading",
    "n.pitch": "kerbal.pitch",
    "n.roll": "kerbal.roll",
    "v.long": "kerbal.longitude",
    "v.lat": "kerbal.latitude",
}
openmct_domain = "localhost"
openmct_port = 8081
telemachus_domain = "127.18.208.1"
telemachus_port = 8085

async def connect_openmct(ws_connections, telemachus_key, openmct_key):
    print(f"Connecting to OpenMCT injest for {openmct_key}")
    connection = await websockets.connect(f"ws://{openmct_domain}:{openmct_port}/injest/{openmct_key}")
    print(f"OpenMCT injest connected! {openmct_key}")
    ws_connections.append({
        'telemachus_key': telemachus_key,
        'ws_connection': connection
    })

async def openmct_send(ws, v):
    await ws.send(v)
    #print(f"Sent {v} to {ws.path}")

async def update_telemetry(ws_connections):
    s = []
    s.append(f"http://{telemachus_domain}:{telemachus_port}/telemachus/datalink?")
    active_connections = list(ws_connections)
    for i, p in enumerate(active_connections):
        s.append(f"a{i}={p['telemachus_key']}&")
    url = ''.join(s)
    async with aiohttp.ClientSession() as session:
        #print(f"Requesting Kerbal telemetry. {url}")
        async with session.get(url) as r:
            data = await r.json()
            #print("Got Kerbal response")
            #print(data)
            for key, val in data.items():
                i = int(key[1:])
                v = float(val)
                #print(f"OpenMCT send {i}: {v}")
                asyncio.ensure_future(openmct_send(active_connections[i]['ws_connection'], str(v)))
    await asyncio.sleep(0.1)
    asyncio.ensure_future(update_telemetry(ws_connections))
    
if __name__ == "__main__":
    ws_connections = []
    for telemachus_key, openmct_key in telemetry_mappings.items():
        asyncio.ensure_future(connect_openmct(
            ws_connections, telemachus_key, openmct_key
        ))
    asyncio.ensure_future(update_telemetry(ws_connections))
    asyncio.get_event_loop().run_forever()