use svg::node::element::Polygon;
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

    let mut document = Document::new().set("viewBox", (0, 0, 240, 160));

    let mut pos = Point { x: 0, y: 0 };
    let mut color = Color { r: 0, g: 0, b: 0 };
    let mut fill = Color {
        r: 255,
        g: 255,
        b: 255,
    };
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
                    let data = Data::new()
                        .move_to((pos.x, pos.y))
                        .line_by((r.x, r.y))
                        .close();
                    let path = Path::new()
                        .set("fill", "none")
                        .set("stroke", format!("{color}").as_str())
                        .set("stroke-width", size)
                        .set("d", data);
                    document = document.add(path);
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
                    let mut curr = pos.clone();
                    let mut points = format!("{pos}");
                    for p in r {
                        curr.add(p);
                        points.push_str(format!(" {curr}").as_str());
                    }
                    let polygon = Polygon::new()
                        .set("fill", format!("{fill}").as_str())
                        .set("points", points);
                    println!("  polygon  {:?}", &polygon);
                    document = document.add(polygon);
                    continue;
                }
            }
            b"FILL" => {
                if let Ok(r) = jiji::color.parse_next(&mut joy) {
                    println!("  fill     {r:?}");
                    fill = r;
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

const RECT: &str = &"POLY 150 0 0 20 -150 0 0 -20";
const COL0: &str = &"91 207 250";
const COL1: &str = &"244 122 230";

fn main() {
    // let mut s = "MOVE 12  13  COLO 23 42 123   SIZE 3  PATH23 32 -4 5 MOVE 42 23 LINE 6 7 MOVE1 2  FILL 244 122 230 POLY  18 19   12 14  13 12";
    let s = format!(
        "MOVE 9 9  PATH 152 0 0 102 -152 0 0 -102  FILL {COL0} MOVE 10 10 {RECT} MOVE 10 90 {RECT} FILL {COL1} MOVE 10 30 {RECT} MOVE 10 70 {RECT}"
    );
    parse(&mut s.as_str()).unwrap();
}
