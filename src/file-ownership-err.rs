
use std::io::{File,IoResult};

fn main() {
    let mut f = File::open(&Path::new("hello.txt"))
        .ok().expect("could not open file");
    use_file(f)
        .ok().expect("could not use file");
    f.flush()
        .ok().expect("could not flush file");
}

fn use_file(f: File) -> IoResult<()> {
    f.write_str("hello")
}
