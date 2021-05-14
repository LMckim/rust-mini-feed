import socket
from time import sleep

sockfd = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
fp = open("test_file.dump", "rb")
sent = 0
while True:
    fp.seek(0)
    for line in fp: 
        sockfd.sendto(line.replace(b'\n',b''), ("localhost", 3333))
        sent += 1
        if sent % 1000 == 0: 
            print(f"Sent: {sent}\r", end='')
        # sleep(0.001)