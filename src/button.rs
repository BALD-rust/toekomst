use crate::display::disp;
use crate::{text, text_inverted, thin_line, thin_line_off, FONT, SMALL_FONT};
use embassy_futures::yield_now;
use embedded_graphics::prelude::{Point, Size};
use embedded_graphics::primitives::{Rectangle, StyledDrawable};
use embedded_graphics::text::Text;
use embedded_graphics::Drawable;

use crate::key::{wait, Accel, Key};
use crate::notify::Notify;
use crate::widget::{Space, Widget};

const SPACING: u32 = 3;

pub struct Button<'s, 'n> {
    text: &'s str,
    notif: &'n Notify,
    key: Key,
    space: Space,
}

impl<'s, 'n> Widget for Button<'s, 'n> {
    type Output = ();

    fn space(&self) -> Space {
        self.space
    }

    async fn render(&self) {
        {
            let disp = &mut *disp().await;
            // Draw bounding rect
            let _ = Rectangle::new(self.space.position, self.space.size)
                .draw_styled(&thin_line(), disp);

            // Draw accelerator
            let _ = Text::new(
                self.key.into(),
                self.space.position
                    + Point::new(
                        SPACING as i32,
                        SPACING as i32 + SMALL_FONT.baseline as i32 + 1,
                    ),
                text_inverted(SMALL_FONT),
            )
            .draw(disp);

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
            .draw(disp);
        }

        loop {
            wait(self.key).await;

            // Draw reaction
            let _ = Rectangle::new(
                self.space.position + Point::new(1, 1),
                self.space.size - Size::new(2, 2),
            )
            .draw_styled(&thin_line(), &mut *disp().await);

            self.notif.notify();
            yield_now().await;

            // Retract reaction
            let _ = Rectangle::new(
                self.space.position + Point::new(1, 1),
                self.space.size - Size::new(2, 2),
            )
            .draw_styled(&thin_line_off(), &mut *disp().await);
        }
    }
}

impl<'s, 'n> Button<'s, 'n> {
    pub fn new(a: Accel, position: Point, text: &'s str, notif: &'n Notify) -> (Accel, Self) {
        let size = Size::new(
            SPACING * 3 + FONT.character_size.width * (text.len() as u32 + 1),
            SPACING * 2 + FONT.character_size.height,
        );
        let (key, g) = a.next();

        (
            g,
            Button {
                text,
                notif,
                key,
                space: Space::new(position, size),
            },
        )
    }
}
