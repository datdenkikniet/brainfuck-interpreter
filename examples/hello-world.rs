use std::{
    io::{stdout, Read, Write},
    time::SystemTime,
};

use brainfuck_interpreter::{BrainfuckProgram, Error};
use number_prefix::NumberPrefix;

fn main() {
    let program = r#"+[>+]"#;

    let tape = [0u8; 1_000_000];

    let mut program: BrainfuckProgram<[u8; 1_000_000]> =
        match BrainfuckProgram::compile(program, tape) {
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
    let mut output = std::io::stdout();

    let output_func = &mut |d| {
        output.write(&[d]).ok();
    };

    let mut buf = [0u8; 1];
    let input_func = &mut || {
        input.read(&mut buf).ok();
        buf[0]
    };

    let start_time = SystemTime::now();

    program.run(output_func, input_func);

    let end_time = SystemTime::now();

    let duration = end_time.duration_since(start_time).unwrap();

    let millis = duration.as_millis() as f64;

    let hertz = program.execution_count as f64 / (millis / 1000.0);
    let hertz_string = match NumberPrefix::decimal(hertz) {
        NumberPrefix::Standalone(hertz) => format!("{} hz", hertz),
        NumberPrefix::Prefixed(prefix, n) => format!("{:.02} {}hz", n, prefix),
    };

    println!(
        "Performed {} instructions in {} ms, running at an effective speed of {} hz.",
        program.execution_count,
        duration.as_millis(),
        hertz_string,
    );
}
