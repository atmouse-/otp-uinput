#[macro_use]
extern crate futures;
extern crate tokio;
extern crate tokio_uds;
extern crate bytes;
extern crate uinput;

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

const SOCK_PATH: &'static str = "/tmp/otp-uinput";

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

    let listener = match UnixListener::bind(SOCK_PATH) {
        Ok(m) => m,
        Err(_) => {
            fs::remove_file(SOCK_PATH).unwrap();
            UnixListener::bind(SOCK_PATH).unwrap()
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
            // .fold(tx, |tx, mut stream| {
            //     println!("fwd {:?}", stream);
            //     let mut buf = vec![0; 1];

            //     let keys = stream.read_to_end(&mut buf)
            //                 .map_err(|e| {println!("IO Error {:?}", e)})
            //                 .map(|_| ());
            //     // let keys = io::read_to_end(stream, buf)
            //     //             .map_err(|e| {println!("IO Error {:?}", e)});
            //     //             // .map(move |(reader, buf, bytes_read)| {
            //     //             //     ()
            //     //             // });
            //     // keys.wait().unwrap();
            //     println!("got keys {}", str::from_utf8(&buf).unwrap());
            //     // tx.send(keys).map_err(|e| panic!("Send error: {}", e))
            //     tx.send(0).map_err(|e| panic!("Send error: {}", e))
            // })
            // .into_future()
            // .and_then(move |(sock, _)| {
            //     println!("fwd {:?}", sock);
            //     tx.clone().send(sock.unwrap()).unwrap();
            //     tx.send_all().unwrap();
            //     Ok(())
            // })
            // .map(drop)
        
    });

    // let i = Interval::new_interval(Duration::from_millis(20))
    //     .map_err(|e| panic!("Interval error: {}", e));
    // let fut = rx.zip(i).for_each(move |stream| {
    // let fut = rx.for_each(move |keys| {
    //     println!("got {:?}", keys);
    //     Ok(())
    // }).map(|_| ()); // never perform here
    // rt.spawn(fut);
    // thread::spawn(|| {loop_inputs(rx)});
    // let rx = rx.map(|x| {
    //     println!("got {:?}", x);
    //     x
    // });

    // rt.block_on(rx.into_future());

    // thread::spawn(move || {
    //     while let Ok(Async::Ready(Some(stream))) = rx.wait() {
    //         println!("got {:?}", stream);
    //         // thread::spawn(move || {
    //         //     let mut rt = Runtime::new().unwrap();
    //         //     let (_, buf) = rt.block_on(io::read_to_end(stream, vec![])).unwrap();
    //         //     let s = match str::from_utf8(&buf) {
    //         //         Ok(v) => {
    //         //             println!("data {}", v);
    //         //             v
    //         //         },
    //         //         Err(_) => panic!("Invalid utf8 sequence from stream"),
    //         //     };
                
    //         // });
    //     };
    // });

    // rt.spawn({
    //     listener.poll_accept()
    //         .into_future()
    // });
    // while let Ok(Async::Ready((sock, _))) = listener.poll_accept() {
    //     println!("got {:?}", sock);
    // };


    println!("3");
    // let server = rt.block_on(rx).unwrap();
    rt.run().unwrap();
    // let (_, buf) = rt.block_on(io::read_to_end(server, vec![])).unwrap();
    
}
