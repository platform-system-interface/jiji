use svg::node::element::Polygon;
use winnow::prelude::*;
use winnow::{
    Result,
    error::{ContextError, ErrMode},
};

use crate::jiji::{Color, Point};

mod jiji;

const CMD_SIZE: usize = 4;

fn parse_and_render(input: &mut &str, file: &str) -> Result<(), ErrMode<ContextError>> {
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
                }
                continue;
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
                }
                continue;
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
                }
                continue;
            }
            b"POLY" => {
                if let Ok(r) = jiji::vectors.parse_next(&mut joy) {
                    println!("  polygon  {r:?}");
                    let mut curr = pos.clone();
                    let mut points = format!("{pos}");
                    for p in r {
                        curr.add(p);
                        points.push_str(format!(" {curr}").as_str());
                    }
                    let polygon = Polygon::new()
                        .set("fill", format!("{fill}").as_str())
                        .set("points", points);
                    document = document.add(polygon);
                }
                continue;
            }
            b"BEZI" => {
                if let Ok(r) = jiji::beziers.parse_next(&mut joy) {
                    println!("  bezier   {r:?}");
                    let mut data = Data::new().move_to((pos.x, pos.y));
                    for c in r {
                        data = data.cubic_curve_by((c.dx1, c.dy1, c.dx2, c.dy2, c.dx, c.dy));
                    }
                    data = data.close();
                    let path = Path::new()
                        .set("fill", format!("none").as_str())
                        .set("stroke", format!("{color}").as_str())
                        .set("stroke-width", size)
                        .set("d", data);
                    document = document.add(path);
                }
                continue;
            }
            b"AREA" => {
                if let Ok(r) = jiji::beziers.parse_next(&mut joy) {
                    println!("  area     {r:?}");
                    let mut data = Data::new().move_to((pos.x, pos.y));
                    for c in r {
                        data = data.cubic_curve_by((c.dx1, c.dy1, c.dx2, c.dy2, c.dx, c.dy));
                    }
                    data = data.close();
                    let path = Path::new()
                        .set("fill", format!("{fill}").as_str())
                        .set("d", data);
                    document = document.add(path);
                }
                continue;
            }
            b"FILL" => {
                if let Ok(r) = jiji::color.parse_next(&mut joy) {
                    println!("  fill     {r:?}");
                    fill = r;
                }
                continue;
            }
            b"COLO" => {
                if let Ok(r) = jiji::color.parse_next(&mut joy) {
                    println!("  color    {r:?}");
                    color = r;
                }
                continue;
            }
            b"SIZE" => {
                if let Ok(r) = jiji::size.parse_next(&mut joy) {
                    println!("  size     {r:?}");
                    size = r.s;
                }
                continue;
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

    svg::save(file, &document).unwrap();

    Ok(())
}

const RECT: &str = &"POLY 150 0 0 20 -150 0 0 -20";
const COL0: &str = &"91 207 250";
const COL1: &str = &"244 122 230";
const TEST: &str = &"MOVE 12  13  COLO 23 42 123   SIZE 3  PATH23 32 -4 5 MOVE 42 23 PATH 6 7 MOVE1 2  FILL 244 122 230 POLY  18 19   2 -14  -13 12";
const BEZI: &str = &"MOVE 70 20 FILL 40 90 120  AREA 27.614 0  50 22.386  50 50    0  27.614  -22.386 50  -50  50   -27.614  0  -50 -22.386  -50 -50  0  -27.614  22.386 -50  50 -50  FILL 240 90 120 COLO 244 182 230 MOVE 55 80  BEZI 15 25  15 25  30 0";

fn main() {
    let s = format!(
        "SIZE 2 MOVE 9 9  PATH 152 0 0 102 -152 0 0 -102 MOVE 10 50 {RECT}  FILL {COL0} MOVE 10 10 {RECT} MOVE 10 90 {RECT} FILL {COL1} MOVE 10 30 {RECT} MOVE 10 70 {RECT} {TEST}"
    );
    let mut s = s.as_str();
    parse_and_render(&mut s, "flag.svg").unwrap();
    parse_and_render(&mut TEST.to_string().as_str(), "test.svg").unwrap();
    parse_and_render(&mut BEZI.to_string().as_str(), "bezi.svg").unwrap();
}
