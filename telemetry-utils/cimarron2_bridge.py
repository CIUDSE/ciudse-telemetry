import asyncio
import websockets
import serial

async def main():
    namespace = 'cimarron2'
    with serial.Serial('/dev/cu.usbserial-02281482', 115200) as ser:
        async with websockets.connect(f'ws://localhost:80/injest?namespace={namespace}&key=velx') as velx:
            async with websockets.connect(f'ws://localhost:80/injest?namespace={namespace}&key=vely') as vely:
                async with websockets.connect(f'ws://localhost:80/injest?namespace={namespace}&key=velz') as velz:
                    async with websockets.connect(f'ws://localhost:80/injest?namespace={namespace}&key=rotx') as rotx:
                        async with websockets.connect(f'ws://localhost:80/injest?namespace={namespace}&key=roty') as roty:
                            async with websockets.connect(f'ws://localhost:80/injest?namespace={namespace}&key=rotz') as rotz:
                                async with websockets.connect(f'ws://localhost:80/injest?namespace={namespace}&key=height') as height:
                                    async with websockets.connect(f'ws://localhost:80/injest?namespace={namespace}&key=pos') as pos:
                                        while True:
                                            await asyncio.sleep(0.1)
                                            if not ser.in_waiting > 0:
                                                continue
                                            line = ser.readline().decode()
                                            words = line.split()
                                            if words[0] != 'DATA':
                                                continue
                                            print(f'Received {line}')
                                            await velx.send(f'value={words[1]}')
                                            await vely.send(f'value={words[2]}')
                                            await velz.send(f'value={words[3]}')
                                            await rotx.send(f'value={words[4]}')
                                            await roty.send(f'value={words[5]}')
                                            await rotz.send(f'value={words[6]}')
                                            await height.send(f'value={words[7]}')
                                            await pos.send(f'lat={words[8]},lng={words[9]},hgt={words[7]}')

if __name__ == "__main__":
    asyncio.run(main())