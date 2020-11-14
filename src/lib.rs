use std::net::{TcpListener, SocketAddr, IpAddr, Ipv4Addr, TcpStream};
use std::process::Command;
use std::borrow::Borrow;
use std::error::Error;

pub mod http;
use http::{ Request, Response };
use std::io::{Read, Write};

const DEFAULT_BUFFER_SIZE: usize = 4096;

/// Run server with closure function as parameter to handle connection.<br><br>
/// unfinished...
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

/// Handle connection of each request.<br><br>
/// unfinished...
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

/// Try open the root view of the `listener` in web browser.<br>
/// 尝试在浏览器中打开这个`listener`所对应的地址。<br><br>
///
/// If it cannot be opened in the browser automatically,
/// the address will be log out to let user to open it
/// in the browser manually.<br>
/// 如果无法自动在浏览器中打开，
/// 将会在控制台输出对应的网址，
/// 用户可以手动在浏览器中打开这个网址。<br><br>
fn try_open_in_browser(listener_borrow: &TcpListener) {
    // use listener_borrow.local_addr() rather than 127.0.0.1:0 here.
    let address_url = listener_borrow.local_addr()
        .expect("Cannot get server address.").to_string();
    let address_url = address_url.as_str();

    match try_call_system_command(address_url) {
        Ok(()) => println!("Opened in browser successfully."),
        Err(_) => println!(
            "Cannot open in browser automatically.\n\
            Please visit {} in your browser.", address_url
        )
    }

    /// Try call system to open an address in browser.<br>
    /// 尝试调用系统在浏览器中打开网址。<br><br>
    ///
    /// # Details 细格
    /// *   Now only `windows` `macos` `linux` are supported.
    ///     And only `windows` is fully supported.
    ///     ( The development of support for other operation
    ///     systems are not finished yet. )
    /// *   In `linux`, as there might be no browser or no default browser,
    ///     it will try `firefox` first, because `firefox`
    ///     is also developed by `mozilla`.
    /// *   仅 `windows` `macos` 和 `linux` 系统是受到支持的，
    ///     并且仅 `windows` 是受到稳定支持的，其他系统还没有适配好。
    /// *   在 `linux` 系统中，可能没有浏览器或没有默认浏览器。
    ///     这个程序首先会尝试火狐浏览器，因为火狐浏览器
    ///     和 Rust 编程语言都是 Mozilla 开发的。(2333)
    fn try_call_system_command(address: &str)
        -> Result<(), Box<dyn Error>> {

        match std::env::consts::OS {
            // try using "cmd" in Windows Operation System
            "windows" => Command::new("cmd")
                .arg("start").arg(address).output()?,

            // try using terminal in Mac Operation System
            "macos" => Command::new("terminal")
                .arg("open").arg(address).output()?,

            // not Windows and not MacOS,
            // so it is most likely to be Linux.
            // If not Linux, it is not supported yet.
            _ => Command::new("terminal")
                .arg("firefox").arg(address).output()?,
        }; Ok(())
    }
}
