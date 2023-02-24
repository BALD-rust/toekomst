use embedded_graphics::prelude::{Point, Size};

use crate::common::{draw_accelerator, draw_thin_rect};
use crate::display::disp;
use crate::key::{Accel, inhibit_key, Key, wait};
use crate::notify::Notify;
use crate::widget::{Space, Widget};
pub use crate::SMALL_FONT;

const SPACING: u32 = 3;

pub struct Input<'n, const N: usize> {
    notif: &'n Notify<String>,
    index: usize,
    text: [u8; N],
    space: Space,
    key: Key,
}

impl<const N: usize> Widget for Input<'_, N> {
    type Output = ();

    fn space(&self) -> Space {
        self.space
    }

    async fn render(&self) {
        {
            let dt = &mut *disp().await;

            let _ = draw_thin_rect(self.space, dt);
            let _ = draw_accelerator(self.space.position, self.key, dt);
        }

        wait(self.key).await;

        println!("I'm inhibiting keys!");
        loop {
            println!("{:?}", inhibit_key().await);
        }
    }
}

impl<'n, 'd, const N: usize> Input<'n, N> {
    pub fn new(ac: Accel, position: Point, notif: &'n Notify<String>) -> (Input<'n, N>, Accel) {
        let (key, ac) = ac.next();

        (
            Self {
                notif,
                index: 0,
                text: [0; N],
                space: Space::new(
                    position,
                    Size::new(
                        SPACING * 2
                            + N as u32 * SMALL_FONT.character_size.width
                            + (N - 1) as u32 * SMALL_FONT.character_spacing,
                        SPACING * 2 + SMALL_FONT.character_size.height,
                    ),
                ),
                key,
            },
            ac,
        )
    }
}
