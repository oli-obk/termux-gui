use nix::cmsg_space;

use nix::sys::socket::{recv, recvmsg, send, MsgFlags, RecvMsg, SockaddrStorage};

use serde::Deserialize;
use serde_json::{json, Value};
use std::io::Read;
use std::io::Write;
use std::os::android::net::SocketAddrExt as _;
use std::os::fd::IntoRawFd;
use std::os::unix::io::RawFd;
use std::os::unix::net::{SocketAddr, UnixListener};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};

pub fn connect() -> (RawFd, RawFd) {
    static DISAMBIGUATE: AtomicUsize = AtomicUsize::new(0);
    let dis = DISAMBIGUATE.fetch_add(1, Ordering::Relaxed);
    let id = std::process::id();
    let main_addr = format!("rust/tgui/{id}/{dis}/main");
    let event_addr = format!("rust/tgui/{id}/{dis}/event");
    let main_sock_addr = SocketAddr::from_abstract_name(&main_addr).unwrap();
    let event_sock_addr = SocketAddr::from_abstract_name(&event_addr).unwrap();

    let main_sock = UnixListener::bind_addr(&main_sock_addr).unwrap();
    let event_sock = UnixListener::bind_addr(&event_sock_addr).unwrap();

    Command::new("am")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .args([
            "broadcast",
            "-n",
            "com.termux.gui/.GUIReceiver",
            "--es",
            "mainSocket",
            &main_addr,
            "--es",
            "eventSocket",
            &event_addr,
        ])
        .output()
        .unwrap();

    let (mut main, _addr) = main_sock.accept().unwrap();
    let (event, _addr) = event_sock.accept().unwrap();

    main.write_all(&[1]).unwrap();
    let mut res = [0u8];
    main.read_exact(&mut res).unwrap();
    assert_eq!(res, [0]);
    (main.into_raw_fd(), event.into_raw_fd())
}

pub fn transmit_buffer(fd: &RawFd, msg: &[u8]) {
    let mut len = msg.len();
    let mut start = 0;
    while len > 0 {
        let ret = send(*fd, &msg[start..], MsgFlags::empty()).unwrap();
        len = len.checked_sub(ret).expect("sent more bytes than we had");
        start += ret;
    }
}

pub fn recv_msg<T: for<'a> Deserialize<'a>>(fd: &RawFd) -> Result<T, serde_json::Error> {
    serde_json::from_slice(&inner_recv_msg(fd))
}

// Workaround for https://github.com/serde-rs/serde/issues/2200#issuecomment-2563562840
fn deser<T: for<'a> Deserialize<'a>>(bytes: &[u8]) -> Result<T, serde_json::Error> {
    let mut value = serde_json::from_slice::<serde_json::Map<String, Value>>(bytes)?;
    if let Some(inner) = value.remove("value") {
        let Value::Object(inner) = inner else {
            return Err(serde::de::Error::custom("value field is not a map"));
        };
        value.extend(inner);
    }

    serde_json::value::from_value(value.into())
}

pub fn try_recv_msg<T: for<'a> Deserialize<'a>>(fd: &RawFd) -> Result<T, serde_json::Error> {
    let msg = inner_recv_msg(fd);
    deser(&msg).map_err(|e| {
        eprintln!(
            "error parsing {} from {}",
            std::any::type_name::<T>(),
            std::str::from_utf8(&msg).unwrap(),
        );
        e
    })
}

fn inner_recv_msg(fd: &RawFd) -> Vec<u8> {
    let mut size = [0u8; 4];
    let mut rem = &mut size[..];

    while !rem.is_empty() {
        let ret = recv(*fd, &mut rem, MsgFlags::empty()).unwrap();
        rem = &mut rem[ret..];
    }

    let n = u32::from_be_bytes(size) as usize;

    let mut msg = [0u8; 1024 * 64];
    let mut rem = &mut msg[..n];
    while !rem.is_empty() {
        let ret = recv(*fd, &mut rem, MsgFlags::empty()).unwrap();
        rem = &mut rem[ret..];
    }
    msg[..n].to_vec()
}

pub fn recv_msg_fd(fd: &RawFd) -> (Value, u8) {
    let mut size = [0u8; 4];
    let mut rem = &mut size[..];

    while !rem.is_empty() {
        let ret = recv(*fd, &mut rem, MsgFlags::empty()).unwrap();
        rem = &mut rem[ret..];
    }

    let n = u32::from_be_bytes(size) as usize;

    let mut msg = [0u8; 1024 * 64];
    let mut fds = cmsg_space!([RawFd; 2]);

    let mut rem = &mut msg[..n];
    while !rem.is_empty() {
        let mut io_mut_buff = [std::io::IoSliceMut::new(rem)];
        let ret: RecvMsg<SockaddrStorage> =
            recvmsg(*fd, &mut io_mut_buff, Some(&mut fds), MsgFlags::empty()).unwrap();
        rem = &mut rem[ret.bytes..];
    }

    match serde_json::from_slice(&msg[..n]) {
        Ok(val) => (val, fds[0]),
        Err(_) => (json!(null), fds[0]),
    }
}

pub fn send_msg(fd: &RawFd, msg: Value) {
    let msg = msg.to_string();
    let msg_bytes = msg.as_bytes();
    let msg_len = u32::to_be_bytes(msg_bytes.len() as u32);

    transmit_buffer(fd, &msg_len);
    transmit_buffer(fd, &msg_bytes);
}

pub fn send_recv_msg<T: for<'a> Deserialize<'a>>(fd: &RawFd, msg: Value) -> T {
    send_msg(fd, msg);
    recv_msg(fd).unwrap()
}

pub fn construct_message(method: &str, args: &Value) -> Value {
    json!({"method": method, "params": args})
}
