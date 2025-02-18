use axum::extract::connect_info::ConnectInfo;
use axum::{
    body::Bytes,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use axum_extra::TypedHeader;

use futures::{SinkExt, StreamExt};
use rand::rngs::OsRng;
use rand::TryRngCore;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::net::SocketAddr;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use std::sync::mpsc::{channel, TryRecvError};

pub fn _wps_bruteforce_handler(
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
    ws.on_upgrade(move |socket| start_wps_bruteforce(socket, addr))
}

#[derive(Serialize, Deserialize)]
pub struct WiFiDosParams {
    interface: String,
    bssid: String,
    wps_pixiedust: bool
}

pub async fn start_wps_bruteforce(mut socket: WebSocket, who: SocketAddr) {
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

    socket.send(Message::text("connected")).await.unwrap();

    let mut interface: Message = socket.recv().await.unwrap().unwrap();
    while interface.to_text().unwrap().len() == 3 {
        interface = socket.recv().await.unwrap().unwrap();
    }
    println!("Interface: {}", interface.to_text().unwrap());
    let params: WiFiDosParams =
        serde_json::from_str(interface.to_text().unwrap()).expect("Can't parse params");

    let (tx, rx) = channel::<()>();
    let (mut sender, mut receiver) = socket.split();

    let mut recv_task = tokio::spawn(async move {
        loop {
            let msg = receiver.next().await.expect("error").expect("error2");
            if msg.to_text().expect("Can't parse to str") == "stop" {
                println!("Stopping clint");
                // aireplay_process.kill().expect("Can't kill aireplay");
                let _raw_res = Command::new("sh")
                    .arg("-c")
                    .arg("pkill -9 reaver")
                    .output()
                    .expect("Can't kill reaver");
                break;
            }
        }
    });

    let random_id = OsRng.try_next_u32().expect("Can't gen"); // Use OsRng since is thread-safe
    let path_name = std::format!("/tmp/reaver_{}.txt", random_id);

    // Using as_str() to avoid clone
    let file = File::create(path_name.as_str()).expect("Can't create reaver buffer");

    let wps = if params.wps_pixiedust { "-K 1" } else { "" };

    let mut send_task = tokio::spawn(async move {
        let mut reaver_process = Command::new("sh")
            .arg("-c")
            .arg(std::format!(
                "reaver -i {} -b {} -vvv {}",
                params.interface,
                params.bssid,
                wps
            ))
            .stdout(Stdio::from(file))
            .spawn()
            .expect("Can't create process");
        let mut previous: String = String::new();
        loop {
            let res = fs::read_to_string(path_name.clone())
                .expect("Should have been able to read the file");
            if res != previous {
                sender
                    .send(Message::Text(res.as_str().into()))
                    .await
                    .expect("Can't send stdout to socket");
                previous = res;
            }
            match rx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    println!("Terminating.");
                    reaver_process.kill().expect("Can't kill reaver");
                    // cancelled = true;
                    break;
                }
                Err(TryRecvError::Empty) => {}
            }
            thread::sleep(Duration::from_millis(2000));
        }
    });
    println!("Start");

    tokio::select! {
        rv_a = (&mut send_task) => {
            match rv_a {
                Ok(_a) => println!("Send killed"),
                Err(a) => println!("Error sending messages {a:?}")
            }
            recv_task.abort();
        },
        rv_b = (&mut recv_task) => {
            match rv_b {
                Ok(_b) => println!("Rec killed"),
                Err(b) => println!("Error receiving messages {b:?}")
            }
            let _ = tx.send(());
        }
    }

    println!("Destroying socket");
}
