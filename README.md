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
miniseed = "1.0.0"
```

and this to your crate root:

```rust
extern crate miniseed;
```

### Example
```rust
extern crate miniseed;

use miniseed::ms_record;

fn main() {
    let file = "tests/sample.miniseed";
    let m = ms_record::read(file);
    println!("{}", m);
}

```

### Documentation

https://docs.rs/miniseed/

