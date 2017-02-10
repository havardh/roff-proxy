use std::sync::mpsc::Receiver;
use std::net::Shutdown;
use std::net::SocketAddr;

use futures::{Future, Stream};
use tokio_core::io::{copy, Io};
use tokio_core::net::{TcpListener, TcpStream};
use tokio_core::reactor::{Core, Handle};

use state::State;

pub fn start(rx: Receiver<State>, server_addr: SocketAddr, client_addr: SocketAddr) {

    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let socket = TcpListener::bind(&client_addr, &handle).unwrap();

    let mut state = State::On;
    let server = socket.incoming().for_each(|(socket, addr)| {
        println!("call {} for {}", server_addr, addr);

        while let Ok(new_state) = rx.try_recv() {
            state = new_state;
        }

        println!("state: {:?}", state);

        match state {
            State::Off => reject_request(socket),
            State::On => handle_request(socket, &server_addr, &handle),
        }

        Ok(())
    });

    core.run(server).unwrap();

}

fn handle_request(sock: TcpStream, server_addr: &SocketAddr, handle: &Handle) {
    let server_pair = TcpStream::connect(&server_addr, &handle).map(|socket| socket.split());

    let amt = server_pair.and_then(|(server_reader, server_writer)| {
        let (client_reader, client_writer) = sock.split();
        let upload = copy(client_reader, server_writer);
        let download = copy(server_reader, client_writer);
        upload.join(download)
    });

    let msg = amt.then(|res| {
        if let Ok((sent, received)) = res {
            println!("bytes sent {:?}, bytes received {:?}", sent, received);
        }
        Ok(())
    });

    handle.spawn(msg);
}

fn reject_request(socket: TcpStream) {
    socket.shutdown(Shutdown::Both).expect("shutdown failed");
}
