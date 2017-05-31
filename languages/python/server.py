# Example of simple echo server
# www.solusipse.net

import socket

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
            data = current_connection.recv(1024)

            '''
            if decoded_data == "quit":
                current_connection.shutdown(1)
                current_connection.close()
                break

            elif decoded_data == "stop":
                current_connection.shutdown(1)
                current_connection.close()
                exit()
            '''

            # elif
            if data: # if data is present
                # decode it
                decoded_data = data.decode('ascii')

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
                print("received: ", decoded_data)
                print("uppercase: ", data_upper)
                # send it
                current_connection.send(encoded_data)

if __name__ == "__main__":
    try:
        listen()
    except KeyboardInterrupt:
        pass