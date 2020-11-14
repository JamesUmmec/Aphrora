use std::net::{TcpListener, SocketAddr, IpAddr, Ipv4Addr, TcpStream};
use std::process::Command;
use std::borrow::Borrow;
use std::error::Error;

pub mod http;
use http::{ Request, Response };
use std::io::{Read, Write};

const DEFAULT_BUFFER_SIZE: usize = 4096;

pub fn run_server<F>(views_handler: F) where
    F: Fn(Request) -> Response,
    F: Send + Copy + 'static {

    // port 0 means allocate port automatically by the system.
    let localhost = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127,0,0,1)), 0);
    // bind port and launch server.
    let listener = TcpListener::bind(localhost)
        .expect("Cannot allocate available port.");

    // open in default browser.
    try_open_in_browser(listener.borrow());

    // deal with each request
    for stream in listener.incoming() {
        // if connection established failed, just pass.
        // because browser will request again.
        match stream { Err(_) => (), Ok(stream) => {
            match handle_connection(stream, views_handler) {
                Ok(_) => (), Err(_) => (),
            };
        }, };
    }
}

fn handle_connection<F>(mut stream: TcpStream, views_handler: F)
    -> Result<(), Box<dyn Error>> where
    F: Fn(Request) -> Response,
    F: Send + Copy + 'static {

    // read into buffer as bytes array.
    let mut buffer = [0u8; DEFAULT_BUFFER_SIZE];
    stream.read(& mut buffer)?;

    // read buffer into String and the parse into Request object.
    let request = String::from_utf8(buffer.to_vec())?;
    let request_object = Request::from(request.as_str());

    let response =  views_handler(request_object).to_string();

    stream.write(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}

fn try_open_in_browser(listener_borrow: &TcpListener) {
    // use listener_borrow.local_addr() rather than 127.0.0.1:0 here.
    let address_url = listener_borrow.local_addr()
        .expect("Cannot get server address.").to_string();
    let address_string = format!("http://{}", address_url);
    let address_str = address_string.as_str();

    match try_call_system_command(address_str) {
        Ok(()) => println!(
            "If your browser is not opened,\n\
            please visit {} in your browser.", address_str
        ),

        Err(_) => println!(
            "Cannot open in browser automatically.\n\
            Please visit {} in your browser.", address_str
        )
    }

    fn try_call_system_command(address: &str)
        -> Result<(), Box<dyn Error>> {

        match std::env::consts::OS {
            // try using powershell in Windows Operation System
            "windows" => { Command::new("powershell")
                .arg("start").arg(address).output()?; },

            // try using terminal in Mac Operation System
            // here might be bugs
            "macos" => { Command::new("terminal")
                .arg("open").arg(address).output()?; },

            // try using curl in Linux Operation System
            // here might be bugs
            "linux" => { Command::new("terminal")
                .arg("curl").arg(address).output()?; },

            _ => { println!("Unsupported Operation System..."); },
        }; Ok(())
    }
}
