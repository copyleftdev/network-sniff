extern crate pnet;

use pnet::datalink::{self, Channel::Ethernet};
use pnet::packet::{Packet, ethernet::EthernetPacket};

fn main() {
    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter().find(|iface| iface.is_up() && !iface.ips.is_empty() && iface.is_broadcast()).unwrap();
    let (_, mut rx) = datalink::channel(&interface, Default::default()).unwrap();

    loop {
        if let Ok(packet) = rx.next() {
            let ethernet_packet = EthernetPacket::new(packet).unwrap();
            println!("{:?}", ethernet_packet);
        }
    }
}
