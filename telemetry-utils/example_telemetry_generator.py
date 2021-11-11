import asyncio
import websockets
from time import time, sleep
from math import sin, pi

def producer():
    t = time()
    x = 5.0*sin(t*2.0*pi/5.0)+5.0
    return str(x)

async def main():
    # 172.17.0.1 is the default IP for host if running inside Docker container
    # If running on Docker Desktop on Windows, you might need to use host.docker.internal
    # For production, create a container for Python apps and attach it to the network with docker-compose
    uri = 'ws://host.docker.internal:8080/injest/test-spaceship.fuel'
    async with websockets.connect(uri) as websocket:
        while True:
            message = producer()
            await websocket.send(message)
            print(message)
            await asyncio.sleep(1)


if __name__ == "__main__":
    asyncio.run(main())