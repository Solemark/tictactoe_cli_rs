mod consts;
mod piece;
mod server;
mod test;
mod tictactoe;

use server::Server;

fn main() {
    Server::new().start();
}
