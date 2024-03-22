use bevy::prelude::*;
use std::net::UdpSocket;
use std::io;

use super::GameModeState;

const CLIENT_ADDRS: &str = "127.0.0.1:8000";
const SERVER_ADDRS: &str = "127.0.0.1:8080";

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameModeState::Online), send_msg);
    }
}

fn send_msg() {
    let socket = UdpSocket::bind(CLIENT_ADDRS).expect("Error al conectar");

    let mut buffer: [u8; 1024] = [0; 1024];

    loop {
        let mut message = String::new();

        io::stdin()
            .read_line(&mut message)
            .unwrap();
    
        socket
            .send_to(message.as_bytes(), SERVER_ADDRS)
            .expect("Error al enviar");

        let (size, _) = socket
            .recv_from(&mut buffer)
            .expect("Error al recibir");

        let request = String::from_utf8_lossy(&buffer[..size]);
    
        println!("Servidor: {request}");
    }
}