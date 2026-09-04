use std::fs::File;
use std::io::BufReader;
//use std::io::Sink;
//use std::time::Duration;
//use std::thread;

fn main() {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();

    let file = File::open("sample.opus").unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

    sink.append(source);
    println!("Playing sound..");
    sink.sleep_until_end();

}

