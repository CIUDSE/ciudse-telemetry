import time
import socket

HOST = '127.0.0.1'
PORT = 9009
# For UDP, change socket.SOCK_STREAM to socket.SOCK_DGRAM
sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

try:
  sock.connect((HOST, PORT))
  # Single record insert
  sock.sendall(('trades,name=client_timestamp value=12.4 %d\n' %(time.time_ns())).encode())
  # Omitting the timestamp allows the server to assign one
  sock.sendall(('trades,name=server_timestamp value=12.4\n').encode())
  # Streams of readings must be newline-delimited
  sock.sendall(('trades,name=ilp_stream_1 value=12.4\ntrades,name=ilp_stream_2 value=11.4\n').encode())

except socket.error as e:
  print("Got error: %s" % (e))

sock.close()