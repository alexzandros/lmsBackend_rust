import zmq
context = zmq.Context()

socket = context.socket(zmq.REQ)
socket.connect("tcp://localhost:6913")
for i in range(10):
    socket.send(b"pedi mis datos " + str(i).encode('utf-8'))
    mensaje = socket.recv()
    print(mensaje)
