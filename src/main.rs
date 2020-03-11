#[macro_use]
extern crate futures;
extern crate tokio;
extern crate tokio_uds;
extern crate bytes;
extern crate uinput;
extern crate libc;

mod input;

use std::thread;
use std::fs;
use std::str;
use std::time::Duration;
use tokio::timer::Interval;
use std::io::{Read};
use std::sync::{Arc, Mutex};
// use std::sync::mpsc;

use tokio::io;
use tokio::prelude::*;
use tokio::runtime::current_thread::Runtime;
use tokio_uds::{UnixStream, UnixListener};

use futures::sync::oneshot;
use futures::sync::mpsc;
use futures::future::{self, Either};
use futures::Async;
use futures::Sink;
use futures::{Future, Stream};

use bytes::{BufMut, BytesMut};

use uinput::event::keyboard;

fn get_user_sock_path() -> String {
    let uid = unsafe { libc::getuid() };
    format!("/run/user/{}/otp-uinput", uid)
}

struct Shared {
    input: input::Uinput,
}

impl Shared {
    fn new() -> Self {
        Shared {
            input: input::Uinput::new(),
        }
    }
}

#[derive(Debug)]
struct Keys {
    stream: UnixStream,
    rd: BytesMut,
}

impl Keys {
    fn new(stream: UnixStream) -> Self {
        Keys {
            stream,
            rd: BytesMut::new(),
        }
    }

    fn fill_read_buf(&mut self) -> Poll<(), io::Error> {
        loop {
            self.rd.reserve(1024);

            let n = try_ready!(self.stream.read_buf(&mut self.rd));

            if n == 0 {
                println!("read ok {:?}", self.rd);
                return Ok(Async::Ready(()));
            }
        }
    }
}

impl Stream for Keys {
    type Item = BytesMut;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let sock_closed = self.fill_read_buf()?.is_ready();

        let pos = self
            .rd
            .windows(2)
            .enumerate()
            .find(|&(_, bytes)| bytes == b"\r\n")
            .map(|(i, _)| i);
        
        if let Some(pos) = pos {
            let mut line = self.rd.split_to(pos + 2);
            line.split_off(pos);
            return Ok(Async::Ready(Some(line)));
        }

        if sock_closed {
            Ok(Async::Ready(None))
        } else {
            Ok(Async::NotReady)
        }
    }
}

fn handle_key(stream: UnixStream, state: Arc<Mutex<Shared>>) {
    let keys = Keys::new(stream);

    let connection = keys
        .into_future()
        .map_err(|e| e)
        .and_then(move |(name, keys)| {
            let name = match name {
                Some(name) => name,
                None => {
                    println!("keys first {:?} got", keys.rd);
                    return Either::A(future::ok(()));
                }
            };

            println!("name {:?} got", name);
            println!("keys second {:?} got", keys.rd);

            if keys.rd.len() != 1 {
                println!("unexpect key length");
                return Either::B(future::ok(()));
            }

            {
                let first_num = keys.rd[0];
                let mut k = state.lock().unwrap();
                k.input.keyin(first_num);
            }

            Either::B(future::ok(()))
        })
        .map_err(|e| {
            println!("connection error = {:?}", e);
        });

    tokio::spawn(connection);
}

fn main() {
    let mut rt = Runtime::new().unwrap();

    let sock_path = get_user_sock_path();
    let listener = match UnixListener::bind(&sock_path) {
        Ok(m) => m,
        Err(_) => {
            fs::remove_file(&sock_path).unwrap();
            UnixListener::bind(&sock_path).unwrap()
        }
    };
    let (tx, rx) = mpsc::channel::<u8>(1024);
    let state = Arc::new(Mutex::new(Shared::new()));
    // rt.spawn({
    //     tx.into_future()
    //     .send(3)
    // });

    // let state = Arc::new(Mutex::new(Shared::new()));

    rt.spawn({
        listener
            .incoming()
            .map_err(|e| println!("err={:?}", e))
            .for_each(move |stream| {
                handle_key(stream, state.clone());
                Ok(())
            })
    });


    println!("3");
    // let server = rt.block_on(rx).unwrap();
    rt.run().unwrap();
    // let (_, buf) = rt.block_on(io::read_to_end(server, vec![])).unwrap();
    
}
