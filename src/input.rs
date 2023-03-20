use embedded_graphics::prelude::{Point, Size};
use alloc::string::String;

use crate::common::{draw_accelerator, draw_thin_rect};
use crate::display::disp;
use crate::key::{Accel, Key, wait, wait_inhibiting};
use crate::label::label_once;
use crate::notify::Notify;
use crate::{character_stride, FONT, request_redraw};
pub use crate::SMALL_FONT;
use crate::widget::{clean_space, Space, Widget};

const SPACING: u32 = 3;

pub struct Input<'n, const N: usize> {
    notif: &'n Notify<String>,
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

        let mut index = 0usize;
        let mut string = String::with_capacity(N);

        'main: loop {
            wait(self.key).await;

            'key: loop {
                let key = wait_inhibiting().await;

                match key {
                    Key::esc => continue 'main,
                    Key::confirm => {
                        // TODO: clone??
                        self.notif.notify(string.clone());
                        continue 'main;
                    },
                    Key::backspace => {
                        if index != 0 {
                            let text_pos = self.space.position + Point::new(character_stride(FONT) as i32 * (index + 2 - 1) as i32, 1);
                            clean_space(Space::new(text_pos, FONT.character_size - Size::new(0, 1))).await;
                            request_redraw();
                            unsafe {
                                let _ = string.as_mut_vec().pop();
                            }
                            index -= 1;
                        }
                        continue 'key;
                    }
                    _ => {}
                };

                let text_pos = self.space.position + Point::new(character_stride(FONT) as i32 * (index + 2) as i32, 0);
                let s = match key {
                    Key::space => " ",
                    other_key => other_key.into()
                };
                label_once(s, text_pos).await;
                unsafe {
                    string.as_mut_vec().push(s.as_bytes()[0]);
                }
                index += 1;

                request_redraw();
            }
        }
    }
}

impl<'n, 'd, const N: usize> Input<'n, N> {
    pub fn new(ac: Accel, position: Point, notif: &'n Notify<String>) -> (Input<'n, N>, Accel) {
        let (key, ac) = ac.next();

        (
            Self {
                notif,
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
