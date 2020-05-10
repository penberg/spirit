//! # Spirit is an asynchronous I/O framework for Rust.

#![no_std]

extern crate io_uring;

use core::cell::RefCell;
use core::marker::PhantomData;
use core::mem;
use core::ptr;
use io_uring::opcode::{self, types};
use io_uring::{CompletionQueue, IoUring, SubmissionQueue, Submitter};

pub struct Future<'a, T> {
    phantom: PhantomData<&'a T>,
}

impl<T> Future<'_, T> {
    pub fn then<'a, F, V>(&self, f: F) -> Future<'a, V>
    where
        F: Fn(T) -> Future<'a, V>,
    {
        unimplemented!();
    }
}

pub fn ready<'a, T>() -> Future<'a, T> {
    unimplemented!("");
}

pub struct Data;

pub struct Endpoint {
    port: u16,
}

impl Endpoint {
    pub fn port(port: u16) -> Self {
        Self { port }
    }
}

type ReceiveFn = fn(&Connection, Data);

pub struct Connection {
    recv_fn: Option<ReceiveFn>,
}

impl Connection {
    pub fn start(&self) {}

    pub fn on_receive(&mut self, recv_fn: ReceiveFn) {
        self.recv_fn = Some(recv_fn);
    }

    pub fn send(&self, d: Data) {
        unimplemented!();
    }
}

pub struct Listener {
    endpoint: Endpoint,
}

impl Listener {
    pub fn new(endpoint: Endpoint) -> Self {
        Self { endpoint }
    }

    pub fn on_connection(&self, f: fn(Connection)) {
        f(Connection { recv_fn: None }) /* FIXME */
    }

    pub fn start(&self, proactor: &mut Proactor) {
        unsafe {
            let sockfd = libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0);
            if sockfd < 0 {
                unimplemented!();
            }
            let sockaddr = libc::sockaddr_in {
                sin_family: libc::AF_INET as u16,
                sin_port: self.endpoint.port.to_be(),
                ..mem::zeroed()
            };
            let saddr = mem::transmute(&sockaddr);
            let saddr_len = mem::size_of::<libc::sockaddr_in>() as u32;
            if (libc::bind(sockfd, saddr, saddr_len) < 0) {
                unimplemented!();
            }
            if (libc::listen(sockfd, 64) < 0) {
                unimplemented!();
            }
            proactor.async_accept(sockfd);
        }
    }
}

pub struct Spirit;

impl Spirit {
    fn new() -> Self {
        Spirit {}
    }
}

pub fn start() -> Spirit {
    Spirit::new()
}

pub fn listen(ep: Endpoint) -> Listener {
    Listener::new(ep)
}

pub struct Proactor {
    io_uring: IoUring,
}

impl Proactor {
    /// Accept a new connection asynchronously.
    pub fn async_accept(&mut self, fd: libc::c_int) {
        let entry =
            opcode::Accept::new(types::Target::Fd(fd), ptr::null_mut(), ptr::null_mut()).build();
        unsafe {
            self.io_uring.submission().available().push(entry);
        }
    }

    /// Create a new proactor instance.
    fn new() -> anyhow::Result<Proactor> {
        let mut ring = IoUring::new(256)?;
        Ok(Proactor { io_uring: ring })
    }

    /// Start the run loop.
    fn run_loop(&mut self) -> anyhow::Result<()> {
        loop {
            match self.io_uring.submitter().submit_and_wait(1) {
                Ok(_) => (),
                Err(ref err) if err.raw_os_error() == Some(libc::EINTR) => (),
                Err(err) => return Err(err.into()),
            }

            let mut cq = self.io_uring.completion();
            for cqe in cq.available() {
                let user_data = cqe.user_data();
            }
        }
        Ok(())
    }
}

pub fn run_loop<F>(app: F) -> anyhow::Result<()>
where
    F: FnOnce(&mut Proactor),
{
    let mut proactor = Proactor::new()?;

    app(&mut proactor);

    proactor.run_loop()
}
