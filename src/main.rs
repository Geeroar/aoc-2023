mod gee;

use ferris_says::say;
use std::io::{stdout, BufWriter};

mod rtob {
    pub mod day1;
}

use rtob::day1;

fn print_result(number: u8, part1: &impl ToString) {
    println!("Question {:}", number);
    let part1_output: String = part1.to_string();
    println!("------------");
    let stdout: std::io::Stdout = stdout();
    let message: String = format!("Part 1: {part1_output}");
    let width: usize = message.chars().count();

    let mut writer: BufWriter<std::io::StdoutLock<'_>> = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}

fn main() -> std::io::Result<()> {
    println!("------------");
    print_result(1, &day1::part_1("input/rtob/q1-p1-input.txt")?);
    print_result(2, &day1::part_2("input/rtob/q1-p2-input.txt")?);

    Ok(())
}
