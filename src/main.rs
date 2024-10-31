use rand::{thread_rng, Rng};
use std::io::{Cursor, Write};
use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to initialize connexion.");
    socket
        .connect("127.0.0.1:53")
        .expect("Failed to connect to the DNS server."); // change the IP here

    let mut query_buffer = [0; 512];
    let mut query_cursor = Cursor::new(&mut query_buffer[..]);

    // Header
    let mut rng = thread_rng();
    // id
    let id: u16 = rng.gen();
    query_cursor.write_all(&id.to_be_bytes()).unwrap();
    // QR|   Opcode  |AA|TC|RD|RA|   Z    |   RCODE
    let mut l2: u16 = 0;
    l2 |= 1 << 8;
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
    query_cursor.write_all(&[3]).unwrap();
    query_cursor.write_all(b"com\0").unwrap();
    let qname: u16 = 1;
    query_cursor.write_all(&qname.to_be_bytes()).unwrap();
    let qtype: u16 = 1;
    query_cursor.write_all(&qtype.to_be_bytes()).unwrap();

    let len: usize = query_cursor.position() as usize;
    socket
        .send(&query_buffer[..len])
        .expect("Failed to send query.");
}

#[repr(u16)]
enum QR {
    Query = 0,
    Response = 1,
}

#[repr(u16)]
enum Opcode {
    StandardQuery = 0,
    InverseQuery = 1,
    ServerStatus = 2,
}

#[repr(u16)]
enum RCode {
    NoError = 0,
    FormatError = 1,
    ServerFailure = 2,
    NameError = 3,
    NotImplemented = 4,
    Refused = 5,
}

#[repr(u16)]
enum Type {
    A = 1,
    NS = 2,
    MD = 3,
    MF = 4,
    CNAME = 5,
    SOA = 6,
    MB = 7,
    MG = 8,
    MR = 9,
    NULL = 10,
    WKS = 11,
    PTR = 12,
    HINFO = 13,
    MINFO = 14,
    MX = 15,
    TXT = 16,
}

#[repr(u16)]
enum QType {
    Base(Type),
    AXFR = 252,
    MAILB = 253,
    MAILA = 254,
    ALL = 255,
}

#[repr(u16)]
enum Class {
    IN = 1,
    CS = 2,
    CH = 3,
    HS = 4,
}

#[repr(u16)]
enum QClass {
    ANY = 255,
}

struct DNSQuery {
    qname: Vec<String>,
    qtype: QType,
    qclass: QClass,
}

struct DNSRessource {
    name: String,
    ressource_type: Type,
    class: Class,
    ttl: u32,
    rdlengh: u16,
    rdata: [u16; 100],
}
struct DNSMessage {
    id: u16,
    qr: QR,
    opcode: Opcode,
    authoritative_answer: bool,
    truncation: bool,
    recursion_desired: bool,
    recursion_available: bool,
    rcode: RCode,
    questions: Vec<DNSQuery>,
    answers: Vec<DNSRessource>,
    authorities: Vec<DNSRessource>,
    additionals: Vec<DNSRessource>,
}
