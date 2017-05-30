import socket
import sys
import os
import select
import re
from thread import *

path = ("./")
fileList = []
_lsremote = "ls-remote"
 
HOST = ''   # Symbolic name meaning all available interfaces
PORT = 22181 # Arbitrary non-privileged port
 
s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
print('Socket created')
 
#Bind socket to local host and port
try:
    s.bind((HOST, PORT))
except socket.error, msg:
    print 'Bind failed. Error Code : ' + str(msg[0]) + ' Message ' + msg[1]
    sys.exit()
     
print 'Socket bind complete'
 
#Start listening on socket
s.listen(10)
print 'Socket now listening'
 
#Function for handling connections. This will be used to create threads
def clientthread(conn):
    #Sending message to connected client
    conn.send('Connected, type !help for command list:\n') #send only takes string
     
    #infinite loop so that function do not terminate and thread do not end.
    while True:
         
        #Receiving from client
        data = conn.recv(1024)
        # telnet sends dirty strings, clean before checking
        # cleanData = re.sub('\W+','', data)
        # if(cleanData == _lsremote):
        if _lsremote in data: # list server's directory
            t = data.isalnum() # proof telnet sends dirty strings
            print(t)
            print("User: "+addr[0]+" "+str(addr[1])+" requested ls-remote")
            remoteList = "remote files:"
            files = [f for f in os.listdir('.') if os.path.isfile(f)]
            for f in files:
                remoteList += ("\n-> "+f)
            conn.sendall(remoteList+"\n")
        else:
            print(data)
            reply = 'OK...' + data
        if not data: 
            break
        # conn.sendall(reply)
     
    #came out of loop
    conn.close()
 
#now keep talking with the client
while 1:
    #wait to accept a connection - blocking call
    conn, addr = s.accept()
    print 'Connected with ' + addr[0] + ':' + str(addr[1])
     
    #start new thread takes 1st argument as a function name to be run, second is the tuple of arguments to the function.
    start_new_thread(clientthread ,(conn,))
 
s.close()