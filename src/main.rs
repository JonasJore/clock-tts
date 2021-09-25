use tts_rust::text_speech;
use chrono;
mod clock_time_algorithm;
mod util;
mod formatters;

fn main() {
    let clock_time: String = chrono::Local::now().format("%H:%M").to_string();
    text_speech(
        clock_time_algorithm::clock_time_algorithm(&clock_time).as_str()
    );
}
