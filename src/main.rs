#![allow(non_snake_case)]

extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::cell::RefCell;
use std::io::BufReader;
use std::rc::Rc;
use std::env;
use std::net::SocketAddr;

use futures::prelude::*;

use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;
use tokio_io::AsyncRead;
use tokio_io::io::{lines, write_all};

use serde_json::Error;

mod transfer;

use transfer::*;

fn main() {

    let addr = env::args().nth(1).unwrap_or("0.0.0.0:6070".to_string());
    let addr = addr.parse::<SocketAddr>().unwrap();
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let listener = TcpListener::bind(&addr, &handle).expect("failed to bind");

    println!("Listening on: {}", addr);

    let initial_db: Vec<FalconMethod> = Vec::new();
    let db = Rc::new(Database {
        methods: RefCell::new(initial_db),
    });

    let done = listener.incoming().for_each(move |(socket, _addr)| {

        let (reader, writer) = socket.split();

        let lines = lines(BufReader::new(reader));

        let db = db.clone();
        let responses = lines.map(move |line| {
            let result: Result<FalconMethod, Error> = serde_json::from_str(&line);
            match result {
                Ok(req) => {
                    let mut db = db.methods.borrow_mut();
                    println!("{}, {} {} {}", db.len(), req.method, req.params[0][0].metric, req.params[0][0].value);
                    db.push(req);

                    return SimpleRpcResponse {Code: 0};
                },
                Err(e) => {
                    println!("{}", e);

                    return SimpleRpcResponse {Code: 1};
                }
            }
        });

        let writes = responses.fold(writer, |writer, response| {

            let mut response = serde_json::to_string(&response).unwrap();
            response.push('\n');
            write_all(writer, response.into_bytes()).map(|(w, _)| w)
        });

        let msg = writes.then(move |_| Ok(()));
        handle.spawn(msg);
        Ok(())
    });

    core.run(done).unwrap();
}
