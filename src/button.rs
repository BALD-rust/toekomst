use embedded_graphics::prelude::{Point, Size};
use embedded_graphics::primitives::{Rectangle, StyledDrawable};
use embedded_graphics::text::Text;
use embedded_graphics::Drawable;

use crate::display::disp;
use crate::key::{wait, Accel, Key};
use crate::notify::Notify;
use crate::widget::{Space, Widget};
use crate::{text, text_inverted, thin_line, FONT, SMALL_FONT};

const SPACING: u32 = 3;

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
            // Draw bounding rect
            let _ =
                Rectangle::new(self.space.position, self.space.size).draw_styled(&thin_line(), dt);

            // Draw accelerator
            let _ = Text::new(
                self.key.into(),
                self.space.position + Point::new(1, SMALL_FONT.baseline as i32 + 1),
                text_inverted(SMALL_FONT),
            )
            .draw(dt);

            // Draw button label
            let _ = Text::new(
                self.text,
                self.space.position
                    + Point::new(
                        SPACING as i32 * 2 + SMALL_FONT.character_size.width as i32,
                        SPACING as i32 + FONT.baseline as i32,
                    ),
                text(FONT),
            )
            .draw(dt);
        }

        loop {
            log::trace!("wait key");
            wait(self.key).await;
            log::trace!("button pressed");

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
            SPACING * 3 + FONT.character_size.width * (text.len() as u32 + 1),
            SPACING * 2 + FONT.character_size.height,
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
