extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::net::TcpListener;
use std::io::{Read, Write};
use std::thread;

use std::borrow::BorrowMut;
use std::rc::Rc;
use std::cell::RefCell;

mod transfer;
use transfer::*;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8433").unwrap();

    let initial_db: Database = Database::new();
    let db = Rc::new(RefCell::new(initial_db));

    for stream in listener.incoming() {

        thread::spawn(|| {

            let db = db.clone().borrow_mut();

            match stream {
                Ok(mut stream) => {
                    let mut buf = String::new();
                    stream.read_to_string(&mut buf);

                    let result: FalconMethod = serde_json::from_str(&buf).unwrap();
                    println!("{} {}, {} {}", result.params[0][0].endpoint, result.method, result.params[0][0].metric, result.params[0][0].value);
                    db.get_mut().push(result);

                    let response = b"{\"id\":0,\"result\":{},\"error\":null}";
                    stream.write(response).expect("Response failed");
                }
                Err(e) => {
                    println!("Unable to connect: {}", e);
                }
            }
        });
    }
}
