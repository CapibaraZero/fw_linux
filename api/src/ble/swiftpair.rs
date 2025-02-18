use axum::extract::connect_info::ConnectInfo;
use axum::{
    body::Bytes,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use axum_extra::TypedHeader;
use std::sync::mpsc::channel;
use std::sync::mpsc::TryRecvError;
use std::thread;
use std::{process::Command, time};

use futures::StreamExt;
use rand::prelude::*;
use std::net::SocketAddr;

pub fn _swiftpair_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    println!("`{user_agent}` at {addr} connected.");
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| send_swiftpair_packet(socket, addr))
}

use rand::distr::Alphanumeric;

pub async fn send_swiftpair_packet(mut socket: WebSocket, who: SocketAddr) {
    if socket
        .send(Message::Ping(Bytes::from_static(&[1, 2, 3])))
        .await
        .is_ok()
    {
        println!("Pinged {who}...");
    } else {
        println!("Could not send ping {who}!");
        return;
    }

    let (tx, rx) = channel();

    let (mut _sender, mut receiver) = socket.split();

    let mut recv_task = tokio::spawn(async move {
        loop {
            let msg = receiver.next().await.expect("error").expect("error2");
            if msg.to_text().expect("Can't parse to str") == "stop" {
                println!("Stopping clint");
                // cancelled = true;
                break;
            }
        }
    });

    thread::spawn(move || {
        let rng = rand::rng();
        // let mut cancelled = false;
        loop {
            let device_name: String = rng.clone()
                .sample_iter(&Alphanumeric)
                .take(5)
                .map(char::from)
                .collect();
            let mut bt_process = Command::new("btmgmt")
            .arg("add-adv")
            .arg("-c")
            .arg("-d")
            .arg(std::format!(
                "{}FF0600030080{}",
                device_name.len(),
                device_name
            ))
            .arg("1")
            .spawn().unwrap();
            bt_process.wait().expect("Can't wait");
            thread::sleep(time::Duration::from_millis(2000));
            match rx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    println!("Terminating.");
                    // cancelled = true;
                    break;
                }
                Err(TryRecvError::Empty) => {}
            }
        }
    });

    // If any one of the tasks exit, abort the other.
    tokio::select! {
        rv_b = (&mut recv_task) => {
            match rv_b {
                Ok(_b) => println!("Rec killed"),
                Err(b) => println!("Error receiving messages {b:?}")
            }
            let _ = tx.send(());
        }
    }
    // send_task.
    println!("Destroying socket");
}
