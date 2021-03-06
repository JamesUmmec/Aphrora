use std::net::{TcpListener, SocketAddr, IpAddr, Ipv4Addr, TcpStream};
use std::process::Command;
use std::borrow::Borrow;
use std::error::Error;

/// Basic support for `HTTP`:<br>
/// Parse http string into `Request`
/// and parse `Response` object into string.
pub mod http;
pub mod file;

use http::{ Request, Response };
use std::io::{Read, Write};

const DEFAULT_BUFFER_SIZE: usize = 4096;

/// Run server with a closure of how to deal with the requests.<br><br>
///
/// # Example
/// Coding a closure in the `run_server()` function like this,
/// and when visit any view within this port,
/// it will display a word `hello` on that page.
/// ```
/// use aphrora::http::{Request, Response, RequestMethod, ResponseStatus};
/// use aphrora::run_server;
///
/// fn main() {
///     run_server(|request| {
///         println!("view of request: {}", request.view);
///         Response{
///             status: ResponseStatus::OK,
///             message: String::from("hello"),
///         }
///     });
/// }
/// ```
/// Here in that example, as the code, when you visit any view,
/// it will return a `hello` into your browser,
/// you can see a line of `hello` in which webpage in your browser.
/// You can also replace the `hello` with
/// a string read from a `example.html` file,
/// and then it will return show the file
/// in the browser webpage.
///
/// There must be various functions in a server,
/// so you are supposed to use something like
/// `match` expression to deal with that.
/// You can match the `view` property of the `Request`
/// and then call some function to deal with it.
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

/// Handle connection in a spawned thread,
/// it takes only the `stream` and will return
/// before the function end.
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

/// Try open the address which the server binds to.
/// It will try to call the system to open the browser
/// via system command.
///
/// For example, in `windows`, it will call `PowerShell`
/// to run `start http://127.0.0.1:your-port`
/// ("your-port" here is the port code number).
///
/// Now only `windows`, `macos` and `linux` are supported,
/// and those codes had been tested only on `windows`,
/// so there might be something wrong when using `linux` and `macos`.
/// But don't worry, it will log out the address in the console.
/// If it didn't open in browser successfully,
/// you can open it manually.
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
