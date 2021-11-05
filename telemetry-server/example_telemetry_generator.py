import asyncio
import websockets
from time import time, sleep
from math import sin, pi

def producer():
    t = time()
    x = 5.0*sin(t*2.0*pi/5.0)+5.0
    return str(x)

async def run_client(uri):
    websocket = await websockets.connect(uri)
    while True:
        message = producer()
        await websocket.send(message)
        print(message)
        await asyncio.sleep(0.1)


if __name__ == "__main__":
    asyncio.ensure_future(run_client("ws://localhost:8081/injest/test-spaceship.fuel"))
    asyncio.get_event_loop().run_forever()