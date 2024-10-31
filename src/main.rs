use rand::{thread_rng, Rng};
use std::io::{Cursor, Write};
use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to initialize connexion.");
    socket.connect("127.0.0.1:53").expect("Failed to connect to the DNS server."); // change the IP here

    let mut query_buffer = [0; 512];
    let mut query_cursor = Cursor::new( &mut query_buffer[..]);

    // Header
    let mut rng = thread_rng();
    // id
    let id: u16 = rng.gen();
    query_cursor.write_all(&id.to_be_bytes()).unwrap();
    // QR|   Opcode  |AA|TC|RD|RA|   Z    |   RCODE
    let mut l2: u16 = 0;
    l2 |= (1 << 8);
    query_cursor.write_all(&l2.to_be_bytes()).unwrap();
    // QDCOUNT
    let qdcount: u16 = 1;
    query_cursor.write_all(&qdcount.to_be_bytes()).unwrap();
    // ANCOUNT
    let ancount: u16 = 0;
    query_cursor.write_all(&ancount.to_be_bytes()).unwrap();
    // NSCOUNT
    let nscount: u16 = 0;
    query_cursor.write_all(&nscount.to_be_bytes()).unwrap();
    // ARCOUNT
    let arcount: u16 = 0;
    query_cursor.write_all(&arcount.to_be_bytes()).unwrap();

    // Query
    query_cursor.write_all(&[6]).unwrap();
    query_cursor.write_all(b"google").unwrap();
    query_cursor.write_all(&[3]);
    query_cursor.write_all(b"com\0");
    let qname: u16 = 1;
    query_cursor.write_all(&qname.to_be_bytes()).unwrap();
    let qtype: u16 = 1;
    query_cursor.write_all(&qtype.to_be_bytes()).unwrap();

    let len: usize = query_cursor.position() as usize;
    socket.send(&query_buffer[..len]).expect("Failed to send query.");
}
