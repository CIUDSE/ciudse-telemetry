import asyncio
import websockets
from time import time, sleep
from math import sin, pi

async def main():
    # 172.17.0.1 is the default IP for host if running inside Docker container
    # If running on Docker Desktop on Windows, you might need to use host.docker.internal
    # For production, create a container for Python apps and attach it to the network with docker-compose
    namespace = 'test-spaceship'
    key = 'fuel'
    uri = 'ws://localhost:80/injest?namespace=test_spaceship&key=pos'
    async with websockets.connect(uri) as websocket:
        t0 = time()
        while True:
            t = time()
            lat = (2*pi*t/30) % (2*pi)
            lng = 0
            hgt = 1000000 + 10000*(t-t0)
            msg = f'lat={lat},lng={lng},hgt={hgt}'
            await websocket.send(msg)
            print(msg)
            await asyncio.sleep(1.0)


if __name__ == "__main__":
    asyncio.run(main())