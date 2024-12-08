use std::{fs::File, io::Write};

fn main() -> std::io::Result<()> {
    let mut file = File::create("foo.txt")?;

    file.write_all(b"AWESOME STUFF!")?;

    Ok(())
}
