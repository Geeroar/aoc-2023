mod utils;
mod gee;
mod roar;

use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() -> std::io::Result<()> {
    let stdout: std::io::Stdout = stdout();
    let message = "AOC 2023!";
    let width: usize = message.chars().count();

    let mut writer: BufWriter<std::io::StdoutLock<'_>> = BufWriter::new(stdout.lock());
    say(message, width, &mut writer).unwrap();

    Ok(())
}
