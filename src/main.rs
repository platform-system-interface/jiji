use winnow::prelude::*;
use winnow::{
    Result,
    error::{ContextError, ErrMode},
};

mod jiji;

const CMD_SIZE: usize = 4;

fn parse(input: &mut &str) -> Result<(), ErrMode<ContextError>> {
    // Our input is pure joy, we hope!
    let mut joy = input.as_bytes();
    while joy.len() > CMD_SIZE {
        let cmd = &joy[0..CMD_SIZE];
        // Skip over whitespaces.
        let c = cmd[0] as char;
        if c == ' ' {
            joy = &joy[1..];
            continue;
        }

        // Consume command and skip along, then parse data.
        joy = &joy[CMD_SIZE..];
        match cmd {
            b"MOVE" => {
                if let Ok(r) = jiji::point.parse_next(&mut joy) {
                    println!("  move     {r:?}");
                    continue;
                }
            }
            b"LINE" => {
                if let Ok(r) = jiji::vector.parse_next(&mut joy) {
                    println!("  line     {r:?}");
                    continue;
                }
            }
            b"PATH" => {
                if let Ok(r) = jiji::vectors.parse_next(&mut joy) {
                    println!("  path     {r:?}");
                    continue;
                }
            }
            b"POLY" => {
                if let Ok(r) = jiji::vectors.parse_next(&mut joy) {
                    println!("  polygon  {r:?}");
                    continue;
                }
            }
            b"COLO" => {
                if let Ok(r) = jiji::color.parse_next(&mut joy) {
                    println!("  color    {r:?}");
                    continue;
                }
            }
            x => {
                if let Ok(s) = str::from_utf8(x) {
                    println!("invalid command: {s}");
                } else {
                    println!("invalid command: {x:?}");
                }
                break;
            }
        }
    }
    Ok(())
}

fn main() {
    let mut s = "MOVE 12  13  COLO 23 42 123  PATH2 3 -4 5 MOVE 42 23 LINE 6 7 MOVE1 2 POLY  8 9   12 14  13 12";
    parse(&mut s).unwrap();
}
