import asyncio
import websockets
from time import time, sleep
from math import sin, pi

async def main():
    uri = "ws://localhost:8081/injest/test.test_point"
    msg = "424242"
    async with websockets.connect(uri) as websocket:
        await websocket.send(msg)

if __name__ == "__main__":
    asyncio.get_event_loop().run_until_complete(main())