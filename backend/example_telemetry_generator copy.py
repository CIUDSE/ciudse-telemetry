import asyncio
import websockets
from time import time, sleep
from math import sin, pi

t0 = time()

def producer():
    t = time() - t0
    x = 100.0 - 0.5 * t + sin(5.0*t)
    return str(x)

async def run_client(uri):
    websocket = await websockets.connect(uri)
    while True:
        message = producer()
        await websocket.send(message)
        await asyncio.sleep(1)


if __name__ == "__main__":
    asyncio.ensure_future(run_client("ws://localhost:8081/injest/test-spaceship.battery"))
    asyncio.get_event_loop().run_forever()