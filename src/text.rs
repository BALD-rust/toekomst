use embedded_graphics::prelude::Point;
use embedded_graphics::text::Text;
use embedded_graphics::Drawable;

use core::future::poll_fn;
use core::task::Poll;

use crate::display::disp;
use crate::{text, FONT};

pub async fn label(s: &str, p: Point) {
    let _ =
        Text::new(s, p + Point::new(0, FONT.baseline as i32), text(FONT)).draw(&mut *disp().await);

    poll_fn(|_| Poll::Pending).await
}
