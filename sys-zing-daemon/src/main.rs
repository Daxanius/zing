use log::{info, warn};
use std::fs;
use std::fs::Permissions;
use std::io::prelude::*;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::{UnixListener, UnixStream};
use sys_zing_daemon::MelodyPlayer;
use zing_protocol::Command;

fn decode_stream(mut stream: UnixStream) -> Result<Command, String> {
    let mut buf: Vec<u8> = Vec::new();
    let _ = stream.read_to_end(&mut buf).map_err(|e| e.to_string())?;
    Command::from_bytes(&buf).map_err(|e| e.to_string())
}

fn main() {
    // Create the logger
    env_logger::init();

    // Make sure leftover sockets are removed
    let _ = fs::remove_file(zing_protocol::SOCKET_PATH);
    let listener = UnixListener::bind(zing_protocol::SOCKET_PATH).expect("Failed to create socket");
    fs::set_permissions(zing_protocol::SOCKET_PATH, Permissions::from_mode(0o666))
        .expect("Failed to set socket permissions");

    let mut melody_player = MelodyPlayer::new();

    info!("Zing daemon running...");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => match decode_stream(stream) {
                Ok(command) => {
                    melody_player.handle_command(command);
                }
                Err(e) => {
                    warn!("Could not decode stream: {e}");
                }
            },
            Err(e) => warn!("Connection failed: {e}"),
        }
    }
}
