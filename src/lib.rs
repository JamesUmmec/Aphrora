use std::net::{TcpListener, SocketAddr, IpAddr, Ipv4Addr};

pub mod http;
use http::{ Request, Response };

pub fn run_server<F>(views_handler: F)
    where F: Fn(Request) -> Response, F: Send + Copy + 'static {

    // port 0 means allocate port automatically by the system.
    let localhost = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127,0,0,1)), 0);
    // bind port and launch server.
    let listener = TcpListener::bind(localhost)
        .expect("Cannot allocate available port.");
}
