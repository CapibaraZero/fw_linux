use axum::Json;
use rand::rngs::OsRng;
use rand::TryRngCore;
use serde::Deserialize;
use std::fs;
use std::process::Command;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct IRSignal {
    address: String,
    btn_name: String,
    command: String,
    protocol: String,
    frequency: u16,
    duty_cycle: u8,
    raw_data: String,
}

pub async fn ir_send(payload: Json<IRSignal>) {
    let ir_sender_device = std::env::var("IR_TRANSMITTER").expect("IR_TRANSMITTER must be set.");
    if payload.raw_data == "" {
        let result = Command::new("ir-ctl")
            .arg("-d")
            .arg(ir_sender_device)
            .arg("-S")
            .arg(std::format!(
                "{}:0x{}{}",
                payload.protocol,
                payload.address,
                payload.command
            ))
            .output()
            .expect("failed to execute process");
        println!("{}", String::from_utf8_lossy(&result.stdout));
        println!("{}", String::from_utf8_lossy(&result.stderr));
    } else {
        let random_id = OsRng.try_next_u32().expect("Can't gen"); // Use OsRng since is thread-safe
        let path_name = std::format!("/tmp/tag_{}.mfd", random_id);
        fs::write(path_name.as_str(), payload.raw_data.as_str()).expect("Can't write IR RAW data");

        let result = Command::new("ir-ctl")
            .arg("--mode2")
            .arg("-d")
            .arg(ir_sender_device)
            .arg("-c")
            .arg(payload.frequency.to_string())
            .arg("-D")
            .arg(payload.duty_cycle.to_string())
            .arg("--send")
            .arg(path_name)
            .output()
            .expect("failed to execute process");

        println!("{}", String::from_utf8_lossy(&result.stdout));
        println!("{}", String::from_utf8_lossy(&result.stderr));
    }
}

pub async fn ir_receive() -> String {
    let ir_receiver_device = std::env::var("IR_RECEIVER").expect("IR_RECEIVER must be set.");
    let result = Command::new("ir-ctl")
        .arg("--mode2")
        .arg("-d")
        .arg(ir_receiver_device)
        .arg("-r")
        .arg("-1")
        .output()
        .expect("failed to execute process");

    String::from_utf8_lossy(&result.stdout).into()
}
