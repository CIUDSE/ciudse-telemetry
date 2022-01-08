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
    uri = 'ws://172.17.0.1:8080/injest?namespace=test-spaceship&key=pos'
    async with websockets.connect(uri) as websocket:
        while True:
            t = time()
            x = 5.0*sin(t*2.0*pi/5.0)+5.0
            y = 3.0*sin(t*3.0*pi)-1.0
            z = 4.0*sin(t*1.5*pi-pi*1.5)
            msg = f'x={x},y={y},z={z}'
            await websocket.send(msg)
            print(msg)
            await asyncio.sleep(1.0/20.0)


if __name__ == "__main__":
    asyncio.run(main())