use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Args {
    file: String,
}

fn read_code(file_path: &str) -> Result<String, String> {
    return fs::read_to_string(file_path).map_err(|_| "Could not read file".to_string());
}

#[derive(Debug)]
enum ParseState {
    Start,
    SawM,
    SawU,
    SawL,
    SawLParen,
    Arg1,
    SawComma,
    Arg2,
    SawD,
    SawO,
    SawN,
    SawApostrophe,
    SawT,
    SawDoLParen,
    SawDontLParen,
}

#[derive(Debug)]
struct ExecState {
    parse_state: ParseState,
    arg1: i32,
    buf_start: usize,
    buf_end: usize,
    mul_enabled: bool,
}

fn execute(code: &str) -> i32 {
    let mut state: ExecState = ExecState {
        parse_state: ParseState::Start,
        arg1: 0,
        buf_start: 0,
        buf_end: 1,
        mul_enabled: true,
    };

    let mut sum = 0;
    for (i, c) in code.chars().enumerate() {
        state.parse_state = match (state.parse_state, c) {
            (ParseState::Start, 'm') => ParseState::SawM,
            (ParseState::SawM, 'u') => ParseState::SawU,
            (ParseState::SawU, 'l') => ParseState::SawL,
            (ParseState::SawL, '(') => ParseState::SawLParen,
            (ParseState::SawLParen, '0'..='9') => {
                state.buf_start = i;
                state.buf_end = i + 1;
                ParseState::Arg1
            }
            (ParseState::Arg1, '0'..='9') => {
                state.buf_end = i + 1;
                ParseState::Arg1
            }
            (ParseState::Arg1, ',') => {
                let buf = &code[state.buf_start..state.buf_end];
                state.arg1 = buf.parse::<i32>().unwrap();
                ParseState::SawComma
            }
            (ParseState::SawComma, '0'..='9') => {
                state.buf_start = i;
                state.buf_end = i + 1;
                ParseState::Arg2
            }
            (ParseState::Arg2, '0'..='9') => {
                state.buf_end = i + 1;
                ParseState::Arg2
            }
            (ParseState::Arg2, ')') => {
                if state.mul_enabled {
                    let buf = &code[state.buf_start..state.buf_end];
                    let arg2 = buf.parse::<i32>().unwrap();
                    sum += state.arg1 * arg2;
                }
                ParseState::Start
            }
            (_, 'm') => ParseState::SawM,
            (ParseState::Start, 'd') => ParseState::SawD,
            (ParseState::SawD, 'o') => ParseState::SawO,
            (ParseState::SawO, 'n') => ParseState::SawN,
            (ParseState::SawN, '\'') => ParseState::SawApostrophe,
            (ParseState::SawApostrophe, 't') => ParseState::SawT,
            (ParseState::SawT, '(') => ParseState::SawDontLParen,
            (ParseState::SawDontLParen, ')') => { 
                state.mul_enabled = false;
                ParseState::Start
            }
            (ParseState::SawO, '(') => ParseState::SawDoLParen,
            (ParseState::SawDoLParen, ')') => {
                state.mul_enabled = true;
                ParseState::Start
            }
            (_, 'd') => ParseState::SawD,
            _ => ParseState::Start,
        }
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute() {
        assert_eq!(execute("mul(3,2)"), 6);
        assert_eq!(execute("asdfaxxmul(3,5mul(3,2)"), 6);
        assert_eq!(execute("mul(11,2)"), 22);
        assert_eq!(execute("mul(11,12),3"), 132);
        assert_eq!(execute("mul(1,2),mul(3,4)"), 14);
        assert_eq!(execute("mul(1,,2),mul(3,4)"), 12);
    }

    #[test]
    fn test_execute_conditionals() {
        assert_eq!(execute("mul(1,2)don't()"), 2);
        assert_eq!(execute("don't()mul(1,2)"), 0);
        assert_eq!(execute("mul(1,2)don't(3,4)do()dontmul(5,6)"), 32);
    }
}

fn main() {
    let args = Args::parse();
    let code = read_code(&args.file).unwrap();

    let sum = execute(&code);

    println!("sum: {}", sum);
}
