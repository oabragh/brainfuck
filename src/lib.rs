use std::collections::VecDeque;
use std::io::Read;

use anyhow::{Context, Result};

/// Represents individual Brainfuck instructions.
pub enum Instruction {
    OpCode(char),
    Loop(Vec<Instruction>),
}

/// Parses Brainfuck source code into a vector of instructions.
///
/// # Arguments
///
/// * `source` - A string containing Brainfuck source code.
///
/// # Returns
///
/// Returns a `Result` containing a vector of `Instruction`s if parsing is successful,
/// or an `anyhow::Error` if there is a parsing error.
pub fn parse(source: String) -> Result<Vec<Instruction>> {
    let mut chars = source.chars();
    let mut result: Vec<Instruction> = vec![];

    while let Some(next) = chars.next() {
        match next {
            code @ ('>' | '<' | '+' | '-' | '.' | ',') => result.push(Instruction::OpCode(code)),
            '[' => {
                let mut nested_source = String::new();
                let mut depth = 1;

                for nested_char in chars.by_ref() {
                    match nested_char {
                        '[' => depth += 1,
                        ']' => {
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                        }
                        _ => (),
                    }
                    nested_source.push(nested_char);
                }

                if depth != 0 {
                    anyhow::bail!("Unclosed loop found.");
                }

                let nested_instructions = parse(nested_source)?;
                result.push(Instruction::Loop(nested_instructions));
            }
            ']' => anyhow::bail!(anyhow::anyhow!("Loop ending has no beginning.")),
            _ => (),
        };
    }

    Ok(result)
}

/// Executes a sequence of Brainfuck instructions.
///
/// # Arguments
///
/// * `instructions` - A slice of `Instruction`s to execute.
/// * `tape` - A mutable reference to a `VecDeque` representing the Brainfuck tape.
/// * `pointer` - A mutable reference to the tape pointer position.
///
/// # Returns
///
/// Returns a `Result` indicating success or an `anyhow::Error` if an execution error occurs.
pub fn run(
    instructions: &[Instruction],
    tape: &mut VecDeque<u8>,
    pointer: &mut usize,
) -> Result<()> {
    use Instruction::*;

    for i in instructions {
        match i {
            OpCode('>') => {
                *pointer += 1;

                while *pointer >= tape.len() {
                    tape.push_back(0u8);
                }
            }
            OpCode('<') => {
                *pointer -= 1;
            }
            OpCode('+') => tape[*pointer] = tape[*pointer].wrapping_add(1),
            OpCode('-') => tape[*pointer] = tape[*pointer].wrapping_sub(1),
            OpCode('.') => print!("{}", tape[*pointer] as char),
            OpCode(',') => {
                let mut input = [0u8; 1];
                std::io::stdin()
                    .read_exact(&mut input)
                    .context("Couldn't read line")?;

                tape[*pointer] = input[0]
            }
            Loop(instructions) => {
                while tape[*pointer] != 0 {
                    run(instructions, tape, pointer)?;
                }
            }
            _ => (),
        }
    }

    Ok(())
}
