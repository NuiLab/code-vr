# Example of simple echo server
# www.solusipse.net

import socket
import os
import subprocess

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

            if acu != "": # if we received a path
                            
                print("received: ", acu)
                raw_path = acu.strip()
                file_delim = raw_path.rfind('/')
                file_name = raw_path[raw_path.rfind('/')+1:raw_path.rfind('.')]
                file_name_ext = file_name + ".json"

                print("checking if file exists...")
                path = "<" + raw_path + ">"
                if (os.path.isfile(raw_path)):
                    print("file exists!")
                    json = open(file_name_ext, 'w+')
                    print("output file name: %s" % file_name_ext)

                    # run the astexport module:
                    # use stderr to check
                    subprocess.call(['astexport', '--i', raw_path], stdout=json)
                    json.seek(0, 0)
                    line = json.readline()
                    print("%s" % line)
                    json.close()

                    # at this point we should have the .json file to send
                    # query .json file size
                    json_path = './'+file_name_ext
                    file_size = os.path.getsize(json_path)
                    print("sending %s bytes" % file_size)

                    # send file size to client
                    current_connection.send(str(file_size).encode('ascii'))

                    # open in byte mode
                    json_bytes = open(file_name_ext, 'rb')
                    buff_read = 0
                    bytes_remaining = int(file_size)
                    
                    while bytes_remaining != 0:
                        if bytes_remaining >= 8: # slab >= 8 bytes
                            buff_read = json_bytes.read(8)
                            sizeof_slab_read = len(buff_read)
                            print('read: %d bytes' % sizeof_slab_read)
                            # send slab to client
                            current_connection.send(buff_read)
                            bytes_remaining = bytes_remaining - int(sizeof_slab_read)
                        else: # slab smaller than 8 bytes
                            buff_read = json_bytes.read(bytes_remaining)
                            sizeof_slab_read = len(buff_read)
                            print('read: %d bytes' % sizeof_slab_read)
                            # send small slab to client
                            current_connection.send(buff_read)
                            bytes_remaining = bytes_remaining - int(sizeof_slab_read)
                    print('read the file completely')

                    # remove local json file (residual)
                    os.remove(json_path)

                else:
                    print("file not found")

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

if __name__ == "__main__":
    try:
        listen()
    except KeyboardInterrupt:
        pass