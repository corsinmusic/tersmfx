use bincode;
use tokio::net::UnixStream;
use tokio::io::AsyncWriteExt;

use crate::daemon::daemon_action::DaemonAction;
use crate::daemon::DAEMON_SOCK_FILE_PATH;

pub fn play(command: &String) {
    let action = DaemonAction::Play(command.clone());
    send_action(action);
}

pub fn print_config() {
    let action = DaemonAction::PrintConfig;
    send_action(action);
}

fn send_action(action: DaemonAction) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        match UnixStream::connect(DAEMON_SOCK_FILE_PATH).await {
            Ok(mut stream) => {
                // Serialize the event
                let encoded_event = bincode::serialize(&action).unwrap_or_else(|e| {
                    eprintln!("Failed to serialize event: {}", e);
                    Vec::new()
                });

                if let Err(e) = stream.write_all(&encoded_event).await {
                    eprintln!("Failed to send event: {}", e);
                }
            }
            Err(_) => {
                // Couldn't connect to the daemon
                // Ignore the error
            }
        }
    });
}
