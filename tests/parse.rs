extern crate miniseed;

use miniseed::ms_record;

#[test]
fn parse() {
    use std::io::Read;
    let mut file = std::fs::File::open("tests/ff00b5d8b3124f1aa2de549070709634").unwrap();
    let mut buf = vec![];
    let _ = file.read_to_end(&mut buf).unwrap();
    buf.drain(..8); // First 8 bytes are the seedlink header
    let r = ms_record::parse(&buf);
    println!("{}", r);
}
