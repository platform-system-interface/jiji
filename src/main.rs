use winnow::prelude::*;
use winnow::{
    Result,
    error::{ContextError, ErrMode},
};

use crate::jiji::{Color, Point};

mod jiji;

const CMD_SIZE: usize = 4;

fn parse(input: &mut &str) -> Result<(), ErrMode<ContextError>> {
    use svg::Document;
    use svg::node::element::Path;
    use svg::node::element::path::Data;

    let mut document = Document::new().set("viewBox", (0, 0, 70, 70));

    let mut pos = Point { x: 0, y: 0 };
    let mut color = Color { r: 0, g: 0, b: 0 };
    let mut size = 1;

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
                    pos = r;
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
                    let mut data = Data::new().move_to((pos.x, pos.y));
                    for p in r {
                        data = data.line_by((p.x, p.y));
                    }
                    data = data.close();
                    let path = Path::new()
                        .set("fill", "none")
                        .set("stroke", format!("{color}").as_str())
                        .set("stroke-width", size)
                        .set("d", data);
                    document = document.add(path);
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
                    color = r;
                    continue;
                }
            }
            b"SIZE" => {
                if let Ok(r) = jiji::size.parse_next(&mut joy) {
                    println!("  size     {r:?}");
                    size = r.s;
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

    svg::save("image.svg", &document).unwrap();

    Ok(())
}

fn main() {
    let mut s = "MOVE 12  13  COLO 23 42 123   SIZE 3  PATH23 32 -4 5 MOVE 42 23 LINE 6 7 MOVE1 2 POLY  8 9   12 14  13 12";
    parse(&mut s).unwrap();
}
