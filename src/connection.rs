use nix::cmsg_space;

use nix::sys::socket::{
    accept, bind, listen, recv, recvmsg, send, socket, AddressFamily, MsgFlags, RecvMsg, SockFlag,
    SockType, SockaddrStorage, UnixAddr,
};

use nix::errno::Errno;

use std::os::unix::io::RawFd;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};

use serde::Deserialize;
use serde_json::{json, Value};

pub fn connect() -> (RawFd, RawFd) {
    static DISAMBIGUATE: AtomicUsize = AtomicUsize::new(0);
    let dis = DISAMBIGUATE.fetch_add(1, Ordering::Relaxed);
    let id = std::process::id();
    let main_addr = format!("rust/tgui/{id}/{dis}/main");
    let event_addr = format!("rust/tgui/{id}/{dis}/event");
    let main_sock_addr = UnixAddr::new_abstract(&main_addr.as_bytes()).unwrap();
    let event_sock_addr = UnixAddr::new_abstract(&event_addr.as_bytes()).unwrap();

    let main_sock = socket(
        AddressFamily::Unix,
        SockType::Stream,
        SockFlag::empty(),
        None,
    )
    .unwrap();

    let event_sock = socket(
        AddressFamily::Unix,
        SockType::Stream,
        SockFlag::empty(),
        None,
    )
    .unwrap();

    //TODO: Handle Error
    for i in 0..=10 {
        if i == 10 {
            println!("Error Establishing connection with socket");
        }
        match bind(main_sock, &main_sock_addr) {
            Ok(_) => break,
            Err(err) => {
                if let Errno::EBADF | Errno::EINVAL | Errno::ENOTSOCK = err {
                    panic!("Failed creating main Socket");
                }
            }
        }
    }

    for i in 0..=10 {
        if i == 10 {
            println!("Error Establishing connection with socket");
        }

        match bind(event_sock, &event_sock_addr) {
            Ok(_) => break,
            Err(err) => {
                if let Errno::EBADF | Errno::EINVAL | Errno::ENOTSOCK = err {
                    panic!("Failed creating event Socket");
                }
            }
        }
    }

    listen(main_sock, 1).unwrap();
    listen(event_sock, 1).unwrap();

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

    let main = accept(main_sock).unwrap();
    let event = accept(event_sock).unwrap();

    let protocol = [1u8];
    while send(main, &protocol, MsgFlags::empty()).unwrap() == 0 {}
    let mut res = [0u8];
    while recv(main, &mut res, MsgFlags::empty()).unwrap() == 0 {}
    assert_eq!(res, [0]);
    (main, event)
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

pub fn recv_msg(fd: &RawFd) -> Value {
    serde_json::from_slice(&inner_recv_msg(fd)).unwrap_or(Value::Null)
}

// Workaround for https://github.com/serde-rs/serde/issues/2200#issuecomment-2563562840
fn deser<T: for<'a> Deserialize<'a>>(bytes: &[u8]) -> Result<T, serde_json::Error> {
    let mut value = serde_json::from_slice::<serde_json::Map<String, Value>>(bytes)?;
    let inner = value
        .remove("value")
        .ok_or_else(|| serde::de::Error::missing_field("value"))?;
    let Value::Object(inner) = inner else {
        return Err(serde::de::Error::custom("value field is not a map"));
    };
    value.extend(inner);

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
    let mut togo = 4usize;
    let mut start = 0;

    while togo > 0 {
        let ret = recv(*fd, &mut size[start..], MsgFlags::empty()).unwrap();
        togo = togo.checked_sub(ret).unwrap();
        start += ret;
    }

    let n = u32::from_be_bytes(size) as usize;
    togo = n;
    start = 0;

    let mut msg = [0u8; 1024 * 64];
    while togo > 0 {
        let ret = recv(*fd, &mut msg[start..n], MsgFlags::empty()).unwrap();
        togo = togo.checked_sub(ret).unwrap();
        start += ret;
    }

    msg.iter().map(|&v| v).filter(|&val| val != b'\0').collect()
}

pub fn recv_msg_fd(fd: &RawFd) -> (Value, u8) {
    let mut size = [0u8; 4];
    let mut togo = 4usize;
    let mut start = 0;

    while togo > 0 {
        let ret = recv(*fd, &mut size[start..], MsgFlags::empty()).unwrap();
        togo = togo.saturating_sub(ret);
        start += ret;
    }

    togo = u32::from_be_bytes(size) as usize;
    let mut msg = [0u8; 1024 * 64];
    let mut fds = cmsg_space!([RawFd; 2]);
    while togo > 0 {
        let mut io_mut_buff = [std::io::IoSliceMut::new(&mut msg[start..])];
        let ret: RecvMsg<SockaddrStorage> =
            recvmsg(*fd, &mut io_mut_buff, Some(&mut fds), MsgFlags::empty()).unwrap();
        let ret = ret.bytes;
        togo = togo.saturating_sub(ret);
        start += ret;
    }

    let msg: Vec<u8> = msg.iter().map(|&v| v).filter(|&val| val != b'\0').collect();
    match serde_json::from_slice(&msg) {
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

pub fn send_recv_msg(fd: &RawFd, msg: Value) -> Value {
    send_msg(fd, msg);
    recv_msg(fd)
}

pub fn construct_message(method: &str, args: &Value) -> Value {
    json!({"method": method, "params": args})
}
