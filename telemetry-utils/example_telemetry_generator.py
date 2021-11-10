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
    # 172.17.0.1 is the default IP for host if running inside Docker container
    # If running on Docker Desktop on Windows, you might need to use host.docker.internal
    # For production, create a container for Python apps and attach it to the network with docker-compose
    asyncio.ensure_future(run_client("ws://172.17.0.1:8080/injest/test-spaceship.fuel"))
    asyncio.get_event_loop().run_forever()