use std::sync::mpsc::channel;
use std::sync::mpsc::TryRecvError;
use std::thread;
use std::{process::Command, time};
use axum::extract::connect_info::ConnectInfo;
use axum::{
    body::Bytes,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse
};
use axum_extra::TypedHeader;

use futures::StreamExt;
use std::net::SocketAddr;
use rand::prelude::*;

pub fn _apple_handler(
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
    ws.on_upgrade(move |socket| send_apple_packet(socket, addr))
}

pub async fn send_apple_packet(mut socket: WebSocket, who: SocketAddr) {
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
    

    let devices = vec![
        "27", "09", "02", "1e", "2b", "2d", "2f", "01", "06", "20", "c0",
    ];

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
        let mut rng = rand::rng();
        loop {
            let mut cancelled = false;
            for device in devices.clone() {
                /* Generate random data for the packet */
                let mut rnd_data1: Vec<char> = vec!['\0'; 3];
                rnd_data1.iter_mut().for_each(|x| *x = rng.sample(rand::distr::Alphanumeric) as char);
                let mut rnd_data2: Vec<char> = vec!['\0'; 3];
                rnd_data2.iter_mut().for_each(|x| *x = rng.sample(rand::distr::Alphanumeric) as char);

                println!("Advertising: {}", device);
                let mut bt_process = Command::new("sh")
                    .arg("-c")
                    .arg(std::format!(
                        "btmgmt add-adv -c -d 16FF4C000F05C1{}{}000010{} 1",
                        device, rnd_data1.into_iter().collect::<String>(), rnd_data2.into_iter().collect::<String>()
                    )).spawn().unwrap();
                    thread::sleep(time::Duration::from_millis(2000));
                    bt_process.wait().expect("Can't wait");
                    match rx.try_recv() {
                        Ok(_) | Err(TryRecvError::Disconnected) => {
                            println!("Terminating.");
                            cancelled = true;
                            break;
                        }
                        Err(TryRecvError::Empty) => {}
                    }
            }
            if cancelled {
                println!("Killing loop");
                let _raw_res = Command::new("sh")
                .arg("-c")
                .arg("pkill -9 btmgmt").spawn();
                break;
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
