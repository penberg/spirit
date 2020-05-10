extern crate spirit;

use spirit::{Connection, Data, Endpoint, Proactor};

fn echo(conn: &Connection, data: Data) {
    conn.send(data);
}

fn echo_server(proactor: &mut Proactor, ep: Endpoint) {
    let listener = spirit::listen(ep);
    listener.on_connection(|mut c| {
        c.on_receive(echo);
        c.start();
    });
    listener.start(proactor)
}

fn main() {
    let ep = Endpoint::port(7777);

    spirit::run_loop(|proactor| {
        echo_server(proactor, ep);
    });
}
