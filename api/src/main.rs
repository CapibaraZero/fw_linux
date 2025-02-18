use axum::{routing::get, routing::post, Router};
use tower_http::cors::CorsLayer;

mod ble;
mod network_attacks;
mod wifi;
mod nfc;
mod ir;

use std::env;

use axum::routing::any;

use std::{net::SocketAddr, path::PathBuf};
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};
use axum::extract::DefaultBodyLimit;
use tower_http::limit::RequestBodyLimitLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok().expect("Can't load env variable");
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

    // build our application with some routes
    let app = Router::new()
        .fallback_service(ServeDir::new(assets_dir).append_index_html_on_directories(true))
        // WiFi Section
        .route("/wifi_scan", get(wifi::scan_wifi))
        .route("/wifi_connect", post(wifi::connect_wifi))
        .route("/wifi_ap", post(wifi::create_ap))
        .route("/stop_ap", get(wifi::stop_evilportal))
        .route("/wifi_dos", any(wifi::wifi_dos_handler))
        .route("/beacon_spam", any(wifi::beacon_spam_handler))
        .route("/wps_bruteforce", any(wifi::wps_bruteforce_handler))
        .route("/read_evilportal_log", get(wifi::get_evilportal_log))
        .route("/enable_monitor_mode", get(wifi::enable_monitor_mode))
        .route("/disable_monitor_mode", get(wifi::disable_monitor_mode))

        // BLE Section
        .route("/fastpair", any(ble::fastpair_handler))
        .route("/applejuice", any(ble::apple_handler))
        .route("/swiftpair", any(ble::swiftpair_handler))
        .route("/ble_scan", any(ble::ble_scan_handler))

        // Network attacks
        .route("/send_arp_packet", post(network_attacks::send_arp_packet))
        .route("/get_interfaces", get(network_attacks::get_interfaces))
        .route("/arp_scan", get(network_attacks::arp_scan))
        .route("/dhcp_starvation", any(network_attacks::dhcp_starvation_handler))
        .route("/get_routes", get(network_attacks::get_routes))
        .route("/launch_bettercap", any(network_attacks::bettercap_handler))
        .route("/sniffer", any(network_attacks::sniffer_handler))

        // NFC
        .route("/nfc_poll", get(nfc::nfc_poll))
        .route("/nfc_read", get(nfc::nfc_read))
        .route("/nfc_write", post(nfc::nfc_write_data))
        .route("/nfc_nested_attack", any(nfc::nested_attack_handler))

        // IR
        .route("/ir_send", post(ir::ir_send))
        .route("/ir_receive", get(ir::ir_receive))
        .layer(CorsLayer::permissive())
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(
            250 * 1024 * 1024, /* 250mb */
        ))
        // logging so we can see whats going on
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

    // run it with hyper
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}