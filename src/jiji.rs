//! JIJI is Joy In Jotting Images
//!
//! - JIJI, start of sequence and set dimensions
//! - COLO(r), set stroke color
//! - FILL, set fill color
//! - SIZE, set stroke/brush size
//! - MOVE, move to a position
//! - PATH, a list of points
//! - LINE (2-point Path)
//! - POLY(gon), closed Path with fill <https://en.wikipedia.org/wiki/Polygon>
//! - RECT, specific Polygon
//! - TRIA(ngle), specific Polygon
//! - BEZI(er), curve with list of points + angles/modifiers
//! - ELLI(pse), specific Curve
//! - CIRC(le), specific Ellipse
//! - TEXT, just text in quotes ""/'' :)
//! - SPRI(te), scaling dimensions + base64 encoded bitmap/PNG/JPEG/webp image
//! - DONE, end of sequence

use winnow::combinator::opt;
use winnow::prelude::*;
use winnow::{
    Result,
    ascii::{dec_uint, space0, space1},
    combinator::seq,
    error::{ContextError, ErrMode},
};

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Color {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
}

impl std::str::FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        color.parse(s.as_bytes()).map_err(|e| e.to_string())
    }
}

/// Parse a Color value.
pub(crate) fn color(input: &mut &[u8]) -> ModalResult<Color> {
    let mut num = dec_uint::<_, u8, ErrMode<ContextError>>;
    seq!(Color {
        _: space0,
        r: num,
        _: space1,
        g: num,
        _: space1,
        b: num
    })
    .parse_next(input)
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Point {
    pub(crate) x: u32,
    pub(crate) y: u32,
}

impl std::str::FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        point.parse(s.as_bytes()).map_err(|e| e.to_string())
    }
}

/// Parse exactly one Point.
pub(crate) fn point(input: &mut &[u8]) -> ModalResult<Point> {
    let mut num = dec_uint::<_, u32, ErrMode<ContextError>>;
    seq!(Point {
        _: space0,
        x: num,
        _: space1,
        y: num
    })
    .parse_next(input)
}

/// Parse a sequence of points.
pub(crate) fn points(input: &mut &[u8]) -> ModalResult<Vec<Point>> {
    let mut points = vec![];
    while let Some(p) = opt(point).parse_next(input)? {
        points.push(p);
    }
    ModalResult::Ok(points)
}
