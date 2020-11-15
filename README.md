![Aphrora 0.1](./static/aphrora-image.png)

# Aphrora

Aphrora is a  simple server frame developed with Rust Programming Language.
It is just a toy server which is not powerful enough to build real web server, but it is convenient to use, especially for beginners to Rust. With `aphrora` imported, you can establish a simple server with only a closure of how to handle the requests, and it will automatically show the homepage in your browser.

Aphrora is not only a solution to build up a simple browser, you can also use it to create a GUI for your Rust application with webpages and http communication.

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

`aphrora::http` is a model contains some basic `struct`s and `enum`s. You can use them to deal with Response and Request as object, rather than `String`.

When launching the server, you only need to call the `run_server()` function with a closure as the example showed. There will be a `http::Request` object which contains structured informations in the http request, and your code are supposed to return a `http::Response` object.

Here in that example, as the code, when you visit any view, it will return a `hello` into your browser, as you can see a line of `hello` in which webpage in your browser. You can also replace the `hello` with a string read from a `example.html` file, and then it will return show the file in the browser webpage.

There must be various functions in a server, so you are supposed to use something like `match` expression to deal with that. You can match the `view` property of the `Request` and then call some function to deal with it.

## About Structures

Here are some of the source code in `aphrora::http`, with which you can have a better understand among the data structure.

```rust
pub struct Request {
    pub method: RequestMethod,
    pub view: String,
    pub message: String,
}
```

```rust
pub enum RequestMethod {
    GET,
    POST,
    Unsupported,
}
```

```rust
pub struct Response {
    pub status: ResponseStatus,
    pub message: String,
}
```

```rust
pub enum ResponseStatus {
    OK,
    NotFound,
    Forbidden,
    InternalServerError,
}
```

As `unfinished` and `under development`, this crate is not available for full HTTP support yet. But it is available for many common utils with only those structures.

So have fun enjoy it, and this project is still under development, it will become more and more powerful gradually.
