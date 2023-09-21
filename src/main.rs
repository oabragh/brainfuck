use std::collections::VecDeque;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    file: String
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let input = std::fs::read_to_string(args.file)?;

    let mut tape= VecDeque::from([0u8]);
    let mut pointer = 0;

    let instructions = brainfuck::parse(input)?;
    brainfuck::run(&instructions, &mut tape, &mut pointer)?;

    Ok(())
}
