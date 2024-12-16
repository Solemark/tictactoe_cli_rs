use std::{
    fs::{read_to_string, File},
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

pub const SR_ROUTE: &str = "127.0.0.1:5000";
pub const P1_ROUTE: &str = "127.0.0.1:5001";
pub const P2_ROUTE: &str = "127.0.0.1:5002";
pub const CN_ROUTE: &str = "127.0.0.1:5003";

pub const BIND_ERR: &str = "TcpListener::bind failed";
pub const CONN_ERR: &str = "TcpStream::connect failed";
pub const WRTE_ERR: &str = "TcpStream::write failed";
pub const READ_ERR: &str = "TcpStream::read failed";

pub const FLSH_ERR: &str = "Stdout::flush failed";
pub const RDLN_ERR: &str = "Stdin::read_line failed";

pub fn send_and_recv(msg: String, send_addr: &str, resp_addr: &str) -> String {
    let mut res = String::new();
    send_message(msg, send_addr);

    for stream in TcpListener::bind(resp_addr).expect(BIND_ERR).incoming() {
        res = handle_response(stream.unwrap());
        break;
    }
    res
}

pub fn send_message(msg: String, addr: &str) {
    TcpStream::connect(addr)
        .expect(CONN_ERR)
        .write(msg.as_bytes())
        .expect(WRTE_ERR);
}

pub fn handle_response(mut stream: TcpStream) -> String {
    let mut msg = [0; 1024];
    stream.read(&mut msg).expect(READ_ERR);
    String::from_utf8_lossy(&msg).to_string()
}

pub fn write_to_file(msg: &str, name: &str) {
    let mut data = read_to_string(format!("data/{}.csv", name))
        .unwrap_or_default()
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    data.push(msg[1..].to_string());
    Write::write_all(
        &mut File::create(format!("data/{}.csv", name)).unwrap_or_else(|_| panic!()),
        data.into_iter()
            .map(|s| format!("{}\n", s))
            .collect::<String>()
            .as_bytes(),
    )
    .unwrap_or_default();
}
