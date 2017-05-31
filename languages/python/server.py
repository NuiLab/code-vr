# Example of simple echo server
# www.solusipse.net

import socket

def listen():
    connection = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    connection.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    connection.bind(('127.0.0.1', 5555))
    connection.listen(10)
    while True:
        current_connection, address = connection.accept()
        while True:
            data = current_connection.recv(1024)

            if data == 'quit\r\n':
                current_connection.shutdown(1)
                current_connection.close()
                break

            elif data == 'stop\r\n':
                current_connection.shutdown(1)
                current_connection.close()
                exit()

            elif data:
                print("from client: ", address)
                decoded_data = data.decode('ascii')
                data_upper = decoded_data.upper()
                print("received: ", decoded_data)
                print("uppercase: ", decoded_data.upper())
                current_connection.send(data_upper.encode('ascii'))

if __name__ == "__main__":
    try:
        listen()
    except KeyboardInterrupt:
        pass