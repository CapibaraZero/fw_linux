use axum::extract::connect_info::ConnectInfo;
use axum::{
    body::Bytes,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use axum_extra::TypedHeader;

use bollard::container::{Config, RemoveContainerOptions};
use bollard::models::HostConfig;
use bollard::secret::PortBinding;
use bollard::Docker;
use futures::{SinkExt, StreamExt};
use pnet::datalink;
use pnet::datalink::NetworkInterface;
use std::net::SocketAddr;

use bollard::exec::{CreateExecOptions, StartExecOptions, StartExecResults};
use std::collections::HashMap;

pub fn _bettercap_handler(
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
    ws.on_upgrade(move |socket| start_bettercap(socket, addr))
}

async fn start_bettercap(mut socket: WebSocket, who: SocketAddr) {
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

    let interfaces: Vec<NetworkInterface> = datalink::interfaces();
    let wifi_interface: NetworkInterface = interfaces
        .iter()
        .map(|v| v.clone())
        .filter(|sen| sen.name == interface.to_text().unwrap())
        .collect::<Vec<NetworkInterface>>()[0]
        .clone();
    // let (tx, rx) = channel();
    let ip = wifi_interface.ips[0].ip();
    println!("{}", ip);
    let (mut _sender, mut receiver) = socket.split();

    let mut recv_task = tokio::spawn(async move {
        loop {
            let msg = receiver.next().await.expect("Connection dropped").expect("Connection dropped");
            if msg.to_text().expect("Can't parse to str") == "stop" {
                println!("Stopping clint");
                // bettercap_process.unwrap().kill().expect("Can't stop bettercap");
                // let _raw_res = Command::new("sh").arg("-c").arg("pkill -9 pig.py").spawn();
                // cancelled = true;
                break;
            }
        }
    });

    // println!("Start");
    let docker = Docker::connect_with_socket_defaults().unwrap();
    let host_config = HostConfig {
        privileged: Some(true),
        network_mode: Some("host".to_string()),
        port_bindings: Some(HashMap::from([
            (
                "8080".to_string(),
                Some(vec![PortBinding {
                    host_ip: Some("127.0.0.1".to_string()),
                    host_port: Some(String::from("8080")),
                }]),
            ),
            (
                "8081".to_string(),
                Some(vec![PortBinding {
                    host_ip: Some("127.0.0.1".to_string()),
                    host_port: Some(String::from("8081")),
                }]),
            ),
        ])),
        ..Default::default()
    };
    let alpine_config = Config {
        image: Some("bettercap/bettercap"),
        tty: Some(false),
        exposed_ports: Some(HashMap::from([
            ("8080/tcp", HashMap::new()),
            ("8081:8081", HashMap::new()),
        ])),
        // cmd: Some(vec!["/app/bettercap"]),
        host_config: Some(host_config),
        ..Default::default()
    };

    let id = docker
        .create_container::<&str, &str>(None, alpine_config)
        .await
        .unwrap()
        .id;

    let coroutine_id = id.clone();

    docker.start_container::<String>(&id, None).await.unwrap();

    let coroutine_docker = docker.clone();
    let mut docker_task = tokio::spawn(async move {
        let exec = coroutine_docker
            .create_exec(
                &coroutine_id,
                CreateExecOptions {
                    attach_stdout: Some(true),
                    attach_stderr: Some(true),
                    attach_stdin: Some(true),
                    tty: Some(true),
                    cmd: Some(vec![
                        "/app/bettercap",
                        "--eval",
                        std::format!("set ui.address {}; set api.rest.address {}; ui on'", ip, ip)
                            .as_str(),
                    ]),
                    privileged: Some(true),
                    // env: Some(vec![])
                    ..Default::default()
                },
            )
            .await
            .unwrap()
            .id;

        if let StartExecResults::Attached {
            mut output,
            input: _,
        } = coroutine_docker
            .start_exec(
                &exec,
                Some(StartExecOptions {
                    // tty: true,
                    ..Default::default()
                }),
            )
            .await
            .unwrap()
        {
            // // set stdout in raw mode so we can do tty stuff
            // let stdout = stdout();
            // let mut stdout = stdout.lock().into_raw_mode().unwrap();

            // pipe docker exec output into stdout
            while let Some(Ok(output)) = output.next().await {
                // stdout.write_all(output.into_bytes().as_ref()).expect("Can' write to websocket");
                // stdout.flush()?;
                _sender
                    .send(Message::text(output.to_string()))
                    .await
                    .expect("Can't send command");
            }
        }
    });
    tokio::select! {
        rv_b = (&mut recv_task) => {
            match rv_b {
                Ok(_b) => println!("Rec killed"),
                Err(b) => println!("Error receiving messages {b:?}")
            }
            // let _ = tx.send(());
        }
        rv_a = (&mut docker_task) => {
            match rv_a {
                Ok(_b) => println!("Docker killed"),
                Err(b) => println!("Error receiving messages {b:?}")
            }
            // let _ = tx.send(());
        }
    }
    docker_task.abort();
    recv_task.abort();

    docker
        .remove_container(
            &id,
            Some(RemoveContainerOptions {
                force: true,
                ..Default::default()
            }),
        )
        .await.expect("Can't terminate container");
    // send_task.
    println!("Destroying socket");
}
