# Raw socket connection with no validation and string quoting logic.
# Refer to protocol description:
# http://questdb.io/docs/reference/api/ilp/overview

import time
import socket
import sys

HOST = 'localhost'
PORT = 9009
# For UDP, change socket.SOCK_STREAM to socket.SOCK_DGRAM
sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)


def send_utf8(msg):
    sock.sendall(msg.encode())


try:
    sock.connect((HOST, PORT))
    # Single record insert
    send_utf8(f'trades,name=client_timestamp value=12.4 {time.time_ns()}\n')
    # Omitting the timestamp allows the server to assign one
    send_utf8('trades,name=server_timestamp value=12.4\n')
    # Streams of readings must be newline-delimited
    send_utf8('trades,name=ilp_stream_1 value=12.4\n' +
              'trades,name=ilp_stream_2 value=11.4\n')
    msg = 'test_spaceship#pos, lng=0,lat=3.7928549024093954,hgt=1010022.3588943481 1656001188112000000\n'
    send_utf8(msg)

except socket.error as e:
    sys.stderr.write(f'Got error: {e}')

sock.close()
