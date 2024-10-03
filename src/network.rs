use std::{io::{Read, Write}, net::{TcpListener, TcpStream}, string, thread};
use rmp_serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use chess_networking::{self, Ack};

const LOCAL_HOST: &str = "127.0.0.1";
pub fn chess_lib_state_to_network_state(state: chess_lib::GameState) -> Option<chess_networking::GameState>{
    return match state{
        chess_lib::GameState::Active => None,
        chess_lib::GameState::GameOver => Some(chess_networking::GameState::CheckMate),
        chess_lib::GameState::Check => None
    }
}
pub fn do_move(stream: &mut TcpStream, _move: chess_lib::Move, promotion: Option<chess_networking::PromotionPiece>) -> (bool, Option<chess_networking::GameState>){
    let to_write = chess_networking::Move {
        from: (
            _move.from.file as u8,
            _move.from.rank as u8,
        ),
        to: (
            _move.to.file as u8,
            _move.to.rank as u8,
        ),
        forfeit: false,
        offer_draw: false,
        promotion: promotion,
    };
    stream.write_all(&Vec::try_from(to_write).unwrap());
    let mut buf = [0u8; 512];
    loop {
        stream.read(&mut buf);
        match chess_networking::Ack::try_from(&buf[..]){
            Ok(_Ack) => {
                return (_Ack.ok, _Ack.end_state);
            },
            _ => {}
        }
    }
}

pub fn await_move(stream: &mut TcpStream) -> (u8, u8, Option<chess_networking::PromotionPiece>){
    let mut buf = [0u8; 512];
    loop {
        stream.read(&mut buf);
        match chess_networking::Move::try_from(&buf[..]){
            Ok(_Move) => {
                return (_Move.from.0, _Move.from.1, _Move.promotion);
            },
            _ => {}
        }
    }
}

pub fn send_ack(stream: &mut TcpStream, valid_move: bool, state: Option<chess_networking::GameState>){
    let to_write = chess_networking::Ack{
        ok: valid_move,
        end_state: state
    };
    stream.write_all(&Vec::try_from(to_write).unwrap());
}

pub fn start_server(port: &str, name: &str) -> (TcpStream, Option<String>){
    let connection = TcpListener::bind(String::from(LOCAL_HOST) + ":" + port);
    if connection.is_err() {
        println!("Failed to setup listener on port {}", port);
    }
    let (mut stream, _addr) = connection.unwrap().accept().unwrap();
    
    let start = chess_networking::Start{
        is_white: false,
        name: Some(String::from(name)),
        fen: None,
        time: None,
        inc: None,
    };
    stream.write(&Vec::try_from(start).unwrap());
    return (stream, None);
}

pub fn start_client(ip: &str, name: &str) -> (TcpStream, Option<String>, chess_lib::Colour){
    let connection = TcpListener::bind(ip);
    if connection.is_err() {
        println!("Failed to connect to {}", ip);
    }
    let (mut stream, _addr) = connection.unwrap().accept().unwrap();
    let mut buf = [0u8; 512];
    loop {
        stream.read(&mut buf);
        match chess_networking::Start::try_from(&buf[..]){
            Ok(_Start) => {
                let color = match _Start.is_white  {
                    true => chess_lib::Colour::White,
                    false => chess_lib::Colour::Black
                };
                return (stream, _Start.name, color);
            },
            _ => {}
        }
    }
}