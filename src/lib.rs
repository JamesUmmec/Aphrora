use std::net::TcpListener;

pub mod http;
use http::{ Request, Response };

pub fn run_server<F>(views_handler: F)
    where F: Fn(Request) -> Response, F: Send + Copy + 'static {
    // coding here
}
