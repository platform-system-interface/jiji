//! JIJI is Joy In Jotting Images
//!
//! JIJI is a linear, streamable command sequence language for vector graphics.
//!
//! - JIJI, start of sequence and set dimensions
//! - COLO(r), set stroke color
//! - FILL, set fill color
//! - SIZE, set stroke/brush size
//! - MOVE, move to a position
//! - PATH, a list of points
//! - POLY(gon), closed Path with fill <https://en.wikipedia.org/wiki/Polygon>
//! - TRIA(ngle), specific Polygon
//! - RECT, specific Polygon
//! - BEZI(er), sequence of cubic Bézier curves <https://en.wikipedia.org/wiki/Composite_B%C3%A9zier_curve>
//! - AREA, an arbitrary, filled area defined by Bezier curves
//! - ELLI(pse), specific Area
//! - CIRC(le), specific Area
//! - CLIP, Bezier curve defined area for clipping
//! - FONT, size + string with font modifier and family
//! - TEXT, just text in quotes ""/'' :)
//! - SPRI(te), scaling dimensions + base64 encoded bitmap/PNG/JPEG/webp image
//! - DONE, end of sequence

use std::fmt::Display;

use winnow::combinator::opt;
use winnow::prelude::*;
use winnow::{
    Result,
    ascii::{dec_int, dec_uint, float, space0, space1},
    combinator::seq,
    error::{ContextError, ErrMode},
};

#[derive(Debug, Clone, Eq, PartialEq)]
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

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
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
        b: num,
    })
    .parse_next(input)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct Size {
    pub(crate) s: u32,
}

impl std::str::FromStr for Size {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        size.parse(s.as_bytes()).map_err(|e| e.to_string())
    }
}

/// Parse a Size, which is exactly one number.
pub(crate) fn size(input: &mut &[u8]) -> ModalResult<Size> {
    let mut num = dec_uint::<_, u32, ErrMode<ContextError>>;
    seq!(Size {
        _: space0,
        s: num,
    })
    .parse_next(input)
}

#[derive(Debug, Clone, Eq, PartialEq)]
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

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl Point {
    pub fn add(&mut self, v: Vector) {
        self.x = (self.x as i32 + v.x) as u32;
        self.y = (self.y as i32 + v.y) as u32;
    }
}

/// Parse exactly one Point.
pub(crate) fn point(input: &mut &[u8]) -> ModalResult<Point> {
    let mut num = dec_uint::<_, u32, ErrMode<ContextError>>;
    seq!(Point {
        _: space0,
        x: num,
        _: space1,
        y: num,
    })
    .parse_next(input)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct Vector {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl std::str::FromStr for Vector {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        vector.parse(s.as_bytes()).map_err(|e| e.to_string())
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

/// Parse exactly one Vector.
pub(crate) fn vector(input: &mut &[u8]) -> ModalResult<Vector> {
    let mut num = dec_int::<_, i32, ErrMode<ContextError>>;
    seq!(Vector {
        _: space0,
        x: num,
        _: space1,
        y: num,
    })
    .parse_next(input)
}

/// Parse a sequence of vectors.
pub(crate) fn vectors(input: &mut &[u8]) -> ModalResult<Vec<Vector>> {
    let mut vectors = vec![];
    while let Some(p) = opt(vector).parse_next(input)? {
        vectors.push(p);
    }
    ModalResult::Ok(vectors)
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Bezier {
    pub(crate) dx1: f32,
    pub(crate) dy1: f32,
    pub(crate) dx2: f32,
    pub(crate) dy2: f32,
    pub(crate) dx: f32,
    pub(crate) dy: f32,
}

impl std::str::FromStr for Bezier {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        bezier.parse(s.as_bytes()).map_err(|e| e.to_string())
    }
}

impl Display for Bezier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}, {} {}, {} {}",
            self.dx1, self.dy1, self.dx2, self.dy2, self.dx, self.dy
        )
    }
}

/// Parse exactly one Vector.
pub(crate) fn bezier(input: &mut &[u8]) -> ModalResult<Bezier> {
    let mut num = float::<_, f32, ErrMode<ContextError>>;
    seq!(Bezier {
        _: space0,
        dx1: num,
        _: space1,
        dy1: num,
        _: space1,
        dx2: num,
        _: space1,
        dy2: num,
        _: space1,
        dx: num,
        _: space1,
        dy: num,
    })
    .parse_next(input)
}

/// Parse a sequence of Bezier curves.
pub(crate) fn beziers(input: &mut &[u8]) -> ModalResult<Vec<Bezier>> {
    let mut vectors = vec![];
    while let Some(p) = opt(bezier).parse_next(input)? {
        vectors.push(p);
    }
    ModalResult::Ok(vectors)
}
