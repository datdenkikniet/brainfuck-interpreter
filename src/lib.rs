//! A simple (and maybe not super efficient) JIT compiler for [`Brainfuck`]
//!
//! [`Brainfuck`]: https://en.wikipedia.org/wiki/Brainfuck

#![deny(missing_docs)]
#![deny(warnings)]

mod tape;

pub use tape::*;

use std::fmt::Display;

/// All valid characters for a Brainfuck program
pub const VALID_CHARS: [char; 8] = ['>', '<', '+', '-', '.', ',', '[', ']'];

/// All of the instructions available in Brainfuck
#[derive(Clone, Copy, Debug)]
pub enum BrainfuckInstruction {
    /// `>` command, to increment the data pointer
    IncrementDataPointer,
    /// `<` command, to decrement the data pointer
    DecrementDataPointer,
    /// `+` command, to increment the data at the data pointer
    IncreaseData,
    /// `-` command, to decerement the data at the data pointer
    DecreaseData,
    /// `.` command, to output a byte
    Output,
    /// `,` command, to input a byte
    Input,
    /// `[`, to jump to the matching `]` if the data at the data pointer
    /// is zero
    JumpForward(usize),
    /// `]`, to jump to the matching `[` if the data at the data pointer
    /// is non-zero
    JumpBackwards(usize),
}

/// An instruction, and its associated position
/// in the input code
#[derive(Clone, Debug)]
pub struct Span<'a> {
    instruction: BrainfuckInstruction,
    text: &'a str,
    line: usize,
    character: usize,
}

/// Print the Span as follows:
/// first, print the line that this span is about
/// then, print a line containing an up-arrow pointing at the character that this span is about
/// I.e.:
/// [[..,,>><<]]
///       ^
impl<'a> Display for Span<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut arrow = String::with_capacity(self.character);
        for _ in 0..self.character - 1 {
            arrow.push(' ');
        }
        arrow.push('^');

        let mut index = 0;
        let mut text_line = None;
        for line in self.text.lines() {
            if index == self.line {
                text_line = Some(line);
                break;
            }
            index += 1;
        }

        let text_line = match text_line {
            Some(text_line) => text_line,
            None => return Err(std::fmt::Error),
        };

        f.write_str(format!("{}\n{}", text_line, arrow).as_str())
    }
}

impl<'a> Span<'a> {
    /// Get the full text of this span
    pub fn get_text(&self) -> &str {
        self.text
    }
    /// Get the line number and character index of the character
    /// that this span describes
    pub fn get_line_character_number(&self) -> (&usize, &usize) {
        (&self.line, &self.character)
    }
}

/// A Brainfuck program
#[derive(Clone, Debug)]
pub struct BrainfuckProgram<T>
where
    T: Tape,
{
    /// The instruction pointer of this program
    pub instruction_pointer: usize,
    /// The data poitner of this program
    pub data_pointer: usize,
    /// The instructions of this program
    pub instructions: Vec<BrainfuckInstruction>,
    /// The tape of this program
    pub tape: T,
}

/// An error that can occur while interpreting/compiling Brainfuck
#[derive(Clone, Debug)]
pub enum Error<'a> {
    /// A `[` does not have a matching `]`
    MissingClosingBrace(Span<'a>),
    /// A `]` does not have a matching `[`
    MissingOpeningBrace(Span<'a>),
}

impl<T> BrainfuckProgram<T>
where
    T: Tape,
{
    fn parse_input(input: &str) -> Result<Vec<Span>, Error> {
        let mut result = Vec::new();
        let mut line_ind = 0;
        let mut char_ind = 0;
        for line in input.lines() {
            for character in line.chars() {
                char_ind += 1;
                if VALID_CHARS.contains(&character) {
                    let instr = match character {
                        '>' => BrainfuckInstruction::IncrementDataPointer,
                        '<' => BrainfuckInstruction::DecrementDataPointer,
                        '+' => BrainfuckInstruction::IncreaseData,
                        '-' => BrainfuckInstruction::DecreaseData,
                        '.' => BrainfuckInstruction::Output,
                        ',' => BrainfuckInstruction::Input,
                        '[' => BrainfuckInstruction::JumpForward(0),
                        ']' => BrainfuckInstruction::JumpBackwards(0),
                        _ => unreachable!(),
                    };

                    result.push(Span {
                        instruction: instr,
                        text: input,
                        line: line_ind,
                        character: char_ind,
                    });
                } else {
                    continue;
                }
            }
            char_ind = 0;
            line_ind += 1;
        }
        Ok(result)
    }

    /// Find matching `[` for a `]` located at `index` in `instructions`
    ///
    /// Returns the offset required for the jump on success, and else an error
    fn find_closer<'a>(index: usize, instructions: &Vec<Span<'a>>) -> Result<usize, Error<'a>> {
        let mut calculated_offset = 0;
        let mut extra_openers = 0;
        let iterator = instructions.iter().skip(index + 1);
        for next_instr in iterator {
            calculated_offset += 1;
            match next_instr.instruction {
                BrainfuckInstruction::JumpBackwards(_) => {
                    if extra_openers == 0 {
                        return Ok(calculated_offset + 1);
                    } else {
                        extra_openers -= 1;
                    }
                }
                BrainfuckInstruction::JumpForward(_) => {
                    extra_openers += 1;
                }
                _ => {}
            }
        }
        unsafe {
            Err(Error::MissingClosingBrace(
                instructions.get_unchecked(index).clone(),
            ))
        }
    }

    /// Find matching `]` for a `[` located at `index` in `instructions`
    ///
    /// Returns the offset required for the jump on success, and else an error
    fn find_opener<'a>(index: usize, instructions: &Vec<Span<'a>>) -> Result<usize, Error<'a>> {
        let mut calculated_offset = 0;
        let mut extra_closers = 0;
        let iterator = instructions.iter().rev().skip(instructions.len() - index);
        for next_instr in iterator {
            calculated_offset += 1;
            match next_instr.instruction {
                BrainfuckInstruction::JumpForward(_) => {
                    if extra_closers == 0 {
                        return Ok(calculated_offset - 1);
                    } else {
                        extra_closers -= 1;
                    }
                }
                BrainfuckInstruction::JumpBackwards(_) => {
                    extra_closers += 1;
                }
                _ => {}
            }
        }
        unsafe {
            return Err(Error::MissingOpeningBrace(
                instructions.get_unchecked(index).clone(),
            ));
        }
    }

    /// Compile a Brainfuck program, given by `input`. All non-valid characters are ignored
    pub fn compile(input: &str, tape: T) -> Result<Self, Error> {
        let mut parse_result = Self::parse_input(input)?;

        let clone = parse_result.clone();
        let mut index = 0;

        for span in parse_result.iter_mut() {
            match &mut span.instruction {
                BrainfuckInstruction::JumpForward(offset) => {
                    *offset = Self::find_closer(index, &clone)?;
                }
                BrainfuckInstruction::JumpBackwards(offset) => {
                    *offset = Self::find_opener(index, &clone)?;
                }
                _ => {}
            }
            index += 1;
        }

        Ok(Self {
            instruction_pointer: 0,
            data_pointer: 0,
            instructions: parse_result
                .iter()
                .map(|span| span.instruction.clone())
                .collect(),
            tape,
        })
    }

    /// Perform a step in the Brainfuck program
    pub fn step<FnOut, FnIn>(&mut self, output: &mut FnOut, input: &mut FnIn) -> Result<(), ()>
    where
        FnOut: FnMut(T::Data),
        FnIn: FnMut() -> T::Data,
    {
        let data_pointer = &mut self.data_pointer;
        let instruction_pointer = &mut self.instruction_pointer;

        let data = match self.tape.get_data_at_mut(*data_pointer) {
            Some(data) => data,
            None => panic!("Data pointer went out of bounds! {}", data_pointer),
        };

        let instructions = &self.instructions;

        let instruction = match instructions.get(*instruction_pointer) {
            Some(instr) => instr,
            None => return Err(()),
        };

        match instruction {
            BrainfuckInstruction::IncrementDataPointer => {
                *data_pointer += 1;
            }
            BrainfuckInstruction::DecrementDataPointer => {
                *data_pointer -= 1;
            }
            BrainfuckInstruction::IncreaseData => {
                data.increase();
            }
            BrainfuckInstruction::DecreaseData => {
                data.decrease();
            }
            BrainfuckInstruction::Output => {
                output(data.clone());
            }
            BrainfuckInstruction::Input => {
                *data = input();
            }
            BrainfuckInstruction::JumpForward(offset) => {
                if *data == T::Data::zero() {
                    *instruction_pointer += offset;
                    return Ok(());
                }
            }
            BrainfuckInstruction::JumpBackwards(offset) => {
                if *data != T::Data::zero() {
                    *instruction_pointer -= offset;
                    return Ok(());
                }
            }
        }
        *instruction_pointer += 1;
        Ok(())
    }

    /// Reset the program
    pub fn reset(&mut self) {
        self.data_pointer = 0;
        self.instruction_pointer = 0;
        self.tape.reset();
    }

    /// Run the Brainfuck program to completion
    pub fn run<FnOut, FnIn>(&mut self, output: &mut FnOut, input: &mut FnIn)
    where
        FnOut: FnMut(T::Data),
        FnIn: FnMut() -> T::Data,
    {
        while self.step(output, input).is_ok() {}
    }
}
