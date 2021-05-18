miniseed
========
MiniSEED Library for rust

This is a interface library over the libmseed C library that can be found at 
https://github.com/iris-edu/libmseed

For information about the data formats, see:

- MiniSEED: http://ds.iris.edu/ds/nodes/dmc/data/formats/miniseed/
- SEED: http://ds.iris.edu/ds/nodes/dmc/data/formats/seed/

### Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
miniseed = "^1"
```

and this to your crate root:

```rust
extern crate miniseed;
```

### Examples

Read a single record from a file and display its metadata:

```rust
extern crate miniseed;

use miniseed::ms_record;

fn main() {
    let file = "tests/sample.miniseed";
    let m = ms_record::read(file);
    println!("{}", m);
}
```

Read from `input.mseed`, and write only those records from the network `AU` to
`output.mseed`:

```rust
extern crate miniseed;

use miniseed::{ms_input, ms_output};

fn main() {
    let input = ms_input::open("input.mseed");
    let mut output = ms_output::open("output.mseed").unwrap();

    for record in input {
        if record.network() == "AU" {
            output.write(&record);
        }
    }
}
```


### Documentation

https://docs.rs/miniseed/
