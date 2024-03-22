use std::net::UdpSocket;

const SERVER_ADDRS: &str = "127.0.0.1:8080";

fn main() {
    let socket: UdpSocket = UdpSocket::bind(SERVER_ADDRS)
        .expect("Failed to bind to address");

    println!("Server listening on {SERVER_ADDRS}");

    let mut buffer: [u8; 1024] = [0; 1024];

    loop {
        let (size, source) = socket.recv_from(&mut buffer)
            .expect("Failed to receive data");
        
        let request = String::from_utf8_lossy(&buffer[..size]);
        
        println!("Received request: {} from {}", request, source);

        let response = "RESPONG";

        socket.send_to(response.as_bytes(), source).expect("Failed to send response");
    }
}