extern crate vsock;
use vsock::{VsockAddr};
use std::io::Write;

fn main() {
    let cid = vsock::VMADDR_CID_ANY;
    let port = 12345; // Replace with your desired port number
    let addr = VsockAddr::new(cid,port);
    let listener = vsock::VsockListener::bind(&addr).expect("Failed to bind vsock listener");

    println!("Listening on vsock port: {}", port);

    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                println!("Connection established!");
                let response = b"Hello from Nitro Enclave!";
                s.write(response).expect("Failed to write to vsock");
            },
            Err(e) => {
                println!("Connection failed: {:?}", e);
            }
        }
    }
}

