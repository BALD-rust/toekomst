use embedded_graphics::prelude::{Point, Size};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Vertical {
    cursor: Point,
    spacing: u32,
}

impl Vertical {
    pub fn new(top_left: Point, spacing: u32) -> Self {
        Self {
            cursor: top_left,
            spacing,
        }
    }

    pub fn new_tight(top_left: Point) -> Self {
        Self::new(top_left, 0)
    }

    #[inline]
    pub fn current(&self) -> Point {
        self.cursor
    }

    pub fn push(&mut self, size: Size) -> Point {
        self.cursor.y += (size.height + self.spacing) as i32;
        self.current()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Horizontal {
    cursor: Point,
    spacing: u32,
}

impl Horizontal {
    pub fn new(top_left: Point, spacing: u32) -> Self {
        Self {
            cursor: top_left,
            spacing,
        }
    }

    pub fn new_tight(top_left: Point) -> Self {
        Self::new(top_left, 0)
    }

    #[inline]
    pub fn current(&self) -> Point {
        self.cursor
    }

    pub fn push(&mut self, size: Size) -> Point {
        self.cursor.x += (size.width + self.spacing) as i32;
        self.current()
    }
}
