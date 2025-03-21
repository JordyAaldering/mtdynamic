use std::os::unix::net::{UnixListener, UnixStream};
use std::io::{self, Read, Write};
use std::{fs, mem};

#[cfg(feature = "corridor")]
use controller::CorridorController as Controller;
#[cfg(feature = "delta")]
use controller::DeltaController as Controller;

use letterbox::*;

fn handle_client(mut stream: UnixStream) -> std::io::Result<()> {
    let mut letterbox: Letterbox<Controller, 20> = Letterbox::default();

    let mut buffer = [0u8; mem::size_of::<Sample>()];

    loop {
        // Try to read from the stream
        match stream.read_exact(&mut buffer) {
            Ok(()) => {
                let incoming = Sample::from(buffer);
                println!("Recv: {:?}", incoming);

                // Update letterbox
                let outgoing = letterbox.update(incoming);

                // Write to stream
                println!("Send: {:?}", outgoing);
                let buf = outgoing.to_bytes();
                stream.write_all(&buf)?;
            }
            Err(e) => {
                eprintln!("Client disconnected: {}", e);
                break;
            }
        }
    }
    Ok(())
}


fn main() -> io::Result<()> {
    // Remove any existing socket file
    if fs::metadata(MTD_LETTERBOX_PATH).is_ok() {
        fs::remove_file(MTD_LETTERBOX_PATH)?;
    }

    // Create a listener
    let listener = UnixListener::bind(MTD_LETTERBOX_PATH)?;
    println!("Server listening on {}", MTD_LETTERBOX_PATH);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || {
                    handle_client(stream)
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    unreachable!()
}
