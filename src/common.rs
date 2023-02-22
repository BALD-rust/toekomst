use crate::key::Key;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Rectangle, StyledDrawable};
use embedded_graphics::text::Text;

use crate::widget::Space;
use crate::{text_inverted, thin_line, SMALL_FONT};

pub const ANNOTATION_SIZE: Size = SMALL_FONT.character_size;
pub const ANNOTATION_WIDTH: i32 = ANNOTATION_SIZE.width as i32;

pub fn draw_thin_rect<D>(space: Space, dt: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    Rectangle::new(space.position, space.size).draw_styled(&thin_line(), dt)
}

pub fn draw_accelerator<D>(
    top_left: Point,
    key: Key,
    dt: &mut D,
) -> Result<Point, <D as DrawTarget>::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    Text::new(
        key.into(),
        top_left + Point::new(1, SMALL_FONT.baseline as i32 + 1),
        text_inverted(SMALL_FONT),
    )
    .draw(dt)
}

#[inline(always)]
pub fn pt(x: i32, y: i32) -> Point {
    Point::new(x, y)
}
