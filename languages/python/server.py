# Example of simple echo server
# www.solusipse.net

import socket
import os

def listen():
    connection = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    connection.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    connection.bind(('127.0.0.1', 5555))
    connection.listen(10)
    print('Server is listening for connections')
    while True:
        #wait to accept a connection - blocking call
        current_connection, address = connection.accept()
        print('Connected with ' + address[0] + ':' + str(address[1]))

        while True:

            # accumulator
            acu = ""

            # receive message size
            size = current_connection.recv(8)

            if size: # if received message size
                
                print("for message size we received: %s" % size)

                # send ACK
                print("sending back message size ACK")
                ack = "ACK"
                current_connection.send(ack.encode('ascii'))

                # prepare to receive message
                decoded_message = size.decode('ascii')
                delim = decoded_message.find('\r')
                decoded_size = decoded_message[:delim]
                
                print("receiving %s bytes" % decoded_size)
                remainingData = int(decoded_size)

                while remainingData != 0:
                    if remainingData >= 8 : # slab >= 8 byte buffer
                        # receive slab from client
                        slab = current_connection.recv(8)
                        acu = acu + slab.decode('ascii')
                        sizeofSlabReceived = len(slab)
                        print("wrote %d bytes" % len (slab))
                        remainingData = remainingData - int(sizeofSlabReceived)
                    else:
                        # receive slab from client
                        slab = current_connection.recv(remainingData)
                        acu = acu + slab.decode('ascii')
                        sizeofSlabReceived = len(slab)
                        print("wrote %d bytes" % len (slab))
                        remainingData = remainingData - int(sizeofSlabReceived)
            
            print("received: ", acu)
            
            if acu != "":
                # make msg upper case
                acu = acu.upper()

                print("sending: ", acu)


                # send message size
                responseSize = len(acu)
                print("sending %d bytes" % responseSize)
                sizeS = str(responseSize)
                current_connection.send(sizeS.encode('ascii'))
                
                # encode it
                encoded_acu = acu.encode('ascii')

                # send message:
                current_connection.send(encoded_acu)


            '''
            if data: # if data is present
                # decode it
                decoded_data = data.decode('ascii')
                print("length: ", len(decoded_data))

                # check if file exists in file path
                path = str(decoded_data)
                #if os.path.isfile(f): # if file exists
                    # call astexport module on file
                    # send the json content to the client

                # check if it is a command
                if decoded_data == "quit\r":
                    print("received: quit, ending sesh")
                    current_connection.send(data)
                    current_connection.shutdown(1)
                    current_connection.close()
                    exit(1)

                # make it upper case
                data_upper = decoded_data.upper()

                # encode it again
                encoded_data = data_upper.encode('ascii')

                # check to add EOF
                #if len(decoded_data) < 8: 
                    #print("adding EOF");
                    #encoded_data += b'\x04'
                    
                print("received: ", decoded_data)
                print("uppercase: ", data_upper)

                # send message size:
                size = len(decoded_data)
                print("sending a message of size ", size)
                sizeS = str(size)
                current_connection.send(sizeS.encode('ascii'))

                # send message:
                current_connection.send(encoded_data)

                '''

if __name__ == "__main__":
    try:
        listen()
    except KeyboardInterrupt:
        pass