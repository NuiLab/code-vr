import socket
import sys
import os
import select
import re
import time
# from thread import * 
from _thread import *

# print 'Number of arguments:', len(sys.argv), 'arguments.'
# print 'Argument List:', str(sys.argv)

# Run command line argument check
if(len(sys.argv)!=2): # user did not pass port
    print('Usage: [python thread-server.py <portno>]')
    print('wrong arguments for <portno>')
    exit()
else:
    strPortno = str(sys.argv[1])
    if(len(strPortno) == 4 and strPortno.isdigit()):
        print('Command line argument check: OK')
        # run program
    else:
        print('Usage: [python thread-server.py <portno>]')
        print('<portno> must be 4 digit number')
        exit()

path = ("./")
# fileList = []
_lsremote = "ls-remote"
files = [f for f in os.listdir('.') if os.path.isfile(f)]
 
HOST = ''   # Symbolic name meaning all available interfaces
PORT = int(sys.argv[1]) # Arbitrary non-privileged port

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
print('Socket created')
 
#Bind socket to local host and port
try:
    s.bind((HOST, PORT))
except(socket.error, msg):
    print('Bind failed. Error Code : ' + str(msg[0]) + ' Message ' + msg[1])
    sys.exit()
     
print('Socket bind complete')
 
#Start listening on socket
s.listen(10)
print('Socket now listening')
 
#Function for handling connections. This will be used to create threads
def clientthread(conn):
    #Sending message to connected client
    # conn.send('Connected, type !help for command list:\n') #send only takes string
     
    #infinite loop so that function do not terminate and thread do not end.
    while True:
         
        #Receiving from client
        data = conn.recv(1024)

        # telnet sends dirty strings, t = data.isalnum() to check. clean if using telnet

        # we split data at the first white space. first word is opcode, second is file
        _data = data.decode('ascii').split(" ", 1)

        #----ls-remote----#
        if(data == _lsremote):
            # refresh file list
            files = [f for f in os.listdir('.') if os.path.isfile(f)]
            print('User: '+addr[0]+':'+str(addr[1])+' requested ls-remote')
            remoteList = 'remote files:'
            for f in files:
                fileSize = os.path.getsize(f)
                remoteList += ("\n-> "+f+"\t%d bytes" % fileSize)
            conn.sendall(remoteList)

        #----PUT----#
        elif(_data[0] == 'put'):
            # catch file name
            reqFileName = _data[1]
            print('User: '+addr[0]+':'+str(addr[1])+' requested a PUT for: %s' % reqFileName)

            # send back the file name as ACK
            conn.sendall(str(reqFileName))

            #receive file size
            reqFileSize = conn.recv(1024)
            print('PUT file size: '+reqFileSize+' bytes')

            # send back the file size as ACK
            conn.sendall(reqFileSize)

            # receive file

            # send file in slices of 1024 bytes:
            # open file in read byte mode:
            f = open((path+reqFileName), "wb") # write bytes flag is passed
            buffRead = 0
            bytesRemaining = int(reqFileSize)
            while bytesRemaining != 0:
                if(bytesRemaining >= 1024): # slab >= than 1024 buffer
                    # receive slab from client
                    slab = conn.recv(1024)
                    f.write(slab)
                    sizeofSlabReceived = len(slab)
                    print("wrote %d bytes" % len(slab))

                    bytesRemaining = bytesRemaining - int(sizeofSlabReceived)
                else:
                    #receive slab from server
                    slab = conn.recv(bytesRemaining) # of 1024
                    f.write(slab)
                    sizeofSlabReceived = len(slab)
                    print("wrote %d bytes" % len(slab))
                    bytesRemaining = bytesRemaining - int(sizeofSlabReceived)
            print("File uploaded completely")

        #----GET----#
        elif(_data[0] == 'get'):
            _found = False # flag in case file is not found
            print('User: '+addr[0]+':'+str(addr[1])+' requested a GET for: %s' % _data[1])
            files = [f for f in os.listdir('.') if os.path.isfile(f)]
            for f in files:
                #--- file requested found ---#
                if(f == _data[1]):
                    _found = True
                    print('File found. Sending file size to user:')
                    # send 'found' flag to user
                    reply = str(_found)
                    conn.sendall(reply)
                    # get file size and send it to user
                    reqFileSize = os.path.getsize(f)
                    conn.sendall(str(reqFileSize))

                    # receive user file size ACK
                    fSizeACK = conn.recv(1024)
                    print("ACK: "+fSizeACK)

                    print('Preparing upload...')

                    # send file in slices of 1024 bytes:
                    # open file in read byte mode:
                    f = open((path+f), "rb") # read bytes flag is passed
                    buffRead = 0
                    bytesRemaining = int(reqFileSize)  

                    while bytesRemaining != 0:
                        if(bytesRemaining >= 1024): # slab >= than 1024 buffer
                            buffRead = f.read(1024)
                            sizeofSlabRead = len(buffRead)
                            print('remaining: %d' % bytesRemaining)
                            print('read: %d'%sizeofSlabRead)
                            # send slab to client:
                            conn.sendall(buffRead)
                            bytesRemaining = bytesRemaining - int(sizeofSlabRead)
                        else: # slab smaller than 1024 buffer
                            buffRead = f.read(bytesRemaining) # read 1024 bytes at a time
                            sizeofSlabRead = len(buffRead)
                            print('remaining: %d' % bytesRemaining)
                            print('read: %d'%sizeofSlabRead)
                            # send slab to client:
                            conn.sendall(buffRead)
                            bytesRemaining = bytesRemaining - int(sizeofSlabRead)
                    print("Read the file completely")

            #--- file requested not found ---#
            if(_found == False):
                print('File requested not available in dir.')
                reply = str(_found)
                conn.sendall(reply)

            #reply = 'OK...' + data
            #conn.sendall(reply)
            #break
        else:
            print(data)
            reply = 'OK...' + data.decode('ascii')
            conn.sendall(reply.encode('ascii'))

        if not data: 
            break
        # conn.sendall(reply)
     
    #came out of loop
    print('User: '+addr[0]+':'+str(addr[1])+' disconnected')
    conn.close()
 
#now keep talking with the client
while 1:
    #wait to accept a connection - blocking call
    conn, addr = s.accept()
    print('Connected with ' + addr[0] + ':' + str(addr[1]))
     
    #start new thread takes 1st argument as a function name to be run, second is the tuple of arguments to the function.
    start_new_thread(clientthread, (conn,))
 
s.close()