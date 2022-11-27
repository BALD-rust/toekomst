use core::future::poll_fn;
use core::task::Poll;

use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::{DrawTarget, Point, Size};
use embedded_graphics::text::Text;
use embedded_graphics::Drawable;

use crate::display::disp;
use crate::notify::Notify;
use crate::text;
use crate::widget::{clean_space_on, Space};
pub use crate::FONT;

pub async fn label(s: &str, p: Point) -> ! {
    label_once(s, p).await;
    poll_fn(|_| Poll::Pending).await
}

pub async fn label_with<S: AsRef<str> + Send>(notif: &Notify<S>, p: Point) -> ! {
    const SIZE: Size = FONT.character_size;

    let s = notif.wait().await;
    let s = s.as_ref();
    label_once(s, p).await;

    let mut space = Space::new(
        p,
        Size::new(
            (SIZE.width + FONT.character_spacing) * s.len() as u32,
            SIZE.height,
        ),
    );

    loop {
        let s = notif.wait().await;
        let s = s.as_ref();

        let dt = &mut *disp().await;

        clean_space_on(space, dt);
        space = Space::new(
            p,
            Size::new(
                (SIZE.width + FONT.character_spacing) * s.len() as u32,
                SIZE.height,
            ),
        );
        label_once_on(s, p, dt);
    }
}

#[inline]
pub async fn label_once<S: AsRef<str>>(s: S, p: Point) {
    label_once_on(s, p, &mut *disp().await)
}

pub fn label_once_on<S, D>(s: S, p: Point, dt: &mut D)
where
    S: AsRef<str>,
    D: DrawTarget<Color = BinaryColor>,
{
    let _ = Text::new(
        s.as_ref(),
        p + Point::new(0, FONT.baseline as i32),
        text(FONT),
    )
    .draw(dt);
}
