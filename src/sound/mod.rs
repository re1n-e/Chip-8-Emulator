extern crate rodio;

use std::time::Duration;
use rodio::{OutputStream, Sink, source::SineWave, Source};
use rodio::source::TakeDuration;

// Sound function: Generates a beep sound at the specified frequency and duration
pub fn beep(frequency: u32, duration_ms: u64) -> Result<(), std::io::Error> {
    // Create an audio output stream
    let (_stream, stream_handle) = OutputStream::try_default()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    let sink = Sink::try_new(&stream_handle)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    // Create a sine wave at the specified frequency
    let source: TakeDuration<SineWave> = SineWave::new(frequency).take_duration(Duration::from_millis(duration_ms));
    sink.append(source);

    // Block until the sound has finished playing
    sink.sleep_until_end();
    Ok(())
}