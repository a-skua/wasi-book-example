#[allow(warnings)]
mod bindings;

use bindings::exports::wasi::cli::run::Guest;
use bindings::wasi::cli::stdout::get_stdout;
use bindings::wasi::sockets::instance_network::instance_network;
use bindings::wasi::sockets::network::{IpAddressFamily, IpSocketAddress, Ipv4SocketAddress};
use bindings::wasi::sockets::tcp::ShutdownType;
use bindings::wasi::sockets::tcp_create_socket::create_tcp_socket;

struct Component;

const HTTP_RESPONSE: &str =
    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 22\n\nHello, wasi:sockets!\r\n";

impl Guest for Component {
    fn run() -> Result<(), ()> {
        let stdout = get_stdout();
        let socket = create_tcp_socket(IpAddressFamily::Ipv4).unwrap();

        let network = instance_network();
        let address = IpSocketAddress::Ipv4(Ipv4SocketAddress {
            port: 8080,
            address: (127, 0, 0, 1),
        });

        // await bind
        stdout.blocking_write_and_flush(b"start_bind...").unwrap();
        socket.start_bind(&network, address).unwrap();
        socket.subscribe().block();

        stdout.blocking_write_and_flush(b"OK!\n").unwrap();
        socket.finish_bind().unwrap();

        // await listen
        stdout.blocking_write_and_flush(b"start_listen...").unwrap();
        socket.start_listen().unwrap();
        socket.subscribe().block();

        stdout.blocking_write_and_flush(b"OK!\n").unwrap();
        socket.finish_listen().unwrap();

        // await accept
        stdout.blocking_write_and_flush(b"accepting...").unwrap();
        socket.subscribe().block();

        stdout.blocking_write_and_flush(b"Accepted!\n").unwrap();
        let (socket, _, output) = socket.accept().unwrap();
        output
            .blocking_write_and_flush(HTTP_RESPONSE.as_bytes())
            .unwrap();
        drop(output);

        stdout
            .blocking_write_and_flush(b"socket shutdown...\n")
            .unwrap();
        socket.shutdown(ShutdownType::Both).unwrap();
        drop(socket);
        Ok(())
    }
}

bindings::export!(Component with_types_in bindings);
