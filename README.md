![Aphrora 0.1](./static/aphrora-image.png)

# Aphrora

Aphrora is a  simple server frame developed with Rust Programming Language.
It is just a toy server that is not powerful, but it is convenient to use,
especially for beginners to Rust.

You can also build up some UI based on webpages for your
rust application with this frame and some `AJAX` communication.

## How to use

First you need to add `aphrora` to your `Cargo.toml` dependencies:
```toml
# file: Cargo.toml
# -- snip --
[dependencies]
aphrora = "0.1.0"
```

After `cargo build`, you can use `aphrora` as a model in your application.
Here is an example about how to use it:

```rust
use aphrora::http::{Request, Response, RequestMethod, ResponseStatus};
use aphrora::run_server;

fn main() {
    run_server(|request| {
        println!("view of request: {}", request.view);
        Response{
            status: ResponseStatus::OK,
            message: String::from("hello"),
        }
    });
}
```

In this example, you can see a line of `hello` in the page
opened in your browser.

The `run_server()` function will launch a `TcpListener`
and bind it to an available port allocated by the system.
Then it will try to open the default browser
and visit the root address of the port.

(unfinished)
