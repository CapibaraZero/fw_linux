use axum::extract::connect_info::ConnectInfo;
use axum::{
    body::Bytes,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use axum_extra::TypedHeader;

use futures::StreamExt;
use std::net::SocketAddr;
use std::process::Command;


pub fn _beacon_spam_handler(
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
    ws.on_upgrade(move |socket| start_beacon_spam(socket, addr))
}

pub async fn start_beacon_spam(mut socket: WebSocket, who: SocketAddr) {
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

    let mut mdk3_process = Command::new("sudo")
        .arg("mdk3")
        .arg(interface.to_text().unwrap())
        .arg("b")
        .arg("-c")
        .arg("2")
        .arg("-s")
        .arg("10000")
        .spawn()
        .unwrap();
    let (mut _sender, mut receiver) = socket.split();

    let mut recv_task = tokio::spawn(async move {
        loop {
            let msg = receiver.next().await.expect("error").expect("error2");
            if msg.to_text().expect("Can't parse to str") == "stop" {
                println!("Stopping clint");
                mdk3_process.kill().expect("Can't kill aireplay");
                let _raw_res = Command::new("sudo").arg("mdk3").arg("-9").arg("mdk3").output().unwrap();
                break;
            }
        }
    });

    // let mut docker_task = tokio::spawn(async move {
    //     let data: Result<Vec<_>, _> = aireplay_process.stdout.iter().clone().collect();
    //    sender.send(Message::binary(data.unwrap()));
    // });
    println!("Start");

    tokio::select! {
        rv_b = (&mut recv_task) => {
            match rv_b {
                Ok(_b) => println!("Rec killed"),
                Err(b) => println!("Error receiving messages {b:?}")
            }
            // let _ = tx.send(());
        }
    }
    // _raw_res.kill().expect("Can't kill yersinia");
    // send_task.
    println!("Destroying socket");
}