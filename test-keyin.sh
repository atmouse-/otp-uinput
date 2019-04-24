echo -en "aaaaa\r\n1" | socat - UNIX-CONNECT:/tmp/socket
echo -en "aaaaa\r\n2" | socat - UNIX-CONNECT:/tmp/socket
echo -en "aaaaa\r\n3" | socat - UNIX-CONNECT:/tmp/socket
echo -en "aaaaa\r\n4" | socat - UNIX-CONNECT:/tmp/socket
echo -en "aaaaa\r\n5" | socat - UNIX-CONNECT:/tmp/socket
echo -en "aaaaa\r\n6" | socat - UNIX-CONNECT:/tmp/socket
