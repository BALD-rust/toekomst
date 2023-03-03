use embedded_graphics::prelude::{Point, Size};

use crate::common::{draw_accelerator, draw_thin_rect, pt, ANNOTATION_WIDTH};
use crate::display::disp;
use crate::key::{wait, Accel, Key};
use crate::label::label_once_on;
use crate::notify::Notify;
use crate::widget::{Space, Widget};
pub use crate::{FONT, SMALL_FONT};
use crate::request_redraw;

const SPACING: i32 = 3;

pub struct Button<'s, 'n, T> {
    text: &'s str,
    notif: &'n Notify<T>,
    key: Key,
    space: Space,
    value: T,
}

impl<'s, 'n, T: Send + Clone> Widget for Button<'s, 'n, T> {
    type Output = ();

    fn space(&self) -> Space {
        self.space
    }

    async fn render(&self) {
        {
            let dt = &mut *disp().await;

            let tl = self.space.position;
            let _ = draw_thin_rect(self.space, dt);
            let _ = draw_accelerator(tl, self.key, dt);

            label_once_on(self.text, tl + pt(ANNOTATION_WIDTH + SPACING, SPACING), dt);
        }

        request_redraw();

        loop {
            wait(self.key).await;

            self.notif.notify(self.value.clone());
        }
    }
}

impl<'s, 'n, T> Button<'s, 'n, T> {
    pub fn new(
        a: Accel,
        position: Point,
        text: &'s str,
        notif: &'n Notify<T>,
        value: T,
    ) -> (Self, Accel) {
        let size = Size::new(
            SPACING as u32 * 2 + FONT.character_size.width * (text.len() as u32 + 1),
            SPACING as u32 * 2 + FONT.character_size.height,
        );
        let (key, g) = a.next();

        (
            Button {
                text,
                notif,
                key,
                space: Space::new(position, size),
                value,
            },
            g,
        )
    }
}
