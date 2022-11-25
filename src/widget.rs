use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::pixelcolor::BinaryColor::Off;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle, StyledDrawable};

use crate::display::disp;

pub trait Widget {
    type Output;

    fn space(&self) -> Space;

    fn size(&self) -> Size {
        self.space().size
    }

    fn position(&self) -> Point {
        self.space().position
    }

    async fn render(&self) -> Self::Output;
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Space {
    pub position: Point,
    pub size: Size,
}

impl Space {
    pub fn new(position: Point, size: Size) -> Self {
        Self { position, size }
    }
}

#[inline]
pub async fn clean_space(space: Space) {
    clean_space_on(space, &mut *disp().await)
}

pub fn clean_space_on<D>(space: Space, dt: &mut D)
where
    D: DrawTarget<Color = BinaryColor>,
{
    let _ =
        Rectangle::new(space.position, space.size).draw_styled(&PrimitiveStyle::with_fill(Off), dt);
}
