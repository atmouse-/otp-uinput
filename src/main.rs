#[macro_use]
extern crate futures;
extern crate tokio;
extern crate tokio_uds;
extern crate bytes;
extern crate uinput;
extern crate libc;

mod input;
mod mouse;

use std::thread;
use std::fs;
use std::str;
use std::time::Duration;
use tokio::timer::Interval;
use std::io::{Read};
use std::sync::{Arc, Mutex};
// use std::sync::mpsc;
use std::str::FromStr;

use structopt::StructOpt;
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
    mouse: mouse::Uinput,
}

impl Shared {
    fn new(mouse_max: Option<(i32, i32)>) -> Self {
        Shared {
            input: input::Uinput::new(),
            mouse: mouse::Uinput::new(mouse_max),
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

            match String::from_utf8_lossy(&name).to_string().as_str() {
                "aaaa" => {
                    if keys.rd.len() != 1 {
                        println!("unexpect key length");
                        return Either::B(future::ok(()));
                    }
                    let first_num = keys.rd[0];
                    let mut k = state.lock().unwrap();
                    k.input.keyin(first_num);
                },
                "posrel" => {
                    // TODO: fix x, y values unwrap
                    let entry = String::from_utf8_lossy(&keys.rd).to_string();
                    let mut split = entry.split(",");
                    let _x = split.next().unwrap();
                    let _y = split.next().unwrap();
                    let x: i32 = i32::from_str(_x).unwrap();
                    let y: i32 = i32::from_str(_y).unwrap();
                    let mut k = state.lock().unwrap();
                    k.mouse.mouse_pos(x, y);
                },
                "posabs" => {
                    // TODO: fix x, y values unwrap
                    let entry = String::from_utf8_lossy(&keys.rd).to_string();
                    let mut split = entry.split(",");
                    let _x = split.next().unwrap();
                    let _y = split.next().unwrap();
                    let x: i32 = i32::from_str(_x).unwrap();
                    let y: i32 = i32::from_str(_y).unwrap();
                    let mut k = state.lock().unwrap();
                    k.mouse.mouse_abs(x, y);
                },
                "mouseclick" => {
                    let mut k = state.lock().unwrap();
                    k.mouse.mouse_click();
                },
                _ => {

                }

            }

            Either::B(future::ok(()))
        })
        .map_err(|e| {
            println!("connection error = {:?}", e);
        });

    tokio::spawn(connection);
}

#[derive(Debug, StructOpt)]
#[structopt(name = "otp-uinput", about = "otp-uinput usage.")]
struct Opt {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long)]
    debug: bool,

    /// Max value for uinput ABS_X event
    #[structopt(short = "x", long = "abs-x-max")]
    abs_x_max: Option<u16>,

    /// Max value for uinput ABS_Y event
    #[structopt(short = "y", long = "abs-y-max")]
    abs_y_max: Option<u16>,
}

fn main() {
    let mut rt = Runtime::new().unwrap();

    let opt = Opt::from_args();

    let sock_path = get_user_sock_path();
    let listener = match UnixListener::bind(&sock_path) {
        Ok(m) => m,
        Err(_) => {
            fs::remove_file(&sock_path).unwrap();
            UnixListener::bind(&sock_path).unwrap()
        }
    };
    let (tx, rx) = mpsc::channel::<u8>(1024);
    let state = Arc::new(Mutex::new(Shared::new(
        match opt.abs_x_max {
            Some(abs_x_max) => {Some((
                abs_x_max as i32,
                opt.abs_y_max.unwrap_or(0) as i32
            ))},
            None => None
        }
    )));
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
