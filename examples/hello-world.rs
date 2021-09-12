use std::io::Read;

use brainfuck_interpreter::{BrainfuckProgram, Error};

fn main() {
    let program = r#"++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>."#;

    let tape = [0u8; 2048];

    let mut program: BrainfuckProgram<[u8; 2048]> = match BrainfuckProgram::compile(program, tape) {
        Ok(program) => program,
        Err(error) => {
            match error {
                Error::MissingClosingBrace(span) => {
                    let (line, character) = span.get_line_character_number();
                    println!(
                        "Missing closing brace at line {}, character {}:\n{}",
                        line, character, span
                    );
                }
                Error::MissingOpeningBrace(span) => {
                    let (line, character) = span.get_line_character_number();
                    println!(
                        "Missing opening brace at line {}, character {}:\n{}",
                        line, character, span
                    );
                }
            }
            return;
        }
    };

    let mut input = std::io::stdin();

    let output_func = &mut |_| {};

    let mut buf = [0u8; 1];
    let input_func = &mut || {
        input.read(&mut buf).ok();
        buf[0]
    };

    program.run(output_func, input_func);
}
