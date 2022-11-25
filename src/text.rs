use core::future::poll_fn;
use core::task::Poll;

use embedded_graphics::prelude::{Point, Size};
use embedded_graphics::text::Text;
use embedded_graphics::Drawable;

use crate::display::disp;
use crate::notify::Notify;
use crate::widget::{clean_space, Space};
use crate::{text, FONT};

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
        log::trace!("draw text");
        let s = s.as_ref();

        clean_space(space).await;
        space = Space::new(
            p,
            Size::new(
                (SIZE.width + FONT.character_spacing) * s.len() as u32,
                SIZE.height,
            ),
        );

        label_once(s, p).await;
    }
}

pub async fn label_once<S: AsRef<str>>(s: S, p: Point) {
    let _ = Text::new(
        s.as_ref(),
        p + Point::new(0, FONT.baseline as i32),
        text(FONT),
    )
    .draw(&mut *disp().await);
}
