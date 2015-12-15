extern crate broadcast;

use broadcast::BroadcastWriter;
use std::io::Write;

fn main() {
    let mut first = Vec::new();
    let mut second = Vec::new();
    {
        let mut broadcaster = BroadcastWriter::new(&mut first, &mut second);
        let _ = broadcaster.write(b"it's over 9000!");
    }
    println!("first -> {:?}", first);
    println!("second -> {:?}", second);
}
