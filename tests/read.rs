
extern crate miniseed;
extern crate glob;

use miniseed::ms_record;

#[test]
fn read() {
    let mut ms = vec![];
    for entry in glob::glob("tests/sample*").unwrap() {
        if let Ok(f) = entry {
            let m = ms_record::read(f);
            ms.push(m)
        }
    }
    for m in &ms {
        println!("{}", m);
    }
}
