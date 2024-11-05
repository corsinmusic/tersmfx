use std::sync::Arc;
use rodio::{Decoder, OutputStreamHandle, Source};

pub fn play_audio(stream_handle: &Arc<OutputStreamHandle>, file_path: &str) {
    // Load audio data
    let file = match std::fs::File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open audio file: {}", e);
            return;
        }
    };

    let source = match Decoder::new(std::io::BufReader::new(file)) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to decode audio: {}", e);
            return;
        }
    };

    // Play the sound
    if let Err(e) = stream_handle.play_raw(source.convert_samples()) {
        eprintln!("Failed to play audio: {}", e);
    }
}
