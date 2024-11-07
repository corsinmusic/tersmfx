pub mod daemon_action;

use bincode;
use std::fs::File;
use daemonize::Daemonize;
use tokio::io::AsyncReadExt;
use tokio::net::{UnixListener, UnixStream};
use sysinfo::{Signal, System, ProcessesToUpdate};
use rodio::{OutputStream, OutputStreamHandle};
use std::sync::Arc;

use crate::audio;
use crate::config::termsfx_config::TermsfxConfig;
use crate::config::termsfx_config_loader::TermsfxConfigLoader;

use daemon_action::DaemonAction;

const DAEMON_STDOUT_FILE_PATH: &str = "/tmp/termsfx_daemon.out";
const DAEMON_STDERR_FILE_PATH: &str = "/tmp/termsfx_daemon.err";
const DAEMON_PID_FILE_PATH: &str = "/tmp/termsfx_daemon.pid";
pub const DAEMON_SOCK_FILE_PATH: &str = "/tmp/termsfx_daemon.sock";

pub struct Daemon {
    config_loader: TermsfxConfigLoader,
    _stream: OutputStream,
    stream_handle: Arc<OutputStreamHandle>,
}

impl Daemon {
    pub fn start() {
        let stdout = File::create(DAEMON_STDOUT_FILE_PATH).unwrap_or_else(|e| {
            eprintln!("Failed to create stdout log file: {}", e);
            std::process::exit(1);
        });
        let stderr = File::create(DAEMON_STDERR_FILE_PATH).unwrap_or_else(|e| {
            eprintln!("Failed to create stderr log file: {}", e);
            std::process::exit(1);
        });

        let daemonize = Daemonize::new()
            .pid_file(DAEMON_PID_FILE_PATH)
            .stdout(stdout)
            .stderr(stderr);

        match daemonize.start() {
            Ok(_) => {
                let (_stream, stream_handle) = OutputStream::try_default().unwrap_or_else(|e| {
                    eprintln!("Failed to initialize audio output: {}", e);
                    std::process::exit(1);
                });

                let daemon = Daemon {
                    config_loader: TermsfxConfigLoader::new(),
                    _stream,
                    stream_handle: Arc::new(stream_handle),
                };

                // Place your daemon logic here
                daemon.run();
            }
            Err(e) => eprintln!("Error, daemonization failed: {}", e),
        }
    }

    pub fn stop() {
       match std::fs::read_to_string(DAEMON_PID_FILE_PATH) {
            Ok(pid_str) => {
                if let Ok(pid) = pid_str.trim().parse::<u32>() {
                    let daemon_pid = sysinfo::Pid::from_u32(pid);
                    let mut system = System::new_all();

                    system.refresh_processes(ProcessesToUpdate::Some(&[daemon_pid]), true);

                    if let Some(process) = system.process(daemon_pid) {
                        if process.kill_with(Signal::Term).unwrap_or(false) {
                            println!("Daemon stopped successfully");
                        } else {
                            eprintln!("Failed to stop daemon");
                        }
                    } else {
                        eprintln!("No process found with PID {}", pid);
                    }
                } else {
                    eprintln!("Invalid PID in {}", DAEMON_PID_FILE_PATH);
                }
            }
            Err(_) => eprintln!("Daemon is not running or PID file not found"),
        } 
    }

    pub fn restart() {
        Daemon::stop();
        Daemon::start();
    }

    pub fn status() {
        match std::fs::read_to_string(DAEMON_PID_FILE_PATH) {
            Ok(pid_str) => {
                if let Ok(pid) = pid_str.trim().parse::<u32>() {
                    let daemon_pid = sysinfo::Pid::from_u32(pid);
                    let mut system = System::new_all();

                    system.refresh_processes(ProcessesToUpdate::Some(&[daemon_pid]), true);

                    if system.process(daemon_pid).is_some() {
                        println!("Daemon is running with PID {}", pid);
                    } else {
                        println!("Daemon PID file found but process is not running");
                    }
                } else {
                    eprintln!("Invalid PID in /tmp/daemon.pid");
                }
            }
            Err(_) => println!("Daemon is not running"),
        }
    }

    pub fn run(&self) {
        // Load config
        self.config_loader.watch();

         // The daemon's main loop
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let _ = std::fs::remove_file(DAEMON_SOCK_FILE_PATH);

            let listener = match UnixListener::bind(DAEMON_SOCK_FILE_PATH) {
                Ok(listener) => listener,
                Err(e) => {
                    eprintln!("Failed to bind to socket: {}", e);
                    return;
                }
            };
            println!("Daemon is listening on {}", DAEMON_SOCK_FILE_PATH);

            loop {
                let config = self.config_loader.config();

                match listener.accept().await {
                    Ok((stream, _)) => {
                        let stream_handle_clone = Arc::clone(&self.stream_handle);
                        tokio::spawn(async move {
                            handle_client(stream, config, stream_handle_clone).await;
                        });
                    }
                    Err(e) => eprintln!("Failed to accept connection: {}", e),
                }

                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        });
    }
}

async fn handle_client(mut stream: UnixStream, config: TermsfxConfig, stream_handle: Arc<OutputStreamHandle>) {
    let mut buf = vec![0u8; 1024];

    match stream.read(&mut buf).await {
        Ok(size) if size > 0 => {
            let data = &buf[..size];
            match bincode::deserialize::<DaemonAction>(data) {
                Ok(event) => {
                    println!("Received event from client: {:?}", event);
                    // Handle the event
                    process_event(event, config, stream_handle).await;
                }
                Err(e) => eprintln!("Failed to deserialize event: {}", e),
            }
        }
        Ok(_) => println!("Client disconnected"),
        Err(e) => eprintln!("Failed to read from client: {}", e),
    }
}

async fn process_event(action: DaemonAction, config: TermsfxConfig, stream_handle: Arc<OutputStreamHandle>) {
    match action {
        DaemonAction::Play(command) => {
            for cmd in &config.commands {
                if cmd.command.is_match(&command) {
                    println!("Playing sound for '{}'", command);

                    if let Some(audio_file_path) = &cmd.audio_file_path {
                        audio::play_audio(&stream_handle, audio_file_path.to_str().unwrap());
                    } else if let Some(audio_file_paths) = &cmd.audio_file_paths {
                        for audio_file_path in audio_file_paths {
                            audio::play_audio(&stream_handle, audio_file_path.to_str().unwrap());
                        }
                    }
                }
            }
        }
        _ => println!("Unsupported event"),
    }
}

