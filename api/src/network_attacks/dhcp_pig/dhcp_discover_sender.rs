extern crate pnet;

use std::net::Ipv4Addr;

use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel;
use pnet::packet::ethernet::MutableEthernetPacket;
use pnet::packet::ip::IpNextHeaderProtocol;
use pnet::packet::ipv4::MutableIpv4Packet;
use pnet::util::MacAddr;
use pnet::packet::ethernet::EtherTypes;
use pnet::packet::{MutablePacket, Packet};
use pnet::packet::udp::MutableUdpPacket;
use pnet::packet::dhcp::{DhcpHardwareType, DhcpOperation, MutableDhcpPacket};
use pnet::packet::udp::ipv4_checksum;
use pnet::packet::ipv4;
use rand;

pub fn send_dhcp_discover(interface: NetworkInterface) {
    let(mut tx, _) = match datalink::channel(&interface, Default::default()) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unknown channel type"),
        Err(e) => panic!("Error happened {}", e),
    };

    // TODO: Calculate checksum
    
    let mut ethernet_buffer = [0u8; 286];
    let mut ethernet_packet = MutableEthernetPacket::new(&mut ethernet_buffer).unwrap();

    let mut ipv4_packet_buffer = [0u8; 272];
    let mut ipv4_packet: MutableIpv4Packet = MutableIpv4Packet::new(&mut ipv4_packet_buffer).unwrap();

    let mut udp_packet_buffer = [0u8; 252];
    let mut udp_packet: MutableUdpPacket = MutableUdpPacket::new(&mut udp_packet_buffer).unwrap();
    
    let mut dhcp_packet_buffer = [0u8; 244];
    let mut dhcp_packet: MutableDhcpPacket = MutableDhcpPacket::new(&mut dhcp_packet_buffer).unwrap();

    let random_bytes: [u8; 6] = rand::random();
    let random_mac_addr = MacAddr::new(random_bytes[0], random_bytes[1], random_bytes[2], random_bytes[3], random_bytes[4], random_bytes[5]);

    ethernet_packet.set_destination(MacAddr::broadcast());
    ethernet_packet.set_source(random_mac_addr);
    ethernet_packet.set_ethertype(EtherTypes::Ipv4);
    
    ipv4_packet.set_version(4);
    ipv4_packet.set_header_length(5);

    let ipv4_source = Ipv4Addr::new(0, 0, 0, 0);
    let ipv4_dst = Ipv4Addr::BROADCAST;

    ipv4_packet.set_source(ipv4_source);
    ipv4_packet.set_destination(ipv4_dst);
    ipv4_packet.set_dscp(4);
    ipv4_packet.set_ecn(0);
    ipv4_packet.set_total_length(272);
    ipv4_packet.set_identification(0);
    ipv4_packet.set_fragment_offset(0);
    ipv4_packet.set_ttl(16);
    ipv4_packet.set_next_level_protocol(IpNextHeaderProtocol::new(17));

    let checksum = ipv4::checksum(&ipv4_packet.to_immutable());
    ipv4_packet.set_checksum(checksum);

    udp_packet.set_source(68);
    udp_packet.set_destination(67);
    udp_packet.set_length(252);

    dhcp_packet.set_op(DhcpOperation::new(1));
    dhcp_packet.set_htype(DhcpHardwareType::new(1));
    dhcp_packet.set_hlen(6);
    dhcp_packet.set_hops(0);
    dhcp_packet.set_xid(0x1e7ff521);
    dhcp_packet.set_secs(0);
    dhcp_packet.set_flags(0x8000);
    dhcp_packet.set_ciaddr(Ipv4Addr::new(0, 0, 0, 0));
    dhcp_packet.set_yiaddr(Ipv4Addr::new(0, 0, 0, 0));
    dhcp_packet.set_siaddr(Ipv4Addr::new(0, 0, 0, 0));
    dhcp_packet.set_giaddr(Ipv4Addr::new(0, 0, 0, 0));
    dhcp_packet.set_chaddr(random_mac_addr);

    let dhcp_buf = dhcp_packet.packet_mut();

    // Pad of zeros positions 34 to 235
    for i in 34..235 {
        dhcp_buf[i] = 0;
    }

    // DHCP Magic cookie
    dhcp_buf[236] = 0x63;
    dhcp_buf[237] = 0x82;
    dhcp_buf[238] = 0x53;
    dhcp_buf[239] = 0x63;
    
    // Option 53(discover)
    dhcp_buf[240] = 0x35;
    dhcp_buf[241] = 0x01;
    dhcp_buf[242] = 0x01;

    // Option 255(end)
    dhcp_buf[243] = 0xff;

    udp_packet.set_payload(dhcp_packet.packet_mut());

    let um_checksum = ipv4_checksum(&udp_packet.to_immutable(), &ipv4_source, &ipv4_dst);
    udp_packet.set_checksum(um_checksum);

    ipv4_packet.set_payload(udp_packet.packet_mut());
    ethernet_packet.set_payload(ipv4_packet.packet_mut());

    tx.send_to(&ethernet_packet.packet(), Some(interface));
}