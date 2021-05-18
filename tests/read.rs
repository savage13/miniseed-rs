extern crate glob;
extern crate miniseed;

use miniseed::{ms_input, ms_output, ms_record};

#[test]
fn read() {
    let mut ms = vec![];
    for entry in glob::glob("tests/sample*").unwrap() {
        if let Ok(f) = entry {
            let m = ms_record::read(f);
            ms.push(m);
        }
    }
    for m in &ms {
        println!("{}", m);
    }
}

#[test]
fn read_multiple() {
    let input = ms_input::open("tests/multiple.seed");
    let ms: Vec<_> = input.collect();
    for m in &ms {
        println!("{}", m);
    }

    // Sequence Number is incorrect, but everything else is "ok"
    let mut out = ms_output::open("tests/multiple_out.seed").unwrap();
    for m in &ms {
        out.write(m);
    }
}
