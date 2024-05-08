use bevy::prelude::*;
use std::net::UdpSocket;
use std::io;

use crate::GameModeState;
use crate::data_pack::DataPack;

const CLIENT_ADDRS: &str = "127.0.0.1:8000";
const SERVER_ADDRS: &str = "127.0.0.1:8080";

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(OnEnter(GameModeState::Online), send_msg);
    }
}

fn send_msg() {
    let socket = UdpSocket::bind(CLIENT_ADDRS).expect("Error al conectar");

    let mut buffer: [u8; 1024] = [0; 1024];

    loop {
        let mut user = String::new();
        let mut password = String::new();

        io::stdin()
            .read_line(&mut user)
            .unwrap();

        io::stdin()
            .read_line(&mut password)
            .unwrap();
            
        let data_pack = DataPack::new(&user, &password);

        socket
            .send_to(data_pack.to_string().as_bytes(), SERVER_ADDRS)
            .expect("Error al enviar");

        let (size, _) = socket
            .recv_from(&mut buffer)
            .expect("Error al recibir");

        let request = String::from_utf8_lossy(&buffer[..size]);
    
        println!("Servidor: {request}");
    }
}